use std::{io::Write, path::Path};
use std::fs::File;
use clap::Parser;

struct Gate {
    name: String,
    x: i32,
    z: i32
}

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
    let and_gate = Gate {
        name: String::from("and"),
        x: 0,
        z: 0
    };
    let not_gate: Gate = Gate {
        name: String::from("not"),
        x: 1,
        z: 4
    };
    let gates: Vec<Gate> = vec![
        and_gate,
        not_gate
    ];

    create_file(gates);
}

fn create_file (gates: Vec<Gate>) {
    let path: &Path = Path::new("res/logic_datapack/data/logic/function/build.mcfunction");
    let mut file = File::create(path).unwrap();

    for gate in &gates {
        let _ = file.write(format!("place template logic:{}_gate ~{} ~ ~{}\n", gate.name, gate.x, gate.z).as_bytes());
    }
}