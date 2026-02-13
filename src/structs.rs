use std::fmt::{Display, Formatter, Result};
use serde::Deserialize;
use std::collections::HashMap;

// Graph & Node implementation
#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub name: String
}

impl Node {
    pub fn new(node_type: NodeType, name: String) -> Node {
        Node { node_type, name }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // write!(f, "({}, {})", self.node_type, self.name)
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub enum NodeType {
    Input,
    Output,
    Gate,
    Net
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match self {
            NodeType::Input => "NodeType::Input",
            NodeType::Output => "NodeType::Output",
            NodeType::Gate => "NodeType::Gate",
            NodeType::Net => "NodeType::Net",
        })
    }
}

pub struct Gate {
    pub name: String,
    pub x: i32,
    pub z: i32
}

#[derive(Clone, Debug)]
pub struct LabeledPoint {
    pub x: i32,
    pub z: i32,
    pub y: i32,
    pub label: Option<String>
}

pub struct Wire {
    pub start: LabeledPoint,
    pub end: LabeledPoint
}

pub enum WireType {
    Wireless,
    Redstone
}

#[derive(Debug, Deserialize, Clone)]
pub struct Point {
    pub x: i32,
    pub z: i32
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