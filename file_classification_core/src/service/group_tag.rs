use super::database::SqliteConnection;
use crate::internal::groups::{decrease_group_reference_count, find_group_by_id, increase_group_reference_count};
use crate::internal::tags::{decrease_tag_reference_count, find_tag_by_id, increase_tag_reference_count};
use crate::model::models::{GroupTagCondition, GroupTagDTO};
use crate::service::AppError;
use diesel::result::Error;
use diesel::Connection;

pub fn create_group_tag(
    conn: &mut SqliteConnection,
    group_tag_dto: GroupTagDTO,
) -> Result<GroupTagDTO, AppError> {
    let _group = find_group_by_id(conn, group_tag_dto.group_id)?
        .ok_or(AppError::GroupNotFound)?;

    let _tag = find_tag_by_id(conn, group_tag_dto.tag_id)?
        .ok_or(AppError::TagNotFound)?;

    let _result = conn.transaction::<_, AppError, _>(|conn| {
        // 业务逻辑：增加引用计数
        increase_group_reference_count(conn, group_tag_dto.group_id)?;
        increase_tag_reference_count(conn, group_tag_dto.tag_id)?;

        // 调用数据访问层执行插入操作
        crate::internal::group_tag::insert_group_tag(conn, &group_tag_dto)?;
        Ok(())
    })?;

    Ok(group_tag_dto)
}

pub fn delete_group_tag_by_id(
    conn: &mut SqliteConnection,
    group_tag_dto: GroupTagDTO,
) -> Result<usize, AppError> {
    let _group = find_group_by_id(conn, group_tag_dto.group_id)?
        .ok_or(AppError::GroupNotFound)?;

    let _tag = find_tag_by_id(conn, group_tag_dto.tag_id)?
        .ok_or(AppError::TagNotFound)?;

    let result = conn.transaction::<_, AppError, _>(|conn| {
        // 业务逻辑：减少引用计数
        decrease_group_reference_count(conn, group_tag_dto.group_id)?;
        decrease_tag_reference_count(conn, group_tag_dto.tag_id)?;

        // 调用数据访问层执行删除操作
        let deleted_count = crate::internal::group_tag::delete_group_tag_by_id(conn, &group_tag_dto)?;

        Ok(deleted_count)
    })?;

    Ok(result)
}


pub fn select_group_tags_by_conditions(
    conn: &mut SqliteConnection,
    condition: Vec<GroupTagCondition>,
    limit: Option<i64>,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
    crate::internal::group_tag::select_group_tags_by_conditions(conn, condition, limit)
}

// NOTE: 这个方法在core里不应该有用法
// 要暴露给用户使用的话 应当改为先select再delete_by_id
// 防止引用计算问题
pub fn delete_group_tags_by_conditions(
    conn: &mut SqliteConnection,
    condition: Vec<GroupTagCondition>,
) -> Result<usize, Error> {
    // TODO: 减少组和标签的引用计数
    crate::internal::group_tag::delete_group_tags_by_conditions(conn, condition)
}
