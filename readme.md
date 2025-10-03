# Veristone

Minecraft Verilog synthesizer

## Usage

```bash
cargo build
cargo run --bin yosys -- -s res/verilog/adder.v
cargo run --bin veristone
```

## Stages
 - Verilog -> Yosys -> netlist (graph data structure)
 - Netlist -> our code -> mcfunction

## Resources
### Rust
 - https://doc.rust-lang.org/stable/rust-by-example/index.html
    - Backup: Python

### Verilog & Yosys
 - https://yosyshq.net/yosys/
 - https://blog.eowyn.net/yosys/CHAPTER_Approach.html

### Minecraft
 - https://minecraft.wiki/w/Function_(Java_Edition)
 - https://minecraft.wiki/w/Commands/setblock

## Previous Projects
 - https://github.com/itsfrank/MinecraftHDL
 - https://github.com/MinecraftMachina/FabricHDL
 - https://github.com/Kenny2github/V2MC
 - https://github.com/InputBlackBoxOutput/Redstone-HDL
 - https://github.com/PietPtr/verilog2minecraft
 - https://github.com/google/minetest_pnr

### qmn
 - http://sigtbd.csail.mit.edu/ (doesn't load???)
 - https://github.com/qmn/pershing
 - https://github.com/qmn/dewey

## Setup

```bash
sudo apt-get install iverilog yosys
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Verilog/Yosys notes

Basic usage of [Icarus Verilog](https://github.com/steveicarus/iverilog):

(g2012 enables systemverilog)
```bash
iverilog -g2012 adder.v -o adder
```

Links for generating BLIF netlists with Yosys:
 - https://github.com/qmn/dewey/blob/master/scripts/yosys.sh
 - https://github.com/qmn/dewey/blob/master/quan.lib
 - https://yosyshq.readthedocs.io/projects/yosys/en/latest/appendix/APPNOTE_010_Verilog_to_BLIF.html
 - https://course.ece.cmu.edu/~ee760/760docs/blif.pdf
 - https://lib.rs/crates/blif-parser
