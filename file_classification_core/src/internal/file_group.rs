// file_group.rs
//! 文件与分组关联管理模块
//!
//! 提供对文件-分组关系表 (`file_groups`) 的增删查操作支持。

use super::models::{FileGroupCondition, FileGroupDTO};
use crate::model::models::{
  FileGroupFilter, FileGroupOrderBy, FileGroupQueryOptions, OrderDirection, PaginationResult,
};
use crate::model::schema::file_groups;
use crate::utils::database::AnyConnection;
use diesel::prelude::*;

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
  diesel::insert_into(file_groups::table).values(file_group_dto).execute(conn)
}

/// 根据 DTO 中的信息删除一个文件-分组关联记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_group_dto`: 包含要删除记录信息的 DTO 对象
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回数据库错误
pub fn delete_file_group_by_dto(
  conn: &mut AnyConnection,
  file_group_dto: &FileGroupDTO,
) -> Result<usize, diesel::result::Error> {
  diesel::delete(
    file_groups::table
      .filter(file_groups::group_id.eq(file_group_dto.group_id))
      .filter(file_groups::file_id.eq(file_group_dto.file_id)),
  )
  .execute(conn)
}

/// 根据多个 DTO 对象批量删除文件-分组关联记录（带事务支持）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_group_dtos`: 包含要删除记录信息的 DTO 对象向量
///
/// 返回值:
/// 成功时返回影响的行数，失败则返回数据库错误
pub fn delete_file_groups_by_dtos(
  conn: &mut AnyConnection,
  file_group_dtos: Vec<FileGroupDTO>,
) -> Result<usize, diesel::result::Error> {
  conn.transaction::<usize, diesel::result::Error, _>(|conn| {
    let mut total_deleted = 0;

    for dto in file_group_dtos {
      total_deleted += delete_file_group_by_dto(conn, &dto)?;
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
fn build_file_group_condition(
  condition: FileGroupCondition,
) -> Box<
  dyn BoxableExpression<
      file_groups::table,
      <AnyConnection as Connection>::Backend,
      SqlType = diesel::sql_types::Bool,
    >,
> {
  match condition {
    // 基本相等条件
    FileGroupCondition::FileId(id) => Box::new(file_groups::file_id.eq(id)),
    FileGroupCondition::GroupId(id) => Box::new(file_groups::group_id.eq(id)),
    FileGroupCondition::RelationType(typ) => Box::new(file_groups::relation_type.eq(typ)),

    // 范围比较条件
    FileGroupCondition::FileIdGreaterThan(value) => Box::new(file_groups::file_id.gt(value)),
    FileGroupCondition::FileIdLessThan(value) => Box::new(file_groups::file_id.lt(value)),
    FileGroupCondition::GroupIdGreaterThan(value) => Box::new(file_groups::group_id.gt(value)),
    FileGroupCondition::GroupIdLessThan(value) => Box::new(file_groups::group_id.lt(value)),
    FileGroupCondition::RelationTypeGreaterThan(value) => {
      Box::new(file_groups::relation_type.gt(value))
    }
    FileGroupCondition::RelationTypeLessThan(value) => {
      Box::new(file_groups::relation_type.lt(value))
    }

    // 集合包含条件
    FileGroupCondition::FileIdIn(values) => Box::new(file_groups::file_id.eq_any(values)),
    FileGroupCondition::GroupIdIn(values) => Box::new(file_groups::group_id.eq_any(values)),
    FileGroupCondition::RelationTypeIn(values) => {
      Box::new(file_groups::relation_type.eq_any(values))
    }

    // 逻辑运算条件
    FileGroupCondition::And(conditions) => {
      let mut result: Option<
        Box<
          dyn BoxableExpression<
              file_groups::table,
              <AnyConnection as Connection>::Backend,
              SqlType = diesel::sql_types::Bool,
            >,
        >,
      > = None;
      for cond in conditions {
        let expr = build_file_group_condition(cond);
        match result {
          None => result = Some(expr),
          Some(prev) => result = Some(Box::new(prev.and(expr))),
        }
      }
      result.unwrap_or_else(|| Box::new(true.into_sql::<diesel::sql_types::Bool>()))
    }
    FileGroupCondition::Or(conditions) => {
      let mut result: Option<
        Box<
          dyn BoxableExpression<
              file_groups::table,
              <AnyConnection as Connection>::Backend,
              SqlType = diesel::sql_types::Bool,
            >,
        >,
      > = None;
      for cond in conditions {
        let expr = build_file_group_condition(cond);
        match result {
          None => result = Some(expr),
          Some(prev) => result = Some(Box::new(prev.or(expr))),
        }
      }
      result.unwrap_or_else(|| Box::new(false.into_sql::<diesel::sql_types::Bool>()))
    }
    FileGroupCondition::Not(condition) => {
      let expr = build_file_group_condition(*condition);
      Box::new(diesel::dsl::not(expr))
    }
  }
}

/// 根据过滤条件查询文件-分组关联列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 文件-分组关联过滤条件
/// - `limit`: 最大返回记录数（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
///
/// 注意: 服务层 filter 变体统一经 `filter_to_conditions` 转条件查询，此函数保留备用
#[allow(dead_code)]
pub fn select_file_groups_by_filter_with_limit(
  conn: &mut AnyConnection,
  search_input: FileGroupFilter,
  limit: Option<i64>,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
  // 使用 into_boxed() 来对查询进行类型擦除
  let mut base_query =
    file_groups::dsl::file_groups.into_boxed::<<AnyConnection as Connection>::Backend>();

  // 如果有限制数量，则添加限制
  if let Some(limit_value) = limit {
    base_query = base_query.limit(limit_value);
  }

  // 如果 search_input 中有各字段，则添加相应的过滤条件
  if let Some(file_id) = search_input.file_id {
    base_query = base_query.filter(file_groups::file_id.eq(file_id));
  }
  if let Some(group_id) = search_input.group_id {
    base_query = base_query.filter(file_groups::group_id.eq(group_id));
  }
  if let Some(relation_type) = search_input.relation_type {
    base_query = base_query.filter(file_groups::relation_type.eq(relation_type));
  }

  base_query.select(FileGroupDTO::as_select()).load(conn)
}

/// 根据过滤条件和选项查询文件-分组关联列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 文件-分组关联过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
///
/// 注意: 当前服务层统一走 conditions 查询路径，此函数预留，后续可能复用
#[allow(dead_code)]
pub fn select_file_groups_by_filter_with_options(
  conn: &mut AnyConnection,
  search_input: FileGroupFilter,
  options: FileGroupQueryOptions,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
  // 构造查询条件
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

  select_file_groups_by_conditions_with_options(conn, conditions, options)
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
pub fn select_file_groups_by_conditions_with_limit(
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

  query.select(FileGroupDTO::as_select()).load(conn)
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
      FileGroupOrderBy::FileId(direction) => match direction {
        OrderDirection::Asc => query.order(file_groups::file_id.asc()),
        OrderDirection::Desc => query.order(file_groups::file_id.desc()),
      },
      FileGroupOrderBy::GroupId(direction) => match direction {
        OrderDirection::Asc => query.order(file_groups::group_id.asc()),
        OrderDirection::Desc => query.order(file_groups::group_id.desc()),
      },
    };
  }

  query.select(FileGroupDTO::as_select()).load(conn)
}

/// 根据多个条件和高级选项查询文件-分组关联记录（支持分页）
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
pub fn select_file_groups_by_conditions_with_pagination(
  conn: &mut AnyConnection,
  conditions: Vec<FileGroupCondition>,
  options: FileGroupQueryOptions,
) -> Result<PaginationResult<FileGroupDTO>, diesel::result::Error> {
  let mut query = file_groups::table.into_boxed::<<AnyConnection as Connection>::Backend>();
  let mut count_query = file_groups::table.into_boxed::<<AnyConnection as Connection>::Backend>();

  // 对每个条件应用 AND 逻辑
  for condition in &conditions {
    let boxed_condition = build_file_group_condition(condition.clone());
    query = query.filter(boxed_condition);
    // 修复：为 count_query 重新构建条件而不是克隆
    let count_condition = build_file_group_condition(condition.clone());
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
      FileGroupOrderBy::FileId(direction) => match direction {
        OrderDirection::Asc => query.order(file_groups::file_id.asc()),
        OrderDirection::Desc => query.order(file_groups::file_id.desc()),
      },
      FileGroupOrderBy::GroupId(direction) => match direction {
        OrderDirection::Asc => query.order(file_groups::group_id.asc()),
        OrderDirection::Desc => query.order(file_groups::group_id.desc()),
      },
    };
  }

  let data = query.select(FileGroupDTO::as_select()).load(conn)?;

  // 构造分页结果
  let page = if options.page.is_some() { options.page.unwrap() } else { offset / limit + 1 };
  let page_size = if options.page_size.is_some() { options.page_size.unwrap() } else { limit };

  Ok(PaginationResult::new(data, page, page_size, total))
}

/// 检查指定分组是否为空（没有关联的文件）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 要检查的分组ID
///
/// 返回值:
/// 成功时返回布尔值，true表示分组为空，false表示分组不为空；失败则返回数据库错误
pub fn check_group_empty(
  conn: &mut AnyConnection,
  group_id: i32,
) -> Result<bool, diesel::result::Error> {
  let count =
    file_groups::table.filter(file_groups::group_id.eq(group_id)).count().first::<i64>(conn)?;

  Ok(count == 0)
}

// /// 根据给定条件批量删除文件-分组关联记录
// ///
// /// 参数:
// /// - `conn`: 数据库连接对象
// /// - `conditions`: 删除条件集合，各条件之间采用 AND 连接
// ///
// /// 返回值:
// /// 成功删除的记录数目或数据库错误
// pub fn delete_file_groups_by_conditions(
//     conn: &mut AnyConnection,
//     conditions: Vec<FileGroupCondition>,
// ) -> Result<usize, diesel::result::Error> {
//     let mut query = diesel::delete(file_groups::table).into_boxed::<<AnyConnection as Connection>::Backend>();
//
//     // 应用所有删除条件
//     for condition in conditions {
//         let boxed_condition = build_file_group_condition(condition);
//         query = query.filter(boxed_condition);
//     }
//
//     query.execute(conn)
// }
