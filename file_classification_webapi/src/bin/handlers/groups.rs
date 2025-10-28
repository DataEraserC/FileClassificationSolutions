use crate::utils::database::DbPool;
use crate::utils::models::{ApiError, ApiResponse};
use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use file_classification_core::model::models::GroupQueryOptions;
use file_classification_core::service::groups::{delete_groups_by_conditions, delete_groups_by_ids, get_group_by_id, select_group_by_file_id, select_group_by_tag_id, select_groups_by_conditions_with_options, select_groups_by_conditions_with_pagination, select_groups_by_filter_with_options, select_groups_by_filter_with_pagination, update_group_by_id};
use file_classification_core::{
    model::models::{GroupCondition, GroupFilter, UpdateGroupDTO},
    service::groups::{
        create_group, delete_group, get_group_tree, select_groups_by_conditions,
        select_groups_by_filter, update_groups_by_conditions,
    }
    ,
};
use serde_json::json;

/// 根据过滤条件获取组列表
///
/// 根据过滤条件获取组列表，默认最多返回100条记录
///
/// 请求路径: GET /api/groups/filter
#[get("/api/groups/filter")]
async fn api_list_groups_by_filter(
    query: web::Query<GroupFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层查询组列表
    match select_groups_by_filter(&mut conn, query.into_inner(), 100) {
        Ok(groups) => {
            let count = groups.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(groups),
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

/// 带选项地根据过滤条件搜索组
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/groups/search/by-filter-with-options
#[get("/api/groups/search/by-filter-with-options")]
async fn api_list_groups_by_filter_with_options(
    query: web::Query<(GroupFilter, GroupQueryOptions)>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let (filter, options) = query.into_inner();

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行带选项的查询
    match select_groups_by_filter_with_options(&mut conn, filter, options) {
        Ok(groups) => {
            let count = groups.len();

            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(groups),
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

/// 根据ID获取组
///
/// 根据组ID获取指定组的详细信息
///
/// 请求路径: GET /api/groups/{id}
#[get("/api/groups/{id}")]
async fn api_get_group_by_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的组ID
    let group_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据ID查询组
    match get_group_by_id(&mut conn, group_id) {
        Ok(group) => {
            // 构造成功响应，返回单个组
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(group),
                message: None,
                count: Some(1),
            }))
        }
        // 组未找到
        Err(diesel::result::Error::NotFound) => {
            Ok(
                HttpResponse::NotFound()
                    .json(ApiError { success: false, message: "组未找到".to_string() }),
            )
        }
        // 其他错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 根据条件搜索组
///
/// 根据提供的条件数组搜索组，最多返回100条记录
///
/// 请求路径: POST /api/groups/search/by-conditions
#[post("/api/groups/search/by-conditions")]
async fn api_list_groups_by_conditions(
    conditions: web::Json<Vec<GroupCondition>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据条件查询组
    match select_groups_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
        Ok(groups) => {
            let count = groups.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(groups),
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

/// 创建新的组
///
/// 创建一个新的组，需要提供 CreateGroupDTO 数据结构
///
/// 请求路径: POST /api/groups
#[post("/api/groups")]
async fn api_create_group(
    payload: web::Json<file_classification_core::model::models::CreateGroupDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 获取请求数据
    let group_dto = payload.into_inner();

    // 调用服务层创建新组
    match create_group(&mut conn, &group_dto) {
        Ok(group) =>
        // 构造成功响应，返回创建的组信息
            {
                Ok(HttpResponse::Created().json(ApiResponse::from(group)))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 根据条件批量更新组
///
/// 根据提供的条件数组和更新数据批量更新组
///
/// 请求路径: PUT /api/groups/update/by-conditions
#[put("/api/groups/update/by-conditions")]
async fn api_update_groups_by_conditions(
    payload: web::Json<(Vec<GroupCondition>, UpdateGroupDTO)>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析请求参数
    let (conditions, update_dto) = payload.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据条件批量更新组
    match update_groups_by_conditions(&mut conn, conditions, update_dto) {
        Ok(count) =>
        // 构造成功响应，包含更新记录数
            {
                Ok(HttpResponse::Ok().json(json!({
					"success": true,
					"message": format!("成功更新 {} 条记录", count),
					"count": count
			})))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 根据ID删除组
///
/// 根据组ID删除指定组
///
/// 请求路径: DELETE /api/groups/{id}
#[delete("/api/groups/{id}")]
async fn api_delete_group_by_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的组ID
    let group_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层删除指定组
    match delete_group(&mut conn, group_id) {
        Ok(_) =>
        // 构造成功响应
            {
                Ok(HttpResponse::Ok().json(json!({
					"success": true,
					"message": "组删除成功"
			})))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 根据文件ID获取组列表
///
/// 根据文件ID获取关联的所有组
///
/// 请求路径: GET /api/groups/file/{file_id}
#[get("/api/groups/file/{file_id}")]
async fn api_list_groups_by_file_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的文件ID
    let file_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据文件ID查询组
    match select_group_by_file_id(&mut conn, file_id) {
        Ok(groups) => {
            let count = groups.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(groups),
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

/// 根据标签ID获取组列表
///
/// 根据标签ID获取关联的所有组
///
/// 请求路径: GET /api/groups/tag/{tag_id}
#[get("/api/groups/tag/{tag_id}")]
async fn api_list_groups_by_tag_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的标签ID
    let tag_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据标签ID查询组
    match select_group_by_tag_id(&mut conn, tag_id) {
        Ok(groups) => {
            let count = groups.len();
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(groups),
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

/// 根据ID更新组
///
/// 根据组ID更新指定组信息
///
/// 请求路径: PUT /api/groups/{id}
#[put("/api/groups/{id}")]
async fn api_update_group_by_id(
    path: web::Path<i32>,
    update_dto: web::Json<file_classification_core::model::models::UpdateGroupDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 获取路径参数中的组ID
    let group_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层更新指定组
    match update_group_by_id(&mut conn, group_id, update_dto.into_inner()) {
        Ok(count) =>
        // 构造成功响应，包含更新记录数
            {
                Ok(HttpResponse::Ok().json(json!({
                    "success": true,
                    "message": format!("成功更新 {} 条记录", count),
                    "count": count
                })))
            }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 根据条件批量删除组
///
/// 根据提供的条件数组批量删除组
///
/// 请求路径: DELETE /api/groups/delete/by-conditions
#[delete("/api/groups/delete/by-conditions")]
async fn api_delete_groups_by_conditions(
    conditions: web::Json<Vec<file_classification_core::model::models::GroupCondition>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据条件批量删除组
    match delete_groups_by_conditions(&mut conn, conditions.into_inner()) {
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

/// 获取组的树状结构
///
/// 根据组ID获取指定组的完整树状结构
///
/// 请求路径: GET /api/groups/{id}/tree
#[get("/api/groups/{id}/tree")]
async fn api_get_group_tree(path: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse> {
    // 获取路径参数中的组ID
    let group_id = path.into_inner();

    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层获取组的树状结构
    match get_group_tree(&mut conn, group_id) {
        Ok(tree) => {
            // 构造成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(tree),
                message: None,
                count: Some(1),
            }))
        }
        // 错误处理
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
        ),
    }
}

/// 带选项地根据条件搜索组
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/groups/search/by-conditions-with-options
#[get("/api/groups/search/by-conditions-with-options")]
async fn api_list_groups_by_conditions_with_options(
    query: web::Query<(
        Vec<GroupCondition>,
        file_classification_core::model::models::GroupQueryOptions,
    )>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let (conditions, options) = query.into_inner();

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行带选项的查询
    match select_groups_by_conditions_with_options(&mut conn, conditions, options) {
        Ok(groups) => {
            let count = groups.len();

            // 返回成功响应
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(groups),
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

/// 根据ID列表批量删除组
///
/// 根据组ID列表批量删除指定组
///
/// 请求路径: DELETE /api/groups/delete/by-ids
#[delete("/api/groups/delete/by-ids")]
async fn api_delete_groups_by_ids(
    group_ids: web::Json<Vec<i32>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 从连接池获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 调用服务层根据ID列表批量删除组
    match delete_groups_by_ids(&mut conn, group_ids.into_inner()) {
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

/// 根据过滤条件分页搜索组
///
/// 此接口允许客户端传递分页参数，以分页方式检索数据
///
/// 请求路径: GET /api/groups/search/by-filter-with-pagination
#[get("/api/groups/search/by-filter-with-pagination")]
async fn api_list_groups_by_filter_with_pagination(
    query: web::Query<std::collections::HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let query_map = query.into_inner();
    let filter_str = query_map.get("filter").cloned().unwrap_or_default();
    let options_str = query_map.get("options").cloned().unwrap_or_default();

    let filter: GroupFilter = serde_json::from_str(&filter_str).unwrap_or_else(|_| GroupFilter {
        id: None,
        name: None,
        reference_count: None,
        is_primary: None,
        click_count: None,
        share_count: None,
        create_time: None,
        modify_time: None,
    });
    let options: file_classification_core::model::models::GroupQueryOptions = serde_json::from_str(&options_str)?;

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行分页查询
    match select_groups_by_filter_with_pagination(&mut conn, filter, options) {
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

/// 根据条件分页搜索组
///
/// 此接口允许客户端传递条件和分页参数，以分页方式检索数据
///
/// 请求路径: GET /api/groups/search/by-conditions-with-pagination
#[get("/api/groups/search/by-conditions-with-pagination")]
async fn api_list_groups_by_conditions_with_pagination(
    query: web::Query<std::collections::HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // 解析查询参数
    let query_map = query.into_inner();
    let conditions_str = query_map.get("conditions").cloned().unwrap_or_default();
    let options_str = query_map.get("options").cloned().unwrap_or_default();

    let conditions: Vec<GroupCondition> = serde_json::from_str(&conditions_str).unwrap_or_else(|_| Vec::new());
    let options: file_classification_core::model::models::GroupQueryOptions = serde_json::from_str(&options_str)?;

    // 获取数据库连接
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // 执行分页查询
    match select_groups_by_conditions_with_pagination(&mut conn, conditions, options) {
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