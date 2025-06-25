use std::io;

fn main(){
    println!("Whats your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Welcome to Rust and Solana, {}!",name.trim());
}