pub mod models;
pub mod schema;

use self::models::{File, Group, NewFile, NewFileGroup, NewGroup, NewGroupTag, NewTag, Tag};
use chrono;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn _create_file(
    conn: &mut SqliteConnection,
    type_: &str,
    path: &str,
) -> Result<File, diesel::result::Error> {
    use crate::schema::files;

    let new_file = NewFile { type_, path };

    diesel::insert_into(files::table)
        .values(&new_file)
        .returning(File::as_returning())
        .get_result(conn)
}
pub fn create_file(
    conn: &mut SqliteConnection,
    name: &str,
    type_: &str,
    path: &str,
) -> Result<(File, Group), diesel::result::Error> {
    let group = match find_group_by_name(conn, name)? {
        Some(existing_group) => existing_group,
        None => create_group(conn, name)?,
    };

    let file = _create_file(conn, type_, path)?;

    create_file_group(conn, file.id.unwrap(), group.id.unwrap())?;

    Ok((file, group))
}

pub fn find_group_by_name(
    conn: &mut SqliteConnection,
    name_input: &str,
) -> Result<Option<Group>, diesel::result::Error> {
    use crate::schema::groups::dsl::*;

    let group = groups
        .select(Group::as_select())
        .filter(name.eq(name_input))
        .first::<Group>(conn)
        .optional()?;
    Ok(group)
}

pub fn find_tag_by_name(
    conn: &mut SqliteConnection,
    name_input: &str,
) -> Result<Option<Tag>, diesel::result::Error> {
    use schema::tags::dsl::*;

    let tag = tags
        .select(Tag::as_select())
        .filter(name.eq(name_input))
        .first::<Tag>(conn)
        .optional()?;

    Ok(tag)
}
pub fn create_group(
    conn: &mut SqliteConnection,
    name: &str,
) -> Result<Group, diesel::result::Error> {
    use crate::schema::groups;

    let current_time = chrono::Utc::now().timestamp();

    let new_group = NewGroup {
        name,
        create_time: &current_time,
        modify_time: &current_time,
    };

    diesel::insert_into(groups::table)
        .values(&new_group)
        .returning(Group::as_returning())
        .get_result(conn)
}

pub fn create_tag(conn: &mut SqliteConnection, name: &str) -> Result<Tag, diesel::result::Error> {
    use crate::schema::tags;

    let new_tag = NewTag { name };

    diesel::insert_into(tags::table)
        .values(&new_tag)
        .returning(Tag::as_returning())
        .get_result(conn)
}

pub fn create_file_group(
    conn: &mut SqliteConnection,
    file_id: i32,
    group_id: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::file_groups;

    let new_file_group = NewFileGroup {
        file_id: Some(file_id),
        group_id: Some(group_id),
    };

    diesel::insert_into(file_groups::table)
        .values(&new_file_group)
        .execute(conn)?;

    Ok(())
}

pub fn create_group_tag(
    conn: &mut SqliteConnection,
    group_id: i32,
    tag_id: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::group_tags;

    let new_group_tag = NewGroupTag {
        group_id: Some(group_id),
        tag_id: Some(tag_id),
    };

    diesel::insert_into(group_tags::table)
        .values(&new_group_tag)
        .execute(conn)?;

    Ok(())
}
