// tags.rs
//! 标签服务模块
//!
//! 提供标签相关的业务逻辑处理，包括标签的创建、删除、查询和更新操作，
//! 并处理标签与其关联分组等资源的引用计数和级联删除。

use crate::internal::group_tag as group_tag_dao;
use crate::internal::groups as groups_dao;
use crate::internal::tags as tags_dao;
use crate::model::models::{CreateTagDTO, Tag, TagFilter};
use crate::model::models::{GroupTagDTO, TagCondition, TagQueryOptions, UpdateTagDTO};
use crate::utils::database::AnyConnection;
use diesel::Connection;

/// 通过名称创建标签
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `name`: 标签名称
///
/// 返回值:
/// 成功时返回插入记录的ID，失败则返回相应的错误
pub fn create_tag_by_name<S>(
	conn: &mut AnyConnection,
	name: S,
) -> Result<i32, diesel::result::Error>
where
	S: Into<String>,
{
	let new_tag = CreateTagDTO { name: name.into() };
	tags_dao::insert_tag(conn, &new_tag)
}

/// 创建标签
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `create_tag_dto`: 包含标签信息的DTO对象
///
/// 返回值:
/// 成功时返回插入记录的ID，失败则返回相应的错误
pub fn create_tag(
	conn: &mut AnyConnection,
	create_tag_dto: &CreateTagDTO,
) -> Result<i32, diesel::result::Error> {
	tags_dao::insert_tag(conn, create_tag_dto)
}

/// 删除标签（级联删除相关资源）
///
/// 该函数负责删除标签并级联删除相关资源：
/// 1. 减少所有关联分组的引用计数
/// 2. 删除标签与分组的关联关系
/// 3. 删除标签本身
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_id`: 要删除的标签ID
///
/// 返回值:
/// 成功时返回删除的记录数，失败时返回数据库错误
pub fn delete_tag(conn: &mut AnyConnection, tag_id: i32) -> Result<usize, diesel::result::Error> {
	// 使用事务确保数据一致性
	conn.transaction::<usize, diesel::result::Error, _>(|conn| {
		// 查找与该标签关联的所有组
		let groups_associated_with_tag = groups_dao::select_groups_by_tag_id(conn, tag_id)?;

		for group in &groups_associated_with_tag {
			// 对于每个关联的组，减少其引用计数
			groups_dao::decrease_group_reference_count_by_id(conn, group.id)?;
			// 删除与该标签关联的所有组标签关系
			group_tag_dao::delete_group_tag_by_dto(conn, &GroupTagDTO { tag_id, group_id: group.id })?;
		}

		// 删除标签本身
		tags_dao::delete_tag_by_id(conn, tag_id)
	})
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
	tags_dao::select_tags_by_filter(conn, search_input, limit)
}

/// 根据过滤条件和选项查询标签列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 标签过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的标签记录列表或数据库错误
pub fn select_tags_by_filter_with_options(
	conn: &mut AnyConnection,
	search_input: TagFilter,
	options: TagQueryOptions,
) -> Result<Vec<Tag>, diesel::result::Error> {
	// 构造查询条件
	let mut conditions = Vec::new();
	
	if let Some(id) = search_input.id {
		conditions.push(TagCondition::Id(id));
	}
	if let Some(name) = search_input.name {
		conditions.push(TagCondition::Name(name));
	}
	if let Some(reference_count) = search_input.reference_count {
		conditions.push(TagCondition::ReferenceCount(reference_count));
	}
	
	tags_dao::select_tags_by_conditions_with_options(conn, conditions, options)
}

/// 根据条件查询标签列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 查询条件向量
/// - `limit`: 返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的标签记录列表或数据库错误
pub fn select_tags_by_conditions(
	conn: &mut AnyConnection,
	condition: Vec<TagCondition>,
	limit: Option<i64>,
) -> Result<Vec<Tag>, diesel::result::Error> {
	tags_dao::select_tags_by_conditions(conn, condition, limit)
}

/// 根据条件和选项查询标签列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的标签记录列表或数据库错误
pub fn select_tags_by_conditions_with_options(
	conn: &mut AnyConnection,
	conditions: Vec<TagCondition>,
	options: TagQueryOptions,
) -> Result<Vec<Tag>, diesel::result::Error> {
	tags_dao::select_tags_by_conditions_with_options(conn, conditions, options)
}

/// 根据条件批量更新标签
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 更新条件向量
/// - `update_set`: 更新内容DTO
///
/// 返回值:
/// 成功更新的记录数或数据库错误
pub fn update_tags_by_conditions(
	conn: &mut AnyConnection,
	conditions: Vec<TagCondition>,
	update_set: UpdateTagDTO,
) -> Result<usize, diesel::result::Error> {
	tags_dao::update_tags_by_conditions(conn, conditions, update_set)
}

/// 根据条件批量删除标签（级联删除相关资源）
///
/// 注意：这个方法在core里不应该有直接用法
/// 要暴露给用户使用的话应当改为先select再delete_by_id，防止引用计算问题
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 删除条件向量
///
/// 返回值:
/// 成功删除的记录数或数据库错误
///
/// 操作流程:
/// 对于每个要删除的标签，直接调用delete_tag函数执行删除操作
pub fn delete_tags_by_conditions(
	conn: &mut AnyConnection,
	conditions: Vec<TagCondition>,
) -> Result<usize, diesel::result::Error> {
	// 首先查询将要删除的标签
	let tags_to_delete =
		select_tags_by_conditions(conn, conditions.clone(), None).map_err(|e| match e {
			diesel::result::Error::NotFound => diesel::result::Error::NotFound,
			_ => e,
		})?;

	// 使用事务确保数据一致性
	conn.transaction::<_, diesel::result::Error, _>(|conn| {
		let mut total_deleted = 0;

		// 对于每个要删除的标签，直接调用delete_tag函数
		for tag in &tags_to_delete {
			// 调用单个标签删除函数，复用其业务逻辑
			let deleted_count = delete_tag(conn, tag.id)?;
			total_deleted += deleted_count;
		}

		Ok(total_deleted)
	})
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
	tags_dao::select_tag_by_group_id(conn, group_id)
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
	tags_dao::get_tag_by_id(conn, tag_id)
}

/// 根据标签ID更新标签信息
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_id`: 标签ID
/// - `update_set`: 更新内容DTO
///
/// 返回值:
/// 成功时返回影响的行数，失败时返回数据库错误
pub fn update_tag_by_id(
	conn: &mut AnyConnection,
	tag_id: i32,
	update_set: UpdateTagDTO,
) -> Result<usize, diesel::result::Error> {
	tags_dao::update_tag_by_id(conn, tag_id, update_set)
}

/// 根据标签ID列表批量删除标签
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_ids`: 要删除的标签ID列表
///
/// 返回值:
/// 成功删除的记录数或数据库错误
pub fn delete_tags_by_ids(
	conn: &mut AnyConnection,
	tag_ids: Vec<i32>,
) -> Result<usize, diesel::result::Error> {
	let mut total_deleted = 0;
	
	// 使用事务确保数据一致性
	conn.transaction::<_, diesel::result::Error, _>(|conn| {
		for &tag_id in &tag_ids {
			// 调用单个标签删除函数，复用其业务逻辑
			let deleted_count = delete_tag(conn, tag_id)?;
			total_deleted += deleted_count;
		}
		Ok(total_deleted)
	})
}
