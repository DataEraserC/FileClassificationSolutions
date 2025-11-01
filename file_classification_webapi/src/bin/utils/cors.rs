use actix_cors::Cors;
use std::env;
use log;

/// 创建 CORS 配置
pub fn create_cors() -> Cors {
    // 从环境变量获取 CORS 配置
    let cors_enabled = env::var("CORS_ENABLED")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true); // 默认启用 CORS

    let cors_origin = env::var("CORS_ORIGIN")
        .ok()
        .unwrap_or_else(|| "http://localhost:8082".to_string());
    
    // 从环境变量获取端口，默认为 8082
    let port = env::var("BIND_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(8082u16);

    if cors_enabled {
        log::info!("CORS 已启用，允许来源: {}", cors_origin);
        Cors::default()
            .allowed_origin(&cors_origin)
            .allowed_origin(&format!("http://127.0.0.1:{}", port))
            .allowed_origin(&format!("http://localhost:{}", port))
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