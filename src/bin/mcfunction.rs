use std::{io::Write, path::Path};
use std::fs::File;
use clap::Parser;
use serde_json;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub struct Gate {
    pub name: String,
    pub x: i32,
    pub z: i32,
    pub y: i32
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

pub fn read_gate_info() -> HashMap<String, GateInfo> {
    let gate_info_str = fs::read_to_string("res/gate_info.json").expect("Failed to read gate info file");
    return serde_json::from_str(&gate_info_str).expect("Gate info JSON not well formatted");
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
        z: 0,
        y: 0
    };
    let not_gate: Gate = Gate {
        name: String::from("not"),
        x: 0,
        z: 8,
        y: 0
    };
    let gates: Vec<Gate> = vec![
        and_gate,
        not_gate
    ];

    let wires: Vec<Wire> = vec![
        Wire {
            start: LabeledPoint {
                x: 1,
                z: 4,
                y: 0,
                label: Some("start".to_string())
            },
            end: LabeledPoint {
                x: 0,
                z: 7,
                y: 0,
                label: Some("end".to_string())
            }
        }
    ];

    write_mcfunction(&gates, &wires, WireType::Wireless);
}

pub fn write_mcfunction(
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

    match wire_type {
        WireType::Redstone => {
            // Find size of allowed volume for wires
            let mut x_min: i32 =  2147483647;
            let mut x_max: i32 = -2147483647;
            let mut z_min: i32 =  2147483647;
            let mut z_max: i32 = -2147483647;

            for wire in wires { // scuffed loop for dimensions
                if wire.start.x < x_min {
                    x_min = wire.start.x;
                }
                if wire.start.x > x_max {
                    x_min = wire.start.x;
                }
                if wire.start.z < z_min {
                    z_min = wire.start.z;
                }
                if wire.start.z > z_max {
                    x_min = wire.start.z;
                }
                if wire.end.x < x_min {
                    x_min = wire.end.x;
                }
                if wire.end.x > x_max {
                    x_min = wire.end.x;
                }
                if wire.end.z < z_min {
                    z_min = wire.end.z;
                }
                if wire.end.z > z_max {
                    z_min = wire.end.z;
                }
            }

            // volume formatted [x][y][z] as per minecraft standard
            let x_size: usize = (x_max - x_min).try_into().unwrap();
            let z_size: usize = (z_max - z_min).try_into().unwrap();
            let y_size: usize = (if x_size < z_size { z_size * 2 } else { x_size * 2 }).try_into().unwrap();

            let mut wire_volume: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; x_size]; y_size]; z_size];

            let gate_info = read_gate_info();

            for gate in gates {
                let info = gate_info.get(&gate.name).unwrap();

                for i in 0..info.x_dim {
                    if gate.x + i < wire_volume.capacity() {
                        wire_volume
                            [(gate.x + i) as usize]
                            [gate.y as usize]
                            [gate.z as usize]
                        = -1;
                    }
                }
                for i in 0..info.z_dim {
                    if gate.z + i < wire_volume[0][0].capacity() {
                        wire_volume
                            [gate.x as usize]
                            [gate.y as usize]
                            [(gate.z + i) as usize]
                        = -1;
                    }
                }
                for i in 0..info.y_dim {
                    if gate.y + i < wire_volume[0].capacity() {
                        wire_volume
                            [gate.x as usize]
                            [(gate.y + i) as usize]
                            [gate.z as usize]
                        = -1;
                    }
                }
            }


            for wire in wires {

            }
            // writeln!(file, "fill ~{} ~ ~{} ~{} ~ ~{} minecraft:redstone_wire", wire.start.x, wire.start.z, wire.end.x, wire.end.x).expect(file_error);
        },
        WireType::Wireless => {
            for wire in wires {
                let relative_start_x = wire.start.x - wire.end.x;
                let relative_start_z = wire.start.z - wire.end.z;

                let on_command = format!("execute if block ~{relative_start_x} ~{} ~{relative_start_z} minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block", 1 + wire.start.y - wire.end.y);
                let off_command = format!("execute if block ~{relative_start_x} ~{} ~{relative_start_z} minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target", 2 + wire.start.y - wire.end.y);

                writeln!(file, "setblock ~{} ~{} ~{} minecraft:redstone_lamp", wire.start.x, wire.start.y, wire.start.z).expect(file_error);
                writeln!(file, "setblock ~{} ~{} ~{} minecraft:target", wire.end.x, wire.end.y, wire.end.z).expect(file_error);
                writeln!(file, "setblock ~{} ~{} ~{} minecraft:repeating_command_block{{auto: 1b, Command: \"{on_command}\"}}", wire.end.x, wire.end.y - 1, wire.end.z).expect(file_error);
                writeln!(file, "setblock ~{} ~{} ~{} minecraft:repeating_command_block{{auto: 1b, Command: \"{off_command}\"}}", wire.end.x, wire.end.y - 2, wire.end.z).expect(file_error);

                if wire.start.label.is_some() {
                    writeln!(file, "summon minecraft:armor_stand ~{} ~{} ~{} {{Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: \"{}\"}}", (wire.start.x as f32) + 0.5, wire.start.y + 1, (wire.start.z as f32) + 0.5, wire.start.label.as_ref().unwrap()).expect(file_error);
                }
                if wire.end.label.is_some() {
                    writeln!(file, "summon minecraft:armor_stand ~{} ~{} ~{} {{Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: \"{}\"}}", (wire.end.x as f32) + 0.5, wire.end.y + 1, (wire.end.z as f32) + 0.5, wire.end.label.as_ref().unwrap()).expect(file_error);
                }
            }
        }
    }

    println!("Successfully wrote mcfunction to {MCFUNCTION_PATH}");
}
