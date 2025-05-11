use crate::{
	errors::AppError,
	internal::{
		groups,
		models::{Group, NewGroup},
	},
	models::GroupFilter,
};
use diesel::SqliteConnection;

pub fn create_group(conn: &mut SqliteConnection, name: &str) -> Result<Group, AppError> {
	let new_group = NewGroup { name };
	groups::create_group(conn, &new_group)
}

pub fn find_group_by_name(
	conn: &mut SqliteConnection,
	name: &str,
) -> Result<Option<Group>, AppError> {
	Ok(groups::find_group_by_name(conn, name)?)
}

pub fn delete_group(
	conn: &mut SqliteConnection,
	group_id: i32,
) -> Result<(), diesel::result::Error> {
	groups::delete_group(conn, group_id)
}
pub fn select_groups(
	conn: &mut SqliteConnection,
	search_input: GroupFilter,
	limit: i64,
) -> Result<Vec<Group>, diesel::result::Error> {
	groups::select_groups(conn, search_input, limit)
}
