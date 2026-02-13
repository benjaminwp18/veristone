use std::path::Path;
use clap::Parser;

mod timberwolf;

#[path = "./bin/make_blif.rs"]
mod make_blif;

#[path = "./bin/read_blif.rs"]
mod read_blif;

#[path = "./bin/mcfunction.rs"]
mod mcfunction;

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
    read_blif::read_blif(blif_path);
}
