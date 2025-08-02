use super::{
    groups::increase_group_reference_count, models::GroupTagDTO, schema::group_tags,
    tags::increase_tag_reference_count,
};
use crate::errors::AppError;
use diesel::prelude::*;
use std::fmt::{Debug, Formatter, Result as fmtResult};
pub fn create_group_tag(
	conn: &mut SqliteConnection,
	group_id: i32,
	tag_id: i32,
) -> Result<GroupTagDTO, AppError> {
	let new_group_tag = GroupTagDTO { group_id, tag_id };
	let result = conn.transaction::<_, AppError, _>(|conn| {
		increase_group_reference_count(conn, group_id)?;
		increase_tag_reference_count(conn, tag_id)?;
		Ok(diesel::insert_into(group_tags::table).values(&new_group_tag).execute(conn))
	});
	let _ = result?;
	Ok(new_group_tag)
}
impl Debug for GroupTagDTO {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
		write!(f, "GroupTag {{ group_id: {}, tag_id: {} }}", self.group_id, self.tag_id)
	}
}
