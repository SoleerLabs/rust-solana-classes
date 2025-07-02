use std::io;

pub fn greeter() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read input");

    println!("Welcome to Rust, {}!", name.trim());
}