use super::database::SqliteConnection;
use crate::model::models::{FileCondition, FileGroupCondition, GroupCondition, GroupTagCondition, UpdateGroupDTO};
use crate::service::AppError;
use crate::{internal::groups, model::models::{CreateGroupDTO, Group, GroupFilter}};
use diesel::result::Error;
use diesel::Connection;

pub fn create_group(conn: &mut SqliteConnection, name: &str) -> Result<usize, Error> {
    let new_group = CreateGroupDTO { name };
    groups::create_group(conn, &new_group)
}

pub fn find_group_by_name(
    conn: &mut SqliteConnection,
    name: &str,
) -> Result<Option<Group>, AppError> {
    Ok(groups::find_group_by_name(conn, name)?)
}

pub fn delete_group(
    conn: &mut SqliteConnection,
    group_id: i32,
) -> Result<usize, Error> {
    // 1.判断是否是primary
    // 若是primary 则直接删除对应的File，删除GroupTag
    // 若不是primary 则先删除GroupTag，再删除FileGroup
    conn.transaction::<usize, Error, _>(|conn| {
        let group = groups::find_group_by_id(conn, group_id)?.ok_or(AppError::GroupNotFound)?;
        if group.is_primary {
            crate::internal::files::delete_files_by_conditions(conn, vec![
                FileCondition::GroupId(group_id)
            ])?;
        } else {
            crate::internal::file_group::delete_file_groups_by_conditions(conn, vec![
                FileGroupCondition::GroupId(group_id)
            ])?;
        }
        crate::internal::group_tag::delete_group_tags_by_conditions(conn, vec![
            GroupTagCondition::GroupId(group_id)
        ])?;
        groups::delete_group(conn, group_id)
    })
}
#[allow(deprecated)]
#[deprecated]
pub fn select_groups(
    conn: &mut SqliteConnection,
    search_input: GroupFilter,
    limit: i64,
) -> Result<Vec<Group>, diesel::result::Error> {
    groups::select_groups(conn, search_input, limit)
}

pub fn select_groups_by_conditions(
    conn: &mut SqliteConnection,
    condition: Vec<GroupCondition>,
    limit: Option<i64>,
) -> Result<Vec<Group>, diesel::result::Error> {
    groups::select_groups_by_conditions(conn, condition, limit)
}

pub fn update_groups_by_conditions(
    conn: &mut SqliteConnection,
    conditions: Vec<GroupCondition>,
    update_set: UpdateGroupDTO,
) -> Result<usize, Error> {
    groups::update_groups_by_conditions(conn, conditions, update_set)
}


// NOTE: 这个方法在core里不应该有用法
// 要暴露给用户使用的话 应当改为先select再delete_by_id
// 防止引用计算问题
pub fn delete_groups_by_conditions(
    conn: &mut SqliteConnection,
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
            } else {
                // 如果不是主组，则删除文件组关联
                crate::internal::file_group::delete_file_groups_by_conditions(
                    conn,
                    vec![FileGroupCondition::GroupId(group.id)]
                )?;
            }

            // 删除组标签关联
            crate::internal::group_tag::delete_group_tags_by_conditions(
                conn,
                vec![GroupTagCondition::GroupId(group.id)]
            )?;

            // 删除组本身
            let deleted_count = groups::delete_group(conn, group.id)?;
            total_deleted += deleted_count;
        }

        Ok(total_deleted)
    })
}

