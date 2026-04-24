use crate::points;

struct GridPoint {
    x: usize,
    y: usize,
    z: usize
}

#[derive(Clone)]
pub struct Grid {
    pub min: points::Point,     // defined as bottom left in the x-z axis
    pub max: points::Point,     // defined as top right in the x-z axis
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

pub mod cell {
    // Penalties for intersecting wires/running them next to each other
    pub static ADJACENCY_COST: i32 = 1;
    pub static INTERSECTION_COST: i32 = 2;

    pub static EMPTY: i32 = 0;
    pub static GATE: i32 = -1;
    pub static PIN_SKIRT: i32 = -2;
    pub static WIRE_BORDER: i32 = -3;
    pub static BASE_WIRE: i32 = -4;

    pub fn is_blocked(grid_value: i32) -> bool {
        grid_value < EMPTY
    }

    pub fn is_wire(grid_value: i32) -> bool {
        grid_value <= BASE_WIRE
    }

    pub fn num_intersections(grid_value: i32) -> i32 {
        if is_wire(grid_value) {
            -(grid_value - BASE_WIRE) + 1
        }
        else {
            0
        }
    }

    pub fn is_lee_floodfill(grid_value: i32) -> bool {
        grid_value > EMPTY
    }
}

impl Grid {
    pub fn new(min: &points::Point, max: &points::Point) -> Grid {
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
    fn to_grid_point(&self, point: &points::Point)
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

    pub fn contains(&self, point: &points::Point) -> bool {
        return self.to_grid_point(point).is_ok();
    }

    /**
    Sets the point to value or returns an error if point is outside the grid
    */
    pub fn set(&mut self, point: &points::Point, value: i32)
            -> Result<(), Box<dyn std::error::Error>> {
        let grid_point = self.to_grid_point(point)?;
        self.grid[grid_point.x][grid_point.y][grid_point.z] = value;
        Ok(())
    }

    pub fn get(&self, point: &points::Point)
            -> Result<i32, Box<dyn std::error::Error>> {
        let grid_point = self.to_grid_point(point)?;
        Ok(self.grid[grid_point.x][grid_point.y][grid_point.z])
    }

    fn modify_skirt(&mut self, pin: &points::LabeledPoint, value_to_replace: i32, value_to_write: i32) {
        for delta1 in points::REDSTONE_NEIGHBORHOOD {
            let neighbor = delta1 + pin;

            match self.get(&neighbor) {
                Ok(neighbor_value) => {
                    if neighbor_value == value_to_replace {
                        self.set(&neighbor, value_to_write).unwrap();
                    }

                    for delta2 in points::REDSTONE_NEIGHBORHOOD {
                        let grandneighbor = &neighbor + delta2;

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

    pub fn add_pin_skirt(&mut self, pin: &points::LabeledPoint) {
        self.modify_skirt(pin, cell::EMPTY, cell::PIN_SKIRT);
    }

    pub fn remove_pin_skirt(&mut self, pin: &points::LabeledPoint) {
        self.modify_skirt(pin, cell::PIN_SKIRT, cell::EMPTY);
    }

    pub fn add_route(&mut self, route: &points::Route) -> i32 {
        let mut cost = 0;

        // Calculate cost based on existing adjacent/intersecting wires/gates
        // Doesn't use adjacency cells (Values::WIRE_BORDER), counts actual nearby wires
        // Skip first & last points, assuming these are gate pins
        println!("{route:?}");
        for point in &route.points {
            let point_value = self.get(point).unwrap();
            assert!(point_value != cell::GATE, "Wire intersected with gate, not supported ({point:?})");
            if cell::is_wire(point_value) {
                cost += cell::num_intersections(point_value) * cell::INTERSECTION_COST;
            }

            for delta in points::MANHATTEN_NEIGHBORHOOD {
                let neighbor = delta + point;
                match self.get(&neighbor) {
                    Ok(neighbor_value) => {
                        if neighbor_value == cell::GATE {
                            cost += cell::ADJACENCY_COST;
                        }
                        else if cell::is_wire(neighbor_value) {
                            cost += cell::num_intersections(neighbor_value) * cell::ADJACENCY_COST;
                        }
                    },
                    Err(_) => {}
                }
            }
        }

        // Update grid with new intersection (not adjacency) counts
        for point in &route.points {
            let point_value = self.get(&point).unwrap();

            if cell::is_wire(point_value) {
                // Add 1 intersection
                self.set(&point, point_value - 1).unwrap();
            }
            else {
                self.set(&point, cell::BASE_WIRE).unwrap();
            }

            for delta in points::REDSTONE_NEIGHBORHOOD {
                let neighbor = point + delta;
                match self.get(&neighbor) {
                    Ok(neighbor_value) => {
                        if cell::is_wire(neighbor_value) {
                            self.set(&neighbor, neighbor_value - 1).unwrap();  // (add 1 intersection)
                        }
                        else if neighbor_value == cell::EMPTY {
                            self.set(&neighbor, cell::WIRE_BORDER).unwrap();
                        }
                    },
                    Err(_) => {}
                }
            }
        }

        return cost;
    }

    pub fn remove_route(&mut self, route: &points::Route) {
        for point in &route.points {
            let point_value = self.get(point).unwrap();
            assert!(point_value != cell::GATE, "Wire intersected with gate, not supported ({point:?})");
            if point_value == cell::BASE_WIRE {
                // Reset cell if this is the last wire on it
                self.set(point, cell::EMPTY).unwrap();
            }
            else {
                // Remove 1 intersection
                self.set(point, point_value + 1).unwrap();
            }

            // Remove adjacency markers if they don't have their own wire neighbors
            for delta1 in points::MANHATTEN_NEIGHBORHOOD {
                let neighbor = delta1 + point;
                if self.get(&neighbor).is_ok_and(|v| v == cell::WIRE_BORDER) {
                    let mut found_other_wire = false;
                    for delta2 in points::MANHATTEN_NEIGHBORHOOD {
                        let grandneighbor = delta2 + &neighbor;
                        if self.get(&grandneighbor).is_ok_and(cell::is_wire) {
                            found_other_wire = true;
                            break;
                        }
                    }
                    if !found_other_wire {
                        self.set(&neighbor, cell::EMPTY).unwrap();
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
