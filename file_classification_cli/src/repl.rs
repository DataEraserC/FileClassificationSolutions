// repl.rs
// REPL模式的实现，低耦合，只依赖handlers和helpers

use clap::Parser;
use rustyline::DefaultEditor;
use std::error::Error;
use std::path::Path;

use file_classification_core::utils::database::AnyConnection;

use crate::cli::Cli;
use crate::context::Context;
use crate::handlers::{handle_command, handle_simplified_command};
use crate::helpers::{print_context, print_help};

/// 运行REPL模式
pub fn run_repl(conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
	let mut rl = DefaultEditor::new()?;
	println!("欢迎来到文件分类 REPL 环境！");
	println!("输入 'help' 查看可用命令，输入 'exit' 或 'quit' 退出。");

	// 加载历史记录
	let history_path = Path::new(".file_classification_history");
	if history_path.exists() {
		let _ = rl.load_history(history_path);
	}

	loop {
		let prompt = match context.selected_file_id {
			Some(id) => format!("file[{}]>> ", id),
			None => match context.selected_group_id {
				Some(id) => format!("group[{}]>> ", id),
				None => match context.selected_tag_id {
					Some(id) => format!("tag[{}]>> ", id),
					None => ">> ".to_string(),
				},
			},
		};

		let readline = rl.readline(&prompt);
		match readline {
			Ok(line) => {
				let line = line.trim();
				if line.is_empty() {
					continue;
				}

				// 添加到历史记录
				let _ = rl.add_history_entry(line);

				// 处理特殊命令
				if line == "exit" || line == "quit" {
					break;
				}
				if line == "help" {
					print_help();
					continue;
				}
				if line == "context" {
					print_context(context);
					continue;
				}
				if line == "clear" {
					print!("\x1B[2J\x1B[1;1H"); // 清屏
					continue;
				}

				// 跳过注释行
				if line.starts_with("#") {
					continue;
				}

				if line.starts_with("!") && line.len() > 1 {
					// 执行系统命令
					let cmd = &line[1..];
					match std::process::Command::new("cmd").args(&["/C", cmd]).status() {
						Ok(_) => {}
						Err(e) => eprintln!("执行系统命令失败: {}", e),
					}
					continue;
				}
				if line.starts_with("run ") && line.len() > 4 {
					// 执行脚本文件
					let script_file = &line[4..].trim();
					let script_cmd =
						Cli { command: crate::cli::Commands::Script { file: script_file.to_string() } };
					if let Err(e) = handle_command(script_cmd, conn, context) {
						eprintln!("执行脚本失败: {}", e);
					}
					continue;
				}

				// 支持分号分隔的多命令
				let commands: Vec<&str> =
					line.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
				for command in commands {
					// 尝试解析为标准命令
					let args = shlex::split(command).unwrap_or_default();
					let cli_args =
						std::iter::once("file_classification_cli".to_string()).chain(args.into_iter());

					match Cli::try_parse_from(cli_args) {
						Ok(cli) => {
							let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
								handle_command(cli, conn, context)
							}));
							match result {
								Ok(Ok(_)) => {}
								Ok(Err(e)) => eprintln!("命令执行出错: {}", e),
								Err(_) => eprintln!("命令执行时发生严重错误 (panic)！"),
							}
						}
						Err(e) => {
							if !handle_simplified_command(command, conn, context) {
								eprintln!("参数解析出错: {}", e);
							}
						}
					}
				}
			}
			Err(_) => {
				break;
			}
		}
	}

	// 保存历史记录
	let _ = rl.save_history(history_path);
	Ok(())
}
