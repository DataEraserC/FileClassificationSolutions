use diesel::result::Error as DieselError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
#[derive(Debug)]
pub enum AppError {
    // Group errors
    GroupNotFound,
    CreateGroupFailed(String),
    DeleteGroupFailed(String),

    // File errors
    FileNotFound,
    CreateFileFailed(String),
    DeleteFileFailed(String),

    // Tag errors
    TagNotFound,
    CreateTagFailed(String),
    DeleteTagFailed(String),

    // GroupTag errors
    GroupTagNotFound,
    CannotAssociateWithPrimary,
    CreateGroupTagFailed(String),
    DeleteGroupTagFailed(String),

    // FileGroup errors
    FileGroupNotFound,
    CreateFileGroupFailed(String),
    DeleteFileGroupFailed(String),

    // Validation errors
    ValidationError(String),

    // DieselError errors
    DieselError(DieselError),
}

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
            AppError::CannotAssociateWithPrimary => write!(f, "Cannot associate with primary group"),
            AppError::CreateGroupTagFailed(msg) => write!(f, "Create GroupTag failed: {}", msg),
            AppError::DeleteGroupTagFailed(msg) => write!(f, "Delete GroupTag failed: {}", msg),

            AppError::FileGroupNotFound => write!(f, "FileGroup not found"),
            AppError::CreateFileGroupFailed(msg) => write!(f, "Create FileGroup failed: {}", msg),
            AppError::DeleteFileGroupFailed(msg) => write!(f, "Delete FileGroup failed: {}", msg),

            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),

            AppError::DieselError(e) => write!(f, "Database error: {}", e),
            // other Errors...
            // _ => write!(f, "Unknown error occurred"),
        }
    }
}
impl Error for AppError {}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        AppError::DieselError(err)
    }
}

impl From<AppError> for DieselError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::DieselError(diesel_err) => diesel_err,
            err => DieselError::QueryBuilderError(Box::new(err)),
        }
    }
}
