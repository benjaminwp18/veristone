use std::{io::Write};
use std::fs::File;
use std::collections::{
    BinaryHeap, VecDeque
};

use crate::{mcfunction, points, grid};

const MAX_SIGNAL_STRENGTH: u8 = 15;

mod repeater_directions {
    pub const TO_POSITIVE_Z: &str = "north";
    pub const TO_NEGATIVE_Z: &str = "south";
    pub const TO_POSITIVE_X: &str = "west";
    pub const TO_NEGATIVE_X: &str = "east";
}

const WIRE_SUPPORT_BLOCK_NAME: &str = "pink_wool";
const WIRE_BLOCK_NAME: &str = "redstone_wire";
const REPEATER_BLOCK_NAME: &str = "repeater";

struct RedstoneSegment {
    parent: Option<usize>,
    delta_to_parent: points::Point,

    children: Vec<usize>,
    deltas_to_children: Vec<points::Point>,

    point: points::Point,
    signal_strength: u8
}

pub struct LeeSettings {
    pub padding: points::Point,
    pub do_rerouting: bool
}

pub fn lee(file: &mut File, settings: &LeeSettings, gates: &Vec<points::Gate>, wires: &Vec<points::Wire>) -> Result<(), Box<dyn std::error::Error>> {
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
    min.x -= settings.padding.x;
    // Skipping negative y padding for now
    min.z -= settings.padding.z;

    max.x += settings.padding.x;
    max.y += settings.padding.y;
    max.z += settings.padding.z;

    // Access to map containing info about all gate types
    let gate_dict = points::get_gate_dict();

    let mut initial_grid = grid::Grid::new(&min, &max);

    // Mark all blocks occupied by gates
    for gate in gates {
        let gate_info = gate_dict.get(&gate.name).unwrap();

        for x in 0..gate_info.x_dim {
            for y in 0..gate_info.y_dim {
                for z in 0..gate_info.z_dim {
                    let point = &points::Point::new(
                        gate.x + x,
                        gate.y + y,
                        gate.z + z
                    );

                    // Ignore gate points outside of grid (i.e. gates that are taller than the highest pin y)
                    if initial_grid.get(point).is_ok_and(|v| v != grid::cell::EMPTY) {
                        return Err(format!("{} gate overlaps a pin skirt @ {point:?}", gate.name))?;
                    }

                    _ = initial_grid.set(point, grid::cell::GATE);
                }
            }
        }

        for pin in gate_info.inputs.values().chain(gate_info.outputs.values()) {
            initial_grid.add_pin_skirt(&gate.local_to_global_coords(pin));
        }
    }

    println!("{initial_grid}");

    let mut routes: Vec<points::Route> = vec![];
    let mut final_grid = initial_grid.clone();

    if settings.do_rerouting {
        let mut route_costs: BinaryHeap<points::RouteCost> = BinaryHeap::new();

        // Initial greedy routes; will intersect
        for wire in wires {
            routes.push(route_wire(&initial_grid, wire)?);

            // Add all wires to final_grid & calculate intersection costs
            route_costs.push(
                points::RouteCost {
                    route_idx: routes.len() - 1,
                    cost: final_grid.add_route(routes.last().unwrap())
                }
            );
        }

        while route_costs.peek().is_some_and(|c| c.cost > 0) {
            // TODO:
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

    // Place blocks in the world for each route
    for route in routes {
        let mut tree: Vec<RedstoneSegment> = vec![];
        // println!("{final_grid}");
        // println!("{route:?}");
        tree.push(
            RedstoneSegment {
                parent: None,
                delta_to_parent: points::Point { x: 0, y: 0, z: 0 },
                children: vec![],
                deltas_to_children: vec![],
                point: route.wire.start.to_point(),
                signal_strength: MAX_SIGNAL_STRENGTH - 1
            }
        );
        let mut idxs_to_visit: Vec<usize> = vec![0];
        while !idxs_to_visit.is_empty() {
            let cur_idx = idxs_to_visit.pop().unwrap();
            // println!("Visiting {:?}", &tree[cur_idx].point);

            for delta in points::REDSTONE_NEIGHBORHOOD {
                let neighbor = delta + &tree[cur_idx].point;
                if !delta.compare(&tree[cur_idx].delta_to_parent) &&
                        final_grid.get(&neighbor).is_ok_and(grid::cell::is_wire) {
                    // TODO: check for ends/start?
                    tree.push(
                        RedstoneSegment {
                            parent: Some(cur_idx),
                            delta_to_parent: -delta,
                            children: vec![],
                            deltas_to_children: vec![],
                            point: neighbor.clone(),
                            // Decrement signal strength w/minimum of 0
                            signal_strength: tree[cur_idx].signal_strength.checked_sub(1).unwrap_or(0)
                        }
                    );
                    // println!("Found wire at {:?}", neighbor);
                    let neighbor_idx = tree.len() - 1;
                    idxs_to_visit.push(neighbor_idx);
                    tree[cur_idx].children.push(neighbor_idx);
                    tree[cur_idx].deltas_to_children.push(delta.clone());

                    // Remove the wire as we go to avoid getting into loops
                    final_grid.set(&neighbor, grid::cell::EMPTY).unwrap();
                }
            }

            if final_grid.get(&tree[cur_idx].point).unwrap() != grid::cell::GATE {
                write_commands_for_segment(file, &mut tree, cur_idx)?;
            }
        }
    }

    Ok(())
}

fn write_commands_for_segment(file: &mut File, tree: &mut Vec<RedstoneSegment>, cur_idx: usize) -> Result<(), Box<dyn std::error::Error>> {
    if tree[cur_idx].signal_strength == 0 {
        let mut repeater_idx = cur_idx;
        while try_add_repeater(file, tree, repeater_idx).is_err() {
            match tree[repeater_idx].parent {
                Some(parent_idx) => {
                    if tree[parent_idx].signal_strength <= MAX_SIGNAL_STRENGTH {
                        repeater_idx = parent_idx;
                    }
                    else {
                        println!("WARNING: failed to create repeater backtracking from {:?} (reached previous signal boost), faking signal strength = 1", tree[cur_idx].point);
                        tree[cur_idx].signal_strength = 1;
                        break;
                    }
                }
                None => {
                    println!("WARNING: failed to create repeater backtracking from {:?} (reached end of wire), faking signal strength = 1", tree[cur_idx].point);
                    tree[cur_idx].signal_strength = 1;
                    break;
                }
            }
        }
    }
    else {
        // TODO: verticality
        place_redstone_wire(file, &tree[cur_idx].point)?;
    }

    Ok(())
}


fn try_add_repeater(file: &mut File, tree: &mut Vec<RedstoneSegment>, target_idx: usize) -> Result<(), ()> {
    // Repeaters can only be added on wire segments that:
    if tree[target_idx].deltas_to_children.len() == 1 {  // don't split,
        let only_child= &tree[target_idx].deltas_to_children[0];
        if // don't drop below the level of the repeater:
            tree[target_idx].delta_to_parent.y >= 0 && only_child.y >= 0 &&
            // and are straight:
            (tree[target_idx].delta_to_parent.x == only_child.x ||
            tree[target_idx].delta_to_parent.z == only_child.z) {

            if tree[target_idx].delta_to_parent.x == only_child.x {
                place_repeater(
                    file, &tree[target_idx].point,
                    if only_child.z > 0 {
                        repeater_directions::TO_POSITIVE_Z
                    }
                    else {
                        repeater_directions::TO_NEGATIVE_Z
                    }
                ).unwrap();
            }
            else if tree[target_idx].delta_to_parent.z == only_child.z {
                place_repeater(
                    file, &tree[target_idx].point,
                    if only_child.x > 0 {
                        repeater_directions::TO_POSITIVE_X
                    }
                    else {
                        repeater_directions::TO_NEGATIVE_X
                    }
                ).unwrap();
            }

            tree[target_idx].signal_strength = MAX_SIGNAL_STRENGTH + 1;

            // Update the signal strength of all descendants
            let mut children_to_visit: Vec<usize> = tree[target_idx].children.clone();
            while !children_to_visit.is_empty() {
                let child_idx = children_to_visit.pop().unwrap();
                tree[child_idx].signal_strength = tree[tree[child_idx].parent.unwrap()].signal_strength.checked_sub(1).unwrap_or(0);
                for grandchild_idx in &tree[child_idx].children {
                    children_to_visit.push(*grandchild_idx);
                }
            }
        }
        else {
            Err(())?
        }
    }
    else {
        // TODO: unnecessary step up cases: (len == 2 but repeater still possible)
        //  #
        // ##
        Err(())?
    }

    Ok(())
}

fn place_redstone_wire(file: &mut File, point: &points::Point) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(file, "setblock ~{} ~{} ~{} minecraft:{WIRE_SUPPORT_BLOCK_NAME}",
             point.x, point.y - 1, point.z)
             .expect(mcfunction::FILE_ERROR);
    writeln!(file, "setblock ~{} ~{} ~{} minecraft:{WIRE_BLOCK_NAME}",
             point.x, point.y, point.z)
             .expect(mcfunction::FILE_ERROR);
    Ok(())
}

fn place_repeater(file: &mut File, point: &points::Point, direction: &str) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(file, "setblock ~{} ~{} ~{} minecraft:{WIRE_SUPPORT_BLOCK_NAME}",
             point.x, point.y - 1, point.z)
             .expect(mcfunction::FILE_ERROR);
    writeln!(file, "setblock ~{} ~{} ~{} minecraft:{REPEATER_BLOCK_NAME}[facing={}]",
             point.x, point.y, point.z, direction)
             .expect(mcfunction::FILE_ERROR);
    Ok(())
}

fn route_wire(initial_grid: &grid::Grid, wire: &points::Wire) -> Result<points::Route, Box<dyn std::error::Error>> {
    let mut temp_grid: grid::Grid = initial_grid.clone();

    let wire_start = wire.start.to_point();
    let wire_ends: Vec<points::Point> = wire.ends.iter().map(|lp| lp.to_point()).collect();

    let mut points_to_check: VecDeque<points::Point> = VecDeque::new();
    points_to_check.push_back(wire_start.clone()); // push the starting point to start there

    print!("Wire: {}, {}, {} -> ", wire.start.x, wire.start.y, wire.start.z);
    for end in &wire.ends {
        print!("({}, {}, {}), ", end.x, end.y, end.z);
    }
    println!();

    let mut ends_to_reach  = wire_ends.clone();
    let mut current_distance;
    let mut at_start = true;
    while points_to_check.len() > 0 {
        let current: points::Point = points_to_check.pop_front().unwrap();
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
        for delta in points::CONNECTED_WIRE_NEIGHBORHOOD {
            let point = delta + &current;
            if temp_grid.get(&point).is_ok_and(|x| x == grid::cell::EMPTY) ||
                    ends_to_reach.contains(&point) {
                _ = temp_grid.set(&point, current_distance + 1);
                points_to_check.push_back(point);
            }
        }
    }

    println!("{temp_grid}");

    let mut route = points::Route { points: vec![], wire: wire.clone() };

    // Work backward from end point back to start &
    // edit final_grid to set final wire positions
    for end in wire_ends {
        let mut current = end.clone();
        let mut current_value = temp_grid.get(&current)?;
        println!("Backtracking from {}, {}, {}", current.x, current.y, current.z);
        'backtrack_loop: while !current.compare(&wire_start) {
            println!("Current: {current:?} = {current_value}");

            for delta in points::CONNECTED_WIRE_NEIGHBORHOOD {
                let neighbor = &current + delta;
                if neighbor.compare(&wire_start) ||
                    temp_grid.get(&neighbor).is_ok_and(|neighbor_value|
                        grid::cell::is_lee_floodfill(neighbor_value) &&  // Don't go to gates/other wires
                        (neighbor_value < current_value ||  // Follow gradient down...
                         grid::cell::is_blocked(current_value))  // ...or get off a gate if we're sitting on one
                    )
                {
                    // We're leaving current, add it to the route
                    if !current.compare(&end) {
                        // Don't add ends, they're inside gates (they're the pin blocks)
                        route.points.push(current);
                    }

                    // Move to neighbor
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
