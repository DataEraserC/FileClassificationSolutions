use super::models::{CreateTagDTO, Tag, TagFilter};
use diesel::prelude::*;
use crate::utils::database::AnyConnection;

pub fn create_tag(
    conn: &mut AnyConnection,
    new_tag: CreateTagDTO,
) -> Result<Tag, diesel::result::Error> {
    diesel::insert_into(tags::table)
        .values(&new_tag)
        .execute(conn)?;

    // 手动获取最新插入的记录
    tags.order(tags::id.desc())
        .select((tags::id, tags::name, tags::reference_count))
        .first(conn)
}

#[allow(dead_code)]
pub fn find_tag_by_name(
    conn: &mut AnyConnection,
    tag_name: &str,
) -> Result<Option<Tag>, diesel::result::Error> {
    tags.filter(tags::name.eq(tag_name))
        .select((tags::id, tags::name, tags::reference_count))
        .first::<Tag>(conn)
        .optional()
}

pub fn find_tag_by_id(conn: &mut AnyConnection, tag_id: i32) -> Result<Option<Tag>, diesel::result::Error> {
    tags.filter(tags::id.eq(tag_id))
        .select((tags::id, tags::name, tags::reference_count))
        .first::<Tag>(conn)
        .optional()
}

pub fn increase_tag_reference_count(
    conn: &mut AnyConnection,
    tag_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(tags::table.find(tag_id))
        .set(tags::reference_count.eq(tags::reference_count + 1))
        .execute(conn)
}

pub fn decrease_tag_reference_count(
    conn: &mut AnyConnection,
    tag_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(tags::table.find(tag_id))
        .set(tags::reference_count.eq(tags::reference_count - 1))
        .execute(conn)
}

pub fn select_tags(
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
    base_query
        .select((tags::id, tags::name, tags::reference_count))
        .load(conn)
}

pub fn delete_tag(conn: &mut AnyConnection, tag_id: i32) -> Result<usize, diesel::result::Error> {
    diesel::delete(tags.filter(tags::id.eq(tag_id))).execute(conn)
}

use crate::model::schema::tags::dsl::*;
use crate::model::schema::{groups, tags};
use std::fmt::{Debug, Formatter, Result as fmtResult};

impl Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "Tag {{ id: {}, name: {}, reference_count: {} }}",
            self.id, self.name, self.reference_count
        )
    }
}

use super::models::TagCondition;
use crate::model::models::{Group, OrderDirection, TagOrderBy, TagQueryOptions, UpdateTagDTO};
use diesel::dsl::not;
use diesel::sql_types::Bool;
use crate::model::schema::group_tags::dsl::group_tags;

// 将 TagCondition 转换为 diesel 查询条件的辅助函数
fn build_tag_condition(condition: TagCondition) -> Box<dyn BoxableExpression<tags::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>> {
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
            let mut result: Option<Box<dyn BoxableExpression<tags::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>>> = None;
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
            let mut result: Option<Box<dyn BoxableExpression<tags::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>>> = None;
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
            Box::new(not(expr))
        }
    }
}

// 根据 TagCondition 向量查询标签
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

    query
        .select((tags::id, tags::name, tags::reference_count))
        .load(conn)
}

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
            TagOrderBy::Id(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(tags::id.asc()),
                    OrderDirection::Desc => query.order(tags::id.desc()),
                }
            }
            TagOrderBy::Name(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(tags::name.asc()),
                    OrderDirection::Desc => query.order(tags::name.desc()),
                }
            }
            TagOrderBy::ReferenceCount(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(tags::reference_count.asc()),
                    OrderDirection::Desc => query.order(tags::reference_count.desc()),
                }
            }
        };
    }

    query
        .select((tags::id, tags::name, tags::reference_count))
        .load(conn)
}

pub fn update_tags_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
    update_set: UpdateTagDTO,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::update(tags::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = crate::internal::tags::build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.set(update_set).execute(conn)
}

pub fn delete_tags_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::delete(tags::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.execute(conn)
}

pub fn increase_tags_reference_count_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::update(tags::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 增加引用计数
    query.set(tags::reference_count.eq(tags::reference_count + 1)).execute(conn)
}

pub fn decrease_tags_reference_count_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::update(tags::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 减少引用计数
    query.set(tags::reference_count.eq(tags::reference_count - 1)).execute(conn)
}

pub fn select_tag_by_group_id(
    conn: &mut AnyConnection,
    group_id: i64,
) -> Result<Vec<Tag>, diesel::result::Error> {
    use crate::model::schema::group_tags;

    tags::table
        .inner_join(group_tags::table.on(tags::id.eq(group_tags::tag_id)))
        .filter(group_tags::group_id.eq(group_id as i32))
        .select(Tag::as_select())
        .load(conn)
}

pub fn get_tag_by_id(
    conn: &mut AnyConnection,
    tag_id: i32,
) -> Result<Tag, diesel::result::Error> {
    tags::table
        .filter(tags::id.eq(tag_id))
        .select(Tag::as_select())
        .first(conn)
}