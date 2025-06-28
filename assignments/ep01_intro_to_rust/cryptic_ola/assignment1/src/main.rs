use std::io;
use rand::Rng;

fn main() {
    println!("Give a random number");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to get user input");
    // generate random number
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1000..5000);
    println!("Your random number is {}", random_number);
    // conver user input to integer
    let user_input = user_input.trim().parse::<i32>().expect("please entr a valid number");
    addition(user_input, random_number);
    mulriplied(user_input, random_number);
    substracted(user_input, random_number);
}
fn addition(a: i32, b: i32) -> i32 {
 let added_user_number: i32 = a + b;
 println!("Your number added is {}", added_user_number);
 added_user_number
}
fn mulriplied(a: i32, b: i32) -> i32 {
    let multiplied_user_number: i32 = a*b;
    println!("Your number multiplied is {}", multiplied_user_number);
    multiplied_user_number
}
fn substracted(a: i32, b: i32) -> i32 {
    let substracted_user_number: i32 = a-b;
    println!("Your substracted number is {}",  substracted_user_number);
    substracted_user_number
}