use std::io;
mod product;
mod orders;
use orders::{Order, OrderStatus, PaymentStatus};
use product::Product; 
mod store;

fn main() {
  let mut apple = Product {
    id: 1,
    name: "Apple".to_string(),
    price: 0.99,
    stock: 10,
};
//check stock reduce stock
    println!("Is in stock? {}", apple.is_in_stock());

    println!("Enter a stock numer to reduce");
    let stock_number = read_input_u32("Failed to read input");
    (&mut apple).reduce_stock(stock_number);
    println!("Updated product: {:?}", apple.stock);

   let mut order = Order::new(1,2,"ola".to_string(), 100, 42, 3);
 
   order.print_status();
   order.update_status(OrderStatus::Shipped);
   order.update_payment_status(PaymentStatus::Paid);
   order.print_status();

   //store part
    // println!("Enter a a new product name");
    // let mut name = String::new();
    //     io::stdin().read_line(&mut name).expect("Failed to read input");
    //     let name = name.trim().to_string(); 
    // println!("Enter a a new price");
    // let mut price = String::new();
    //     io::stdin().read_line(&mut price).expect("Failed to read input");
    // println!("Enter a a new stock");
    // let mut stock = String::new();
    //     io::stdin().read_line(&mut stock).expect("Failed to read input");
    // let mut store = store::Store::new();
    // store.add_product(name, price.trim().parse::<f64>().unwrap_or(0.0), stock.trim().parse::<u32>().unwrap_or(0));
    let mut store = store::Store::new();

    // Product addition loop
    loop {
        println!("\n=== Product Management ===");
        println!("1. Add a new product");
        println!("2. List all products");
        println!("3. Exit product management");
        
        let choice = read_input_u32("Enter your choice:");

        match choice {
            1 => {
                let name = read_input("Enter product name:");
                let price = read_input_f64("Enter product price:");
                let stock = read_input_u32("Enter stock quantity:");
                
                store.add_product(name, price, stock);
                println!("Product added successfully!");
            }
            2 => {
                println!("\nCurrent Products:");
                store.list_products();
            }
            3 => break,
            _ => println!("Invalid choice, please try again"),
        }
    }
    loop {
        println!("\n=== Order Management ===");
        println!("1. Add a new order");
        println!("2. List all orders");
        println!("3. Exit order management");
        let choice = read_input_u32("Enter your choice:");
    match choice {
        1 => {
            let customer_name = read_input("Enter customer name for the order:");
            let product_id = read_input_u32("Enter product ID to order:");
            let quantity = read_input_u32("Enter quantity to order:");
            store.place_order(customer_name, product_id, quantity);
        }
        1 => {
            println!("\nList Orders");
            store.list_orders();
        }
        2 => break,
        _ => println!("Invalid choice, please try again"),
    }
}
    store.update_order_status(1, OrderStatus::Delivered);



    //to close the programme
    println!("Program finished. Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
// Helper functions remain the same as previous version
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn read_input_u32(prompt: &str) -> u32 {
    loop {
        let input = read_input(prompt);
        match input.parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid positive integer"),
        }
    }
}

fn read_input_f64(prompt: &str) -> f64 {
    loop {
        let input = read_input(prompt);
        match input.parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number"),
        }
    }
}