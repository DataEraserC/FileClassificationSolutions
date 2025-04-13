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

	// Tag errors
	TagNotFound,
	CreateTagFailed(String),

	// NewGroupTag errors
	NewGroupTagNotFound,
	CannotAssociateWithPrimary,
	CreateNewGroupTagFailed(String),
	DeleteNewGroupTagFailed(String),

	// NewFileGroup errors
	NewFileGroupNotFound,
	CreateNewFileGroupFailed(String),
	DeleteNewFileGroupFailed(String),

	// DieselError errors
	DieselError(DieselError),

	// Validation errors
	ValidationError(String),
}

impl Display for AppError {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			AppError::GroupNotFound => write!(f, "Group not found"),
			AppError::CreateGroupFailed(msg) => write!(f, "Create group failed: {}", msg),
			AppError::DieselError(e) => write!(f, "Database error: {}", e),
			AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
			AppError::DeleteGroupFailed(msg) => write!(f, "Delete group failed: {}", msg),
			AppError::FileNotFound => write!(f, "File not found"),
			AppError::CreateFileFailed(msg) => write!(f, "Create file failed: {}", msg),
			AppError::TagNotFound => write!(f, "Tag not found"),
			AppError::CreateTagFailed(msg) => write!(f, "Create tag failed: {}", msg),
			AppError::NewGroupTagNotFound => write!(f, "NewGroupTag not found"),
			AppError::CannotAssociateWithPrimary => write!(f, "Cannot associate with primary group"),
			AppError::CreateNewGroupTagFailed(msg) => write!(f, "Create NewGroupTag failed: {}", msg),
			AppError::DeleteNewGroupTagFailed(msg) => write!(f, "Delete NewGroupTag failed: {}", msg),
			AppError::NewFileGroupNotFound => write!(f, "NewFileGroup not found"),
			AppError::CreateNewFileGroupFailed(msg) => write!(f, "Create NewFileGroup failed: {}", msg),
			AppError::DeleteNewFileGroupFailed(msg) => write!(f, "Delete NewFileGroup failed: {}", msg),
			// 其他错误处理...
			_ => write!(f, "Unknown error occurred"),
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
