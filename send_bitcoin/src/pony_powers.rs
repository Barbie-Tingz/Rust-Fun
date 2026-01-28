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


fn create_earth() -> PonyAbilities {
    let earth_pony = PonyAbilities {
        name:PonyName::AppleJack, 
        pony_type:PonyType::Earth, 
        magic_points:89, 
    };
    return earth_pony;
}


fn create_peg() -> PonyAbilities {
    let pegasus_pony = PonyAbilities {
    name:PonyName::RainbowDash, 
    pony_type:PonyType::Pegasus, 
    magic_points:400, 
};
return pegasus_pony;
}

fn create_unicorn()-> PonyAbilities {
    let unicorn_pony = PonyAbilities {
    name:PonyName::Fluttershy, 
    pony_type:PonyType::Unicorn, 
    magic_points: 254, 
};
return unicorn_pony
}

fn create_pony() -> &'static str{
    let earth = create_earth(); 
    let peg = create_peg(); 
    let uni = create_unicorn(); 
    
    let earth_pony = match earth {
        PonyAbilities {name: PonyName::AppleJack, pony_type: PonyType::Earth, magic_points: 89} =>
        "Apple Jack, Earth, points: 89", 
    }
    earth_pony
}



fn create_pony() -> &'static str{
    let earth = create_earth(); 
    let peg = create_peg(); 
    let uni = create_unicorn(); 
    
   let earth_pony =  match earth {
        PonyAbilities {name: PonyName::AppleJack, pony_type: PonyType::Earth, magic_points: 89} =>
        println! ("Apple Jack, Earth, points: 89"), 
    }
    earth_pony
}


fn main() {
create_pony(); 
}