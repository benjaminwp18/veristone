place template logic:and_gate ~116 ~ ~34
place template logic:or_gate ~53 ~ ~43
place template logic:and_gate ~32 ~ ~49
place template logic:xor_gate ~15 ~ ~2
place template logic:xor_gate ~39 ~ ~14
setblock ~117 ~-1 ~37 minecraft:redstone_lamp
setblock ~53 ~-1 ~43 minecraft:target
setblock ~53 ~-2 ~43 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~64 ~1 ~-6 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~53 ~-3 ~43 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~64 ~2 ~-6 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~117.5 ~0 ~37.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.y"}
summon minecraft:armor_stand ~53.5 ~0 ~43.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.y -> A"}
setblock ~33 ~-1 ~52 minecraft:redstone_lamp
setblock ~55 ~-1 ~43 minecraft:target
setblock ~55 ~-2 ~43 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-22 ~1 ~9 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~55 ~-3 ~43 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-22 ~2 ~9 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~33.5 ~0 ~52.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.z"}
summon minecraft:armor_stand ~55.5 ~0 ~43.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.z -> B"}
setblock ~54 ~-1 ~45 minecraft:redstone_lamp
setblock ~-4 ~0 ~0 minecraft:target
setblock ~-4 ~-1 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~58 ~0 ~45 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-4 ~-2 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~58 ~1 ~45 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~54.5 ~0 ~45.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.cout"}
summon minecraft:armor_stand ~-3.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cout"}
setblock ~-2 ~0 ~0 minecraft:redstone_lamp
setblock ~39 ~-1 ~14 minecraft:target
setblock ~39 ~-2 ~14 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-41 ~2 ~-14 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~39 ~-3 ~14 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-41 ~3 ~-14 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~39.5 ~0 ~14.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-2 ~0 ~0 minecraft:redstone_lamp
setblock ~116 ~-1 ~34 minecraft:target
setblock ~116 ~-2 ~34 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-118 ~2 ~-34 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~116 ~-3 ~34 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-118 ~3 ~-34 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~116.5 ~0 ~34.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-2 ~0 ~2 minecraft:redstone_lamp
setblock ~41 ~-1 ~14 minecraft:target
setblock ~41 ~-2 ~14 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-43 ~2 ~-12 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~41 ~-3 ~14 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-43 ~3 ~-12 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~41.5 ~0 ~14.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~-2 ~0 ~2 minecraft:redstone_lamp
setblock ~118 ~-1 ~34 minecraft:target
setblock ~118 ~-2 ~34 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-120 ~2 ~-32 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~118 ~-3 ~34 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-120 ~3 ~-32 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~118.5 ~0 ~34.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~40 ~-1 ~18 minecraft:redstone_lamp
setblock ~15 ~-1 ~2 minecraft:target
setblock ~15 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~25 ~1 ~16 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~15 ~-3 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~25 ~2 ~16 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~40.5 ~0 ~18.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~15.5 ~0 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~40 ~-1 ~18 minecraft:redstone_lamp
setblock ~32 ~-1 ~49 minecraft:target
setblock ~32 ~-2 ~49 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~8 ~1 ~-31 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~32 ~-3 ~49 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~8 ~2 ~-31 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~40.5 ~0 ~18.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~32.5 ~0 ~49.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~-2 ~0 ~4 minecraft:redstone_lamp
setblock ~17 ~-1 ~2 minecraft:target
setblock ~17 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-19 ~2 ~2 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~17 ~-3 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-19 ~3 ~2 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~17.5 ~0 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~-2 ~0 ~4 minecraft:redstone_lamp
setblock ~34 ~-1 ~49 minecraft:target
setblock ~34 ~-2 ~49 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-36 ~2 ~-45 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~34 ~-3 ~49 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-36 ~3 ~-45 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~34.5 ~0 ~49.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~16 ~-1 ~6 minecraft:redstone_lamp
setblock ~-4 ~0 ~2 minecraft:target
setblock ~-4 ~-1 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~20 ~0 ~4 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-4 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~20 ~1 ~4 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~16.5 ~0 ~6.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.sum"}
summon minecraft:armor_stand ~-3.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.sum"}
