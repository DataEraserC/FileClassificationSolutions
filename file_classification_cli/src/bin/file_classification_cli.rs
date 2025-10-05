use clap::{Parser, Subcommand};
use file_classification_core::model::models;
use file_classification_core::service;
use file_classification_core::utils::database::{establish_connection, AnyConnection};
use std::io::{self, Write};
use rustyline::DefaultEditor;
use shlex;
use std::fs::File as StdFile;
use std::io::Read;

#[derive(Parser)]
#[clap(name = "文件分类系统", version = "1.0", author = "Developer")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 文件相关操作
    File {
        #[clap(subcommand)]
        action: FileActions,
    },
    /// 组相关操作
    Group {
        #[clap(subcommand)]
        action: GroupActions,
    },
    /// 标签相关操作
    Tag {
        #[clap(subcommand)]
        action: TagActions,
    },
    /// 文件组关联操作
    FileGroup {
        #[clap(subcommand)]
        action: FileGroupActions,
    },
    /// 组标签关联操作
    GroupTag {
        #[clap(subcommand)]
        action: GroupTagActions,
    },
    /// 进入 REPL 模式
    Repl,
    /// 执行脚本文件
    Script {
        #[clap(short, long)]
        file: String,
    },
}

#[derive(Subcommand)]
enum FileActions {
    /// 创建文件
    Create {
        #[clap(short, long)]
        type_: Option<String>,
        #[clap(short, long)]
        path: Option<String>,
        #[clap(short, long)]
        group_id: Option<i32>,
    },
    /// 删除文件
    Delete {
        #[clap(short, long)]
        id: Option<i32>,
    },
    /// 查询文件（交互式）
    ListInteractive,
    /// 根据条件查询文件
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
        order_by: Vec<String>,
        #[clap(long)]
        limit: Option<i64>,
        #[clap(long)]
        offset: Option<i64>,
    },
    /// 根据组ID查询文件
    ListByGroupId {
        #[clap(short, long)]
        group_id: i64,
    },
    /// 更新文件
    UpdateByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(short, long)]
        path: Option<String>,
        #[clap(long)]
        type_: Option<String>,
        #[clap(long)]
        reference_count: Option<i32>,
        #[clap(long)]
        group_id: Option<i32>,
    },
    /// 删除文件（按条件）
    DeleteByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
    },
}

#[derive(Subcommand)]
enum GroupActions {
    /// 创建组
    Create {
        #[clap(short, long)]
        name: Option<String>,
    },
    /// 删除组
    Delete {
        #[clap(short, long)]
        id: Option<i32>,
    },
    /// 查询组（交互式）
    ListInteractive,
    /// 根据条件查询组
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
        order_by: Vec<String>,
        #[clap(long)]
        limit: Option<i64>,
        #[clap(long)]
        offset: Option<i64>,
    },
    /// 根据文件ID查询组
    ListByFileId {
        #[clap(short, long)]
        file_id: i32,
    },
    /// 根据标签ID查询组
    ListByTagId {
        #[clap(short, long)]
        tag_id: i32,
    },
    /// 更新组
    UpdateByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(short, long)]
        name: Option<String>,
        #[clap(long)]
        reference_count: Option<i32>,
        #[clap(long)]
        is_primary: Option<bool>,
        #[clap(long)]
        click_count: Option<i32>,
        #[clap(long)]
        share_count: Option<i32>,
    },
    /// 删除组（按条件）
    DeleteByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
    },
}

#[derive(Subcommand)]
enum TagActions {
    /// 创建标签
    Create {
        #[clap(short, long)]
        name: Option<String>,
    },
    /// 删除标签
    Delete {
        #[clap(short, long)]
        id: i32,
    },
    /// 查询标签（交互式）
    ListInteractive,
    /// 根据条件查询标签
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
        order_by: Vec<String>,
        #[clap(long)]
        limit: Option<i64>,
        #[clap(long)]
        offset: Option<i64>,
    },
    /// 根据组ID查询标签
    ListByGroupId {
        #[clap(short, long)]
        group_id: i64,
    },
    /// 更新标签
    UpdateByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(short, long)]
        name: Option<String>,
        #[clap(long)]
        reference_count: Option<i32>,
    },
    /// 删除标签（按条件）
    DeleteByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
    },
}

#[derive(Subcommand)]
enum FileGroupActions {
    /// 创建文件组关联
    Create {
        #[clap(short, long)]
        file_id: Option<i32>,
        #[clap(short, long)]
        group_id: Option<i32>,
    },
    /// 删除文件组关联
    Delete {
        #[clap(short, long)]
        file_id: Option<i32>,
        #[clap(short, long)]
        group_id: Option<i32>,
    },
    /// 查询文件组关联（交互式）
    ListInteractive,
    /// 根据条件查询文件组关联
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
        order_by: Vec<String>,
        #[clap(long)]
        limit: Option<i64>,
        #[clap(long)]
        offset: Option<i64>,
    },
    /// 删除文件组关联（按条件）
    DeleteByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
    },
}

#[derive(Subcommand)]
enum GroupTagActions {
    /// 创建组标签关联
    Create {
        #[clap(short, long)]
        group_id: Option<i32>,
        #[clap(short, long)]
        tag_id: Option<i32>,
    },
    /// 删除组标签关联
    Delete {
        #[clap(short, long)]
        group_id: Option<i32>,
        #[clap(short, long)]
        tag_id: Option<i32>,
    },
    /// 查询组标签关联（交互式）
    ListInteractive,
    /// 根据条件查询组标签关联
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
        order_by: Vec<String>,
        #[clap(long)]
        limit: Option<i64>,
        #[clap(long)]
        offset: Option<i64>,
    },
    /// 删除组标签关联（按条件）
    DeleteByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut conn = establish_connection();
    let mut context = Context::new();

    match cli.command {
        Commands::Repl => {
            if let Err(e) = run_repl(&mut conn, &mut context) {
                eprintln!("REPL Error: {}", e);
            }
        }
        command => {
            if let Err(e) = handle_command(Cli { command }, &mut conn, &mut context) {
                eprintln!("Command Error: {}", e);
            }
        }
    }

    Ok(())
}

struct Context {
    selected_file_id: Option<i32>,
    selected_group_id: Option<i32>,
    selected_tag_id: Option<i32>,
}

impl Context {
    fn new() -> Self {
        Self {
            selected_file_id: None,
            selected_group_id: None,
            selected_tag_id: None,
        }
    }
}

fn print_context(context: &Context) {
    println!("当前上下文:");
    println!("  选中的文件ID: {:?}", context.selected_file_id);
    println!("  选中的组ID: {:?}", context.selected_group_id);
    println!("  选中的标签ID: {:?}", context.selected_tag_id);
}

fn handle_simplified_command(
    line: &str,
    conn: &mut AnyConnection,
    context: &mut Context,
) -> bool {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.is_empty() {
        return true;
    }

    match parts[0] {
        "ls" => {
            // 根据当前上下文列出相关项目
            if let Some(file_id) = context.selected_file_id {
                println!("列出文件ID {} 相关的组:", file_id);
                let condition = format!("file_id={}", file_id);
                let args = vec!["file-group", "list-by-conditions", "-c", &condition];
                if let Ok(cmd) = Cli::try_parse_from(args) {
                    let _ = handle_command(cmd, conn, context);
                }
            } else if let Some(group_id) = context.selected_group_id {
                println!("列出组ID {} 相关的文件:", group_id);
                let group_id_str = group_id.to_string();
                let args = vec!["file", "list-by-group-id", "--group_id", &group_id_str];
                if let Ok(cmd) = Cli::try_parse_from(args) {
                    let _ = handle_command(cmd, conn, context);
                }

                println!("列出组ID {} 相关的标签:", group_id);
                let group_id_str = group_id.to_string();
                let args = vec!["tag", "list-by-group-id", "--group_id", &group_id_str];
                if let Ok(cmd) = Cli::try_parse_from(args) {
                    let _ = handle_command(cmd, conn, context);
                }
            } else if let Some(tag_id) = context.selected_tag_id {
                println!("列出标签ID {} 相关的组:", tag_id);
                let tag_id_str = tag_id.to_string();
                let args = vec!["group", "list-by-tag-id", "--tag_id", &tag_id_str];
                if let Ok(cmd) = Cli::try_parse_from(args) {
                    let _ = handle_command(cmd, conn, context);
                }
            } else {
                println!("没有选中任何项目，列出所有组:");
                let args = vec!["group", "list-interactive"];
                if let Ok(cmd) = Cli::try_parse_from(args) {
                    let _ = handle_command(cmd, conn, context);
                }
            }
            true
        }
        "cd" => {
            if parts.len() < 2 {
                println!("错误: 缺少ID参数");
                return true;
            }
            let id = parts[1].parse::<i32>().unwrap_or(0);
            if id <= 0 {
                println!("错误: 无效的ID");
                return true;
            }

            // 尝试确定ID类型并设置上下文
            // 简单实现: 假设是组ID
            context.selected_file_id = None;
            context.selected_tag_id = None;
            context.selected_group_id = Some(id);
            println!("已选择组ID: {}", id);
            true
        }
        "select" => {
            if parts.len() < 3 {
                println!("错误: 用法 select <type> <id>");
                return true;
            }

            let id = parts[2].parse::<i32>().unwrap_or(0);
            if id <= 0 {
                println!("错误: 无效的ID");
                return true;
            }

            match parts[1] {
                "file" => {
                    context.selected_file_id = Some(id);
                    context.selected_group_id = None;
                    context.selected_tag_id = None;
                    println!("已选择文件ID: {}", id);
                }
                "group" => {
                    context.selected_file_id = None;
                    context.selected_group_id = Some(id);
                    context.selected_tag_id = None;
                    println!("已选择组ID: {}", id);
                }
                "tag" => {
                    context.selected_file_id = None;
                    context.selected_group_id = None;
                    context.selected_tag_id = Some(id);
                    println!("已选择标签ID: {}", id);
                }
                _ => {
                    println!("错误: 未知类型 {}", parts[1]);
                }
            }
            true
        }
        "new" => {
            if parts.len() < 3 {
                println!("错误: 用法 new <type> <name/path>");
                return true;
            }

            match parts[1] {
                "group" => {
                    let args = vec!["group", "create", "--name", parts[2]];
                    if let Ok(cmd) = Cli::try_parse_from(args) {
                        let _ = handle_command(cmd, conn, context);
                    }
                }
                "tag" => {
                    let args = vec!["tag", "create", "--name", parts[2]];
                    if let Ok(cmd) = Cli::try_parse_from(args) {
                        let _ = handle_command(cmd, conn, context);
                    }
                }
                "file" => {
                    let mut args = vec![
                        "file",
                        "create",
                        "--path",
                        parts[2],
                        "--type",
                        "regular",
                    ];
                    let group_id_str: String;
                    if let Some(group_id) = context.selected_group_id {
                        args.push("--group_id");
                        group_id_str = group_id.to_string();
                        args.push(&group_id_str);
                    }
                    if let Ok(cmd) = Cli::try_parse_from(args) {
                        let _ = handle_command(cmd, conn, context);
                    }
                }
                _ => {
                    println!("错误: 未知类型 {}", parts[1]);
                }
            }
            true
        }
        "rm" => {
            if parts.len() < 3 {
                println!("错误: 用法 rm <type> <id>");
                return true;
            }

            let id = parts[2].parse::<i32>().unwrap_or(0);
            if id <= 0 {
                println!("错误: 无效的ID");
                return true;
            }

            match parts[1] {
                "file" => {
                    let args = vec!["file", "delete", "--id", parts[2]];
                    if let Ok(cmd) = Cli::try_parse_from(args) {
                        let _ = handle_command(cmd, conn, context);
                    }
                }
                "group" => {
                    let args = vec!["group", "delete", "--id", parts[2]];
                    if let Ok(cmd) = Cli::try_parse_from(args) {
                        let _ = handle_command(cmd, conn, context);
                    }
                }
                "tag" => {
                    let args = vec!["tag", "delete", "--id", parts[2]];
                    if let Ok(cmd) = Cli::try_parse_from(args) {
                        let _ = handle_command(cmd, conn, context);
                    }
                }
                _ => {
                    println!("错误: 未知类型 {}", parts[1]);
                }
            }
            true
        }
        _ => false,
    }
}

fn print_help() {
    println!("可用命令:");
    println!("  file create --type <type> --path <path> --group_id <id>     创建文件");
    println!("  file delete --id <id>                                       删除文件");
    println!("  file list-interactive                                       交互式查询文件");
    println!("  file list-by-conditions -c <conditions...>                  按条件查询文件");
    println!("  file list-by-group-id --group_id <id>                       按组ID查询文件");
    println!("  file update-by-conditions -c <conditions...>                按条件更新文件");
    println!("  file delete-by-conditions -c <conditions...>                按条件删除文件");
    println!("  group create --name <name>                                  创建组");
    println!("  group delete --id <id>                                      删除组");
    println!("  group list-interactive                                      交互式查询组");
    println!("  group list-by-conditions -c <conditions...>                 按条件查询组");
    println!("  group list-by-file-id --file_id <id>                        按文件ID查询组");
    println!("  group list-by-tag-id --tag_id <id>                          按标签ID查询组");
    println!("  group update-by-conditions -c <conditions...>               按条件更新组");
    println!("  group delete-by-conditions -c <conditions...>               按条件删除组");
    println!("  tag create --name <name>                                    创建标签");
    println!("  tag delete --id <id>                                        删除标签");
    println!("  tag list-interactive                                        交互式查询标签");
    println!("  tag list-by-conditions -c <conditions...>                   按条件查询标签");
    println!("  tag list-by-group-id --group_id <id>                        按组ID查询标签");
    println!("  tag update-by-conditions -c <conditions...>                 按条件更新标签");
    println!("  tag delete-by-conditions -c <conditions...>                 按条件删除标签");
    println!("  file-group create --file_id <id> --group_id <id>            创建文件-组关联");
    println!("  file-group delete --id <id>                                 删除文件-组关联");
    println!("  file-group list-by-conditions -c <conditions...>            按条件查询文件-组关联");
    println!("  group-tag create --group_id <id> --tag_id <id>              创建组-标签关联");
    println!("  group-tag delete --id <id>                                  删除组-标签关联");
    println!("  group-tag list-by-conditions -c <conditions...>             按条件查询组-标签关联");
    println!("");
    println!("简化命令:");
    println!("  ls                                                         列出当前上下文相关的项目");
    println!("  cd <id>                                                    切换当前上下文");
    println!("  select <type> <id>                                         选择特定类型和ID");
    println!("  new <type> <name/path>                                     快速创建新项目");
    println!("  rm <type> <id>                                             快速删除项目");
    println!("  context                                                    显示当前上下文");
    println!("  clear                                                      清屏");
    println!("  !<command>                                                 执行系统命令");
    println!("  run <script_file>                                          执行脚本文件");
    println!("  help                                                       显示此帮助信息");
    println!("  exit/quit                                                  退出 REPL 环境");
}

fn run_repl(
    conn: &mut AnyConnection,
    context: &mut Context,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    println!("欢迎来到文件分类 REPL 环境！");
    println!("输入 'help' 查看可用命令，输入 'exit' 或 'quit' 退出。");

    // 尝试加载历史记录
    let history_path = std::path::Path::new(".file_classification_history");
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
                    let script_cmd = Cli {
                        command: Commands::Script {
                            file: script_file.to_string(),
                        },
                    };
                    if let Err(e) = handle_command(script_cmd, conn, context) {
                        eprintln!("执行脚本失败: {}", e);
                    }
                    continue;
                }

                let args = shlex::split(line).unwrap_or_default();
                let cli_args = std::iter::once("file_classification_cli".to_string()).chain(args);

                match Cli::try_parse_from(cli_args) {
                    Ok(cli) => {
                        let command = cli.command;

                        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            handle_command(Cli { command }, conn, context)
                        }));
                        match result {
                            Ok(Ok(_)) => {},
                            Ok(Err(e)) => eprintln!("命令执行出错: {}", e),
                            Err(_) => eprintln!("命令执行时发生严重错误 (panic)！"),
                        }
                    }
                    Err(e) => {
                        // 尝试解析为简化命令
                        if !handle_simplified_command(line, conn, context) {
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

    // 保存历史记录
    let _ = rl.save_history(history_path);
    Ok(())
}

fn handle_command(
    command: Cli,
    conn: &mut AnyConnection,
    context: &mut Context,
) -> Result<(), Box<dyn std::error::Error>> {
    let command = command.command;
    match command {
        Commands::Script { file } => {
            let mut script_file = StdFile::open(&file)?;
            let mut content = String::new();
            script_file.read_to_string(&mut content)?;

            println!("执行脚本: {}", file);
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue; // 跳过空行和注释
                }

                println!("执行命令: {}", line);

                // 尝试解析为标准命令
                let args: Vec<&str> = line.split_whitespace().collect();
                if let Ok(cmd) = Cli::try_parse_from(args) {
                    if let Err(e) = handle_command(cmd, conn, context) {
                        println!("命令执行错误: {}", e);
                    }
                } else {
                    // 尝试作为简化命令处理
                    if !handle_simplified_command(line, conn, context) {
                        println!("无法解析命令: {}", line);
                    }
                }
            }
            println!("脚本执行完成");
        }
        Commands::Repl => {
            if let Err(e) = run_repl(conn, context) {
                eprintln!("REPL Error: {}", e);
            }
        }
        Commands::File { action } => match action {
            FileActions::Create {
                type_,
                path,
                group_id,
            } => {
                let type_ = type_.clone().unwrap_or_else(|| {
                    let mut input = String::new();
                    print!("请输入文件类型: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    input.trim().to_string()
                });

                let path = path.clone().unwrap_or_else(|| {
                    let mut input = String::new();
                    print!("请输入文件路径: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    input.trim().to_string()
                });

                let group_id = group_id.unwrap_or_else(|| {
                    let mut input = String::new();
                    print!("请输入组 ID: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    input.trim().parse().expect("无效的组 ID")
                });

                let dto = models::CreateFileDTO {
                    type_,
                    path,
                    group_id,
                };
                match service::files::create_file(conn, dto) {
                    Ok(count) => println!("成功创建文件，影响 {} 行", count),
                    Err(e) => eprintln!("创建文件失败: {:?}", e),
                }
            }
            FileActions::Delete { id } => {
                let file_id = if let Some(id) = id {
                    id
                } else if let Some(selected_id) = context.selected_file_id {
                    selected_id
                } else {
                    eprintln!("错误：未提供文件 ID，也未在上下文中选中任何文件。");
                    return Ok(());
                };

                let mut input = String::new();
                print!("确定要删除 ID 为 {} 的文件吗? (y/n): ", file_id);
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim().eq_ignore_ascii_case("y") {
                    match service::files::delete_file(conn, file_id) {
                        Ok(()) => {
                            println!("成功删除文件");
                            if context.selected_file_id == Some(file_id) {
                                context.selected_file_id = None;
                            }
                        }
                        Err(e) => eprintln!("删除文件失败: {:?}", e),
                    }
                } else {
                    println!("操作已取消");
                }
            }
            FileActions::ListInteractive => {
                list_files_interactive(conn, context);
            }
            FileActions::ListByConditions {
                conditions,
                order_by,
                limit,
                offset,
            } => {
                let conditions = parse_file_conditions(&conditions);
                let mut options = models::FileQueryOptions::default();
                options.limit = limit;
                options.offset = offset;
                options.order_by = parse_file_order_by(&order_by);

                match service::files::select_files_by_conditions_with_options(conn, conditions, options) {
                    Ok(results) => {
                        if results.is_empty() {
                            println!("未找到匹配的文件。");
                        } else {
                            println!("查询结果:");
                            for file in &results {
                                println!(
                                    "  - ID: {}, Type: {}, Path: {}, Group ID: {}",
                                    file.id, file.type_, file.path, file.group_id
                                );
                            }
                            // 将第一个结果的 ID 存储到上下文中
                            if let Some(first_file) = results.first() {
                                context.selected_file_id = Some(first_file.id);
                                println!(
                                    "\n提示：第一个文件的 ID ({}) 已被选中，可用于后续操作。",
                                    first_file.id
                                );
                            }
                        }
                    }
                    Err(e) => eprintln!("查询文件失败: {:?}", e),
                }
            }
            FileActions::ListByGroupId { group_id } => {
                match service::files::select_file_by_group_id(conn, group_id) {
                    Ok(files) => {
                        println!("查询结果 (共 {} 条记录):", files.len());
                        for file in files {
                            println!("{:?}", file);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            FileActions::UpdateByConditions {
                conditions,
                path,
                type_,
                reference_count,
                group_id,
            } => {
                let update_conditions = if conditions.is_empty() {
                    if let Some(selected_id) = context.selected_file_id {
                        vec![format!("id={}", selected_id)]
                    } else {
                        eprintln!("错误：未提供更新条件，也未在上下文中选中任何文件。");
                        return Ok(());
                    }
                } else {
                    conditions
                };

                let mut changes = models::UpdateFileDTO::default();
                if let Some(path) = path {
                    changes.path = Some(path);
                }
                if let Some(type_) = type_ {
                    changes.type_ = Some(type_);
                }
                if let Some(reference_count) = reference_count {
                    changes.reference_count = Some(reference_count);
                }
                if let Some(group_id) = group_id {
                    changes.group_id = Some(group_id);
                }

                match service::files::update_files_by_conditions(
                    conn,
                    parse_file_conditions(&update_conditions),
                    changes,
                ) {
                    Ok(count) => println!("成功更新 {} 个文件", count),
                    Err(e) => eprintln!("更新文件失败: {:?}", e),
                }
            }
            FileActions::DeleteByConditions { conditions } => {
                let conditions = parse_file_conditions(&conditions);
                match service::files::delete_files_by_conditions(conn, conditions) {
                    Ok(count) => println!("成功删除 {} 条记录", count),
                    Err(e) => eprintln!("删除失败: {:?}", e),
                }
            }
        },
        Commands::Group { action } => match action {
            GroupActions::Create { name } => {
                let name = name.clone().unwrap_or_else(|| {
                    let mut input = String::new();
                    print!("请输入组名称: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    input.trim().to_string()
                });

                match service::groups::create_group_by_name(conn, &name) {
                    Ok(count) => println!("成功创建组，影响 {} 行", count),
                    Err(e) => eprintln!("创建组失败: {:?}", e),
                }
            }
            GroupActions::Delete { id } => {
                let group_id = if let Some(id) = id {
                    id
                } else if let Some(selected_id) = context.selected_group_id {
                    selected_id
                } else {
                    eprintln!("错误：未提供组 ID，也未在上下文中选中任何组。");
                    return Ok(());
                };

                let mut input = String::new();
                print!("确定要删除 ID 为 {} 的组吗? (y/n): ", group_id);
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim().eq_ignore_ascii_case("y") {
                    match service::groups::delete_group(conn, group_id) {
                        Ok(count) => {
                            println!("成功删除组，影响 {} 行", count);
                            if context.selected_group_id == Some(group_id) {
                                context.selected_group_id = None;
                            }
                        }
                        Err(e) => eprintln!("删除组失败: {:?}", e),
                    }
                } else {
                    println!("操作已取消");
                }
            }
            GroupActions::ListInteractive => {
                list_groups_interactive(conn, context);
            }
            GroupActions::ListByConditions {
                conditions,
                order_by,
                limit,
                offset,
            } => {
                let conditions = parse_group_conditions(&conditions);
                let mut options = models::GroupQueryOptions::default();
                options.limit = limit;
                options.offset = offset;
                options.order_by = parse_group_order_by(&order_by);

                match service::groups::select_groups_by_conditions_with_options(
                    conn,
                    conditions,
                    options,
                ) {
                    Ok(results) => {
                        if results.is_empty() {
                            println!("未找到匹配的组。");
                        } else {
                            println!("查询结果:");
                            for group in &results {
                                println!("  - ID: {}, Name: {}", group.id, group.name);
                            }
                            // 将第一个结果的 ID 存储到上下文中
                            if let Some(first_group) = results.first() {
                                context.selected_group_id = Some(first_group.id);
                                println!(
                                    "\n提示：第一个组的 ID ({}) 已被选中，可用于后续操作。",
                                    first_group.id
                                );
                            }
                        }
                    }
                    Err(e) => eprintln!("查询组失败: {:?}", e),
                }
            }
            GroupActions::ListByFileId { file_id } => {
                match service::groups::select_group_by_file_id(conn, file_id) {
                    Ok(groups) => {
                        println!("查询结果 (共 {} 条记录):", groups.len());
                        for group in groups {
                            println!("{:?}", group);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            GroupActions::ListByTagId { tag_id } => {
                match service::groups::select_group_by_tag_id(conn, tag_id) {
                    Ok(groups) => {
                        println!("查询结果 (共 {} 条记录):", groups.len());
                        for group in groups {
                            println!("{:?}", group);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            GroupActions::UpdateByConditions {
                conditions,
                name,
                reference_count,
                is_primary,
                click_count,
                share_count,
            } => {
                let update_conditions = if conditions.is_empty() {
                    if let Some(selected_id) = context.selected_group_id {
                        vec![format!("id={}", selected_id)]
                    } else {
                        eprintln!("错误：未提供更新条件，也未在上下文中选中任何组。");
                        return Ok(());
                    }
                } else {
                    conditions
                };

                let mut changes = models::UpdateGroupDTO::default();
                if let Some(name) = name {
                    changes.name = Some(name);
                }
                if let Some(reference_count) = reference_count {
                    changes.reference_count = Some(reference_count);
                }
                if let Some(is_primary) = is_primary {
                    changes.is_primary = Some(is_primary);
                }
                if let Some(click_count) = click_count {
                    changes.click_count = Some(click_count);
                }
                if let Some(share_count) = share_count {
                    changes.share_count = Some(share_count);
                }

                match service::groups::update_groups_by_conditions(
                    conn,
                    parse_group_conditions(&update_conditions),
                    changes,
                ) {
                    Ok(count) => println!("成功更新 {} 个组", count),
                    Err(e) => eprintln!("更新组失败: {:?}", e),
                }
            }
            GroupActions::DeleteByConditions { conditions } => {
                let conditions = parse_group_conditions(&conditions);
                match service::groups::delete_groups_by_conditions(conn, conditions) {
                    Ok(count) => println!("成功删除 {} 条记录", count),
                    Err(e) => eprintln!("删除失败: {:?}", e),
                }
            }
        },
        Commands::Tag { action } => match action {
            TagActions::Create { name } => {
                let name = name.clone().unwrap_or_else(|| {
                    let mut input = String::new();
                    print!("请输入标签名称: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    input.trim().to_string()
                });

                match service::tags::create_tag_by_name(conn, &name) {
                    Ok(tag) => println!("成功创建标签: {:?}", tag),
                    Err(e) => eprintln!("创建标签失败: {:?}", e),
                }
            }
            TagActions::Delete { id } => {
                let tag_id = id;

                let mut input = String::new();
                print!("确定要删除 ID 为 {} 的标签吗? (y/n): ", tag_id);
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim().eq_ignore_ascii_case("y") {
                    match service::tags::delete_tag(conn, tag_id) {
                        Ok(count) => {
                            println!("成功删除标签，影响 {} 行", count);
                            if context.selected_tag_id == Some(tag_id) {
                                context.selected_tag_id = None;
                            }
                        }
                        Err(e) => eprintln!("删除标签失败: {:?}", e),
                    }
                } else {
                    println!("操作已取消");
                }
            }
            TagActions::ListInteractive => {
                list_tags_interactive(conn, context);
            }
            TagActions::ListByConditions {
                conditions,
                order_by,
                limit,
                offset,
            } => {
                let conditions = parse_tag_conditions(&conditions);
                let mut options = models::TagQueryOptions::default();
                options.limit = limit;
                options.offset = offset;
                options.order_by = parse_tag_order_by(&order_by);

                match service::tags::select_tags_by_conditions_with_options(
                    conn,
                    conditions,
                    options,
                ) {
                    Ok(tags) => {
                        println!("查询结果 (共 {} 条记录):", tags.len());
                        for tag in tags {
                            println!("{:?}", tag);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            TagActions::ListByGroupId { group_id } => {
                match service::tags::select_tag_by_group_id(conn, group_id) {
                    Ok(tags) => {
                        println!("查询结果 (共 {} 条记录):", tags.len());
                        for tag in tags {
                            println!("{:?}", tag);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            TagActions::UpdateByConditions {
                conditions,
                name,
                reference_count,
            } => {
                let mut conditions = parse_tag_conditions(&conditions);
                if conditions.is_empty() {
                    if let Some(group_id) = context.selected_group_id {
                        conditions.push(models::TagCondition::Id(group_id));
                    } else {
                        eprintln!("没有活动的标签，请先运行 'tag list' 或 'tag list-by-conditions' 选择一个标签");
                        return Ok(());
                    }
                }

                let name = name.clone().unwrap_or_else(|| {
                    let mut input = String::new();
                    print!("请输入新的标签名称 (留空则不修改): ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    input.trim().to_string()
                });

                let update_dto = models::UpdateTagDTO {
                    name: if name.is_empty() {
                        None
                    } else {
                        Some(name)
                    },
                    reference_count: reference_count,
                };
                match service::tags::update_tags_by_conditions(conn, conditions, update_dto) {
                    Ok(count) => println!("成功更新 {} 条记录", count),
                    Err(e) => eprintln!("更新失败: {:?}", e),
                }
            }
            TagActions::DeleteByConditions { conditions } => {
                let conditions = parse_tag_conditions(&conditions);
                match service::tags::delete_tags_by_conditions(conn, conditions) {
                    Ok(count) => println!("成功删除 {} 条记录", count),
                    Err(e) => eprintln!("删除失败: {:?}", e),
                }
            }
        },
        Commands::FileGroup { action } => match action {
            FileGroupActions::Create { file_id, group_id } => {
                let file_id = file_id.unwrap_or_else(|| {
                    let mut input = String::new();
                    print!("请输入文件 ID: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    input.trim().parse().expect("无效的文件 ID")
                });
                let group_id = group_id
                    .or(context.selected_group_id)
                    .unwrap_or_else(|| {
                        let mut input = String::new();
                        print!("请输入组 ID: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        input.trim().parse().expect("无效的组 ID")
                    });

                let dto = models::FileGroupDTO {
                    file_id,
                    group_id,
                };
                match service::file_group::create_file_group(conn, dto) {
                    Ok(dto) => println!("成功创建文件组关联: {:?}", dto),
                    Err(e) => eprintln!("创建文件组关联失败: {:?}", e),
                }
            }
            FileGroupActions::Delete { file_id, group_id } => {
                let file_id = file_id
                    .or(context.selected_file_id)
                    .unwrap_or_else(|| {
                        let mut input = String::new();
                        print!("请输入文件 ID: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        input.trim().parse().expect("无效的文件 ID")
                    });

                let group_id = group_id
                    .or(context.selected_group_id)
                    .unwrap_or_else(|| {
                        let mut input = String::new();
                        print!("请输入组 ID: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        input.trim().parse().expect("无效的组 ID")
                    });

                let mut input = String::new();
                print!(
                    "确定要删除文件 ID 为 {} 和组 ID 为 {} 的关联吗? (y/n): ",
                    file_id, group_id
                );
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim().eq_ignore_ascii_case("y") {
                    let dto = models::FileGroupDTO {
                        file_id,
                        group_id,
                    };
                    match service::file_group::delete_file_group(conn, dto) {
                        Ok(count) => println!("成功删除 {} 个文件组关联", count),
                        Err(e) => eprintln!("删除文件组关联失败: {:?}", e),
                    }
                } else {
                    println!("操作已取消");
                }
            }
            FileGroupActions::ListInteractive => {
                list_file_groups_interactive(conn, context);
            }
            FileGroupActions::ListByConditions {
                conditions,
                order_by,
                limit,
                offset,
            } => {
                let conditions = parse_file_group_conditions(&conditions);
                let mut options = models::FileGroupQueryOptions::default();
                options.limit = limit;
                options.offset = offset;
                options.order_by = parse_file_group_order_by(&order_by);

                match service::file_group::select_file_groups_by_conditions_with_options(
                    conn,
                    conditions,
                    options,
                ) {
                    Ok(file_groups) => {
                        println!("查询结果 (共 {} 条记录):", file_groups.len());
                        for fg in &file_groups {
                            println!("{:?}", fg);
                        }

                        if let Some(first_fg) = file_groups.first() {
                            context.selected_file_id = Some(first_fg.file_id);
                            context.selected_group_id = Some(first_fg.group_id);
                            println!(
                                "\n提示：第一个文件组关联的文件 ID ({}) 和组 ID ({}) 已被选中，可用于后续操作。",
                                first_fg.file_id, first_fg.group_id
                            );
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            FileGroupActions::DeleteByConditions { conditions } => {
                let conditions = parse_file_group_conditions(&conditions);
                match service::file_group::delete_file_groups_by_conditions(conn, conditions) {
                    Ok(count) => println!("成功删除 {} 条记录", count),
                    Err(e) => eprintln!("删除失败: {:?}", e),
                }
            }
        },
        Commands::GroupTag { action } => match action {
            GroupTagActions::Create { group_id, tag_id } => {
                let group_id = group_id
                    .or(context.selected_group_id)
                    .unwrap_or_else(|| {
                        let mut input = String::new();
                        print!("请输入组 ID: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        input.trim().parse().expect("无效的组 ID")
                    });

                let tag_id = tag_id.unwrap_or_else(|| {
                    let mut input = String::new();
                    print!("请输入标签 ID: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    input.trim().parse().expect("无效的标签 ID")
                });

                let dto = models::GroupTagDTO {
                    group_id,
                    tag_id,
                };
                match service::group_tag::create_group_tag(conn, dto) {
                    Ok(dto) => println!("成功创建组标签关联: {:?}", dto),
                    Err(e) => eprintln!("创建组标签关联失败: {:?}", e),
                }
            }
            GroupTagActions::Delete { group_id, tag_id } => {
                let group_id = group_id
                    .or(context.selected_group_id)
                    .unwrap_or_else(|| {
                        let mut input = String::new();
                        print!("请输入组 ID: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        input.trim().parse().expect("无效的组 ID")
                    });

                let tag_id = tag_id
                    .or(context.selected_group_id)
                    .unwrap_or_else(|| {
                        let mut input = String::new();
                        print!("请输入标签 ID: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        input.trim().parse().expect("无效的标签 ID")
                    });

                let mut input = String::new();
                print!(
                    "确定要删除组 ID 为 {} 和标签 ID 为 {} 的关联吗? (y/n): ",
                    group_id, tag_id
                );
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim().eq_ignore_ascii_case("y") {
                    let dto = models::GroupTagDTO {
                        group_id,
                        tag_id,
                    };
                    match service::group_tag::delete_group_tag_by_id(conn, dto) {
                        Ok(count) => println!("成功删除 {} 个组标签关联", count),
                        Err(e) => eprintln!("删除组标签关联失败: {:?}", e),
                    }
                } else {
                    println!("操作已取消");
                }
            }
            GroupTagActions::ListInteractive => {
                list_group_tags_interactive(conn, context);
            }
            GroupTagActions::ListByConditions {
                conditions,
                order_by,
                limit,
                offset,
            } => {
                let conditions = parse_group_tag_conditions(&conditions);
                let mut options = models::GroupTagQueryOptions::default();
                options.limit = limit;
                options.offset = offset;
                options.order_by = parse_group_tag_order_by(&order_by);

                match service::group_tag::select_group_tags_by_conditions_with_options(
                    conn,
                    conditions,
                    options,
                ) {
                    Ok(group_tags) => {
                        println!("查询结果 (共 {} 条记录):", group_tags.len());
                        for gt in &group_tags {
                            println!("{:?}", gt);
                        }

                        if let Some(first_gt) = group_tags.first() {
                            context.selected_group_id = Some(first_gt.group_id);
                            // 我们将 tag_id 也存储在 selected_group_id 中，因为没有专用的字段
                            // context.selected_tag_id = Some(first_gt.tag_id);
                            println!(
                                "\n提示：第一个组标签关联的组 ID ({}) 和标签 ID ({}) 已被选中，可用于后续操作。",
                                first_gt.group_id, first_gt.tag_id
                            );
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            GroupTagActions::DeleteByConditions { conditions } => {
                let conditions = parse_group_tag_conditions(&conditions);
                match service::group_tag::delete_group_tags_by_conditions(conn, conditions) {
                    Ok(count) => println!("成功删除 {} 条记录", count),
                    Err(e) => eprintln!("删除失败: {:?}", e),
                }
            }
        }
    }
    Ok(())
}

// 解析文件条件参数
fn parse_file_conditions(args: &[String]) -> Vec<models::FileCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::Id(value));
                }
                i += 2;
            }
            "id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::IdGreaterThan(value));
                }
                i += 2;
            }
            "id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::IdLessThan(value));
                }
                i += 2;
            }
            "id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::FileCondition::IdIn(values));
                }
                i += 2;
            }
            "type" if i + 1 < args.len() => {
                conditions.push(models::FileCondition::Type(args[i + 1].clone()));
                i += 2;
            }
            "type_like" if i + 1 < args.len() => {
                conditions.push(models::FileCondition::TypeLike(args[i + 1].clone()));
                i += 2;
            }
            "type_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1]
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                conditions.push(models::FileCondition::TypeIn(values));
                i += 2;
            }
            "path" if i + 1 < args.len() => {
                conditions.push(models::FileCondition::Path(args[i + 1].clone()));
                i += 2;
            }
            "path_like" if i + 1 < args.len() => {
                conditions.push(models::FileCondition::PathLike(args[i + 1].clone()));
                i += 2;
            }
            "path_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1]
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                conditions.push(models::FileCondition::PathIn(values));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "ref_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::ReferenceCountGreaterThan(value));
                }
                i += 2;
            }
            "ref_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::ReferenceCountLessThan(value));
                }
                i += 2;
            }
            "ref_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::FileCondition::ReferenceCountIn(values));
                }
                i += 2;
            }
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::GroupId(value));
                }
                i += 2;
            }
            "group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::GroupIdGreaterThan(value));
                }
                i += 2;
            }
            "group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::GroupIdLessThan(value));
                }
                i += 2;
            }
            "group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::FileCondition::GroupIdIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 解析组条件参数
fn parse_group_conditions(args: &[String]) -> Vec<models::GroupCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::Id(value));
                }
                i += 2;
            }
            "id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::IdGreaterThan(value));
                }
                i += 2;
            }
            "id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::IdLessThan(value));
                }
                i += 2;
            }
            "id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupCondition::IdIn(values));
                }
                i += 2;
            }
            "name" if i + 1 < args.len() => {
                conditions.push(models::GroupCondition::Name(args[i + 1].clone()));
                i += 2;
            }
            "name_like" if i + 1 < args.len() => {
                conditions.push(models::GroupCondition::NameLike(args[i + 1].clone()));
                i += 2;
            }
            "name_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1]
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                conditions.push(models::GroupCondition::NameIn(values));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "ref_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ReferenceCountGreaterThan(value));
                }
                i += 2;
            }
            "ref_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ReferenceCountLessThan(value));
                }
                i += 2;
            }
            "ref_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupCondition::ReferenceCountIn(values));
                }
                i += 2;
            }
            "is_primary" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<bool>() {
                    conditions.push(models::GroupCondition::IsPrimary(value));
                }
                i += 2;
            }
            "click_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ClickCount(value));
                }
                i += 2;
            }
            "click_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ClickCountGreaterThan(value));
                }
                i += 2;
            }
            "click_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ClickCountLessThan(value));
                }
                i += 2;
            }
            "click_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupCondition::ClickCountIn(values));
                }
                i += 2;
            }
            "share_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ShareCount(value));
                }
                i += 2;
            }
            "share_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ShareCountGreaterThan(value));
                }
                i += 2;
            }
            "share_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ShareCountLessThan(value));
                }
                i += 2;
            }
            "share_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupCondition::ShareCountIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 解析标签条件参数
fn parse_tag_conditions(args: &[String]) -> Vec<models::TagCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::Id(value));
                }
                i += 2;
            }
            "id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::IdGreaterThan(value));
                }
                i += 2;
            }
            "id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::IdLessThan(value));
                }
                i += 2;
            }
            "id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::TagCondition::IdIn(values));
                }
                i += 2;
            }
            "name" if i + 1 < args.len() => {
                conditions.push(models::TagCondition::Name(args[i + 1].clone()));
                i += 2;
            }
            "name_like" if i + 1 < args.len() => {
                conditions.push(models::TagCondition::NameLike(args[i + 1].clone()));
                i += 2;
            }
            "name_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1]
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                conditions.push(models::TagCondition::NameIn(values));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "ref_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::ReferenceCountGreaterThan(value));
                }
                i += 2;
            }
            "ref_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::ReferenceCountLessThan(value));
                }
                i += 2;
            }
            "ref_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::TagCondition::ReferenceCountIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 解析文件组关联条件参数
fn parse_file_group_conditions(args: &[String]) -> Vec<models::FileGroupCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "file_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::FileId(value));
                }
                i += 2;
            }
            "file_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::FileIdGreaterThan(value));
                }
                i += 2;
            }
            "file_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::FileIdLessThan(value));
                }
                i += 2;
            }
            "file_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::FileGroupCondition::FileIdIn(values));
                }
                i += 2;
            }
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::GroupId(value));
                }
                i += 2;
            }
            "group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::GroupIdGreaterThan(value));
                }
                i += 2;
            }
            "group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::GroupIdLessThan(value));
                }
                i += 2;
            }
            "group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::FileGroupCondition::GroupIdIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 解析组标签关联条件参数
fn parse_group_tag_conditions(args: &[String]) -> Vec<models::GroupTagCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::GroupId(value));
                }
                i += 2;
            }
            "group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::GroupIdGreaterThan(value));
                }
                i += 2;
            }
            "group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::GroupIdLessThan(value));
                }
                i += 2;
            }
            "group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupTagCondition::GroupIdIn(values));
                }
                i += 2;
            }
            "tag_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::TagId(value));
                }
                i += 2;
            }
            "tag_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::TagIdGreaterThan(value));
                }
                i += 2;
            }
            "tag_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::TagIdLessThan(value));
                }
                i += 2;
            }
            "tag_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupTagCondition::TagIdIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 解析文件排序参数
fn parse_file_order_by(args: &[String]) -> Vec<models::FileOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => models::OrderDirection::Asc,
            "desc" => models::OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "id" => models::FileOrderBy::Id(direction),
            "type" => models::FileOrderBy::Type(direction),
            "path" => models::FileOrderBy::Path(direction),
            "ref_count" => models::FileOrderBy::ReferenceCount(direction),
            "group_id" => models::FileOrderBy::GroupId(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

// 解析组排序参数
fn parse_group_order_by(args: &[String]) -> Vec<models::GroupOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => models::OrderDirection::Asc,
            "desc" => models::OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "id" => models::GroupOrderBy::Id(direction),
            "name" => models::GroupOrderBy::Name(direction),
            "ref_count" => models::GroupOrderBy::ReferenceCount(direction),
            "is_primary" => models::GroupOrderBy::IsPrimary(direction),
            "click_count" => models::GroupOrderBy::ClickCount(direction),
            "share_count" => models::GroupOrderBy::ShareCount(direction),
            "create_time" => models::GroupOrderBy::CreateTime(direction),
            "modify_time" => models::GroupOrderBy::ModifyTime(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

// 解析标签排序参数
fn parse_tag_order_by(args: &[String]) -> Vec<models::TagOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => models::OrderDirection::Asc,
            "desc" => models::OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "id" => models::TagOrderBy::Id(direction),
            "name" => models::TagOrderBy::Name(direction),
            "ref_count" => models::TagOrderBy::ReferenceCount(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

// 解析文件组关联排序参数
fn parse_file_group_order_by(args: &[String]) -> Vec<models::FileGroupOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => models::OrderDirection::Asc,
            "desc" => models::OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "file_id" => models::FileGroupOrderBy::FileId(direction),
            "group_id" => models::FileGroupOrderBy::GroupId(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

// 解析组标签关联排序参数
fn parse_group_tag_order_by(args: &[String]) -> Vec<models::GroupTagOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => models::OrderDirection::Asc,
            "desc" => models::OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "group_id" => models::GroupTagOrderBy::GroupId(direction),
            "tag_id" => models::GroupTagOrderBy::TagId(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

// 交互式查询函数（示例）
fn list_files_interactive(conn: &mut AnyConnection, context: &mut Context) {
    use file_classification_core::service::files as file_service;
    use dialoguer::{Select, theme::ColorfulTheme};

    let files = file_service::select_files_by_conditions(conn, vec![], None);

    match files {
        Ok(files) => {
            if files.is_empty() {
                println!("没有找到任何文件。");
                return;
            }

            let items: Vec<String> = files
                .iter()
                .map(|f| format!("[{}] {}", f.id, f.path))
                .collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("请选择一个文件：")
                .items(&items)
                .default(0)
                .interact_opt()
                .unwrap();

            if let Some(index) = selection {
                context.selected_file_id = Some(files[index].id);
                println!("已选择文件 ID: {}", files[index].id);
            } else {
                println!("没有选择文件。");
            }
        }
        Err(e) => {
            eprintln!("查询文件时出错: {}", e);
        }
    }
}

fn list_groups_interactive(conn: &mut AnyConnection, context: &mut Context) {
    use file_classification_core::service::groups as group_service;
    use dialoguer::{Select, theme::ColorfulTheme};

    let groups = group_service::select_groups_by_conditions(conn, vec![], None);

    match groups {
        Ok(groups) => {
            if groups.is_empty() {
                println!("没有找到任何组。");
                return;
            }

            let items: Vec<String> = groups
                .iter()
                .map(|g| format!("[{}] {}", g.id, g.name))
                .collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("请选择一个组：")
                .items(&items)
                .default(0)
                .interact_opt()
                .unwrap();

            if let Some(index) = selection {
                context.selected_group_id = Some(groups[index].id);
                println!("已选择组 ID: {}", groups[index].id);
            } else {
                println!("没有选择组。");
            }
        }
        Err(e) => {
            eprintln!("查询组时出错: {}", e);
        }
    }
}

fn list_tags_interactive(conn: &mut AnyConnection, context: &mut Context) {
    use file_classification_core::service::tags as tag_service;
    use dialoguer::{Select, theme::ColorfulTheme};

    let tags = tag_service::select_tags_by_conditions(conn, vec![], None);

    match tags {
        Ok(tags) => {
            if tags.is_empty() {
                println!("没有找到任何标签。");
                return;
            }

            let items: Vec<String> = tags
                .iter()
                .map(|t| format!("[{}] {}", t.id, t.name))
                .collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("请选择一个标签：")
                .items(&items)
                .default(0)
                .interact_opt()
                .unwrap();

            if let Some(index) = selection {
                context.selected_tag_id = Some(tags[index].id);
                println!("已选择标签 ID: {}", tags[index].id);
            } else {
                println!("没有选择标签。");
            }
        }
        Err(e) => {
            eprintln!("查询标签时出错: {}", e);
        }
    }
}

fn list_file_groups_interactive(conn: &mut AnyConnection, context: &mut Context) {
    use file_classification_core::service::file_group as file_group_service;
    use dialoguer::{Select, theme::ColorfulTheme};

    let file_groups = file_group_service::select_file_groups_by_conditions(conn, vec![], None);

    match file_groups {
        Ok(file_groups) => {
            if file_groups.is_empty() {
                println!("没有找到任何文件组关联。");
                return;
            }

            let items: Vec<String> = file_groups
                .iter()
                .map(|fg| format!("文件 ID: {}, 组 ID: {}", fg.file_id, fg.group_id))
                .collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("请选择一个文件组关联：")
                .items(&items)
                .default(0)
                .interact_opt()
                .unwrap();

            if let Some(index) = selection {
                context.selected_file_id = Some(file_groups[index].file_id);
                context.selected_group_id = Some(file_groups[index].group_id);
                println!(
                    "已选择文件 ID: {}, 组 ID: {}",
                    file_groups[index].file_id, file_groups[index].group_id
                );
            } else {
                println!("没有选择文件组关联。");
            }
        }
        Err(e) => {
            eprintln!("查询文件组关联时出错: {}", e);
        }
    }
}

fn list_group_tags_interactive(conn: &mut AnyConnection, context: &mut Context) {
    use file_classification_core::service::group_tag as group_tag_service;
    use dialoguer::{Select, theme::ColorfulTheme};

    let group_tags = group_tag_service::select_group_tags_by_conditions(conn, vec![], None);

    match group_tags {
        Ok(group_tags) => {
            if group_tags.is_empty() {
                println!("没有找到任何组标签关联。");
                return;
            }

            let items: Vec<String> = group_tags
                .iter()
                .map(|gt| format!("组 ID: {}, 标签 ID: {}", gt.group_id, gt.tag_id))
                .collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("请选择一个组标签关联：")
                .items(&items)
                .default(0)
                .interact_opt()
                .unwrap();

            if let Some(index) = selection {
                context.selected_group_id = Some(group_tags[index].group_id);
                context.selected_tag_id = Some(group_tags[index].tag_id);
                println!(
                    "已选择组 ID: {}, 标签 ID: {}",
                    group_tags[index].group_id, group_tags[index].tag_id
                );
            } else {
                println!("没有选择组标签关联。");
            }
        }
        Err(e) => {
            eprintln!("查询组标签关联时出错: {}", e);
        }
    }
}
