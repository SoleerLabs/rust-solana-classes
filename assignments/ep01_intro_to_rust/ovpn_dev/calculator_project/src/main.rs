use std::io;
use rand::Rng;

fn main(){
    println!("Welcome to Suggestor Calculator!");
    println!("================================");

    loop{
        //Get user input first
        let user_number = get_user_input();

        //Generate a random number between 1000 - 5000 sike!
        let our_number = generate_random_number();
        println!("Our generated number is: {}", our_number);

        //Show operation menu
        show_menu();
        let operation = get_operation_choice();

        match operation {
            1 => perform_multiplication(user_number, our_number),
            2 => perform_addition(user_number, our_number),
            3 => perform_subtraction(user_number, our_number),
            4 => perform_division(user_number, our_number),
            5 => {
                println!("Thanks for using the calculator! Goodbye!");
                break;
            },
            _=> println!("Invalid choice, Please try again."),
        }

        println!("\n{}\n", "-".repeat(50));
    }
}

fn get_user_input() -> f64 {
    loop {
        println!("Please enter a number:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input, please enter a valid number."),
        }
    }
}

fn generate_random_number() -> i32 {
    let mut rng = rand::rng();
    rng.random_range(1000..=5000)
}

fn show_menu() {
    println!("Please choose an arithmetic operation:");
    println!("1. Multiplication (x)");
    println!("2. Addition (+)");
    println!("3. Subtraction (-)");
    println!("4. Division (÷)");
    println!("5. Exit");
}

fn get_operation_choice() -> u32 {
    loop {
        println!("Enter your choice (1-5):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(num) if num >= 1 && num <= 5 => return num,
            _ => println!("Invalid choice, please enter a number between 1 and 5."),
        }
    }
}

fn perform_multiplication(user_num: f64, our_num: i32) {
    let result = user_num * our_num as f64;
    println!("\n MULTIPLICATION RESULT:");
    println!("{} × {} = {}", user_num, our_num, result);
    suggest_operation("multiplication", result);
}

fn perform_addition(user_num: f64, our_num: i32) {
    let result = user_num + our_num as f64;
    println!("\nADDITION RESULT:");
    println!("{} + {} = {}", user_num, our_num, result);
    suggest_operation("addition", result);
}

fn perform_subtraction(user_num: f64, our_num: i32) {
    let result = user_num - our_num as f64;
    println!("\nSUBTRACTION RESULT:");
    println!("{} - {} = {}", user_num, our_num, result);
    suggest_operation("subtraction", result);
}

fn perform_division(user_num: f64, our_num: i32) {
    if our_num == 0 {
        println!("Error: Cannot divide by zero!");
        return;
    }
    
    let result = user_num / our_num as f64;
    println!("\n➗ DIVISION RESULT:");
    println!("{} ÷ {} = {:.4}", user_num, our_num, result);
    suggest_operation("division", result);
}

// This is the main suggestion function as requested
fn suggest_operation(operation_name: &str, result: f64) {
    println!("\n💡 SUGGESTION:");
    
    match operation_name {
        "multiplication" => {
            if result > 100000.0 {
                println!("Wow! That's a large result from {}. You might want to try division to see smaller numbers.", operation_name);
            } else if result < 0.0 {
                println!("Your {} resulted in a negative number. Try addition to get a positive result!", operation_name);
            } else {
                println!("Great {} result! Try squaring this result using the power operation.", operation_name);
            }
        },
        "addition" => {
            println!("Nice {} result! Consider trying subtraction to see the difference between the numbers.", operation_name);
        },
        "subtraction" => {
            if result < 0.0 {
                println!("Your {} resulted in a negative number. Try swapping the order or use addition instead!", operation_name);
            } else {
                println!("Good {} result! Try multiplication to see how much larger the product would be.", operation_name);
            }
        },
        "division" => {
            if result < 1.0 {
                println!("Your {} resulted in a decimal less than 1. Try multiplication to get a larger number!", operation_name);
            } else {
                println!("Interesting {} result! Try the modulo operation to see the remainder.", operation_name);
            }
        },
        _ => println!("Try exploring other arithmetic operations to see different results!"),
    }
}