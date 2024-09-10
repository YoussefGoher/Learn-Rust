fn main() {
    //demo_if();
    demo_match();
}
/*
fn demo_if() {
    let age = 59;
    if age > 50 {
        println!("You are old");
    }

    let height = 1.67;
    if height > 1.8 {
        println!("You are tall");
    } else {
        println!("You are not so tall");
    }

    let swans_games = 500;
    if swans_games > 300 {
        println!("You are a very loyal fan, we appreciate it dude");
    } else if swans_games > 100 {
        println!("You are a discerning fan");
    } else {
        println!("You are quite a new fan, welcome buddy");
    }

    let message = if age > 50 { "Hi oldie" } else { "Hi newbie" };
    println!("{}", message);
}
*/
fn demo_match() {
    let num = 100;
    println!("\nUsing a match to match to test an expreession against patterns");

    match num {
        100 => println!("A hundred"),
        200 => println!("Two hundred"),
        _ => println!("something else"),
    }

    match num {
        25..=50 => println!("25 to 50"),
        51..=100 => println!("51 to 100"),
        _ => println!("something else"),
    }

    match num {
        25 | 50 | 75 => println!("25, 50, or 75"),
        100 | 200 => println!("100 or 200"),
        _ => println!("Who cares"),
    }

    match num {
        x if x < 50 => println!("Less than 50"),
        x if x == 75 => println!("75"),
        _ => println!("Who cares (could be 100 maybe)"),
    }
    let num = 200;
    let res = match num {
        100 => "A hundred",
        200 => "Two hundred",
        _ => "something else",
    };

    println!("the reslut is {}", res);
}
