// group_tag.rs
//! 组与标签关联管理模块
//!
//! 提供对组-标签关系表 (`group_tags`) 的增删查操作支持。

use super::models::GroupTagCondition;
use super::models::GroupTagDTO;
use crate::model::models::{GroupTagOrderBy, GroupTagQueryOptions, OrderDirection};
use crate::model::schema::group_tags;
use crate::utils::database::AnyConnection;
use diesel::dsl::not;
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
		GroupTagCondition::GroupId(id) => Box::new(group_tags::group_id.eq(id)),
		GroupTagCondition::TagId(id) => Box::new(group_tags::tag_id.eq(id)),

		GroupTagCondition::GroupIdGreaterThan(value) => Box::new(group_tags::group_id.gt(value)),
		GroupTagCondition::GroupIdLessThan(value) => Box::new(group_tags::group_id.lt(value)),
		GroupTagCondition::TagIdGreaterThan(value) => Box::new(group_tags::tag_id.gt(value)),
		GroupTagCondition::TagIdLessThan(value) => Box::new(group_tags::tag_id.lt(value)),

		GroupTagCondition::GroupIdIn(values) => Box::new(group_tags::group_id.eq_any(values)),
		GroupTagCondition::TagIdIn(values) => Box::new(group_tags::tag_id.eq_any(values)),

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
			Box::new(not(expr))
		}
	}
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
pub fn select_group_tags_by_conditions(
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
