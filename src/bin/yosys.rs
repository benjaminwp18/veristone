use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    verilog: String,

    #[arg(short, long, default_value = "res/mc.lib")]
    lib: String,
}

fn main() {
    let args = Args::parse();

    let verilog_path = Path::new(&args.verilog);
    let lib_path = Path::new(&args.lib);

    dbg!(verilog_path, lib_path);
}