use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
mod handlers;
mod utils;

use actix_cors::Cors;
use actix_files::NamedFile;
use log;
use std::env;
use std::path::PathBuf;

// 引入数据库连接相关类型
use file_classification_core::utils::database::{establish_connection, run_pending_migrations};
use crate::utils::database::establish_connection_pool;

// 创建 CORS 中间件
fn create_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
}

// 静态文件服务处理器
async fn index_handler() -> Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

// 静态资源处理器
async fn static_handler(path: web::Path<String>) -> Result<NamedFile> {
    let mut full_path = PathBuf::from("./static/");
    full_path.push(path.as_str());
    Ok(NamedFile::open(full_path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志系统
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // 输出日志等级信息
    let console_log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let file_log_level = env::var("RUST_LOG_FILE").unwrap_or_else(|_| "debug".to_string());
    log::info!("终端日志等级设置为: {}", console_log_level);
    log::info!("文件日志等级设置为: {}", file_log_level);

    log::info!("正在启动文件分类 Web API...");

    // 加载 .env 文件中的环境变量
    dotenvy::dotenv().ok();

    // 从环境变量中获取数据库连接URL和数据库类型
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_type = std::env::var("DATABASE_TYPE").expect("DATABASE_TYPE must be set");
    
    // 运行待处理的数据库迁移
    let mut conn = establish_connection(&database_url, &database_type);
    if let Err(e) = run_pending_migrations(&mut conn) {
        log::error!("数据库迁移失败: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
    }

    let pool = establish_connection_pool();

    // 输出当前工作目录
    if let Ok(current_dir) = std::env::current_dir() {
        log::info!("当前工作目录: {:?}", current_dir);
    }

    // 输出可执行文件路径
    if let Ok(exe_path) = std::env::current_exe() {
        log::info!("可执行文件路径: {:?}", exe_path);
    }

    // 获取服务器绑定配置
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_port = env::var("BIND_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(8082u16);

    let bind_info = format!("{}:{}", bind_address, bind_port);
    log::info!("服务器将绑定到: {}", bind_info);

    // 在HttpServer::new中添加新的路由
    HttpServer::new(move || {
        // 创建 CORS 中间件
        let cors = create_cors();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            // API路由 - 放在静态文件服务之前以确保优先匹配
            // 文件相关路由
            .service(handlers::files::api_list_files_by_filter)
            .service(handlers::files::api_list_files_by_filter_with_options)
            .service(handlers::files::api_list_files_by_filter_with_pagination)
            .service(handlers::files::api_list_files_by_conditions)
            .service(handlers::files::api_list_files_by_conditions_with_options)
            .service(handlers::files::api_list_files_by_conditions_with_pagination)
            .service(handlers::files::api_list_files_by_group_id)
            .service(handlers::files::api_get_file_by_id)
            .service(handlers::files::api_create_file)
            .service(handlers::files::api_update_files_by_conditions)
            .service(handlers::files::api_update_file_by_id)
            .service(handlers::files::api_delete_file_by_id)
            .service(handlers::files::api_delete_files_by_ids)
            .service(handlers::files::api_delete_files_by_conditions)
            // 组相关路由
            .service(handlers::groups::api_list_groups_by_filter)
            .service(handlers::groups::api_list_groups_by_filter_with_options)
            .service(handlers::groups::api_list_groups_by_filter_with_pagination)
            .service(handlers::groups::api_list_groups_by_conditions)
            .service(handlers::groups::api_list_groups_by_conditions_with_options)
            .service(handlers::groups::api_list_groups_by_conditions_with_pagination)
            .service(handlers::groups::api_list_groups_by_file_id)
            .service(handlers::groups::api_list_groups_by_tag_id)
            .service(handlers::groups::api_get_group_by_id)
            .service(handlers::groups::api_create_group)
            .service(handlers::groups::api_update_groups_by_conditions)
            .service(handlers::groups::api_update_group_by_id)
            .service(handlers::groups::api_delete_group_by_id)
            .service(handlers::groups::api_delete_groups_by_ids)
            .service(handlers::groups::api_delete_groups_by_conditions)
            .service(handlers::groups::api_get_group_tree)
            // 标签相关路由
            .service(handlers::tags::api_list_tags_by_filter)
            .service(handlers::tags::api_list_tags_by_filter_with_options)
            .service(handlers::tags::api_list_tags_by_filter_with_pagination)
            .service(handlers::tags::api_list_tags_by_conditions)
            .service(handlers::tags::api_list_tags_by_conditions_with_options)
            .service(handlers::tags::api_list_tags_by_conditions_with_pagination)
            .service(handlers::tags::api_list_tags_by_group_id)
            .service(handlers::tags::api_get_tag_by_id)
            .service(handlers::tags::api_create_tag)
            .service(handlers::tags::api_update_tags_by_conditions)
            .service(handlers::tags::api_update_tag_by_id)
            .service(handlers::tags::api_delete_tag_by_id)
            .service(handlers::tags::api_delete_tags_by_ids)
            .service(handlers::tags::api_delete_tags_by_conditions)
            // 文件组关联路由
            .service(handlers::file_groups::api_list_file_groups_by_filter)
            .service(handlers::file_groups::api_list_file_groups_by_filter_with_options)
            .service(handlers::file_groups::api_list_file_groups_by_filter_with_pagination)
            .service(handlers::file_groups::api_list_file_groups_by_conditions)
            .service(handlers::file_groups::api_list_file_groups_by_conditions_with_options)
            .service(handlers::file_groups::api_list_file_groups_by_conditions_with_pagination)
            .service(handlers::file_groups::api_create_file_group)
            .service(handlers::file_groups::api_delete_file_group)
            .service(handlers::file_groups::api_delete_file_groups_by_dtos)
            .service(handlers::file_groups::api_delete_file_groups_by_conditions)
            // 组标签关联路由
            .service(handlers::group_tags::api_list_group_tags_by_filter)
            .service(handlers::group_tags::api_list_group_tags_by_filter_with_options)
            .service(handlers::group_tags::api_list_group_tags_by_filter_with_pagination)
            .service(handlers::group_tags::api_list_group_tags_by_conditions)
            .service(handlers::group_tags::api_list_group_tags_by_conditions_with_options)
            .service(handlers::group_tags::api_list_group_tags_by_conditions_with_pagination)
            .service(handlers::group_tags::api_create_group_tag)
            .service(handlers::group_tags::api_delete_group_tag)
            .service(handlers::group_tags::api_delete_group_tags_by_dtos)
            .service(handlers::group_tags::api_delete_group_tags_by_conditions)
            // 组关系路由
            .service(handlers::group_relations::api_list_group_relations_by_filter)
            .service(handlers::group_relations::api_list_group_relations_by_filter_with_options)
            .service(handlers::group_relations::api_list_group_relations_by_filter_with_pagination)
            .service(handlers::group_relations::api_list_group_relations_by_conditions)
            .service(handlers::group_relations::api_list_group_relations_by_conditions_with_options)
            .service(handlers::group_relations::api_list_group_relations_by_conditions_with_pagination)
            .service(handlers::group_relations::api_create_group_relation)
            .service(handlers::group_relations::api_delete_group_relation)
            .service(handlers::group_relations::api_delete_group_relations_by_dtos)
            .service(handlers::group_relations::api_delete_group_relations_by_conditions)
            // 静态文件服务 - 使用嵌入的资源
            .route("/", web::get().to(index_handler))
            .route("/{filename:.*}", web::get().to(static_handler))
    })
        .bind(&bind_info)?
        .run()
        .await
}