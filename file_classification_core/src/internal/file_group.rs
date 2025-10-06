use super::models::{FileGroupCondition, FileGroupDTO};
use crate::model::schema::{file_groups, files};
use diesel::prelude::*;
use crate::utils::database::AnyConnection;
use crate::model::models::{File, FileGroupOrderBy, FileGroupQueryOptions, OrderDirection};
use crate::utils::errors::AppError;
use crate::utils::errors::AppError::CannotUnbindPrimaryGroup;

pub fn insert_file_group(
    conn: &mut AnyConnection,
    file_group_dto: &FileGroupDTO,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(file_groups::table)
        .values(file_group_dto)
        .execute(conn)
}

pub fn delete_file_group_by_id(
    conn: &mut AnyConnection,
    file_group_dto: &FileGroupDTO,
) -> Result<usize, AppError> {
    // 判断是否是文件-主组关系 若是则抛异常
    let file = files::table
        .select(File::as_select())
        .filter(files::id.eq(file_group_dto.file_id))
        .first(conn);

    if file.is_ok() && file?.group_id == file_group_dto.group_id {
        return Err(CannotUnbindPrimaryGroup);
    }

    // 删除关联记录
    diesel::delete(
        file_groups::table
            .filter(file_groups::group_id.eq(file_group_dto.group_id))
            .filter(file_groups::file_id.eq(file_group_dto.file_id)),
    )
    .execute(conn)
    .map_err(AppError::from) // 将 QueryResult 转换为 AppError
}

// 将 FileGroupCondition 转换为 diesel 查询条件的辅助函数
fn build_file_group_condition(condition: FileGroupCondition) -> Box<dyn BoxableExpression<file_groups::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>> {
    match condition {
        FileGroupCondition::FileId(id) => Box::new(file_groups::file_id.eq(id)),
        FileGroupCondition::GroupId(id) => Box::new(file_groups::group_id.eq(id)),

        FileGroupCondition::FileIdGreaterThan(value) => Box::new(file_groups::file_id.gt(value)),
        FileGroupCondition::FileIdLessThan(value) => Box::new(file_groups::file_id.lt(value)),
        FileGroupCondition::GroupIdGreaterThan(value) => Box::new(file_groups::group_id.gt(value)),
        FileGroupCondition::GroupIdLessThan(value) => Box::new(file_groups::group_id.lt(value)),

        FileGroupCondition::FileIdIn(values) => Box::new(file_groups::file_id.eq_any(values)),
        FileGroupCondition::GroupIdIn(values) => Box::new(file_groups::group_id.eq_any(values)),

        FileGroupCondition::And(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<file_groups::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>>> = None;
            for cond in conditions {
                let expr = build_file_group_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.and(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(true.into_sql::<diesel::sql_types::Bool>()))
        }
        FileGroupCondition::Or(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<file_groups::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>>> = None;
            for cond in conditions {
                let expr = build_file_group_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.or(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(false.into_sql::<diesel::sql_types::Bool>()))
        }
        FileGroupCondition::Not(condition) => {
            let expr = build_file_group_condition(*condition);
            Box::new(diesel::dsl::not(expr))
        }
    }
}

// 根据 FileGroupCondition 向量查询文件组关联
pub fn select_file_groups_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileGroupCondition>,
    limit: Option<i64>,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
    let mut query = file_groups::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_file_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    if let Some(limit) = limit {
        query = query.limit(limit)
    }

    query
        .select(FileGroupDTO::as_select())
        .load(conn)
}

#[allow(dead_code)]
pub fn select_file_groups_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<FileGroupCondition>,
    options: FileGroupQueryOptions,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
    let mut query = file_groups::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_file_group_condition(condition);
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
            FileGroupOrderBy::FileId(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(file_groups::file_id.asc()),
                    OrderDirection::Desc => query.order(file_groups::file_id.desc()),
                }
            }
            FileGroupOrderBy::GroupId(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(file_groups::group_id.asc()),
                    OrderDirection::Desc => query.order(file_groups::group_id.desc()),
                }
            }
        };
    }

    query
        .select(FileGroupDTO::as_select())
        .load(conn)
}

pub fn delete_file_groups_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileGroupCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::delete(file_groups::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_file_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.execute(conn)
}
