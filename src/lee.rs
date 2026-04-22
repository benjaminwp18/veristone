use std::{io::Write};
use std::fs::File;
use std::collections::{
    VecDeque,
    BinaryHeap
};

use crate::{points, grid};

const MAX_SIGNAL_STRENGTH: u8 = 15;

struct RedstoneSegment {
    point: points::Point,
    prev_delta: points::Point,
    signal_strength: u8
}

pub struct LeeSettings {
    pub padding: points::Point,
    pub do_rerouting: bool
}

pub fn lee(file: &mut File, file_error: &str, settings: &LeeSettings, gates: &Vec<points::Gate>, wires: &Vec<points::Wire>) -> Result<(), Box<dyn std::error::Error>> {
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
                    let point = &points::LabeledPoint {
                        x: gate.x + x,
                        y: gate.y + y,
                        z: gate.z + z,
                        label: None
                    };

                    // Ignore gate points outside of grid (i.e. gates that are taller than the highest pin y)
                    if initial_grid.get(point).is_ok_and(|v| v != grid::cell::EMPTY) {
                        return Err(format!("{} gate overlaps a pin skirt @ {point:?}", gate.name))?;
                    }

                    _ = initial_grid.set(point, grid::cell::GATE);
                }
            }
        }

        for pin in gate_info.inputs.values().chain(gate_info.outputs.values()) {
            initial_grid.add_pin_skirt(&points::LabeledPoint { x: gate.x + pin.x, y: gate.y + pin.y, z: gate.z + pin.z, label: None });
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

    // for route in routes {
    //     let mut to_visit: Vec<RedstoneSegment> = vec![RedstoneSegment {
    //         point: route.wire.start.to_point(),
    //         prev_delta: points::Point { x: 0, y: 0, z: 0 },
    //         signal_strength: MAX_SIGNAL_STRENGTH - 1
    //     }];
    //     while !to_visit.is_empty() {
    //         let cur_point = to_visit.pop().unwrap();
    //         let mut next_points: Vec<points::Point> = vec![];

    //         for delta in points::REDSTONE_NEIGHBORHOOD {
    //             if !delta.compare_point(&cur_point.prev_delta) && final_grid.get() {
    //             }
    //         }
    //     }
    // }

    // Translate wires to Minecraft blocks
    for x in final_grid.min.x..=final_grid.max.x {
        for y in final_grid.min.y..=final_grid.max.y {
            for z in final_grid.min.z..=final_grid.max.z {
                if grid::cell::is_wire(final_grid.get(&points::LabeledPoint { x, y, z, label: None }).unwrap()) {
                    writeln!(file, "setblock ~{} ~{} ~{} minecraft:pink_wool",
                        x, y - 1, z).expect(file_error);
                    writeln!(file, "setblock ~{} ~{} ~{} minecraft:redstone_wire",
                        x, y, z).expect(file_error);
                }
            }
        }
    }

    Ok(())
}

fn route_wire(initial_grid: &grid::Grid, wire: &points::Wire) -> Result<points::Route, Box<dyn std::error::Error>> {
    let mut temp_grid: grid::Grid = initial_grid.clone();

    let mut points_to_check: VecDeque<points::LabeledPoint> = VecDeque::new();
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
        let current: points::LabeledPoint = points_to_check.pop_front().unwrap();
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
        for delta in points::MANHATTEN_NEIGHBORHOOD {
            let point = points::LabeledPoint {
                x: current.x + delta.x,
                y: current.y + delta.y,
                z: current.z + delta.z,
                label: None
            };
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

            for delta in points::MANHATTEN_NEIGHBORHOOD {
                let neighbor = points::LabeledPoint {
                    x: current.x + delta.x,
                    y: current.y + delta.y,
                    z: current.z + delta.z,
                    label: None
                };
                if neighbor.compare(&wire.start) ||
                    temp_grid.get(&neighbor).is_ok_and(|neighbor_value|
                        grid::cell::is_lee_floodfill(neighbor_value) &&  // Don't go to gates/other wires
                        (neighbor_value < current_value ||  // Follow gradient down...
                         grid::cell::is_blocked(current_value))  // ...or get off a gate if we're sitting on one
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
