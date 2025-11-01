mod handlers;
mod utils;

use crate::utils::logger::setup_logger;
use crate::utils::server;
use crate::utils::app_config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志记录器
    setup_logger().expect("日志系统初始化失败");

    // 加载应用配置
    let config = AppConfig::load().expect("应用配置加载失败");
    
    // 运行数据库迁移
    config.run_migrations().expect("数据库迁移失败");
    
    // 启动服务器
    server::start_server(config).await
}