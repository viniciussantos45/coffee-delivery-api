use std::sync::Arc;

use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::auth::jwt_auth::Claims;
use crate::models::order::Order;
use crate::{
    deserializers::order_deserializer::CreateOrderBody, models::order::NewOrder, state::AppState,
};

use crate::models::coffee::Coffee;
use crate::models::order_item::NewOrderItem;
use crate::models::user::User;

use crate::config::db::schema::coffees::dsl::*;
use crate::config::db::schema::order_items::dsl as order_items_dsl;
use crate::config::db::schema::orders::dsl::*;
use crate::config::db::schema::users::dsl::*;

use diesel::prelude::*;

pub async fn create_order(
    claims: Claims,
    State(state_app): State<Arc<AppState>>,
    Json(payload): Json<CreateOrderBody>,
) -> impl IntoResponse {
    let connection = &mut state_app.db_pool.as_ref().expect("loaded").get().unwrap();

    let user = users
        .select(User::as_select())
        .filter(email.eq(&claims.sub))
        .first(connection)
        .expect("Error loading coffees");

    let total_price_sum: f64 = payload
        .order_items
        .iter()
        .map(|item| {
            let coffee = coffees
                .select(Coffee::as_select())
                .filter(crate::config::db::schema::coffees::dsl::id.eq(&item.coffee_id))
                .first(connection)
                .expect("Error loading coffees");

            coffee.price * item.quantity as f64
        })
        .sum::<f64>();

    let new_order: NewOrder<'_> = NewOrder {
        id: &uuid::Uuid::new_v4().to_string(),
        user_id: &user.id,
        street: &payload.street,
        number: &payload.number,
        neighborhood: &payload.neighborhood,
        city: &payload.city,
        state: &payload.state,
        zip_code: &payload.zip_code,
        country: &payload.country,
        complement: &payload.complement,
        payment_method: &payload.payment_method,
        total_price: &total_price_sum,
    };

    diesel::insert_into(orders)
        .values(&new_order)
        .execute(connection)
        .expect("Error saving new order");

    payload.order_items.iter().for_each(|item| {
        let coffee = coffees
            .select(Coffee::as_select())
            .filter(crate::config::db::schema::coffees::dsl::id.eq(&item.coffee_id))
            .first(connection)
            .expect("Error loading coffees");

        let new_order_item = NewOrderItem {
            id: &uuid::Uuid::new_v4().to_string(),
            order_id: &new_order.id,
            coffee_id: &coffee.id,
            quantity: &item.quantity,
            unit_price: &coffee.price,
        };

        diesel::insert_into(order_items_dsl::order_items)
            .values(&new_order_item)
            .execute(connection)
            .expect("Error saving new order item");
    });

    (
        StatusCode::CREATED,
        Json(json!({"message": "Order created successfully"})),
    )
}

pub async fn list_orders(
    claims: Claims,
    State(state_app): State<Arc<AppState>>,
) -> impl IntoResponse {
    let connection = &mut state_app.db_pool.as_ref().expect("loaded").get().unwrap();

    let user = users
        .select(User::as_select())
        .filter(email.eq(&claims.sub))
        .first(connection)
        .expect("Error loading coffees");

    let results: Vec<_> = orders
        .limit(5)
        .select(Order::as_select())
        .filter(user_id.eq(&user.id))
        .load(connection)
        .expect("Error loading orders");

    (StatusCode::OK, Json(json!({ "orders": results })))
}
