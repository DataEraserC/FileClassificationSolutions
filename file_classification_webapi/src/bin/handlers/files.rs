use crate::utils::database::DbPool;
use crate::utils::models::{ApiError, ApiResponse};
use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use file_classification_core::service::files::{
	create_file, delete_files_by_conditions, select_file_by_group_id,
	select_files_by_conditions_with_options,
};
use file_classification_core::{
	model::models::{FileCondition, FileFilter, UpdateFileDTO},
	service::files::{
		delete_file, select_files_by_conditions, select_files_by_filter, update_files_by_conditions,
	}
	,
};
use serde_json::json;

/// 根据过滤条件获取文件列表
///
/// 根据过滤条件获取文件列表，默认最多返回100条记录
///
/// 请求路径: GET /api/files/filter
#[get("/api/files/filter")]
async fn api_list_files_by_filter(
	query: web::Query<FileFilter>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 从连接池获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用服务层查询文件列表
	match select_files_by_filter(&mut conn, query.into_inner(), 100) {
		Ok(files) => {
			let count = files.len();
			// 构造成功响应
			Ok(HttpResponse::Ok().json(ApiResponse {
				success: true,
				data: Some(files),
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

	// 构造条件数组，只包含当前文件ID
	let conditions = vec![file_classification_core::model::models::FileCondition::Id(file_id)];

	// 调用服务层根据ID查询文件
	match select_files_by_conditions(&mut conn, conditions, Some(1)) {
		Ok(files) => {
			if files.is_empty() {
				// 未找到文件时返回404
				Ok(
					HttpResponse::NotFound()
						.json(ApiError { success: false, message: "文件未找到".to_string() }),
				)
			} else {
				// 构造成功响应，返回单个文件
				Ok(HttpResponse::Ok().json(ApiResponse {
					success: true,
					data: Some(&files[0]),
					message: None,
					count: Some(1),
				}))
			}
		}
		// 错误处理
		Err(e) => Ok(
			HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
		),
	}
}

/// 根据条件搜索文件
///
/// 根据提供的条件数组搜索文件，最多返回100条记录
///
/// 请求路径: POST /api/files/search/by-conditions
#[post("/api/files/search/by-conditions")]
async fn api_list_files_by_conditions(
	conditions: web::Json<Vec<FileCondition>>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 从连接池获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用服务层根据条件查询文件
	match select_files_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
		Ok(files) => {
			let count = files.len();
			// 构造成功响应
			Ok(HttpResponse::Ok().json(ApiResponse {
				success: true,
				data: Some(files),
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
			Ok(HttpResponse::Ok().json(json!({
					"success": true,
					"message": "文件删除成功"
			})))
		}
		// 错误处理
		Err(e) => Ok(
			HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
		),
	}
}

/// 根据组ID获取文件列表
///
/// 根据文件组ID获取该组下的所有文件
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
			Ok(HttpResponse::Ok().json(ApiResponse {
				success: true,
				data: Some(files),
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

	// 构造条件数组，只包含当前文件ID
	let conditions = vec![file_classification_core::model::models::FileCondition::Id(file_id)];

	// 调用服务层更新指定文件
	match update_files_by_conditions(&mut conn, conditions, update_dto.into_inner()) {
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
			Ok(HttpResponse::Ok().json(ApiResponse {
				success: true,
				data: Some(files),
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
			Ok(HttpResponse::Created().json(ApiResponse::from(file)))
		}
		// 错误处理
		Err(e) => Ok(
			HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
		),
	}
}
