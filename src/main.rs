pub mod auth;
pub mod config;
pub mod deserializers;
pub mod handlers;
pub mod models;
pub mod state;

use crate::handlers::*;
use crate::state::AppState;
use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use coffee_delivery_api::config::db::connection_db::establish_connection;
use std::sync::Arc;

use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let pool = establish_connection();

    let shared_state = Arc::new(AppState { db_pool: pool });

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(tower_http::cors::Any);

    let app: Router = Router::new()
        .route("/", get(coffee_handler::list_coffees))
        .route("/users/create", post(user_handler::create_user))
        .route("/orders/create", post(order_handler::create_order))
        .route("/orders/my_orders", get(order_handler::list_orders))
        .route("/session", post(auth_handler::authorize))
        .layer(cors)
        .with_state(shared_state);

    let server: &str = "0.0.0.0:10000";

    axum::Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
