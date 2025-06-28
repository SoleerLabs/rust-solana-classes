fn main() {
    let item_one = String::from("Fizz");
    let item_two = String::from("Buzz");
    let item_three = String::from("FizzBuzz");
    for i in 1..=100{
        if i % 3 == 0 {
            print!("{}", item_one);
        }else if i % 5 == 0 {
            println!("{}", item_two);
        }else if i % 3 == 0 && i % 5 == 0{
            println!("{}", item_three);
        }else {
            println!("{}", i);
        }
    }
}