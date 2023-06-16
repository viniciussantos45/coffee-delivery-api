use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateSessionBody {
    pub email: String,
    pub password: String,
}
