place template logic:or_gate ~1 ~ ~1
place template logic:and_gate ~1 ~ ~8
place template logic:and_gate ~8 ~ ~1
place template logic:xor_gate ~8 ~ ~8
place template logic:xor_gate ~1 ~ ~15
setblock ~2 ~-1 ~11 minecraft:redstone_lamp
setblock ~1 ~-1 ~1 minecraft:target
setblock ~1 ~-2 ~1 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~1 ~1 ~10 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~1 ~-3 ~1 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~1 ~2 ~10 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~2.5 ~0 ~11.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.y"}
summon minecraft:armor_stand ~1.5 ~0 ~1.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.y -> A"}
setblock ~9 ~-1 ~4 minecraft:redstone_lamp
setblock ~3 ~-1 ~1 minecraft:target
setblock ~3 ~-2 ~1 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~6 ~1 ~3 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~3 ~-3 ~1 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~6 ~2 ~3 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~9.5 ~0 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.z"}
summon minecraft:armor_stand ~3.5 ~0 ~1.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.z -> B"}
setblock ~2 ~-1 ~3 minecraft:redstone_lamp
setblock ~-4 ~0 ~0 minecraft:target
setblock ~-4 ~-1 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~6 ~0 ~3 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-4 ~-2 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~6 ~1 ~3 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~2.5 ~0 ~3.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.cout"}
summon minecraft:armor_stand ~-3.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cout"}
setblock ~-2 ~0 ~0 minecraft:redstone_lamp
setblock ~1 ~-1 ~15 minecraft:target
setblock ~1 ~-2 ~15 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-3 ~2 ~-15 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~1 ~-3 ~15 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-3 ~3 ~-15 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~1.5 ~0 ~15.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-2 ~0 ~0 minecraft:redstone_lamp
setblock ~1 ~-1 ~8 minecraft:target
setblock ~1 ~-2 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-3 ~2 ~-8 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~1 ~-3 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-3 ~3 ~-8 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~1.5 ~0 ~8.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-2 ~0 ~2 minecraft:redstone_lamp
setblock ~3 ~-1 ~15 minecraft:target
setblock ~3 ~-2 ~15 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-5 ~2 ~-13 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~3 ~-3 ~15 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-5 ~3 ~-13 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~3.5 ~0 ~15.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~-2 ~0 ~2 minecraft:redstone_lamp
setblock ~3 ~-1 ~8 minecraft:target
setblock ~3 ~-2 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-5 ~2 ~-6 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~3 ~-3 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-5 ~3 ~-6 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~3.5 ~0 ~8.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~2 ~-1 ~19 minecraft:redstone_lamp
setblock ~8 ~-1 ~8 minecraft:target
setblock ~8 ~-2 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-6 ~1 ~11 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~8 ~-3 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-6 ~2 ~11 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~2.5 ~0 ~19.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~8.5 ~0 ~8.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~2 ~-1 ~19 minecraft:redstone_lamp
setblock ~8 ~-1 ~1 minecraft:target
setblock ~8 ~-2 ~1 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-6 ~1 ~18 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~8 ~-3 ~1 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-6 ~2 ~18 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~2.5 ~0 ~19.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~8.5 ~0 ~1.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~-2 ~0 ~4 minecraft:redstone_lamp
setblock ~10 ~-1 ~8 minecraft:target
setblock ~10 ~-2 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-12 ~2 ~-4 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~10 ~-3 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-12 ~3 ~-4 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~10.5 ~0 ~8.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~-2 ~0 ~4 minecraft:redstone_lamp
setblock ~10 ~-1 ~1 minecraft:target
setblock ~10 ~-2 ~1 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-12 ~2 ~3 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~10 ~-3 ~1 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-12 ~3 ~3 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~10.5 ~0 ~1.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~9 ~-1 ~12 minecraft:redstone_lamp
setblock ~-4 ~0 ~2 minecraft:target
setblock ~-4 ~-1 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~13 ~0 ~10 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-4 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~13 ~1 ~10 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~9.5 ~0 ~12.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.sum"}
summon minecraft:armor_stand ~-3.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.sum"}
