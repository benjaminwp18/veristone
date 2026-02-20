use clap::Parser;
use std::path::Path;

use veristone::make_blif;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path (relative) to verilog source file")]
    src: String,

    #[arg(short, long, default_value = "res/mc.lib",
          help = "Path (relative) to the liberty gate library file")]
    lib: String,

    #[arg(short, long, action, help = "Whether to view stdout from YOSYS")]
    verbose: bool,
}

#[allow(dead_code)]
fn main() {
    let args = Args::parse();

    let verilog_path = Path::new(&args.src);
    let lib_path = Path::new(&args.lib);
    let blif_buf = make_blif::make_blif_path(verilog_path);
    let blif_path = blif_buf.as_path();

    make_blif::generate_blif(verilog_path, lib_path, &blif_path, args.verbose);
}
