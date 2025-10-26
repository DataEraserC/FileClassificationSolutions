use actix_web::{middleware::Logger, web, App, HttpServer};
mod handlers;
mod utils;

use actix_files as fs;
use file_classification_core::utils::database::establish_connection;
use file_classification_core::utils::database::run_pending_migrations;
use std::path::Path;
use utils::database::establish_connection_pool;

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

	// 尝试多种方式定位 static 目录
	let static_dir = find_static_directory()?;
	println!("使用静态文件目录: {:?}", static_dir);

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
            // 静态文件服务 - 放在最后，作为兜底
            .service(fs::Files::new("/", &static_dir).index_file("index.html"))
	})
	.bind("127.0.0.1:8082")?
	.run()
	.await
}

/// 查找静态文件目录，支持从不同目录运行程序
fn find_static_directory() -> std::io::Result<std::path::PathBuf> {
    // 首先尝试从可执行文件所在目录查找
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let static_dir = exe_dir.join("static");
            if Path::new(&static_dir).exists() {
                return Ok(static_dir);
            }
        }
    }

    // 然后尝试从当前工作目录查找
    if let Ok(current_dir) = std::env::current_dir() {
        let static_dir = current_dir.join("static");
        if Path::new(&static_dir).exists() {
            return Ok(static_dir);
        }
        
        // 尝试从当前工作目录的子目录 file_classification_webapi 中查找
        let static_dir = current_dir.join("file_classification_webapi").join("static");
        if Path::new(&static_dir).exists() {
            return Ok(static_dir);
        }
    }

    // 如果都没找到，则返回默认路径并让后续逻辑处理错误
    if let Ok(current_dir) = std::env::current_dir() {
        let static_dir = current_dir.join("static");
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("静态文件目录不存在: {:?}", static_dir)))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "无法确定静态文件目录位置"))
    }
}