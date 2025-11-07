use std::env;
use log;
use file_classification_common::env_loader::load_env_file;
use file_classification_core::utils::database::{establish_connection, run_pending_migrations};
use std::error::Error;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub database_type: String,
    pub bind_address: String,
    pub bind_port: u16,
    pub upload_path: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        // 加载环境变量文件
        if let Err(e) = load_env_file() {
            log::error!("加载环境变量文件失败: {}", e);
        }

        // 从环境变量中获取数据库连接URL和数据库类型
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_type = std::env::var("DATABASE_TYPE").expect("DATABASE_TYPE must be set");

        // 获取服务器绑定配置
        let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
        let bind_port = env::var("BIND_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(8082u16);
            
        // 获取上传路径配置，默认为 "uploads"
        let upload_path = env::var("UPLOAD_PATH").unwrap_or_else(|_| "uploads".to_string());

        Ok(AppConfig {
            database_url,
            database_type,
            bind_address,
            bind_port,
            upload_path,
        })
    }

    pub fn run_migrations(&self) -> Result<(), Box<dyn Error>> {
        let mut conn = establish_connection(&self.database_url, &self.database_type);
        if let Err(e) = run_pending_migrations(&mut conn, &self.database_type) {
            log::error!("数据库迁移失败: {}", e);
            return Err(e);
        }
        Ok(())
    }

    pub fn bind_info(&self) -> String {
        format!("{}:{}", self.bind_address, self.bind_port)
    }

    pub fn log_startup_info(&self) {
        // 输出日志等级信息
        let console_log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        let file_log_level = env::var("RUST_LOG_FILE").unwrap_or_else(|_| "debug".to_string());
        log::info!("终端日志等级设置为: {}", console_log_level);
        log::info!("文件日志等级设置为: {}", file_log_level);

        log::info!("正在启动文件分类 Web API...");

        // 输出当前工作目录
        if let Ok(current_dir) = std::env::current_dir() {
            log::info!("当前工作目录: {:?}", current_dir);
        }

        // 输出可执行文件路径
        if let Ok(exe_path) = std::env::current_exe() {
            log::info!("可执行文件路径: {:?}", exe_path);
        }

        log::info!("服务器将绑定到: {}", self.bind_info());
        log::info!("文件上传路径设置为: {}", self.upload_path);
    }
}