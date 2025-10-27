use crate::utils::database::DbPool;
use crate::utils::models::{ApiError, ApiResponse};
use actix_web::{delete, get, post, web, HttpResponse, Result};
use file_classification_core::model::models::GroupTagDTO;
use file_classification_core::service::group_tag::{
	delete_group_tags_by_conditions, delete_group_tags_by_dtos, select_group_tags_by_conditions_with_options, select_group_tags_by_filter, select_group_tags_by_filter_with_options, select_group_tags_by_filter_with_pagination, select_group_tags_by_conditions_with_pagination,
};
use file_classification_core::{
	model::models::{GroupTagCondition, GroupTagFilter, PaginationResult},
	service::group_tag::{
		create_group_tag, delete_group_tag_by_dto, select_group_tags_by_conditions,
	}
	,
};
use serde_json::json;

/// 根据过滤条件搜索组标签关联
///
/// 接收查询参数，支持group_id和tag_id过滤条件
///
/// 请求路径: GET /api/group-tags/filter
#[get("/api/group-tags/filter")]
async fn api_list_group_tags_by_filter(
	web::Query(filter): web::Query<GroupTagFilter>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 从连接池获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用核心服务层的方法执行查询，并限制最大结果数为 100
	match select_group_tags_by_filter(&mut conn, filter, 100) {
		Ok(group_tags) => {
			let count = group_tags.len();

			// 构造成功的响应对象并返回
			Ok(HttpResponse::Ok().json(ApiResponse {
				success: true,
				data: Some(group_tags),
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

/// 根据条件搜索组标签关联
///
/// 接收一个 JSON 数组作为请求体，数组中的每个元素都是一个 GroupTagCondition 类型的对象，
/// 代表一个查询条件。最多返回 100 条匹配的结果。
///
/// 请求路径: GET /api/group-tags/search/by-conditions
#[get("/api/group-tags/search/by-conditions")]
async fn api_list_group_tags_by_conditions(
	conditions: web::Json<Vec<GroupTagCondition>>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 从连接池获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用核心服务层的方法执行查询，并限制最大结果数为 100
	match select_group_tags_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
		Ok(group_tags) => {
			let count = group_tags.len();

			// 构造成功的响应对象并返回
			Ok(HttpResponse::Ok().json(ApiResponse {
				success: true,
				data: Some(group_tags),
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

/// 创建一个新的组标签关联
///
/// 客户端需提供完整的 GroupTagDTO 数据结构作为请求体。
///
/// 请求路径: POST /api/group-tags
#[post("/api/group-tags")]
async fn api_create_group_tag(
	group_tag_dto: web::Json<GroupTagDTO>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用服务层创建新记录
	match create_group_tag(&mut conn, group_tag_dto.into_inner()) {
		Ok(_) =>
		// 成功时返回 Created 状态码及提示信息
		{
			Ok(HttpResponse::Created().json(json!({
					"success": true,
					"message": "组标签关联创建成功"
			})))
		}

		// 错误处理
		Err(e) => Ok(
			HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
		),
	}
}

/// 删除指定的组标签关联
///
/// 需要提供完整的 GroupTagDTO 结构来标识要删除的具体项。
///
/// 请求路径: DELETE /api/group-tags
#[delete("/api/group-tags")]
async fn api_delete_group_tag(
	group_tag_dto: web::Json<GroupTagDTO>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用服务层删除记录
	match delete_group_tag_by_dto(&mut conn, &group_tag_dto.into_inner()) {
		Ok(_) =>
		// 成功时返回 OK 状态码及确认消息
		{
			Ok(HttpResponse::Ok().json(json!({
					"success": true,
					"message": "组标签关联删除成功"
			})))
		}

		// 错误处理
		Err(e) => Ok(
			HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
		),
	}
}

/// 根据条件批量删除组标签关联
///
/// 客户端应发送一个包含多个 GroupTagCondition 的数组，所有满足这些条件的记录都将被删除。
///
/// 请求路径: DELETE /api/group-tags/delete/by-conditions
#[delete("/api/group-tags/delete/by-conditions")]
async fn api_delete_group_tags_by_conditions(
	conditions: web::Json<Vec<file_classification_core::model::models::GroupTagCondition>>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用服务层执行批量删除操作
	match delete_group_tags_by_conditions(&mut conn, conditions.into_inner()) {
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

/// 带选项地根据条件搜索组标签关联
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/group-tags/search/by-conditions-with-options
#[get("/api/group-tags/search/by-conditions-with-options")]
async fn api_list_group_tags_by_conditions_with_options(
	query: web::Query<(
		Vec<GroupTagCondition>,
		file_classification_core::model::models::GroupTagQueryOptions,
	)>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 解析查询参数
	let (conditions, options) = query.into_inner();

	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 执行带选项的查询
	match select_group_tags_by_conditions_with_options(&mut conn, conditions, options) {
		Ok(group_tags) => {
			let count = group_tags.len();

			// 返回成功响应
			Ok(HttpResponse::Ok().json(ApiResponse {
				success: true,
				data: Some(group_tags),
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

/// 带选项地根据过滤条件搜索组标签关联
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/group-tags/search/by-filter-with-options
#[get("/api/group-tags/search/by-filter-with-options")]
async fn api_list_group_tags_by_filter_with_options(
	query: web::Query<(GroupTagFilter, file_classification_core::model::models::GroupTagQueryOptions)>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 解析查询参数
	let (filter, options) = query.into_inner();

	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 执行带选项的查询
	match select_group_tags_by_filter_with_options(&mut conn, filter, options) {
		Ok(group_tags) => {
			let count = group_tags.len();

			// 返回成功响应
			Ok(HttpResponse::Ok().json(ApiResponse {
				success: true,
				data: Some(group_tags),
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

/// 根据过滤条件分页搜索组标签关联
///
/// 此接口允许客户端传递分页参数，以分页方式检索数据
///
/// 请求路径: GET /api/group-tags/search/by-filter-with-pagination
#[get("/api/group-tags/search/by-filter-with-pagination")]
async fn api_list_group_tags_by_filter_with_pagination(
	query: web::Query<std::collections::HashMap<String, String>>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 解析查询参数
	let query_map = query.into_inner();
	let filter_str = query_map.get("filter").cloned().unwrap_or_default();
	let options_str = query_map.get("options").cloned().unwrap_or_default();

	let filter: GroupTagFilter = serde_json::from_str(&filter_str).unwrap_or_else(|_| GroupTagFilter {
	    group_id: None,
	    tag_id: None,
	});
	let options: file_classification_core::model::models::GroupTagQueryOptions = serde_json::from_str(&options_str)?;

	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 执行分页查询
	match select_group_tags_by_filter_with_pagination(&mut conn, filter, options) {
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

/// 根据条件分页搜索组标签关联
///
/// 此接口允许客户端传递条件和分页参数，以分页方式检索数据
///
/// 请求路径: GET /api/group-tags/search/by-conditions-with-pagination
#[get("/api/group-tags/search/by-conditions-with-pagination")]
async fn api_list_group_tags_by_conditions_with_pagination(
	query: web::Query<std::collections::HashMap<String, String>>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 解析查询参数
	let query_map = query.into_inner();
	let conditions_str = query_map.get("conditions").cloned().unwrap_or_default();
	let options_str = query_map.get("options").cloned().unwrap_or_default();

	let conditions: Vec<GroupTagCondition> = serde_json::from_str(&conditions_str).unwrap_or_else(|_| Vec::new());
	let options: file_classification_core::model::models::GroupTagQueryOptions = serde_json::from_str(&options_str)?;

	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 执行分页查询
	match select_group_tags_by_conditions_with_pagination(&mut conn, conditions, options) {
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

/// 根据DTO列表批量删除组标签关联
///
/// 根据组标签关联DTO列表批量删除指定关联
///
/// 请求路径: DELETE /api/group-tags/delete/by-dtos
#[delete("/api/group-tags/delete/by-dtos")]
async fn api_delete_group_tags_by_dtos(
	dtos: web::Json<Vec<GroupTagDTO>>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 从连接池获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用服务层根据DTO列表批量删除组标签关联
	match delete_group_tags_by_dtos(&mut conn, dtos.into_inner()) {
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