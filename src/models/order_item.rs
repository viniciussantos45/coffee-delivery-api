use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::config::db::schema::order_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub coffee_id: String,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::config::db::schema::order_items)]
pub struct NewOrderItem<'a> {
    pub id: &'a str,
    pub order_id: &'a str,
    pub coffee_id: &'a str,
    pub quantity: &'a i32,
    pub unit_price: &'a f64,
}
