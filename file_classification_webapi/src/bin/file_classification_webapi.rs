use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse, Result};
use actix_files as fs;
mod handlers;
mod utils;

use file_classification_core::utils::database::establish_connection;
use file_classification_core::utils::database::run_pending_migrations;
use rust_embed::RustEmbed;
use std::path::Path;
use std::path::PathBuf;
use utils::database::establish_connection_pool;
use log;
use fern;
use chrono;
use dotenvy::dotenv;
use std::env;
use actix_cors::Cors;

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

/// 查找静态文件目录，支持从不同目录运行程序
fn find_static_directory() -> std::io::Result<PathBuf> {
    // 首先尝试从可执行文件所在目录查找
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let static_dir = exe_dir.join("static");
            if Path::new(&static_dir).exists() {
                log::info!("找到静态资源目录: {:?}", static_dir);
                return Ok(static_dir);
            }
        }
    }

    // 然后尝试从当前工作目录查找
    if let Ok(current_dir) = std::env::current_dir() {
        let static_dir = current_dir.join("static");
        if Path::new(&static_dir).exists() {
            log::info!("找到静态资源目录: {:?}", static_dir);
            return Ok(static_dir);
        }
        
        // 尝试从当前工作目录的子目录 file_classification_webapi 中查找
        let static_dir = current_dir.join("file_classification_webapi").join("static");
        if Path::new(&static_dir).exists() {
            log::info!("找到静态资源目录: {:?}", static_dir);
            return Ok(static_dir);
        }
    }

    // 如果都没找到，则返回默认路径并让后续逻辑处理错误
    if let Ok(current_dir) = std::env::current_dir() {
        let static_dir = current_dir.join("static");
        log::warn!("静态文件目录不存在: {:?}", static_dir);
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("静态文件目录不存在: {:?}", static_dir)))
    } else {
        log::error!("无法确定静态文件目录位置");
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "无法确定静态文件目录位置"))
    }
}

async fn index_handler() -> HttpResponse {
    // 首先尝试从物理目录提供文件
    match find_static_directory() {
        Ok(static_dir) => {
            let index_path = static_dir.join("index.html");
            if index_path.exists() {
                log::info!("从物理目录提供 index.html 文件: {:?}", index_path);
                return HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(std::fs::read(index_path).unwrap_or_else(|_| Vec::new()));
            }
        }
        Err(_) => {
            // 物理目录不存在，回退到嵌入资源
            log::info!("物理目录中未找到 index.html，回退到嵌入资源");
        }
    }
    
    // 回退到嵌入资源
    log::info!("从嵌入资源提供 index.html 文件");
    handle_embedded_file("index.html")
}

async fn static_handler(path: web::Path<String>) -> HttpResponse {
    let path = path.into_inner();
    
    // 首先尝试从物理目录提供文件
    match find_static_directory() {
        Ok(static_dir) => {
            let file_path = static_dir.join(&path);
            log::debug!("尝试从物理目录提供文件: {:?}, 请求路径: {}", file_path, path);
            
            if file_path.exists() && file_path.is_file() {
                // 确保请求的文件在 static 目录内，防止路径遍历攻击
                if let Ok(abs_file_path) = file_path.canonicalize() {
                    if abs_file_path.starts_with(&static_dir.canonicalize().unwrap_or(static_dir.clone())) {
                        log::info!("从物理目录提供文件: {:?}", file_path);
                        let content = std::fs::read(&file_path);
                        match content {
                            Ok(data) => {
                                let mime = mime_guess::from_path(&path).first_or_octet_stream();
                                return HttpResponse::Ok()
                                    .content_type(mime.as_ref())
                                    .body(data);
                            }
                            Err(e) => {
                                log::error!("读取文件失败 {:?}: {}", file_path, e);
                            }
                        }
                    } else {
                        log::warn!("文件路径不在静态目录内: {:?}", file_path);
                    }
                }
            } else {
                log::debug!("文件不存在或不是文件: {:?}", file_path);
            }
        }
        Err(e) => {
            // 物理目录不存在，回退到嵌入资源
            log::info!("查找物理目录失败: {}，回退到嵌入资源，请求路径: {}", e, path);
        }
    }
    
    // 回退到嵌入资源
    log::info!("从嵌入资源提供文件: {}", path);
    handle_embedded_file(&path)
}

/// 初始化日志系统，同时输出到终端和文件
fn setup_logger() -> Result<(), fern::InitError> {
    // 加载 .env 文件
    let _ = dotenv();
    
    // 创建 logs 目录（如果不存在）
    std::fs::create_dir_all("logs")?;
    
    // 获取当前日期时间作为日志文件名
    let local_time = chrono::Local::now();
    let date_str = local_time.format("%Y-%m-%d").to_string();
    let log_file_path = format!("logs/{}.log", date_str);
    
    // 获取终端日志等级
    let console_log_level = env::var("RUST_LOG")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(log::LevelFilter::Info);
        
    // 文件日志等级默认为 Debug
    let file_log_level = env::var("RUST_LOG_FILE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(log::LevelFilter::Debug);

    // 创建日志分发器
    let dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        });
        
    // 添加控制台输出
    let dispatch = dispatch.chain(
        fern::Dispatch::new()
            .level(console_log_level)
            .chain(std::io::stdout())
    );
    
    // 添加文件输出
    let dispatch = dispatch.chain(
        fern::Dispatch::new()
            .level(file_log_level)
            .chain(fern::log_file(log_file_path)?)
    );
    
    dispatch.apply()?;
        
    Ok(())
}

/// 创建 CORS 配置
fn create_cors() -> Cors {
    // 从环境变量获取 CORS 配置
    let cors_enabled = env::var("CORS_ENABLED")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true); // 默认启用 CORS
        
    let cors_origin = env::var("CORS_ORIGIN")
        .ok()
        .unwrap_or_else(|| "http://localhost:8082".to_string());

    if cors_enabled {
        log::info!("CORS 已启用，允许来源: {}", cors_origin);
        Cors::default()
            .allowed_origin(&cors_origin)
            .allowed_origin("http://127.0.0.1:8082")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600)
    } else {
        log::info!("CORS 已禁用");
        Cors::default()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志记录器
    setup_logger().expect("日志系统初始化失败");
    
    // 输出日志等级信息
    let console_log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let file_log_level = env::var("RUST_LOG_FILE").unwrap_or_else(|_| "debug".to_string());
    log::info!("终端日志等级设置为: {}", console_log_level);
    log::info!("文件日志等级设置为: {}", file_log_level);

    log::info!("正在启动文件分类 Web API...");

    // 运行待处理的数据库迁移
    let mut conn = establish_connection();
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
    .bind(&bind_info)?
    .run()
    .await
}