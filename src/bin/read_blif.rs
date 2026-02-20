use std::path::Path;
use clap::Parser;

use veristone::read_blif;

#[path = "../timberwolf.rs"]
mod timberwolf;

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
    read_blif::read_blif(blif_path, read_blif::PlacementAlgo::DumbGrid { num_cols: 4 });
}
