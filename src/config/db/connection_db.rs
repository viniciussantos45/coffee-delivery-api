use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Result<Pool<ConnectionManager<SqliteConnection>>, r2d2::Error> {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager: ConnectionManager<SqliteConnection> =
        ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder().build(manager)
}
