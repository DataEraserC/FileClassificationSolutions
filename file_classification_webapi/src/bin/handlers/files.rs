use actix_web::{get, post, put, delete, web, HttpResponse, Result};
use serde_json::json;
use file_classification_core::{model::models::{FileCondition, UpdateFileDTO, FileFilter}, service::files::{select_files, select_files_by_conditions, update_files_by_conditions, delete_file}, utils};
use crate::utils::models::{ApiResponse, ApiError};
#[get("/api/files")]
async fn api_list_files(
    query: web::Query<FileFilter>,
) -> Result<HttpResponse> {
    let mut conn = utils::database::establish_connection();
    match select_files(&mut conn, query.into_inner(), 100) {
        Ok(files) => {
            let count = files.len();
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(files),
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

#[post("/api/files/search")]
async fn api_list_files_by_conditions(
    conditions: web::Json<Vec<FileCondition>>,
) -> Result<HttpResponse> {
    let mut conn = utils::database::establish_connection();
    match select_files_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
        Ok(files) => {
            let count = files.len();
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(files),
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

// #[post("/api/files")]
// async fn api_create_file(
//     file_dto: web::Json<CreateFileDTO>,
// ) -> Result<HttpResponse> {
//     let create_dto = file_classification_core::model::models::CreateFileDTO {
//         type_: &file_dto.type_,
//         path: &file_dto.path,
//         group_id: file_dto.group_id,
//     };
//
//     let mut conn = utils::database::establish_connection();
//     match create_file(&mut conn, &create_dto) {
//         Ok(file) => Ok(HttpResponse::Created().json(ApiResponse::from(file))),
//         Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
//             success: false,
//             message: e.to_string(),
//         }))
//     }
// }

#[put("/api/files")]
async fn api_update_files_by_conditions(
    payload: web::Json<(Vec<FileCondition>, UpdateFileDTO)>,
) -> Result<HttpResponse> {
    let (conditions, update_dto) = payload.into_inner();

    let mut conn = utils::database::establish_connection();
    match update_files_by_conditions(&mut conn, conditions, update_dto) {
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

#[delete("/api/files/{id}")]
async fn api_delete_file(
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let file_id = path.into_inner();

    let mut conn = utils::database::establish_connection();
    match delete_file(&mut conn, file_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": "文件删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}
