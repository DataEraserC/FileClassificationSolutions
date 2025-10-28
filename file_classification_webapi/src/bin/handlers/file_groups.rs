use crate::utils::database::DbPool;
use crate::utils::models::{ApiError, ApiResponse};
use actix_web::{delete, get, post, web, HttpResponse, Result};
use file_classification_core::model::models::FileGroupDTO;
use file_classification_core::service::file_group::{
    delete_file_groups_by_conditions, delete_file_groups_by_dtos, select_file_groups_by_conditions_with_options, select_file_groups_by_conditions_with_pagination, select_file_groups_by_filter, select_file_groups_by_filter_with_options, select_file_groups_by_filter_with_pagination,
};
use file_classification_core::{
    model::models::{FileGroupCondition, FileGroupFilter},
    service::file_group::{
        create_file_group, delete_file_group_by_dto, select_file_groups_by_conditions,
    }
    ,
};
use serde_json::json;

/// 根据过滤条件获取文件组关联列表
///
/// 根据过滤条件获取文件组关联列表，默认最多返回100条记录
///
/// 请求路径: GET /api/file-groups/filter
#[get("/api/file-groups/filter")]
async fn api_list_file_groups_by_filter(
    query: web::Query<FileGroupFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层查询文件组关联列表
    match select_file_groups_by_filter(&mut conn, query.into_inner(), 100) {
        Ok(file_groups) => {
            let count = file_groups.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(file_groups),
                message: None,
                count: Some(count),
            }))
        }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 根据条件搜索文件组
///
/// 接收一个 JSON 数组作为请求体，数组中的每个元素都是一个 FileGroupCondition 类型的对象，
/// 代表一个查询条件。最多返回 100 条匹配的结果。
///
/// 请求路径: GET /api/file-groups/search/by-conditions
#[get("/api/file-groups/search/by-conditions")]
async fn api_list_file_groups_by_conditions(
    conditions: web::Json<Vec<FileGroupCondition>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用核心服务层的方法执行查询，并限制最大结果数为 100
    match select_file_groups_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
        Ok(file_groups) => {
            let count = file_groups.len();

            // 构造成功的响应对象并返回
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(file_groups),
                message: None,
                count: Some(count),
            }))
        }

        // 如果出现错误，则构造失败的响应对象并返回
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 带选项地根据条件搜索文件组
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/file-groups/search/by-conditions-with-options
#[get("/api/file-groups/search/by-conditions-with-options")]
async fn api_list_file_groups_by_conditions_with_options(
    query: web::Query<(
        Vec<FileGroupCondition>,
        file_classification_core::model::models::FileGroupQueryOptions,
    )>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let (conditions, options) = query.into_inner();

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行带选项的查询
    match select_file_groups_by_conditions_with_options(&mut conn, conditions, options) {
        Ok(file_groups) => {
            let count = file_groups.len();

            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(file_groups),
                message: None,
                count: Some(count),
            }))
        }

        // 处理错误情况
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 带选项地根据过滤条件搜索文件组
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/file-groups/search/by-filter-with-options
#[get("/api/file-groups/search/by-filter-with-options")]
async fn api_list_file_groups_by_filter_with_options(
    query: web::Query<(FileGroupFilter, file_classification_core::model::models::FileGroupQueryOptions)>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let (filter, options) = query.into_inner();

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行带选项的查询
    match select_file_groups_by_filter_with_options(&mut conn, filter, options) {
        Ok(file_groups) => {
            let count = file_groups.len();

            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(file_groups),
                message: None,
                count: Some(count),
            }))
        }

        // 处理错误情况
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 根据过滤条件分页搜索文件组
///
/// 此接口允许客户端传递分页参数，以分页方式检索数据
///
/// 请求路径: GET /api/file-groups/search/by-filter-with-pagination
#[get("/api/file-groups/search/by-filter-with-pagination")]
async fn api_list_file_groups_by_filter_with_pagination(
    query: web::Query<std::collections::HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let query_map = query.into_inner();
    let filter_str = query_map.get("filter").cloned().unwrap_or_default();
    let options_str = query_map.get("options").cloned().unwrap_or_default();

    let filter: FileGroupFilter = serde_json::from_str(&filter_str).unwrap_or_else(|_| FileGroupFilter {
        file_id: None,
        group_id: None,
        relation_type: None,
    });
    let options: file_classification_core::model::models::FileGroupQueryOptions = serde_json::from_str(&options_str)?;

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行分页查询
    match select_file_groups_by_filter_with_pagination(&mut conn, filter, options) {
        Ok(result) => {
            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(result.clone()),
                message: None,
                count: Some(result.data.len()),
            }))
        }

        // 处理错误情况
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 根据条件分页搜索文件组
///
/// 此接口允许客户端传递条件和分页参数，以分页方式检索数据
///
/// 请求路径: GET /api/file-groups/search/by-conditions-with-pagination
#[get("/api/file-groups/search/by-conditions-with-pagination")]
async fn api_list_file_groups_by_conditions_with_pagination(
    query: web::Query<std::collections::HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let query_map = query.into_inner();
    let conditions_str = query_map.get("conditions").cloned().unwrap_or_default();
    let options_str = query_map.get("options").cloned().unwrap_or_default();

    let conditions: Vec<FileGroupCondition> = serde_json::from_str(&conditions_str).unwrap_or_else(|_| Vec::new());
    let options: file_classification_core::model::models::FileGroupQueryOptions = serde_json::from_str(&options_str)?;

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行分页查询
    match select_file_groups_by_conditions_with_pagination(&mut conn, conditions, options) {
        Ok(result) => {
            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(result.clone()),
                message: None,
                count: Some(result.data.len()),
            }))
        }

        // 处理错误情况
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 创建一个新的文件组关联
///
/// 客户端需提供完整的 FileGroupDTO 数据结构作为请求体。
///
/// 请求路径: POST /api/file-groups
#[post("/api/file-groups")]
async fn api_create_file_group(
    file_group_dto: web::Json<FileGroupDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层创建新记录
    match create_file_group(&mut conn, file_group_dto.into_inner()) {
        Ok(_) =>
        // 成功时返回 Created 状态码及提示信息
            {
                Ok(HttpResponse::Created().json(json!({
					"success": true,
					"message": "文件组关联创建成功"
			})))
            }

        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 删除指定的文件组关联
///
/// 需要提供完整的 FileGroupDTO 结构来标识要删除的具体项。
///
/// 请求路径: DELETE /api/file-groups
#[delete("/api/file-groups")]
async fn api_delete_file_group(
    file_group_dto: web::Json<FileGroupDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层删除记录
    match delete_file_group_by_dto(&mut conn, &file_group_dto.into_inner()) {
        Ok(_) =>
        // 成功时返回 OK 状态码及确认消息
            {
                Ok(HttpResponse::Ok().json(json!({
					"success": true,
					"message": "文件组关联删除成功"
			})))
            }

        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 根据条件批量删除文件组关联
///
/// 客户端应发送一个包含多个 FileGroupCondition 的数组，所有满足这些条件的记录都将被删除。
///
/// 请求路径: DELETE /api/file-groups/delete/by-conditions
#[delete("/api/file-groups/delete/by-conditions")]
async fn api_delete_file_groups_by_conditions(
    conditions: web::Json<Vec<file_classification_core::model::models::FileGroupCondition>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层执行批量删除操作
    match delete_file_groups_by_conditions(&mut conn, conditions.into_inner()) {
        Ok(count) =>
        // 成功时返回删除条目数量
            {
                Ok(HttpResponse::Ok().json(json!({
					"success": true,
					"message": format!("成功删除 {} 条记录", count),
					"count": count
			})))
            }

        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 根据DTO列表批量删除文件组关联
///
/// 根据文件组关联DTO列表批量删除指定关联
///
/// 请求路径: DELETE /api/file-groups/delete/by-dtos
#[delete("/api/file-groups/delete/by-dtos")]
async fn api_delete_file_groups_by_dtos(
    dtos: web::Json<Vec<FileGroupDTO>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据DTO列表批量删除文件组关联
    match delete_file_groups_by_dtos(&mut conn, dtos.into_inner()) {
        Ok(count) =>
        // 构造成功响应，包含删除记录数
            {
                Ok(HttpResponse::Ok().json(json!({
					"success": true,
					"message": format!("成功删除 {} 条记录", count),
					"count": count
			})))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}