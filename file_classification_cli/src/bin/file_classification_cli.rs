use file_classification_core::utils::database::establish_connection;
use file_classification_cli::{
    cli_types::Cli,
    handlers::handle_command,
    repl::run_repl,
};
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 建立数据库连接
    let mut conn = establish_connection()?;
    
    // 解析命令行参数
    let cli = Cli::parse();
    
    match cli.command {
        file_classification_cli::cli_types::Commands::Repl => {
            // 进入REPL模式
            run_repl(&mut conn)?;
        }
        _ => {
            // 处理其他命令
            handle_command(cli, &mut conn, &mut file_classification_cli::context::Context::new())?;
        }
    }
    
    Ok(())
}
