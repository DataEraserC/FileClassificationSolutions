use actix_web::{get, post, put, delete, web, HttpResponse, Result};
use serde_json::json;
use file_classification_core::{model::models::{TagCondition, UpdateTagDTO, TagFilter}, service::tags::{select_tags, select_tags_by_conditions, create_tag, update_tags_by_conditions, delete_tag}, utils};
use crate::utils::models::{CreateTagDTO, ApiResponse, ApiError};

#[get("/api/tags")]
async fn api_list_tags(
    query: web::Query<TagFilter>,
) -> Result<HttpResponse> {
    let mut conn = utils::database::establish_connection();
    match select_tags(&mut conn, query.into_inner(), 100) {
        Ok(tags) => {
            let count = tags.len();
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(tags),
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

#[post("/api/tags/search")]
async fn api_list_tags_by_conditions(
    conditions: web::Json<Vec<TagCondition>>,
) -> Result<HttpResponse> {
    let mut conn = utils::database::establish_connection();
    match select_tags_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
        Ok(tags) => {
            let count = tags.len();
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(tags),
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

#[post("/api/tags")]
async fn api_create_tag(
    tag_dto: web::Json<CreateTagDTO>,
) -> Result<HttpResponse> {
    let mut conn = utils::database::establish_connection();
    match create_tag(&mut conn, &tag_dto.name) {
        Ok(tag) => Ok(HttpResponse::Created().json(ApiResponse::from(tag))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

#[put("/api/tags")]
async fn api_update_tags_by_conditions(
    payload: web::Json<(Vec<TagCondition>, UpdateTagDTO)>,
) -> Result<HttpResponse> {
    let (conditions, update_dto) = payload.into_inner();
    let mut conn = utils::database::establish_connection();

    match update_tags_by_conditions(&mut conn, conditions, update_dto) {
        Ok(count) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": format!("成功更新 {} 条记录", count),
            "count": count
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

#[delete("/api/tags/{id}")]
async fn api_delete_tag(
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let tag_id = path.into_inner();

    let mut conn = utils::database::establish_connection();
    match delete_tag(&mut conn, tag_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": "标签删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}
