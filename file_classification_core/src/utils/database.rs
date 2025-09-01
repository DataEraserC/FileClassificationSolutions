pub use diesel::{Connection, QueryResult};
use dotenvy::dotenv;
use std::env;

#[derive(diesel::MultiConnection)]
pub enum AnyConnection {
    // Postgresql(diesel::PgConnection),
    // Mysql(diesel::MysqlConnection),
    Sqlite(diesel::SqliteConnection),
}

pub fn establish_connection() -> AnyConnection {
    dotenv().ok();
    // NOTE: from ./.env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_type = env::var("DATABASE_TYPE").expect("DATABASE_TYPE must be set");

    match database_type.as_str() {
        "sqlite" => AnyConnection::Sqlite(
            diesel::SqliteConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
        ),
        _ => panic!("Unsupported database type: {}", database_type),
    }
}
