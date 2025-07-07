pub fn fizzbuzz() {
  println!("Hello, Welcome to Fizz Buzz!");
for num in 1..=100 {
   if num % 3 == 0 && num % 5 == 0 {
    println!("FizzBuzz {}", num);
   }
  if num % 3 == 0 {
    println!("Fizz {}", num);
  } else if num % 5 == 0 {
    println!("Buzz {}", num);
  } else {
    println!("{num}");
  }
}
}