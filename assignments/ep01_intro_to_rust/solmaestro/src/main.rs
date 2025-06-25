use std::io;
use rand::Rng;

fn subtraction () {

    println!("Please enter a number for subtraction: ");

     let random_number = rand::thread_rng().gen_range(1000, 5001);

    let mut user_number = String::new();
    io::stdin().read_line(&mut user_number).expect("Failed to readline");

    let number: u32 = user_number.trim().parse().expect("Invalid number");
   
    println!("{} - {} is = {}", random_number, number, random_number - number);

}

fn addition () {
    println!("Please enter a number for addition: ");

     let random_number = rand::thread_rng().gen_range(1000, 5001);

    let mut user_number = String::new();
    io::stdin().read_line(&mut user_number).expect("Failed to readline");

    let number: u32 = user_number.trim().parse().expect("Invalid number");

    println!("{} + {} is = {}", random_number, number, random_number + number);
}

fn multiplication () {
    println!("Please enter a number for multiplication: ");

     let random_number = rand::thread_rng().gen_range(1000, 5001);

    let mut user_number = String::new();
    io::stdin().read_line(&mut user_number).expect("Failed to readline");

    let number: u32 = user_number.trim().parse().expect("Invalid number");

    println!("{} * {} is = {}", random_number, number, random_number * number);
}

fn division () {
     println!("Please enter a number for division: ");

     let random_number = rand::thread_rng().gen_range(1000, 5001);

    let mut user_number = String::new();
    io::stdin().read_line(&mut user_number).expect("Failed to readline");

    let number: u32 = user_number.trim().parse().expect("Invalid number");

    println!("{} / {} is = {}", random_number, number, random_number / number);
}


fn main() {
    println!("Hello! Welcome to Rust random number calculator");

    subtraction();
    addition();
    multiplication();
    division();

}
