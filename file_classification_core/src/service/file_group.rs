use crate::internal::file_group as file_groups;
use crate::internal::files::{decrease_file_reference_count, find_file_by_id, increase_file_reference_count};
use crate::internal::groups::{decrease_group_reference_count, find_group_by_id, increase_group_reference_count};
use crate::model::models::{FileGroupCondition, FileGroupDTO, FileGroupQueryOptions};
use crate::service::AppError;
use diesel::result::Error;
use diesel::Connection;
use crate::utils::database::AnyConnection;

// 创建文件组关联(不允许使用于主组绑定)
pub fn create_file_group(
    conn: &mut AnyConnection,
    file_group_dto: FileGroupDTO,
) -> Result<FileGroupDTO, AppError> {
    // 业务规则验证
    let group = find_group_by_id(conn, file_group_dto.group_id)?
        .ok_or(AppError::GroupNotFound)?;

    let _file = find_file_by_id(conn, file_group_dto.file_id)?
        .ok_or(AppError::FileNotFound)?;

    if group.is_primary {
        return Err(AppError::CannotBindToPrimaryGroup);
    }

    // 使用事务处理引用计数和数据插入
    let _result = conn.transaction::<_, AppError, _>(|conn| {
        increase_file_reference_count(conn, file_group_dto.file_id)?;
        increase_group_reference_count(conn, file_group_dto.group_id)?;
        // 错误类型转换，将 diesel::result::Error 转换为 AppError
        file_groups::insert_file_group(conn, &file_group_dto)?;
        Ok(())
    })?;

    Ok(file_group_dto)
}

pub fn delete_file_group(
    conn: &mut AnyConnection,
    file_group_dto: FileGroupDTO,
) -> Result<usize, AppError> {
    let group = find_group_by_id(conn, file_group_dto.group_id)?
        .ok_or(AppError::GroupNotFound)?;

    let _file = find_file_by_id(conn, file_group_dto.file_id)?
        .ok_or(AppError::FileNotFound)?;

    if group.is_primary {
        return Err(AppError::CannotUnbindPrimaryGroup);
    }

    let result = conn.transaction::<_, AppError, _>(|conn| {
        // 减少组和标签的引用计数
        decrease_group_reference_count(conn, file_group_dto.group_id)?;
        decrease_file_reference_count(conn, file_group_dto.file_id)?;

        // 调用数据访问层执行删除操作
        let deleted_count = file_groups::delete_file_group_by_id(conn, &file_group_dto)?;

        Ok(deleted_count)
    });

    result
}

pub fn select_file_groups_by_conditions(
    conn: &mut AnyConnection,
    condition: Vec<FileGroupCondition>,
    limit: Option<i64>,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
    crate::internal::file_group::select_file_groups_by_conditions(conn, condition, limit)
}
pub fn select_file_groups_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<FileGroupCondition>,
    options: FileGroupQueryOptions,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
    crate::internal::file_group::select_file_groups_by_conditions_with_options(conn, conditions, options)
}

// NOTE: 这个方法在core里不应该有用法
// 要暴露给用户使用的话 应当改为先select再delete_by_id
// 防止引用计算问题
pub fn delete_file_groups_by_conditions(
    conn: &mut AnyConnection,
    condition: Vec<FileGroupCondition>,
) -> Result<usize, Error> {
    // 首先查询将要删除的记录
    let file_groups_to_delete = select_file_groups_by_conditions(conn, condition.clone(), None)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Error::NotFound,
            _ => e,
        })?;

    // 使用事务确保数据一致性
    conn.transaction::<_, Error, _>(|conn| {
        // 对于每个要删除的文件组关联，减少对应的文件和组的引用计数
        for file_group in &file_groups_to_delete {
            // 减少文件的引用计数
            decrease_file_reference_count(conn, file_group.file_id)?;

            // 减少组的引用计数
            decrease_group_reference_count(conn, file_group.group_id)?;
        }

        // 执行实际的删除操作
        let deleted_count = crate::internal::file_group::delete_file_groups_by_conditions(conn, condition)?;

        Ok(deleted_count)
    })
}
