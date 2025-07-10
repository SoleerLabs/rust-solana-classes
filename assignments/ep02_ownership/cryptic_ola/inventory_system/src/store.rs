use super::product::Product;
use super::orders::{Order, OrderStatus};
pub struct Store {
    products: Vec<Product>,
    orders: Vec<Order>,
    next_order_id: u32,
}

impl Store {
    pub fn new() -> Self {
        Store {
            products: Vec::new(),
            orders: Vec::new(),
            next_order_id: 1,
        }
    }

    pub fn add_product(&mut self, name: String, price: f64, stock: u32) {
        let id = self.products.len() as u32 + 1;
        self.products.push(Product::new(id, name, price, stock));
    }

    pub fn place_order(&mut self, customer_name: String, product_id: u32, quantity: u32) {
        if let Some(product) = self.products.iter_mut().find(|p| p.id == product_id) {
            if product.is_in_stock() {
                match product.reduce_stock(quantity) {
                    Ok(_) => {
                        let order = Order::new(self.next_order_id,0, customer_name,0, product_id, quantity);
                        self.orders.push(order);
                        println!("Order {} placed successfully!", self.next_order_id);
                        self.next_order_id += 1;
                    }
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Error: Not enough stock for product {}", product_id);
            }
        } else {
            println!("Error: Product {} not found", product_id);
        }
    }

    pub fn update_order_status(&mut self, order_id: u32, new_status: OrderStatus) {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == order_id) {
            order.update_status(new_status);
            println!("Order {} status updated to {:?}", order_id, order.status);
        } else {
            println!("Error: Order {} not found", order_id);
        }
    }

    pub fn list_products(&self) {
        println!("\n Products");
        for product in &self.products {
            println!(
                "ID: {}, Name: {}, Price: ${:.2}, Stock: {}",
                product.id, product.name, product.price, product.stock
            );
        }
    }

    pub fn list_orders(&self) {
        println!("\n Orders ");
        for order in &self.orders {
            println!(
                "ID: {}, Customer: {}, Product ID: {}, Quantity: {}, Status: {:?}",
                order.id, order.customer_name, order.product_id, order.quantity, order.status
            );
        }
    }
}
