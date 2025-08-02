pub use diesel::{sqlite::SqliteConnection, Connection, QueryResult};
use dotenvy::dotenv;
use std::env;

// #[derive(diesel::MultiConnection)]
// pub enum AnyConnection {
// 	Postgresql(diesel::PgConnection),
// 	Mysql(diesel::MysqlConnection),
// 	Sqlite(diesel::SqliteConnection),
// }

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    // NOTE: from ./.env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
