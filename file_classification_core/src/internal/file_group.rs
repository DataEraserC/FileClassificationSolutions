// file_group.rs
//! 文件与分组关联管理模块
//!
//! 提供对文件-分组关系表 (`file_groups`) 的增删查操作支持。

use super::models::{FileGroupCondition, FileGroupDTO};
use crate::model::schema::{file_groups, files};
use diesel::prelude::*;
use crate::utils::database::AnyConnection;
use crate::model::models::{File, FileGroupOrderBy, FileGroupQueryOptions, OrderDirection};
use crate::utils::errors::AppError;
use crate::utils::errors::AppError::CannotUnbindPrimaryGroup;

/// 插入一个新的文件-分组关联记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_group_dto`: 包含待插入数据的 DTO 对象
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回数据库错误
pub fn insert_file_group(
    conn: &mut AnyConnection,
    file_group_dto: &FileGroupDTO,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(file_groups::table)
        .values(file_group_dto)
        .execute(conn)
}

/// 根据 DTO 中的信息删除一个文件-分组关联记录
///
/// 注意：若尝试解除文件与其主分组的关系，则会抛出 `CannotUnbindPrimaryGroup` 错误。
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_group_dto`: 包含要删除记录信息的 DTO 对象
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回自定义错误或数据库错误
pub fn delete_file_group_by_id(
    conn: &mut AnyConnection,
    file_group_dto: &FileGroupDTO,
) -> Result<usize, AppError> {
    // 检查是否试图解绑文件的主分组
    let file = files::table
        .select(File::as_select())
        .filter(files::id.eq(file_group_dto.file_id))
        .first(conn);

    if file.is_ok() && file?.group_id == file_group_dto.group_id {
        return Err(CannotUnbindPrimaryGroup);
    }

    // 执行实际的删除操作
    diesel::delete(
        file_groups::table
            .filter(file_groups::group_id.eq(file_group_dto.group_id))
            .filter(file_groups::file_id.eq(file_group_dto.file_id)),
    )
    .execute(conn)
    .map_err(AppError::from) // 将 QueryResult 转换为 AppError
}

/// 构建符合 Diesel 查询语法的条件表达式
///
/// 参数:
/// - `condition`: 表达查询条件的数据结构
///
/// 返回值:
/// 符合 Diesel 查询条件类型的动态表达式盒子
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
        },
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
        },
        FileGroupCondition::Not(condition) => {
            let expr = build_file_group_condition(*condition);
            Box::new(diesel::dsl::not(expr))
        }
    }
}

/// 根据多个条件查询文件-分组关联记录，并可设置最大返回数量
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件集合，各条件之间采用 AND 连接
/// - `limit`: 最大返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_file_groups_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileGroupCondition>,
    limit: Option<i64>,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
    let mut query = file_groups::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_file_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 设置返回条目上限
    if let Some(limit) = limit {
        query = query.limit(limit)
    }

    query
        .select(FileGroupDTO::as_select())
        .load(conn)
}

/// 根据多个条件和高级选项查询文件-分组关联记录
///
/// 支持分页、排序等复杂查询需求
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件集合，各条件之间采用 AND 连接
/// - `options`: 查询选项，包括分页和排序配置
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
#[allow(dead_code)]
pub fn select_file_groups_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<FileGroupCondition>,
    options: FileGroupQueryOptions,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
    let mut query = file_groups::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_file_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 分页设置
    if let Some(limit) = options.limit {
        query = query.limit(limit);
    }

    if let Some(offset) = options.offset {
        query = query.offset(offset);
    }

    // 排序设置
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
/// 根据给定条件批量删除文件-分组关联记录
///
/// 注意：若尝试解除文件与其主分组的关系，则会抛出 `CannotUnbindPrimaryGroup` 错误。
/// 该操作在事务中执行，任何一个删除失败都会导致整个操作回滚。
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 删除条件集合，各条件之间采用 AND 连接
///
/// 返回值:
/// 成功删除的记录数目或数据库错误
pub fn delete_file_groups_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileGroupCondition>,
) -> Result<usize, AppError> {
    // 开始事务
    conn.transaction::<usize, AppError, _>(|conn| {
        // 首先查询将要删除的所有记录
        let mut select_query = file_groups::table.into_boxed::<<AnyConnection as Connection>::Backend>();

        // 应用所有条件到查询
        for condition in &conditions {
            let boxed_condition = build_file_group_condition(condition.clone());
            select_query = select_query.filter(boxed_condition);
        }

        // 获取将要删除的记录
        let records_to_delete: Vec<FileGroupDTO> = select_query
            .select(FileGroupDTO::as_select())
            .load(conn)?;

        // 检查每条记录是否是文件的主分组关联
        for record in &records_to_delete {
            let file = files::table
                .select(File::as_select())
                .filter(files::id.eq(record.file_id))
                .first(conn);

            // 如果文件存在且该分组是其主分组，则不允许删除
            if file.is_ok() && file?.group_id == record.group_id {
                return Err(CannotUnbindPrimaryGroup);
            }
        }

        // 执行实际的删除操作
        let mut delete_query = diesel::delete(file_groups::table).into_boxed::<<AnyConnection as Connection>::Backend>();

        // 应用所有删除条件
        for condition in conditions {
            let boxed_condition = build_file_group_condition(condition);
            delete_query = delete_query.filter(boxed_condition);
        }

        let deleted_count = delete_query.execute(conn)?;
        Ok(deleted_count)
    })
}
