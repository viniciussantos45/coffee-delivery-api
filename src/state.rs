use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};

pub struct AppState {
    pub db_pool: Result<Pool<ConnectionManager<SqliteConnection>>, r2d2::Error>,
}
