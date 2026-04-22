use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;

pub const MANHATTEN_NEIGHBORHOOD: &[LabeledPoint; 6] = &[
    LabeledPoint { x:  1, y:  0, z:  0, label: None },
    LabeledPoint { x: -1, y:  0, z:  0, label: None },
    LabeledPoint { x:  0, y:  1, z:  0, label: None },
    LabeledPoint { x:  0, y: -1, z:  0, label: None },
    LabeledPoint { x:  0, y:  0, z:  1, label: None },
    LabeledPoint { x:  0, y:  0, z: -1, label: None }
];

pub const REDSTONE_NEIGHBORHOOD: &[LabeledPoint; 14] = &[
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

#[derive(Debug, Deserialize, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Point {
    pub fn to_labeled_point(&self) -> LabeledPoint {
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
