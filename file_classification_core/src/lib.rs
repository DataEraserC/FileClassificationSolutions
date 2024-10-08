pub mod models;
pub mod schema;

use self::models::{
	File, Group, NewFile, NewFileGroup, NewGroup, NewGroupTag, NewTag, SearchFile, SearchGroup,
	SearchTag, Tag,
};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub fn establish_connection() -> SqliteConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	SqliteConnection::establish(&database_url)
		.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn _create_file(
	conn: &mut SqliteConnection,
	type_: &str,
	path: &str,
	group_id: i32,
) -> Result<File, diesel::result::Error> {
	use crate::schema::files;

	let new_file: NewFile<'_> = NewFile { type_, path, group_id };

	diesel::insert_into(files::table)
		.values(&new_file)
		.returning(File::as_returning())
		.get_result(conn)
}
pub fn create_file(
	conn: &mut SqliteConnection,
	name: &str,
	type_: &str,
	path: &str,
) -> Result<(File, Group), diesel::result::Error> {
	let group = match find_group_by_name(conn, name)? {
		Some(existing_group) => existing_group,
		None => create_group(conn, name)?,
	};

	let file = _create_file(conn, type_, path, group.id)?;

	create_file_group(conn, file.id, group.id)?;
	mark_group_as_primary(conn, group.id)?;

	Ok((file, group))
}

pub fn find_group_by_name(
	conn: &mut SqliteConnection,
	group_name: &str,
) -> Result<Option<Group>, diesel::result::Error> {
	use crate::schema::groups;
	use crate::schema::groups::dsl::*;

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
) -> Result<Option<Group>, diesel::result::Error> {
	use crate::schema::groups;
	use crate::schema::groups::dsl::*;

	let group = groups
		.select(Group::as_select())
		.filter(groups::id.eq(group_id))
		.first::<Group>(conn)
		.optional()?;
	Ok(group)
}
pub fn find_tag_by_name(
	conn: &mut SqliteConnection,
	tag_name: &str,
) -> Result<Option<Tag>, diesel::result::Error> {
	use schema::tags::dsl::*;

	let tag =
		tags.select(Tag::as_select()).filter(name.eq(tag_name)).first::<Tag>(conn).optional()?;

	Ok(tag)
}
pub fn find_tag_by_id(
	conn: &mut SqliteConnection,
	tag_id: i32,
) -> Result<Option<Tag>, diesel::result::Error> {
	use crate::schema::tags;
	use schema::tags::dsl::*;

	let tag =
		tags.select(Tag::as_select()).filter(tags::id.eq(tag_id)).first::<Tag>(conn).optional()?;

	Ok(tag)
}
pub fn create_group(
	conn: &mut SqliteConnection,
	name: &str,
) -> Result<Group, diesel::result::Error> {
	use crate::schema::groups;

	let new_group = NewGroup { name };

	diesel::insert_into(groups::table)
		.values(&new_group)
		.returning(Group::as_returning())
		.get_result(conn)
}

pub fn create_tag(conn: &mut SqliteConnection, name: &str) -> Result<Tag, diesel::result::Error> {
	use crate::schema::tags;

	let new_tag = NewTag { name };

	diesel::insert_into(tags::table).values(&new_tag).returning(Tag::as_returning()).get_result(conn)
}

use diesel::result::Error as DieselError;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

// 自定义错误类型
#[derive(Debug)]
pub enum CreateFileGroupError {
	CannotAssociateFileWithPrimaryGroup,
	DieselError(DieselError),
}

impl Display for CreateFileGroupError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			CreateFileGroupError::CannotAssociateFileWithPrimaryGroup => {
				write!(f, "Cannot associate file with a primary group")
			}
			CreateFileGroupError::DieselError(err) => write!(f, "{}", err),
		}
	}
}

impl Error for CreateFileGroupError {}

// 从 DieselError 转换到 CreateFileGroupError
impl From<DieselError> for CreateFileGroupError {
	fn from(err: DieselError) -> Self {
		CreateFileGroupError::DieselError(err)
	}
}

// 从 CreateFileGroupError 转换到 DieselError
impl From<CreateFileGroupError> for DieselError {
	fn from(err: CreateFileGroupError) -> Self {
		match err {
			CreateFileGroupError::CannotAssociateFileWithPrimaryGroup => {
				DieselError::QueryBuilderError(Box::new(err))
			}
			CreateFileGroupError::DieselError(diesel_err) => diesel_err,
		}
	}
}

pub fn create_file_group(
	conn: &mut SqliteConnection,
	file_id: i32,
	group_id: i32,
) -> Result<(), CreateFileGroupError> {
	use crate::schema::file_groups;

	let group = find_group_by_id(conn, group_id)?
		.ok_or(CreateFileGroupError::DieselError(DieselError::NotFound))?;

	if group.is_primary {
		return Err(CreateFileGroupError::CannotAssociateFileWithPrimaryGroup);
	}

	conn.transaction::<_, CreateFileGroupError, _>(|conn| {
		let new_file_group = NewFileGroup { file_id, group_id };

		increase_file_reference_count(conn, file_id)?;

		diesel::insert_into(file_groups::table).values(&new_file_group).execute(conn)?;

		Ok(())
	})
}

pub fn create_group_tag(
	conn: &mut SqliteConnection,
	group_id: i32,
	tag_id: i32,
) -> Result<(), diesel::result::Error> {
	use crate::schema::group_tags;

	let new_group_tag = NewGroupTag { group_id, tag_id };

	diesel::insert_into(group_tags::table).values(&new_group_tag).execute(conn)?;

	Ok(())
}

pub fn increase_file_reference_count(
	conn: &mut SqliteConnection,
	file_id: i32,
) -> Result<(), diesel::result::Error> {
	use crate::schema::files;

	diesel::update(files::table.find(file_id))
		.set(files::reference_count.eq(files::reference_count + 1))
		.execute(conn)?;

	Ok(())
}
pub fn decrease_file_reference_count(
	conn: &mut SqliteConnection,
	file_id: i32,
) -> Result<(), diesel::result::Error> {
	use crate::schema::files;

	diesel::update(files::table.find(file_id))
		.set(files::reference_count.eq(files::reference_count - 1))
		.execute(conn)?;

	Ok(())
}

pub fn mark_group_as_primary(
	conn: &mut SqliteConnection,
	group_id: i32,
) -> Result<(), diesel::result::Error> {
	use crate::schema::groups;

	diesel::update(groups::table)
		.filter(groups::id.eq(group_id))
		.set(groups::is_primary.eq(true))
		.execute(conn)?;

	Ok(())
}
pub fn mark_group_as_non_primary(conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
	use crate::schema::groups;

	diesel::update(groups::table).set(groups::is_primary.eq(false)).execute(conn)?;

	Ok(())
}
pub fn select_tags(
	conn: &mut SqliteConnection,
	search_input: SearchTag,
	limit: i64,
) -> Result<Vec<Tag>, diesel::result::Error> {
	use self::models::*;
	use self::schema::tags::dsl::*;

	// 使用 into_boxed() 来对查询进行类型擦除
	let mut base_query = tags.limit(limit).select(Tag::as_select()).into_boxed();

	// 如果 search_input 中有 id，则添加过滤条件
	if let Some(tag_id) = search_input.id {
		base_query = base_query.filter(id.eq(tag_id));
	}
	if let Some(tag_name) = search_input.name {
		base_query = base_query.filter(name.eq(tag_name));
	}

	// 执行查询
	base_query.load(conn)
}
pub fn select_files(
	conn: &mut SqliteConnection,
	search_input: SearchFile,
	limit: i64,
) -> Result<Vec<File>, diesel::result::Error> {
	use self::models::*;
	use self::schema::files::dsl::*;

	// 使用 into_boxed() 来对查询进行类型擦除
	let mut base_query = files.limit(limit).select(File::as_select()).into_boxed();

	// 如果 search_input 中有各字段，则添加相应的过滤条件
	if let Some(file_id) = search_input.id {
		base_query = base_query.filter(id.eq(file_id));
	}
	if let Some(file_type) = search_input.type_ {
		base_query = base_query.filter(type_.eq(file_type));
	}
	if let Some(file_path) = search_input.path {
		base_query = base_query.filter(path.eq(file_path));
	}
	if let Some(ref_count) = search_input.reference_count {
		base_query = base_query.filter(reference_count.eq(ref_count));
	}
	if let Some(group) = search_input.group_id {
		base_query = base_query.filter(group_id.eq(group));
	}

	// 执行查询
	base_query.load(conn)
}
pub fn select_groups(
	conn: &mut SqliteConnection,
	search_input: SearchGroup,
	limit: i64,
) -> Result<Vec<Group>, diesel::result::Error> {
	use self::models::*;
	use self::schema::groups::dsl::*;

	// 使用 into_boxed() 来对查询进行类型擦除
	let mut base_query = groups.limit(limit).select(Group::as_select()).into_boxed();

	// 如果 search_input 中有各字段，则添加相应的过滤条件
	if let Some(group_id) = search_input.id {
		base_query = base_query.filter(id.eq(group_id));
	}
	if let Some(group_name) = search_input.name {
		base_query = base_query.filter(name.eq(group_name));
	}
	if let Some(ref_count) = search_input.reference_count {
		base_query = base_query.filter(reference_count.eq(ref_count));
	}
	if let Some(is_primary_val) = search_input.is_primary {
		base_query = base_query.filter(is_primary.eq(is_primary_val));
	}
	if let Some(clicks) = search_input.click_count {
		base_query = base_query.filter(click_count.eq(clicks));
	}
	if let Some(shares) = search_input.share_count {
		base_query = base_query.filter(share_count.eq(shares));
	}
	if let Some(created) = search_input.create_time {
		base_query = base_query.filter(create_time.eq(created));
	}
	if let Some(modified) = search_input.modify_time {
		base_query = base_query.filter(modify_time.eq(modified));
	}

	// 执行查询
	base_query.load(conn)
}

pub fn delete_group(
	conn: &mut SqliteConnection,
	group_id: i32,
) -> Result<(), diesel::result::Error> {
	use self::schema::groups::dsl::*;
	match diesel::delete(groups.filter(id.eq(group_id))).execute(conn) {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}

pub fn delete_file(conn: &mut SqliteConnection, file_id: i32) -> Result<(), diesel::result::Error> {
	use self::schema::files::dsl::*;

	match diesel::delete(files.filter(id.eq(file_id))).execute(conn) {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}

pub fn delete_tag(conn: &mut SqliteConnection, tag_id: i32) -> Result<(), diesel::result::Error> {
	use self::schema::tags::dsl::*;

	match diesel::delete(tags.filter(id.eq(tag_id))).execute(conn) {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}
