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
    let connection = &mut state.db_pool.get().unwrap();

    let new_coffee: NewCoffee<'_> = NewCoffee {
        coffee_name: &payload.coffee_name,
        image_path: &payload.image_path,
        price: &payload.price,
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

pub async fn list_coffees(
    // _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let connection = &mut state.db_pool.get().unwrap();
    let results: Vec<_> = coffees
        .select(Coffee::as_select())
        .load(connection)
        .expect("Error loading coffees");

    (StatusCode::OK, Json(json!({ "coffees": results })))
}
