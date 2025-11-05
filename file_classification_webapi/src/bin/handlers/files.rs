use crate::utils::database::DbPool;
use crate::utils::models::{ApiError, ApiResponse};
use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use file_classification_core::service::files::{create_file, delete_files_by_conditions, delete_files_by_ids, select_file_by_group_id, select_files_by_conditions_with_limit, select_files_by_conditions_with_options, select_files_by_conditions_with_pagination, select_files_by_filter_with_limit, select_files_by_filter_with_options, select_files_by_filter_with_pagination, update_file_by_id};
use file_classification_core::{
    model::models::{FileCondition, FileFilter, FileQueryOptions, UpdateFileDTO},
    service::files::{
        delete_file, get_file_by_id, update_files_by_conditions,
    }
    ,
};
use serde_json::json;

/// 根据过滤条件获取文件列表
///
/// 根据过滤条件获取文件列表，可以指定返回记录数量上限
///
/// 请求路径: GET /api/files/search/by-filter-with-limit
#[get("/api/files/search/by-filter-with-limit")]
async fn api_list_files_by_filter_with_limit(
    query: web::Query<FileFilter>,
    limit: web::Query<Option<i64>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层查询文件列表
    match select_files_by_filter_with_limit(&mut conn, query.into_inner(), limit.into_inner()) {
        Ok(files) => {
            let count = files.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse::success_with_count(files, count)))
        }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 带选项地根据过滤条件搜索文件
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/files/search/by-filter-with-options
#[get("/api/files/search/by-filter-with-options")]
async fn api_list_files_by_filter_with_options(
    query: web::Query<(FileFilter, FileQueryOptions)>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let (filter, options) = query.into_inner();

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行带选项的查询
    match select_files_by_filter_with_options(&mut conn, filter, options) {
        Ok(files) => {
            let count = files.len();

            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse::success_with_count(files, count)))
        }

        // 处理错误情况
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 根据ID获取文件
///
/// 根据文件ID获取指定文件的详细信息
///
/// 请求路径: GET /api/files/{id}
#[get("/api/files/{id}")]
async fn api_get_file_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse> {
    // 获取路径参数中的文件ID
    let file_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据ID查询文件
    match get_file_by_id(&mut conn, file_id) {
        Ok(file) => {
            // 构造成功响应，返回单个文件
            Ok(HttpResponse::Ok().json(ApiResponse::success(file)))
        }
        // 文件未找到
        Err(diesel::result::Error::NotFound) => {
            Ok(
                HttpResponse::NotFound()
                    .json(ApiResponse::<()>::error_with_code("FILE_NOT_FOUND", "文件未找到")),
            )
        }
        // 其他错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 根据条件搜索文件
///
/// 根据提供的条件数组搜索文件，可以指定返回记录数量上限
///
/// 请求路径: GET /api/files/search/by-conditions-with-limit
#[get("/api/files/search/by-conditions-with-limit")]
async fn api_list_files_by_conditions_with_limit(
    conditions: web::Json<Vec<FileCondition>>,
    limit: web::Query<Option<i64>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据条件查询文件
    match select_files_by_conditions_with_limit(&mut conn, conditions.into_inner(), limit.into_inner()) {
        Ok(files) => {
            let count = files.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse::success_with_count(files, count)))
        }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 根据条件批量更新文件
///
/// 根据提供的条件数组和更新数据批量更新文件
///
/// 请求路径: PUT /api/files/update/by-conditions
#[put("/api/files/update/by-conditions")]
async fn api_update_files_by_conditions(
    payload: web::Json<(Vec<FileCondition>, UpdateFileDTO)>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析请求参数
    let (conditions, update_dto) = payload.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据条件批量更新文件
    match update_files_by_conditions(&mut conn, conditions, update_dto) {
        Ok(count) =>
        // 构造成功响应，包含更新记录数
            {
                Ok(HttpResponse::Ok().json(ApiResponse::success_with_msg(count, &format!("成功更新 {} 条记录", count))))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("UPDATE_FILE_FAILED", &e.to_string())),
        ),
    }
}

/// 根据ID删除文件
///
/// 根据文件ID删除指定文件
///
/// 请求路径: DELETE /api/files/{id}
#[delete("/api/files/{id}")]
async fn api_delete_file_by_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的文件ID
    let file_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层删除指定文件
    match delete_file(&mut conn, file_id) {
        Ok(_) =>
        // 构造成功响应
            {
                Ok(HttpResponse::Ok().json(ApiResponse::success_with_msg((), "文件删除成功")))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("DELETE_FILE_FAILED", &e.to_string())),
        ),
    }
}

/// 根据组ID获取文件列表
///
/// 根据组ID获取关联的所有文件
///
/// 请求路径: GET /api/files/group/{group_id}
#[get("/api/files/group/{group_id}")]
async fn api_list_files_by_group_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的组ID
    let group_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据组ID查询文件
    match select_file_by_group_id(&mut conn, group_id) {
        Ok(files) => {
            let count = files.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse::success_with_count(files, count)))
        }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 根据ID更新文件
///
/// 根据文件ID更新指定文件信息
///
/// 请求路径: PUT /api/files/{id}
#[put("/api/files/{id}")]
async fn api_update_file_by_id(
    path: web::Path<i32>,
    update_dto: web::Json<UpdateFileDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的文件ID
    let file_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层更新指定文件
    match update_file_by_id(&mut conn, file_id, update_dto.into_inner()) {
        Ok(count) =>
        // 构造成功响应，包含更新记录数
            {
                Ok(HttpResponse::Ok().json(ApiResponse::success_with_msg(count, &format!("成功更新 {} 条记录", count))))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("UPDATE_FILE_FAILED", &e.to_string())),
        ),
    }
}

/// 根据条件批量删除文件
///
/// 根据提供的条件数组批量删除文件
///
/// 请求路径: DELETE /api/files/delete/by-conditions
#[delete("/api/files/delete/by-conditions")]
async fn api_delete_files_by_conditions(
    conditions: web::Json<Vec<file_classification_core::model::models::FileCondition>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据条件批量删除文件
    match delete_files_by_conditions(&mut conn, conditions.into_inner()) {
        Ok(count) =>
        // 构造成功响应，包含删除记录数
            {
                Ok(HttpResponse::Ok().json(ApiResponse::success_with_msg(count, &format!("成功删除 {} 条记录", count))))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("DELETE_FILE_FAILED", &e.to_string())),
        ),
    }
}

/// 带选项地根据条件搜索文件
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/files/search/by-conditions-with-options
#[get("/api/files/search/by-conditions-with-options")]
async fn api_list_files_by_conditions_with_options(
    query: web::Query<(
        Vec<FileCondition>,
        file_classification_core::model::models::FileQueryOptions,
    )>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let (conditions, options) = query.into_inner();

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行带选项的查询
    match select_files_by_conditions_with_options(&mut conn, conditions, options) {
        Ok(files) => {
            let count = files.len();

            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse::success_with_count(files, count)))
        }

        // 处理错误情况
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("INTERNAL_ERROR", &e.to_string())),
        ),
    }
}

/// 创建新的文件
///
/// 创建一个新的文件，需要提供 CreateFileDTO 数据结构
///
/// 请求路径: POST /api/files
#[post("/api/files")]
async fn api_create_file(
    payload: web::Json<file_classification_core::model::models::CreateFileDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 获取请求数据
    let file_dto = payload.into_inner();

    // 调用服务层创建新文件
    match create_file(&mut conn, file_dto) {
        Ok(file) =>
        // 构造成功响应，返回创建的文件信息
            {
                Ok(HttpResponse::Created().json(ApiResponse::success(file)))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("CREATE_FILE_FAILED", &e.to_string())),
        ),
    }
}

/// 根据ID列表批量删除文件
///
/// 根据文件ID列表批量删除指定文件
///
/// 请求路径: DELETE /api/files/delete/by-ids
#[delete("/api/files/delete/by-ids")]
async fn api_delete_files_by_ids(
    file_ids: web::Json<Vec<i32>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据ID列表批量删除文件
    match delete_files_by_ids(&mut conn, file_ids.into_inner()) {
        Ok(count) =>
        // 构造成功响应，包含删除记录数
            {
                Ok(HttpResponse::Ok().json(ApiResponse::success_with_msg(count, &format!("成功删除 {} 条记录", count))))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error_with_code("DELETE_FILE_FAILED", &e.to_string())),
        ),
    }
}

/// 根据过滤条件分页搜索文件
///
/// 此接口允许客户端传递分页参数，以分页方式检索数据
///
/// 请求路径: GET /api/files/search/by-filter-with-pagination
#[get("/api/files/search/by-filter-with-pagination")]
async fn api_list_files_by_filter_with_pagination(
    query: web::Query<std::collections::HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let query_map = query.into_inner();
    let filter_str = query_map.get("filter").cloned().unwrap_or_default();
    let options_str = query_map.get("options").cloned().unwrap_or_default();

    let filter: FileFilter = serde_json::from_str(&filter_str).unwrap_or_else(|_| FileFilter {
        id: None,
        type_: None,
        path: None,
        reference_count: None,
        group_id: None,
        description: None,
    });
    let options: file_classification_core::model::models::FileQueryOptions = serde_json::from_str(&options_str)?;

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行分页查询
    match select_files_by_filter_with_pagination(&mut conn, filter, options) {
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

/// 根据条件分页搜索文件
///
/// 此接口允许客户端传递分页参数，以分页方式检索数据
///
/// 请求路径: GET /api/files/search/by-conditions-with-pagination
#[get("/api/files/search/by-conditions-with-pagination")]
async fn api_list_files_by_conditions_with_pagination(
    query: web::Query<std::collections::HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let query_map = query.into_inner();
    let conditions_str = query_map.get("conditions").cloned().unwrap_or_default();
    let options_str = query_map.get("options").cloned().unwrap_or_default();

    let conditions: Vec<FileCondition> = serde_json::from_str(&conditions_str).unwrap_or_else(|_| Vec::new());
    let options: file_classification_core::model::models::FileQueryOptions = serde_json::from_str(&options_str)?;

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行分页查询
    match select_files_by_conditions_with_pagination(&mut conn, conditions, options) {
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