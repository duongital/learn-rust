pub mod a_module;
pub mod a_standard_io;

// main is root function and used only in file `main.rs`
fn main() {
    // a_standard_io::print_me();
    a_module::draw_me();
    a_module::draw_me();
    println!("Hello world");
    a_standard_io::print_me();
}
