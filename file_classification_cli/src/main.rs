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
    let mut conn = utils::database::establish_connection();
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
