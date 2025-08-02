use crate::{
    internal::tags,
    model::models::{CreateTagDTO, Tag, TagFilter},
};
use diesel::SqliteConnection;
use crate::model::models::{TagCondition, UpdateTagDTO};
use crate::utils::errors::AppError;

pub fn create_tag(conn: &mut SqliteConnection, name: &str) -> Result<Tag, diesel::result::Error> {
    let new_tag = CreateTagDTO { name };
    tags::create_tag(conn, new_tag)
}

pub fn delete_tag(conn: &mut SqliteConnection, tag_id: i32) -> Result<(), diesel::result::Error> {
    tags::delete_tag(conn, tag_id)
}

pub fn select_tags(
    conn: &mut SqliteConnection,
    search_input: TagFilter,
    limit: i64,
) -> Result<Vec<Tag>, diesel::result::Error> {
    tags::select_tags(conn, search_input, limit)
}

pub fn select_tags_by_conditions(
    conn: &mut SqliteConnection,
    condition: Vec<TagCondition>,
    limit: i64,
) -> Result<Vec<Tag>, diesel::result::Error> {
    tags::select_tags_by_conditions(conn, condition, limit)
}

pub fn update_tags_by_conditions(
    conn: &mut SqliteConnection,
    conditions: Vec<TagCondition>,
    update_set: UpdateTagDTO,
) -> Result<usize, AppError> {
    tags::update_tags_by_conditions(conn, conditions, update_set)
}
