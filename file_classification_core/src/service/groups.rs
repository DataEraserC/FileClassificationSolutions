use super::database::SqliteConnection;
use crate::service::AppError;
use crate::{
    internal::groups,
    model::models::{Group, GroupFilter, CreateGroupDTO},
};
use crate::model::models::{GroupCondition, UpdateGroupDTO};

pub fn create_group(conn: &mut SqliteConnection, name: &str) -> Result<Group, AppError> {
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
) -> Result<(), diesel::result::Error> {
    groups::delete_group(conn, group_id)
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
) -> Result<usize, AppError> {
    groups::update_groups_by_conditions(conn, conditions, update_set)
}

