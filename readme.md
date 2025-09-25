# Veristone

Minecraft Verilog synthesizer

## Stages
 - Verilog -> Yosys -> netlist (graph data structure)
 - Netlist -> our code -> mcfunction

## Resources
### Python
 - https://doc.rust-lang.org/stable/rust-by-example/index.html
    - Backup: Python

### Yosys
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

 - http://sigtbd.csail.mit.edu/ (doesn't load???)
 - https://github.com/qmn/pershing
 - https://github.com/qmn/dewey

## Setup

```bash
sudo apt-get install iverilog yosys
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Testing

(g2012 enables systemverilog)
```bash
iverilog -g2012 adder.v -o adder
```

Example Usage

Yosys is controlled using synthesis scripts. For example, the following Yosys synthesis script reads a design (with the top module mytop) from the verilog file mydesign.v, synthesizes it to a gate-level netlist using the cell library in the Liberty file mycells.lib and writes the synthesized results as Verilog netlist to synth.v:

```bash
# read design
read_verilog mydesign.v

# elaborate design hierarchy
hierarchy -check -top mytop

# the high-level stuff
proc; opt; fsm; opt; memory; opt

# mapping to internal cell library
techmap; opt

# mapping flip-flops to mycells.lib
dfflibmap -liberty mycells.lib

# mapping logic to mycells.lib
abc -liberty mycells.lib

# cleanup
clean

# write synthesized design
write_verilog synth.v
```

The synth command provides a good default script that can be used as basis for simple synthesis scripts:

```bash
# read design
read_verilog mydesign.v

# generic synthesis
synth -top mytop

# mapping to mycells.lib
dfflibmap -liberty mycells.lib
abc -liberty mycells.lib
clean

# write synthesized design
write_verilog synth.v
```

See help synth for details on the synth command. 
