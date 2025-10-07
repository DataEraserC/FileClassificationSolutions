// groups.rs
//! 分组服务模块
//!
//! 提供分组相关的业务逻辑处理，包括分组的创建、删除、查询和更新操作，
//! 并处理分组与其关联文件、标签等资源的引用计数和级联删除。

use crate::model::models::{FileCondition, FileGroupCondition, GroupCondition, GroupQueryOptions, GroupTagCondition, UpdateGroupDTO};
use crate::service::AppError;
use crate::{internal::groups, model::models::{CreateGroupDTO, Group, GroupFilter}};
use diesel::result::Error;
use diesel::{Connection};
use crate::utils::database::AnyConnection;

/// 通过名称创建分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `name`: 分组名称
///
/// 返回值:
/// 成功时返回影响的行数，失败时返回数据库错误
pub fn create_group_by_name<S>(conn: &mut AnyConnection, name: S) -> Result<usize, Error>
where
    S: Into<String>,
{
    let new_group = CreateGroupDTO { name: name.into() };
    groups::create_group(conn, &new_group)
}

/// 创建分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `create_group_dto`: 包含分组信息的DTO对象
///
/// 返回值:
/// 成功时返回影响的行数，失败时返回数据库错误
pub fn create_group(conn: &mut AnyConnection, create_group_dto: &CreateGroupDTO) -> Result<usize, Error> {
    groups::create_group(conn, create_group_dto)
}

/// 根据名称查找分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `name`: 要查找的分组名称
///
/// 返回值:
/// 成功时返回匹配的分组记录（如果存在），失败时返回相应的错误
pub fn find_group_by_name(
    conn: &mut AnyConnection,
    name: &str,
) -> Result<Option<Group>, AppError> {
    Ok(groups::find_group_by_name(conn, name)?)
}

/// 删除分组（级联删除相关资源）
///
/// 该函数负责删除分组并级联删除相关资源，根据分组是否为主分组采取不同策略：
/// 1. 主分组：删除关联的文件、文件组关系和标签关系
/// 2. 非主分组：减少关联文件的引用计数，删除文件组关系和标签关系
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `group_id`: 要删除的分组ID
///
/// 返回值:
/// 成功时返回删除的记录数，失败时返回数据库错误
pub fn delete_group(
    conn: &mut AnyConnection,
    group_id: i32,
) -> Result<usize, Error> {
    // 1.判断是否是primary
    // 若是primary 则先删除对应的File，删除GroupTag，删除FileGroup
    // 若不是primary 则先删除GroupTag，再删除FileGroup
    conn.transaction::<usize, Error, _>(|conn| {
        let group = groups::find_group_by_id(conn, group_id)?.ok_or(AppError::GroupNotFound)?;

        if group.is_primary {
            // 删除主组时，先处理关联的文件
            crate::internal::files::delete_files_by_conditions(conn, vec![
                FileCondition::GroupId(group_id)
            ])?;

            // 显式删除文件组关系（作为额外保障）
            crate::internal::file_group::delete_file_groups_by_conditions(conn, vec![
                FileGroupCondition::GroupId(group_id)
            ])?;
        } else {
            // 对于非主组，需要先减少关联文件的引用计数
            let file_groups = crate::internal::file_group::select_file_groups_by_conditions(
                conn,
                vec![FileGroupCondition::GroupId(group_id)],
                None,
            )?;

            // 减少每个关联文件的引用计数
            for file_group in &file_groups {
                crate::internal::files::decrease_file_reference_count(conn, file_group.file_id)?;
            }

            // 删除文件组关系
            crate::internal::file_group::delete_file_groups_by_conditions(conn, vec![
                FileGroupCondition::GroupId(group_id)
            ])?;
        }

        // 减少组关联标签的引用计数
        let group_tags = crate::internal::group_tag::select_group_tags_by_conditions(
            conn,
            vec![GroupTagCondition::GroupId(group_id)],
            None,
        )?;

        // 减少每个关联标签的引用计数
        for group_tag in &group_tags {
            crate::internal::tags::decrease_tag_reference_count(conn, group_tag.tag_id)?;
        }

        // 删除组标签关系
        crate::internal::group_tag::delete_group_tags_by_conditions(conn, vec![
            GroupTagCondition::GroupId(group_id)
        ])?;

        // 最后删除组本身
        groups::delete_group(conn, group_id)
    })
}

/// [已弃用] 根据过滤条件查询分组列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 分组过滤条件
/// - `limit`: 最大返回记录数
///
/// 返回值:
/// 查询成功的分组记录列表或数据库错误
#[allow(deprecated)]
#[deprecated]
pub fn select_groups(
    conn: &mut AnyConnection,
    search_input: GroupFilter,
    limit: i64,
) -> Result<Vec<Group>, diesel::result::Error> {
    groups::select_groups(conn, search_input, limit)
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
    groups::select_groups_by_conditions(conn, condition, limit)
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
    groups::select_groups_by_conditions_with_options(conn, conditions, options)
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
    groups::update_groups_by_conditions(conn, conditions, update_set)
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
/// 3. 删除文件组关联关系
/// 4. 减少关联标签的引用计数
/// 5. 删除组标签关联关系
/// 6. 删除分组本身
pub fn delete_groups_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<GroupCondition>,
) -> Result<usize, Error> {
    // 首先查询将要删除的组
    let groups_to_delete = select_groups_by_conditions(conn, conditions.clone(), None)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => diesel::result::Error::NotFound,
            _ => e,
        })?;

    // 使用事务确保数据一致性
    conn.transaction::<_, Error, _>(|conn| {
        let mut total_deleted = 0;

        // 对于每个要删除的组，处理相关的引用关系和关联数据
        for group in &groups_to_delete {
            // 如果是主组，则删除相关的文件
            if group.is_primary {
                crate::internal::files::delete_files_by_conditions(
                    conn,
                    vec![FileCondition::GroupId(group.id)]
                )?;

                // 显式删除文件组关系（作为额外保障）
                crate::internal::file_group::delete_file_groups_by_conditions(
                    conn,
                    vec![FileGroupCondition::GroupId(group.id)]
                )?;
            } else {
                // 如果不是主组，则先减少关联文件的引用计数
                let file_groups = crate::internal::file_group::select_file_groups_by_conditions(
                    conn,
                    vec![FileGroupCondition::GroupId(group.id)],
                    None,
                )?;

                // 减少每个关联文件的引用计数
                for file_group in &file_groups {
                    crate::internal::files::decrease_file_reference_count(conn, file_group.file_id)?;
                }

                // 删除文件组关联
                crate::internal::file_group::delete_file_groups_by_conditions(
                    conn,
                    vec![FileGroupCondition::GroupId(group.id)]
                )?;
            }

            // 减少组关联标签的引用计数
            let group_tags = crate::internal::group_tag::select_group_tags_by_conditions(
                conn,
                vec![GroupTagCondition::GroupId(group.id)],
                None,
            )?;

            // 减少每个关联标签的引用计数
            for group_tag in &group_tags {
                crate::internal::tags::decrease_tag_reference_count(conn, group_tag.tag_id)?;
            }

            // 删除组标签关联
            crate::internal::group_tag::delete_group_tags_by_conditions(
                conn,
                vec![GroupTagCondition::GroupId(group.id)]
            )?;

            // 删除组本身
            let deleted_count = crate::internal::groups::delete_group(conn, group.id)?;
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
    crate::internal::groups::select_group_by_file_id(conn, other_file_id)
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
    crate::internal::groups::select_group_by_tag_id(conn, tag_id)
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
    groups::get_group_by_id(conn, group_id)
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
    groups::update_group_by_id(conn, group_id, update_set)
}
