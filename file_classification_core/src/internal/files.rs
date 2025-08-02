use super::{models::{CreateFileDTO, File, FileFilter}, AppError};
use diesel::prelude::*;

pub fn create_file(conn: &mut SqliteConnection, new_file: &CreateFileDTO) -> Result<File, AppError> {
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
    search_input: FileFilter,
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

// pub fn update_file(
// 	conn: &mut SqliteConnection,
// 	update_input: UpdateFile,
// ) -> Result<usize, diesel::result::Error> {
// 	// 初始化更新查询
// 	let mut query = diesel::update(files).into_boxed();

// 	// 动态设置需要更新的字段
// 	if let Some(new_path) = update_input.set.path {
// 		query = query.set(path.eq(new_path));
// 	}
// 	if let Some(new_type) = update_input.set.type_ {
// 		query = query.set(type_.eq(new_type));
// 	}
// 	if let Some(new_ref_count) = update_input.set.reference_count {
// 		query = query.set(reference_count.eq(new_ref_count));
// 	}
// 	if let Some(new_group) = update_input.set.group_id {
// 		query = query.set(group_id.eq(new_group));
// 	}

// 	// 动态添加过滤条件
// 	if let Some(file_id) = update_input.filter.id {
// 		query = query.filter(id.eq(file_id));
// 	}
// 	if let Some(file_type) = update_input.filter.type_ {
// 		query = query.filter(type_.eq(file_type));
// 	}
// 	if let Some(file_path) = update_input.filter.path {
// 		query = query.filter(path.eq(file_path));
// 	}
// 	if let Some(ref_count) = update_input.filter.reference_count {
// 		query = query.filter(reference_count.eq(ref_count));
// 	}
// 	if let Some(group) = update_input.filter.group_id {
// 		query = query.filter(group_id.eq(group));
// 	}
// 	query.load(conn)
// }

pub fn delete_file(conn: &mut SqliteConnection, file_id: i32) -> Result<(), diesel::result::Error> {
    match diesel::delete(files.filter(files::id.eq(file_id))).execute(conn) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
use crate::model::schema::files;
use crate::model::schema::files::dsl::*;
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
