use super::models::{CreateGroupDTO, Group, GroupFilter};
use diesel::prelude::*;
pub fn create_group(conn: &mut AnyConnection, new_group: &CreateGroupDTO) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(groups::table)
        .values(new_group).execute(conn)
}

pub fn find_group_by_name(
    conn: &mut AnyConnection,
    group_name: &str,
) -> Result<Option<Group>, diesel::result::Error> {
    groups
        .select(Group::as_select())
        .filter(groups::name.eq(group_name))
        .first::<Group>(conn)
        .optional()
}
pub fn find_group_by_id(
    conn: &mut AnyConnection,
    group_id: i32,
) -> Result<Option<Group>, diesel::result::Error> {
    groups
        .select(Group::as_select())
        .filter(groups::id.eq(group_id))
        .first::<Group>(conn)
        .optional()
}

#[allow(dead_code)]
pub fn mark_group_as_primary(conn: &mut AnyConnection, group_id: i32) -> Result<usize, diesel::result::Error> {
    diesel::update(groups::table)
        .filter(groups::id.eq(group_id))
        .set(groups::is_primary.eq(true))
        .execute(conn)
}
#[allow(dead_code)]
pub fn mark_group_as_non_primary(conn: &mut AnyConnection) -> Result<usize, diesel::result::Error> {
    diesel::update(groups::table).set(groups::is_primary.eq(false)).execute(conn)
}

#[deprecated]
pub fn select_groups(
    conn: &mut AnyConnection,
    search_input: GroupFilter,
    limit: i64,
) -> Result<Vec<Group>, diesel::result::Error> {
    // 使用 into_boxed() 来对查询进行类型擦除
    let mut base_query = groups.limit(limit).select(Group::as_select()).into_boxed();

    // 如果 search_input 中有各字段，则添加相应的过滤条件
    if let Some(group_id) = search_input.id {
        base_query = base_query.filter(groups::id.eq(group_id));
    }
    if let Some(group_name) = search_input.name {
        base_query = base_query.filter(groups::name.eq(group_name));
    }
    if let Some(ref_count) = search_input.reference_count {
        base_query = base_query.filter(groups::reference_count.eq(ref_count));
    }
    if let Some(is_primary_val) = search_input.is_primary {
        base_query = base_query.filter(groups::is_primary.eq(is_primary_val));
    }
    if let Some(clicks) = search_input.click_count {
        base_query = base_query.filter(groups::click_count.eq(clicks));
    }
    if let Some(shares) = search_input.share_count {
        base_query = base_query.filter(groups::share_count.eq(shares));
    }
    if let Some(created) = search_input.create_time {
        base_query = base_query.filter(groups::create_time.eq(created));
    }
    if let Some(modified) = search_input.modify_time {
        base_query = base_query.filter(groups::modify_time.eq(modified));
    }

    // 执行查询
    base_query.load(conn)
}

pub fn delete_group(
    conn: &mut AnyConnection,
    group_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(groups.filter(groups::id.eq(group_id))).execute(conn)
}
use crate::model::schema::groups::dsl::*;
use crate::model::schema::groups;
use std::fmt::{Debug, Formatter, Result as fmtResult};

pub fn increase_group_reference_count(
    conn: &mut AnyConnection,
    group_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(groups::table.find(group_id))
        .set(groups::reference_count.eq(groups::reference_count + 1))
        .execute(conn)
}
pub fn decrease_group_reference_count(
    conn: &mut AnyConnection,
    group_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(groups::table.find(group_id))
        .set(groups::reference_count.eq(groups::reference_count - 1))
        .execute(conn)
}
impl Debug for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "Group {{ id: {}, name: {}, reference_count: {}, is_primary: {}, click_count: {}, share_count: {}, create_time: {}, modify_time: {} }}",
            self.id,
            self.name,
            self.reference_count,
            self.is_primary,
            self.click_count,
            self.share_count,
            self.create_time,
            self.modify_time
        )
    }
}

use super::models::GroupCondition;
use crate::model::models::{GroupOrderBy, GroupQueryOptions, OrderDirection, UpdateGroupDTO};
use diesel::dsl::not;
use diesel::sql_types::Bool;
use crate::utils::database::AnyConnection;

// 将 GroupCondition 转换为 diesel 查询条件的辅助函数
fn build_group_condition(condition: GroupCondition) -> Box<dyn BoxableExpression<groups::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>> {
    match condition {
        GroupCondition::Id(_id) => Box::new(groups::id.eq(_id)),
        GroupCondition::Name(_name) => Box::new(groups::name.eq(_name)),
        GroupCondition::ReferenceCount(count) => Box::new(groups::reference_count.eq(count)),
        GroupCondition::IsPrimary(_is_primary) => Box::new(groups::is_primary.eq(_is_primary)),
        GroupCondition::ClickCount(count) => Box::new(groups::click_count.eq(count)),
        GroupCondition::ShareCount(count) => Box::new(groups::share_count.eq(count)),
        GroupCondition::CreateTime(time) => Box::new(groups::create_time.eq(time)),
        GroupCondition::ModifyTime(time) => Box::new(groups::modify_time.eq(time)),

        GroupCondition::IdGreaterThan(value) => Box::new(groups::id.gt(value)),
        GroupCondition::IdLessThan(value) => Box::new(groups::id.lt(value)),
        GroupCondition::NameLike(pattern) => Box::new(groups::name.like(pattern)),
        GroupCondition::ReferenceCountGreaterThan(value) => Box::new(groups::reference_count.gt(value)),
        GroupCondition::ReferenceCountLessThan(value) => Box::new(groups::reference_count.lt(value)),
        GroupCondition::ClickCountGreaterThan(value) => Box::new(groups::click_count.gt(value)),
        GroupCondition::ClickCountLessThan(value) => Box::new(groups::click_count.lt(value)),
        GroupCondition::ShareCountGreaterThan(value) => Box::new(groups::share_count.gt(value)),
        GroupCondition::ShareCountLessThan(value) => Box::new(groups::share_count.lt(value)),
        GroupCondition::CreateTimeGreaterThan(time) => Box::new(groups::create_time.gt(time)),
        GroupCondition::CreateTimeLessThan(time) => Box::new(groups::create_time.lt(time)),
        GroupCondition::ModifyTimeGreaterThan(time) => Box::new(groups::modify_time.gt(time)),
        GroupCondition::ModifyTimeLessThan(time) => Box::new(groups::modify_time.lt(time)),
        
        GroupCondition::IdIn(values) => Box::new(groups::id.eq_any(values)),
        GroupCondition::NameIn(values) => Box::new(groups::name.eq_any(values)),
        GroupCondition::ReferenceCountIn(values) => Box::new(groups::reference_count.eq_any(values)),
        GroupCondition::ClickCountIn(values) => Box::new(groups::click_count.eq_any(values)),
        GroupCondition::ShareCountIn(values) => Box::new(groups::share_count.eq_any(values)),
        GroupCondition::CreateTimeIn(values) => Box::new(groups::create_time.eq_any(values)),
        GroupCondition::ModifyTimeIn(values) => Box::new(groups::modify_time.eq_any(values)),

        GroupCondition::And(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<groups::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>>> = None;
            for cond in conditions {
                let expr = build_group_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.and(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(true.into_sql::<Bool>()))
        }
        GroupCondition::Or(conditions) => {
            let mut result: Option<Box<dyn BoxableExpression<groups::table, <AnyConnection as Connection>::Backend, SqlType=diesel::sql_types::Bool>>> = None;
            for cond in conditions {
                let expr = build_group_condition(cond);
                match result {
                    None => result = Some(expr),
                    Some(prev) => result = Some(Box::new(prev.or(expr))),
                }
            }
            result.unwrap_or_else(|| Box::new(false.into_sql::<Bool>()))
        }
        GroupCondition::Not(condition) => {
            let expr = build_group_condition(*condition);
            Box::new(not(expr))
        }
    }
}

// 根据 GroupCondition 向量查询组
pub fn select_groups_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<GroupCondition>,
    limit: Option<i64>,
) -> Result<Vec<Group>, diesel::result::Error> {
    let mut query = groups::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    query
        .select(Group::as_select())
        .load(conn)
}


#[allow(dead_code)]
pub fn select_groups_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<GroupCondition>,
    options: GroupQueryOptions,
) -> Result<Vec<Group>, diesel::result::Error> {
    let mut query = groups::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用过滤条件
    for condition in conditions {
        let boxed_condition = build_group_condition(condition);
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
            GroupOrderBy::Id(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(groups::id.asc()),
                    OrderDirection::Desc => query.order(groups::id.desc()),
                }
            }
            GroupOrderBy::Name(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(groups::name.asc()),
                    OrderDirection::Desc => query.order(groups::name.desc()),
                }
            }
            GroupOrderBy::ReferenceCount(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(groups::reference_count.asc()),
                    OrderDirection::Desc => query.order(groups::reference_count.desc()),
                }
            }
            GroupOrderBy::IsPrimary(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(groups::is_primary.asc()),
                    OrderDirection::Desc => query.order(groups::is_primary.desc()),
                }
            }
            GroupOrderBy::ClickCount(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(groups::click_count.asc()),
                    OrderDirection::Desc => query.order(groups::click_count.desc()),
                }
            }
            GroupOrderBy::ShareCount(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(groups::share_count.asc()),
                    OrderDirection::Desc => query.order(groups::share_count.desc()),
                }
            }
            GroupOrderBy::CreateTime(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(groups::create_time.asc()),
                    OrderDirection::Desc => query.order(groups::create_time.desc()),
                }
            }
            GroupOrderBy::ModifyTime(direction) => {
                match direction {
                    OrderDirection::Asc => query.order(groups::modify_time.asc()),
                    OrderDirection::Desc => query.order(groups::modify_time.desc()),
                }
            }
        };
    }

    query.select(Group::as_select()).load(conn)
}


pub fn update_groups_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<GroupCondition>,
    update_set: UpdateGroupDTO,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::update(groups::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = crate::internal::groups::build_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.set(update_set).execute(conn)
}

pub fn delete_groups_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<GroupCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = groups::table.into_boxed::<<AnyConnection as Connection>::Backend>();

    // 对每个条件应用 AND 逻辑
    for condition in conditions {
        let boxed_condition = build_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    query.execute(conn)
}

pub fn increase_groups_reference_count_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<GroupCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::update(groups::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 增加引用计数
    query.set(groups::reference_count.eq(groups::reference_count + 1)).execute(conn)
}

pub fn decrease_groups_reference_count_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<GroupCondition>,
) -> Result<usize, diesel::result::Error> {
    let mut query = diesel::update(groups::table).into_boxed::<<AnyConnection as Connection>::Backend>();

    // 应用所有条件
    for condition in conditions {
        let boxed_condition = build_group_condition(condition);
        query = query.filter(boxed_condition);
    }

    // 减少引用计数
    query.set(groups::reference_count.eq(groups::reference_count - 1)).execute(conn)
}

pub fn select_group_by_file_id(
    conn: &mut AnyConnection,
    other_file_id: i32,
) -> Result<Vec<Group>, diesel::result::Error> {
    use crate::model::schema::file_groups;
    
    groups::table
        .inner_join(file_groups::table.on(groups::id.eq(file_groups::group_id)))
        .filter(file_groups::file_id.eq(other_file_id))
        .select(Group::as_select())
        .load(conn)
}

pub fn select_group_by_tag_id(
    conn: &mut AnyConnection,
    tag_id: i32,
) -> Result<Vec<Group>, diesel::result::Error> {
    use crate::model::schema::group_tags;
    
    groups::table
        .inner_join(group_tags::table.on(groups::id.eq(group_tags::group_id)))
        .filter(group_tags::tag_id.eq(tag_id))
        .select(Group::as_select())
        .load(conn)
}

pub fn get_group_by_id(
    conn: &mut AnyConnection,
    group_id: i32,
) -> Result<Group, diesel::result::Error> {
    groups::table
        .filter(groups::id.eq(group_id))
        .select(Group::as_select())
        .first(conn)
}