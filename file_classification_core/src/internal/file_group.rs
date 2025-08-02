use super::{models::FileGroupDTO, AppError};
use crate::internal::{
    files::increase_file_reference_count,
    groups::{find_group_by_id, increase_group_reference_count},
};
use crate::model::schema::{file_groups};
use diesel::prelude::*;
use std::fmt::{Debug, Formatter, Result as fmtResult};

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

pub fn delete_file_group(
    conn: &mut SqliteConnection,
    file_id: i32,
    group_id: i32,
) -> Result<usize, AppError> {
    let result = conn.transaction::<_, AppError, _>(|conn| {
        // 减少组和标签的引用计数
        decrease_group_reference_count(conn, group_id)?;
        decrease_file_reference_count(conn, file_id)?;

        // 删除关联记录
        let deleted_count = diesel::delete(
            file_groups::table
                .filter(file_groups::group_id.eq(group_id))
                .filter(file_groups::file_id.eq(file_id))
        )
            .execute(conn)?;

        Ok(deleted_count)
    });

    result.map_err(|e| e)
}

impl Debug for FileGroupDTO {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "FileGroup {{ file_id: {}, group_id: {} }}", self.file_id, self.group_id)
    }
}

// 在 files.rs 文件中添加以下代码（需要添加到文件末尾，在其他 use 语句之后）

use super::models::FileGroupCondition;
use diesel::dsl::not;
use diesel::sql_types::Bool;
use diesel::sqlite::Sqlite;
use crate::internal::files::decrease_file_reference_count;
use crate::internal::groups::decrease_group_reference_count;

// 将 FileGroupCondition 转换为 diesel 查询条件的辅助函数
fn build_file_group_condition(condition: FileGroupCondition) -> Box<dyn BoxableExpression<file_groups::table, Sqlite, SqlType = diesel::sql_types::Bool>> {
    match condition {
        FileGroupCondition::FileId(id) => Box::new(file_groups::file_id.eq(id)),
        FileGroupCondition::GroupId(id) => Box::new(file_groups::group_id.eq(id)),

        FileGroupCondition::FileIdGreaterThan(value) => Box::new(file_groups::file_id.gt(value)),
        FileGroupCondition::FileIdLessThan(value) => Box::new(file_groups::file_id.lt(value)),
        FileGroupCondition::GroupIdGreaterThan(value) => Box::new(file_groups::group_id.gt(value)),
        FileGroupCondition::GroupIdLessThan(value) => Box::new(file_groups::group_id.lt(value)),

        FileGroupCondition::And(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<file_groups::table, Sqlite, SqlType = diesel::sql_types::Bool>>> = None;
            for cond in conditions {
                let expr = build_file_group_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.and(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(true.into_sql::<Bool>()))
        },
        FileGroupCondition::Or(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<file_groups::table, Sqlite, SqlType = diesel::sql_types::Bool>>> = None;
            for cond in conditions {
                let expr = build_file_group_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.or(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(false.into_sql::<Bool>()))
        },
        FileGroupCondition::Not(condition) => {
            let expr = build_file_group_condition(*condition);
            Box::new(not(expr))
        }
    }
}

// 根据 FileGroupCondition 向量查询文件组关联
pub fn select_file_groups_by_conditions(
    conn: &mut SqliteConnection,
    conditions: Vec<FileGroupCondition>,
    limit: i64,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
    let mut query = file_groups::table.into_boxed::<Sqlite>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_file_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    query
        .limit(limit)
        .select((file_groups::file_id, file_groups::group_id))
        .load(conn)
}
