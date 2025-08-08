use diesel::Connection;
use diesel::result::Error;
use super::database::SqliteConnection;
use crate::internal::{group_tag as group_tags, group_tag};
use crate::internal::groups::{decrease_group_reference_count, find_group_by_id, increase_group_reference_count};
use crate::internal::tags::{decrease_tag_reference_count, find_tag_by_id, increase_tag_reference_count};
use crate::model::models::{GroupTagCondition, GroupTagDTO};
use crate::service::AppError;

pub fn create_group_tag(
    conn: &mut SqliteConnection,
    group_tag_dto: GroupTagDTO
) -> Result<GroupTagDTO, AppError> {
    let group = find_group_by_id(conn, group_tag_dto.group_id)?
        .ok_or(AppError::GroupNotFound)?;

    let tag = find_tag_by_id(conn, group_tag_dto.tag_id)?
        .ok_or(AppError::TagNotFound)?;

    let result = conn.transaction::<_, AppError, _>(|conn| {
        // 业务逻辑：增加引用计数
        increase_group_reference_count(conn, group_tag_dto.group_id)?;
        increase_tag_reference_count(conn, group_tag_dto.tag_id)?;

        // 调用数据访问层执行插入操作
        group_tag::insert_group_tag(conn, &group_tag_dto)?;
        Ok(())
    })?;

    Ok(group_tag_dto)
}

pub fn delete_group_tag_by_id(
    conn: &mut SqliteConnection,
    group_tag_dto: GroupTagDTO
) -> Result<usize, AppError> {
    let group = find_group_by_id(conn, group_tag_dto.group_id)?
        .ok_or(AppError::GroupNotFound)?;

    let tag = find_tag_by_id(conn, group_tag_dto.tag_id)?
        .ok_or(AppError::TagNotFound)?;

    let result = conn.transaction::<_, AppError, _>(|conn| {
        // 业务逻辑：减少引用计数
        decrease_group_reference_count(conn, group_tag_dto.group_id)?;
        decrease_tag_reference_count(conn, group_tag_dto.tag_id)?;

        // 调用数据访问层执行删除操作
        let deleted_count = group_tag::delete_group_tag_by_id(conn, &group_tag_dto)?;

        Ok(deleted_count)
    })?;

    Ok(result)
}


pub fn select_group_tags_by_conditions(
    conn: &mut SqliteConnection,
    condition: Vec<GroupTagCondition>,
    limit: Option<i64>,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
    group_tags::select_group_tags_by_conditions(conn, condition, limit)
}

pub fn delete_group_tags_by_conditions(
    conn: &mut SqliteConnection,
    condition: Vec<GroupTagCondition>,
) -> Result<usize, Error> {
    group_tags::delete_group_tags_by_conditions(conn, condition)
}
