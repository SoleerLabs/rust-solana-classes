pub fn fizz_buzz() {
    for i in 1..=100 {
        if i % 5 == 0 && i % 3 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}