use once_cell::sync::Lazy;
use chrono::{DateTime, Utc};
use std::thread::sleep;
use std::time::Duration;
pub fn do_it(){
    println!("\nIn demo_static_local::do_it()");

    static_init_at_compile_time();
    static_init_at_run_time();
}

fn static_init_at_compile_time() {
    static MESSAGE: &str = "Hello Static 😃";
    println!("MESSAGE: {}", MESSAGE);
}

fn static_init_at_run_time() {
    println!("Curr time: {}", Utc::now().format("%T"));  // Print the current time
    // static TIMESTAMP_WONT_WORK: DateTime<Utc> = Utc::now();
    // Static initialization using Lazy
    static TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
        sleep(Duration::new(5, 0));  // Simulate a delay
        let now = Utc::now();  // Get the current UTC time
        println!("Curr time: {}", now.format("%T"));  // Print the time after the delay
        return now;
    });

    // First access to the TIMESTAMP static variable
    println!("TIMESTAMP: {}", (*TIMESTAMP).format("%T"));
    
}