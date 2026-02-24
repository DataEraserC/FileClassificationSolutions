// file_classification_web/src/models.rs

use serde::Serialize;

// 通用响应结构，符合code/data/msg格式
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
  pub code: String,
  pub data: Option<T>,
  pub msg: String,
  pub count: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
  pub code: String,
  pub msg: String,
}

impl<T> ApiResponse<T> {
  pub fn success(data: T) -> Self {
    Self {
      code: "SUCCESS".to_string(),
      data: Some(data),
      msg: "Operation successful".to_string(),
      count: None,
    }
  }

  pub fn success_with_msg(data: T, msg: &str) -> Self {
    Self { code: "SUCCESS".to_string(), data: Some(data), msg: msg.to_string(), count: None }
  }

  pub fn success_with_count(data: T, count: usize) -> Self {
    Self {
      code: "SUCCESS".to_string(),
      data: Some(data),
      msg: "Operation successful".to_string(),
      count: Some(count),
    }
  }

  pub fn error(msg: &str) -> Self {
    Self { code: "ERROR".to_string(), data: None, msg: msg.to_string(), count: None }
  }

  pub fn error_with_code(code: &str, msg: &str) -> Self {
    Self { code: code.to_string(), data: None, msg: msg.to_string(), count: None }
  }
}

impl ApiError {
  pub fn new(code: &str, msg: &str) -> Self {
    Self { code: code.to_string(), msg: msg.to_string() }
  }
}
