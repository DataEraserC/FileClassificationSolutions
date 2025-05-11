use super::{
	models::{Group, GroupFilter, NewGroup},
	schema::groups,
	schema::groups::dsl::*,
};
use crate::errors::AppError;
use diesel::prelude::*;
pub fn create_group(conn: &mut SqliteConnection, new_group: &NewGroup) -> Result<Group, AppError> {
	diesel::insert_into(groups::table)
		.values(new_group)
		.returning(Group::as_returning())
		.get_result(conn)
		.map_err(|e| AppError::CreateGroupFailed(e.to_string()))
}

pub fn find_group_by_name(
	conn: &mut SqliteConnection,
	group_name: &str,
) -> Result<Option<Group>, AppError> {
	let group = groups
		.select(Group::as_select())
		.filter(groups::name.eq(group_name))
		.first::<Group>(conn)
		.optional()?;
	Ok(group)
}
pub fn find_group_by_id(
	conn: &mut SqliteConnection,
	group_id: i32,
) -> Result<Option<Group>, AppError> {
	let group = groups
		.select(Group::as_select())
		.filter(groups::id.eq(group_id))
		.first::<Group>(conn)
		.optional()?;
	Ok(group)
}

pub fn mark_group_as_primary(conn: &mut SqliteConnection, group_id: i32) -> Result<(), AppError> {
	diesel::update(groups::table)
		.filter(groups::id.eq(group_id))
		.set(groups::is_primary.eq(true))
		.execute(conn)?;

	Ok(())
}

#[allow(dead_code)]
pub fn mark_group_as_non_primary(conn: &mut SqliteConnection) -> Result<(), AppError> {
	diesel::update(groups::table).set(groups::is_primary.eq(false)).execute(conn)?;

	Ok(())
}

pub fn select_groups(
	conn: &mut SqliteConnection,
	search_input: GroupFilter,
	limit: i64,
) -> Result<Vec<Group>, diesel::result::Error> {
	// 使用 into_boxed() 来对查询进行类型擦除
	let mut base_query = groups.limit(limit).select(Group::as_select()).into_boxed();

	// 如果 search_input 中有各字段，则添加相应的过滤条件
	if let Some(group_id) = search_input.id {
		base_query = base_query.filter(groups::id.eq(group_id));
	}
	if let Some(group_name) = search_input.name {
		base_query = base_query.filter(groups::name.eq(group_name));
	}
	if let Some(ref_count) = search_input.reference_count {
		base_query = base_query.filter(groups::reference_count.eq(ref_count));
	}
	if let Some(is_primary_val) = search_input.is_primary {
		base_query = base_query.filter(groups::is_primary.eq(is_primary_val));
	}
	if let Some(clicks) = search_input.click_count {
		base_query = base_query.filter(groups::click_count.eq(clicks));
	}
	if let Some(shares) = search_input.share_count {
		base_query = base_query.filter(groups::share_count.eq(shares));
	}
	if let Some(created) = search_input.create_time {
		base_query = base_query.filter(groups::create_time.eq(created));
	}
	if let Some(modified) = search_input.modify_time {
		base_query = base_query.filter(groups::modify_time.eq(modified));
	}

	// 执行查询
	base_query.load(conn)
}

pub fn delete_group(
	conn: &mut SqliteConnection,
	group_id: i32,
) -> Result<(), diesel::result::Error> {
	match diesel::delete(groups.filter(groups::id.eq(group_id))).execute(conn) {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}
use std::fmt::{Debug, Formatter, Result as fmtResult};

pub fn increase_group_reference_count(
	conn: &mut SqliteConnection,
	group_id: i32,
) -> Result<(), AppError> {
	diesel::update(groups::table.find(group_id))
		.set(groups::reference_count.eq(groups::reference_count + 1))
		.execute(conn)?;

	Ok(())
}
#[allow(dead_code)]
pub fn decrease_group_reference_count(
	conn: &mut SqliteConnection,
	group_id: i32,
) -> Result<(), AppError> {
	diesel::update(groups::table.find(group_id))
		.set(groups::reference_count.eq(groups::reference_count - 1))
		.execute(conn)?;

	Ok(())
}
impl Debug for Group {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
		write!(
			f,
			"Group {{ id: {}, name: {}, reference_count: {}, is_primary: {}, click_count: {}, share_count: {}, create_time: {}, modify_time: {} }}",
			self.id,
			self.name,
			self.reference_count,
			self.is_primary,
			self.click_count,
			self.share_count,
			self.create_time,
			self.modify_time
		)
	}
}
