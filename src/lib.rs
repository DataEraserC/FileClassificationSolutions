pub mod models;
pub mod schema;

use self::models::{File, Group, NewFile, NewGroup, NewTag, Tag};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_file(conn: &mut SqliteConnection, type_: &str, path: &str) -> File {
    use crate::schema::files;

    let new_file = NewFile { type_, path };

    diesel::insert_into(files::table)
        .values(&new_file)
        .returning(File::as_returning())
        .get_result(conn)
        .expect("Error saving new file")
}

pub fn create_group(conn: &mut SqliteConnection, name: &str) -> Group {
    use crate::schema::groups;

    let new_group = NewGroup { name };

    diesel::insert_into(groups::table)
        .values(&new_group)
        .returning(Group::as_returning())
        .get_result(conn)
        .expect("Error saving new group")
}

pub fn create_tag(conn: &mut SqliteConnection, name: &str) -> Tag {
    use crate::schema::tags;

    let new_tag = NewTag { name };

    diesel::insert_into(tags::table)
        .values(&new_tag)
        .returning(Tag::as_returning())
        .get_result(conn)
        .expect("Error saving new tag")
}
