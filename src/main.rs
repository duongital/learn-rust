// getting from related module files
pub mod standard_io;
pub mod hello_world;

// main is root function and used only in file `main.rs`
fn main() {
    standard_io::print_me();
    hello_world::hello_world();
}

