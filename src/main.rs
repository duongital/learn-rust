// getting from related module files
pub mod a_standard_io;
pub mod a_module;

// main is root function and used only in file `main.rs`
fn main() {
    // a_standard_io::print_me();
    a_module::draw_me();
}
