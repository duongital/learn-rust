// getting from related module files
pub mod hello_world;
pub mod standard_io;
pub mod variable;

// main is root function and used only in file `main.rs`
fn main() {
    standard_io::print_me();
    hello_world::hello_world();
}
