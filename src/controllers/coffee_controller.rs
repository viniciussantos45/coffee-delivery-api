use axum::response::IntoResponse;
use axum::{extract::Json as JsonExtract, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::json;

use crate::config::db::schema::coffees::dsl::*;
use crate::establish_connection;
use crate::models::coffee::*;

use diesel::prelude::*;

#[derive(Deserialize)]
pub struct CreateCoffeeBody {
    coffee_name: String,
    image_path: String,
}

pub async fn create_coffee(Json(payload): Json<CreateCoffeeBody>) {
    let connection = &mut establish_connection();

    let new_coffee = NewCoffee {
        id: uuid::Uuid::new_v4().to_string(),
        coffee_name: payload.coffee_name,
        image_path: payload.image_path,
    };

    diesel::insert_into(coffees)
        .values(&new_coffee)
        .execute(connection)
        .expect("Error saving new coffee");

    dbg!(&new_coffee);
}

pub async fn list_coffees() {
    let connection = &mut establish_connection();
    let results = coffees
        .limit(5)
        .select(Coffee::as_select())
        .load(connection)
        .expect("Error loading coffees");

    println!("Displaying {} coffees", results.len());
    for coffee in results {
        println!("{}", coffee.coffee_name);
        println!("-----------\n");
        println!("{}", coffee.image_path);
    }
}
