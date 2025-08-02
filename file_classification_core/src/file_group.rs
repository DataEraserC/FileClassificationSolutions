use diesel::SqliteConnection;

use crate::errors::AppError;
use crate::internal::file_group as file_groups;
use crate::internal::models::FileGroupDTO;

pub fn create_file_group(
	conn: &mut SqliteConnection,
	file_id: i32,
	group_id: i32,
) -> Result<FileGroupDTO, AppError> {
	file_groups::create_file_group(conn, file_id, group_id)
}
