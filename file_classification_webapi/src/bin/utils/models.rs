// file_classification_web/src/models.rs

use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, serde::ts_seconds_option};

// File 相关 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFileDTO {
    pub type_: String,
    pub path: String,
    pub group_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFileDTO {
    pub path: Option<String>,
    pub type_: Option<String>,
    pub reference_count: Option<i32>,
    pub group_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFilter {
    pub id: Option<i32>,
    pub type_: Option<String>,
    pub path: Option<String>,
    pub reference_count: Option<i32>,
    pub group_id: Option<i32>,
}

// Group 相关 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupDTO {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGroupDTO {
    pub name: Option<String>,
    pub reference_count: Option<i32>,
    pub is_primary: Option<bool>,
    pub click_count: Option<i32>,
    pub share_count: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub modify_time: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupFilter {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub reference_count: Option<i32>,
    pub is_primary: Option<bool>,
    pub click_count: Option<i32>,
    pub share_count: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub modify_time: Option<NaiveDateTime>,
}

// Tag 相关 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagDTO {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTagDTO {
    pub name: Option<String>,
    pub reference_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagFilter {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub reference_count: Option<i32>,
}

// FileGroup 相关 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFileGroupDTO {
    pub file_id: i32,
    pub group_id: i32,
}

// GroupTag 相关 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupTagDTO {
    pub group_id: i32,
    pub tag_id: i32,
}

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
        Self {
            success: true,
            data: Some(data),
            message: None,
            count: None,
        }
    }
}
