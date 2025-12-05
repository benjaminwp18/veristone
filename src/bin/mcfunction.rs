use std::{io::Write, path::Path};
use std::fs::File;
use clap::Parser;

struct Gate {
    name: String,
    x: i32,
    z: i32
}

struct Wire {
    start_x: i32,
    start_z: i32,
    end_x: i32,
    end_z: i32
}

// PlacementAlgo::DumbGrid { num_cols: 4 }, RoutingAlgo::Wireless

// enum PlacementAlgo {
//     DumbGrid { num_cols: i32 },
//     // TimberWolf
// }

// enum RoutingAlgo {
//     Wireless
// }

enum WireType {
    Wireless,
    Redstone
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path to write mcfunction file")]
    src: String
}

const MCFUNCTION_PATH: &'static str = "res/logic_datapack/data/logic/function/build.mcfunction";

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
        x: 0,
        z: 8
    };
    let gates: Vec<Gate> = vec![
        and_gate,
        not_gate
    ];

    let wires: Vec<Wire> = vec![
        Wire {
            start_x: 1,
            start_z: 4,
            end_x: 0,
            end_z: 7
        }
    ];

    write_mcfunction(&gates, &wires, WireType::Wireless);
}

fn write_mcfunction(
    gates: &Vec<Gate>,
    wires: &Vec<Wire>,
    wire_type: WireType
) {
    let path: &Path = Path::new(MCFUNCTION_PATH);
    let mut file = File::create(path).unwrap();
    let file_error: &str = &format!("Failed to write gate to mcfunction file at {MCFUNCTION_PATH}");

    for gate in gates {
        writeln!(file, "place template logic:{}_gate ~{} ~ ~{}", gate.name, gate.x, gate.z).expect(file_error);
    }

    for wire in wires {
        match wire_type {
            WireType::Redstone => {
                panic!("TODO: straight-line only redstone wire??");
                // writeln!(file, "fill ~{} ~ ~{} ~{} ~ ~{} minecraft:redstone_wire", wire.start_x, wire.start_z, wire.end_x, wire.end_x).expect(file_error);
            },
            WireType::Wireless => {
                let relative_start_x = wire.start_x - wire.end_x;
                let relative_start_z = wire.start_z - wire.end_z;

                let on_command = format!("execute if block ~{relative_start_x} ~1 ~{relative_start_z} minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block");
                let off_command = format!("execute if block ~{relative_start_x} ~2 ~{relative_start_z} minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target");

                writeln!(file, "setblock ~{} ~ ~{} minecraft:redstone_lamp", wire.start_x, wire.start_z).expect(file_error);
                writeln!(file, "setblock ~{} ~ ~{} minecraft:target", wire.end_x, wire.end_z).expect(file_error);
                writeln!(file, "setblock ~{} ~-1 ~{} minecraft:repeating_command_block{{auto: 1b, Command: \"{on_command}\"}}", wire.end_x, wire.end_z).expect(file_error);
                writeln!(file, "setblock ~{} ~-2 ~{} minecraft:repeating_command_block{{auto: 1b, Command: \"{off_command}\"}}", wire.end_x, wire.end_z).expect(file_error);
            }
        }
    }

    println!("Successfully wrote mcfunction to {MCFUNCTION_PATH}");
}