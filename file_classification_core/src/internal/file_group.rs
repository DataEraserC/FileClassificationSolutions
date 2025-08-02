use crate::{
	errors::AppError,
	internal::{
		files::increase_file_reference_count,
		groups::{find_group_by_id, increase_group_reference_count},
	},
};
use diesel::prelude::*;
use std::fmt::{Debug, Formatter, Result as fmtResult};

use super::{models::FileGroupDTO, schema::file_groups};

pub fn create_file_group(
	conn: &mut SqliteConnection,
	file_id: i32,
	group_id: i32,
) -> Result<FileGroupDTO, AppError> {
	let group = find_group_by_id(conn, group_id)?.ok_or(AppError::GroupNotFound)?;

	if group.is_primary {
		return Err(AppError::CannotAssociateWithPrimary);
	}

	let new_file_group = FileGroupDTO { file_id, group_id };
	let result = conn.transaction::<_, AppError, _>(|conn| {
		increase_file_reference_count(conn, file_id)?;
		increase_group_reference_count(conn, group_id)?;
		Ok(diesel::insert_into(file_groups::table).values(&new_file_group).execute(conn))
	});
	let _ = result?;
	Ok(new_file_group)
}
impl Debug for FileGroupDTO {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
		write!(f, "FileGroup {{ file_id: {}, group_id: {} }}", self.file_id, self.group_id)
	}
}
