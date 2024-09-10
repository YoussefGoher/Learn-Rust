/*
    integer types in Rust signed    -> {i8,i16,i32,i64,i128,isize"word size"}
                          unsigned  -> {u8,u16,u32,u64,u128,usize"word size"}
*/
fn main() {
    println!("\nHello, Rust variables");
    //demo_integers();
    //demo_floats();
    //demo_other_simple_types();
    demo_techniques();
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
/*
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
*/
fn demo_techniques() {

    // Rust can infer types.
    let a = -12345;
    let b = 3.14;
    let c = 'X';
    println!("\na is {}, b is {}, c is {}", a, b, c);

    // Variables are immutable by default.
    let d = 0;
    //d = 1;
    println!("d is {}", d);

    // You must use the mut qualifier to make a variable mutable.
    let mut e = 0;
    println!("e originally is {}", e);
    e = 1;
    println!("e afterwards is {}", e);
 
    // If you don't use a variable, prefix name with _ to avoid compiler warning.
    let _f = 0;

    // You can cast using the "as" keyword.
    let g = 3.99;
    let h = g as i32;
    println!("g is {}, h is {}", g, h);

    // Rust enables you to redeclare a variable in the current scope. This is called shadowing and it's quite cool.
    let num = String::from("12345");
    println!("num as a string is {}", num);
    let num = 12345;
    println!("num+1 as a number is {}", num + 1);

    // You can define compile-time constants. You must specify the type btw.
    const SECONDS_IN_HOUR: i32 = 3_600;
    const SECONDS_IN_DAY: i32 = 24 * SECONDS_IN_HOUR;
    println!("There are {} seconds in a day", SECONDS_IN_DAY);  
}