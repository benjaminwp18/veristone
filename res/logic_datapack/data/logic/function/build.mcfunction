place template logic:and_gate ~49 ~ ~44
place template logic:and_gate ~96 ~ ~11
place template logic:xor_gate ~-1 ~ ~-4
place template logic:xor_gate ~-8 ~ ~-48
place template logic:or_gate ~-176 ~ ~69
setblock ~97 ~-1 ~14 minecraft:redstone_lamp
setblock ~-176 ~-1 ~69 minecraft:target
setblock ~-176 ~-2 ~69 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~273 ~1 ~-55 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-176 ~-3 ~69 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~273 ~2 ~-55 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~97.5 ~0 ~14.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.y"}
summon minecraft:armor_stand ~-175.5 ~0 ~69.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.y -> A"}
setblock ~50 ~-1 ~47 minecraft:redstone_lamp
setblock ~-174 ~-1 ~69 minecraft:target
setblock ~-174 ~-2 ~69 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~224 ~1 ~-22 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-174 ~-3 ~69 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~224 ~2 ~-22 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~50.5 ~0 ~47.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.z"}
summon minecraft:armor_stand ~-173.5 ~0 ~69.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.z -> B"}
setblock ~-175 ~-1 ~71 minecraft:redstone_lamp
setblock ~-4 ~0 ~0 minecraft:target
setblock ~-4 ~-1 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-171 ~0 ~71 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-4 ~-2 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-171 ~1 ~71 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-174.5 ~0 ~71.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.cout"}
summon minecraft:armor_stand ~-3.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cout"}
setblock ~-2 ~0 ~0 minecraft:redstone_lamp
setblock ~-1 ~-1 ~-4 minecraft:target
setblock ~-1 ~-2 ~-4 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-1 ~2 ~4 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-1 ~-3 ~-4 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-1 ~3 ~4 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~-0.5 ~0 ~-3.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-2 ~0 ~0 minecraft:redstone_lamp
setblock ~96 ~-1 ~11 minecraft:target
setblock ~96 ~-2 ~11 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-98 ~2 ~-11 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~96 ~-3 ~11 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-98 ~3 ~-11 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~96.5 ~0 ~11.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-2 ~0 ~2 minecraft:redstone_lamp
setblock ~1 ~-1 ~-4 minecraft:target
setblock ~1 ~-2 ~-4 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-3 ~2 ~6 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~1 ~-3 ~-4 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-3 ~3 ~6 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~1.5 ~0 ~-3.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~-2 ~0 ~2 minecraft:redstone_lamp
setblock ~98 ~-1 ~11 minecraft:target
setblock ~98 ~-2 ~11 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-100 ~2 ~-9 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~98 ~-3 ~11 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-100 ~3 ~-9 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~98.5 ~0 ~11.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~0 ~-1 ~0 minecraft:redstone_lamp
setblock ~-8 ~-1 ~-48 minecraft:target
setblock ~-8 ~-2 ~-48 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~8 ~1 ~48 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-8 ~-3 ~-48 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~8 ~2 ~48 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~0.5 ~0 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~-7.5 ~0 ~-47.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~0 ~-1 ~0 minecraft:redstone_lamp
setblock ~49 ~-1 ~44 minecraft:target
setblock ~49 ~-2 ~44 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-49 ~1 ~-44 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~49 ~-3 ~44 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-49 ~2 ~-44 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~0.5 ~0 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~49.5 ~0 ~44.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~-2 ~0 ~4 minecraft:redstone_lamp
setblock ~-6 ~-1 ~-48 minecraft:target
setblock ~-6 ~-2 ~-48 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~4 ~2 ~52 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-6 ~-3 ~-48 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~4 ~3 ~52 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~-5.5 ~0 ~-47.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~-2 ~0 ~4 minecraft:redstone_lamp
setblock ~51 ~-1 ~44 minecraft:target
setblock ~51 ~-2 ~44 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-53 ~2 ~-40 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~51 ~-3 ~44 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-53 ~3 ~-40 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~51.5 ~0 ~44.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~-7 ~-1 ~-44 minecraft:redstone_lamp
setblock ~-4 ~0 ~2 minecraft:target
setblock ~-4 ~-1 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-3 ~0 ~-46 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-4 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-3 ~1 ~-46 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-6.5 ~0 ~-43.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.sum"}
summon minecraft:armor_stand ~-3.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.sum"}
