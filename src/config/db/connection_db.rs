use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder().build(manager).expect("loaded")
}
