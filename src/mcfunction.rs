use std::{io::Write, path::Path};
use std::fs::File;

use crate::{lee, points};

const MCFUNCTION_PATH: &'static str =
    "res/logic_datapack/data/logic/function/build.mcfunction";
pub const FILE_ERROR: &'static str =
    "Failed to write gate to mcfunction file at res/logic_datapack/data/logic/function/build.mcfunction";

pub enum RoutingAlgo {
    Wireless,
    Lee(lee::LeeSettings)
}

pub fn write_mcfunction(
    gates: &Vec<points::Gate>,
    wires: &Vec<points::Wire>,
    routing_algo: RoutingAlgo
) -> Result<(), Box<dyn std::error::Error>> {
    let path: &Path = Path::new(MCFUNCTION_PATH);
    let mut file = File::create(path).unwrap();

    // place gates in the world
    for gate in gates {
        writeln!(file, "place template logic:{}_gate ~{} ~ ~{}",
                 gate.name, gate.x, gate.z).expect(FILE_ERROR);
    }

    match routing_algo {
        RoutingAlgo::Lee(settings) => {
            lee::lee(&mut file, &settings, gates, wires)?
        },
        RoutingAlgo::Wireless => {
            for wire in wires {
                for end in &wire.ends {
                    let relative_start_x = wire.start.x - end.x;
                    let relative_start_z = wire.start.z - end.z;

                    let on_command = format!("execute if block ~{relative_start_x} ~{} ~{relative_start_z} minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block", 1 + wire.start.y - end.y);
                    let off_command = format!("execute if block ~{relative_start_x} ~{} ~{relative_start_z} minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target", 2 + wire.start.y - end.y);

                    writeln!(file, "setblock ~{} ~{} ~{} minecraft:redstone_lamp", wire.start.x, wire.start.y - 1, wire.start.z).expect(FILE_ERROR);
                    writeln!(file, "setblock ~{} ~{} ~{} minecraft:target", end.x, end.y - 1, end.z).expect(FILE_ERROR);
                    writeln!(file, "setblock ~{} ~{} ~{} minecraft:repeating_command_block{{auto: 1b, Command: \"{on_command}\"}}", end.x, end.y - 2, end.z).expect(FILE_ERROR);
                    writeln!(file, "setblock ~{} ~{} ~{} minecraft:repeating_command_block{{auto: 1b, Command: \"{off_command}\"}}", end.x, end.y - 3, end.z).expect(FILE_ERROR);

                    if wire.start.label.is_some() {
                        writeln!(file, "summon minecraft:armor_stand ~{} ~{} ~{} {{Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: \"{}\"}}", (wire.start.x as f32) + 0.5, wire.start.y + 1, (wire.start.z as f32) + 0.5, wire.start.label.as_ref().unwrap()).expect(FILE_ERROR);
                    }
                    if end.label.is_some() {
                        writeln!(file, "summon minecraft:armor_stand ~{} ~{} ~{} {{Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: \"{}\"}}", (end.x as f32) + 0.5, end.y + 1, (end.z as f32) + 0.5, end.label.as_ref().unwrap()).expect(FILE_ERROR);
                    }
                }
            }
        }
    }

    println!("Successfully wrote mcfunction to {MCFUNCTION_PATH}");

    Ok(())
}
