use super::{models::{CreateTagDTO, Tag, TagFilter}, AppError};
use diesel::prelude::*;
pub fn create_tag(
    conn: &mut SqliteConnection,
    new_tag: CreateTagDTO,
) -> Result<Tag, diesel::result::Error> {
    diesel::insert_into(tags::table).values(&new_tag).returning(Tag::as_returning()).get_result(conn)
}
#[allow(dead_code)]
pub fn find_tag_by_name(
    conn: &mut SqliteConnection,
    tag_name: &str,
) -> Result<Option<Tag>, AppError> {
    let tag =
        tags.select(Tag::as_select()).filter(tags::name.eq(tag_name)).first::<Tag>(conn).optional()?;

    Ok(tag)
}
#[allow(dead_code)]
pub fn find_tag_by_id(conn: &mut SqliteConnection, tag_id: i32) -> Result<Option<Tag>, AppError> {
    let tag =
        tags.select(Tag::as_select()).filter(tags::id.eq(tag_id)).first::<Tag>(conn).optional()?;

    Ok(tag)
}
pub fn increase_tag_reference_count(
    conn: &mut SqliteConnection,
    tag_id: i32,
) -> Result<(), AppError> {
    diesel::update(tags::table.find(tag_id))
        .set(tags::reference_count.eq(tags::reference_count + 1))
        .execute(conn)?;

    Ok(())
}
#[allow(dead_code)]
pub fn decrease_tag_reference_count(
    conn: &mut SqliteConnection,
    tag_id: i32,
) -> Result<(), AppError> {
    diesel::update(tags::table.find(tag_id))
        .set(tags::reference_count.eq(tags::reference_count - 1))
        .execute(conn)?;

    Ok(())
}

pub fn select_tags(
    conn: &mut SqliteConnection,
    search_input: TagFilter,
    limit: i64,
) -> Result<Vec<Tag>, diesel::result::Error> {
    // 使用 into_boxed() 来对查询进行类型擦除
    let mut base_query = tags.limit(limit).select(Tag::as_select()).into_boxed();

    // 如果 search_input 中有 id，则添加过滤条件
    if let Some(tag_id) = search_input.id {
        base_query = base_query.filter(tags::id.eq(tag_id));
    }
    if let Some(tag_name) = search_input.name {
        base_query = base_query.filter(tags::name.eq(tag_name));
    }

    // 执行查询
    base_query.load(conn)
}

pub fn delete_tag(conn: &mut SqliteConnection, tag_id: i32) -> Result<(), diesel::result::Error> {
    match diesel::delete(tags.filter(tags::id.eq(tag_id))).execute(conn) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

use crate::model::schema::tags;
use crate::model::schema::tags::dsl::*;
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
use diesel::dsl::not;
use diesel::sql_types::Bool;
use diesel::sqlite::Sqlite;

// 将 TagCondition 转换为 diesel 查询条件的辅助函数
fn build_tag_condition(condition: TagCondition) -> Box<dyn BoxableExpression<tags::table, Sqlite, SqlType = diesel::sql_types::Bool>> {
    match condition {
        TagCondition::Id(_id) => Box::new(tags::id.eq(_id)),
        TagCondition::Name(_name) => Box::new(tags::name.eq(_name)),
        TagCondition::ReferenceCount(count) => Box::new(tags::reference_count.eq(count)),

        TagCondition::IdGreaterThan(value) => Box::new(tags::id.gt(value)),
        TagCondition::IdLessThan(value) => Box::new(tags::id.lt(value)),
        TagCondition::NameLike(pattern) => Box::new(tags::name.like(pattern)),
        TagCondition::ReferenceCountGreaterThan(value) => Box::new(tags::reference_count.gt(value)),
        TagCondition::ReferenceCountLessThan(value) => Box::new(tags::reference_count.lt(value)),

        TagCondition::And(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<tags::table, Sqlite, SqlType = diesel::sql_types::Bool>>> = None;
            for cond in conditions {
                let expr = build_tag_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.and(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(true.into_sql::<Bool>()))
        },
        TagCondition::Or(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<tags::table, Sqlite, SqlType = diesel::sql_types::Bool>>> = None;
            for cond in conditions {
                let expr = build_tag_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.or(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(false.into_sql::<Bool>()))
        },
        TagCondition::Not(condition) => {
            let expr = build_tag_condition(*condition);
            Box::new(not(expr))
        }
    }
}

// 根据 TagCondition 向量查询标签
pub fn select_tags_by_conditions(
    conn: &mut SqliteConnection,
    conditions: Vec<TagCondition>,
    limit: i64,
) -> Result<Vec<Tag>, diesel::result::Error> {
    let mut query = tags::table.into_boxed::<Sqlite>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    query
        .limit(limit)
        .select(Tag::as_select())
        .load(conn)
}
