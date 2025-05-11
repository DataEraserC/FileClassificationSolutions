use super::{
	models::{File, NewFile, SearchFile},
	schema::files,
	schema::files::dsl::*,
};
use crate::errors::AppError;
use diesel::prelude::*;

pub fn create_file(conn: &mut SqliteConnection, new_file: &NewFile) -> Result<File, AppError> {
	diesel::insert_into(files::table)
		.values(new_file)
		.returning(File::as_returning())
		.get_result(conn)
		.map_err(|e| AppError::CreateFileFailed(e.to_string()))
}

pub fn increase_file_reference_count(
	conn: &mut SqliteConnection,
	file_id: i32,
) -> Result<(), AppError> {
	diesel::update(files::table.find(file_id))
		.set(files::reference_count.eq(files::reference_count + 1))
		.execute(conn)?;

	Ok(())
}

#[allow(dead_code)]
pub fn decrease_file_reference_count(
	conn: &mut SqliteConnection,
	file_id: i32,
) -> Result<(), AppError> {
	diesel::update(files::table.find(file_id))
		.set(files::reference_count.eq(files::reference_count - 1))
		.execute(conn)?;

	Ok(())
}

pub fn select_files(
	conn: &mut SqliteConnection,
	search_input: SearchFile,
	limit: i64,
) -> Result<Vec<File>, diesel::result::Error> {
	// 使用 into_boxed() 来对查询进行类型擦除
	let mut base_query = files.limit(limit).select(File::as_select()).into_boxed();

	// 如果 search_input 中有各字段，则添加相应的过滤条件
	if let Some(file_id) = search_input.id {
		base_query = base_query.filter(files::id.eq(file_id));
	}
	if let Some(file_type) = search_input.type_ {
		base_query = base_query.filter(files::type_.eq(file_type));
	}
	if let Some(file_path) = search_input.path {
		base_query = base_query.filter(files::path.eq(file_path));
	}
	if let Some(ref_count) = search_input.reference_count {
		base_query = base_query.filter(files::reference_count.eq(ref_count));
	}
	if let Some(group) = search_input.group_id {
		base_query = base_query.filter(files::group_id.eq(group));
	}

	// 执行查询
	base_query.load(conn)
}

pub fn delete_file(conn: &mut SqliteConnection, file_id: i32) -> Result<(), diesel::result::Error> {
	match diesel::delete(files.filter(files::id.eq(file_id))).execute(conn) {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}
use std::fmt::{Debug, Formatter, Result as fmtResult};
impl Debug for File {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
		write!(
			f,
			"File {{ id: {}, type_: {}, path: {}, reference_count: {}, group_id: {} }}",
			self.id, self.type_, self.path, self.reference_count, self.group_id
		)
	}
}
