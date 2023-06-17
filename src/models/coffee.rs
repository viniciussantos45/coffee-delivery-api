use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::config::db::schema::coffees)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Coffee {
    pub id: String,
    pub coffee_name: String,
    pub image_path: String,
    pub price: f64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::config::db::schema::coffees)]
pub struct NewCoffee<'a> {
    pub id: &'a str,
    pub coffee_name: &'a str,
    pub image_path: &'a str,
    pub price: &'a f64,
}
