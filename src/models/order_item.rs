use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::config::db::schema::order_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub coffee_id: Uuid,
    pub quantity: i64,
    pub unit_price: f64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::config::db::schema::order_items)]
pub struct NewOrderItem<'a> {
    pub id: &'a str,
    pub order_id: &'a str,
    pub coffee_id: &'a Uuid,
    pub quantity: &'a i64,
    pub unit_price: &'a f64,
}
