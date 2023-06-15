pub mod config;
pub mod deserializers;
pub mod handlers;
pub mod models;
pub mod state;

use std::sync::Arc;

use crate::handlers::*;

use coffee_delivery_api::config::db::connection_db::establish_connection;

use crate::state::AppState;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let pool = establish_connection();

    let shared_state = Arc::new(AppState { db_pool: pool });

    // let shared_state = std::sync::Arc::new(state);

    // build our application with a single route
    let app: Router = Router::new()
        .route("/", get(coffee_handler::list_coffees))
        .route("/create", post(coffee_handler::create_coffee))
        .with_state(shared_state);
    // .layer(Extension(AppState {
    //     db_pool: pool.clone(),
    // }));
    // .route("");

    // run it with hyper on localhost:3333
    let server: &str = "0.0.0.0:3333";

    axum::Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
