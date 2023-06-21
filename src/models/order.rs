use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::config::db::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub street: String,
    pub number: String,
    pub neighborhood: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
    pub complement: String,
    pub payment_method: String,
    pub total_price: f64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::config::db::schema::orders)]
pub struct NewOrder<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub street: &'a str,
    pub number: &'a str,
    pub neighborhood: &'a str,
    pub city: &'a str,
    pub state: &'a str,
    pub zip_code: &'a str,
    pub country: &'a str,
    pub complement: &'a str,
    pub payment_method: &'a str,
    pub total_price: &'a f64,
}
