pub mod config;
pub mod models;

use self::config::db::schema::coffees::dsl::*;
use self::models::*;
use coffee_delivery_api::establish_connection;
use diesel::prelude::*;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app: Router = Router::new()
        .route("/", get(root))
        .route("/create", get(create_coffee));
    // .route("");

    // run it with hyper on localhost:3333
    let server: &str = "0.0.0.0:3333";

    axum::Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() {
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

async fn create_coffee() {
    let connection = &mut establish_connection();

    let new_coffee = NewCoffee {
        id: "2",
        coffee_name: "Cafe aq",
        image_path: "deste_aqui",
    };

    diesel::insert_into(coffees)
        .values(&new_coffee)
        .execute(connection)
        .expect("Error saving new coffee");
}
