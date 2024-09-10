/*
    integer types in Rust signed    -> {i8,i16,i32,i64,i128,isize"word size"}
                          unsigned  -> {u8,u16,u32,u64,u128,usize"word size"}
*/
fn main() {
    println!("\n Hello, Rust variables");
    //demo_integers();
    //demo_floats();
    demo_other_simple_types();
}

//create new function to test integer types
/*
fn demo_integers() {
    //Rust signed integer -> {u8,u16,u32,u64,u128,usize"word size"}
    let a1: i32 = -12345; //decimal
    let a2: i32 = 0xFFFF; //hex
    let a3: i32 = 0o177; //octal
    let a4: i32 = 0b1111_0000; //binary

    //Rust unsigned integer -> {u8,u16,u32,u64,u128,usize"word size"}
    let b: u32 = 12345;

    //Rust architecture-spacific integer types isize usize
    let c: isize = 24680;

    println!("\n Numbers are {} {} {} {} {} {}", a1, a2, a3, a4, b, c);
    println!(
        "\n Numbers in reverse older are {5} {4} {3} {2} {1} {0}",
        a1, a2, a3, a4, b, c
    );
    println!(
        "\n isize is {} bytes on my machine",
        std::mem::size_of::<isize>()
    );
}
*/
/*
fn demo_floats() {
    //Rust has single-precision and double-precision floats
    let f1: f32 = 1.23456;
    let f2: f64 = 9.87654;
    println!("\n Floats are {} {}", f1, f2);
    println!("\n Floats to 2dp are {:.2} {:.2}", f1, f2);
    println!("\n Floats field width 10 L-aligned filled with space are ***{:<10.2}*** and ***{:<10.2}***",f1,f2);
    println!("\n Floats field width 10 R-aligned filled with space are ***{:>10.2}*** and ***{:>10.2}***",f1,f2);
    println!("\n Floats field width 10 L-aligned filled with Hashtag are ***{:#<10.2}*** and ***{:#<10.2}***",f1,f2);
    println!("\n Floats field width 10 R-aligned filled with Hashtag are ***{:#>10.2}*** and ***{:#>10.2}***",f1,f2);

    //you can use sentific notation with floats
    let f3: f32 = -1.60217663e-16;
    let f4: f64 = 2.99792458e8;
    println!("\n Eletron charge {0}, {0:e}, {0:.4e}", f3);
    println!("\n speed of light {0}, {0:e}, {0:.4e}", f4);
}
*/

fn demo_other_simple_types() {
    let is_youssef: bool = true;
    let can_sing: bool = false;

    println!("\nIs youssef? {}, can sing? {}", is_youssef, can_sing);

    //char type:
    let middle_initial = 'C';
    let favourite_emoji = '😎';
    println!(
        "Hey you with the middle initial {}, your fav emoji is {}",
        middle_initial, favourite_emoji
    );
    println!(
        "Hey you with the middle initial {}, your fav emoji is {}, size of char is {}",
        middle_initial,
        favourite_emoji,
        std::mem::size_of::<char>()
    );
}
