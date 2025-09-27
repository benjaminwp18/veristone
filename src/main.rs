use blif_parser::*;
use primitives::ParsedPrimitive;
use std::path::{self, Path};

#[allow(unused_variables)]
fn main() {
    let path = Path::new("counter.blif");
    let binding = path::absolute(path).unwrap().into_os_string();
    let full_path = binding.to_str().unwrap();

    println!("{}", full_path);

    let list = parser::parse_blif_file(full_path).unwrap();
    print_blif_components(list);
}

#[allow(unused_variables)]
fn print_blif_components(list: Vec<ParsedPrimitive>) {
    for x in list.into_iter() {
        match x {
            ParsedPrimitive::NOP => println!("NOP"),
            ParsedPrimitive::Input { name } => println!("{}", name),
            ParsedPrimitive::Output { name } => println!("{}", name),
            ParsedPrimitive::Lut {
                inputs,
                output,
                table,
            } => println!("{}", output),
            ParsedPrimitive::Gate { c, d, q, r, e } => println!("{}", c),
            ParsedPrimitive::Latch {
                input,
                output,
                control,
                init,
            } => println!("{}", input),
            ParsedPrimitive::Subckt { name, conns } => println!("Subckt name: {}", name),
            ParsedPrimitive::Module {
                name,
                inputs,
                outputs,
                elems,
            } => {
                println!("Module name: {}", name);

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
