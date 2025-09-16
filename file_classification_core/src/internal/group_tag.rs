use super::models::GroupTagDTO;
use crate::model::schema::group_tags;
use diesel::prelude::*;
use std::fmt::{Debug, Formatter, Result as fmtResult};
use crate::utils::database::AnyConnection;

// 在 group_tag.rs 中添加数据访问层函数
pub fn insert_group_tag(
    conn: &mut AnyConnection,
    group_tag_dto: &GroupTagDTO,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(group_tags::table)
        .values(group_tag_dto)
        .execute(conn)
}

pub fn delete_group_tag_by_id(
    conn: &mut AnyConnection,
    group_tag_dto: &GroupTagDTO,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(
        group_tags::table
            .filter(group_tags::group_id.eq(group_tag_dto.group_id))
            .filter(group_tags::tag_id.eq(group_tag_dto.tag_id))
    )
        .execute(conn)
}

impl Debug for GroupTagDTO {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "GroupTag {{ group_id: {}, tag_id: {} }}", self.group_id, self.tag_id)
    }
}

use super::models::GroupTagCondition;
use crate::model::models::{GroupTagOrderBy, GroupTagQueryOptions, OrderDirection};
use diesel::dsl::not;
use diesel::sql_types::Bool;

// 将 GroupTagCondition 转换为 diesel 查询条件的辅助函数
fn build_group_tag_condition(condition: GroupTagCondition) -> Box<dyn BoxableExpression<group_tags::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>> {
    match condition {
        GroupTagCondition::GroupId(id) => Box::new(group_tags::group_id.eq(id)),
        GroupTagCondition::TagId(id) => Box::new(group_tags::tag_id.eq(id)),

        GroupTagCondition::GroupIdGreaterThan(value) => Box::new(group_tags::group_id.gt(value)),
        GroupTagCondition::GroupIdLessThan(value) => Box::new(group_tags::group_id.lt(value)),
        GroupTagCondition::TagIdGreaterThan(value) => Box::new(group_tags::tag_id.gt(value)),
        GroupTagCondition::TagIdLessThan(value) => Box::new(group_tags::tag_id.lt(value)),
        
        GroupTagCondition::GroupIdIn(values) => Box::new(group_tags::group_id.eq_any(values)),
        GroupTagCondition::TagIdIn(values) => Box::new(group_tags::tag_id.eq_any(values)),

        GroupTagCondition::And(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<group_tags::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>>> = None;
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
            let mut result: Option<Box<dyn BoxableExpression<group_tags::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>>> = None;
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
            Box::new(not(expr))
        }
    }
}

// 根据 GroupTagCondition 向量查询组标签关联
pub fn select_group_tags_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<GroupTagCondition>,
    limit: Option<i64>,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
    let mut query = group_tags::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_group_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    query
        .select((group_tags::group_id, group_tags::tag_id))
        .load(conn)
}

#[allow(dead_code)]
pub fn select_group_tags_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<GroupTagCondition>,
    options: GroupTagQueryOptions,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
    let mut query = group_tags::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_group_tag_condition(condition);
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
            GroupTagOrderBy::GroupId(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(group_tags::group_id.asc()),
                    OrderDirection::Desc => query.order(group_tags::group_id.desc()),
                }
            }
            GroupTagOrderBy::TagId(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(group_tags::tag_id.asc()),
                    OrderDirection::Desc => query.order(group_tags::tag_id.desc()),
                }
            }
        };
    }

    query
        .select((group_tags::group_id, group_tags::tag_id))
        .load(conn)
}

pub fn delete_group_tags_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<GroupTagCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::delete(group_tags::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_group_tag_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.execute(conn)
}
