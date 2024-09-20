pub fn do_it() {
    println!("\nIn demo_borrow_checker::do_it()");
    defineing_many_immutable_references();
    restrictions_after_defining_mutable_reference();
    restrictions_after_defining_immutable_reference();
}

fn defineing_many_immutable_references() {
    println!("\nIn demo_borrow_checker::defineing_many_immutable_reference()");
    let mut my_name: String = String::from("Youssef");
    my_name.push_str(" Goher");

    let ref1_my_name: &String = &my_name;
    let ref2_my_name: &String = &my_name;
    let ref3_my_name: &String = &my_name;

    println!(
        "my_name is {}\nref1_my_name is {}\nref2_my_name is {}\nref3_my_name is {}",
        my_name, ref1_my_name, ref2_my_name, ref3_my_name
    );
}

fn restrictions_after_defining_mutable_reference() {
    println!("\nIn demo_borrow_checker::drestrictions_after_defining_mutable_reference()");
    let mut my_name: String = String::from("Youssef");

    let ref1_my_name: &mut String = &mut my_name;
    //let ref2_my_name: &String = &my_name; // compiler error
    //let ref3_my_name: &String = &my_name; // compiler error
    //println!("my_name is {}",my_name);    // compiler error also

    ref1_my_name.push_str(" Goher");

    println!("ref1_my_name is {}", ref1_my_name);

    let mut my_name2: String = String::from("Youssef");

    if !(my_name2.is_empty()) {
        let ref1_my_name2: &mut String = &mut my_name2;
        println!("ref1_my_name2 is {}", ref1_my_name2);
    }
    println!("my_name2 is {}", my_name2); // not compiler error end of mutable ref scope
}

fn restrictions_after_defining_immutable_reference() {
    println!("\nIn demo_borrow_checker::restrictions_after_defining_immutable_reference()");
    let mut my_name: String = String::from("Youssef");
    my_name.push_str(" Goher");

    let ref1_my_name: &String = &my_name;
    let ref2_my_name: &String = &my_name;
    let ref3_my_name: &String = &my_name;

    //let ref4_my_name: &mut String = &mut my_name;// error cant have mutable reference after immutable reference
    //my_name.push_str(" Goher");// error cant have mutable reference after immutable reference

    println!(
        "my_name is {}\nref1_my_name is {}\nref2_my_name is {}\nref3_my_name is {}",
        my_name, ref1_my_name, ref2_my_name, ref3_my_name
    );
    let mut my_name2: String = String::from("Youssef");
    my_name2.push_str(" Goher");
    {
        let ref1_my_name2: &String = &my_name2;
        let ref2_my_name2: &String = &my_name2;
        let ref3_my_name2: &String = &my_name2;

        //let ref4_my_name: &mut String = &mut my_name;// error cant have mutable reference after immutable reference
        //my_name.push_str(" Goher");// error cant have mutable reference after immutable reference

        println!(
            "my_name2 is {}\nref1_my_name2 is {}\nref2_my_name2 is {}\nref3_my_name2 is {}",
            my_name2, ref1_my_name2, ref2_my_name2, ref3_my_name2
        );
    }
    my_name2.push_str(" Goher");
    println!("my_name2 is {}", my_name2); // not compiler error end of immutable ref scope
}
