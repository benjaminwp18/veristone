use std::{
    collections::HashMap, 
    path::{self, Path},
    fmt::{Display, Result, Formatter},
};
use clap::Parser;
use blif_parser::*;
use petgraph::{graph, Directed};
use primitives::ParsedPrimitive;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path (relative) to blif file")]
    blif: String,
}

#[allow(dead_code)]
fn main() {
    let args = Args::parse();
    let blif_path = Path::new(&args.blif);
    read_blif(blif_path);
}

#[allow(unused_variables)]
pub fn read_blif(blif_path: &Path) {
    let binding = path::absolute(blif_path).unwrap().into_os_string();
    let full_path = binding.to_str().unwrap();

    println!("Reading BLIF file {full_path}");

    let list = parser::parse_blif_file(full_path).unwrap();
    
    let mut modules: HashMap<String, Module> = Vec::new();

    for module in list {
        match module {
            ParsedPrimitive::Module { name, inputs, outputs, elems } => {
                modules.add(name, Module { name, inputs, outputs, elems });
            },
            _ => print!("not a module")
        }
    }



    //blif_to_graph(list);
}

// const INPUT_PORT_NAMES: Vec<&str> = Vec::from(["A", "B"]);


// Graph & Node implementation
struct Node {
    node_type: NodeType,
    name: String
}

impl Node {
    fn new(node_type: NodeType, name: String) -> Node {
        Node { node_type, name }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.node_type, self.name)
    }
}

enum NodeType {
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

struct Module {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    elems: Vec<ParsedPrimitive>
}




#[allow(unused_variables)]
fn blif_to_graph(list: Vec<ParsedPrimitive>) -> graph::Graph<Node, String, Directed> {
    // graph = new Graph()
    // nets = new HashTable()
    // for (gate in blif file) {
    //     gate_node = new Node()
    //     gate_node.gate_type = gate.name
    //     graph.addNode(gate_node)
    //     for (cnxn in gate.conns) {
    //         port_node = new Node()
    //         port_node.port_name = cnxn[0]
    //         graph.addNode(port_node)
    //         graph.addEdge(gate_node, port_node)
    //         if (!nets.hasKey(cnxn[1])) {
    //             net_node = new Node()
    //             net_node.name = cnxn[1]
    //             nets.add(cnxn[1], net_node)
    //             graph.addNode(net_node)
    //         }
    //         graph.addEdge(nets.get(cnxn[1]), port_node)
    //     }
    // }
    
    let INPUT_PORT_NAMES: Vec<&str> = Vec::from(["A", "B"]);

    let mut graph: graph::Graph<Node, String, Directed> = graph::Graph::new();
    let mut nets: HashMap<String, graph::NodeIndex> = HashMap::new();

    


    for x in list.into_iter() {
        match x {
            ParsedPrimitive::NOP => print!("NOP"),

            ParsedPrimitive::Input { name } => print!("Input"),

            ParsedPrimitive::Output { name } => print!("Output"),

            ParsedPrimitive::Lut { inputs, output, table } => print!("Lut"),

            ParsedPrimitive::Gate { c, d, q, r, e } => print!("Gate"),

            ParsedPrimitive::Latch { input, output, control, init } => print!("Latch"),

            ParsedPrimitive::Subckt { name, conns } => {
                print!("Subckt");
                let gate: Node = Node::new(NodeType::Gate, name);
                let gate_index = graph.add_node(gate);

                for (port_name, net_name) in conns {
                    let net_index: graph::NodeIndex = if nets.contains_key(&net_name) {
                        *nets.get(&net_name).unwrap()
                    }
                    else {
                        let net: Node = Node::new(NodeType::Net, net_name);
                        graph.add_node(net)
                    };

                    graph.add_edge(gate_index, net_index, port_name);
                }
            },

            ParsedPrimitive::Module { name, inputs, outputs, elems } => {
                blif_to_graph(elems);
            },

            //_ => print!("not work"),
        }
    }

    print!("{}", graph.node_count());

    for n in graph.node_indices() {
        print!("{} ", &graph[n]);
    };

    graph
}



// Print blif file items
#[allow(unused_variables)]
fn print_blif_components(list: Vec<ParsedPrimitive>) {
    for x in list.into_iter() {
        match x {
            ParsedPrimitive::NOP => println!("NOP"),

            ParsedPrimitive::Input { name } => println!("--Input: {}", name),

            ParsedPrimitive::Output { name } => println!("--Output: {}", name),

            ParsedPrimitive::Lut {
                inputs,
                output,
                table,
            } => {
                println!("----Lut----");
                //inputs
                println!("--Inputs--");
                inputs.iter().for_each(|a| println!("{}", a));

                //output
                println!("Output: {}", output);

                //table
                println!("--Table--");
                let lut_table = table.iter().flatten();
                lut_table.for_each(|a| println!("{}", a));
            }

            ParsedPrimitive::Gate { c, d, q, r, e } => {
                println!("----Gate----");

                //whatever these are
                println!("c: {}", c);
                println!("d: {}", d);
                println!("q: {}", q);
                println!("r: {}", r.unwrap());
                println!("e: {}", e.unwrap());
            }

            ParsedPrimitive::Latch {
                input,
                output,
                control,
                init,
            } => {
                println!("----Latch----");
                println!("Input: {}", input);
                println!("Output: {}", output);

                // no idea what this is for
                match init {
                    primitives::LatchInit::ZER0 => println!("0"), // Zero spelled with a 0
                    primitives::LatchInit::ONE => println!("1"),
                    primitives::LatchInit::DONTCARE => println!("2"),
                    primitives::LatchInit::UNKNOWN => println!("3"),
                }
            }

            ParsedPrimitive::Subckt { name, conns } => {
                println!("----Subckt----");
                println!("Name: {}", name);
                for (key, value) in conns {
                    println!("{key}: {value}");
                }
            }

            ParsedPrimitive::Module {
                name,
                inputs,
                outputs,
                elems,
            } => {
                println!("----Module----");
                println!("Name: {}", name);

                //inputs
                print!("Inputs:");
                inputs.iter().for_each(|a| print!(" {},", a));
                println!();

                //outputs
                print!("Outputs:");
                outputs.iter().for_each(|a| print!(" {},", a));
                println!();

                //elements
                println!("Elements:");
                print_blif_components(elems);
            }
        }
    }
}
