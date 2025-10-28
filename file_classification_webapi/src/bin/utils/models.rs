// file_classification_web/src/models.rs

use serde::Serialize;

// 通用响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub count: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub success: bool,
    pub message: String,
}

impl<T> From<T> for ApiResponse<T> {
    fn from(data: T) -> Self {
        Self { success: true, data: Some(data), message: None, count: None }
    }
}
