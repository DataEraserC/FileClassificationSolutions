use actix_web::{get, post, put, delete, web, HttpResponse, Result};
use serde_json::json;
use file_classification_core::{model::models::{GroupCondition, UpdateGroupDTO, GroupFilter}, service::groups::{select_groups, select_groups_by_conditions, create_group, update_groups_by_conditions, delete_group}, utils};
use crate::utils::models::{CreateGroupDTO, ApiResponse, ApiError};

#[get("/api/groups")]
async fn api_list_groups(
    query: web::Query<GroupFilter>,
) -> Result<HttpResponse> {
    let mut conn = utils::database::establish_connection();
    match select_groups(&mut conn, query.into_inner(), 100) {
        Ok(groups) => {
            let count = groups.len();
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(groups),
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

#[post("/api/groups/search")]
async fn api_list_groups_by_conditions(
    conditions: web::Json<Vec<GroupCondition>>,
) -> Result<HttpResponse> {
    let mut conn = utils::database::establish_connection();
    match select_groups_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
        Ok(groups) => {
            let count = groups.len();
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(groups),
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

#[post("/api/groups")]
async fn api_create_group(
    group_dto: web::Json<CreateGroupDTO>,
) -> Result<HttpResponse> {
    let mut conn = utils::database::establish_connection();
    match create_group(&mut conn, &group_dto.name) {
        Ok(group) => Ok(HttpResponse::Created().json(ApiResponse::from(group))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

#[put("/api/groups")]
async fn api_update_groups_by_conditions(
    payload: web::Json<(Vec<GroupCondition>, UpdateGroupDTO)>,
) -> Result<HttpResponse> {
    let (conditions, update_dto) = payload.into_inner();

    let mut conn = utils::database::establish_connection();
    match update_groups_by_conditions(&mut conn, conditions, update_dto) {
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

#[delete("/api/groups/{id}")]
async fn api_delete_group(
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let group_id = path.into_inner();

    let mut conn = utils::database::establish_connection();
    match delete_group(&mut conn, group_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": "组删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}
