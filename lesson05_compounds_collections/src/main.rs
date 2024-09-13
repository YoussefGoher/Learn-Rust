fn main() {
    //demo_arrays();
    //demo_arrays_techniques();
    //demo_tuples();
    demo_vec();
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
/*
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
*/
/*
fn demo_tuples() {
    println!("\ndemo Function to use tuples in Rust");
    // A tuple is a fixed-size heterogeneous collection.
    let t1 = (100, String::from("hello"), 3.14);
    println!(
        "\nt1 is {:?}, individual elements are {}, {}, {}",
        t1, t1.0, t1.1, t1.2
    );

    // You can also create a mutable tuple (but you have to be consistent with element types).
    let mut t2 = (100, String::from("hello"), 3.14);
    t2.0 = 199;
    println!("t2 is {:?}", t2);

    // You can specify type info.
    let t3: (i32, String, f64);
    t3 = (200, String::from("world"), 9.99);
    println!("t3 is {:?}", t3);

    // You can also have an empty tuple (handy for functions that return nothing at all).
    let t4 = ();
    println!("t4 is {:?}", t4);
}
*/

fn demo_vec() {
    println!("\nDemo Using Vector In Rust");
    let mut vec1: Vec<i32> = Vec::new();

    let item = vec1.get(0);

    match item {
        Some(vlue) => println!("Vlue= {}", vlue),
        None => println!("This Index Has No Vlue"),
    };

    vec1.push(32);

    let item = vec1.pop();

    match item {
        Some(vlue) => println!("POP Vlue= {}", vlue),
        None => println!("POP of Vector Has No Vlue"),
    };

    vec1 = vec![10, 3, 4, 5, 6, 7];

    println!("print vector vec1 using debug: {:?}", vec1);

    // First loop: Borrowing the vector
    for item in &vec1 {
        println!("Borrowed: {:>3}", item);
    }

    // vec1 is still available here
    println!(
        "Print vector vec1 using debug after print with borrowing: {:?}",
        vec1
    );

    // Second loop: Moving ownership
    for item in vec1 {
        println!("Owned: {:>3}", item);
    }

    // vec1 is no longer available after this point because it was moved
    // println!("Trying to use vec1 after move: {:?}", vec1); // This line would cause an error
}
