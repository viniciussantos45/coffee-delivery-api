use std::sync::Arc;

use crate::config::db::schema::users::dsl::*;
use crate::deserializers::user_deserializer::CreateUserBody;
use crate::models::user::{NewUser, User};
use crate::state::AppState;

use bcrypt::hash;

use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};

use serde_json::json;

use diesel::prelude::*;

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserBody>,
) -> impl IntoResponse {
    let connection = &mut state.db_pool.as_ref().expect("loaded").get().unwrap();

    let results: Vec<_> = users
        .limit(1)
        .select(User::as_select())
        .filter(email.eq(&payload.email))
        .load(connection)
        .expect("Error loading coffees");

    if results.len() > 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "User already exists"})),
        );
    }

    let new_user: NewUser<'_> = NewUser {
        id: &uuid::Uuid::new_v4().to_string(),
        name: &payload.name,
        email: &payload.email,
        password: &hash(&payload.password, 7).expect("Error hashing password"),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");

    (
        StatusCode::CREATED,
        Json(json!({"message": "User created successfully"})),
    )
}
