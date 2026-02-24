// group_tag.rs
//! 组与标签关联服务模块
//!
//! 提供组与标签之间关联关系的业务逻辑处理，包括创建、删除和查询组-标签关联，
//! 并处理相关的引用计数管理和业务规则验证。

use crate::internal::group_tag as group_tag_dao;
use crate::internal::groups as groups_dao;
use crate::internal::tags as tags_dao;
use crate::model::models::{
  GroupTagCondition, GroupTagDTO, GroupTagFilter, GroupTagQueryOptions, PaginationResult,
};
use crate::service::AppError;
use crate::utils::database::AnyConnection;
use diesel::Connection;
use diesel::result::Error;

/// 创建组-标签关联关系
///
/// 该函数负责创建组和标签之间的关联关系，并处理相关的引用计数。
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_tag_dto`: 包含组ID和标签ID的关联信息
///
/// 返回值:
/// 成功时返回创建的关联关系DTO，失败时返回相应的错误
///
/// 操作流程:
/// 1. 验证组和标签是否存在
/// 2. 在事务中执行以下操作：
///    - 增加组的引用计数
///    - 增加标签的引用计数
///    - 插入组-标签关联记录
pub fn create_group_tag(
  conn: &mut AnyConnection,
  group_tag_dto: GroupTagDTO,
) -> Result<GroupTagDTO, AppError> {
  // 验证组是否存在
  let _group =
    groups_dao::find_group_by_id(conn, group_tag_dto.group_id)?.ok_or(AppError::GroupNotFound)?;

  // 验证标签是否存在
  let _tag = tags_dao::find_tag_by_id(conn, group_tag_dto.tag_id)?.ok_or(AppError::TagNotFound)?;

  let _result = conn.transaction::<_, AppError, _>(|conn| {
    // 业务逻辑：增加引用计数
    groups_dao::increase_group_reference_count_by_id(conn, group_tag_dto.group_id)?;
    tags_dao::increase_tag_reference_count_by_id(conn, group_tag_dto.tag_id)?;

    // 调用数据访问层执行插入操作
    group_tag_dao::insert_group_tag(conn, &group_tag_dto)?;
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
/// 1. 验证组和标签是否存在
/// 2. 在事务中执行以下操作：
///    - 减少组的引用计数
///    - 减少标签的引用计数
///    - 删除组-标签关联记录
pub fn delete_group_tag_by_dto(
  conn: &mut AnyConnection,
  group_tag_dto: &GroupTagDTO,
) -> Result<usize, AppError> {
  let _group =
    groups_dao::find_group_by_id(conn, group_tag_dto.group_id)?.ok_or(AppError::GroupNotFound)?;

  let _tag = tags_dao::find_tag_by_id(conn, group_tag_dto.tag_id)?.ok_or(AppError::TagNotFound)?;

  let _result = conn.transaction::<_, AppError, _>(|conn| {
    // 业务逻辑：减少引用计数
    groups_dao::decrease_group_reference_count_by_id(conn, group_tag_dto.group_id)?;
    tags_dao::decrease_tag_reference_count_by_id(conn, group_tag_dto.tag_id)?;

    // 调用数据访问层执行删除操作
    let deleted_count = group_tag_dao::delete_group_tag_by_dto(conn, &group_tag_dto)?;

    Ok(deleted_count)
  })?;

  Ok(_result)
}

/// 根据过滤条件查询组-标签关联列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 组-标签关联过滤条件
/// - `limit`: 最大返回记录数（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_tags_by_filter_with_limit(
  conn: &mut AnyConnection,
  search_input: GroupTagFilter,
  limit: Option<i64>,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
  group_tag_dao::select_group_tags_by_filter_with_limit(conn, search_input, limit)
}

/// 根据过滤条件和选项查询分组-标签关联列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 组-标签关联过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_tags_by_filter_with_options(
  conn: &mut AnyConnection,
  search_input: GroupTagFilter,
  options: GroupTagQueryOptions,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
  group_tag_dao::select_group_tags_by_filter_with_options(conn, search_input, options)
}

/// 根据过滤条件和选项查询分组-标签关联列表（支持分页结果）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 组-标签关联过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
pub fn select_group_tags_by_filter_with_pagination(
  conn: &mut AnyConnection,
  search_input: GroupTagFilter,
  options: GroupTagQueryOptions,
) -> Result<PaginationResult<GroupTagDTO>, diesel::result::Error> {
  // 构造查询条件
  let mut conditions = Vec::new();

  if let Some(group_id) = search_input.group_id {
    conditions.push(GroupTagCondition::GroupId(group_id));
  }
  if let Some(tag_id) = search_input.tag_id {
    conditions.push(GroupTagCondition::TagId(tag_id));
  }

  select_group_tags_by_conditions_with_pagination(conn, conditions, options)
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
pub fn select_group_tags_by_conditions_with_limit(
  conn: &mut AnyConnection,
  condition: Vec<GroupTagCondition>,
  limit: Option<i64>,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
  group_tag_dao::select_group_tags_by_conditions_with_limit(conn, condition, limit)
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
  group_tag_dao::select_group_tags_by_conditions_with_options(conn, conditions, options)
}

/// 根据条件和选项查询分组-标签关联记录（支持分页结果）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
pub fn select_group_tags_by_conditions_with_pagination(
  conn: &mut AnyConnection,
  conditions: Vec<GroupTagCondition>,
  options: GroupTagQueryOptions,
) -> Result<PaginationResult<GroupTagDTO>, diesel::result::Error> {
  group_tag_dao::select_group_tags_by_conditions_with_pagination(conn, conditions, options)
}

/// 根据条件批量删除分组-标签关联记录（级联删除相关资源）
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
/// 2. 在事务中对每条记录调用delete_group_tag_by_id执行删除操作
pub fn delete_group_tags_by_conditions(
  conn: &mut AnyConnection,
  condition: Vec<GroupTagCondition>,
) -> Result<usize, Error> {
  // 首先查询将要删除的组标签关联
  let group_tags_to_delete =
    group_tag_dao::select_group_tags_by_conditions_with_limit(conn, condition.clone(), None)
      .map_err(|e| match e {
        diesel::result::Error::NotFound => Error::NotFound,
        _ => e,
      })?;

  // 使用事务确保数据一致性
  conn.transaction::<_, Error, _>(|conn| {
    let mut total_deleted = 0;

    // 对于每个要删除的组标签关联，直接调用delete_group_tag_by_dto函数
    for group_tag in &group_tags_to_delete {
      let group_tag_dto = GroupTagDTO { group_id: group_tag.group_id, tag_id: group_tag.tag_id };

      // 调用单个删除函数，复用其业务逻辑和验证规则
      let deleted_count = delete_group_tag_by_dto(conn, &group_tag_dto)?;
      total_deleted += deleted_count;
    }

    Ok(total_deleted)
  })
}

/// 根据DTO列表批量删除组标签关联
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `dtos`: 要删除的组标签关联DTO列表
///
/// 返回值:
/// 成功删除的记录数或数据库错误
pub fn delete_group_tags_by_dtos(
  conn: &mut AnyConnection,
  dtos: Vec<GroupTagDTO>,
) -> Result<usize, Error> {
  let mut total_deleted = 0;

  // 使用事务确保数据一致性
  conn.transaction::<_, Error, _>(|conn| {
    for dto in &dtos {
      // 调用单个删除函数，复用其业务逻辑和验证规则
      let deleted_count = delete_group_tag_by_dto(conn, dto)?;
      total_deleted += deleted_count;
    }
    Ok(total_deleted)
  })
}
