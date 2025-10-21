// errors.rs
//! 应用程序错误处理模块
//!
//! 定义了应用程序中可能发生的各种错误类型，包括业务逻辑错误、数据验证错误和数据库错误等，
//! 并提供了错误类型的显示格式化和错误类型转换功能。

use diesel::result::Error as DieselError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// 应用程序自定义错误枚举
/// 
/// 包含了应用程序中可能遇到的所有错误类型，便于统一错误处理和返回友好的错误信息
#[derive(Debug)]
pub enum AppError {
    // Group errors
    /// 分组未找到错误
    GroupNotFound,
    /// 创建分组失败错误，包含具体的失败原因
    CreateGroupFailed(String),
    /// 删除分组失败错误，包含具体的失败原因
    DeleteGroupFailed(String),

    // File errors
    /// 文件未找到错误
    FileNotFound,
    /// 创建文件失败错误，包含具体的失败原因
    CreateFileFailed(String),
    /// 删除文件失败错误，包含具体的失败原因
    DeleteFileFailed(String),

    // Tag errors
    /// 标签未找到错误
    TagNotFound,
    /// 创建标签失败错误，包含具体的失败原因
    CreateTagFailed(String),
    /// 删除标签失败错误，包含具体的失败原因
    DeleteTagFailed(String),

    // GroupTag errors
    /// 分组-标签关联未找到错误
    GroupTagNotFound,
    /// 创建分组-标签关联失败错误，包含具体的失败原因
    CreateGroupTagFailed(String),
    /// 删除分组-标签关联失败错误，包含具体的失败原因
    DeleteGroupTagFailed(String),

    // FileGroup errors
    /// 文件-分组关联未找到错误
    FileGroupNotFound,
    /// 创建文件-分组关联失败错误，包含具体的失败原因
    CreateFileGroupFailed(String),
    /// 删除文件-分组关联失败错误，包含具体的失败原因
    DeleteFileGroupFailed(String),
    /// 无法绑定到主分组错误
    CannotBindToPrimaryGroup,
    /// 无法解绑主分组错误
    CannotUnbindPrimaryGroup,
    /// 未来主分组应为空错误
    FuturePrimaryGroupShouldBeEmpty,

    // GroupRelation errors
    /// 组关系循环引用错误
    GroupRelationCycleDetected,
    /// 组关系已存在错误
    GroupRelationAlreadyExists,
    /// 主组不能作为父组错误
    PrimaryGroupCannotBeParent,

    // Validation errors
    /// 数据验证错误，包含具体的验证失败原因
    ValidationError(String),

    // DieselError errors
    /// 数据库错误，包装了 Diesel 数据库错误
    DieselError(DieselError),
}

/// 为 `AppError` 实现 `Display` trait
/// 
/// 定义了各种错误类型的友好显示格式，便于向用户展示错误信息
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AppError::GroupNotFound => write!(f, "Group not found"),
            AppError::CreateGroupFailed(msg) => write!(f, "Create group failed: {}", msg),
            AppError::DeleteGroupFailed(msg) => write!(f, "Delete group failed: {}", msg),

            AppError::FileNotFound => write!(f, "File not found"),
            AppError::CreateFileFailed(msg) => write!(f, "Create file failed: {}", msg),
            AppError::DeleteFileFailed(msg) => write!(f, "Delete file failed: {}", msg),

            AppError::TagNotFound => write!(f, "Tag not found"),
            AppError::CreateTagFailed(msg) => write!(f, "Create tag failed: {}", msg),
            AppError::DeleteTagFailed(msg) => write!(f, "Delete tag failed: {}", msg),

            AppError::GroupTagNotFound => write!(f, "GroupTag not found"),
            AppError::CreateGroupTagFailed(msg) => write!(f, "Create GroupTag failed: {}", msg),
            AppError::DeleteGroupTagFailed(msg) => write!(f, "Delete GroupTag failed: {}", msg),

            AppError::FileGroupNotFound => write!(f, "FileGroup not found"),
            AppError::CreateFileGroupFailed(msg) => write!(f, "Create FileGroup failed: {}", msg),
            AppError::DeleteFileGroupFailed(msg) => write!(f, "Delete FileGroup failed: {}", msg),
            AppError::CannotBindToPrimaryGroup => write!(f, "Cannot associate with primary group"),
            AppError::CannotUnbindPrimaryGroup => write!(f, "Cannot unbind from primary group"),
            AppError::FuturePrimaryGroupShouldBeEmpty => write!(
                f,
                "Future primary group should be empty"
            ),

            // GroupRelation errors
            AppError::GroupRelationCycleDetected => write!(f, "Group relation cycle detected"),
            AppError::GroupRelationAlreadyExists => write!(f, "Group relation already exists"),
            AppError::PrimaryGroupCannotBeParent => write!(f, "Primary group cannot be parent"),

            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),

            AppError::DieselError(e) => write!(f, "Database error: {}", e),
            // other Errors...
            // _ => write!(f, "Unknown error occurred"),
        }
    }
}

/// 为 `AppError` 实现 `Error` trait
/// 
/// 使 `AppError` 成为标准的错误类型，可以与其他错误类型一起使用
impl Error for AppError {}

/// 从 `DieselError` 转换为 `AppError` 的实现
/// 
/// 实现了 `From` trait，允许将 Diesel 数据库错误自动转换为应用程序错误
/// 方便在使用 ? 操作符时自动进行错误类型转换
impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        AppError::DieselError(err)
    }
}

/// 从 `AppError` 转换为 `DieselError` 的实现
/// 
/// 实现了 `From` trait，允许将应用程序错误转换为 Diesel 数据库错误
/// 主要用于需要返回 DieselError 的场景
impl From<AppError> for DieselError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::DieselError(diesel_err) => diesel_err,
            err => DieselError::QueryBuilderError(Box::new(err)),
        }
    }
}
