pub fn do_it(){
    println!("\nIn demo_locals::do_it()");
    let name = "youssef";

    println!("Outer name before if is {}", name);

    if !name.is_empty() {
        let name = "Goher";
        println!("if name is {}", name);
    }
    
    println!("Outer name after if is {}", name);
}