pub fn example() {
    let mut line: String = String::new();

    println!("Enter your name :");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();

    println!("Hello , {}", line);
    println!("no of bytes read , {}", b1);
}

/*
simple out put hellow world text
*/
pub fn print_me() {
    println!("just hello world text")
}