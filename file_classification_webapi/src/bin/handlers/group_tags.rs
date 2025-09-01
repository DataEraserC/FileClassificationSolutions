use actix_web::{get, post, delete, web, HttpResponse, Result};
use serde_json::json;
use file_classification_core::{model::models::GroupTagCondition, service::group_tag::{select_group_tags_by_conditions, create_group_tag, delete_group_tag_by_id}, utils};
use file_classification_core::model::models::GroupTagDTO;
use crate::utils::database::{DbPool, DbPooledConnection};
use crate::utils::models::{CreateGroupTagDTO, ApiResponse, ApiError};

#[get("/api/group-tags")]
async fn api_list_group_tags_by_conditions(
    conditions: web::Json<Vec<GroupTagCondition>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    match select_group_tags_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
        Ok(group_tags) => {
            let count = group_tags.len();
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(group_tags),
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

#[post("/api/group-tags")]
async fn api_create_group_tag(
    group_tag_dto: web::Json<GroupTagDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    match create_group_tag(&mut conn, group_tag_dto.into_inner()) {
        Ok(_) => Ok(HttpResponse::Created().json(json!({
            "success": true,
            "message": "组标签关联创建成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

#[delete("/api/group-tags")]
async fn api_delete_group_tag(
    group_tag_dto: web::Json<GroupTagDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    match delete_group_tag_by_id(&mut conn, group_tag_dto.into_inner()) {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": "组标签关联删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}
