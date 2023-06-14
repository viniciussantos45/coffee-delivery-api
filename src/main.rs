pub mod config;
pub mod controllers;
pub mod models;

use crate::controllers::*;

use coffee_delivery_api::establish_connection;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app: Router = Router::new()
        .route("/", get(coffee_controller::list_coffees))
        .route("/create", post(coffee_controller::create_coffee));
    // .route("");

    // run it with hyper on localhost:3333
    let server: &str = "0.0.0.0:3333";

    axum::Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
