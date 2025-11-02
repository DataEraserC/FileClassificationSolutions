// tags.rs
//! 标签管理模块
//!
//! 提供对标签表 (`tags`) 的增删改查操作支持，包括基本的CRUD操作、条件查询、批量操作等。

use super::models::{CreateTagDTO, Tag, TagCondition, TagFilter, UpdateTagDTO};
use crate::model::models::{OrderDirection, PaginationResult, TagOrderBy, TagQueryOptions};
use crate::model::schema::tags;
use crate::model::schema::tags::dsl::*;
use crate::utils::database::AnyConnection;
use diesel::prelude::*;
use diesel::sql_types::Bool;

/// 创建一个新的标签记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `new_tag`: 包含待插入标签数据的 DTO 对象
///
/// 返回值:
/// 成功时返回插入记录的ID，失败则返回数据库错误
pub fn insert_tag(
    conn: &mut AnyConnection,
    new_tag: &CreateTagDTO,
) -> Result<i32, diesel::result::Error> {
    // 使用 match 表达式根据连接类型选择实现方式
    match conn {
        // 对于 SQLite 连接，使用 returning 子句
        AnyConnection::Sqlite(_) => {
            diesel::insert_into(tags::table).values(new_tag).returning(tags::id).get_result(conn)
        }
        // 对于 MySQL 连接，使用事务方式（暂时注释掉，因为目前没有启用mysql，注意不要删除以下注释内容，将来会用到）
        /*
        AnyConnection::Mysql(_) => {
                conn.transaction(|conn| {
                        // 执行插入操作
                        diesel::insert_into(tags::table)
                                .values(new_tag)
                                .execute(conn)?;

                        // MySQL使用LAST_INSERT_ID()获取最后插入的ID
                        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>("LAST_INSERT_ID()"))
                                .get_result(conn)?;

                        Ok(last_id)
                })
        },
        */
        // 默认情况（如其他数据库类型）使用 returning 子句
        _ => diesel::insert_into(tags::table).values(new_tag).returning(tags::id).get_result(conn),
    }
}

/// 根据名称查找标签记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_name`: 要查找的标签名称
///
/// 返回值:
/// 成功时返回匹配的标签记录（如果存在），失败则返回数据库错误
#[allow(dead_code)]
pub fn find_tag_by_name(
    conn: &mut AnyConnection,
    tag_name: &str,
) -> Result<Option<Tag>, diesel::result::Error> {
    tags.filter(tags::name.eq(tag_name)).select(Tag::as_select()).first::<Tag>(conn).optional()
}

/// 根据ID查找标签记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_id`: 要查找的标签ID
///
/// 返回值:
/// 成功时返回匹配的标签记录（如果存在），失败则返回数据库错误
pub fn find_tag_by_id(
    conn: &mut AnyConnection,
    tag_id: i32,
) -> Result<Option<Tag>, diesel::result::Error> {
    tags.filter(tags::id.eq(tag_id)).select(Tag::as_select()).first::<Tag>(conn).optional()
}

/// 增加标签的引用计数
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_id`: 标签ID
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回数据库错误
pub fn increase_tag_reference_count_by_id(
    conn: &mut AnyConnection,
    tag_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(tags::table.find(tag_id))
        .set(tags::reference_count.eq(tags::reference_count + 1))
        .execute(conn)
}

/// 减少标签的引用计数
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_id`: 标签ID
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回数据库错误
pub fn decrease_tag_reference_count_by_id(
    conn: &mut AnyConnection,
    tag_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(tags::table.find(tag_id))
        .set(tags::reference_count.eq(tags::reference_count - 1))
        .execute(conn)
}

/// 根据多个标签ID批量增加标签引用计数
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_ids`: 标签ID列表
///
/// 返回值:
/// 成功时返回影响的行数，失败则返回数据库错误
pub fn increase_tag_reference_count_by_ids(
    conn: &mut AnyConnection,
    tag_ids: Vec<i32>,
) -> Result<usize, diesel::result::Error> {
    diesel::update(tags::table)
        .filter(tags::id.eq_any(tag_ids))
        .set(tags::reference_count.eq(tags::reference_count + 1))
        .execute(conn)
}

/// 根据多个标签ID批量减少标签引用计数
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_ids`: 标签ID列表
///
/// 返回值:
/// 成功时返回影响的行数，失败则返回数据库错误
pub fn decrease_tag_reference_count_by_ids(
    conn: &mut AnyConnection,
    tag_ids: Vec<i32>,
) -> Result<usize, diesel::result::Error> {
    diesel::update(tags::table)
        .filter(tags::id.eq_any(tag_ids))
        .set(tags::reference_count.eq(tags::reference_count - 1))
        .execute(conn)
}

/// 根据过滤条件查询标签列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 标签过滤条件
/// - `limit`: 最大返回记录数
///
/// 返回值:
/// 查询成功的标签记录列表或数据库错误
pub fn select_tags_by_filter(
    conn: &mut AnyConnection,
    search_input: TagFilter,
    limit: i64,
) -> Result<Vec<Tag>, diesel::result::Error> {
    // 使用 into_boxed() 来对查询进行类型擦除
    let mut base_query = tags.limit(limit).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 如果 search_input 中有 id，则添加过滤条件
    if let Some(tag_id) = search_input.id {
        base_query = base_query.filter(tags::id.eq(tag_id));
    }
    if let Some(tag_name) = search_input.name {
        base_query = base_query.filter(tags::name.eq(tag_name));
    }

    // 执行查询，手动指定选择的字段
    base_query.select(Tag::as_select()).load(conn)
}

/// 根据标签ID删除标签记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_id`: 要删除的标签ID
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回数据库错误
pub fn delete_tag_by_id(
    conn: &mut AnyConnection,
    tag_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(tags.filter(tags::id.eq(tag_id))).execute(conn)
}

/// 根据多个标签ID批量删除标签记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_ids`: 要删除的标签ID列表
///
/// 返回值:
/// 成功时返回影响的行数，失败则返回数据库错误
pub fn delete_tags_by_ids(
    conn: &mut AnyConnection,
    tag_ids: Vec<i32>,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(tags::table.filter(tags::id.eq_any(tag_ids))).execute(conn)
}

/// 构建符合 Diesel 查询语法的条件表达式
///
/// 参数:
/// - `condition`: 表达查询条件的数据结构
///
/// 返回值:
/// 符合 Diesel 查询条件类型的动态表达式盒子
fn build_tag_condition(
    condition: TagCondition,
) -> Box<
    dyn BoxableExpression<
        tags::table,
        <AnyConnection as Connection>::Backend,
        SqlType=diesel::sql_types::Bool,
    >,
> {
    match condition {
        TagCondition::Id(_id) => Box::new(tags::id.eq(_id)),
        TagCondition::Name(_name) => Box::new(tags::name.eq(_name)),
        TagCondition::ReferenceCount(count) => Box::new(tags::reference_count.eq(count)),

        TagCondition::IdGreaterThan(value) => Box::new(tags::id.gt(value)),
        TagCondition::IdLessThan(value) => Box::new(tags::id.lt(value)),
        TagCondition::NameLike(pattern) => Box::new(tags::name.like(pattern)),
        TagCondition::ReferenceCountGreaterThan(value) => Box::new(tags::reference_count.gt(value)),
        TagCondition::ReferenceCountLessThan(value) => Box::new(tags::reference_count.lt(value)),

        TagCondition::IdIn(values) => Box::new(tags::id.eq_any(values)),
        TagCondition::NameIn(values) => Box::new(tags::name.eq_any(values)),
        TagCondition::ReferenceCountIn(values) => Box::new(tags::reference_count.eq_any(values)),

        TagCondition::And(conditions) => {
            let mut result: Option<
                Box<
                    dyn BoxableExpression<
                        tags::table,
                        <AnyConnection as Connection>::Backend,
                        SqlType=diesel::sql_types::Bool,
                    >,
                >,
            > = None;
            for cond in conditions {
                let expr = build_tag_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.and(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(true.into_sql::<Bool>()))
        }
        TagCondition::Or(conditions) => {
            let mut result: Option<
                Box<
                    dyn BoxableExpression<
                        tags::table,
                        <AnyConnection as Connection>::Backend,
                        SqlType=diesel::sql_types::Bool,
                    >,
                >,
            > = None;
            for cond in conditions {
                let expr = build_tag_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.or(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(false.into_sql::<Bool>()))
        }
        TagCondition::Not(condition) => {
            let expr = build_tag_condition(*condition);
            Box::new(diesel::dsl::not(expr))
        }
    }
}

/// 根据多个条件查询标签记录，并可设置最大返回数量
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件集合，各条件之间采用 AND 连接
/// - `limit`: 最大返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的标签记录列表或数据库错误
pub fn select_tags_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
    limit: Option<i64>,
) -> Result<Vec<Tag>, diesel::result::Error> {
    let mut query = tags::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    query.select(Tag::as_select()).load(conn)
}

/// 根据多个条件和高级选项查询标签记录
///
/// 支持分页、排序等复杂查询需求
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件集合，各条件之间采用 AND 连接
/// - `options`: 查询选项，包括分页和排序配置
///
/// 返回值:
/// 查询成功的标签记录列表或数据库错误
#[allow(dead_code)]
pub fn select_tags_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
    options: TagQueryOptions,
) -> Result<Vec<Tag>, diesel::result::Error> {
    let mut query = tags::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 应用查询选项（排序、限制等）
    if let Some(limit) = options.limit {
        query = query.limit(limit);
    }

    if let Some(offset) = options.offset {
        query = query.offset(offset);
    }

    // 应用排序
    for order_by in options.order_by {
        query = match order_by {
            TagOrderBy::Id(direction) => match direction {
                OrderDirection::Asc => query.order(tags::id.asc()),
                OrderDirection::Desc => query.order(tags::id.desc()),
            },
            TagOrderBy::Name(direction) => match direction {
                OrderDirection::Asc => query.order(tags::name.asc()),
                OrderDirection::Desc => query.order(tags::name.desc()),
            },
            TagOrderBy::ReferenceCount(direction) => match direction {
                OrderDirection::Asc => query.order(tags::reference_count.asc()),
                OrderDirection::Desc => query.order(tags::reference_count.desc()),
            },
        };
    }

    query.select(Tag::as_select()).load(conn)
}

/// 根据多个条件和高级选项查询标签记录（支持分页）
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
pub fn select_tags_by_conditions_with_pagination(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
    options: TagQueryOptions,
) -> Result<PaginationResult<Tag>, diesel::result::Error> {
    let mut query = tags::table.into_boxed::<<AnyConnection as Connection>::Backend>();
    let mut count_query = tags::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in &conditions {
        let boxed_condition = build_tag_condition(condition.clone());
        query = query.filter(boxed_condition);
        // 修复：为 count_query 重新构建条件而不是克隆
        let count_condition = build_tag_condition(condition.clone());
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
            TagOrderBy::Id(direction) => match direction {
                OrderDirection::Asc => query.order(tags::id.asc()),
                OrderDirection::Desc => query.order(tags::id.desc()),
            },
            TagOrderBy::Name(direction) => match direction {
                OrderDirection::Asc => query.order(tags::name.asc()),
                OrderDirection::Desc => query.order(tags::name.desc()),
            },
            TagOrderBy::ReferenceCount(direction) => match direction {
                OrderDirection::Asc => query.order(tags::reference_count.asc()),
                OrderDirection::Desc => query.order(tags::reference_count.desc()),
            },
        };
    }

    let data = query.select(Tag::as_select()).load(conn)?;

    // 构造分页结果
    let page = if options.page.is_some() { options.page.unwrap() } else { offset / limit + 1 };
    let page_size = if options.page_size.is_some() { options.page_size.unwrap() } else { limit };

    Ok(PaginationResult::new(data, page, page_size, total))
}

/// 根据给定条件批量更新标签记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 更新条件集合，各条件之间采用 AND 连接
/// - `update_set`: 包含更新数据的 DTO 对象
///
/// 返回值:
/// 成功更新的记录数目或数据库错误
pub fn update_tags_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
    update_set: UpdateTagDTO,
) -> Result<usize, diesel::result::Error> {
    let mut query =
        diesel::update(tags::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = crate::internal::tags::build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.set(update_set).execute(conn)
}

/// 根据给定条件批量删除标签记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 删除条件集合，各条件之间采用 AND 连接
///
/// 返回值:
/// 成功删除的记录数目或数据库错误
pub fn delete_tags_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query =
        diesel::delete(tags::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.execute(conn)
}

/// 根据给定条件批量增加标签引用计数
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件集合，各条件之间采用 AND 连接
///
/// 返回值:
/// 成功更新的记录数目或数据库错误
pub fn increase_tags_reference_count_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query =
        diesel::update(tags::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 增加引用计数
    query.set(tags::reference_count.eq(tags::reference_count + 1)).execute(conn)
}

/// 根据给定条件批量减少标签引用计数
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件集合，各条件之间采用 AND 连接
///
/// 返回值:
/// 成功更新的记录数目或数据库错误
pub fn decrease_tags_reference_count_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query =
        diesel::update(tags::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 减少引用计数
    query.set(tags::reference_count.eq(tags::reference_count - 1)).execute(conn)
}

/// 根据分组ID查询关联的标签列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 分组ID
///
/// 返回值:
/// 查询成功的标签记录列表或数据库错误
pub fn select_tag_by_group_id(
    conn: &mut AnyConnection,
    group_id: i32,
) -> Result<Vec<Tag>, diesel::result::Error> {
    use crate::model::schema::group_tags;

    tags::table
        .inner_join(group_tags::table.on(tags::id.eq(group_tags::tag_id)))
        .filter(group_tags::group_id.eq(group_id))
        .select(Tag::as_select())
        .load(conn)
}

/// 根据标签ID获取标签详情
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_id`: 标签ID
///
/// 返回值:
/// 查询成功的标签记录或数据库错误
pub fn get_tag_by_id(conn: &mut AnyConnection, tag_id: i32) -> Result<Tag, diesel::result::Error> {
    tags::table.filter(tags::id.eq(tag_id)).select(Tag::as_select()).first(conn)
}

/// 根据标签ID更新标签信息
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_id`: 标签ID
/// - `update_set`: 包含更新数据的 DTO 对象
///
/// 返回值:
/// 成功时返回影响的行数（通常应为1），失败则返回数据库错误
pub fn update_tag_by_id(
    conn: &mut AnyConnection,
    tag_id: i32,
    update_set: UpdateTagDTO,
) -> Result<usize, diesel::result::Error> {
    diesel::update(tags::table.filter(tags::id.eq(tag_id))).set(update_set).execute(conn)
}
