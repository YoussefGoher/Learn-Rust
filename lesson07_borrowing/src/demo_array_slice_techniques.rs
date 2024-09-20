pub fn do_it() {
    println!("\nIn demo_array_slice_techniques::do_it()");

    slice_itration();
    slice_part_of_string();
    slice_mutability();
}

fn slice_itration() {
    //define array in Rust
    let array: [i32; 4] = [102, 103, 10, 20];

    //create slice of array
    let array_slice = &array;

    println!("\nElements in array_slice");

    //itrat over array slice
    for element in array_slice {
        println!("{}", element);
    }
}

fn slice_part_of_string() {
    //define array in Rust
    let array: [i32; 8] = [102, 103, 10, 20, 123, 80, 65, 193];

    //create slices of array
    let array_slice1 = &array[0..2];
    let array_slice2 = &array[3..5];
    let array_slice3 = &array[5..];
    let array_slice4 = &array[5..7];

    //print array slices:
    println!("array_slice1: {:?}", array_slice1);
    println!("array_slice2: {:?}", array_slice2);
    println!("array_slice3: {:?}", array_slice3);
    println!("array_slice4: {:?}", array_slice4);
}

fn slice_mutability() {
    //define array in Rust
    let mut array: [i32; 8] = [102, 103, 10, 20, 123, 80, 65, 193];
    println!("array_before: {:?}", array);

    if true {
        //create mutble slices of array
        let array_slice2 = &mut array[3..5];
        array_slice2[0] = 1000;
    }

    println!("array_after: {:?}", array);
}
