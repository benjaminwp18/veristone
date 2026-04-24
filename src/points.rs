use std::ops;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;

pub const MANHATTEN_NEIGHBORHOOD: &[Point; 6] = &[
    Point::new( 1,  0,  0),
    Point::new(-1,  0,  0),
    Point::new( 0,  1,  0),
    Point::new( 0, -1,  0),
    Point::new( 0,  0,  1),
    Point::new( 0,  0, -1)
];

pub const REDSTONE_NEIGHBORHOOD: &[Point; 14] = &[
    Point::new( 1,  0,  0),
    Point::new(-1,  0,  0),
    Point::new( 1,  1,  0),
    Point::new(-1,  1,  0),
    Point::new( 1, -1,  0),
    Point::new(-1, -1,  0),

    Point::new( 0,  1,  0),
    Point::new( 0, -1,  0),

    Point::new( 0,  0,  1),
    Point::new( 0,  0, -1),
    Point::new( 0,  1,  1),
    Point::new( 0,  1, -1),
    Point::new( 0, -1,  1),
    Point::new( 0, -1, -1)
];

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Point {
    pub const fn new(x: i32, y: i32, z: i32) -> Point {
        return Point { x: x, y: y, z: z };
    }

    pub fn to_labeled_point(&self) -> LabeledPoint {
        LabeledPoint {
            x: self.x,
            y: self.y,
            z: self.z,
            label: None
        }
    }

    pub fn compare(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y && self.z == point.z
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
    pub const fn no_label(x: i32, y: i32, z: i32) -> LabeledPoint {
        return LabeledPoint { x: x, y: y, z: z, label: None };
    }

    pub fn compare(&self, point: &LabeledPoint) -> bool {
        self.x == point.x && self.y == point.y && self.z == point.z
    }

    pub fn compare_point(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y && self.z == point.z
    }

    pub fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}

impl_op_ex!(- |a: &Point| -> Point {
    Point::new(-a.x, -a.y, -a.z)
});

impl_op_ex!(+ |a: &Point, b: &Point| -> Point {
    Point::new(a.x + b.x, a.y + b.y, a.z + b.z)
});

impl_op_ex!(+ |a: &Point, b: &LabeledPoint| -> Point {
    Point::new(a.x + b.x, a.y + b.y, a.z + b.z)
});

impl_op_ex!(- |a: &LabeledPoint| -> LabeledPoint {
    LabeledPoint::no_label(-a.x, -a.y, -a.z)
});

impl_op_ex!(+ |a: &LabeledPoint, b: &LabeledPoint| -> LabeledPoint {
    LabeledPoint::no_label(a.x + b.x, a.y + b.y, a.z + b.z)
});

impl_op_ex!(+ |a: &LabeledPoint, b: &Point| -> LabeledPoint {
    LabeledPoint::no_label(a.x + b.x, a.y + b.y, a.z + b.z)
});

pub fn get_gate_dict() -> &'static HashMap<String, GateInfo> {
    static CONFIG: OnceLock<HashMap<String, GateInfo>> = OnceLock::new();

    // The closure inside get_or_init runs only once
    CONFIG.get_or_init(|| {
        let gate_info_str = fs::read_to_string("res/gate_info.json")
            .expect("Failed to read gate info file");
        return serde_json::from_str(&gate_info_str)
            .expect("Gate info JSON not well formatted");
    })
}

pub struct Gate {
    pub name: String,
    pub x: i32,
    pub z: i32,
    pub y: i32
}

impl Gate {
    pub fn local_to_global_coords(&self, point: &Point) -> LabeledPoint {
        return LabeledPoint::no_label(self.x + point.x, self.y + point.y, self.z + point.z);
    }
}

#[derive(Clone, Debug)]
pub struct Wire {
    pub start: LabeledPoint,
    pub ends: Vec<LabeledPoint>
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

#[derive(Debug)]
pub struct Route {
    pub wire: Wire,
    // Points contains all points in the route (for all wire ends)
    //  EXCEPT those that are on gate pins (i.e. wire.start & wire.ends are excluded)
    pub points: Vec<Point>
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RouteCost {
    pub route_idx: usize,
    pub cost: i32
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
