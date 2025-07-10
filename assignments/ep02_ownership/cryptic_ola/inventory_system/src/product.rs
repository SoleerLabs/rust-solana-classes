pub struct Product{
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub stock: u32,
}
impl Product{
    pub fn is_in_stock(&self) -> bool {
        self.stock > 0
    }
    pub fn new(id:u32, name: String, price: f64, stock:u32) -> Self{
        Product {
            id,
            name: name.to_string(),
            price,
            stock,
        }
    }
   pub fn reduce_stock(&mut self, quantity: u32) -> Result<(), String>{
        if self.is_in_stock() {
            self.stock -= quantity;
            Ok(())
        } else {
            Err("Product is out of stock".to_string())
        }
    }
}