use std::{io::Write, path::Path};
use std::fs::File;
use serde_json;
use std::collections::{
    HashMap,
    VecDeque,
    BinaryHeap
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

const REDSTONE_NEIGHBORHOOD: &[LabeledPoint; 14] = &[
    LabeledPoint { x:  1, y:  0, z:  0, label: None },
    LabeledPoint { x: -1, y:  0, z:  0, label: None },
    LabeledPoint { x:  1, y:  1, z:  0, label: None },
    LabeledPoint { x: -1, y:  1, z:  0, label: None },
    LabeledPoint { x:  1, y:  -1, z:  0, label: None },
    LabeledPoint { x: -1, y:  -1, z:  0, label: None },

    LabeledPoint { x:  0, y:  1, z:  0, label: None },
    LabeledPoint { x:  0, y: -1, z:  0, label: None },

    LabeledPoint { x:  0, y:  0, z:  1, label: None },
    LabeledPoint { x:  0, y:  0, z: -1, label: None },
    LabeledPoint { x:  0, y:  1, z:  1, label: None },
    LabeledPoint { x:  0, y:  1, z: -1, label: None },
    LabeledPoint { x:  0, y:  -1, z:  1, label: None },
    LabeledPoint { x:  0, y:  -1, z: -1, label: None }
];

const MAX_SIGNAL_STRENGTH: u8 = 15;

pub struct Gate {
    pub name: String,
    pub x: i32,
    pub z: i32,
    pub y: i32
}

#[derive(Debug, Deserialize, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Point {
    fn to_labeled_point(&self) -> LabeledPoint {
        LabeledPoint {
            x: self.x,
            y: self.y,
            z: self.z,
            label: None
        }
    }
}

#[derive(Clone, Debug)]
pub struct LabeledPoint {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub label: Option<String>
}

impl PartialEq for LabeledPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl LabeledPoint {
    fn compare(&self, point: &LabeledPoint) -> bool {
        self.x == point.x && self.y == point.y && self.z == point.z
    }

    fn compare_point(&self, point: &Point) -> bool {
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

#[derive(Clone, Debug)]
pub struct Wire {
    pub start: LabeledPoint,
    pub ends: Vec<LabeledPoint>
}

#[derive(Debug)]
struct Route {
    wire: Wire,
    // Points contains all points in the route (for all wire ends)
    //  EXCEPT those that are on gate pins (i.e. wire.start & wire.ends are excluded)
    points: Vec<Point>
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct RouteCost {
    route_idx: usize,
    cost: i32
}

impl Ord for RouteCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for RouteCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct RedstoneSegment {
    point: Point,
    prev_delta: Point,
    signal_strength: u8
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
    Lee { padding: Point, do_rerouting: bool }
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
     *   0   empty and unchecked
     *  >0   checked, marked by distance from starting point
     *  -1   gate in this location
     *  -2   pin skirt, no wires in this area unless they're connecting to the pin
     *  -3   borders a wire
     *  <=-4 wire, marked by # wires/wire bordering cells at this location
     */
    grid: Vec<Vec<Vec<i32>>>
}

// Penalties for intersecting wires/running them next to each other
static GRID_ADJACENCY_COST: i32 = 1;
static GRID_INTERSECTION_COST: i32 = 2;

static GRID_EMPTY: i32 = 0;
static GRID_GATE: i32 = -1;
static GRID_PIN_SKIRT: i32 = -2;
static GRID_WIRE_BORDER: i32 = -3;
static GRID_BASE_WIRE: i32 = -4;

fn grid_is_blocked(grid_value: i32) -> bool {
    grid_value < GRID_EMPTY
}

fn grid_is_wire(grid_value: i32) -> bool {
    grid_value <= GRID_BASE_WIRE
}

fn grid_num_intersections(grid_value: i32) -> i32 {
    if grid_is_wire(grid_value) {
        -(grid_value - GRID_BASE_WIRE) + 1
    }
    else {
        0
    }
}

fn grid_is_lee_floodfill(grid_value: i32) -> bool {
    grid_value > GRID_EMPTY
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

    fn contains(&self, point: &LabeledPoint) -> bool {
        return self.to_grid_point(point).is_ok();
    }

    /**
    Sets the point to value or returns an error if point is outside the grid
    */
    fn set(&mut self, point: &LabeledPoint, value: i32)
            -> Result<(), Box<dyn std::error::Error>> {
        let grid_point = self.to_grid_point(point)?;
        self.grid[grid_point.x][grid_point.y][grid_point.z] = value;
        Ok(())
    }

    fn get(&self, point: &LabeledPoint)
            -> Result<i32, Box<dyn std::error::Error>> {
        let grid_point = self.to_grid_point(point)?;
        Ok(self.grid[grid_point.x][grid_point.y][grid_point.z])
    }

    fn modify_skirt(&mut self, pin: &LabeledPoint, value_to_replace: i32, value_to_write: i32) {
        for delta1 in REDSTONE_NEIGHBORHOOD {
            let neighbor = LabeledPoint {
                x: pin.x + delta1.x,
                y: pin.y + delta1.y,
                z: pin.z + delta1.z,
                label: None
            };
            match self.get(&neighbor) {
                Ok(neighbor_value) => {
                    if neighbor_value == value_to_replace {
                        self.set(&neighbor, value_to_write).unwrap();
                    }

                    for delta2 in REDSTONE_NEIGHBORHOOD {
                        let grandneighbor = LabeledPoint {
                            x: neighbor.x + delta2.x,
                            y: neighbor.y + delta2.y,
                            z: neighbor.z + delta2.z,
                            label: None
                        };

                        match self.get(&grandneighbor) {
                            Ok(grandneighbor_value) => {
                                if grandneighbor_value == value_to_replace {
                                    self.set(&grandneighbor, value_to_write).unwrap();
                                }
                            },
                            Err(_) => {}
                        }
                    }
                },
                Err(_) => {}
            }
        }
    }

    fn add_pin_skirt(&mut self, pin: &LabeledPoint) {
        self.modify_skirt(pin, GRID_EMPTY, GRID_PIN_SKIRT);
    }

    fn remove_pin_skirt(&mut self, pin: &LabeledPoint) {
        self.modify_skirt(pin, GRID_PIN_SKIRT, GRID_EMPTY);
    }

    fn add_route(&mut self, route: &Route) -> i32 {
        let mut cost = 0;

        // Calculate cost based on existing adjacent/intersecting wires/gates
        // Doesn't use adjacency cells (GRID_WIRE_BORDER), counts actual nearby wires
        // Skip first & last points, assuming these are gate pins
        println!("{route:?}");
        for point in &route.points {
            let point_value = self.get(&point.to_labeled_point()).unwrap();
            assert!(point_value != GRID_GATE, "Wire intersected with gate, not supported ({point:?})");
            if grid_is_wire(point_value) {
                cost += grid_num_intersections(point_value) * GRID_INTERSECTION_COST;
            }

            for delta in MANHATTEN_NEIGHBORHOOD {
                let neighbor = LabeledPoint { x: point.x + delta.x, y: point.y + delta.y, z: point.z + delta.z, label: None };
                match self.get(&neighbor) {
                    Ok(neighbor_value) => {
                        if neighbor_value == GRID_GATE {
                            cost += GRID_ADJACENCY_COST;
                        }
                        else if grid_is_wire(neighbor_value) {
                            cost += grid_num_intersections(neighbor_value) * GRID_ADJACENCY_COST;
                        }
                    },
                    Err(_) => {}
                }
            }
        }

        // Update grid with new intersection (not adjacency) counts
        for point in &route.points {
            let point_value = self.get(&point.to_labeled_point()).unwrap();

            if grid_is_wire(point_value) {
                // Add 1 intersection
                self.set(&point.to_labeled_point(), point_value - 1).unwrap();
            }
            else {
                self.set(&point.to_labeled_point(), GRID_BASE_WIRE).unwrap();
            }

            for delta in REDSTONE_NEIGHBORHOOD {
                let neighbor = LabeledPoint { x: point.x + delta.x, y: point.y + delta.y, z: point.z + delta.z, label: None };
                match self.get(&neighbor) {
                    Ok(neighbor_value) => {
                        if grid_is_wire(neighbor_value) {
                            self.set(&neighbor, neighbor_value - 1).unwrap();  // (add 1 intersection)
                        }
                        else if neighbor_value == GRID_EMPTY {
                            self.set(&neighbor, GRID_WIRE_BORDER).unwrap();
                        }
                    },
                    Err(_) => {}
                }
            }
        }

        return cost;
    }

    fn remove_route(&mut self, route: &Route) {
        for point in &route.points {
            let point_value = self.get(&point.to_labeled_point()).unwrap();
            assert!(point_value != GRID_GATE, "Wire intersected with gate, not supported ({point:?})");
            if point_value == GRID_BASE_WIRE {
                // Reset cell if this is the last wire on it
                self.set(&point.to_labeled_point(), GRID_EMPTY).unwrap();
            }
            else {
                // Remove 1 intersection
                self.set(&point.to_labeled_point(), point_value + 1).unwrap();
            }

            // Remove adjacency markers if they don't have their own wire neighbors
            for delta in MANHATTEN_NEIGHBORHOOD {
                let neighbor = LabeledPoint { x: point.x + delta.x, y: point.y + delta.y, z: point.z + delta.z, label: None };
                if self.get(&neighbor).is_ok_and(|v| v == GRID_WIRE_BORDER) {
                    let mut found_other_wire = false;
                    for delta in MANHATTEN_NEIGHBORHOOD {
                        let grandneighbor = LabeledPoint { x: point.x + delta.x, y: point.y + delta.y, z: point.z + delta.z, label: None };
                        if self.get(&grandneighbor).is_ok_and(grid_is_wire) {
                            found_other_wire = true;
                            break;
                        }
                    }
                    if !found_other_wire {
                        self.set(&neighbor, GRID_EMPTY).unwrap();
                    }
                }
            }
        }
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
    routing_algo: RoutingAlgo
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
        RoutingAlgo::Lee { padding, do_rerouting } => {
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
            min.z -= padding.z;

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
                            let point = &LabeledPoint {
                                x: gate.x + x,
                                y: gate.y + y,
                                z: gate.z + z,
                                label: None
                            };

                            // Ignore gate points outside of grid (i.e. gates that are taller than the highest pin y)
                            if initial_grid.get(point).is_ok_and(|v| v != GRID_EMPTY) {
                                return Err(format!("{} gate overlaps a pin skirt @ {point:?}", gate.name))?;
                            }

                            _ = initial_grid.set(point, GRID_GATE);
                        }
                    }
                }

                for pin in info.inputs.values().chain(info.outputs.values()) {
                    initial_grid.add_pin_skirt(&LabeledPoint { x: gate.x + pin.x, y: gate.y + pin.y, z: gate.z + pin.z, label: None });
                }
            }

            println!("{initial_grid}");

            let mut routes: Vec<Route> = vec![];
            let mut final_grid = initial_grid.clone();

            if do_rerouting {
                let mut route_costs: BinaryHeap<RouteCost> = BinaryHeap::new();

                // Initial greedy routes; will intersect
                for wire in wires {
                    routes.push(route_wire(&initial_grid, wire)?);

                    // Add all wires to final_grid & calculate intersection costs
                    route_costs.push(
                        RouteCost {
                            route_idx: routes.len() - 1,
                            cost: final_grid.add_route(routes.last().unwrap())
                        }
                    );
                }

                while route_costs.peek().is_some_and(|c| c.cost > 0) {
                    // find N (random) highest intersection wires
                    // remove high inters. wires from grid & route costs
                    // reroute high inters. wires
                    // add high inters. wires to grid & route costs again
                    // update high inters. wires in routes
                }
            }
            else {
                // Feed final_grid into route_wire to do a first pass w/o intersections
                // Without rerouting, impossible routes will kill routing
                for wire in wires {
                    for pin in std::iter::once(&wire.start).chain(wire.ends.iter()) {
                        final_grid.remove_pin_skirt(pin);
                    }
                    routes.push(route_wire(&final_grid, wire)?);
                    final_grid.add_route(routes.last().unwrap());
                    for pin in std::iter::once(&wire.start).chain(wire.ends.iter()) {
                        final_grid.add_pin_skirt(pin);
                    }
                }
            }

            for route in routes {
                let mut to_visit: Vec<RedstoneSegment> = vec![RedstoneSegment {
                    point: route.wire.start.to_point(),
                    prev_delta: Point { x: 0, y: 0, z: 0 },
                    signal_strength: MAX_SIGNAL_STRENGTH - 1
                }];
                while !to_visit.is_empty() {
                    let cur_point = to_visit.pop().unwrap();
                    let mut next_points: Vec<Point> = vec![];

                    for delta in REDSTONE_NEIGHBORHOOD {
                        if !delta.compare_point(&cur_point.prev_delta) && final_grid.get() {
                        }
                    }
                }
            }

            // Translate wires to Minecraft blocks
            for x in final_grid.min.x..=final_grid.max.x {
                for y in final_grid.min.y..=final_grid.max.y {
                    for z in final_grid.min.z..=final_grid.max.z {
                        if grid_is_wire(final_grid.get(&LabeledPoint { x, y, z, label: None }).unwrap()) {
                            writeln!(file, "setblock ~{} ~{} ~{} minecraft:pink_wool",
                                x, y - 1, z).expect(file_error);
                            writeln!(file, "setblock ~{} ~{} ~{} minecraft:redstone_wire",
                                x, y, z).expect(file_error);
                        }
                    }
                }
            }
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

fn route_wire(initial_grid: &Grid, wire: &Wire) -> Result<Route, Box<dyn std::error::Error>> {
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
            current_distance = temp_grid.get(&current)?;
        }
        // println!("Current dist: {current_distance}");

        // Check if the current point is one of the wire ends
        match ends_to_reach.iter()
                .position(|end| current.compare(end)) {
            Some(idx) => {
                ends_to_reach.swap_remove(idx);
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
            if temp_grid.get(&point).is_ok_and(|x| x == GRID_EMPTY) ||
                    ends_to_reach.contains(&point) {
                _ = temp_grid.set(&point, current_distance + 1);
                points_to_check.push_back(point);
            }
        }
    }

    println!("{temp_grid}");

    let mut route: Route = Route { points: vec![], wire: wire.clone() };

    // Work backward from end point back to start &
    // edit final_grid to set final wire positions
    for end in &wire.ends {
        let mut current = end.clone();
        let mut current_value = temp_grid.get(&current)?;
        println!("Backtracking from {}, {}, {}", current.x, current.y, current.z);
        'backtrack_loop: while !current.compare(&wire.start) {
            if !current.compare(end) {
                // Don't add ends, they're inside gates (they're the pin blocks)
                route.points.push(current.to_point());
            }
            println!("Current: {current:?} = {current_value}");

            for delta in MANHATTEN_NEIGHBORHOOD {
                let neighbor = LabeledPoint {
                    x: current.x + delta.x,
                    y: current.y + delta.y,
                    z: current.z + delta.z,
                    label: None
                };
                if neighbor.compare(&wire.start) ||
                    temp_grid.get(&neighbor).is_ok_and(|neighbor_value|
                        grid_is_lee_floodfill(neighbor_value) &&  // Don't go to gates/other wires
                        (neighbor_value < current_value ||  // Follow gradient down...
                         grid_is_blocked(current_value))  // ...or get off a gate if we're sitting on one
                    )
                {
                    current = neighbor;
                    current_value = temp_grid.get(&current)?;
                    continue 'backtrack_loop;
                }
            }

            return Err("Failed to backtrack in Lee routing")?;
        }
    }

    return Ok(route);
}