fn main() {
    //demo_arrays();
    demo_arrays_techniques();
}
/*
fn demo_arrays() {
    println!("\ndemo Function to use arrays in Rust");
    let arr1: [i32; 8] = [100, 34, 60, -99, 0, 0, 0, 0];

    //get array length and its frist element
    println!(
        "arr1 length = {} & arr1 frist Element = {}",
        arr1.len(),
        arr1[0]
    );

    //chenge element vlue
    let mut arr2: [i32; 3] = [-66, 100, 40];
    arr2[1] = -50;
    println!(
        "arr2 length = {} & arr2 second Element = {}",
        arr2.len(),
        arr2[1]
    );

    //use for loop to print the array
    let arr3: [i32; 7] = [-66, 100, 40, 20, 10, 2, -30];
    println!("arr3 Elements:");
    for elem in arr3 {
        println!("{:>4}", elem);
    }
}
*/
fn demo_arrays_techniques() {
    // You can specify type info and size.
    let a1: [i64; 5];
    a1 = [100, 101, 102, 103, 104];
    println!("\na1 is {:?}", a1);

    // You can fill an array with [filler;size] syntax.
    let mut a2 = [99; 5];
    a2[0] = 24;
    println!("a2 is {:?}", a2);
}
