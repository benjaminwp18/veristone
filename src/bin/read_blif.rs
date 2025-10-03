use std::path::{self, Path};
use clap::Parser;
use blif_parser::*;
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
    print_blif_components(list);
}

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
