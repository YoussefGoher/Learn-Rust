use once_cell::sync::Lazy;
use chrono::{DateTime, Utc};

pub static GLOBAL_MESSAGE : &str ="This is Global Message";

static GLOBAL_TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
    let now = Utc::now();  // Get the current UTC time
    println!("this is the Lazy Initialize of GLOBAL_TIMESTAMP");
    println!("Curr time: {}", now.format("%T"));  // Print the time after the delay
    return now;
});

pub fn do_it(){
    println!("\nIn demo_static_global::do_it()");
    f1();
    f1();
    f2();
    f2();
}

fn f1(){
    println!("\nIn f1,  GLOBAL_MESSAGE: {}",GLOBAL_MESSAGE);
    println!("In f1,  GLOBAL_TIMESTAMP: {}",(*GLOBAL_TIMESTAMP).format("%T"));
}

fn f2(){
    println!("\nIn f2,  GLOBAL_MESSAGE: {}",GLOBAL_MESSAGE);
    println!("In f2,  GLOBAL_TIMESTAMP: {}",(*GLOBAL_TIMESTAMP).format("%T"));
}