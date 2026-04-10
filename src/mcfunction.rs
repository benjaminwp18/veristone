use std::{io::Write, path::Path};
use std::fs::File;
use serde_json;
use std::collections::{
    HashMap,
    VecDeque
};
use std::fs;
use serde::Deserialize;

const MCFUNCTION_PATH: &'static str =
    "res/logic_datapack/data/logic/function/build.mcfunction";

const MANHATTEN_NEIGHBORHOOD: &[LabeledPoint; 6] = &[
    LabeledPoint { x:  1, y:  0, z:  0, label: None },
    LabeledPoint { x: -1, y:  0, z:  0, label: None },
    LabeledPoint { x:  0, y:  1, z:  0, label: None },
    LabeledPoint { x:  0, y: -1, z:  0, label: None },
    LabeledPoint { x:  0, y:  0, z:  1, label: None },
    LabeledPoint { x:  0, y:  0, z: -1, label: None }
];

pub struct Gate {
    pub name: String,
    pub x: i32,
    pub z: i32,
    pub y: i32
}

#[derive(Clone, Debug)]
pub struct LabeledPoint {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub label: Option<String>
}

impl LabeledPoint {
    fn compare(&self, point: &LabeledPoint) -> bool {
        self.x == point.x && self.y == point.y && self.z == point.z
    }

    fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}

pub struct Wire {
    pub start: LabeledPoint,
    pub ends: Vec<LabeledPoint>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Point {
    pub x: i32,
    pub z: i32,
    pub y: i32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GateInfo {
    pub x_dim: i32,
    pub z_dim: i32,
    pub y_dim: i32,
    pub inputs: HashMap<String, Point>,
    pub outputs: HashMap<String, Point>,
}

pub enum RoutingAlgo {
    Wireless,
    Lee { padding: Point }
}

struct GridPoint {
    x: usize,
    y: usize,
    z: usize
}

#[derive(Clone)]
struct Grid {
    min: Point,     // defined as bottom left in the x-z axis
    max: Point,     // defined as top right in the x-z axis
    x_size: usize,
    y_size: usize,
    z_size: usize,
    /*
     * grid values defined as follows:
     *  0 = empty and unchecked
     *  >0 = checked, marked by distance from starting point
     *  -1 = gate in this location
     *  -2 = wire in this location
     *  -3 = borders a wire
     */
    grid: Vec<Vec<Vec<i32>>>
}

impl Grid {
    fn new(min: &Point, max: &Point) -> Grid {
        let x_size: usize = usize::try_from((max.x - min.x).abs()).unwrap() + 1;
        let y_size: usize = usize::try_from((max.y - min.y).abs()).unwrap() + 1;
        let z_size: usize = usize::try_from((max.z - min.z).abs()).unwrap() + 1;

        println!("Grid: {x_size}, {y_size}, {z_size} @ {min:?}");

        Grid {
            min: min.clone(),
            max: max.clone(),
            x_size: x_size,
            y_size: y_size,
            z_size: z_size,
            grid: vec![vec![vec![0; z_size]; y_size]; x_size]
        }
    }

    fn contains(&self, point: &LabeledPoint) -> bool {
        return self.to_grid_point(point).is_ok();
    }

    /**
    Err() on the point lying outside the grid
    Ok(corresponding GridPoint) otherwise
    */
    fn to_grid_point(&self, point: &LabeledPoint)
            -> Result<GridPoint, Box<dyn std::error::Error>> {
        let x = point.x - self.min.x;
        let y = point.y - self.min.y;
        let z = point.z - self.min.z;
        if 0 <= x && 0 <= y && 0 <= z &&
           x < self.x_size as i32 && y < self.y_size as i32 && z < self.z_size as i32 {
            Ok(GridPoint { x: x.try_into()?, y: y.try_into()?, z: z.try_into()? })
        }
        else {
            Err(format!("Failed to convert point {point:?} to a point on the grid"))?
        }
    }

    /**
    Sets the point to dist or returns an error if point is outside the grid
    */
    fn set(&mut self, point: &LabeledPoint, dist: i32)
            -> Result<(), Box<dyn std::error::Error>> {
        let grid_point = self.to_grid_point(point)?;
        self.grid[grid_point.x][grid_point.y][grid_point.z] = dist;
        Ok(())
    }

    fn block_wire_area(&mut self, center_point: &LabeledPoint) {
        // Add -3s around each wire core
        for delta in MANHATTEN_NEIGHBORHOOD {
            match self.to_grid_point(&LabeledPoint {
                x: center_point.x + delta.x,
                y: center_point.y + delta.y,
                z: center_point.z + delta.z,
                label: None
            }) {
                Ok(grid_point) => {
                    // Don't overwrite other gates/wire centers
                    if self.grid[grid_point.x][grid_point.y][grid_point.z] >= 0 {
                        self.grid[grid_point.x][grid_point.y][grid_point.z] = -3;
                    }
                },
                Err(_) => {}  // Ignore if we're outside the grid
            }
        }

        // Mark wire center
        match self.to_grid_point(center_point) {
            Ok(grid_point) => {
                let current_value =
                    self.grid[grid_point.x][grid_point.y][grid_point.z];
                if current_value >= 0 || current_value == -3 {
                    self.grid[grid_point.x][grid_point.y][grid_point.z] = -2;
                }
            },
            Err(_) => {}  // Ignore if we're outside the grid
        }
    }

    fn get(&self, point: &LabeledPoint) -> i32 {
        let x: usize = (point.x - self.min.x) as usize;
        let y: usize = (point.y - self.min.y) as usize;
        let z: usize = (point.z - self.min.z) as usize;

        if x >= self.x_size || y >= self.y_size || z >= self.z_size {
            //println!("{}, {}, {}", x, y, z);
            // TODO: should probably error instead of returning a magic value
            return -4;
        }

        //println!("{}, {}, {}", self.x_size, self.y_size, self.z_size);
        self.grid[x][y][z]
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "GRID: {}x{}x{} at ({}, {}, {})",
            self.x_size, self.y_size, self.z_size,
            self.min.x, self.min.y, self.min.z)?;
        for y in 0..self.y_size {
            writeln!(f, "Y-Layer y={y}")?;
            write!(f, " X\\Z ")?;
            for z in 0..self.z_size {
                write!(f, "{:2} ", self.min.z + (z as i32))?;
            }
            writeln!(f)?;

            for x in 0..self.x_size {
                write!(f, " {:2}  ", self.min.x + (x as i32))?;
                for z in 0..self.z_size {
                    write!(f, "{:2} ", self.grid[x][y][z])?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn read_gate_info() -> HashMap<String, GateInfo> {
    let gate_info_str = fs::read_to_string("res/gate_info.json")
        .expect("Failed to read gate info file");
    return serde_json::from_str(&gate_info_str)
        .expect("Gate info JSON not well formatted");
}

pub fn write_mcfunction(
    gates: &Vec<Gate>,
    wires: &Vec<Wire>,
    routing_algo :RoutingAlgo
) -> Result<(), Box<dyn std::error::Error>> {
    let path: &Path = Path::new(MCFUNCTION_PATH);
    let mut file = File::create(path).unwrap();
    let file_error: &str =
        &format!("Failed to write gate to mcfunction file at {MCFUNCTION_PATH}");

    // place gates in the world
    for gate in gates {
        writeln!(file, "place template logic:{}_gate ~{} ~ ~{}",
                 gate.name, gate.x, gate.z).expect(file_error);
    }

    match routing_algo {
        RoutingAlgo::Lee { padding } => {
            // Find size of allowed volume for wires
            let mut min = wires[0].start.to_point();
            let mut max = wires[0].start.to_point();
            for wire in wires {
                for point in std::iter::once(&wire.start).chain(wire.ends.iter()) {
                    if point.x < min.x { min.x = point.x; }
                    if point.y < min.y { min.y = point.y; }
                    if point.z < min.z { min.z = point.z; }
                    if point.x > max.x { max.x = point.x; }
                    if point.y > max.y { max.y = point.y; }
                    if point.z > max.z { max.z = point.z; }
                }
            }
            min.x -= padding.x;
            // Skipping negative y padding for now
            max.z -= padding.z;

            max.x += padding.x;
            max.y += padding.y;
            max.z += padding.z;

            // Access to map containing info about all gate types
            let gate_info = read_gate_info();

            let mut initial_grid = Grid::new(&min, &max);

            // Mark all blocks occupied by gates
            for gate in gates {
                let info = gate_info.get(&gate.name).unwrap();

                for x in 0..info.x_dim {
                    for y in 0..info.y_dim {
                        for z in 0..info.z_dim {
                            _ = initial_grid.set(&LabeledPoint {
                                x: gate.x + x,
                                y: gate.y + y,
                                z: gate.z + z,
                                label: None
                            }, -1);
                        }
                    }
                }
            }

            let mut final_grid = initial_grid.clone();

            for wire in wires {
                let wire_points = route_wire(&initial_grid, wire)?;
                for point in wire_points {
                    
                }
            }

            // Translate wires to Minecraft blocks
            // for x in 0..initial_grid.grid.len() {
            //     for y in 0..initial_grid.grid[0].len() {
            //         for z in 0..initial_grid.grid[0][0].len() {
            //             if initial_grid.grid[x][y][z] == -2 {
            //                 writeln!(file, "setblock ~{} ~{} ~{} minecraft:pink_wool",
            //                     x, y - 1, z).expect(file_error);
            //                 writeln!(file, "setblock ~{} ~{} ~{} minecraft:redstone_wire",
            //                     x, y, z).expect(file_error);
            //             }
            //         }
            //     }
            // }
        },
        RoutingAlgo::Wireless => {
            for wire in wires {
                for end in &wire.ends {
                    let relative_start_x = wire.start.x - end.x;
                    let relative_start_z = wire.start.z - end.z;

                    let on_command = format!("execute if block ~{relative_start_x} ~{} ~{relative_start_z} minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block", 1 + wire.start.y - end.y);
                    let off_command = format!("execute if block ~{relative_start_x} ~{} ~{relative_start_z} minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target", 2 + wire.start.y - end.y);

                    writeln!(file, "setblock ~{} ~{} ~{} minecraft:redstone_lamp", wire.start.x, wire.start.y - 1, wire.start.z).expect(file_error);
                    writeln!(file, "setblock ~{} ~{} ~{} minecraft:target", end.x, end.y - 1, end.z).expect(file_error);
                    writeln!(file, "setblock ~{} ~{} ~{} minecraft:repeating_command_block{{auto: 1b, Command: \"{on_command}\"}}", end.x, end.y - 2, end.z).expect(file_error);
                    writeln!(file, "setblock ~{} ~{} ~{} minecraft:repeating_command_block{{auto: 1b, Command: \"{off_command}\"}}", end.x, end.y - 3, end.z).expect(file_error);

                    if wire.start.label.is_some() {
                        writeln!(file, "summon minecraft:armor_stand ~{} ~{} ~{} {{Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: \"{}\"}}", (wire.start.x as f32) + 0.5, wire.start.y + 1, (wire.start.z as f32) + 0.5, wire.start.label.as_ref().unwrap()).expect(file_error);
                    }
                    if end.label.is_some() {
                        writeln!(file, "summon minecraft:armor_stand ~{} ~{} ~{} {{Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: \"{}\"}}", (end.x as f32) + 0.5, end.y + 1, (end.z as f32) + 0.5, end.label.as_ref().unwrap()).expect(file_error);
                    }
                }
            }
        }
    }

    println!("Successfully wrote mcfunction to {MCFUNCTION_PATH}");

    Ok(())
}

fn route_wire(initial_grid: &Grid, wire: &Wire) -> Result<Vec<Point>, Box<dyn std::error::Error>> {
    let mut temp_grid: Grid = initial_grid.clone();

    let mut points_to_check: VecDeque<LabeledPoint> = VecDeque::new();
    points_to_check.push_back(wire.start.clone()); // push the starting point to start there

    print!("Wire: {}, {}, {} -> ", wire.start.x, wire.start.y, wire.start.z);
    for end in &wire.ends {
        print!("({}, {}, {}), ", end.x, end.y, end.z);
    }
    println!();

    let mut ends_to_reach = wire.ends.clone();
    let mut current_distance;
    let mut at_start = true;
    while points_to_check.len() > 0 {
        let current: LabeledPoint = points_to_check.pop_front().unwrap();
        // println!("Getting @ {current:?}");
        if at_start {
            // Distance to start is 0 when at start
            // (even though start is on a gate, so its value is -1)
            current_distance = 0;
            at_start = false;
        }
        else {
            current_distance = temp_grid.get(&current);
        }
        // println!("Current dist: {current_distance}");

        match ends_to_reach.iter()
                .position(|end| current.compare(end)) {
            Some(idx) => {
                ends_to_reach.remove(idx);
                if ends_to_reach.len() == 0 {
                    // Stop searching if we've reached all wire ends
                    break;
                }
            },
            None => {}
        }

        // Add adjacent points to queue if they're empty
        for delta in MANHATTEN_NEIGHBORHOOD {
            let point = LabeledPoint {
                x: current.x + delta.x,
                y: current.y + delta.y,
                z: current.z + delta.z,
                label: None
            };
            if temp_grid.contains(&point) && temp_grid.get(&point) == 0 &&
                    !point.compare(&wire.start) {
                _ = temp_grid.set(&point, current_distance + 1);
                points_to_check.push_back(point);
            }
        }
    }

    println!("{temp_grid}");

    let mut wire_points: Vec<Point> = vec![];

    // Work backward from end point back to start &
    // edit final_grid to set final wire positions
    for end in &wire.ends {
        let mut current = end.clone();
        let mut current_value = temp_grid.get(&current);
        println!("Backtracking from {}, {}, {}", current.x, current.y, current.z);
        'backtrack_loop: while !current.compare(&wire.start) {
            wire_points.push(current.to_point());
            println!("Current: {current:?} = {}", temp_grid.get(&current));

            for delta in MANHATTEN_NEIGHBORHOOD {
                let neighbor = LabeledPoint {
                    x: current.x + delta.x,
                    y: current.y + delta.y,
                    z: current.z + delta.z,
                    label: None
                };
                let neighbor_value = temp_grid.get(&neighbor);
                println!("Checking {neighbor:?} = {neighbor_value}");
                if neighbor.compare(&wire.start) || (
                    temp_grid.contains(&neighbor) && (
                        0 < neighbor_value &&  // Don't go to gates/other wires
                        (neighbor_value < current_value ||  // Follow gradient down...
                            current_value < 0)  // ...or get off a gate if we're sitting on one
                    )
                ) {
                    println!("Setting current");
                    current = neighbor;
                    current_value = temp_grid.get(&current);
                    continue 'backtrack_loop;
                }
            }

            return Err("Failed to backtrack in Lee routing")?;
        }
    }

    return Ok(wire_points);
}