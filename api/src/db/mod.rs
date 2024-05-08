use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    const DATABASE_URL: &str = dotenv!("DATABASE_URL");
    PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL))
}
