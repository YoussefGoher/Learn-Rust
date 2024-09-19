pub fn do_it() {
    println!("\nIn demo_string_slice_intro::do_it()");

    slice_string_object();
    slice_string_literal();
}

fn slice_string_object() {
    let mut my_name: String = String::from("Youssef");
    my_name.push_str(" Goher");

    let ref1_my_name: &String = &my_name; //&string
    let ref2_my_name: &str = &my_name; //&str

    println!(
        "my_name is {}\nref1_my_name is {}\nref2_my_name is {}\n",
        my_name, ref1_my_name, ref2_my_name
    );
}

fn slice_string_literal() {
    let my_name = "Youssef";

    let my_name2: &'static str = "Goher";

    println!("my_name is {}\nmy_name2 is {}\n", my_name, my_name2);
}
