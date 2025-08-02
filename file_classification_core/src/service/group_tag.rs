use super::database::SqliteConnection;
use crate::internal::{group_tag as group_tags};
use crate::model::models::{GroupTagCondition, GroupTagDTO};
use crate::service::AppError;

pub fn create_group_tag(
    conn: &mut SqliteConnection,
    group_id: i32,
    tag_id: i32,
) -> Result<GroupTagDTO, AppError> {
    group_tags::create_group_tag(conn, group_id, tag_id)
}

pub fn delete_group_tag(
    conn: &mut SqliteConnection,
    group_id: i32,
    tag_id: i32,
) -> Result<usize, AppError> {
    group_tags::delete_group_tag(conn, group_id, tag_id)
}

pub fn select_group_tags_by_conditions(
    conn: &mut SqliteConnection,
    condition: Vec<GroupTagCondition>,
    limit: i64,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
    group_tags::select_group_tags_by_conditions(conn, condition, limit)
}
