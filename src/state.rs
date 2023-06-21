use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub struct AppState {
    pub db_pool: Result<Pool<ConnectionManager<PgConnection>>, r2d2::Error>,
}
