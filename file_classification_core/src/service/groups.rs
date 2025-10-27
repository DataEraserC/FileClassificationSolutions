// groups.rs
//! 分组服务模块
//!
//! 提供分组相关的业务逻辑处理，包括分组的创建、删除、查询和更新操作，
//! 并处理分组与其关联文件、标签等资源的引用计数和级联删除。

use crate::internal::file_group as file_group_dao;
use crate::internal::files as files_dao;
use crate::internal::group_tag as group_tag_dao;
use crate::internal::groups as groups_dao;
use crate::internal::tags as tags_dao;
use crate::model::models::{FileGroupCondition, FileGroupDTO, Group, GroupCondition, GroupFilter, GroupQueryOptions, GroupTagCondition, GroupTreeNode, UpdateGroupDTO, PaginationResult, CreateGroupDTO};
use crate::service::group_relations as group_relations_service;
use crate::service::AppError;
use crate::utils::database::AnyConnection;
use diesel::result::Error;
use diesel::Connection;
/// 通过名称创建分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `name`: 分组名称
///
/// 返回值:
/// 成功时返回插入记录的ID，失败则返回相应的错误
pub fn create_group_by_name<S>(conn: &mut AnyConnection, name: S) -> Result<i32, Error>
where
	S: Into<String>,
{
	let new_group = CreateGroupDTO { name: name.into() };
	groups_dao::insert_group(conn, &new_group)
}

/// 创建分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `create_group_dto`: 包含分组信息的DTO对象
///
/// 返回值:
/// 成功时返回插入记录的ID，失败则返回相应的错误
pub fn create_group(
	conn: &mut AnyConnection,
	create_group_dto: &CreateGroupDTO,
) -> Result<i32, Error> {
	groups_dao::insert_group(conn, create_group_dto)
}

/// 根据名称查找分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `name`: 要查找的分组名称
///
/// 返回值:
/// 成功时返回匹配的分组记录（如果存在），失败时返回相应的错误
pub fn find_group_by_name(conn: &mut AnyConnection, name: &str) -> Result<Option<Group>, AppError> {
	Ok(groups_dao::find_group_by_name(conn, name)?)
}

/// 删除分组（级联删除相关资源）
///
/// 该函数负责删除分组并级联删除相关资源，根据分组是否为主分组采取不同策略：
/// 1. 主分组：减少组的关联文件的关联组、组的关联标签的引用计数，删除组关联的文件、关联文件的文件组关系、组标签关系
/// 2. 非主分组：减少关联文件、关联标签的引用计数，删除文件组关系、组标签关系
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 要删除的分组ID
///
/// 返回值:
/// 成功时返回删除的记录数，失败时返回数据库错误
pub fn delete_group(conn: &mut AnyConnection, group_id: i32) -> Result<usize, Error> {
	conn.transaction::<usize, Error, _>(|conn| {
		let group = groups_dao::find_group_by_id(conn, group_id)?.ok_or(AppError::GroupNotFound)?;

		// 获取关联的文件
		let files_associated_with_group = files_dao::select_files_by_group_id(conn, group_id)?;

		if group.is_primary {
			// 获取主组的（唯一）关联文件
			let file_required_operation =
				files_associated_with_group.get(0).ok_or(AppError::FileNotFound)?;

			// 删除主组时，先删除关联的文件 及 关联文件的文件组关系
			files_dao::delete_file_by_id(conn, file_required_operation.id)?;

			// 搜索关联文件的文件组关系
			let file_groups_required_operation = file_group_dao::select_file_groups_by_conditions(
				conn,
				vec![FileGroupCondition::FileId(file_required_operation.id)],
				None,
			)?;

			// 删除关联文件的文件组关系
			file_group_dao::delete_file_groups_by_dtos(conn, file_groups_required_operation)?;
		} else {
			for file in &files_associated_with_group {
				// 对于非主组，需要先减少关联文件的引用计数
				files_dao::decrease_file_reference_count_by_id(conn, file.id)?;
				// 删除文件组关系
				file_group_dao::delete_file_group_by_dto(
					conn,
					&FileGroupDTO { file_id: file.id, group_id, relation_type: 1 },
				)?;
			}
		}

		// 减少组关联标签的引用计数
		let group_tags = group_tag_dao::select_group_tags_by_conditions(
			conn,
			vec![GroupTagCondition::GroupId(group_id)],
			None,
		)?;

		for group_tag in &group_tags {
			// 减少每个关联标签的引用计数
			tags_dao::decrease_tag_reference_count_by_id(conn, group_tag.tag_id)?;

			// 删除组标签关系
			group_tag_dao::delete_group_tag_by_dto(conn, group_tag)?;
		}

		// 最后删除组本身
		groups_dao::delete_group_by_id(conn, group_id)
	})
}

/// 根据过滤条件查询分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 分组过滤条件
/// - `limit`: 最大返回记录数
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_groups_by_filter(
	conn: &mut AnyConnection,
	search_input: GroupFilter,
	limit: i64,
) -> Result<Vec<Group>, diesel::result::Error> {
	groups_dao::select_groups_by_filter(conn, search_input, limit)
}

/// 根据过滤条件和选项查询分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 分组过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_groups_by_filter_with_options(
	conn: &mut AnyConnection,
	search_input: GroupFilter,
	options: GroupQueryOptions,
) -> Result<Vec<Group>, diesel::result::Error> {
	// 构造查询条件
	let mut conditions = Vec::new();
	
	if let Some(id) = search_input.id {
		conditions.push(GroupCondition::Id(id));
	}
	if let Some(name) = search_input.name {
		conditions.push(GroupCondition::Name(name));
	}
	if let Some(reference_count) = search_input.reference_count {
		conditions.push(GroupCondition::ReferenceCount(reference_count));
	}
	if let Some(is_primary) = search_input.is_primary {
		conditions.push(GroupCondition::IsPrimary(is_primary));
	}
	if let Some(click_count) = search_input.click_count {
		conditions.push(GroupCondition::ClickCount(click_count));
	}
	if let Some(share_count) = search_input.share_count {
		conditions.push(GroupCondition::ShareCount(share_count));
	}
	if let Some(create_time) = search_input.create_time {
		conditions.push(GroupCondition::CreateTime(create_time));
	}
	if let Some(modify_time) = search_input.modify_time {
		conditions.push(GroupCondition::ModifyTime(modify_time));
	}
	
	groups_dao::select_groups_by_conditions_with_options(conn, conditions, options)
}

/// 根据过滤条件和选项查询分组列表（支持分页结果）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 分组过滤条件
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
pub fn select_groups_by_filter_with_pagination(
	conn: &mut AnyConnection,
	search_input: GroupFilter,
	options: GroupQueryOptions,
) -> Result<PaginationResult<Group>, diesel::result::Error> {
	// 构造查询条件
	let mut conditions = Vec::new();
	
	if let Some(id) = search_input.id {
		conditions.push(GroupCondition::Id(id));
	}
	if let Some(name) = search_input.name {
		conditions.push(GroupCondition::Name(name));
	}
	if let Some(reference_count) = search_input.reference_count {
		conditions.push(GroupCondition::ReferenceCount(reference_count));
	}
	if let Some(is_primary) = search_input.is_primary {
		conditions.push(GroupCondition::IsPrimary(is_primary));
	}
	if let Some(click_count) = search_input.click_count {
		conditions.push(GroupCondition::ClickCount(click_count));
	}
	if let Some(share_count) = search_input.share_count {
		conditions.push(GroupCondition::ShareCount(share_count));
	}
	if let Some(create_time) = search_input.create_time {
		conditions.push(GroupCondition::CreateTime(create_time));
	}
	if let Some(modify_time) = search_input.modify_time {
		conditions.push(GroupCondition::ModifyTime(modify_time));
	}
	
	select_groups_by_conditions_with_pagination(conn, conditions, options)
}

/// 根据条件查询分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 查询条件向量
/// - `limit`: 返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_groups_by_conditions(
	conn: &mut AnyConnection,
	condition: Vec<GroupCondition>,
	limit: Option<i64>,
) -> Result<Vec<Group>, diesel::result::Error> {
	groups_dao::select_groups_by_conditions(conn, condition, limit)
}

/// 根据条件和选项查询分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_groups_by_conditions_with_options(
	conn: &mut AnyConnection,
	conditions: Vec<GroupCondition>,
	options: GroupQueryOptions,
) -> Result<Vec<Group>, diesel::result::Error> {
	groups_dao::select_groups_by_conditions_with_options(conn, conditions, options)
}

/// 根据条件和选项查询分组列表（支持分页结果）
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的分页结果或数据库错误
pub fn select_groups_by_conditions_with_pagination(
	conn: &mut AnyConnection,
	conditions: Vec<GroupCondition>,
	options: GroupQueryOptions,
) -> Result<PaginationResult<Group>, diesel::result::Error> {
	groups_dao::select_groups_by_conditions_with_pagination(conn, conditions, options)
}

/// 根据条件批量更新分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 更新条件向量
/// - `update_set`: 更新内容DTO
///
/// 返回值:
/// 成功更新的记录数或数据库错误
pub fn update_groups_by_conditions(
	conn: &mut AnyConnection,
	conditions: Vec<GroupCondition>,
	update_set: UpdateGroupDTO,
) -> Result<usize, Error> {
	groups_dao::update_groups_by_conditions(conn, conditions, update_set)
}

/// 根据条件批量删除分组（级联删除相关资源）
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
/// 对于每个要删除的分组，执行以下操作：
/// 1. 如果是主分组，则删除关联的文件
/// 2. 如果是非主分组，则减少关联文件的引用计数
/// 3. 删除文件组关系
/// 4. 减少关联标签的引用计数
/// 5. 删除组标签关联关系
/// 6. 删除分组本身
pub fn delete_groups_by_conditions(
	conn: &mut AnyConnection,
	conditions: Vec<GroupCondition>,
) -> Result<usize, Error> {
	// 首先查询将要删除的组
	let groups_to_delete =
		select_groups_by_conditions(conn, conditions.clone(), None).map_err(|e| match e {
			diesel::result::Error::NotFound => diesel::result::Error::NotFound,
			_ => e,
		})?;

	// 使用事务确保数据一致性
	conn.transaction::<_, Error, _>(|conn| {
		let mut total_deleted = 0;

		// 对于每个要删除的组，直接调用delete_group函数
		for group in &groups_to_delete {
			let deleted_count = delete_group(conn, group.id)?;
			total_deleted += deleted_count;
		}

		Ok(total_deleted)
	})
}

/// 根据文件ID查询关联的分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `other_file_id`: 文件ID
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_group_by_file_id(
	conn: &mut AnyConnection,
	other_file_id: i32,
) -> Result<Vec<Group>, diesel::result::Error> {
	groups_dao::select_groups_by_file_id(conn, other_file_id)
}

/// 根据标签ID查询关联的分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `tag_id`: 标签ID
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
pub fn select_group_by_tag_id(
	conn: &mut AnyConnection,
	tag_id: i32,
) -> Result<Vec<Group>, diesel::result::Error> {
	groups_dao::select_groups_by_tag_id(conn, tag_id)
}

/// 根据分组ID获取分组详情
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 分组ID
///
/// 返回值:
/// 查询成功的分组记录或数据库错误
pub fn get_group_by_id(
	conn: &mut AnyConnection,
	group_id: i32,
) -> Result<Group, diesel::result::Error> {
	groups_dao::get_group_by_id(conn, group_id)
}

/// 根据分组ID更新分组信息
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 分组ID
/// - `update_set`: 更新内容DTO
///
/// 返回值:
/// 成功时返回影响的行数，失败时返回数据库错误
pub fn update_group_by_id(
	conn: &mut AnyConnection,
	group_id: i32,
	update_set: UpdateGroupDTO,
) -> Result<usize, diesel::result::Error> {
	groups_dao::update_group_by_id(conn, group_id, update_set)
}

/// 根据组ID获取组的树状结构
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 根节点组ID
///
/// 返回值:
/// 成功时返回组的树状结构，失败时返回错误
pub fn get_group_tree(conn: &mut AnyConnection, group_id: i32) -> Result<GroupTreeNode, AppError> {
	// 首先获取根节点组信息
	let group = groups_dao::find_group_by_id(conn, group_id)?.ok_or(AppError::GroupNotFound)?;

	// 获取直接子节点的ID列表
	let child_ids = group_relations_service::get_direct_children_ids(conn, group_id)?;

	let mut children = Vec::new();
	for child_id in child_ids {
		// 递归获取每个子节点的树状结构
		let child_tree = get_group_tree(conn, child_id)?;
		children.push(child_tree);
	}

	Ok(GroupTreeNode { group, children })
}

/// 根据分组ID列表批量删除分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_ids`: 要删除的分组ID列表
///
/// 返回值:
/// 成功删除的记录数或数据库错误
pub fn delete_groups_by_ids(
	conn: &mut AnyConnection,
	group_ids: Vec<i32>,
) -> Result<usize, Error> {
	let mut total_deleted = 0;
	
	// 使用事务确保数据一致性
	conn.transaction::<_, Error, _>(|conn| {
		for &group_id in &group_ids {
			// 调用单个分组删除函数，复用其业务逻辑
			let deleted_count = delete_group(conn, group_id)?;
			total_deleted += deleted_count;
		}
		Ok(total_deleted)
	})
}
