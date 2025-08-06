use super::models::{FileGroupDTO,FileGroupCondition};
use crate::model::schema::{file_groups};
use diesel::prelude::*;
use std::fmt::{Debug, Formatter, Result as fmtResult};

pub fn insert_file_group(
    conn: &mut SqliteConnection,
    file_group_dto: &FileGroupDTO,
) -> Result<usize, diesel::result::Error>{
    diesel::insert_into(file_groups::table)
        .values(file_group_dto)
        .execute(conn)
}

pub fn delete_file_group_by_id(
    conn: &mut SqliteConnection,
    file_group_dto: &FileGroupDTO,
) -> Result<usize, diesel::result::Error> {
    // 删除关联记录
    diesel::delete(
        file_groups::table
            .filter(file_groups::group_id.eq(file_group_dto.group_id))
            .filter(file_groups::file_id.eq(file_group_dto.file_id))
    ).execute(conn)
}

impl Debug for FileGroupDTO {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "FileGroup {{ file_id: {}, group_id: {} }}", self.file_id, self.group_id)
    }
}

use diesel::sqlite::Sqlite;

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
            result.unwrap_or_else(|| Box::new(true.into_sql::<diesel::sql_types::Bool>()))
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
            result.unwrap_or_else(|| Box::new(false.into_sql::<diesel::sql_types::Bool>()))
        },
        FileGroupCondition::Not(condition) => {
            let expr = build_file_group_condition(*condition);
            Box::new(diesel::dsl::not(expr))
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

pub fn delete_file_groups_by_conditions(
    conn: &mut SqliteConnection,
    conditions: Vec<FileGroupCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::delete(file_groups::table).into_boxed::<Sqlite>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_file_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.execute(conn)
}

