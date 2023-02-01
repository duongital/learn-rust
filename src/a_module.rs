// call function from library
use ferris_says::say;
use std::io::{stdout, BufWriter};

// use the function from library
pub fn draw_me() {
    /*
    let stdout = stdout();
    let message = String::from("Hello fellow Rusters");
    let width = message.chars().count();

    let mut writer = BufReader::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
    */
    let out = b"Hello fellow Rustaceans!";
    let width = 24;
    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}
