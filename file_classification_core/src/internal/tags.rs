use crate::
	errors::AppError
;
use diesel::prelude::*;
use super::{
	models::{NewTag, Tag,SearchTag},
	schema::tags,
	schema::tags::dsl::*,
};
pub fn create_tag(
	conn: &mut SqliteConnection,
	new_tag: NewTag,
) -> Result<Tag, diesel::result::Error> {

	diesel::insert_into(tags::table).values(&new_tag).returning(Tag::as_returning()).get_result(conn)
}
#[allow(dead_code)]
pub fn find_tag_by_name(
	conn: &mut SqliteConnection,
	tag_name: &str,
) -> Result<Option<Tag>, AppError> {

	let tag =
		tags.select(Tag::as_select()).filter(tags::name.eq(tag_name)).first::<Tag>(conn).optional()?;

	Ok(tag)
}
#[allow(dead_code)]
pub fn find_tag_by_id(
	conn: &mut SqliteConnection,
	tag_id: i32,
) -> Result<Option<Tag>, AppError> {

	let tag =
		tags.select(Tag::as_select()).filter(tags::id.eq(tag_id)).first::<Tag>(conn).optional()?;

	Ok(tag)
}
pub fn increase_tag_reference_count(
	conn: &mut SqliteConnection,
	tag_id: i32,
) -> Result<(), AppError> {

	diesel::update(tags::table.find(tag_id))
		.set(tags::reference_count.eq(tags::reference_count + 1))
		.execute(conn)?;

	Ok(())
}
#[allow(dead_code)]
pub fn decrease_tag_reference_count(
	conn: &mut SqliteConnection,
	tag_id: i32,
) -> Result<(), AppError> {

	diesel::update(tags::table.find(tag_id))
		.set(tags::reference_count.eq(tags::reference_count - 1))
		.execute(conn)?;

	Ok(())
}

pub fn select_tags(
	conn: &mut SqliteConnection,
	search_input: SearchTag,
	limit: i64,
) -> Result<Vec<Tag>, diesel::result::Error> {

	// 使用 into_boxed() 来对查询进行类型擦除
	let mut base_query = tags.limit(limit).select(Tag::as_select()).into_boxed();

	// 如果 search_input 中有 id，则添加过滤条件
	if let Some(tag_id) = search_input.id {
		base_query = base_query.filter(tags::id.eq(tag_id));
	}
	if let Some(tag_name) = search_input.name {
		base_query = base_query.filter(tags::name.eq(tag_name));
	}

	// 执行查询
	base_query.load(conn)
}

pub fn delete_tag(conn: &mut SqliteConnection, tag_id: i32) -> Result<(), diesel::result::Error> {

	match diesel::delete(tags.filter(tags::id.eq(tag_id))).execute(conn) {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}

use std::fmt::{Debug, Formatter, Result as fmtResult};
impl Debug for Tag {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
		write!(
			f,
			"Tag {{ id: {}, name: {}, reference_count: {} }}",
			self.id, self.name, self.reference_count
		)
	}
}