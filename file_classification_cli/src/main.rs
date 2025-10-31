// main.rs
// 主入口文件，负责解析CLI参数并分派到相应处理函数

use std::error::Error;
use std::env;
use std::fs;
use std::path::Path;

use clap::Parser;

use crate::cli::Cli;
use crate::context::Context;
use crate::handlers::handle_command;
use crate::repl::run_repl;
use file_classification_core::utils;

mod cli;
mod context;
mod handlers;
mod helpers;
mod interactive;
mod parsers;
mod repl;

/// 加载环境变量文件
///
/// 加载顺序：
/// 1. 通过 ENV_FILE 环境变量指定的文件
/// 2. 回退到 .file_classification_env
/// 3. 如果以上都不存在，则创建带有默认配置的 .file_classification_env
fn load_env_file() -> Result<Option<String>, Box<dyn std::error::Error>> {
    // 首先检查是否通过环境变量指定了env文件
    let env_file = env::var("ENV_FILE").unwrap_or_else(|_| ".file_classification_env".to_string());
    
    // 尝试加载指定的env文件
    if Path::new(&env_file).exists() {
        dotenvy::dotenv_override().ok();
        dotenvy::from_filename_override(&env_file)?;
        Ok(Some(env_file))
    } else if env_file != ".file_classification_env" && Path::new(".file_classification_env").exists() {
        // 如果指定了自定义env文件但不存在，回退到.file_classification_env
        dotenvy::dotenv_override().ok();
        dotenvy::from_filename_override(".file_classification_env")?;
        Ok(Some(".file_classification_env".to_string()))
    } else {
        // 如果文件都不存在，创建默认的.file_classification_env
        create_default_env_file()?;
        dotenvy::dotenv_override().ok();
        dotenvy::from_filename_override(".file_classification_env")?;
        Ok(None)
    }
}

/// 创建默认的环境变量配置文件
fn create_default_env_file() -> Result<(), Box<dyn std::error::Error>> {
    let default_content = r#"# File Classification 系统配置文件
# 数据库配置
DATABASE_URL=file_classification.db
DATABASE_TYPE=sqlite

# Web API 配置
BIND_ADDRESS=127.0.0.1
BIND_PORT=8082

# 日志配置
RUST_LOG=info
RUST_LOG_FILE=debug

# CORS 配置
CORS_ENABLED=true
CORS_ORIGIN=http://localhost:8082
"#;
    
    fs::write(".file_classification_env", default_content)?;
    Ok(())
}

/// 主函数：解析命令行参数，初始化数据库连接和上下文，并处理命令
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    // 加载环境变量文件
    if let Err(e) = load_env_file() {
        eprintln!("加载环境变量文件失败: {}", e);
    }

    // 从环境变量中获取数据库连接URL和数据库类型
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_type = std::env::var("DATABASE_TYPE").expect("DATABASE_TYPE must be set");
    
    let mut conn = utils::database::establish_connection(&database_url, &database_type);

    // 运行待处理的数据库迁移
    if let Err(e) = utils::database::run_pending_migrations(&mut conn) {
        eprintln!("数据库迁移失败: {}", e);
        return Err(e);
    }

    let mut context = Context::new();

    match cli.command {
        cli::Commands::Repl => {
            if let Err(e) = run_repl(&mut conn, &mut context) {
                eprintln!("REPL Error: {}", e);
            }
        }
        command => {
            if let Err(e) = handle_command(cli::Cli { command }, &mut conn, &mut context) {
                eprintln!("Command Error: {}", e);
            }
        }
    }

    Ok(())
}