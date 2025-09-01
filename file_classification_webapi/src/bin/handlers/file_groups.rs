use actix_web::{get, post, delete, web, HttpResponse, Result};
use serde_json::json;
use file_classification_core::{model::models::FileGroupCondition, service::file_group::{select_file_groups_by_conditions, create_file_group, delete_file_group}, utils};
use file_classification_core::model::models::FileGroupDTO;
use crate::utils::database::{DbPool, DbPooledConnection};
use crate::utils::models::{ApiResponse, ApiError};


#[get("/api/file-groups")]
async fn api_list_file_groups_by_conditions(
    conditions: web::Json<Vec<FileGroupCondition>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    match select_file_groups_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
        Ok(file_groups) => {
            let count = file_groups.len();
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(file_groups),
                message: None,
                count: Some(count),
            }))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

#[post("/api/file-groups")]
async fn api_create_file_group(
    file_group_dto: web::Json<FileGroupDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    match create_file_group(&mut conn, file_group_dto.into_inner()) {
        Ok(_) => Ok(HttpResponse::Created().json(json!({
            "success": true,
            "message": "文件组关联创建成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

#[delete("/api/file-groups")]
async fn api_delete_file_group(
    file_group_dto: web::Json<FileGroupDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    match delete_file_group(&mut conn, file_group_dto.into_inner()) {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": "文件组关联删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}
