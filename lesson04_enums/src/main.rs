mod mytypes;
//use mytypes::Colour;
//like namespace 

fn main() {
    demo_simple_enums();
}

fn demo_simple_enums() {
    println!("Demo simple enums");

    let c: mytypes::Colour = mytypes::Colour::_Red;
    match c {
        mytypes::Colour::_Red => println!("coch"),
        mytypes::Colour::_Green => println!("gwyrdd"),
        mytypes::Colour::_Blue => println!("glas"),
    }
}
