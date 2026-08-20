// file_group.rs
//! 文件-分组关联服务模块
//!
//! 提供文件与分组之间关联关系的业务逻辑处理，包括创建、删除和查询文件-分组关联，
//! 并处理相关的引用计数管理和业务规则验证。

use crate::internal::file_group as file_group_dao;
use crate::internal::files as files_dao;
use crate::internal::groups as groups_dao;
use crate::model::models::{
  FileGroupCondition, FileGroupDTO, FileGroupFilter, FileGroupQueryOptions,
};
use crate::service::AppError;
use crate::utils::database::AnyConnection;
use diesel::Connection;

/// 创建文件-分组关联关系
///
/// 该函数负责创建文件和分组之间的关联关系，并处理相关的引用计数。
/// 业务规则：不允许将文件关联到主分组。
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_group_dto`: 包含文件ID和分组ID的关联信息
///
/// 返回值:
/// 成功时返回创建的关联关系DTO，失败时返回相应的错误
///
/// 操作流程:
/// 1. 验证分组和文件是否存在
/// 2. 检查分组是否为主分组（主分组不允许通过此方法关联）
/// 3. 在事务中执行以下操作：
///    - 增加文件的引用计数
///    - 增加分组的引用计数
///    - 插入文件-分组关联记录
pub fn create_file_group(
  conn: &mut AnyConnection,
  file_group_dto: FileGroupDTO,
) -> Result<FileGroupDTO, AppError> {
  // 验证分组是否存在
  let _group =
    groups_dao::find_group_by_id(conn, file_group_dto.group_id)?.ok_or(AppError::GroupNotFound)?;

  // 验证文件是否存在
  let _file =
    files_dao::get_file_by_id(conn, file_group_dto.file_id).map_err(|_| AppError::FileNotFound)?;

  // 检查分组是否为主分组（主分组不能通过此方法关联）
  if _group.is_primary {
    return Err(AppError::CannotBindToPrimaryGroup);
  }

  // 使用事务确保数据一致性
  let result = conn.transaction::<FileGroupDTO, AppError, _>(|conn| {
    // 业务逻辑：增加文件和分组的引用计数
    files_dao::increase_file_reference_count_by_id(conn, file_group_dto.file_id)?;
    groups_dao::increase_group_reference_count_by_id(conn, file_group_dto.group_id)?;

    // 调用数据访问层执行插入操作
    file_group_dao::insert_file_group(conn, &file_group_dto)?;
    Ok(file_group_dto)
  })?;

  Ok(result)
}

/// 根据DTO信息删除文件-分组关联关系
///
/// 该函数负责删除指定的文件-分组关联关系，并处理相关的引用计数。
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_group_dto`: 包含文件ID和分组ID的关联信息
///
/// 返回值:
/// 成功时返回删除的记录数，失败时返回相应的错误
///
/// 操作流程:
/// 1. 验证分组和文件是否存在
/// 2. 检查分组是否为主分组（主分组不能通过此方法解绑）
/// 3. 在事务中执行以下操作：
///    - 减少文件的引用计数
///    - 减少分组的引用计数
///    - 删除文件-分组关联记录
pub fn delete_file_group_by_dto(
  conn: &mut AnyConnection,
  file_group_dto: &FileGroupDTO,
) -> Result<usize, AppError> {
  // 验证分组是否存在
  let _group =
    groups_dao::find_group_by_id(conn, file_group_dto.group_id)?.ok_or(AppError::GroupNotFound)?;

  // 验证文件是否存在
  let _file =
    files_dao::get_file_by_id(conn, file_group_dto.file_id).map_err(|_| AppError::FileNotFound)?;

  // 检查分组是否为主分组（主分组不能通过此方法解绑）
  if _group.is_primary {
    return Err(AppError::CannotUnbindPrimaryGroup);
  }

  // 使用事务确保数据一致性
  let result = conn.transaction::<_, AppError, _>(|conn| {
    // 调用数据访问层执行删除操作
    let deleted_count = file_group_dao::delete_file_group_by_dto(conn, file_group_dto)?;

    // 仅当实际删除成功时，减少文件和分组的引用计数
    if deleted_count > 0 {
      files_dao::decrease_file_reference_count_by_id(conn, file_group_dto.file_id)?;
      groups_dao::decrease_group_reference_count_by_id(conn, file_group_dto.group_id)?;
    }

    Ok(deleted_count)
  })?;

  Ok(result)
}

/// 将文件-分组过滤条件转换为查询条件向量
fn file_group_filter_to_conditions(search_input: FileGroupFilter) -> Vec<FileGroupCondition> {
  let mut conditions = Vec::new();

  if let Some(file_id) = search_input.file_id {
    conditions.push(FileGroupCondition::FileId(file_id));
  }
  if let Some(group_id) = search_input.group_id {
    conditions.push(FileGroupCondition::GroupId(group_id));
  }
  if let Some(relation_type) = search_input.relation_type {
    conditions.push(FileGroupCondition::RelationType(relation_type));
  }

  conditions
}

select_functions! {
  dao: file_group_dao,
  filter_with_limit: select_file_groups_by_filter_with_limit,
  filter_with_options: select_file_groups_by_filter_with_options,
  filter_with_pagination: select_file_groups_by_filter_with_pagination,
  conditions_with_limit: select_file_groups_by_conditions_with_limit,
  conditions_with_options: select_file_groups_by_conditions_with_options,
  conditions_with_pagination: select_file_groups_by_conditions_with_pagination,
  dto: FileGroupDTO,
  filter: FileGroupFilter,
  condition: FileGroupCondition,
  options: FileGroupQueryOptions,
  to_conditions: file_group_filter_to_conditions,
}

/// 根据条件批量删除文件-分组关联记录（级联删除相关资源）
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
/// 2. 在事务中对每条记录调用delete_file_group执行删除操作
pub fn delete_file_groups_by_conditions(
  conn: &mut AnyConnection,
  condition: Vec<FileGroupCondition>,
) -> Result<usize, AppError> {
  // 首先查询将要删除的记录
  let file_groups_to_delete =
    file_group_dao::select_file_groups_by_conditions_with_limit(conn, condition.clone(), None)?;

  // 使用事务确保数据一致性
  conn.transaction::<_, AppError, _>(|conn| {
    let mut total_deleted = 0;

    // 对于每个要删除的文件组关联，直接调用delete_file_group函数
    for file_group in &file_groups_to_delete {
      // 调用单个删除函数，复用其业务逻辑和验证规则，保留 DB 中的实际 relation_type
      let deleted_count = delete_file_group_by_dto(conn, file_group)?;
      total_deleted += deleted_count;
    }

    Ok(total_deleted)
  })
}

/// 根据DTO列表批量删除文件组关联
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `dtos`: 要删除的文件组关联DTO列表
///
/// 返回值:
/// 成功删除的记录数或数据库错误
pub fn delete_file_groups_by_dtos(
  conn: &mut AnyConnection,
  dtos: Vec<FileGroupDTO>,
) -> Result<usize, AppError> {
  let mut total_deleted = 0;

  // 使用事务确保数据一致性
  conn.transaction::<_, AppError, _>(|conn| {
    for dto in &dtos {
      // 调用单个删除函数，复用其业务逻辑和验证规则
      let deleted_count = delete_file_group_by_dto(conn, dto)?;
      total_deleted += deleted_count;
    }
    Ok(total_deleted)
  })
}
