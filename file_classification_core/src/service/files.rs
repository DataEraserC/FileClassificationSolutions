// files.rs
//! 文件服务模块
//!
//! 提供文件相关的业务逻辑处理，包括文件的创建、删除、查询和更新操作，
//! 并处理文件与其关联分组、标签等资源的引用计数和级联删除。

use crate::internal::file_group::select_file_groups_by_conditions;
use crate::internal::groups::select_groups_by_conditions;
use crate::model::models::{CreateFileDTO, File, FileCondition, FileFilter, FileGroupCondition, FileGroupDTO, FileQueryOptions, GroupCondition, GroupTagCondition, TagCondition, UpdateFileDTO, UpdateGroupDTO};
use crate::service::groups::update_groups_by_conditions;
use crate::service::AppError;
use crate::utils::errors::AppError::{FuturePrimaryGroupShouldBeEmpty};
use crate::{internal, service};
use diesel::Connection;
use crate::utils::database::AnyConnection;

/// 创建文件的底层函数
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `type_`: 文件类型
/// - `path_`: 文件路径
/// - `group_id`: 分组ID
///
/// 返回值:
/// 成功时返回影响的行数，失败时返回数据库错误
pub fn raw_create_file<S1, S2>(
    conn: &mut AnyConnection,
    type_: S1,
    path_: S2,
    group_id: i32,
) -> Result<usize, diesel::result::Error>
where
    S1: Into<String>,
    S2: Into<String>,
{
    let new_file = CreateFileDTO {
        type_: type_.into(),
        path: path_.into(),
        group_id
    };
    internal::files::create_file(conn, &new_file)
}

/// 创建文件（业务逻辑处理）
///
/// 该函数负责创建文件并处理相关业务逻辑，包括：
/// 1. 验证目标分组是否存在且为空（作为主分组）
/// 2. 创建文件记录
/// 3. 建立文件与主分组的关联关系
/// 4. 将目标分组标记为主分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `create_file_dto`: 包含文件信息的DTO对象
///
/// 返回值:
/// 成功时返回创建的记录数，失败时返回相应的错误
pub fn create_file(conn: &mut AnyConnection, create_file_dto: CreateFileDTO) -> Result<usize, AppError> {
    conn.transaction::<usize, AppError, _>(|conn| {
        // 验证目标分组是否存在
        let group_list = select_groups_by_conditions(conn, vec![
            GroupCondition::Id(create_file_dto.group_id)
        ], None)?;
        let Some(group) = group_list.get(0) else { todo!() };
        
        // 检查目标分组是否为空（作为主分组必须为空）
        let file_groups = select_file_groups_by_conditions(
            conn,
            vec![
                FileGroupCondition::GroupId(create_file_dto.group_id)
            ], None,
        )?;

        if file_groups.len() != 0 {
            return Err(FuturePrimaryGroupShouldBeEmpty);
        }
        
        let mut count = 0;
        // 创建文件记录
        count += internal::files::create_file(conn, &create_file_dto)?;
        
        // 获取刚创建的文件
        let file_list = internal::files::select_files_by_conditions(conn, vec![
            FileCondition::GroupId(create_file_dto.group_id)
        ], None)?;
        let file = file_list.get(0).ok_or(AppError::FileNotFound)?;

        // 建立文件与主分组的关联关系
        match service::file_group::create_file_group(conn, FileGroupDTO { file_id: file.id, group_id: group.id }) {
            Ok(_) => {
                count += 1;
            }
            Err(e) => {
                return Err(e)
            }
        }

        // 将目标分组标记为主分组
        update_groups_by_conditions(conn, vec![
            GroupCondition::Id(group.id)
        ], UpdateGroupDTO {
            id: None,
            name: None,
            reference_count: None,
            is_primary: Some(true),
            click_count: None,
            share_count: None,
            create_time: None,
            modify_time: None,
        }).expect("Error when updating group");
        Ok(count)
    })
}

/// 删除文件（级联删除相关资源）
///
/// 该函数负责删除文件并级联删除相关资源，包括：
/// 1. 删除文件主分组关联的所有标签关系
/// 2. 删除文件关联的所有分组关系
/// 3. 删除文件的主分组
/// 4. 删除文件本身
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_id`: 要删除的文件ID
///
/// 返回值:
/// 成功时返回空元组，失败时返回相应的错误
pub fn delete_file(conn: &mut AnyConnection, file_id: i32) -> Result<(), AppError> {
    // 开始事务
    conn.transaction::<(), diesel::result::Error, _>(|conn| {
        // 0. 删除对应的PrimaryGroup对应的GroupTag
        // 1. 删除对应的PrimaryGroup
        // 2. 删除对应的剩余FileGroup
        let file_required_to_delete = internal::files::find_file_by_id(conn, file_id)?
            .ok_or_else(|| diesel::result::Error::NotFound)?;

        // 查找与该文件关联的所有文件组关系（包括主组和其他组）
        let file_groups = select_file_groups_by_conditions(
            conn,
            vec![FileGroupCondition::FileId(file_required_to_delete.id)],
            None,
        )?;

        // 对于每个文件组关系，减少对应组的引用计数
        for file_group in &file_groups {
            internal::groups::decrease_group_reference_count(conn, file_group.group_id)?;
        }

        // 仅对主组关联的标签减少引用计数
        let tag_list = internal::group_tag::select_group_tags_by_conditions(
            conn,
            vec![GroupTagCondition::GroupId(file_required_to_delete.group_id)],
            None,
        )?;

        if !tag_list.is_empty() {
            internal::tags::decrease_tags_reference_count_by_conditions(
                conn,
                vec![TagCondition::IdIn(tag_list.iter().map(|tag| tag.tag_id).collect::<Vec<_>>())],
            )?;
        }

        // 删除与文件主组关联的所有组标签关系
        internal::group_tag::delete_group_tags_by_conditions(
            conn,
            vec![GroupTagCondition::GroupId(file_required_to_delete.group_id)],
        )?;

        // 删除与文件关联的所有文件组关系
        internal::file_group::delete_file_groups_by_conditions(
            conn,
            vec![FileGroupCondition::FileId(file_required_to_delete.id)],
        )?;

        // 删除主组和文件本身
        internal::groups::delete_group(conn, file_required_to_delete.group_id)?;
        internal::files::delete_file_by_id(conn, file_id)?;

        Ok(())
    })?;
    Ok(())
}

/// [已弃用] 根据过滤条件查询文件列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `search_input`: 文件过滤条件
/// - `limit`: 最大返回记录数
///
/// 返回值:
/// 查询成功的文件记录列表或数据库错误
#[allow(deprecated)]
#[deprecated]
pub fn select_files(
    conn: &mut AnyConnection,
    search_input: FileFilter,
    limit: i64,
) -> Result<Vec<File>, diesel::result::Error> {
    internal::files::select_files(conn, search_input, limit)
}

/// 根据条件查询文件列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 查询条件向量
/// - `limit`: 返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的文件记录列表或数据库错误
pub fn select_files_by_conditions(
    conn: &mut AnyConnection,
    condition: Vec<FileCondition>,
    limit: Option<i64>,
) -> Result<Vec<File>, diesel::result::Error> {
    internal::files::select_files_by_conditions(conn, condition, limit)
}

/// 根据条件和选项查询文件列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的文件记录列表或数据库错误
pub fn select_files_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<FileCondition>,
    options: FileQueryOptions,
) -> Result<Vec<File>, diesel::result::Error> {
    internal::files::select_files_by_conditions_with_options(conn, conditions, options)
}

/// 根据条件批量更新文件
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 更新条件向量
/// - `update_set`: 更新内容DTO
///
/// 返回值:
/// 成功更新的记录数或数据库错误
pub fn update_files_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileCondition>,
    update_set: UpdateFileDTO,
) -> Result<usize, diesel::result::Error> {
    internal::files::update_files_by_conditions(conn, conditions, update_set)
}

/// 根据条件批量删除文件（级联删除相关资源）
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
/// 对于每个要删除的文件，执行以下操作：
/// 1. 减少所有关联分组的引用计数
/// 2. 减少主分组关联标签的引用计数
/// 3. 删除主分组的标签关联关系
/// 4. 删除文件的所有分组关联关系
/// 5. 删除文件的主分组
/// 6. 删除文件本身
pub fn delete_files_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<FileCondition>,
) -> Result<usize, diesel::result::Error> {
    // 首先查询将要删除的文件
    let files_to_delete = select_files_by_conditions(conn, conditions.clone(), None)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => diesel::result::Error::NotFound,
            _ => e,
        })?;

    // 使用事务确保数据一致性
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        let mut total_deleted = 0;

        // 对于每个要删除的文件，处理相关的引用关系
        for file in &files_to_delete {
            // 查找与该文件关联的所有文件组关系（包括主组和其他组）
            let file_groups = select_file_groups_by_conditions(
                conn,
                vec![FileGroupCondition::FileId(file.id)],
                None,
            )?;

            // 对于每个文件组关系，减少对应组的引用计数
            for file_group in &file_groups {
                internal::groups::decrease_group_reference_count(conn, file_group.group_id)?;
            }

            // 仅对主组关联的标签减少引用计数
            let tag_list = internal::group_tag::select_group_tags_by_conditions(
                conn,
                vec![GroupTagCondition::GroupId(file.group_id)],
                None,
            )?;

            if !tag_list.is_empty() {
                internal::tags::decrease_tags_reference_count_by_conditions(
                    conn,
                    vec![TagCondition::IdIn(tag_list.iter().map(|tag| tag.tag_id).collect::<Vec<_>>())],
                )?;
            }

            // 删除与文件主组关联的所有组标签关系
            internal::group_tag::delete_group_tags_by_conditions(
                conn,
                vec![GroupTagCondition::GroupId(file.group_id)],
            )?;

            // 删除与该文件关联的所有文件组关系
            internal::file_group::delete_file_groups_by_conditions(
                conn,
                vec![FileGroupCondition::FileId(file.id)],
            )?;

            // 删除主组和文件本身
            internal::groups::delete_group(conn, file.group_id)?;
            let deleted_count = internal::files::delete_file_by_id(conn, file.id)?;
            total_deleted += deleted_count;
        }

        Ok(total_deleted)
    })
}

/// 根据分组ID查询关联的文件列表
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `other_group_id`: 分组ID
///
/// 返回值:
/// 查询成功的文件记录列表或数据库错误
pub fn select_file_by_group_id(
    conn: &mut AnyConnection,
    other_group_id: i64,
) -> Result<Vec<File>, diesel::result::Error> {
    internal::files::select_file_by_group_id(conn, other_group_id)
}

/// 根据文件ID获取文件详情
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_id`: 文件ID
///
/// 返回值:
/// 查询成功的文件记录或数据库错误
pub fn get_file_by_id(
    conn: &mut AnyConnection,
    file_id: i32,
) -> Result<File, diesel::result::Error> {
    internal::files::get_file_by_id(conn, file_id)
}

/// 根据文件ID更新文件信息
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_id`: 文件ID
/// - `update_set`: 更新内容DTO
///
/// 返回值:
/// 成功时返回影响的行数，失败时返回数据库错误
pub fn update_file_by_id(
    conn: &mut AnyConnection,
    file_id: i32,
    update_set: UpdateFileDTO,
) -> Result<usize, diesel::result::Error> {
    internal::files::update_file_by_id(conn, file_id, update_set)
}
