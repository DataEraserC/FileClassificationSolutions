use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse, Result};
use actix_files as fs;
mod handlers;
mod utils;

use file_classification_core::utils::database::establish_connection;
use file_classification_core::utils::database::run_pending_migrations;
use rust_embed::RustEmbed;
use std::path::Path;
use utils::database::establish_connection_pool;

// 嵌入静态资源
#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime.as_ref())
                .body(content.data)
        }
        None => {
            // Avoid recursive call by directly getting index.html
            if path == "index.html" {
                return HttpResponse::NotFound().body("404 Not Found");
            }
            handle_embedded_file("index.html")
        }
    }
}

async fn index_handler() -> HttpResponse {
    handle_embedded_file("index.html")
}

async fn static_handler(path: web::Path<String>) -> HttpResponse {
    let path = path.into_inner();
    handle_embedded_file(&path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init();

	println!("正在启动文件分类 Web API...");

	// 运行待处理的数据库迁移
	let mut conn = establish_connection();
	if let Err(e) = run_pending_migrations(&mut conn) {
		eprintln!("数据库迁移失败: {}", e);
		return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
	}

	let pool = establish_connection_pool();

	// 在HttpServer::new中添加新的路由
	HttpServer::new(move || {
		App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            // API路由 - 放在静态文件服务之前以确保优先匹配
            // 文件相关路由
            .service(handlers::files::api_list_files_by_filter)
            .service(handlers::files::api_get_file_by_id)
            .service(handlers::files::api_list_files_by_conditions)
            .service(handlers::files::api_list_files_by_conditions_with_options)
            .service(handlers::files::api_list_files_by_group_id)
            .service(handlers::files::api_create_file)
            .service(handlers::files::api_update_files_by_conditions)
            .service(handlers::files::api_update_file_by_id)
            .service(handlers::files::api_delete_file_by_id)
            .service(handlers::files::api_delete_files_by_conditions)
            // 组相关路由
            .service(handlers::groups::api_list_groups_by_filter)
            .service(handlers::groups::api_get_group_by_id)
            .service(handlers::groups::api_list_groups_by_conditions)
            .service(handlers::groups::api_list_groups_by_conditions_with_options)
            .service(handlers::groups::api_list_groups_by_file_id)
            .service(handlers::groups::api_list_groups_by_tag_id)
            .service(handlers::groups::api_create_group)
            .service(handlers::groups::api_update_groups_by_conditions)
            .service(handlers::groups::api_update_group_by_id)
            .service(handlers::groups::api_delete_group_by_id)
            .service(handlers::groups::api_delete_groups_by_conditions)
            .service(handlers::groups::api_get_group_tree)
            // 标签相关路由
            .service(handlers::tags::api_list_tags_by_filter)
            .service(handlers::tags::api_get_tag_by_id)
            .service(handlers::tags::api_list_tags_by_conditions)
            .service(handlers::tags::api_list_tags_by_conditions_with_options)
            .service(handlers::tags::api_list_tags_by_group_id)
            .service(handlers::tags::api_create_tag)
            .service(handlers::tags::api_update_tags_by_conditions)
            .service(handlers::tags::api_update_tag_by_id)
            .service(handlers::tags::api_delete_tag_by_id)
            .service(handlers::tags::api_delete_tags_by_conditions)
            // 文件组关联路由
            .service(handlers::file_groups::api_list_file_groups_by_filter)
            .service(handlers::file_groups::api_list_file_groups_by_conditions)
            .service(handlers::file_groups::api_list_file_groups_by_conditions_with_options)
            .service(handlers::file_groups::api_create_file_group)
            .service(handlers::file_groups::api_delete_file_group)
            .service(handlers::file_groups::api_delete_file_groups_by_conditions)
            // 组标签关联路由
            .service(handlers::group_tags::api_list_group_tags_by_filter)
            .service(handlers::group_tags::api_list_group_tags_by_conditions)
            .service(handlers::group_tags::api_list_group_tags_by_conditions_with_options)
            .service(handlers::group_tags::api_create_group_tag)
            .service(handlers::group_tags::api_delete_group_tag)
            .service(handlers::group_tags::api_delete_group_tags_by_conditions)
            // 组关系路由
            .service(handlers::group_relations::api_list_group_relations_by_filter)
            .service(handlers::group_relations::api_list_group_relations_by_conditions)
            .service(handlers::group_relations::api_list_group_relations_by_conditions_with_options)
            .service(handlers::group_relations::api_create_group_relation)
            .service(handlers::group_relations::api_delete_group_relation)
            .service(handlers::group_relations::api_delete_group_relations_by_conditions)
            // 静态文件服务 - 使用嵌入的资源
            .route("/", web::get().to(index_handler))
            .route("/{filename:.*}", web::get().to(static_handler))
	})
	.bind("127.0.0.1:8082")?
	.run()
	.await
}