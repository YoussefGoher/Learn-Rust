mod demo_locals;
mod demo_static_local;
fn main() {
    println!("Hello, world!");
    demo_locals::do_it();
    demo_static_local::do_it();
}
