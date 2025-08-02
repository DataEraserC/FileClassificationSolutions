use super::database::SqliteConnection;
use crate::internal::file_group as file_groups;
use crate::model::models::{FileGroupCondition, FileGroupDTO};
use crate::service::AppError;

pub fn create_file_group(
    conn: &mut SqliteConnection,
    file_id: i32,
    group_id: i32,
) -> Result<FileGroupDTO, AppError> {
    file_groups::create_file_group(conn, file_id, group_id)
}

pub fn select_file_groups_by_conditions(
    conn: &mut SqliteConnection,
    condition: Vec<FileGroupCondition>,
    limit: i64,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
    crate::internal::file_group::select_file_groups_by_conditions(conn, condition, limit)
}
