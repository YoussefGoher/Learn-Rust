//crate-level attribute.
#![allow(dead_code)]
mod mytypes;
//mod mytypes_with_complier_flag;
use mytypes::{Colour, HouseLocation};
//use mytypes_with_complier_flag::Colour;
//like namespace

fn main() {
    //demo_simple_enums();
    demo_enum_with_data();
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
