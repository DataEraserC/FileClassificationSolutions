// file_group.rs
//! 文件-分组关联服务模块
//!
//! 提供文件与分组之间关联关系的业务逻辑处理，包括创建、删除和查询文件-分组关联，
//! 并处理相关的引用计数管理和业务规则验证。

use crate::internal::file_group as file_groups;
use crate::internal::files::{decrease_file_reference_count_by_id, find_file_by_id, increase_file_reference_count_by_id};
use crate::internal::groups::{decrease_group_reference_count_by_id, find_group_by_id, increase_group_reference_count_by_id};
use crate::model::models::{FileGroupCondition, FileGroupDTO, FileGroupQueryOptions};
use crate::service::AppError;
use diesel::result::Error;
use diesel::Connection;
use crate::utils::database::AnyConnection;

/// 创建文件-分组关联关系
///
/// 该函数负责创建文件和分组之间的关联关系，并处理相关的引用计数。
/// 业务规则：不允许将文件关联到主分组。
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_group_dto`: 包含文件ID和分组ID的关联信息
///
/// 返回值:
/// 成功时返回创建的关联关系DTO，失败时返回相应的错误
///
/// 操作流程:
/// 1. 验证分组和文件是否存在
/// 2. 检查分组是否为主分组（主分组不允许通过此方法关联）
/// 3. 在事务中执行以下操作：
///    - 增加文件的引用计数
///    - 增加分组的引用计数
///    - 插入文件-分组关联记录
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
        increase_file_reference_count_by_id(conn, file_group_dto.file_id)?;
        increase_group_reference_count_by_id(conn, file_group_dto.group_id)?;
        // 错误类型转换，将 diesel::result::Error 转换为 AppError
        file_groups::insert_file_group(conn, &file_group_dto)?;
        Ok(())
    })?;

    Ok(file_group_dto)
}

/// 删除文件-分组关联关系
///
/// 该函数负责删除文件和分组之间的关联关系，并处理相关的引用计数。
/// 业务规则：不允许解除文件与主分组的关联。
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `file_group_dto`: 包含文件ID和分组ID的关联信息
///
/// 返回值:
/// 成功时返回删除的记录数，失败时返回相应的错误
///
/// 操作流程:
/// 1. 验证分组和文件是否存在
/// 2. 检查分组是否为主分组（主分组不允许通过此方法解绑）
/// 3. 在事务中执行以下操作：
///    - 减少分组的引用计数
///    - 减少文件的引用计数
///    - 删除文件-分组关联记录
pub fn delete_file_group_by_dto(
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
        decrease_group_reference_count_by_id(conn, file_group_dto.group_id)?;
        decrease_file_reference_count_by_id(conn, file_group_dto.file_id)?;

        // 调用数据访问层执行删除操作
        let deleted_count = file_groups::delete_file_group_by_dto(conn, &file_group_dto)?;

        Ok(deleted_count)
    });

    result
}

/// 根据条件查询文件-分组关联记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 查询条件向量
/// - `limit`: 返回记录数限制（可选）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_file_groups_by_conditions(
    conn: &mut AnyConnection,
    condition: Vec<FileGroupCondition>,
    limit: Option<i64>,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
    crate::internal::file_group::select_file_groups_by_conditions(conn, condition, limit)
}

/// 根据条件和选项查询文件-分组关联记录
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `conditions`: 查询条件向量
/// - `options`: 查询选项（包括分页和排序）
///
/// 返回值:
/// 查询成功的记录列表或数据库错误
pub fn select_file_groups_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<FileGroupCondition>,
    options: FileGroupQueryOptions,
) -> Result<Vec<FileGroupDTO>, diesel::result::Error> {
    crate::internal::file_group::select_file_groups_by_conditions_with_options(conn, conditions, options)
}

/// 根据条件批量删除文件-分组关联记录
///
/// 注意：这个方法在core里不应该有直接用法
/// 要暴露给用户使用的话应当改为先select再delete_by_id，防止引用计算问题
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `condition`: 删除条件向量
///
/// 返回值:
/// 成功删除的记录数或数据库错误
///
/// 操作流程:
/// 1. 先查询将要删除的所有记录
/// 2. 在事务中对每条记录调用delete_file_group执行删除操作
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
        let mut total_deleted = 0;

        // 对于每个要删除的文件组关联，直接调用delete_file_group函数
        for file_group in &file_groups_to_delete {
            let file_group_dto = FileGroupDTO {
                file_id: file_group.file_id,
                group_id: file_group.group_id,
            };

            // 调用单个删除函数，复用其业务逻辑和验证规则
            let deleted_count = delete_file_group_by_dto(conn, file_group_dto)?;
            total_deleted += deleted_count;
        }

        Ok(total_deleted)
    })
}
