// main.rs
// 主入口文件，负责解析CLI参数并分派到相应处理函数

use std::error::Error;

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

/// 主函数：解析命令行参数，初始化数据库连接和上下文，并处理命令
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    // 加载 .env 文件中的环境变量
    dotenvy::dotenv().ok();

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