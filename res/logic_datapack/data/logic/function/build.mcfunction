place template logic:and_gate ~0 ~ ~0
place template logic:not_gate ~0 ~ ~8
setblock ~1 ~ ~4 minecraft:redstone_lamp
setblock ~0 ~ ~7 minecraft:target
setblock ~0 ~-1 ~7 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~1 ~1 ~-3 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~0 ~-2 ~7 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~1 ~2 ~-3 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
