pub fn do_it() {
    println!("\nIn demo_copying_vs_moving: :do_it()");
    // Simple types implement the Copy trait.
    // When you assign, it bit-copies the value.
    let a = 42;
    let b = a;
    println!("a: {}, b: {}",a,b);
    let mut s1 = String:: from ("hello");
    let s2= s1.clone();
    s1.push_str(" world");
    println!("{}", s1); // Prints "hello world" 
    println! ("{}", s2); // Prints "hello"
}