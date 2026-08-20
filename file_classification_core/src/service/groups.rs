// groups.rs
//! 分组服务模块
//!
//! 提供分组相关的业务逻辑处理，包括分组的创建、删除、查询和更新操作，
//! 并处理分组与其关联文件、标签等资源的引用计数和级联删除。

use crate::internal::file_group as file_group_dao;
use crate::internal::files as files_dao;
use crate::internal::group_tag as group_tag_dao;
use crate::internal::groups as groups_dao;
use crate::internal::tags as tags_dao;
use crate::model::models::{
  CreateGroupDTO, FileGroupCondition, FileGroupDTO, Group, GroupCondition, GroupFilter,
  GroupQueryOptions, GroupTagCondition, GroupTreeNode, PaginationResult, UpdateGroupDTO,
};
use crate::service::AppError;
use crate::service::group_relations as group_relations_service;
use crate::utils::database::AnyConnection;
use diesel::Connection;
/// 通过名称创建分组
///
/// 参数:
/// 创建分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `create_group_dto`: 包含分组信息的DTO对象
///
/// 返回值:
/// 成功时返回插入记录的ID，失败则返回相应的错误
pub fn create_group(
  conn: &mut AnyConnection,
  create_group_dto: &CreateGroupDTO,
) -> Result<i32, AppError> {
  groups_dao::insert_group(conn, create_group_dto).map_err(AppError::from)
}

/// 根据名称查找分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `name`: 要查找的分组名称
///
/// 返回值:
/// 成功时返回匹配的分组记录（如果存在），失败时返回相应的错误
pub fn find_group_by_name(conn: &mut AnyConnection, name: &str) -> Result<Option<Group>, AppError> {
  Ok(groups_dao::find_group_by_name(conn, name)?)
}

/// 删除分组（级联删除相关资源）
///
/// 该函数负责删除分组并级联删除相关资源，根据分组是否为主分组采取不同策略：
/// 1. 主分组：减少组的关联文件的关联组、组的关联标签的引用计数，删除组关联的文件、关联文件的文件组关系、组标签关系
/// 2. 非主分组：减少关联文件、关联标签的引用计数，删除文件组关系、组标签关系
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 要删除的分组ID
///
/// 返回值:
/// 成功时返回删除的记录数，失败时返回数据库错误
pub fn delete_group(conn: &mut AnyConnection, group_id: i32) -> Result<usize, AppError> {
  conn.transaction::<usize, AppError, _>(|conn| {
    let group = groups_dao::find_group_by_id(conn, group_id)?.ok_or(AppError::GroupNotFound)?;

    // 1. 清理该组涉及的所有组关系（作为父组或子组），并维护两端引用计数
    group_relations_service::delete_group_relations_by_group_id(conn, group_id)?;

    // 2. 获取经 file_groups 关联到该组的文件列表
    let files_associated_with_group = files_dao::select_files_by_group_id(conn, group_id)?;

    if group.is_primary {
      // 主组与唯一文件一一对应，此处显式平铺删除该文件及其全部关联
      if let Some(file) = files_associated_with_group.first() {
        // 2a. 文件的所有文件组关联：减少各关联组的引用计数并删除关联行
        let file_groups = file_group_dao::select_file_groups_by_conditions_with_limit(
          conn,
          vec![FileGroupCondition::FileId(file.id)],
          None,
        )?;
        for file_group in &file_groups {
          groups_dao::decrease_group_reference_count_by_id(conn, file_group.group_id)?;
        }
        file_group_dao::delete_file_groups_by_dtos(conn, file_groups)?;

        // 2b. 主组关联的标签：减少各标签引用计数并删除组标签关系
        let tag_list = tags_dao::select_tag_by_group_id(conn, group_id)?;
        if !tag_list.is_empty() {
          tags_dao::decrease_tag_reference_count_by_ids(
            conn,
            tag_list.iter().map(|tag| tag.id).collect::<Vec<_>>(),
          )?;
        }
        group_tag_dao::delete_group_tags_by_conditions(
          conn,
          vec![GroupTagCondition::GroupId(group_id)],
        )?;

        // 2c. 先删除文件，再删除主组（满足 files.group_id 外键约束）
        files_dao::delete_file_by_id(conn, file.id)?;
      } else {
        // 主组无关联文件（异常状态）：仅清理其标签后删除组
        let tag_list = tags_dao::select_tag_by_group_id(conn, group_id)?;
        if !tag_list.is_empty() {
          tags_dao::decrease_tag_reference_count_by_ids(
            conn,
            tag_list.iter().map(|tag| tag.id).collect::<Vec<_>>(),
          )?;
        }
        group_tag_dao::delete_group_tags_by_conditions(
          conn,
          vec![GroupTagCondition::GroupId(group_id)],
        )?;
      }

      groups_dao::delete_group_by_id(conn, group_id)?;
    } else {
      // 非主组：解除文件关联
      for file in &files_associated_with_group {
        // 2a. 减少关联文件的引用计数并删除文件组关系
        files_dao::decrease_file_reference_count_by_id(conn, file.id)?;
        file_group_dao::delete_file_group_by_dto(
          conn,
          &FileGroupDTO { file_id: file.id, group_id, relation_type: 1 },
        )?;
      }

      // 2b. 减少组关联标签的引用计数并删除组标签关系
      let group_tags = group_tag_dao::select_group_tags_by_conditions_with_limit(
        conn,
        vec![GroupTagCondition::GroupId(group_id)],
        None,
      )?;
      for group_tag in &group_tags {
        tags_dao::decrease_tag_reference_count_by_id(conn, group_tag.tag_id)?;
        group_tag_dao::delete_group_tag_by_dto(conn, group_tag)?;
      }

      // 2c. 删除组本身
      groups_dao::delete_group_by_id(conn, group_id)?;
    }

    Ok(1)
  })
}

/// 根据过滤条件查询分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 分组过滤条件
/// - `limit`: 最大返回记录数（可选）
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_groups_by_filter_with_limit(
  conn: &mut AnyConnection,
  search_input: GroupFilter,
  limit: Option<i64>,
) -> Result<Vec<Group>, AppError> {
  groups_dao::select_groups_by_filter_with_limit(conn, search_input, limit).map_err(AppError::from)
}

/// 将分组过滤条件转换为查询条件向量
fn group_filter_to_conditions(search_input: GroupFilter) -> Vec<GroupCondition> {
  let mut conditions = Vec::new();

  if let Some(id) = search_input.id {
    conditions.push(GroupCondition::Id(id));
  }
  if let Some(name) = search_input.name {
    conditions.push(GroupCondition::NameLike(name));
  }
  if let Some(reference_count) = search_input.reference_count {
    conditions.push(GroupCondition::ReferenceCount(reference_count));
  }
  if let Some(is_primary) = search_input.is_primary {
    conditions.push(GroupCondition::IsPrimary(is_primary));
  }
  if let Some(click_count) = search_input.click_count {
    conditions.push(GroupCondition::ClickCount(click_count));
  }
  if let Some(share_count) = search_input.share_count {
    conditions.push(GroupCondition::ShareCount(share_count));
  }
  if let Some(create_time) = search_input.create_time {
    conditions.push(GroupCondition::CreateTime(create_time));
  }
  if let Some(modify_time) = search_input.modify_time {
    conditions.push(GroupCondition::ModifyTime(modify_time));
  }
  if let Some(description) = search_input.description {
    conditions.push(GroupCondition::DescriptionLike(description));
  }

  conditions
}

/// 根据过滤条件和选项查询分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 分组过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_groups_by_filter_with_options(
  conn: &mut AnyConnection,
  search_input: GroupFilter,
  options: GroupQueryOptions,
) -> Result<Vec<Group>, AppError> {
  let conditions = group_filter_to_conditions(search_input);

  groups_dao::select_groups_by_conditions_with_options(conn, conditions, options)
    .map_err(AppError::from)
}

/// 根据过滤条件和选项查询分组列表（支持分页结果）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 分组过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
pub fn select_groups_by_filter_with_pagination(
  conn: &mut AnyConnection,
  search_input: GroupFilter,
  options: GroupQueryOptions,
) -> Result<PaginationResult<Group>, AppError> {
  let conditions = group_filter_to_conditions(search_input);

  select_groups_by_conditions_with_pagination(conn, conditions, options)
}

/// 根据条件查询分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 查询条件向量
/// - `limit`: 返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_groups_by_conditions_with_limit(
  conn: &mut AnyConnection,
  condition: Vec<GroupCondition>,
  limit: Option<i64>,
) -> Result<Vec<Group>, AppError> {
  groups_dao::select_groups_by_conditions_with_limit(conn, condition, limit)
    .map_err(AppError::from)
}

/// 根据条件和选项查询分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_groups_by_conditions_with_options(
  conn: &mut AnyConnection,
  conditions: Vec<GroupCondition>,
  options: GroupQueryOptions,
) -> Result<Vec<Group>, AppError> {
  groups_dao::select_groups_by_conditions_with_options(conn, conditions, options)
    .map_err(AppError::from)
}

/// 根据条件和选项查询分组列表（支持分页结果）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
pub fn select_groups_by_conditions_with_pagination(
  conn: &mut AnyConnection,
  conditions: Vec<GroupCondition>,
  options: GroupQueryOptions,
) -> Result<PaginationResult<Group>, AppError> {
  groups_dao::select_groups_by_conditions_with_pagination(conn, conditions, options)
    .map_err(AppError::from)
}

/// 根据条件批量更新分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 更新条件向量
/// - `update_set`: 更新内容DTO
///
/// 返回值:
/// 成功更新的记录数或数据库错误
pub fn update_groups_by_conditions(
  conn: &mut AnyConnection,
  conditions: Vec<GroupCondition>,
  update_set: UpdateGroupDTO,
) -> Result<usize, AppError> {
  // 主组标记与引用计数由业务层维护，不允许通过更新接口直接修改
  if update_set.is_primary.is_some() {
    return Err(AppError::CannotModifyPrimaryStatus);
  }
  if update_set.reference_count.is_some() {
    return Err(AppError::CannotModifyReferenceCount);
  }
  groups_dao::update_groups_by_conditions(conn, conditions, update_set).map_err(AppError::from)
}

/// 根据条件批量删除分组（级联删除相关资源）
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
/// 对于每个要删除的分组，执行以下操作：
/// 1. 如果是主分组，则删除关联的文件
/// 2. 如果是非主分组，则减少关联文件的引用计数
/// 3. 删除文件组关系
/// 4. 减少关联标签的引用计数
/// 5. 删除组标签关联关系
/// 6. 删除分组本身
pub fn delete_groups_by_conditions(
  conn: &mut AnyConnection,
  conditions: Vec<GroupCondition>,
) -> Result<usize, AppError> {
  // 首先查询将要删除的组
  let groups_to_delete = select_groups_by_conditions_with_limit(conn, conditions.clone(), None)?;

  // 使用事务确保数据一致性
  conn.transaction::<_, AppError, _>(|conn| {
    let mut total_deleted = 0;

    // 对于每个要删除的组，直接调用delete_group函数
    for group in &groups_to_delete {
      let deleted_count = delete_group(conn, group.id)?;
      total_deleted += deleted_count;
    }

    Ok(total_deleted)
  })
}

/// 根据文件ID查询关联的分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `other_file_id`: 文件ID
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_group_by_file_id(
  conn: &mut AnyConnection,
  other_file_id: i32,
) -> Result<Vec<Group>, AppError> {
  groups_dao::select_groups_by_file_id(conn, other_file_id).map_err(AppError::from)
}

/// 根据标签ID查询关联的分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_id`: 标签ID
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_group_by_tag_id(
  conn: &mut AnyConnection,
  tag_id: i32,
) -> Result<Vec<Group>, AppError> {
  groups_dao::select_groups_by_tag_id(conn, tag_id).map_err(AppError::from)
}

/// 根据分组ID获取分组详情
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 分组ID
///
/// 返回值:
/// 查询成功的分组记录或数据库错误
pub fn get_group_by_id(conn: &mut AnyConnection, group_id: i32) -> Result<Group, AppError> {
  groups_dao::get_group_by_id(conn, group_id).map_err(AppError::from)
}

/// 根据分组ID更新分组信息
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 分组ID
/// - `update_set`: 更新内容DTO
///
/// 返回值:
/// 成功时返回影响的行数，失败时返回数据库错误
pub fn update_group_by_id(
  conn: &mut AnyConnection,
  group_id: i32,
  update_set: UpdateGroupDTO,
) -> Result<usize, AppError> {
  // 主组标记与引用计数由业务层维护，不允许通过更新接口直接修改
  if update_set.is_primary.is_some() {
    return Err(AppError::CannotModifyPrimaryStatus);
  }
  if update_set.reference_count.is_some() {
    return Err(AppError::CannotModifyReferenceCount);
  }
  groups_dao::update_group_by_id(conn, group_id, update_set).map_err(AppError::from)
}

/// 根据组ID获取组的树状结构
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 根节点组ID
///
/// 返回值:
/// 成功时返回组的树状结构，失败时返回错误
pub fn get_group_tree(conn: &mut AnyConnection, group_id: i32) -> Result<GroupTreeNode, AppError> {
  // 首先获取根节点组信息
  let group = groups_dao::find_group_by_id(conn, group_id)?.ok_or(AppError::GroupNotFound)?;

  // 获取直接子节点的ID列表
  let child_ids = group_relations_service::get_direct_children_ids(conn, group_id)?;

  let mut children = Vec::new();
  for child_id in child_ids {
    // 递归获取每个子节点的树状结构
    let child_tree = get_group_tree(conn, child_id)?;
    children.push(child_tree);
  }

  Ok(GroupTreeNode { group, children })
}

/// 根据分组ID列表批量删除分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_ids`: 要删除的分组ID列表
///
/// 返回值:
/// 成功删除的记录数或数据库错误
pub fn delete_groups_by_ids(conn: &mut AnyConnection, group_ids: Vec<i32>) -> Result<usize, AppError> {
  let mut total_deleted = 0;

  // 使用事务确保数据一致性
  conn.transaction::<_, AppError, _>(|conn| {
    for &group_id in &group_ids {
      // 调用单个分组删除函数，复用其业务逻辑
      let deleted_count = delete_group(conn, group_id)?;
      total_deleted += deleted_count;
    }
    Ok(total_deleted)
  })
}
