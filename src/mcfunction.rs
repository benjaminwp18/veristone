use std::{io::Write, path::Path};
use std::fs::File;
use graphviz_rust::attributes::start;
use graphviz_rust::print;
use petgraph::algo::condensation;
use serde_json;
use std::collections::{
    HashMap,
    VecDeque
};
use std::fs;
use serde::Deserialize;

const MCFUNCTION_PATH: &'static str = "res/logic_datapack/data/logic/function/build.mcfunction";

pub struct Gate {
    pub name: String,
    pub x: i32,
    pub z: i32,
    pub y: i32
}

#[derive(Clone, Debug)]
pub struct LabeledPoint {
    pub x: i32,
    pub z: i32,
    pub y: i32,
    pub label: Option<String>
}

impl LabeledPoint {
    fn compare(&self, point: &LabeledPoint) -> bool {
        self.x == point.x && self.y == point.y && self.z == point.z
    }
}

pub struct Wire {
    pub start: LabeledPoint,
    pub end: LabeledPoint
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
    Lee
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
    fn new(min: Point, max: Point) -> Grid {
        let x_size: usize = (max.x - min.x).abs().try_into().unwrap();
        let z_size: usize = (max.z - min.z).abs().try_into().unwrap();
        let y_size: usize = (if x_size < z_size { z_size * 2 } else { x_size * 2 }).try_into().unwrap();

        Grid { 
            min: min, 
            max: max, 
            x_size:x_size, 
            y_size:y_size, 
            z_size:z_size, 
            grid: vec![vec![vec![0; z_size]; y_size]; x_size] 
        }
    }

    fn set(&mut self, point: &LabeledPoint, dist: i32) {
        if dist == -2 {
            self.block_wire_area(point, dist);
        } else {
            let x: usize = (point.x - self.min.x) as usize;
            let y: usize = point.y as usize;
            let z: usize = (point.z - self.min.z) as usize;
            
            if !(x >= self.x_size || y >= self.y_size || z >= self.z_size) {
                self.grid[x][y][z] = dist;
            }
        }
    }

    fn block_wire_area(&mut self, point: &LabeledPoint, dist: i32) {
        let x: usize = (point.x - self.min.x) as usize;
        let y: usize = point.y as usize;
        let z: usize = (point.z - self.min.z) as usize;

        for i in 0..27 {
            self.grid[x-1 + (i/9)][y-1 + (i/3)][z-1 + (i%3)] = dist-1;
        }

        self.grid[x][y][z] = dist;
    }

    fn get(&self, point: &LabeledPoint) -> i32 {
        let x: usize = (point.x - self.min.x) as usize;
        let y: usize = point.y as usize;
        let z: usize = (point.z - self.min.z) as usize;

        println!("{}, {}, {}", x, y, z);

        if x >= self.x_size || y >= self.y_size || z >= self.z_size {
            return -4;
        }

        self.grid[x][y][z]
    }

    fn increment_distance(&mut self, point: &LabeledPoint) {
        self.set(point, self.get(point) + 1);
    }
}

pub fn read_gate_info() -> HashMap<String, GateInfo> {
    let gate_info_str = fs::read_to_string("res/gate_info.json").expect("Failed to read gate info file");
    return serde_json::from_str(&gate_info_str).expect("Gate info JSON not well formatted");
}

pub fn write_mcfunction(
    gates: &Vec<Gate>,
    wires: &Vec<Wire>,
    routing_algo :RoutingAlgo
) {
    let path: &Path = Path::new(MCFUNCTION_PATH);
    let mut file = File::create(path).unwrap();
    let file_error: &str = &format!("Failed to write gate to mcfunction file at {MCFUNCTION_PATH}");

    // place gates in the world
    for gate in gates {
        writeln!(file, "place template logic:{}_gate ~{} ~ ~{}", gate.name, gate.x, gate.z).expect(file_error);
    }

    match routing_algo {
        RoutingAlgo::Lee => {
            // Find size of allowed volume for wires
            let xs = wires.iter().map(|wire| vec![wire.start.x, wire.end.x]).flatten();
            let x_max = xs.clone().max().unwrap();
            let x_min = xs.min().unwrap();

            let zs = wires.iter().map(|wire| vec![wire.start.z, wire.end.z]).flatten();
            let z_max = zs.clone().max().unwrap();
            let z_min = zs.min().unwrap();

            // Access to map containing info about all gate types
            let gate_info = read_gate_info();

            let mut final_grid = Grid::new(Point { x: x_min, z: z_min, y: 0 }, Point { x: x_max, z: z_max, y: 10 });

            for gate in gates {
                let info = gate_info.get(&gate.name).unwrap();

                // Mark all blocks occupied by gates
                for i in 0..info.y_dim {
                    if (gate.y - final_grid.min.y + i) as usize > final_grid.y_size { break; }
                    for j in 0..info.z_dim {
                        if (gate.z - final_grid.min.z + j) as usize > final_grid.z_size { break; }
                        for k in 0..info.z_dim {
                            if (gate.z - final_grid.min.z + k) as usize > final_grid.x_size { break; }
                            final_grid.set(&LabeledPoint { x: k, z: j, y: i, label: None }, -1);
                        }
                    }    
                }
            }


            for wire in wires {
                let mut temp_grid: Grid = final_grid.clone();

                let mut blocks_to_check: VecDeque<LabeledPoint> = VecDeque::new();
                blocks_to_check.push_back(wire.start.clone()); // push the starting point to start there

                let end_point: LabeledPoint = wire.end.clone();
                
                while blocks_to_check.len() > 0 {
                    let current: LabeledPoint = blocks_to_check.pop_front().unwrap();
                    // check current point
                    if current.compare(&end_point) {
                        break;
                    }
                    
                    // check adjacent points and add them to list if applicable
                    let checking = [
                        LabeledPoint{x:current.x+1, y:current.y,   z:current.z,   label:None},
                        LabeledPoint{x:current.x,   y:current.y,   z:current.z+1, label:None},
                        LabeledPoint{x:current.x-1, y:current.y,   z:current.z,   label:None},
                        LabeledPoint{x:current.x,   y:current.y,   z:current.z-1, label:None},
                        LabeledPoint{x:current.x,   y:current.y+1, z:current.z,   label:None},
                        LabeledPoint{x:current.x,   y:current.y-1, z:current.z,   label:None}
                    ];

                    for p in checking {
                        if temp_grid.get(&p) < 0 {
                            continue;
                        } else {
                            temp_grid.increment_distance(&p);
                            blocks_to_check.push_back(p);
                        }
                    }
                }

                // Work backward from end point back to start, edit final_grid to set final wire positions
                let mut current: LabeledPoint = end_point;
                while !current.compare(&wire.start) {
                    let checking = [
                        LabeledPoint{x:current.x+1, y:current.y,   z:current.z,   label:None},
                        LabeledPoint{x:current.x,   y:current.y,   z:current.z+1, label:None},
                        LabeledPoint{x:current.x-1, y:current.y,   z:current.z,   label:None},
                        LabeledPoint{x:current.x,   y:current.y,   z:current.z-1, label:None},
                        LabeledPoint{x:current.x,   y:current.y+1, z:current.z,   label:None},
                        LabeledPoint{x:current.x,   y:current.y-1, z:current.z,   label:None}
                    ];

                    for p in checking {
                        let value = temp_grid.get(&p);
                        if value > 0 && value < temp_grid.get(&current) {
                            current = p.clone();
                            break;
                        }   
                    }
                }

                final_grid = temp_grid;
            }

            for x in 0..final_grid.grid.len() {
                for y in 0..final_grid.grid[0].len() {
                    for z in 0.. final_grid.grid[0][0].len() {
                        if final_grid.grid[x][y][z] == -2 {
                            writeln!(file, "setblock ~{} ~{} ~{} minecraft:pink_wool", x, y, z).expect(file_error);
                            writeln!(file, "setblock ~{} ~{} ~{} minecraft:redstone_wire", x, y+1, z).expect(file_error);
                        }
                    }
                }
            }
        },
        RoutingAlgo::Wireless => {
            for wire in wires {
                let relative_start_x = wire.start.x - wire.end.x;
                let relative_start_z = wire.start.z - wire.end.z;

                let on_command = format!("execute if block ~{relative_start_x} ~{} ~{relative_start_z} minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block", 1 + wire.start.y - wire.end.y);
                let off_command = format!("execute if block ~{relative_start_x} ~{} ~{relative_start_z} minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target", 2 + wire.start.y - wire.end.y);

                writeln!(file, "setblock ~{} ~{} ~{} minecraft:redstone_lamp", wire.start.x, wire.start.y, wire.start.z).expect(file_error);
                writeln!(file, "setblock ~{} ~{} ~{} minecraft:target", wire.end.x, wire.end.y, wire.end.z).expect(file_error);
                writeln!(file, "setblock ~{} ~{} ~{} minecraft:repeating_command_block{{auto: 1b, Command: \"{on_command}\"}}", wire.end.x, wire.end.y - 1, wire.end.z).expect(file_error);
                writeln!(file, "setblock ~{} ~{} ~{} minecraft:repeating_command_block{{auto: 1b, Command: \"{off_command}\"}}", wire.end.x, wire.end.y - 2, wire.end.z).expect(file_error);

                if wire.start.label.is_some() {
                    writeln!(file, "summon minecraft:armor_stand ~{} ~{} ~{} {{Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: \"{}\"}}", (wire.start.x as f32) + 0.5, wire.start.y + 1, (wire.start.z as f32) + 0.5, wire.start.label.as_ref().unwrap()).expect(file_error);
                }
                if wire.end.label.is_some() {
                    writeln!(file, "summon minecraft:armor_stand ~{} ~{} ~{} {{Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: \"{}\"}}", (wire.end.x as f32) + 0.5, wire.end.y + 1, (wire.end.z as f32) + 0.5, wire.end.label.as_ref().unwrap()).expect(file_error);
                }
            }
        }
    }

    println!("Successfully wrote mcfunction to {MCFUNCTION_PATH}");
}
