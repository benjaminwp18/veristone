use std::path;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn generate_blif(verilog_path: &Path, lib_path: &Path, blif_path: &Path, verbose: bool) {
    let abs_verilog_path = path::absolute(verilog_path).unwrap().into_os_string().into_string().unwrap();
    let abs_lib_path = path::absolute(lib_path).unwrap().into_os_string().into_string().unwrap();
    let abs_blif_path = path::absolute(blif_path).unwrap().into_os_string().into_string().unwrap();

    println!("Converting {abs_verilog_path} to {abs_blif_path} with liberty file {abs_lib_path}");

    let output = Command::new("yosys")
        .arg("-p")
        .arg(format!("read_verilog {abs_verilog_path};
                      hierarchy -check -auto-top;
                      proc; opt; fsm; opt; memory; opt;
                      techmap; opt; dfflibmap -liberty {abs_lib_path};
                      abc -liberty {abs_lib_path};
                      clean; write_blif {abs_blif_path}"))
        .output()
        .expect("Failed to generate blif");
    if verbose {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }
    if output.stderr.len() > 0 {
        eprintln!("YOSYS had stderr output:");
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());
    }
}

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

pub fn make_blif_path(verilog_path: &Path) -> PathBuf {
    let filename = verilog_path.file_stem().unwrap().to_owned().into_string().unwrap();
    let blif_buf = Path::new("res/blif").join(filename + ".blif");
    return blif_buf;
}

#[allow(dead_code)]
fn main() {
    let args = Args::parse();

    let verilog_path = Path::new(&args.src);
    let lib_path = Path::new(&args.lib);
    let blif_buf = make_blif_path(verilog_path);
    let blif_path = blif_buf.as_path();

    generate_blif(verilog_path, lib_path, &blif_path, args.verbose);
}
