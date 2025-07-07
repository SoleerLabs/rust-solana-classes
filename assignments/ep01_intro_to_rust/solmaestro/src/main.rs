mod calculator;
use crate::calculator::{subtraction, multiplication, division, addition};

fn main() {
    println!("Hello! Welcome to Rust random number calculator");

    subtraction();
    addition();
    multiplication();
    division();

}