use std::sync::Arc;

use axum::{extract::State, Json};
use bcrypt::verify;
use jsonwebtoken::{encode, Header};

use chrono::{Duration, Utc};

use crate::config::db::schema::users::dsl::*;

use crate::{auth::jwt_auth::*, models::user::User, state::AppState};

use diesel::prelude::*;

pub async fn authorize(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    let connection = &mut state.db_pool.as_ref().expect("loaded").get().unwrap();

    let user = users
        .select(User::as_select())
        .filter(email.eq(&payload.email))
        .first(connection)
        .expect("Error loading coffees");

    if verify(&payload.password, &user.password).unwrap() {
        let now = Utc::now();
        let seven_days = now + Duration::days(7);
        let seven_days_exp = seven_days.timestamp() as usize;

        let claims = Claims {
            sub: user.email,
            company: user.name,
            exp: seven_days_exp,
        };

        let token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| AuthError::TokenCreation)?;

        return Ok(Json(AuthBody::new(token)));
    }

    Err(AuthError::WrongCredentials)
}
