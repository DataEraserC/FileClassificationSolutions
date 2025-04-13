use crate::
	errors::AppError;
use diesel::prelude::*;
use std::fmt::{Debug, Formatter, Result as fmtResult};
use super::{
    groups::increase_group_reference_count,tags::increase_tag_reference_count,  models::NewGroupTag, schema::group_tags
};
pub fn create_group_tag(
	conn: &mut SqliteConnection,
	group_id: i32,
	tag_id: i32,
) -> Result<NewGroupTag, AppError> {

	let new_group_tag = NewGroupTag { group_id, tag_id };
	let result = conn.transaction::<_, AppError, _>(|conn| {
		increase_group_reference_count(conn, group_id)?;
		increase_tag_reference_count(conn, tag_id)?;
		Ok(diesel::insert_into(group_tags::table).values(&new_group_tag).execute(conn))
	});
	let _ = result?;
	Ok(new_group_tag)
}
impl Debug for NewGroupTag {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
		write!(f, "NewGroupTag {{ group_id: {}, tag_id: {} }}", self.group_id, self.tag_id)
	}
}