// group_tag.rs
//! 组与标签关联管理模块
//!
//! 提供对组-标签关系表 (`group_tags`) 的增删查操作支持。

use super::models::GroupTagCondition;
use super::models::GroupTagDTO;
use crate::model::models::{
  GroupTagFilter, GroupTagOrderBy, GroupTagQueryOptions, OrderDirection, PaginationResult,
};
use crate::model::schema::group_tags;
use crate::utils::database::AnyConnection;
use diesel::prelude::*;
use diesel::sql_types::Bool;

/// 插入一个新的组-标签关联记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_tag_dto`: 包含待插入数据的 DTO 对象
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回数据库错误
pub fn insert_group_tag(
  conn: &mut AnyConnection,
  group_tag_dto: &GroupTagDTO,
) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(group_tags::table).values(group_tag_dto).execute(conn)
}

/// 根据 DTO 中的信息删除一个组-标签关联记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_tag_dto`: 包含要删除记录信息的 DTO 对象
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回数据库错误
pub fn delete_group_tag_by_dto(
  conn: &mut AnyConnection,
  group_tag_dto: &GroupTagDTO,
) -> Result<usize, diesel::result::Error> {
  diesel::delete(
    group_tags::table
      .filter(group_tags::group_id.eq(group_tag_dto.group_id))
      .filter(group_tags::tag_id.eq(group_tag_dto.tag_id)),
  )
  .execute(conn)
}

/// 根据多个 DTO 对象批量删除组-标签关联记录（带事务支持）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_tag_dtos`: 包含要删除记录信息的 DTO 对象向量
///
/// 返回值:
/// 成功时返回影响的行数，失败则返回数据库错误
pub fn delete_group_tags_by_dtos(
  conn: &mut AnyConnection,
  group_tag_dtos: Vec<GroupTagDTO>,
) -> Result<usize, diesel::result::Error> {
  conn.transaction::<usize, diesel::result::Error, _>(|conn| {
    let mut total_deleted = 0;

    for dto in group_tag_dtos {
      total_deleted += delete_group_tag_by_dto(conn, &dto)?;
    }

    Ok(total_deleted)
  })
}

/// 构建符合 Diesel 查询语法的条件表达式
///
/// 参数:
/// - `condition`: 表达查询条件的数据结构
///
/// 返回值:
/// 符合 Diesel 查询条件类型的动态表达式盒子
fn build_group_tag_condition(
  condition: GroupTagCondition,
) -> Box<
  dyn BoxableExpression<
      group_tags::table,
      <AnyConnection as Connection>::Backend,
      SqlType = diesel::sql_types::Bool,
    >,
> {
  match condition {
    // 基本相等条件
    GroupTagCondition::GroupId(id) => Box::new(group_tags::group_id.eq(id)),
    GroupTagCondition::TagId(id) => Box::new(group_tags::tag_id.eq(id)),

    // 范围比较条件
    GroupTagCondition::GroupIdGreaterThan(value) => Box::new(group_tags::group_id.gt(value)),
    GroupTagCondition::GroupIdLessThan(value) => Box::new(group_tags::group_id.lt(value)),
    GroupTagCondition::TagIdGreaterThan(value) => Box::new(group_tags::tag_id.gt(value)),
    GroupTagCondition::TagIdLessThan(value) => Box::new(group_tags::tag_id.lt(value)),

    // 集合包含条件
    GroupTagCondition::GroupIdIn(values) => Box::new(group_tags::group_id.eq_any(values)),
    GroupTagCondition::TagIdIn(values) => Box::new(group_tags::tag_id.eq_any(values)),

    // 逻辑运算条件
    GroupTagCondition::And(conditions) => {
      let mut result: Option<
        Box<
          dyn BoxableExpression<
              group_tags::table,
              <AnyConnection as Connection>::Backend,
              SqlType = diesel::sql_types::Bool,
            >,
        >,
      > = None;
      for cond in conditions {
        let expr = build_group_tag_condition(cond);
        match result {
          None => result = Some(expr),
          Some(prev) => result = Some(Box::new(prev.and(expr))),
        }
      }
      result.unwrap_or_else(|| Box::new(true.into_sql::<Bool>()))
    }
    GroupTagCondition::Or(conditions) => {
      let mut result: Option<
        Box<
          dyn BoxableExpression<
              group_tags::table,
              <AnyConnection as Connection>::Backend,
              SqlType = diesel::sql_types::Bool,
            >,
        >,
      > = None;
      for cond in conditions {
        let expr = build_group_tag_condition(cond);
        match result {
          None => result = Some(expr),
          Some(prev) => result = Some(Box::new(prev.or(expr))),
        }
      }
      result.unwrap_or_else(|| Box::new(false.into_sql::<Bool>()))
    }
    GroupTagCondition::Not(condition) => {
      let expr = build_group_tag_condition(*condition);
      Box::new(diesel::dsl::not(expr))
    }
  }
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
  // 使用 into_boxed() 来对查询进行类型擦除
  let mut base_query =
    group_tags::dsl::group_tags.into_boxed::<<AnyConnection as Connection>::Backend>();

  // 如果有限制数量，则添加限制
  if let Some(limit_value) = limit {
    base_query = base_query.limit(limit_value);
  }

  // 如果 search_input 中有各字段，则添加相应的过滤条件
  if let Some(group_id) = search_input.group_id {
    base_query = base_query.filter(group_tags::group_id.eq(group_id));
  }
  if let Some(tag_id) = search_input.tag_id {
    base_query = base_query.filter(group_tags::tag_id.eq(tag_id));
  }

  base_query.select(GroupTagDTO::as_select()).load(conn)
}

/// 根据过滤条件和选项查询组-标签关联列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 组-标签关联过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
///
/// 注意: 当前服务层统一走 conditions 查询路径，此函数预留，后续可能复用
#[allow(dead_code)]
pub fn select_group_tags_by_filter_with_options(
  conn: &mut AnyConnection,
  search_input: GroupTagFilter,
  options: GroupTagQueryOptions,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
  // 构造查询条件
  let mut conditions = Vec::new();

  if let Some(group_id) = search_input.group_id {
    conditions.push(GroupTagCondition::GroupId(group_id));
  }
  if let Some(tag_id) = search_input.tag_id {
    conditions.push(GroupTagCondition::TagId(tag_id));
  }

  select_group_tags_by_conditions_with_options(conn, conditions, options)
}

/// 根据多个条件查询组-标签关联记录，并可设置最大返回数量
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件集合，各条件之间采用 AND 连接
/// - `limit`: 最大返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_tags_by_conditions_with_limit(
  conn: &mut AnyConnection,
  conditions: Vec<GroupTagCondition>,
  limit: Option<i64>,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
  let mut query = group_tags::table.into_boxed::<<AnyConnection as Connection>::Backend>();

  // 应用所有条件
  for condition in conditions {
    let boxed_condition = build_group_tag_condition(condition);
    query = query.filter(boxed_condition);
  }

  // 设置返回条目上限
  if let Some(limit) = limit {
    query = query.limit(limit);
  }

  query.select(GroupTagDTO::as_select()).load(conn)
}

/// 根据多个条件和高级选项查询组-标签关联记录
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
pub fn select_group_tags_by_conditions_with_options(
  conn: &mut AnyConnection,
  conditions: Vec<GroupTagCondition>,
  options: GroupTagQueryOptions,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
  let mut query = group_tags::table.into_boxed::<<AnyConnection as Connection>::Backend>();

  // 应用所有条件
  for condition in conditions {
    let boxed_condition = build_group_tag_condition(condition);
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
      GroupTagOrderBy::GroupId(direction) => match direction {
        OrderDirection::Asc => query.order(group_tags::group_id.asc()),
        OrderDirection::Desc => query.order(group_tags::group_id.desc()),
      },
      GroupTagOrderBy::TagId(direction) => match direction {
        OrderDirection::Asc => query.order(group_tags::tag_id.asc()),
        OrderDirection::Desc => query.order(group_tags::tag_id.desc()),
      },
    };
  }

  query.select(GroupTagDTO::as_select()).load(conn)
}

/// 根据多个条件和高级选项查询组-标签关联记录（支持分页）
///
/// 支持分页、排序等复杂查询需求，返回分页结果
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件集合，各条件之间采用 AND 连接
/// - `options`: 查询选项，包括分页和排序配置
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
#[allow(dead_code)]
pub fn select_group_tags_by_conditions_with_pagination(
  conn: &mut AnyConnection,
  conditions: Vec<GroupTagCondition>,
  options: GroupTagQueryOptions,
) -> Result<PaginationResult<GroupTagDTO>, diesel::result::Error> {
  let mut query = group_tags::table.into_boxed::<<AnyConnection as Connection>::Backend>();
  let mut count_query = group_tags::table.into_boxed::<<AnyConnection as Connection>::Backend>();

  // 对每个条件应用 AND 逻辑
  for condition in &conditions {
    let boxed_condition = build_group_tag_condition(condition.clone());
    query = query.filter(boxed_condition);
    // 修复：为 count_query 重新构建条件而不是克隆
    let count_condition = build_group_tag_condition(condition.clone());
    count_query = count_query.filter(count_condition);
  }

  // 计算总记录数
  let total = count_query.count().get_result::<i64>(conn)?;

  // 处理分页参数
  let (limit, offset) = if let (Some(page), Some(page_size)) = (options.page, options.page_size) {
    let offset = (page - 1) * page_size;
    (page_size, offset)
  } else {
    (options.limit.unwrap_or(10), options.offset.unwrap_or(0))
  };

  // 应用查询选项（排序、限制等）
  query = query.limit(limit).offset(offset);

  // 应用排序
  for order_by in options.order_by {
    query = match order_by {
      GroupTagOrderBy::GroupId(direction) => match direction {
        OrderDirection::Asc => query.order(group_tags::group_id.asc()),
        OrderDirection::Desc => query.order(group_tags::group_id.desc()),
      },
      GroupTagOrderBy::TagId(direction) => match direction {
        OrderDirection::Asc => query.order(group_tags::tag_id.asc()),
        OrderDirection::Desc => query.order(group_tags::tag_id.desc()),
      },
    };
  }

  let data = query.select(GroupTagDTO::as_select()).load(conn)?;

  // 构造分页结果
  let page = if options.page.is_some() { options.page.unwrap() } else { offset / limit + 1 };
  let page_size = if options.page_size.is_some() { options.page_size.unwrap() } else { limit };

  Ok(PaginationResult::new(data, page, page_size, total))
}

/// 根据给定条件批量删除组-标签关联记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 删除条件集合，各条件之间采用 AND 连接
///
/// 返回值:
/// 成功删除的记录数目或数据库错误
pub fn delete_group_tags_by_conditions(
  conn: &mut AnyConnection,
  conditions: Vec<GroupTagCondition>,
) -> Result<usize, diesel::result::Error> {
  let mut query =
    diesel::delete(group_tags::table).into_boxed::<<AnyConnection as Connection>::Backend>();

  // 应用所有删除条件
  for condition in conditions {
    let boxed_condition = build_group_tag_condition(condition);
    query = query.filter(boxed_condition);
  }

  query.execute(conn)
}
