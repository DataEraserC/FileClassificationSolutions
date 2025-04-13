use diesel::SqliteConnection;

use crate::errors::AppError;
use crate::internal::models::NewGroupTag;
use crate::internal::group_tag as group_tags;
pub fn create_group_tag(
	conn: &mut SqliteConnection,
	group_id: i32,
	tag_id: i32,
) -> Result<NewGroupTag, AppError> {
group_tags::create_group_tag(conn, group_id, tag_id)

}