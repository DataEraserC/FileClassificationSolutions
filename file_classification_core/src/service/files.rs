// files.rs
//! 文件服务模块
//!
//! 提供文件相关的业务逻辑处理，包括文件的创建、删除、查询和更新操作，
//! 并处理文件与其关联分组、标签等资源的引用计数和级联删除。

use crate::internal::file_group as file_group_dao;
use crate::internal::files as files_dao;
use crate::internal::group_relations as group_relations_dao;
use crate::internal::group_tag as group_tag_dao;
use crate::internal::groups as groups_dao;
use crate::internal::tags as tags_dao;
use crate::model::models::{
  CreateFileDTO, File, FileCondition, FileFilter, FileGroupCondition, FileGroupDTO,
  FileQueryOptions, GroupTagCondition, PaginationResult, UpdateFileDTO,
};
use crate::service::AppError;
use crate::service::file_group as file_group_service;
use crate::service::group_relations as group_relations_service;
use crate::utils::database::AnyConnection;
use crate::utils::errors::AppError::FuturePrimaryGroupShouldBeEmpty;
use diesel::Connection;

/// 创建文件（业务逻辑处理）
///
/// 该函数负责创建文件并处理相关业务逻辑，包括：
/// 1. 验证目标分组是否存在且为空（作为主分组）
/// 2. 创建文件记录
/// 3. 建立文件与主分组的关联关系
/// 4. 将目标分组标记为主分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `create_file_dto`: 包含文件信息的DTO对象
///
/// 返回值:
/// 成功时返回插入记录的ID，失败则返回相应的错误
pub fn create_file(
  conn: &mut AnyConnection,
  create_file_dto: CreateFileDTO,
) -> Result<i32, AppError> {
  conn.transaction::<i32, AppError, _>(|conn| {
    // 验证目标分组是否存在
    let target_group = groups_dao::get_group_by_id(conn, create_file_dto.group_id)?;

    // 目标分组不能已经是别人的主分组
    if target_group.is_primary == true {
      return Err(AppError::CannotBindToPrimaryGroup);
    }

    // 检查目标分组是否为空（作为主分组必须为空）
    if file_group_dao::check_group_empty(conn, target_group.id)? == false {
      return Err(FuturePrimaryGroupShouldBeEmpty);
    }

    // 检查该组是否已经是其他组的父组
    let children = group_relations_dao::get_direct_children_ids(conn, target_group.id)?;
    if !children.is_empty() {
      // 如果该组已经是其他组的父组，则不能转为主组
      return Err(FuturePrimaryGroupShouldBeEmpty);
    }

    // 记录影响条数
    // let mut count = 0;

    // 创建文件记录
    let file_id = files_dao::insert_file(conn, &create_file_dto)?;

    // count += 1;

    // // 获取刚创建的文件 主分组id可以区别文件
    // let file_list = files_dao::select_files_by_conditions(conn, vec![
    //     FileCondition::GroupId(create_file_dto.group_id)
    // ], None)?;
    // let file = file_list.get(0).ok_or(AppError::FileNotFound)?;
    // file_id = file.id;

    // 建立文件与主分组的关联关系
    match file_group_service::create_file_group(
      conn,
      FileGroupDTO { file_id, group_id: target_group.id, relation_type: 1 },
    ) {
      Ok(_) => {
        // count += 1;
      }
      Err(e) => return Err(e),
    }

    // 将目标分组标记为主分组
    groups_dao::mark_group_as_primary(conn, target_group.id)?;
    // count += groups_dao::mark_group_as_primary(conn, target_group.id)?;

    // 返回文件id
    Ok(file_id)
  })
}

/// 删除文件（级联删除相关资源）
///
/// 该函数负责删除文件并级联删除相关资源，包括：
/// 1. 删除文件主分组关联的所有标签关系
/// 2. 删除文件关联的所有分组关系
/// 3. 删除文件的主分组
/// 4. 删除文件本身
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_id`: 要删除的文件ID
///
/// 返回值:
/// 成功时返回空元组，失败时返回相应的错误
pub fn delete_file(conn: &mut AnyConnection, file_id: i32) -> Result<(), AppError> {
  // 开始事务
  conn.transaction::<(), AppError, _>(|conn| {
    // 0. 删除对应的PrimaryGroup对应的GroupTag
    // 1. 删除对应的PrimaryGroup
    // 2. 删除对应的剩余FileGroup
    let file_required_to_delete =
      files_dao::find_file_by_id(conn, file_id)?.ok_or_else(|| diesel::result::Error::NotFound)?;

    // 查找与该文件关联的所有文件组关系（包括主组和其他组）
    let file_groups = file_group_dao::select_file_groups_by_conditions_with_limit(
      conn,
      vec![FileGroupCondition::FileId(file_required_to_delete.id)],
      None,
    )?;

    // 对于每个文件组关系，减少对应组的引用计数
    for file_group in &file_groups {
      groups_dao::decrease_group_reference_count_by_id(conn, file_group.group_id)?;
    }

    // 仅对主组关联的标签减少引用计数
    let tag_list = tags_dao::select_tag_by_group_id(conn, file_required_to_delete.group_id)?;

    if !tag_list.is_empty() {
      tags_dao::decrease_tag_reference_count_by_ids(
        conn,
        tag_list.iter().map(|tag| tag.id).collect::<Vec<_>>(),
      )?;
    }

    // 删除与文件主组关联的所有组标签关系
    group_tag_dao::delete_group_tags_by_conditions(
      conn,
      vec![GroupTagCondition::GroupId(file_required_to_delete.group_id)],
    )?;

    // 删除与文件关联的所有文件组关系
    file_group_dao::delete_file_groups_by_dtos(conn, file_groups)?;

    // 清理文件主组涉及的所有组关系（防止外键约束拦截组删除）
    group_relations_service::delete_group_relations_by_group_id(
      conn,
      file_required_to_delete.group_id,
    )?;

    // 先删除文件，再删除主组（满足 files.group_id 外键约束）
    files_dao::delete_file_by_id(conn, file_id)?;
    groups_dao::delete_group_by_id(conn, file_required_to_delete.group_id)?;

    Ok(())
  })?;
  Ok(())
}

/// 根据过滤条件查询文件列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 文件过滤条件
/// - `limit`: 最大返回记录数（可选）
///
/// 返回值:
/// 查询成功的文件记录列表或数据库错误
pub fn select_files_by_filter_with_limit(
  conn: &mut AnyConnection,
  search_input: FileFilter,
  limit: Option<i64>,
) -> Result<Vec<File>, AppError> {
  files_dao::select_files_by_filter_with_limit(conn, search_input, limit).map_err(AppError::from)
}

/// 根据过滤条件和选项查询文件列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 文件过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的文件记录列表或数据库错误
pub fn select_files_by_filter_with_options(
  conn: &mut AnyConnection,
  search_input: FileFilter,
  options: FileQueryOptions,
) -> Result<Vec<File>, AppError> {
  // 构造查询条件
  let mut conditions = Vec::new();

  if let Some(id) = search_input.id {
    conditions.push(FileCondition::Id(id));
  }
  if let Some(type_) = search_input.type_ {
    conditions.push(FileCondition::Type(type_));
  }
  if let Some(path) = search_input.path {
    conditions.push(FileCondition::Path(path));
  }
  if let Some(reference_count) = search_input.reference_count {
    conditions.push(FileCondition::ReferenceCount(reference_count));
  }
  if let Some(group_id) = search_input.group_id {
    conditions.push(FileCondition::GroupId(group_id));
  }

  files_dao::select_files_by_conditions_with_options(conn, conditions, options)
    .map_err(AppError::from)
}

/// 根据过滤条件和选项查询文件列表（支持分页结果）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 文件过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
pub fn select_files_by_filter_with_pagination(
  conn: &mut AnyConnection,
  search_input: FileFilter,
  options: FileQueryOptions,
) -> Result<PaginationResult<File>, AppError> {
  // 构造查询条件
  let mut conditions = Vec::new();

  if let Some(id) = search_input.id {
    conditions.push(FileCondition::Id(id));
  }
  if let Some(type_) = search_input.type_ {
    conditions.push(FileCondition::TypeLike(type_));
  }
  if let Some(path) = search_input.path {
    conditions.push(FileCondition::PathLike(path));
  }
  if let Some(reference_count) = search_input.reference_count {
    conditions.push(FileCondition::ReferenceCount(reference_count));
  }
  if let Some(group_id) = search_input.group_id {
    conditions.push(FileCondition::GroupId(group_id));
  }
  if let Some(description) = search_input.description {
    conditions.push(FileCondition::DescriptionLike(description));
  }

  select_files_by_conditions_with_pagination(conn, conditions, options)
}

/// 根据条件查询文件列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 查询条件向量
/// - `limit`: 返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的文件记录列表或数据库错误
pub fn select_files_by_conditions_with_limit(
  conn: &mut AnyConnection,
  condition: Vec<FileCondition>,
  limit: Option<i64>,
) -> Result<Vec<File>, AppError> {
  files_dao::select_files_by_conditions_with_limit(conn, condition, limit).map_err(AppError::from)
}

/// 根据条件和选项查询文件列表（支持分页）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的文件记录列表或数据库错误
pub fn select_files_by_conditions_with_options(
  conn: &mut AnyConnection,
  conditions: Vec<FileCondition>,
  options: FileQueryOptions,
) -> Result<Vec<File>, AppError> {
  files_dao::select_files_by_conditions_with_options(conn, conditions, options)
    .map_err(AppError::from)
}

/// 根据条件和选项查询文件列表（支持分页结果）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
pub fn select_files_by_conditions_with_pagination(
  conn: &mut AnyConnection,
  conditions: Vec<FileCondition>,
  options: FileQueryOptions,
) -> Result<PaginationResult<File>, AppError> {
  files_dao::select_files_by_conditions_with_pagination(conn, conditions, options)
    .map_err(AppError::from)
}

/// 根据条件批量更新文件
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 更新条件向量
/// - `update_set`: 更新内容DTO
///
/// 返回值:
/// 成功更新的记录数或AppError错误
pub fn update_files_by_conditions(
  conn: &mut AnyConnection,
  conditions: Vec<FileCondition>,
  update_set: UpdateFileDTO,
) -> Result<usize, AppError> {
  // 引用计数由业务层维护，不允许通过更新接口直接修改
  if update_set.reference_count.is_some() {
    return Err(AppError::CannotModifyReferenceCount);
  }

  // 首先查询将要更新的文件
  let files_to_update = select_files_by_conditions_with_limit(conn, conditions, None)?;

  // 使用事务确保数据一致性
  conn.transaction::<usize, AppError, _>(|conn| {
    let mut total_updated = 0;

    // 对于每个要更新的文件，调用单个文件更新函数
    for file in &files_to_update {
      update_file_by_id(conn, file.id, update_set.clone())?;
      total_updated += 1;
    }

    Ok(total_updated)
  })
}

/// 根据条件批量删除文件（级联删除相关资源）
///
/// 注意：这个方法在core里不应该有直接用法
/// 要暴露给用户使用的话应当改为先select再delete_by_id，防止引用计算问题
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 删除条件向量
///
/// 返回值:
/// 成功删除的记录数或数据库错误
///
/// 操作流程:
/// 对于每个要删除的文件，直接调用delete_file函数执行删除操作
pub fn delete_files_by_conditions(
  conn: &mut AnyConnection,
  conditions: Vec<FileCondition>,
) -> Result<usize, AppError> {
  // 首先查询将要删除的文件
  let files_to_delete = select_files_by_conditions_with_limit(conn, conditions.clone(), None)?;

  // 使用事务确保数据一致性
  conn.transaction::<_, AppError, _>(|conn| {
    let mut total_deleted = 0;

    // 对于每个要删除的文件，直接调用delete_file函数
    for file in &files_to_delete {
      // 调用单个文件删除函数，复用其业务逻辑
      delete_file(conn, file.id)?;
      total_deleted += 1;
    }

    Ok(total_deleted)
  })
}

/// 根据分组ID查询关联的文件列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `other_group_id`: 分组ID
///
/// 返回值:
/// 查询成功的文件记录列表或数据库错误
pub fn select_file_by_group_id(
  conn: &mut AnyConnection,
  other_group_id: i32,
) -> Result<Vec<File>, AppError> {
  files_dao::select_files_by_group_id(conn, other_group_id).map_err(AppError::from)
}

/// 根据文件ID获取文件详情
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_id`: 文件ID
///
/// 返回值:
/// 查询成功的文件记录或数据库错误
pub fn get_file_by_id(conn: &mut AnyConnection, file_id: i32) -> Result<File, AppError> {
  files_dao::get_file_by_id(conn, file_id).map_err(AppError::from)
}

/// 根据文件ID更新文件信息
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_id`: 文件ID
/// - `update_set`: 更新内容DTO
///
/// 返回值:
/// 成功时返回影响的行数，失败时返回数据库错误
pub fn update_file_by_id(
  conn: &mut AnyConnection,
  file_id: i32,
  update_set: UpdateFileDTO,
) -> Result<usize, AppError> {
  // 引用计数由业务层维护，不允许通过更新接口直接修改
  if update_set.reference_count.is_some() {
    return Err(AppError::CannotModifyReferenceCount);
  }

  conn.transaction::<usize, AppError, _>(|conn| {
    // 获取当前文件信息
    let current_file = get_file_by_id(conn, file_id)?;

    // 如果没有更新group_id，则直接更新
    if update_set.group_id.is_none() {
      return Ok(files_dao::update_file_by_id(conn, file_id, update_set)?);
    }

    // 如果更新了group_id，则需要进行类似创建文件时的检查
    let new_group_id = update_set.group_id.unwrap();

    // 如果组ID没有变化，则直接更新
    if new_group_id == current_file.group_id {
      return Ok(files_dao::update_file_by_id(conn, file_id, update_set)?);
    }

    // 验证新目标分组是否存在
    let target_group = groups_dao::get_group_by_id(conn, new_group_id)?;

    // 目标分组不能已经是别人的主分组
    if target_group.is_primary == true {
      return Err(AppError::CannotBindToPrimaryGroup);
    }

    // 检查目标分组是否为空（作为主分组必须为空）
    if file_group_dao::check_group_empty(conn, target_group.id)? == false {
      return Err(FuturePrimaryGroupShouldBeEmpty);
    }

    // 检查该组是否已经是其他组的父组
    let children = group_relations_dao::get_direct_children_ids(conn, target_group.id)?;
    if !children.is_empty() {
      // 如果该组已经是其他组的父组，则不能转为主组
      return Err(FuturePrimaryGroupShouldBeEmpty);
    }

    // 获取原主组
    let old_primary_group_id = current_file.group_id;

    // 更新文件（改变其主组外键）
    let rows_affected = files_dao::update_file_by_id(conn, file_id, update_set)?;

    // 删除旧的文件-组关联（主组关系）：文件引用计数-1，旧主组引用计数-1
    file_group_dao::delete_file_group_by_dto(
      conn,
      &FileGroupDTO { file_id, group_id: old_primary_group_id, relation_type: 1 },
    )?;
    files_dao::decrease_file_reference_count_by_id(conn, file_id)?;
    groups_dao::decrease_group_reference_count_by_id(conn, old_primary_group_id)?;

    // 创建新的文件-组关联（主组关系）：文件引用计数+1，新主组引用计数+1
    file_group_dao::insert_file_group(
      conn,
      &FileGroupDTO { file_id, group_id: target_group.id, relation_type: 1 },
    )?;
    files_dao::increase_file_reference_count_by_id(conn, file_id)?;
    groups_dao::increase_group_reference_count_by_id(conn, target_group.id)?;

    // 将新目标分组标记为主分组
    groups_dao::mark_group_as_primary(conn, target_group.id)?;

    // 将旧的主分组标记为非主分组
    groups_dao::mark_group_as_non_primary(conn, old_primary_group_id)?;

    Ok(rows_affected)
  })
}

/// 根据文件ID列表批量删除文件
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_ids`: 要删除的文件ID列表
///
/// 返回值:
/// 成功删除的记录数或数据库错误
pub fn delete_files_by_ids(
  conn: &mut AnyConnection,
  file_ids: Vec<i32>,
) -> Result<usize, AppError> {
  let mut total_deleted = 0;

  // 使用事务确保数据一致性
  conn.transaction::<_, AppError, _>(|conn| {
    for &file_id in &file_ids {
      // 调用单个文件删除函数，复用其业务逻辑
      delete_file(conn, file_id)?;
      total_deleted += 1;
    }
    Ok(total_deleted)
  })
}
