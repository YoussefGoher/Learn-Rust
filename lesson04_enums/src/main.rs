//crate-level attribute.
#![allow(dead_code)]
mod mytypes;
//mod mytypes_with_complier_flag;
use mytypes::Colour;
//use mytypes_with_complier_flag::Colour;
//like namespace 

fn main() {
    demo_simple_enums();
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
