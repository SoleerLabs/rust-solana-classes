pub fn multiply_numbers(a: i32, b: i32) -> i32 {
    a * b
}

pub fn add_numbers(a:i32, b:i32) -> i32 {
    a + b
}

pub fn subtract_numbers(a:i32, b:i32) -> i32 {
    a - b
}

pub fn divide_numbers(a:i32, b:i32) -> i32 {
    if b == 0 {
        panic!("cannot divide by zero");
    }

    a / b
}
