use clap::Parser;

use veristone::mcfunction;

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
    let and_gate = mcfunction::Gate {
        name: String::from("and"),
        x: 0,
        z: 0
    };
    let not_gate: mcfunction::Gate = mcfunction::Gate {
        name: String::from("not"),
        x: 0,
        z: 8
    };
    let gates: Vec<mcfunction::Gate> = vec![
        and_gate,
        not_gate
    ];

    let wires: Vec<mcfunction::Wire> = vec![
        mcfunction::Wire {
            start: mcfunction::LabeledPoint {
                x: 1,
                z: 4,
                y: 0,
                label: Some("start".to_string())
            },
            end: mcfunction::LabeledPoint {
                x: 0,
                z: 7,
                y: 0,
                label: Some("end".to_string())
            }
        }
    ];

    mcfunction::write_mcfunction(&gates, &wires, mcfunction::RoutingAlgo::Wireless);
}
