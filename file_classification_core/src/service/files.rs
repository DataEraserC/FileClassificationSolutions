use super::database::SqliteConnection;
use crate::internal::file_group::select_file_groups_by_conditions;
use crate::internal::groups::select_groups_by_conditions;
use crate::model::models::{CreateFileDTO, File, FileCondition, FileFilter, FileGroupCondition, FileGroupDTO, GroupCondition, GroupTagCondition, UpdateFileDTO, UpdateGroupDTO};
use crate::service::groups::update_groups_by_conditions;
use crate::service::AppError;
use crate::utils::errors::AppError::CannotBindToPrimaryGroup;
use crate::{internal, service};
use diesel::Connection;

pub fn raw_create_file(
    conn: &mut SqliteConnection,
    type_: &str,
    path_: &str,
    group_id: i32,
) -> Result<usize, diesel::result::Error> {
    let new_file = CreateFileDTO { type_, path: path_, group_id };
    internal::files::create_file(conn, &new_file)
}
// pub fn create_file(
//     conn: &mut SqliteConnection,
//     name: &str,
//     type_: &str,
//     path_: &str,
// ) -> Result<(File, Group), diesel::result::Error> {
//     let mut is_primary = false;
//     let group = match find_group_by_name(conn, name)? {
//         Some(existing_group) => existing_group,
//         _ => {
//             let group = create_group(conn, name)?;
//             is_primary = true;
//             group
//         }
//     };
//
//     let file = raw_create_file(conn, type_, path_, group.id)?;
//
//     create_file_group(conn, FileGroupDTO{ file_id: file.id, group_id: group.id})?;
//     if is_primary {
//         mark_group_as_primary(conn, group.id)?;
//     }
//
//     Ok((file, group))
// }

pub fn create_file(conn: &mut SqliteConnection, create_file_dto: CreateFileDTO) -> Result<usize, AppError> {
    conn.transaction::<usize, AppError, _>(|conn| {
        let group_list = select_groups_by_conditions(conn, vec![
            GroupCondition::Id(create_file_dto.group_id)
        ], None)?;
        let Some(group) = group_list.get(0) else { todo!() };
        let file_groups = select_file_groups_by_conditions(
            conn,
            vec![
                FileGroupCondition::GroupId(create_file_dto.group_id)
            ], None,
        )?;

        if file_groups.len() != 0 {
            return Err(CannotBindToPrimaryGroup);
        }
        let mut count = 0;
        count += internal::files::create_file(conn, &create_file_dto)?;
        let file_list = internal::files::select_files_by_conditions(conn, vec![
            FileCondition::GroupId(create_file_dto.group_id)
        ], None)?;
        let file = file_list.get(0).ok_or(AppError::FileNotFound)?;

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
        }).expect("Error when creating group");

        match service::file_group::create_file_group(conn, FileGroupDTO { file_id: file.id, group_id: group.id }) {
            Ok(_) => {
                count += 1;
            }
            Err(e) => {
                return Err(e)
            }
        }
        Ok(count)
    })
}

pub fn delete_file(conn: &mut SqliteConnection, file_id: i32) -> Result<(), AppError> {
    // 开始事务
    conn.transaction::<(), diesel::result::Error, _>(|conn| {
        // 0. 删除对应的PrimaryGroup对应的GroupTag
        // 1. 删除对应的PrimaryGroup
        // 2. 删除对应的剩余FileGroup
        let file_required_to_delete = internal::files::find_file_by_id(conn, file_id)?
            .ok_or_else(|| diesel::result::Error::NotFound)?;

        // 删除与文件关联的组标签
        internal::group_tag::delete_group_tags_by_conditions(
            conn,
            vec![GroupTagCondition::GroupId(file_required_to_delete.group_id)],
        )?;

        // 删除与文件关联的文件组关系
        internal::file_group::delete_file_groups_by_conditions(
            conn,
            vec![FileGroupCondition::FileId(file_required_to_delete.id)],
        )?;

        // 删除组和文件本身
        service::groups::delete_group(conn, file_required_to_delete.group_id)?;
        internal::files::delete_file_by_id(conn, file_id)?;

        Ok(())
    })?;
    Ok(())
}


#[allow(deprecated)]
#[deprecated]
pub fn select_files(
    conn: &mut SqliteConnection,
    search_input: FileFilter,
    limit: i64,
) -> Result<Vec<File>, diesel::result::Error> {
    internal::files::select_files(conn, search_input, limit)
}

pub fn select_files_by_conditions(
    conn: &mut SqliteConnection,
    condition: Vec<FileCondition>,
    limit: Option<i64>,
) -> Result<Vec<File>, diesel::result::Error> {
    internal::files::select_files_by_conditions(conn, condition, limit)
}

pub fn update_files_by_conditions(
    conn: &mut SqliteConnection,
    conditions: Vec<FileCondition>,
    update_set: UpdateFileDTO,
) -> Result<usize, diesel::result::Error> {
    internal::files::update_files_by_conditions(conn, conditions, update_set)
}

// NOTE: 这个方法在core里不应该有用法
// 要暴露给用户使用的话 应当改为先select再delete_by_id
// 防止引用计算问题
pub fn delete_files_by_conditions(
    conn: &mut SqliteConnection,
    conditions: Vec<FileCondition>,
) -> Result<usize, diesel::result::Error> {
    // TODO: 减少文件的引用计数
    internal::files::delete_files_by_conditions(conn, conditions)
}

