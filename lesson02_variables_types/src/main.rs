/*
    integer types in Rust signed    -> {i8,i16,i32,i64,i128,isize"word size"}
                          unsigned  -> {u8,u16,u32,u64,u128,usize"word size"}
*/
fn main() {
    println!("\n Hello, Rust variables");
    demo_integers();
}

//create new function to test integer types

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
