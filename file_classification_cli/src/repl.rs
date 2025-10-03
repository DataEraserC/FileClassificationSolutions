use crate::cli_types::Cli;
use crate::context::{Context, print_context};
use crate::handlers::{handle_command, handle_file_command, handle_group_command, handle_tag_command};
use file_classification_core::utils::database::AnyConnection;
use rustyline::DefaultEditor;
use std::io::{self, Write};
use std::error::Error;

/// 打印帮助信息
pub fn print_help() {
    println!("可用命令:");
    println!("  help                - 显示此帮助信息");
    println!("  exit, quit          - 退出程序");
    println!("  context             - 显示当前上下文变量");
    println!("  clear               - 清屏");
    println!("  !<command>          - 执行系统命令");
    println!("  run <script_file>   - 执行脚本文件");
    println!("  eval <expression>   - 执行表达式");
    println!("  <key>=<value>       - 设置上下文变量");
    println!();
    println!("简化命令:");
    println!("  file create <path> [type]");
    println!("  file list");
    println!("  group create <name>");
    println!("  group list");
    println!("  tag create <name>");
    println!("  tag list");
    println!("  link file-group <file_id> <group_id>");
    println!("  link group-tag <group_id> <tag_id>");
    println!("  unlink file-group <file_id> <group_id>");
    println!("  unlink group-tag <group_id> <tag_id>");
    println!();
    println!("表达式语法:");
    println!("  file.list()");
    println!("  file.get(id)");
    println!("  file.create(path, type)");
    println!("  file.update(id, path, type)");
    println!("  file.delete(id)");
    println!("  group.list()");
    println!("  group.get(id)");
    println!("  group.create(name)");
    println!("  group.update(id, name)");
    println!("  group.delete(id)");
    println!("  tag.list()");
    println!("  tag.get(id)");
    println!("  tag.create(name)");
    println!("  tag.update(id, name)");
    println!("  tag.delete(id)");
    println!("  file.link(file_id, group_id)");
    println!("  file.unlink(file_id, group_id)");
    println!("  group.link(group_id, tag_id)");
    println!("  group.unlink(group_id, tag_id)");
}

/// 运行REPL模式
pub fn run_repl(conn: &mut AnyConnection) -> Result<(), Box<dyn Error>> {
    let mut context = Context::new();
    let mut rl = DefaultEditor::new()?;
    
    println!("欢迎使用文件分类系统 REPL 模式");
    println!("输入 'help' 获取帮助，输入 'exit' 或 'quit' 退出");
    
    loop {
        let prompt = "fc> ";
        io::stdout().flush()?;
        
        match rl.readline(prompt) {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                
                rl.add_history_entry(line)?;
                
                // 处理退出命令
                if line == "exit" || line == "quit" {
                    println!("再见！");
                    break;
                }
                
                // 处理变量赋值
                if line.contains('=') {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim();
                        let value = parts[1].trim();
                        context.set(key, value);
                        println!("设置变量: {} = {}", key, value);
                    }
                    continue;
                }
                
                // 处理帮助命令
                if line == "help" {
                    print_help();
                    continue;
                }
                
                // 处理上下文命令
                if line == "context" {
                    print_context(&context);
                    continue;
                }
                
                // 处理清屏命令
                if line == "clear" {
                    print!("\x1B[2J\x1B[1;1H"); // 清屏
                    continue;
                }
                
                // 执行系统命令
                if line.starts_with("!") && line.len() > 1 {
                    let cmd = &line[1..];
                    match std::process::Command::new("cmd")
                        .args(&["/C", cmd])
                        .status() {
                        Ok(status) => {
                            if !status.success() {
                                eprintln!("命令执行失败，退出代码: {:?}", status.code());
                            }
                        },
                        Err(e) => eprintln!("执行系统命令失败: {}", e),
                    }
                    continue;
                }
                
                // 执行脚本文件
                if line.starts_with("run ") && line.len() > 4 {
                    let script_file = &line[4..].trim();
                    let script_cmd = Cli { command: crate::cli_types::Commands::Script { file: script_file.to_string() } };
                    if let Err(e) = handle_command(script_cmd, conn, &mut context) {
                        eprintln!("执行脚本失败: {}", e);
                    }
                    continue;
                }
                
                // 解释器模式 - 执行表达式
                if line.starts_with("eval ") && line.len() > 5 {
                    let expr = &line[5..].trim();
                    match crate::eval_interpreter::eval_expression(expr, conn, &mut context) {
                        Ok(_) => {},
                        Err(e) => eprintln!("表达式执行失败: {}", e),
                    }
                    continue;
                }
                
                // 直接执行表达式（无需eval前缀）
                if line.contains(".") && (
                    line.contains("list") || line.contains("get") || 
                    line.contains("create") || line.contains("update") || 
                    line.contains("delete") || line.contains("link") || 
                    line.contains("unlink")
                ) {
                    match crate::eval_interpreter::eval_expression(line, conn, &mut context) {
                        Ok(_) => {},
                        Err(e) => eprintln!("表达式执行失败: {}", e),
                    }
                    continue;
                }

                let args = shlex::split(line).unwrap_or_default();
                let cli_args = std::iter::once("file_classification_cli".to_string()).chain(args.into_iter().map(|s| s.to_string()));

                match clap::Parser::try_parse_from(cli_args) as Result<Cli, _> {
                    Ok(cli) => {
                        let command = cli.command;

                        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            handle_command(Cli { command }, conn, &mut context)
                        }));
                        match result {
                            Ok(Ok(_)) => {},
                            Ok(Err(e)) => eprintln!("命令执行出错: {}", e),
                            Err(_) => eprintln!("命令执行时发生严重错误 (panic)！"),
                        }
                    }
                    Err(e) => {
                        // 尝试解析为简化命令
                    if line.starts_with("file ") || line.starts_with("group ") || 
                       line.starts_with("tag ") || line.starts_with("link ") || 
                       line.starts_with("unlink ") {
                        // 简化命令处理
                        let parts: Vec<&str> = line.splitn(2, ' ').collect();
                        let cmd = parts[0].trim();
                        let args = if parts.len() > 1 { parts[1].trim() } else { "" };
                        
                        match cmd {
                            "file" => handle_file_command(args, conn, &mut context)?,
                            "group" => handle_group_command(args, conn, &mut context)?,
                            "tag" => handle_tag_command(args, conn, &mut context)?,
                            "link" => handle_link_command(args, conn, &mut context)?,
                            "unlink" => handle_unlink_command(args, conn, &mut context)?,
                            _ => eprintln!("参数解析出错"),
                        }
                    } else {
                        eprintln!("参数解析出错: {}", e);
                    }
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok(())
}