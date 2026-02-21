use std::{io::Write, path::Path};
use std::fs::File;
use serde_json;
use std::collections::HashMap;
use std::fs;
use serde::Deserialize;

const MCFUNCTION_PATH: &'static str = "res/logic_datapack/data/logic/function/build.mcfunction";

#[derive(Clone)]
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

pub enum RoutingAlgo {
    Wireless
}

pub fn read_gate_info() -> HashMap<String, GateInfo> {
    let gate_info_str = fs::read_to_string("res/gate_info.json").expect("Failed to read gate info file");
    return serde_json::from_str(&gate_info_str).expect("Gate info JSON not well formatted");
}

pub fn write_mcfunction(
    gates: &Vec<Gate>,
    wires: &Vec<Wire>,
    routing_algo: RoutingAlgo
) {
    println!("\n\n=== MCFUNCTION ===\n");

    let path: &Path = Path::new(MCFUNCTION_PATH);
    let mut file = File::create(path).unwrap();
    let file_error: &str = &format!("Failed to write gate to mcfunction file at {MCFUNCTION_PATH}");

    for gate in gates {
        writeln!(file, "place template logic:{}_gate ~{} ~ ~{}", gate.name, gate.x, gate.z).expect(file_error);
    }

    for wire in wires {
        match routing_algo {
            RoutingAlgo::Wireless => {
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
