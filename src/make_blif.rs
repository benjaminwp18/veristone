use std::path;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn generate_blif(verilog_path: &Path, lib_path: &Path, blif_path: &Path, verbose: bool) {
    let abs_verilog_path = path::absolute(verilog_path).unwrap().into_os_string().into_string().unwrap();
    let abs_lib_path = path::absolute(lib_path).unwrap().into_os_string().into_string().unwrap();
    let abs_blif_path = path::absolute(blif_path).unwrap().into_os_string().into_string().unwrap();

    println!("\n\n=== MAKE BLIF ===\n");
    println!("Converting {abs_verilog_path}");
    println!("\tto {abs_blif_path}");
    println!("\twith liberty file {abs_lib_path}");

    let output = Command::new("yosys")
        .arg("-p")
        .arg(format!("read_verilog \"{abs_verilog_path}\";
                      hierarchy -check -auto-top;
                      proc; opt; fsm; opt; memory; opt;
                      techmap; opt; dfflibmap -liberty \"{abs_lib_path}\";
                      abc -liberty \"{abs_lib_path}\";
                      clean; write_blif \"{abs_blif_path}\""))
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

pub fn make_blif_path(verilog_path: &Path) -> PathBuf {
    let filename = verilog_path.file_stem().unwrap().to_owned().into_string().unwrap();
    let blif_buf = Path::new("res/blif").join(filename + ".blif");
    return blif_buf;
}
