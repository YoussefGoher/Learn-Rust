//mod demo_locals;
//mod demo_static_local;
//mod demo_static_global; 
mod demo_static_mutable;
fn main() {
    //println!("\nBtw GLOBAL_MESSAGE is {}",demo_static_global::GLOBAL_MESSAGE);
    //demo_locals::do_it();
    //demo_static_local::do_it();
    //demo_static_global::do_it();
    demo_static_mutable::do_it();
}
