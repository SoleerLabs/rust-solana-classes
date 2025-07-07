mod calculator;
mod fizzbuzz;

use calculator::*;
use fizzbuzz::*;

fn main() {
    println!("Hello! Welcome to Rust random number calculator");

    subtraction();
    addition();
    multiplication();
    division();

    fizzbuzz();

}