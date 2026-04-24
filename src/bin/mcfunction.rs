use clap::Parser;
use veristone::{mcfunction, points};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path to write mcfunction file")]
    src: String
}

#[allow(dead_code)]
fn main() {
    // let args: Args = Args::parse();
    // let path = Path::new(&args.src);
    let and_gate = points::Gate {
        name: String::from("and"),
        x: 0,
        z: 0,
        y: 0
    };
    let not_gate: points::Gate = points::Gate {
        name: String::from("not"),
        x: 0,
        z: 8,
        y: 0
    };
    let gates: Vec<points::Gate> = vec![
        and_gate,
        not_gate
    ];

    let wires: Vec<points::Wire> = vec![
        points::Wire {
            start: points::LabeledPoint {
                x: 1,
                z: 4,
                y: 0,
                label: Some("start".to_string())
            },
            ends: vec![
                points::LabeledPoint {
                    x: 0,
                    z: 7,
                    y: 0,
                    label: Some("end".to_string())
                }
            ]
        }
    ];

    mcfunction::write_mcfunction(&gates, &wires, &vec![mcfunction::GateRule::Template], mcfunction::RoutingAlgo::Wireless).unwrap();
}
