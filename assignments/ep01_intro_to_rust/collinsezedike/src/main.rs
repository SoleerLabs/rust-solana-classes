use std::io;
use rand;

fn _add(x: i32, y: i32) -> i32 {
    x + y
}

fn _subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn _divide(x: i32, y: i32) -> i32 {
    x / y
}

fn suggestion() {
    // - Ask user to give you a random number
    println!("Enter a random number:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to get user input");

    let user_num: i32 = user_input.trim().parse().expect("User input isn't a valid number");

    // - generate your own random number form 1k - 5k
    let sys_num = rand::random_range(1000..5000);

    // - multiply yours to theirs
    let result = multiply(user_num, sys_num.try_into().unwrap());

    // - then show result
    println!("{user_num} * {sys_num} = {result}");
}

fn main() {
    suggestion();
}
