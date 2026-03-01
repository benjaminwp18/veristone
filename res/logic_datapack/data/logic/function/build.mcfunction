place template logic:and_gate ~-1 ~ ~12
place template logic:or_gate ~-16 ~ ~92
place template logic:and_gate ~2 ~ ~24
place template logic:xor_gate ~8 ~ ~5
place template logic:xor_gate ~1 ~ ~2
setblock ~0 ~-1 ~15 minecraft:redstone_lamp
setblock ~-16 ~-1 ~92 minecraft:target
setblock ~-16 ~-2 ~92 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~16 ~1 ~-77 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-16 ~-3 ~92 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~16 ~2 ~-77 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~0.5 ~0 ~15.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.y"}
summon minecraft:armor_stand ~-15.5 ~0 ~92.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.y -> A"}
setblock ~3 ~-1 ~27 minecraft:redstone_lamp
setblock ~-14 ~-1 ~92 minecraft:target
setblock ~-14 ~-2 ~92 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~17 ~1 ~-65 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-14 ~-3 ~92 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~17 ~2 ~-65 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~3.5 ~0 ~27.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.z"}
summon minecraft:armor_stand ~-13.5 ~0 ~92.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.z -> B"}
setblock ~-15 ~-1 ~94 minecraft:redstone_lamp
setblock ~-6 ~0 ~0 minecraft:target
setblock ~-6 ~-1 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-9 ~0 ~94 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-6 ~-2 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-9 ~1 ~94 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-14.5 ~0 ~94.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.cout"}
summon minecraft:armor_stand ~-5.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cout"}
setblock ~-4 ~0 ~0 minecraft:redstone_lamp
setblock ~1 ~-1 ~2 minecraft:target
setblock ~1 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-5 ~2 ~-2 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~1 ~-3 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-5 ~3 ~-2 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~1.5 ~0 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-4 ~0 ~0 minecraft:redstone_lamp
setblock ~-1 ~-1 ~12 minecraft:target
setblock ~-1 ~-2 ~12 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-3 ~2 ~-12 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-1 ~-3 ~12 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-3 ~3 ~-12 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~-0.5 ~0 ~12.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-4 ~0 ~2 minecraft:redstone_lamp
setblock ~3 ~-1 ~2 minecraft:target
setblock ~3 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-7 ~2 ~0 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~3 ~-3 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-7 ~3 ~0 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~3.5 ~0 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~-4 ~0 ~2 minecraft:redstone_lamp
setblock ~1 ~-1 ~12 minecraft:target
setblock ~1 ~-2 ~12 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-5 ~2 ~-10 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~1 ~-3 ~12 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-5 ~3 ~-10 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~1.5 ~0 ~12.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~2 ~-1 ~6 minecraft:redstone_lamp
setblock ~8 ~-1 ~5 minecraft:target
setblock ~8 ~-2 ~5 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-6 ~1 ~1 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~8 ~-3 ~5 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-6 ~2 ~1 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~2.5 ~0 ~6.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~8.5 ~0 ~5.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~2 ~-1 ~6 minecraft:redstone_lamp
setblock ~2 ~-1 ~24 minecraft:target
setblock ~2 ~-2 ~24 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~0 ~1 ~-18 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~2 ~-3 ~24 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~0 ~2 ~-18 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~2.5 ~0 ~6.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~2.5 ~0 ~24.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~-4 ~0 ~4 minecraft:redstone_lamp
setblock ~10 ~-1 ~5 minecraft:target
setblock ~10 ~-2 ~5 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-14 ~2 ~-1 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~10 ~-3 ~5 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-14 ~3 ~-1 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~10.5 ~0 ~5.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~-4 ~0 ~4 minecraft:redstone_lamp
setblock ~4 ~-1 ~24 minecraft:target
setblock ~4 ~-2 ~24 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-8 ~2 ~-20 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~4 ~-3 ~24 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-8 ~3 ~-20 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-3.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~4.5 ~0 ~24.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~9 ~-1 ~9 minecraft:redstone_lamp
setblock ~-6 ~0 ~2 minecraft:target
setblock ~-6 ~-1 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~15 ~0 ~7 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-6 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~15 ~1 ~7 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~9.5 ~0 ~9.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.sum"}
summon minecraft:armor_stand ~-5.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.sum"}
