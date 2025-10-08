use actix_web::{get, post, put, delete, web, HttpResponse, Result};
use serde_json::json;
use file_classification_core::{model::models::{TagCondition, UpdateTagDTO, TagFilter}, service::tags::{select_tags_by_filter, select_tags_by_conditions, create_tag, update_tags_by_conditions, delete_tag}, utils};
use file_classification_core::model::models::CreateTagDTO;
use file_classification_core::service::tags::{delete_tags_by_conditions, select_tag_by_group_id, select_tags_by_conditions_with_options};
use crate::utils::database::{DbPool, DbPooledConnection};
use crate::utils::models::{ApiResponse, ApiError};

/// 根据过滤条件获取标签列表
///
/// 根据过滤条件获取标签列表，默认最多返回100条记录
///
/// 请求路径: GET /api/tags/filter
#[get("/api/tags/filter")]
async fn api_list_tags_by_filter(
    query: web::Query<TagFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层查询标签列表
    match select_tags_by_filter(&mut conn, query.into_inner(), 100) {
        Ok(tags) => {
            let count = tags.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(tags),
                message: None,
                count: Some(count),
            }))
        }
        // 错误处理
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

/// 根据ID获取标签
///
/// 根据标签ID获取指定标签的详细信息
///
/// 请求路径: GET /api/tags/{id}
#[get("/api/tags/{id}")]
async fn api_get_tag_by_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的标签ID
    let tag_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 构造条件数组，只包含当前标签ID
    let conditions = vec![file_classification_core::model::models::TagCondition::Id(tag_id)];

    // 调用服务层根据ID查询标签
    match select_tags_by_conditions(&mut conn, conditions, Some(1)) {
        Ok(tags) => {
            if tags.is_empty() {
                // 未找到标签时返回404
                Ok(HttpResponse::NotFound().json(ApiError {
                    success: false,
                    message: "标签未找到".to_string(),
                }))
            } else {
                // 构造成功响应，返回单个标签
                Ok(HttpResponse::Ok().json(ApiResponse {
                    success: true,
                    data: Some(&tags[0]),
                    message: None,
                    count: Some(1),
                }))
            }
        }
        // 错误处理
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

/// 根据条件搜索标签
///
/// 根据提供的条件数组搜索标签，最多返回100条记录
///
/// 请求路径: POST /api/tags/search/by-conditions
#[post("/api/tags/search/by-conditions")]
async fn api_list_tags_by_conditions(
    conditions: web::Json<Vec<TagCondition>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据条件查询标签
    match select_tags_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
        Ok(tags) => {
            let count = tags.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(tags),
                message: None,
                count: Some(count),
            }))
        }
        // 错误处理
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

/// 创建新的标签
///
/// 创建一个新的标签，需要提供 CreateTagDTO 数据结构
///
/// 请求路径: POST /api/tags
#[post("/api/tags")]
async fn api_create_tag(
    payload: web::Json<CreateTagDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 获取请求数据
    let tag_dto = payload.into_inner();

    // 调用服务层创建新标签
    match create_tag(&mut conn, &tag_dto) {
        Ok(tag) =>
            // 构造成功响应，返回创建的标签信息
            Ok(HttpResponse::Created().json(ApiResponse::from(tag))),
        // 错误处理
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

/// 根据条件批量更新标签
///
/// 根据提供的条件数组和更新数据批量更新标签
///
/// 请求路径: PUT /api/tags/update/by-conditions
#[put("/api/tags/update/by-conditions")]
async fn api_update_tags_by_conditions(
    payload: web::Json<(Vec<TagCondition>, UpdateTagDTO)>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析请求参数
    let (conditions, update_dto) = payload.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据条件批量更新标签
    match update_tags_by_conditions(&mut conn, conditions, update_dto) {
        Ok(count) =>
            // 构造成功响应，包含更新记录数
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("成功更新 {} 条记录", count),
                "count": count
            }))),
        // 错误处理
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

/// 根据ID删除标签
///
/// 根据标签ID删除指定标签
///
/// 请求路径: DELETE /api/tags/{id}
#[delete("/api/tags/{id}")]
async fn api_delete_tag_by_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的标签ID
    let tag_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层删除指定标签
    match delete_tag(&mut conn, tag_id) {
        Ok(_) =>
            // 构造成功响应
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "message": "标签删除成功"
            }))),
        // 错误处理
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

/// 根据组ID获取标签列表
///
/// 根据组ID获取关联的所有标签
///
/// 请求路径: GET /api/tags/group/{group_id}
#[get("/api/tags/group/{group_id}")]
async fn api_list_tags_by_group_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的组ID
    let group_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据组ID查询标签
    match select_tag_by_group_id(&mut conn, group_id) {
        Ok(tags) => {
            let count = tags.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(tags),
                message: None,
                count: Some(count),
            }))
        }
        // 错误处理
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

/// 根据ID更新标签
///
/// 根据标签ID更新指定标签信息
///
/// 请求路径: PUT /api/tags/{id}
#[put("/api/tags/{id}")]
async fn api_update_tag_by_id(
    path: web::Path<i32>,
    update_dto: web::Json<file_classification_core::model::models::UpdateTagDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的标签ID
    let tag_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 构造条件数组，只包含当前标签ID
    let conditions = vec![file_classification_core::model::models::TagCondition::Id(tag_id)];

    // 调用服务层更新指定标签
    match update_tags_by_conditions(&mut conn, conditions, update_dto.into_inner()) {
        Ok(count) =>
            // 构造成功响应，包含更新记录数
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("成功更新 {} 条记录", count),
                "count": count
            }))),
        // 错误处理
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

/// 根据条件批量删除标签
///
/// 根据提供的条件数组批量删除标签
///
/// 请求路径: DELETE /api/tags/delete/by-conditions
#[delete("/api/tags/delete/by-conditions")]
async fn api_delete_tags_by_conditions(
    conditions: web::Json<Vec<file_classification_core::model::models::TagCondition>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据条件批量删除标签
    match delete_tags_by_conditions(&mut conn, conditions.into_inner()) {
        Ok(count) =>
            // 构造成功响应，包含删除记录数
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("成功删除 {} 条记录", count),
                "count": count
            }))),
        // 错误处理
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}

/// 带选项地根据条件搜索标签
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/tags/search/by-conditions-with-options
#[get("/api/tags/search/by-conditions-with-options")]
async fn api_list_tags_by_conditions_with_options(
    query: web::Query<(Vec<TagCondition>, file_classification_core::model::models::TagQueryOptions)>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let (conditions, options) = query.into_inner();

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行带选项的查询
    match select_tags_by_conditions_with_options(&mut conn, conditions, options) {
        Ok(tags) => {
            let count = tags.len();

            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(tags),
                message: None,
                count: Some(count),
            }))
        },

        // 处理错误情况
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiError {
            success: false,
            message: e.to_string(),
        }))
    }
}
