place template logic:and_gate ~11 ~ ~2
place template logic:or_gate ~76 ~ ~11
place template logic:xor_gate ~4 ~ ~0
place template logic:xor_gate ~0 ~ ~4
place template logic:and_gate ~57 ~ ~8
setblock ~12 ~-1 ~5 minecraft:redstone_lamp
setblock ~76 ~-1 ~11 minecraft:target
setblock ~76 ~-2 ~11 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-64 ~1 ~-6 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~76 ~-3 ~11 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-64 ~2 ~-6 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~12.5 ~0 ~5.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.y"}
summon minecraft:armor_stand ~76.5 ~0 ~11.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.y -> A"}
setblock ~58 ~-1 ~11 minecraft:redstone_lamp
setblock ~78 ~-1 ~11 minecraft:target
setblock ~78 ~-2 ~11 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-20 ~1 ~0 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~78 ~-3 ~11 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-20 ~2 ~0 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~58.5 ~0 ~11.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.z"}
summon minecraft:armor_stand ~78.5 ~0 ~11.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.z -> B"}
setblock ~77 ~-1 ~13 minecraft:redstone_lamp
setblock ~-6 ~0 ~0 minecraft:target
setblock ~-6 ~-1 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~83 ~0 ~13 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-6 ~-2 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~83 ~1 ~13 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~77.5 ~0 ~13.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.cout"}
summon minecraft:armor_stand ~-5.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cout"}
setblock ~-4 ~0 ~0 minecraft:redstone_lamp
setblock ~4 ~-1 ~0 minecraft:target
setblock ~4 ~-2 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-8 ~2 ~0 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~4 ~-3 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-8 ~3 ~0 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~4.5 ~0 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-4 ~0 ~0 minecraft:redstone_lamp
setblock ~11 ~-1 ~2 minecraft:target
setblock ~11 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-15 ~2 ~-2 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~11 ~-3 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-15 ~3 ~-2 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~11.5 ~0 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-4 ~0 ~2 minecraft:redstone_lamp
setblock ~6 ~-1 ~0 minecraft:target
setblock ~6 ~-2 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-10 ~2 ~2 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~6 ~-3 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-10 ~3 ~2 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~6.5 ~0 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~-4 ~0 ~2 minecraft:redstone_lamp
setblock ~13 ~-1 ~2 minecraft:target
setblock ~13 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-17 ~2 ~0 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~13 ~-3 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-17 ~3 ~0 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~13.5 ~0 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~5 ~-1 ~4 minecraft:redstone_lamp
setblock ~0 ~-1 ~4 minecraft:target
setblock ~0 ~-2 ~4 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~5 ~1 ~0 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~0 ~-3 ~4 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~5 ~2 ~0 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~5.5 ~0 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~0.5 ~0 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~5 ~-1 ~4 minecraft:redstone_lamp
setblock ~57 ~-1 ~8 minecraft:target
setblock ~57 ~-2 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-52 ~1 ~-4 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~57 ~-3 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-52 ~2 ~-4 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~5.5 ~0 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~57.5 ~0 ~8.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~-4 ~0 ~4 minecraft:redstone_lamp
setblock ~2 ~-1 ~4 minecraft:target
setblock ~2 ~-2 ~4 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-6 ~2 ~0 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~2 ~-3 ~4 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-6 ~3 ~0 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~2.5 ~0 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~-4 ~0 ~4 minecraft:redstone_lamp
setblock ~59 ~-1 ~8 minecraft:target
setblock ~59 ~-2 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-63 ~2 ~-4 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~59 ~-3 ~8 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-63 ~3 ~-4 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~59.5 ~0 ~8.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~1 ~-1 ~8 minecraft:redstone_lamp
setblock ~-6 ~0 ~2 minecraft:target
setblock ~-6 ~-1 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~7 ~0 ~6 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-6 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~7 ~1 ~6 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~1.5 ~0 ~8.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.sum"}
summon minecraft:armor_stand ~-5.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.sum"}
