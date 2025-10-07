// group_tag.rs
//! 分组-标签关联服务模块
//!
//! 提供分组与标签之间关联关系的业务逻辑处理，包括创建、删除和查询分组-标签关联，
//! 并处理相关的引用计数管理。

use crate::internal::groups::{decrease_group_reference_count, find_group_by_id, increase_group_reference_count};
use crate::internal::tags::{decrease_tag_reference_count, find_tag_by_id, increase_tag_reference_count};
use crate::model::models::{GroupTagCondition, GroupTagDTO, GroupTagQueryOptions};
use crate::service::AppError;
use diesel::result::Error;
use diesel::Connection;
use crate::utils::database::AnyConnection;

/// 创建分组-标签关联关系
///
/// 该函数负责创建分组和标签之间的关联关系，并处理相关的引用计数。
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_tag_dto`: 包含分组ID和标签ID的关联信息
///
/// 返回值:
/// 成功时返回创建的关联关系DTO，失败时返回相应的错误
///
/// 操作流程:
/// 1. 验证分组和标签是否存在
/// 2. 在事务中执行以下操作：
///    - 增加分组的引用计数
///    - 增加标签的引用计数
///    - 插入分组-标签关联记录
pub fn create_group_tag(
    conn: &mut AnyConnection,
    group_tag_dto: GroupTagDTO,
) -> Result<GroupTagDTO, AppError> {
    let _group = find_group_by_id(conn, group_tag_dto.group_id)?
        .ok_or(AppError::GroupNotFound)?;

    let _tag = find_tag_by_id(conn, group_tag_dto.tag_id)?
        .ok_or(AppError::TagNotFound)?;

    let _result = conn.transaction::<_, AppError, _>(|conn| {
        // 业务逻辑：增加引用计数
        increase_group_reference_count(conn, group_tag_dto.group_id)?;
        increase_tag_reference_count(conn, group_tag_dto.tag_id)?;

        // 调用数据访问层执行插入操作
        crate::internal::group_tag::insert_group_tag(conn, &group_tag_dto)?;
        Ok(())
    })?;

    Ok(group_tag_dto)
}

/// 根据ID删除分组-标签关联关系
///
/// 该函数负责删除指定的分组-标签关联关系，并处理相关的引用计数。
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_tag_dto`: 包含分组ID和标签ID的关联信息
///
/// 返回值:
/// 成功时返回删除的记录数，失败时返回相应的错误
///
/// 操作流程:
/// 1. 验证分组和标签是否存在
/// 2. 在事务中执行以下操作：
///    - 减少分组的引用计数
///    - 减少标签的引用计数
///    - 删除分组-标签关联记录
pub fn delete_group_tag_by_id(
    conn: &mut AnyConnection,
    group_tag_dto: GroupTagDTO,
) -> Result<usize, AppError> {
    let _group = find_group_by_id(conn, group_tag_dto.group_id)?
        .ok_or(AppError::GroupNotFound)?;

    let _tag = find_tag_by_id(conn, group_tag_dto.tag_id)?
        .ok_or(AppError::TagNotFound)?;

    let result = conn.transaction::<_, AppError, _>(|conn| {
        // 业务逻辑：减少引用计数
        decrease_group_reference_count(conn, group_tag_dto.group_id)?;
        decrease_tag_reference_count(conn, group_tag_dto.tag_id)?;

        // 调用数据访问层执行删除操作
        let deleted_count = crate::internal::group_tag::delete_group_tag_by_id(conn, &group_tag_dto)?;

        Ok(deleted_count)
    })?;

    Ok(result)
}

/// 根据条件查询分组-标签关联记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 查询条件向量
/// - `limit`: 返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_tags_by_conditions(
    conn: &mut AnyConnection,
    condition: Vec<GroupTagCondition>,
    limit: Option<i64>,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
    crate::internal::group_tag::select_group_tags_by_conditions(conn, condition, limit)
}

/// 根据条件和选项查询分组-标签关联记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_tags_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<GroupTagCondition>,
    options: GroupTagQueryOptions,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
    crate::internal::group_tag::select_group_tags_by_conditions_with_options(conn, conditions, options)
}

/// 根据条件批量删除分组-标签关联记录
///
/// 注意：这个方法在core里不应该有直接用法
/// 要暴露给用户使用的话应当改为先select再delete_by_id，防止引用计算问题
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 删除条件向量
///
/// 返回值:
/// 成功删除的记录数或数据库错误
///
/// 操作流程:
/// 1. 先查询将要删除的所有记录
/// 2. 在事务中执行以下操作：
///    - 对每条记录减少对应分组和标签的引用计数
///    - 执行实际的批量删除操作
pub fn delete_group_tags_by_conditions(
    conn: &mut AnyConnection,
    condition: Vec<GroupTagCondition>,
) -> Result<usize, Error> {
    // 首先查询将要删除的组标签关联
    let group_tags_to_delete = select_group_tags_by_conditions(conn, condition.clone(), None)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => diesel::result::Error::NotFound,
            _ => e,
        })?;

    // 使用事务确保数据一致性
    conn.transaction::<_, Error, _>(|conn| {
        // 对于每个要删除的组标签关联，减少对应的组和标签的引用计数
        for group_tag in &group_tags_to_delete {
            // 减少组的引用计数
            decrease_group_reference_count(conn, group_tag.group_id)?;

            // 减少标签的引用计数
            decrease_tag_reference_count(conn, group_tag.tag_id)?;
        }

        // 执行实际的删除操作
        let deleted_count = crate::internal::group_tag::delete_group_tags_by_conditions(conn, condition)?;

        Ok(deleted_count)
    })
}
