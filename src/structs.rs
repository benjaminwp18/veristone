use std::fmt::{Display, Formatter, Result};

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