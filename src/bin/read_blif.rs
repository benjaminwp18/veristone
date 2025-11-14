use std::{
    collections::HashMap, fmt::{Display, Formatter, Result}, path::{self, Path}, fs
};
use clap::Parser;
use blif_parser::*;
use petgraph::{graph, Directed};
use petgraph::dot::Dot;
use primitives::ParsedPrimitive;
use graphviz_rust::dot_structures;

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

    let primitive_list = parser::parse_blif_file(full_path).unwrap();

    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut first_module_name: Option<String> = None;

    for module in primitive_list {
        match module {
            ParsedPrimitive::Module { name, inputs, outputs, elems } => {
                if first_module_name == None {
                    first_module_name = Some(name.clone())
                }
                modules.insert(name.clone(), Module { name: name.clone(), inputs, outputs, elems, inst_count: 0 });
            },
            _ => println!("Warning: Not Implemented: Top-level non-module primitive")
        }
    }

    let net_aliases: HashMap<String, String> = HashMap::new();
    let mut graph: graph::Graph<Node, String, Directed> = graph::Graph::new();
    let mut nets: HashMap<String, graph::NodeIndex> = HashMap::new();

    add_module_to_graph(&first_module_name.unwrap(), &modules, &mut graph, &mut nets, net_aliases);

    let graph_dot_str = Dot::new(&graph).to_string();
    println!("{graph_dot_str}");
    let format = graphviz_rust::cmd::Format::Svg;
    let graph_svg = graphviz_rust::exec_dot(graph_dot_str, vec![format.into()]).unwrap();
    let stem = blif_path.file_stem().unwrap().to_str().unwrap();
    let result = fs::write(format!("graph_{stem}.svg"), graph_svg).expect("Writing SVG to file:");

    //blif_to_graph(list);
}

// TODO: load this info from a data file that is also used to generate mc.lib
const INPUT_PIN_NAMES: [&'static str; 4] = ["A", "B", "C", "D"];
const OUTPUT_PIN_NAMES: [&'static str; 2] = ["Y", "Q"];

fn add_module_to_graph(
    module_name: &String,
    modules: &HashMap<String, Module>,
    graph: &mut graph::Graph<Node, String, Directed>,
    nets: &mut HashMap<String, graph::NodeIndex>,
    net_aliases: HashMap<String, String>
) {
    let module = modules.get(module_name).unwrap();
    let mut local_net_aliases: HashMap<String, String> = HashMap::new();
    for parsed_primitive in module.elems.iter() {
        match parsed_primitive {
            ParsedPrimitive::Subckt { name: subckt_name, conns } => {
                println!("Processing subcircuit {subckt_name}");

                // TODO: actually increment inst counters (probably need a separate data structure)
                // module.inst_count += 1;

                let subckt_is_module: bool = modules.contains_key(subckt_name);

                // Make simple gate node if we can
                let gate_index: Option<graph::NodeIndex> =
                    if !subckt_is_module {
                        let gate: Node = Node::new(NodeType::Gate, subckt_name.clone());
                        Some(graph.add_node(gate))
                    }
                    else { None };

                let mut sub_module_nets: HashMap<String, graph::NodeIndex> = HashMap::new();
                let mut sub_module_net_aliases: HashMap<String, String> = HashMap::new();

                for (pin_name, raw_net_name) in conns {
                    let true_net_name: &String;

                    if net_aliases.contains_key(raw_net_name) {
                        // First look in the aliases given to us by the containing module
                        true_net_name = net_aliases.get(raw_net_name).unwrap();
                    }
                    else {
                        // Then look in our private aliases that guarantee local nets won't collide with nets from other modules
                        if !local_net_aliases.contains_key(raw_net_name) {
                            let inst_count = module.inst_count;
                            let new_alias = format!("{module_name}#{inst_count}.{raw_net_name}");
                            local_net_aliases.insert(raw_net_name.clone(), new_alias);
                        }
                        true_net_name = local_net_aliases.get(raw_net_name).unwrap();
                    }

                    let net_index: graph::NodeIndex =
                        if nets.contains_key(true_net_name) {
                            *nets.get(true_net_name).unwrap()
                        }
                        else {
                            let net: Node = Node::new(NodeType::Net, true_net_name.clone());
                            let net_index = graph.add_node(net);
                            nets.insert(true_net_name.clone(), net_index);
                            net_index
                        };

                    if subckt_is_module {
                        // Submodule instances only get access to the connections
                        //  that they request for their pins
                        sub_module_nets.insert(true_net_name.clone(), net_index);
                        sub_module_net_aliases.insert(pin_name.clone(), true_net_name.clone());
                    }
                    else {
                        // We're responsible for edges for simple gates
                        if INPUT_PIN_NAMES.contains(&pin_name.as_str()) {
                            graph.add_edge(net_index, gate_index.unwrap(), pin_name.clone());
                        }
                        else if OUTPUT_PIN_NAMES.contains(&pin_name.as_str()) {
                            graph.add_edge(gate_index.unwrap(), net_index, pin_name.clone());
                        }
                        else {
                            panic!("Could not identify type of pin {pin_name}");
                        }
                    }
                }

                if subckt_is_module {
                    // Recurse for module instantiation
                    add_module_to_graph(subckt_name, modules, graph, &mut sub_module_nets, sub_module_net_aliases);
                }
            },

            ParsedPrimitive::NOP => println!("Warning: Not implemented: NOP"),
            ParsedPrimitive::Input { name: _ } => println!("Warning: Not implemented: Input"),
            ParsedPrimitive::Output { name: _ } => println!("Warning: Not implemented: Output"),
            ParsedPrimitive::Lut { inputs: _, output: _, table: _ } => println!("Ignoring LUT"),
            ParsedPrimitive::Gate { c: _, d: _, q: _, r: _, e: _ } => println!("Warning: Not implemented: Gate"),
            ParsedPrimitive::Latch { input: _, output: _, control: _, init: _ } => println!("Warning: Not implemented: Latch"),
            ParsedPrimitive::Module { name: _, inputs: _, outputs: _, elems: _ } => println!("Warning: Impossible Verilog: nested private module definition?????"),
        }
    }
}

// Graph & Node implementation
#[derive(Debug)]
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
        // write!(f, "({}, {})", self.node_type, self.name)
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
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
    elems: Vec<ParsedPrimitive>,
    inst_count: i32
}
/*
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
            ParsedPrimitive::NOP => println!("NOP"),

            ParsedPrimitive::Input { name } => println!("Input"),

            ParsedPrimitive::Output { name } => println!("Output"),

            ParsedPrimitive::Lut { inputs, output, table } => println!("Lut"),

            ParsedPrimitive::Gate { c, d, q, r, e } => println!("Gate"),

            ParsedPrimitive::Latch { input, output, control, init } => println!("Latch"),

            ParsedPrimitive::Subckt { name, conns } => {
                println!("Subckt");
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

            //_ => println!("not work"),
        }
    }

    println!("{}", graph.node_count());

    for n in graph.node_indices() {
        println!("{} ", &graph[n]);
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
                println!("Inputs:");
                inputs.iter().for_each(|a| println!(" {},", a));
                println!();

                //outputs
                println!("Outputs:");
                outputs.iter().for_each(|a| println!(" {},", a));
                println!();

                //elements
                println!("Elements:");
                print_blif_components(elems);
            }
        }
    }
}
*/