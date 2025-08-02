use diesel::SqliteConnection;

use crate::errors::AppError;
use crate::internal::group_tag as group_tags;
use crate::internal::models::GroupTagDTO;
pub fn create_group_tag(
	conn: &mut SqliteConnection,
	group_id: i32,
	tag_id: i32,
) -> Result<GroupTagDTO, AppError> {
	group_tags::create_group_tag(conn, group_id, tag_id)
}
