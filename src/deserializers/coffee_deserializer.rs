use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCoffeeBody {
    pub coffee_name: String,
    pub image_path: String,
}
