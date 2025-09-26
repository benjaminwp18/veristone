use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    verilog: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = "mc.lib")]
    lib: String,
}

fn main() {
    let args = Args::parse();

    std::Path
}