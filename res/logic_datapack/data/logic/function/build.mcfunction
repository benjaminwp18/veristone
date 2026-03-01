place template logic:and_gate ~105 ~ ~-124
place template logic:xor_gate ~-19 ~ ~99
place template logic:xor_gate ~-31 ~ ~31
place template logic:or_gate ~121 ~ ~-141
place template logic:and_gate ~-53 ~ ~-14
setblock ~106 ~-1 ~-121 minecraft:redstone_lamp
setblock ~121 ~-1 ~-141 minecraft:target
setblock ~121 ~-2 ~-141 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-15 ~1 ~20 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~121 ~-3 ~-141 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-15 ~2 ~20 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~106.5 ~0 ~-120.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.y"}
summon minecraft:armor_stand ~121.5 ~0 ~-140.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.y -> A"}
setblock ~-52 ~-1 ~-11 minecraft:redstone_lamp
setblock ~123 ~-1 ~-141 minecraft:target
setblock ~123 ~-2 ~-141 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-175 ~1 ~130 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~123 ~-3 ~-141 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-175 ~2 ~130 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-51.5 ~0 ~-10.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.z"}
summon minecraft:armor_stand ~123.5 ~0 ~-140.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.z -> B"}
setblock ~122 ~-1 ~-139 minecraft:redstone_lamp
setblock ~-4 ~0 ~0 minecraft:target
setblock ~-4 ~-1 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~126 ~0 ~-139 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-4 ~-2 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~126 ~1 ~-139 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~122.5 ~0 ~-138.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.cout"}
summon minecraft:armor_stand ~-3.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cout"}
setblock ~-2 ~0 ~0 minecraft:redstone_lamp
setblock ~-31 ~-1 ~31 minecraft:target
setblock ~-31 ~-2 ~31 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~29 ~2 ~-31 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-31 ~-3 ~31 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~29 ~3 ~-31 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~-30.5 ~0 ~31.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-2 ~0 ~0 minecraft:redstone_lamp
setblock ~105 ~-1 ~-124 minecraft:target
setblock ~105 ~-2 ~-124 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-107 ~2 ~124 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~105 ~-3 ~-124 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-107 ~3 ~124 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~105.5 ~0 ~-123.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-2 ~0 ~2 minecraft:redstone_lamp
setblock ~-29 ~-1 ~31 minecraft:target
setblock ~-29 ~-2 ~31 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~27 ~2 ~-29 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-29 ~-3 ~31 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~27 ~3 ~-29 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~-28.5 ~0 ~31.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~-2 ~0 ~2 minecraft:redstone_lamp
setblock ~107 ~-1 ~-124 minecraft:target
setblock ~107 ~-2 ~-124 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-109 ~2 ~126 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~107 ~-3 ~-124 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-109 ~3 ~126 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~107.5 ~0 ~-123.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~-30 ~-1 ~35 minecraft:redstone_lamp
setblock ~-19 ~-1 ~99 minecraft:target
setblock ~-19 ~-2 ~99 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-11 ~1 ~-64 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-19 ~-3 ~99 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-11 ~2 ~-64 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-29.5 ~0 ~35.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~-18.5 ~0 ~99.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~-30 ~-1 ~35 minecraft:redstone_lamp
setblock ~-53 ~-1 ~-14 minecraft:target
setblock ~-53 ~-2 ~-14 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~23 ~1 ~49 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-53 ~-3 ~-14 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~23 ~2 ~49 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-29.5 ~0 ~35.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~-52.5 ~0 ~-13.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~-2 ~0 ~4 minecraft:redstone_lamp
setblock ~-17 ~-1 ~99 minecraft:target
setblock ~-17 ~-2 ~99 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~15 ~2 ~-95 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-17 ~-3 ~99 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~15 ~3 ~-95 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~-16.5 ~0 ~99.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~-2 ~0 ~4 minecraft:redstone_lamp
setblock ~-51 ~-1 ~-14 minecraft:target
setblock ~-51 ~-2 ~-14 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~49 ~2 ~18 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-51 ~-3 ~-14 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~49 ~3 ~18 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~-50.5 ~0 ~-13.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~-18 ~-1 ~103 minecraft:redstone_lamp
setblock ~-4 ~0 ~2 minecraft:target
setblock ~-4 ~-1 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-14 ~0 ~101 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-4 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-14 ~1 ~101 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-17.5 ~0 ~103.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.sum"}
summon minecraft:armor_stand ~-3.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.sum"}
