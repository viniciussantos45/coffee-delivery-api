use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::config::db::schema::coffees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Coffee {
    pub id: Uuid,
    pub coffee_name: String,
    pub description: String,
    pub additions: Vec<Option<String>>,
    pub image_path: String,
    pub price: f64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::config::db::schema::coffees)]
pub struct NewCoffee<'a> {
    pub coffee_name: &'a str,
    pub image_path: &'a str,
    pub price: &'a f64,
}
