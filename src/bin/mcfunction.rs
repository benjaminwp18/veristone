use std::{io::Write, path::Path};
use std::fs::File;
use clap::Parser;

struct Gate {
    name: String
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
    let path = Path::new("build.mcfunction");
    let and_gate = Gate {
        name: String::from("and_gate")
    };
    let not_gate = Gate {
        name: String::from("not_gate")
    };
    let gates: Vec<(i32, i32, Gate)> = vec![
        (0, 0, and_gate),
        (0, 4, not_gate)
    ];

    create_file(path, gates);
}

fn create_file (path: &Path, gates: Vec<(i32, i32, Gate)>) {
    let mut file = File::create(path).unwrap();

    for gate_info in &gates {
        let (x, y, gate) = gate_info;
        let _ = file.write(format!("place template logic:{} ~{x} ~-1 ~{y}\n", gate.name).as_bytes());
    }
}