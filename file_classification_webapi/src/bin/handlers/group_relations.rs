use crate::utils::database::DbPool;
use crate::utils::models::{ApiError, ApiResponse};
use actix_web::{delete, get, post, web, HttpResponse, Result};
use file_classification_core::model::models::GroupRelation;
use file_classification_core::service::group_relations::{
	delete_group_relations_by_conditions, select_group_relations_by_conditions_with_options,
};
use file_classification_core::{
	model::models::GroupRelationCondition,
	service::group_relations::{
		create_group_relation, delete_group_relation, select_group_relations_by_conditions,
	},
};
use serde_json::json;

/// 根据条件搜索组关系
///
/// 接收一个 JSON 数组作为请求体，数组中的每个元素都是一个 GroupRelationCondition 类型的对象，
/// 代表一个查询条件。最多返回 100 条匹配的结果。
///
/// 请求路径: GET /api/group-relations/search/by-conditions
#[get("/api/group-relations/search/by-conditions")]
async fn api_list_group_relations_by_conditions(
	conditions: web::Json<Vec<GroupRelationCondition>>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 从连接池获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用核心服务层的方法执行查询，并限制最大结果数为 100
	match select_group_relations_by_conditions(&mut conn, conditions.into_inner(), Some(100)) {
		Ok(group_relations) => {
			let count = group_relations.len();

			// 构造成功的响应对象并返回
			Ok(HttpResponse::Ok().json(ApiResponse {
				success: true,
				data: Some(group_relations),
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

/// 带选项地根据条件搜索组关系
///
/// 此接口允许客户端传递额外的查询选项（例如排序规则），以更灵活的方式检索数据。
///
/// 请求路径: GET /api/group-relations/search/by-conditions-with-options
#[get("/api/group-relations/search/by-conditions-with-options")]
async fn api_list_group_relations_by_conditions_with_options(
	query: web::Query<(
		Vec<GroupRelationCondition>,
		file_classification_core::model::models::GroupRelationQueryOptions,
	)>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 解析查询参数
	let (conditions, options) = query.into_inner();

	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 执行带选项的查询
	match select_group_relations_by_conditions_with_options(&mut conn, conditions, options) {
		Ok(group_relations) => {
			let count = group_relations.len();

			// 返回成功响应
			Ok(HttpResponse::Ok().json(ApiResponse {
				success: true,
				data: Some(group_relations),
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

/// 创建一个新的组关系
///
/// 客户端需提供完整的 GroupRelation 数据结构作为请求体。
///
/// 请求路径: POST /api/group-relations
#[post("/api/group-relations")]
async fn api_create_group_relation(
	group_relation_dto: web::Json<GroupRelation>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用服务层创建新记录
	match create_group_relation(&mut conn, group_relation_dto.into_inner()) {
		Ok(_) =>
		// 成功时返回 Created 状态码及提示信息
		{
			Ok(HttpResponse::Created().json(json!({
					"success": true,
					"message": "组关系创建成功"
			})))
		}

		// 错误处理
		Err(e) => Ok(
			HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
		),
	}
}

/// 删除指定的组关系
///
/// 需要提供完整的 GroupRelation 结构来标识要删除的具体项。
///
/// 请求路径: DELETE /api/group-relations
#[delete("/api/group-relations")]
async fn api_delete_group_relation(
	group_relation_dto: web::Json<GroupRelation>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用服务层删除记录
	match delete_group_relation(&mut conn, group_relation_dto.into_inner()) {
		Ok(_) =>
		// 成功时返回 OK 状态码及确认消息
		{
			Ok(HttpResponse::Ok().json(json!({
					"success": true,
					"message": "组关系删除成功"
			})))
		}

		// 错误处理
		Err(e) => Ok(
			HttpResponse::InternalServerError().json(ApiError { success: false, message: e.to_string() }),
		),
	}
}

/// 根据条件批量删除组关系
///
/// 客户端应发送一个包含多个 GroupRelationCondition 的数组，所有满足这些条件的记录都将被删除。
///
/// 请求路径: DELETE /api/group-relations/delete/by-conditions
#[delete("/api/group-relations/delete/by-conditions")]
async fn api_delete_group_relations_by_conditions(
	conditions: web::Json<Vec<file_classification_core::model::models::GroupRelationCondition>>,
	pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
	// 获取数据库连接
	let mut conn = pool.get().expect("Failed to get connection from pool");

	// 调用服务层执行批量删除操作
	match delete_group_relations_by_conditions(&mut conn, conditions.into_inner()) {
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
