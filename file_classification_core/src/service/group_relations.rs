// group_relations.rs
//! 组关系服务模块
//!
//! 提供组与组之间关系的业务逻辑处理，包括创建、删除和查询组关系，
//! 并处理相关的业务规则验证和循环引用检测。

use crate::internal::group_relations as group_relations_dao;
use crate::internal::groups as groups_dao;
use crate::model::models::{
  GroupRelation, GroupRelationCondition, GroupRelationFilter, GroupRelationQueryOptions,
  PaginationResult, RELATION_TYPE_PARENT_CHILD,
};
use crate::service::AppError;
use crate::utils::database::AnyConnection;
use diesel::Connection;
use diesel::result::Error;

/// 创建组关系
///
/// 该函数负责创建组与组之间的关系，并处理相关的业务规则验证。
/// 业务规则：
/// 1. 两个组都必须存在
/// 2. 不能创建循环引用
/// 3. 主组不能作为父组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_relation`: 包含组关系信息的对象
///
/// 返回值:
/// 成功时返回创建的组关系，失败时返回相应的错误
pub fn create_group_relation(
  conn: &mut AnyConnection,
  group_relation: GroupRelation,
) -> Result<GroupRelation, AppError> {
  // 业务规则验证
  let first_group = groups_dao::find_group_by_id(conn, group_relation.first_group_id)?
    .ok_or(AppError::GroupNotFound)?;

  let second_group = groups_dao::find_group_by_id(conn, group_relation.second_group_id)?
    .ok_or(AppError::GroupNotFound)?;

  // 检查是否尝试创建循环引用
  if would_create_cycle(
    conn,
    group_relation.first_group_id,
    group_relation.second_group_id,
    group_relation.relation_type,
  )? {
    return Err(AppError::GroupRelationCycleDetected);
  }

  // 检查第一个组是否为主组（主组不能作为父组）
  if first_group.is_primary && group_relation.relation_type == RELATION_TYPE_PARENT_CHILD {
    return Err(AppError::PrimaryGroupCannotBeParent);
  }

  // 检查关系是否已存在
  if group_relations_dao::check_group_relation_exists(
    conn,
    group_relation.first_group_id,
    group_relation.second_group_id,
    group_relation.relation_type,
  )? {
    return Err(AppError::GroupRelationAlreadyExists);
  }

  // 使用事务处理数据插入
  let result = conn.transaction::<GroupRelation, AppError, _>(|conn| {
    group_relations_dao::insert_group_relation(conn, &group_relation)?;

    // 增加两个组的引用计数
    groups_dao::increase_group_reference_count_by_ids(
      conn,
      vec![group_relation.first_group_id, group_relation.second_group_id],
    )?;

    // 构造返回对象
    let created_relation = GroupRelation {
      first_group_id: group_relation.first_group_id,
      second_group_id: group_relation.second_group_id,
      relation_type: group_relation.relation_type,
    };

    if group_relation.relation_type == RELATION_TYPE_PARENT_CHILD {
      // 设置子组的父组
      groups_dao::update_group_parent_id(
        conn,
        group_relation.second_group_id,
        group_relation.first_group_id,
      )?;
      groups_dao::increase_group_reference_count_by_id(conn, group_relation.second_group_id)?;
    }

    Ok(created_relation)
  })?;

  Ok(result)
}

/// 删除组关系
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_relation`: 包含组关系信息的对象
///
/// 返回值:
/// 成功时返回删除的记录数，失败时返回相应的错误
pub fn delete_group_relation(
  conn: &mut AnyConnection,
  group_relation: &GroupRelation,
) -> Result<usize, AppError> {
  let result = conn.transaction::<_, AppError, _>(|conn| {
    // 调用数据访问层执行删除操作
    let deleted_count = group_relations_dao::delete_group_relation_by_dto(conn, &group_relation)?;

    // 如果删除成功，减少两个组的引用计数
    if deleted_count > 0 {
      groups_dao::decrease_group_reference_count_by_ids(
        conn,
        vec![group_relation.first_group_id, group_relation.second_group_id],
      )?;
    }

    Ok(deleted_count)
  });

  result
}

/// 根据过滤条件查询组关系列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 组关系过滤条件
/// - `limit`: 最大返回记录数（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_relations_by_filter_with_limit(
  conn: &mut AnyConnection,
  search_input: GroupRelationFilter,
  limit: Option<i64>,
) -> Result<Vec<GroupRelation>, diesel::result::Error> {
  group_relations_dao::select_group_relations_by_filter_with_limit(conn, search_input, limit)
}

/// 根据过滤条件和选项查询组关系列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 组关系过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_relations_by_filter_with_options(
  conn: &mut AnyConnection,
  search_input: GroupRelationFilter,
  options: GroupRelationQueryOptions,
) -> Result<Vec<GroupRelation>, diesel::result::Error> {
  group_relations_dao::select_group_relations_by_filter_with_options(conn, search_input, options)
}

/// 根据条件查询组关系记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 查询条件向量
/// - `limit`: 返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_relations_by_conditions_with_limit(
  conn: &mut AnyConnection,
  condition: Vec<GroupRelationCondition>,
  limit: Option<i64>,
) -> Result<Vec<GroupRelation>, diesel::result::Error> {
  group_relations_dao::select_group_relations_by_conditions_with_limit(conn, condition, limit)
}

/// 根据条件和选项查询组关系记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_relations_by_conditions_with_options(
  conn: &mut AnyConnection,
  conditions: Vec<GroupRelationCondition>,
  options: GroupRelationQueryOptions,
) -> Result<Vec<GroupRelation>, diesel::result::Error> {
  group_relations_dao::select_group_relations_by_conditions_with_options(conn, conditions, options)
}

/// 根据过滤条件和选项查询组关系记录（支持分页结果）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 组关系过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
pub fn select_group_relations_by_filter_with_pagination(
  conn: &mut AnyConnection,
  search_input: GroupRelationFilter,
  options: GroupRelationQueryOptions,
) -> Result<PaginationResult<GroupRelation>, diesel::result::Error> {
  // 构造查询条件
  let mut conditions = Vec::new();

  if let Some(first_group_id) = search_input.first_group_id {
    conditions.push(GroupRelationCondition::FirstGroupId(first_group_id));
  }
  if let Some(second_group_id) = search_input.second_group_id {
    conditions.push(GroupRelationCondition::SecondGroupId(second_group_id));
  }
  if let Some(relation_type) = search_input.relation_type {
    conditions.push(GroupRelationCondition::RelationType(relation_type));
  }

  select_group_relations_by_conditions_with_pagination(conn, conditions, options)
}

/// 根据条件和选项查询组关系记录（支持分页结果）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
pub fn select_group_relations_by_conditions_with_pagination(
  conn: &mut AnyConnection,
  conditions: Vec<GroupRelationCondition>,
  options: GroupRelationQueryOptions,
) -> Result<PaginationResult<GroupRelation>, diesel::result::Error> {
  group_relations_dao::select_group_relations_by_conditions_with_pagination(
    conn, conditions, options,
  )
}

/// 根据条件批量删除组关系记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 删除条件向量
///
/// 返回值:
/// 成功删除的记录数或数据库错误
pub fn delete_group_relations_by_conditions(
  conn: &mut AnyConnection,
  condition: Vec<GroupRelationCondition>,
) -> Result<usize, Error> {
  // 首先查询将要删除的记录
  let relations_to_delete =
    select_group_relations_by_conditions_with_limit(conn, condition.clone(), None).map_err(
      |e| match e {
        diesel::result::Error::NotFound => Error::NotFound,
        _ => e,
      },
    )?;

  // 使用事务确保数据一致性
  conn.transaction::<_, Error, _>(|conn| {
    let mut total_deleted = 0;

    // 对于每个要删除的组关系，直接调用delete_group_relation函数
    for relation in &relations_to_delete {
      let relation_dto = GroupRelation {
        first_group_id: relation.first_group_id,
        second_group_id: relation.second_group_id,
        relation_type: relation.relation_type,
      };

      // 调用单个删除函数，复用其业务逻辑和验证规则
      let deleted_count = delete_group_relation(conn, &relation_dto)?;
      total_deleted += deleted_count;
    }

    Ok(total_deleted)
  })
}

/// 根据DTO列表批量删除组关系
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `dtos`: 要删除的组关系DTO列表
///
/// 返回值:
/// 成功删除的记录数或数据库错误
pub fn delete_group_relations_by_dtos(
  conn: &mut AnyConnection,
  dtos: Vec<GroupRelation>,
) -> Result<usize, Error> {
  let mut total_deleted = 0;

  // 使用事务确保数据一致性
  conn.transaction::<_, Error, _>(|conn| {
    for dto in &dtos {
      // 调用单个删除函数，复用其业务逻辑和验证规则
      let deleted_count = delete_group_relation(conn, dto)?;
      total_deleted += deleted_count;
    }
    Ok(total_deleted)
  })
}

/// 获取指定组的直接子组ID列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 组ID
///
/// 返回值:
/// 成功时返回直接子组ID列表，失败时返回错误
pub fn get_direct_children_ids(
  conn: &mut AnyConnection,
  group_id: i32,
) -> Result<Vec<i32>, diesel::result::Error> {
  group_relations_dao::get_direct_children_ids(conn, group_id)
}

/// 获取指定组的直接父组ID列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 组ID
///
/// 返回值:
/// 成功时返回直接父组ID列表，失败时返回错误
pub fn get_direct_parents_ids(
  conn: &mut AnyConnection,
  group_id: i32,
) -> Result<Vec<i32>, diesel::result::Error> {
  group_relations_dao::get_direct_parents_ids(conn, group_id)
}

/// 检查是否会创建循环引用
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `parent_id`: 父组ID
/// - `child_id`: 子组ID
/// - `relation_type`: 关系类型
///
/// 返回值:
/// 成功时返回布尔值，true表示会创建循环引用，false表示不会；失败时返回数据库错误
fn would_create_cycle(
  conn: &mut AnyConnection,
  parent_id: i32,
  child_id: i32,
  relation_type: i32,
) -> Result<bool, diesel::result::Error> {
  // 如果尝试将组设置为自己的子组，则会创建循环
  if parent_id == child_id {
    return Ok(true);
  }

  // 检查child是否已经是parent的祖先
  let mut ancestors = vec![child_id];
  let mut current_groups = vec![child_id];

  while !current_groups.is_empty() {
    let mut next_groups = Vec::new();

    for &group_id in &current_groups {
      // 获取group_id的所有父组ID
      let parent_ids = group_relations_dao::get_first_group_ids(conn, group_id, relation_type)?;

      for parent_id_value in parent_ids {
        // 如果发现parent_id在祖先中，则会形成循环
        if parent_id_value == parent_id {
          return Ok(true);
        }

        // 如果这个祖先还没有被处理过，加入到待处理列表
        if !ancestors.contains(&parent_id_value) {
          ancestors.push(parent_id_value);
          next_groups.push(parent_id_value);
        }
      }
    }

    current_groups = next_groups;
  }

  Ok(false)
}
