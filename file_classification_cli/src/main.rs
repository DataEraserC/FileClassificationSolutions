// main.rs
// 主入口文件，负责解析CLI参数并分派到相应处理函数

use std::error::Error;

use clap::Parser;

use crate::cli::Cli;
use crate::context::Context;
use crate::handlers::handle_command;
use crate::repl::run_repl;
use file_classification_core::utils::database;
// 引入环境变量加载工具
use file_classification_common::env_loader::load_env_file;

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

  // 加载环境变量文件
  if let Err(e) = load_env_file() {
    eprintln!("加载环境变量文件失败: {}", e);
  }

  // 从环境变量中获取数据库连接URL和数据库类型
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let database_type = std::env::var("DATABASE_TYPE").expect("DATABASE_TYPE must be set");

  let mut conn = match database::establish_connection(&database_url, &database_type) {
    Ok(conn) => conn,
    Err(e) => {
      eprintln!("数据库连接失败: {}", e);
      return Err(Box::new(e));
    }
  };

  // 运行待处理的数据库迁移
  if let Err(e) = database::run_pending_migrations(&mut conn, &database_type) {
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
