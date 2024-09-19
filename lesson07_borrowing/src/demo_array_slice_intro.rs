pub fn do_it() {
    println!("\nIn demo_array_slice_intro::do_it()");

    //define array in Rust
    let array: [i32; 4] = [102, 103, 10, 20];

    //create slice of array
    let array_slice = &array;
    let array_slice2: &[i32] = &array;

    println!(
        "array_slice ptr: {:p}, len:{}, elements: {:?}",
        array_slice.as_ptr(),
        array_slice.len(),
        array_slice
    );
    println!(
        "array_slice2 ptr: {:p}, len:{}, elements: {:?}",
        array_slice2.as_ptr(),
        array_slice2.len(),
        array_slice2
    );
}
