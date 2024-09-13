fn main() {
    demo_arrays();
}

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
