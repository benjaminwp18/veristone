use std::path::Path;
use clap::Parser;

use veristone::{lee, make_blif, mcfunction, points, read_blif};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path (relative) to verilog file")]
    src: String
}

fn main() {
    let args = Args::parse();
    let verilog_path = Path::new(&args.src);
    let blif_buf = make_blif::make_blif_path(verilog_path);
    let blif_path = blif_buf.as_path();
    make_blif::generate_blif(verilog_path, Path::new("res/mc.lib"), blif_path, false);
    let (gates, wires, inputs, outputs) = read_blif::read_blif(blif_path, read_blif::PlacementAlgo::TimberWolf, false);
    mcfunction::write_mcfunction(
        &gates, &wires,
        &vec![mcfunction::GateRule::Template],
        mcfunction::RoutingAlgo::Lee(lee::LeeSettings {
            padding: points::Point { x: 3, y: 5, z: 3 },
            do_rerouting: false
        }),
        &inputs,
        &outputs
    ).unwrap();
}
