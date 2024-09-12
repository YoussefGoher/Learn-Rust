//crate-level attribute.
#![allow(dead_code)]
mod mytypes;
//mod mytypes_with_complier_flag;
use mytypes::{Colour, HouseLocation};
//use mytypes_with_complier_flag::Colour;
//like namespace

fn main() {
    //demo_simple_enums();
    //demo_enum_with_data();
    demo_using_option_enum();
}

fn demo_simple_enums() {
    println!("Demo simple enums");

    let c: Colour = Colour::Red;
    match c {
        Colour::Red => println!("coch"),
        Colour::Green => println!("gwyrdd"),
        Colour::Blue => println!("glas"),
    }
}

fn demo_enum_with_data() {
    println!("\nDemo enums with data");

    //let h: HouseLocation = HouseLocation::Number(4);
    let h: HouseLocation = HouseLocation::Name("Egypt".to_string());
    //let h: HouseLocation = HouseLocation::Unknown;
    match h {
        HouseLocation::Number(n) => println!("You live in house number {}", n),
        HouseLocation::Name(s) => println!("You live in a house in {}", s),
        HouseLocation::Unknown => println!("Your house location is unknown"),
    }
    println!(
        "Btw the size of HouseLocation is {} bytes",
        std::mem::size_of::<HouseLocation>()
    );
}

fn demo_using_option_enum() {
    println!("\nDemo Using enum Option type");
    //let secs: Option<u32> = sec_of_day(23, 59, 59);
    let secs: Option<u32> = sec_of_day(23, 59, 60);
    match secs {
        Some(s) => println!("second of day using match: is {}", s),
        None => println!("second of day using match: no value available"),
    }
    println!(
        "second of day using unwrap_or method: {}",
        secs.unwrap_or(0)
    );
}

fn sec_of_day(hours: u32, minutes: u32, seconds: u32) -> Option<u32> {
    if hours <= 23 && minutes <= 59 && seconds <= 59 {
        return Option::Some(hours * 3600 + minutes * 60 + seconds);
    } else {
        return Option::None;
    }
}
