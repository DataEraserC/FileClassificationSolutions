mod handlers;
mod utils;

use crate::utils::logger::setup_logger;
use crate::utils::server;
use crate::utils::app_config::AppConfig;
use log::error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志记录器
    if let Err(e) = setup_logger() {
        eprintln!("日志系统初始化失败: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "日志系统初始化失败"));
    }

    // 加载应用配置
    let config = match AppConfig::load() {
        Ok(config) => config,
        Err(e) => {
            error!("应用配置加载失败: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "应用配置加载失败"));
        }
    };
    
    // 运行数据库迁移
    if let Err(e) = config.run_migrations() {
        error!("数据库迁移失败: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "数据库迁移失败"));
    }
    
    // 启动服务器
    server::start_server(config).await
}