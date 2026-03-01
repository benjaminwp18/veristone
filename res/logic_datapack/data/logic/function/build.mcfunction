place template logic:or_gate ~-66 ~ ~131
place template logic:and_gate ~-58 ~ ~202
place template logic:xor_gate ~-58 ~ ~-108
place template logic:and_gate ~-139 ~ ~-32
place template logic:xor_gate ~52 ~ ~26
setblock ~-57 ~-1 ~205 minecraft:redstone_lamp
setblock ~-66 ~-1 ~131 minecraft:target
setblock ~-66 ~-2 ~131 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~9 ~1 ~74 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-66 ~-3 ~131 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~9 ~2 ~74 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-56.5 ~0 ~205.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.y"}
summon minecraft:armor_stand ~-65.5 ~0 ~131.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.y -> A"}
setblock ~-138 ~-1 ~-29 minecraft:redstone_lamp
setblock ~-64 ~-1 ~131 minecraft:target
setblock ~-64 ~-2 ~131 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-74 ~1 ~-160 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-64 ~-3 ~131 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-74 ~2 ~-160 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-137.5 ~0 ~-28.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.z"}
summon minecraft:armor_stand ~-63.5 ~0 ~131.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.z -> B"}
setblock ~-65 ~-1 ~133 minecraft:redstone_lamp
setblock ~-4 ~0 ~0 minecraft:target
setblock ~-4 ~-1 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-61 ~0 ~133 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-4 ~-2 ~0 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-61 ~1 ~133 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-64.5 ~0 ~133.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.cout"}
summon minecraft:armor_stand ~-3.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cout"}
setblock ~-2 ~0 ~0 minecraft:redstone_lamp
setblock ~-58 ~-1 ~-108 minecraft:target
setblock ~-58 ~-2 ~-108 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~56 ~2 ~108 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-58 ~-3 ~-108 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~56 ~3 ~108 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~-57.5 ~0 ~-107.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-2 ~0 ~0 minecraft:redstone_lamp
setblock ~-58 ~-1 ~202 minecraft:target
setblock ~-58 ~-2 ~202 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~56 ~2 ~-202 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-58 ~-3 ~202 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~56 ~3 ~-202 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~0.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a"}
summon minecraft:armor_stand ~-57.5 ~0 ~202.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.a -> A"}
setblock ~-2 ~0 ~2 minecraft:redstone_lamp
setblock ~-56 ~-1 ~-108 minecraft:target
setblock ~-56 ~-2 ~-108 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~54 ~2 ~110 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-56 ~-3 ~-108 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~54 ~3 ~110 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~-55.5 ~0 ~-107.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~-2 ~0 ~2 minecraft:redstone_lamp
setblock ~-56 ~-1 ~202 minecraft:target
setblock ~-56 ~-2 ~202 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~54 ~2 ~-200 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-56 ~-3 ~202 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~54 ~3 ~-200 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b"}
summon minecraft:armor_stand ~-55.5 ~0 ~202.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.b -> B"}
setblock ~-57 ~-1 ~-104 minecraft:redstone_lamp
setblock ~52 ~-1 ~26 minecraft:target
setblock ~52 ~-2 ~26 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-109 ~1 ~-130 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~52 ~-3 ~26 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-109 ~2 ~-130 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-56.5 ~0 ~-103.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~52.5 ~0 ~26.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~-57 ~-1 ~-104 minecraft:redstone_lamp
setblock ~-139 ~-1 ~-32 minecraft:target
setblock ~-139 ~-2 ~-32 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~82 ~1 ~-72 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-139 ~-3 ~-32 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~82 ~2 ~-72 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-56.5 ~0 ~-103.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.x"}
summon minecraft:armor_stand ~-138.5 ~0 ~-31.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.x -> A"}
setblock ~-2 ~0 ~4 minecraft:redstone_lamp
setblock ~54 ~-1 ~26 minecraft:target
setblock ~54 ~-2 ~26 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-56 ~2 ~-22 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~54 ~-3 ~26 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~-56 ~3 ~-22 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~54.5 ~0 ~26.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~-2 ~0 ~4 minecraft:redstone_lamp
setblock ~-137 ~-1 ~-32 minecraft:target
setblock ~-137 ~-2 ~-32 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~135 ~2 ~36 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-137 ~-3 ~-32 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~135 ~3 ~36 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~-1.5 ~1 ~4.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin"}
summon minecraft:armor_stand ~-136.5 ~0 ~-31.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.cin -> B"}
setblock ~53 ~-1 ~30 minecraft:redstone_lamp
setblock ~-4 ~0 ~2 minecraft:target
setblock ~-4 ~-1 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~57 ~0 ~28 minecraft:redstone_lamp[lit=true] run setblock ~ ~1 ~ minecraft:redstone_block"}
setblock ~-4 ~-2 ~2 minecraft:repeating_command_block{auto: 1b, Command: "execute if block ~57 ~1 ~28 minecraft:redstone_lamp[lit=false] run setblock ~ ~2 ~ minecraft:target"}
summon minecraft:armor_stand ~53.5 ~0 ~30.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "Y -> full_add#0.sum"}
summon minecraft:armor_stand ~-3.5 ~1 ~2.5 {Invisible: 1b, NoGravity: 1b, Marker: 1b, CustomNameVisible: 1b, CustomName: "full_add#0.sum"}
