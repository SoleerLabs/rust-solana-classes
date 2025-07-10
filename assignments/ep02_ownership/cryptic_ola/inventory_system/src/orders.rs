

#[derive(Debug)]
pub struct Order {
    pub id: u32,
    pub customer_id: u32,
    pub customer_name: String, 
    pub product_id: u32,
    pub quantity: u32,
    pub status: OrderStatus,
    pub payment_status: PaymentStatus,
    pub next_order_id: u32,

}
#[derive(Debug)]
pub enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}
#[derive(Debug)]
pub enum PaymentStatus {
    Unpaid,
    Paid,
    Failed,
    Refunded,
}



impl Order {
    pub fn new(next_order_id: u32, id: u32,customer_name: String, customer_id: u32, product_id: u32, quantity: u32) -> Self {
        Order {
            id,
            customer_id,
            product_id,
            customer_name,
            next_order_id,
            quantity,
            status: OrderStatus::Pending,
            payment_status: PaymentStatus::Unpaid,
        }
    }

    pub fn update_status(&mut self, new_status: OrderStatus) {
        self.status = new_status;
    }

    pub fn update_payment_status(&mut self, new_payment_status: PaymentStatus) {
        self.payment_status = new_payment_status;
    }

    pub fn print_status(&self) {
        println!("Order {}: Status - {:?}, Payment - {:?}", 
            self.id, self.status, self.payment_status);
    }
}