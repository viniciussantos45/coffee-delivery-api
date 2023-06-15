use std::sync::Arc;

use crate::state::AppState;
use axum::extract::State;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::config::db::schema::coffees::dsl::*;
use crate::deserializers::coffee_deserializer::*;
use crate::models::coffee::*;

use diesel::prelude::*;

pub async fn create_coffee(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCoffeeBody>,
) -> impl IntoResponse {
    let connection = &mut state.db_pool.as_ref().expect("loaded").get().unwrap();

    let new_coffee: NewCoffee<'_> = NewCoffee {
        id: &uuid::Uuid::new_v4().to_string(),
        coffee_name: &payload.coffee_name,
        image_path: &payload.image_path,
    };

    diesel::insert_into(coffees)
        .values(&new_coffee)
        .execute(connection)
        .expect("Error saving new coffee");

    (
        StatusCode::CREATED,
        Json(json!({"message": "Coffee created successfully"})),
    )
}

pub async fn list_coffees(State(state): State<Arc<AppState>>) {
    let connection = &mut state.db_pool.as_ref().expect("loaded").get().unwrap();
    let results: Vec<_> = coffees
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
