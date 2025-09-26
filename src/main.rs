use blif_parser::*;
use primitives::ParsedPrimitive;

fn main() {
    let list = parser::parse_blif_file("~/veristone/counter.blif");
    for x in list.ok().unwrap().into_iter() {
        match x {
            ParsedPrimitive::Gate { c, d, q, r, e } => {
                print!("{}", c);
            }
            _ => todo!(),
        }
    }
}
