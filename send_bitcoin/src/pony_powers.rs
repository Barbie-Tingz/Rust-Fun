// mini project to build on our knowledge of structs, enums and pattern matching
// different ponies have different attributes and pony info will include (name, health, magic level, etc)

struct PonyAbilities { 
    name: PonyName, 
    pony_type: PonyType, 
    magic_points: u32, 
}

enum PonyType {
    Earth, 
    Pegasus, 
    Unicorn, 
}

enum PonyName {
    Fluttershy, 
    RainbowDash, 
    AppleJack, 
}

fn main() {

    match PonyAbilities{name, pony_type, magic_points} {
        PonyAbilities {name:PonyName::RainbowDash, pony_type:PonyType::Pegasus, magic_points: 89} => "Peg, Rainbow, 89 points",
        PonyAbilities {name:PonyName::Fluttershy, pony_type:PonyType::Earth, magic_points: 16} => "Earth, Flutter, 16 points",
    };
}