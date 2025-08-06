use diesel::Connection;
use diesel::result::Error;
use super::database::SqliteConnection;
use crate::service::{file_group, files, group_tag, AppError};
use crate::{internal::groups, model::models::{Group, GroupFilter, CreateGroupDTO}, service};
use crate::model::models::{FileCondition, FileGroupCondition, GroupCondition, GroupTagCondition, UpdateGroupDTO};

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
            files::delete_files_by_conditions(conn, vec![
                FileCondition::GroupId(group_id)
            ])?;
        } else {
            file_group::delete_file_groups_by_conditions(conn, vec![
                FileGroupCondition::GroupId(group_id)
            ])?;
        }
        group_tag::delete_group_tags_by_conditions(conn, vec![
            GroupTagCondition::GroupId(group_id)
        ])?;
        groups::delete_group(conn, group_id)
    })
}

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
    limit: i64,
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


pub fn delete_groups_by_conditions(
    conn: &mut SqliteConnection,
    conditions: Vec<GroupCondition>,
) -> Result<usize, Error> {
    groups::delete_groups_by_conditions(conn, conditions)
}

