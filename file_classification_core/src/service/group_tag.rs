use crate::internal::groups::{decrease_group_reference_count, find_group_by_id, increase_group_reference_count};
use crate::internal::tags::{decrease_tag_reference_count, find_tag_by_id, increase_tag_reference_count};
use crate::model::models::{GroupTagCondition, GroupTagDTO, GroupTagQueryOptions};
use crate::service::AppError;
use diesel::result::Error;
use diesel::Connection;
use crate::utils::database::AnyConnection;

pub fn create_group_tag(
    conn: &mut AnyConnection,
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
    conn: &mut AnyConnection,
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
    conn: &mut AnyConnection,
    condition: Vec<GroupTagCondition>,
    limit: Option<i64>,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
    crate::internal::group_tag::select_group_tags_by_conditions(conn, condition, limit)
}
pub fn select_group_tags_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<GroupTagCondition>,
    options: GroupTagQueryOptions,
) -> Result<Vec<GroupTagDTO>, diesel::result::Error> {
    crate::internal::group_tag::select_group_tags_by_conditions_with_options(conn, conditions, options)
}

// NOTE: 这个方法在core里不应该有用法
// 要暴露给用户使用的话 应当改为先select再delete_by_id
// 防止引用计算问题
pub fn delete_group_tags_by_conditions(
    conn: &mut AnyConnection,
    condition: Vec<GroupTagCondition>,
) -> Result<usize, Error> {
    // 首先查询将要删除的组标签关联
    let group_tags_to_delete = select_group_tags_by_conditions(conn, condition.clone(), None)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => diesel::result::Error::NotFound,
            _ => e,
        })?;

    // 使用事务确保数据一致性
    conn.transaction::<_, Error, _>(|conn| {
        // 对于每个要删除的组标签关联，减少对应的组和标签的引用计数
        for group_tag in &group_tags_to_delete {
            // 减少组的引用计数
            decrease_group_reference_count(conn, group_tag.group_id)?;

            // 减少标签的引用计数
            decrease_tag_reference_count(conn, group_tag.tag_id)?;
        }

        // 执行实际的删除操作
        let deleted_count = crate::internal::group_tag::delete_group_tags_by_conditions(conn, condition)?;

        Ok(deleted_count)
    })
}
