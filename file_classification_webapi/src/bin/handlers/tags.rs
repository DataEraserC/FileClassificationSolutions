use crate::utils::database::DbPool;
use crate::utils::models::{ApiError, ApiResponse};
use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use file_classification_core::model::models::{CreateTagDTO, TagQueryOptions};
use file_classification_core::service::tags::{delete_tags_by_conditions, delete_tags_by_ids, select_tag_by_group_id, select_tags_by_conditions_with_options, select_tags_by_conditions_with_pagination, select_tags_by_filter_with_options, select_tags_by_filter_with_pagination, update_tag_by_id};
use file_classification_core::{
    model::models::{TagCondition, TagFilter, UpdateTagDTO},
    service::tags::{
        create_tag, delete_tag, get_tag_by_id, select_tags_by_conditions_with_limit, select_tags_by_filter_with_limit,
        update_tags_by_conditions,
    }
    ,
};
use serde_json::json;

/// 根据过滤条件获取标签列表
///
/// 根据过滤条件获取标签列表，可以指定返回记录数量上限
///
/// 请求路径: GET /api/tags/search/by-filter-with-limit
#[get("/api/tags/search/by-filter-with-limit")]
async fn api_list_tags_by_filter_with_limit(
    query: web::Query<TagFilter>,
    limit: web::Query<Option<i64>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层查询标签列表
    match select_tags_by_filter_with_limit(&mut conn, query.into_inner(), limit.into_inner()) {
        Ok(tags) => {
            let count = tags.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse::success_with_count(tags, count)))
        }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 带选项地根据过滤条件搜索标签
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/tags/search/by-filter-with-options
#[get("/api/tags/search/by-filter-with-options")]
async fn api_list_tags_by_filter_with_options(
    query: web::Query<(TagFilter, TagQueryOptions)>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let (filter, options) = query.into_inner();

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行带选项的查询
    match select_tags_by_filter_with_options(&mut conn, filter, options) {
        Ok(tags) => {
            let count = tags.len();

            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse::success_with_count(tags, count)))
        }

        // 处理错误情况
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 根据ID获取标签
///
/// 根据标签ID获取指定标签的详细信息
///
/// 请求路径: GET /api/tags/{id}
#[get("/api/tags/{id}")]
async fn api_get_tag_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse> {
    // 获取路径参数中的标签ID
    let tag_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据ID查询标签
    match get_tag_by_id(&mut conn, tag_id) {
        Ok(tag) => {
            // 构造成功响应，返回单个标签
            Ok(HttpResponse::Ok().json(ApiResponse::success(tag)))
        }
        // 标签未找到
        Err(diesel::result::Error::NotFound) => {
            Ok(
                HttpResponse::NotFound()
                    .json(ApiResponse::<()>::error_with_code("TAG_NOT_FOUND", "标签未找到")),
            )
        }
        // 其他错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 根据条件搜索标签
///
/// 根据提供的条件数组搜索标签，可以指定返回记录数量上限
///
/// 请求路径: GET /api/tags/search/by-conditions-with-limit
#[get("/api/tags/search/by-conditions-with-limit")]
async fn api_list_tags_by_conditions_with_limit(
    conditions: web::Json<Vec<TagCondition>>,
    limit: web::Query<Option<i64>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据条件查询标签
    match select_tags_by_conditions_with_limit(&mut conn, conditions.into_inner(), limit.into_inner()) {
        Ok(tags) => {
            let count = tags.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse::success_with_count(tags, count)))
        }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
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
            {
                Ok(HttpResponse::Created().json(ApiResponse::success(tag)))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("CREATE_TAG_FAILED", &e.to_string())),
        ),
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
            {
                Ok(HttpResponse::Ok().json(ApiResponse::success_with_msg(count, &format!("成功更新 {} 条记录", count))))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("UPDATE_TAG_FAILED", &e.to_string())),
        ),
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
            {
                Ok(HttpResponse::Ok().json(ApiResponse::success_with_msg((), "标签删除成功")))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("DELETE_TAG_FAILED", &e.to_string())),
        ),
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
            Ok(HttpResponse::Ok().json(ApiResponse::success_with_count(tags, count)))
        }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
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
    update_dto: web::Json<UpdateTagDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的标签ID
    let tag_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层更新指定标签
    match update_tag_by_id(&mut conn, tag_id, update_dto.into_inner()) {
        Ok(count) =>
        // 构造成功响应，包含更新记录数
            {
                Ok(HttpResponse::Ok().json(ApiResponse::success_with_msg(count, &format!("成功更新 {} 条记录", count))))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("UPDATE_TAG_FAILED", &e.to_string())),
        ),
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
            {
                Ok(HttpResponse::Ok().json(ApiResponse::success_with_msg(count, &format!("成功删除 {} 条记录", count))))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("DELETE_TAG_FAILED", &e.to_string())),
        ),
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
            Ok(HttpResponse::Ok().json(ApiResponse::success_with_count(tags, count)))
        }

        // 处理错误情况
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 根据ID列表批量删除标签
///
/// 根据标签ID列表批量删除指定标签
///
/// 请求路径: DELETE /api/tags/delete/by-ids
#[delete("/api/tags/delete/by-ids")]
async fn api_delete_tags_by_ids(
    tag_ids: web::Json<Vec<i32>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据ID列表批量删除标签
    match delete_tags_by_ids(&mut conn, tag_ids.into_inner()) {
        Ok(count) =>
        // 构造成功响应，包含删除记录数
            {
                Ok(HttpResponse::Ok().json(ApiResponse::success_with_msg(count, &format!("成功删除 {} 条记录", count))))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("DELETE_TAG_FAILED", &e.to_string())),
        ),
    }
}

/// 根据过滤条件分页搜索标签
///
/// 此接口允许客户端传递分页参数，以分页方式检索数据
///
/// 请求路径: GET /api/tags/search/by-filter-with-pagination
#[get("/api/tags/search/by-filter-with-pagination")]
async fn api_list_tags_by_filter_with_pagination(
    query: web::Query<std::collections::HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let query_map = query.into_inner();
    let filter_str = query_map.get("filter").cloned().unwrap_or_default();
    let options_str = query_map.get("options").cloned().unwrap_or_default();

    let filter: TagFilter = serde_json::from_str(&filter_str).unwrap_or_else(|_| TagFilter {
        id: None,
        name: None,
        reference_count: None,
        description: None,
    });
    let options: TagQueryOptions = serde_json::from_str(&options_str)?;

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行分页查询
    match select_tags_by_filter_with_pagination(&mut conn, filter, options) {
        Ok(result) => {
            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse::success(result.clone())))
        }

        // 处理错误情况
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 根据条件分页搜索标签
///
/// 此接口允许客户端传递分页参数，以分页方式检索数据
///
/// 请求路径: GET /api/tags/search/by-conditions-with-pagination
#[get("/api/tags/search/by-conditions-with-pagination")]
async fn api_list_tags_by_conditions_with_pagination(
    query: web::Query<std::collections::HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let query_map = query.into_inner();
    let conditions_str = query_map.get("conditions").cloned().unwrap_or_default();
    let options_str = query_map.get("options").cloned().unwrap_or_default();

    let conditions: Vec<TagCondition> = serde_json::from_str(&conditions_str).unwrap_or_else(|_| Vec::new());
    let options: TagQueryOptions = serde_json::from_str(&options_str)?;

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行分页查询
    match select_tags_by_conditions_with_pagination(&mut conn, conditions, options) {
        Ok(result) => {
            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse::success(result.clone())))
        }

        // 处理错误情况
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}
