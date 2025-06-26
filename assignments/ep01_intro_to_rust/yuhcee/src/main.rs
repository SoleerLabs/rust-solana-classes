use std::io;
use rand::Rng;
use yuhcee::{multiply_numbers, add_numbers, subtract_numbers, divide_numbers};

fn main() {
    println!("Hello, Rusteaceans!");
    greeter();

    loop {
        println!("Choose an option:");
        println!("1. Transform a number");
        println!("2. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => transform_number(),
            "2" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn greeter() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read input");

    println!("Welcome to Rust, {}!", name.trim());
}

fn transform_number() {
    println!("Enter a number to transform:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: i32 = input.trim().parse().expect("Please enter a valid number");

    let random_number: i32 = rand::rng()
    .random_range(1000..=5000);

    let multiplied_number = multiply_numbers(number, random_number);
    let added_number = add_numbers(number, random_number);
    let subtracted_number = subtract_numbers(number, random_number);
    let divided_number = divide_numbers(number, random_number);

    println!("Transformed numbers:");
    println!("Multiplied: {}", multiplied_number);
    println!("Added: {}", added_number);
    println!("Subtracted: {}", subtracted_number);
    println!("Divided: {}", divided_number);
    println!("Random number used: {}", random_number);
    println!("Original number: {}", number);
    println!("Thank you for using the number transformer!");

}