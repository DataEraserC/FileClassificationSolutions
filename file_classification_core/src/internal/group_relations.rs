// group_relations.rs
//! 组关系管理模块
//!
//! 提供对组关系表 (`group_relations`) 的增删改查操作支持，包括基本的CRUD操作、条件查询、批量操作等。

use super::models::GroupRelation;
use super::models::GroupRelationCondition;
use crate::model::models::{
    GroupRelationFilter, GroupRelationOrderBy, GroupRelationQueryOptions, OrderDirection, PaginationResult, RELATION_TYPE_PARENT_CHILD,
};
use crate::model::schema::group_relations;
use crate::utils::database::AnyConnection;
use diesel::prelude::*;

/// 插入一个新的组关系记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_relation`: 包含待插入数据的对象
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回数据库错误
pub fn insert_group_relation(
    conn: &mut AnyConnection,
    group_relation: &GroupRelation,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(group_relations::table).values(group_relation).execute(conn)
}

/// 根据 DTO 中的信息删除一个组关系记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_relation`: 包含要删除记录信息的对象
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回数据库错误
pub fn delete_group_relation_by_dto(
    conn: &mut AnyConnection,
    group_relation: &GroupRelation,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(
        group_relations::table
            .filter(group_relations::first_group_id.eq(group_relation.first_group_id))
            .filter(group_relations::second_group_id.eq(group_relation.second_group_id))
            .filter(group_relations::relation_type.eq(group_relation.relation_type)),
    )
        .execute(conn)
}

/// 根据多个 DTO 对象批量删除组关系记录（带事务支持）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `relations`: 包含要删除记录信息的对象向量
///
/// 返回值:
/// 成功时返回影响的行数，失败则返回数据库错误
pub fn delete_group_relations_by_dtos(
    conn: &mut AnyConnection,
    relations: Vec<GroupRelation>,
) -> Result<usize, diesel::result::Error> {
    conn.transaction::<usize, diesel::result::Error, _>(|conn| {
        let mut total_deleted = 0;

        for relation in relations {
            total_deleted += delete_group_relation_by_dto(conn, &relation)?;
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
fn build_group_relation_condition(
    condition: GroupRelationCondition,
) -> Box<
    dyn BoxableExpression<
        group_relations::table,
        <AnyConnection as Connection>::Backend,
        SqlType=diesel::sql_types::Bool,
    >,
> {
    match condition {
        GroupRelationCondition::FirstGroupId(id) => Box::new(group_relations::first_group_id.eq(id)),
        GroupRelationCondition::SecondGroupId(id) => Box::new(group_relations::second_group_id.eq(id)),
        GroupRelationCondition::RelationType(typ) => Box::new(group_relations::relation_type.eq(typ)),

        GroupRelationCondition::FirstGroupIdGreaterThan(value) => {
            Box::new(group_relations::first_group_id.gt(value))
        }
        GroupRelationCondition::FirstGroupIdLessThan(value) => {
            Box::new(group_relations::first_group_id.lt(value))
        }
        GroupRelationCondition::SecondGroupIdGreaterThan(value) => {
            Box::new(group_relations::second_group_id.gt(value))
        }
        GroupRelationCondition::SecondGroupIdLessThan(value) => {
            Box::new(group_relations::second_group_id.lt(value))
        }
        GroupRelationCondition::RelationTypeGreaterThan(value) => {
            Box::new(group_relations::relation_type.gt(value))
        }
        GroupRelationCondition::RelationTypeLessThan(value) => {
            Box::new(group_relations::relation_type.lt(value))
        }

        GroupRelationCondition::FirstGroupIdIn(values) => {
            Box::new(group_relations::first_group_id.eq_any(values))
        }
        GroupRelationCondition::SecondGroupIdIn(values) => {
            Box::new(group_relations::second_group_id.eq_any(values))
        }
        GroupRelationCondition::RelationTypeIn(values) => {
            Box::new(group_relations::relation_type.eq_any(values))
        }

        GroupRelationCondition::And(conditions) => {
            let mut result: Option<
                Box<
                    dyn BoxableExpression<
                        group_relations::table,
                        <AnyConnection as Connection>::Backend,
                        SqlType=diesel::sql_types::Bool,
                    >,
                >,
            > = None;
            for cond in conditions {
                let expr = build_group_relation_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.and(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(true.into_sql::<diesel::sql_types::Bool>()))
        }
        GroupRelationCondition::Or(conditions) => {
            let mut result: Option<
                Box<
                    dyn BoxableExpression<
                        group_relations::table,
                        <AnyConnection as Connection>::Backend,
                        SqlType=diesel::sql_types::Bool,
                    >,
                >,
            > = None;
            for cond in conditions {
                let expr = build_group_relation_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.or(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(false.into_sql::<diesel::sql_types::Bool>()))
        }
        GroupRelationCondition::Not(condition) => {
            let expr = build_group_relation_condition(*condition);
            Box::new(diesel::dsl::not(expr))
        }
    }
}

/// 根据过滤条件查询组关系列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 组关系过滤条件
/// - `limit`: 最大返回记录数
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_relations_by_filter(
    conn: &mut AnyConnection,
    search_input: GroupRelationFilter,
    limit: i64,
) -> Result<Vec<GroupRelation>, diesel::result::Error> {
    // 使用 into_boxed() 来对查询进行类型擦除
    let mut base_query = group_relations::dsl::group_relations.limit(limit).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 如果 search_input 中有各字段，则添加相应的过滤条件
    if let Some(first_group_id) = search_input.first_group_id {
        base_query = base_query.filter(group_relations::first_group_id.eq(first_group_id));
    }
    if let Some(second_group_id) = search_input.second_group_id {
        base_query = base_query.filter(group_relations::second_group_id.eq(second_group_id));
    }
    if let Some(relation_type) = search_input.relation_type {
        base_query = base_query.filter(group_relations::relation_type.eq(relation_type));
    }

    base_query.select(GroupRelation::as_select()).load::<GroupRelation>(conn)
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

    select_group_relations_by_conditions_with_options(conn, conditions, options)
}

/// 根据多个条件查询组关系记录，并可设置最大返回数量
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件集合，各条件之间采用 AND 连接
/// - `limit`: 最大返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_group_relations_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<GroupRelationCondition>,
    limit: Option<i64>,
) -> Result<Vec<GroupRelation>, diesel::result::Error> {
    let mut query = group_relations::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_group_relation_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 设置返回条目上限
    if let Some(limit) = limit {
        query = query.limit(limit)
    }

    query.select(GroupRelation::as_select()).load::<GroupRelation>(conn)
}

/// 根据多个条件和高级选项查询组关系记录
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
pub fn select_group_relations_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<GroupRelationCondition>,
    options: GroupRelationQueryOptions,
) -> Result<Vec<GroupRelation>, diesel::result::Error> {
    let mut query = group_relations::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_group_relation_condition(condition);
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
            GroupRelationOrderBy::FirstGroupId(direction) => match direction {
                OrderDirection::Asc => query.order(group_relations::first_group_id.asc()),
                OrderDirection::Desc => query.order(group_relations::first_group_id.desc()),
            },
            GroupRelationOrderBy::SecondGroupId(direction) => match direction {
                OrderDirection::Asc => query.order(group_relations::second_group_id.asc()),
                OrderDirection::Desc => query.order(group_relations::second_group_id.desc()),
            },
            GroupRelationOrderBy::RelationType(direction) => match direction {
                OrderDirection::Asc => query.order(group_relations::relation_type.asc()),
                OrderDirection::Desc => query.order(group_relations::relation_type.desc()),
            },
        };
    }

    query.select(GroupRelation::as_select()).load::<GroupRelation>(conn)
}

/// 根据多个条件和高级选项查询组关系记录（支持分页）
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
pub fn select_group_relations_by_conditions_with_pagination(
    conn: &mut AnyConnection,
    conditions: Vec<GroupRelationCondition>,
    options: GroupRelationQueryOptions,
) -> Result<PaginationResult<GroupRelation>, diesel::result::Error> {
    let mut query = group_relations::table.into_boxed::<<AnyConnection as Connection>::Backend>();
    let mut count_query = group_relations::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in &conditions {
        let boxed_condition = build_group_relation_condition(condition.clone());
        query = query.filter(boxed_condition);
        // 修复：为 count_query 重新构建条件而不是克隆
        let count_condition = build_group_relation_condition(condition.clone());
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
            GroupRelationOrderBy::FirstGroupId(direction) => match direction {
                OrderDirection::Asc => query.order(group_relations::first_group_id.asc()),
                OrderDirection::Desc => query.order(group_relations::first_group_id.desc()),
            },
            GroupRelationOrderBy::SecondGroupId(direction) => match direction {
                OrderDirection::Asc => query.order(group_relations::second_group_id.asc()),
                OrderDirection::Desc => query.order(group_relations::second_group_id.desc()),
            },
            GroupRelationOrderBy::RelationType(direction) => match direction {
                OrderDirection::Asc => query.order(group_relations::relation_type.asc()),
                OrderDirection::Desc => query.order(group_relations::relation_type.desc()),
            },
        };
    }

    let data = query.select(GroupRelation::as_select()).load(conn)?;

    // 构造分页结果
    let page = if options.page.is_some() { options.page.unwrap() } else { offset / limit + 1 };
    let page_size = if options.page_size.is_some() { options.page_size.unwrap() } else { limit };

    Ok(PaginationResult::new(data, page, page_size, total))
}

/// 检查两个组之间是否存在指定类型的关系
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `first_group_id`: 第一个组ID
/// - `second_group_id`: 第二个组ID
/// - `relation_type`: 关系类型
///
/// 返回值:
/// 成功时返回布尔值，true表示存在关系，false表示不存在关系；失败则返回数据库错误
pub fn check_group_relation_exists(
    conn: &mut AnyConnection,
    first_id: i32,
    second_id: i32,
    rel_type: i32,
) -> Result<bool, diesel::result::Error> {
    let count = group_relations::table
        .filter(group_relations::first_group_id.eq(first_id))
        .filter(group_relations::second_group_id.eq(second_id))
        .filter(group_relations::relation_type.eq(rel_type))
        .count()
        .first::<i64>(conn)?;

    Ok(count > 0)
}

/// 获取指定组的所有父组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 组ID
/// - `rel_type`: 关系类型（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn get_first_group(
    conn: &mut AnyConnection,
    group_id: i32,
    rel_type: Option<i32>,
) -> Result<Vec<GroupRelation>, diesel::result::Error> {
    let query = group_relations::table.filter(group_relations::second_group_id.eq(group_id));

    if let Some(relation_type_value) = rel_type {
        query
            .filter(group_relations::relation_type.eq(relation_type_value))
            .select(GroupRelation::as_select())
            .load::<GroupRelation>(conn)
    } else {
        query.select(GroupRelation::as_select()).load::<GroupRelation>(conn)
    }
}

/// 获取指定组的所有子组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 组ID
/// - `rel_type`: 关系类型（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn get_second_group(
    conn: &mut AnyConnection,
    group_id: i32,
    rel_type: Option<i32>,
) -> Result<Vec<GroupRelation>, diesel::result::Error> {
    let query = group_relations::table.filter(group_relations::first_group_id.eq(group_id));

    if let Some(relation_type_value) = rel_type {
        query
            .filter(group_relations::relation_type.eq(relation_type_value))
            .select(GroupRelation::as_select())
            .load::<GroupRelation>(conn)
    } else {
        query.select(GroupRelation::as_select()).load::<GroupRelation>(conn)
    }
}

/// 获取指定组的直接子组ID列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 组ID
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn get_direct_children_ids(
    conn: &mut AnyConnection,
    group_id: i32,
) -> Result<Vec<i32>, diesel::result::Error> {
    group_relations::table
        .filter(group_relations::first_group_id.eq(group_id))
        .filter(group_relations::relation_type.eq(RELATION_TYPE_PARENT_CHILD))
        .select(group_relations::second_group_id)
        .load::<i32>(conn)
}

/// 获取指定组的直接父组ID列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 组ID
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn get_direct_parents_ids(
    conn: &mut AnyConnection,
    group_id: i32,
) -> Result<Vec<i32>, diesel::result::Error> {
    group_relations::table
        .filter(group_relations::second_group_id.eq(group_id))
        .filter(group_relations::relation_type.eq(RELATION_TYPE_PARENT_CHILD))
        .select(group_relations::first_group_id)
        .load::<i32>(conn)
}

/// 获取指定组的所有父组ID列表（用于循环检测）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 组ID
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn get_first_group_ids(
    conn: &mut AnyConnection,
    group_id: i32,
    relation_type_value: i32,
) -> Result<Vec<i32>, diesel::result::Error> {
    group_relations::table
        .filter(group_relations::second_group_id.eq(group_id))
        .filter(group_relations::relation_type.eq(relation_type_value))
        .select(group_relations::first_group_id)
        .load::<i32>(conn)
}