use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateOrderBody {
    pub street: String,
    pub number: String,
    pub neighborhood: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
    pub complement: String,
    pub payment_method: String,
    pub order_items: Vec<OrderItem>,
}

#[derive(Deserialize)]
pub struct OrderItem {
    pub coffee_id: String,
    pub quantity: i32,
}
