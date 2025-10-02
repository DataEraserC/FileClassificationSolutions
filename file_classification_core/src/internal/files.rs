use super::models::{CreateFileDTO, File, FileCondition, FileFilter, UpdateFileDTO};
use diesel::prelude::*;
use crate::utils::database::AnyConnection;

pub fn create_file(conn: &mut AnyConnection, new_file: &CreateFileDTO) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(files::table)
        .values(new_file).execute(conn)
}

pub fn find_file_by_id(conn: &mut AnyConnection, _id: i32) -> Result<Option<File>, diesel::result::Error> {
    files::table
        .filter(files::id.eq(_id))
        .select((files::id, files::type_, files::path, files::reference_count, files::group_id))
        .first(conn)
        .optional()
}

pub fn increase_file_reference_count(
    conn: &mut AnyConnection,
    file_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(files::table.find(file_id))
        .set(files::reference_count.eq(files::reference_count + 1))
        .execute(conn)
}

pub fn decrease_file_reference_count(
    conn: &mut AnyConnection,
    file_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(files::table.find(file_id))
        .set(files::reference_count.eq(files::reference_count - 1))
        .execute(conn)
}

#[deprecated]
pub fn select_files(
    conn: &mut AnyConnection,
    search_input: FileFilter,
    limit: i64,
) -> Result<Vec<File>, diesel::result::Error> {
    // 使用 into_boxed() 来对查询进行类型擦除
    let mut base_query = files.limit(limit).into_boxed::<<AnyConnection as Connection>::Backend>();

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
    base_query
        .select((files::id, files::type_, files::path, files::reference_count, files::group_id))
        .load(conn)
}

pub fn delete_file_by_id(conn: &mut AnyConnection, file_id: i32) -> Result<usize, diesel::result::Error> {
    diesel::delete(files.filter(files::id.eq(file_id))).execute(conn)
}

use crate::model::models::{FileOrderBy, FileQueryOptions, OrderDirection};
use crate::model::schema::files;
use crate::model::schema::files::dsl::*;
use diesel::sql_types::Bool;
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

// 将 FileCondition 转换为 diesel 查询条件的辅助函数
// 更新 build_condition 函数以处理新增的条件类型
fn build_file_condition(condition: FileCondition) -> Box<dyn BoxableExpression<files::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>> {
    match condition {
        FileCondition::Id(_id) => Box::new(files::id.eq(_id)),
        FileCondition::Type(t) => Box::new(files::type_.eq(t)),
        FileCondition::Path(p) => Box::new(files::path.eq(p)),
        FileCondition::ReferenceCount(rc) => Box::new(files::reference_count.eq(rc)),
        FileCondition::GroupId(gid) => Box::new(files::group_id.eq(gid)),

        // 新增条件的处理
        FileCondition::IdGreaterThan(value) => Box::new(files::id.gt(value)),
        FileCondition::IdLessThan(value) => Box::new(files::id.lt(value)),
        FileCondition::TypeLike(pattern) => Box::new(files::type_.like(pattern)),
        FileCondition::PathLike(pattern) => Box::new(files::path.like(pattern)),
        FileCondition::ReferenceCountGreaterThan(value) => Box::new(files::reference_count.gt(value)),
        FileCondition::ReferenceCountLessThan(value) => Box::new(files::reference_count.lt(value)),
        FileCondition::GroupIdGreaterThan(value) => Box::new(files::group_id.gt(value)),
        FileCondition::GroupIdLessThan(value) => Box::new(files::group_id.lt(value)),
        
        FileCondition::IdIn(values) => Box::new(files::id.eq_any(values)),
        FileCondition::TypeIn(values) => Box::new(files::type_.eq_any(values)),
        FileCondition::PathIn(values) => Box::new(files::path.eq_any(values)),
        FileCondition::ReferenceCountIn(values) => Box::new(files::reference_count.eq_any(values)),
        FileCondition::GroupIdIn(values) => Box::new(files::group_id.eq_any(values)),

        FileCondition::And(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<files::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>>> = None;
            for cond in conditions {
                let expr = build_file_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.and(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(true.into_sql::<Bool>()))
        }
        FileCondition::Or(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<files::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>>> = None;
            for cond in conditions {
                let expr = build_file_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.or(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(false.into_sql::<Bool>()))
        }
        FileCondition::Not(condition) => {
            let expr = build_file_condition(*condition);
            Box::new(diesel::dsl::not(expr))
        }
    }
}

// 修改 select_files_by_condition 函数以接受 Vec<FileCondition>
pub fn select_files_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileCondition>,
    limit: Option<i64>,
) -> Result<Vec<File>, diesel::result::Error> {
    let mut query = files::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_file_condition(condition);
        query = query.filter(boxed_condition);
    }

    if let Some(limit) = limit {
        query = query.limit(limit)
    }

    query
        .select((files::id, files::type_, files::path, files::reference_count, files::group_id))
        .load(conn)
}

#[allow(dead_code)]
pub fn select_files_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<FileCondition>,
    options: FileQueryOptions,
) -> Result<Vec<File>, diesel::result::Error> {
    let mut query = files::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_file_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 应用查询选项（排序、限制等）
    if let Some(limit) = options.limit {
        query = query.limit(limit);
    }

    if let Some(offset) = options.offset {
        query = query.offset(offset);
    }

    // 应用排序
    for order_by in options.order_by {
        query = match order_by {
            FileOrderBy::Id(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(files::id.asc()),
                    OrderDirection::Desc => query.order(files::id.desc()),
                }
            }
            FileOrderBy::Type(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(files::type_.asc()),
                    OrderDirection::Desc => query.order(files::type_.desc()),
                }
            }
            FileOrderBy::Path(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(files::path.asc()),
                    OrderDirection::Desc => query.order(files::path.desc()),
                }
            }
            FileOrderBy::ReferenceCount(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(files::reference_count.asc()),
                    OrderDirection::Desc => query.order(files::reference_count.desc()),
                }
            }
            FileOrderBy::GroupId(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(files::group_id.asc()),
                    OrderDirection::Desc => query.order(files::group_id.desc()),
                }
            }
        };
    }

    query
        .select((files::id, files::type_, files::path, files::reference_count, files::group_id))
        .load(conn)
}

pub fn update_files_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileCondition>,
    update_set: UpdateFileDTO,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::update(files::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_file_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.set(update_set).execute(conn)
}

pub fn delete_files_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::delete(files::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_file_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.execute(conn)
}
pub fn increase_files_reference_count_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::update(files::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_file_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 增加引用计数
    query.set(files::reference_count.eq(files::reference_count + 1)).execute(conn)
}

pub fn decrease_files_reference_count_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::update(files::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_file_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 减少引用计数
    query.set(files::reference_count.eq(files::reference_count - 1)).execute(conn)
}

pub fn select_file_by_group_id(
    conn: &mut AnyConnection,
    other_group_id: i64,
) -> Result<Vec<File>, diesel::result::Error> {
    use crate::model::schema::file_groups;

    files::table
        .inner_join(file_groups::table.on(files::id.eq(file_groups::file_id)))
        .filter(file_groups::group_id.eq(other_group_id as i32))
        .select(File::as_select())
        .load(conn)
}
