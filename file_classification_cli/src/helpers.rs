// helpers.rs
// 辅助函数，如打印帮助、确认删除、获取输入等

use std::io::{self, Write};

use crate::context::Context;

/// 打印当前上下文
pub fn print_context(context: &Context) {
    println!("当前上下文:");
    println!("  选中的文件ID: {:?}", context.selected_file_id);
    println!("  选中的组ID: {:?}", context.selected_group_id);
    println!("  选中的标签ID: {:?}", context.selected_tag_id);
}

/// 打印帮助信息
pub fn print_help() {
    println!("可用命令:");
    println!("  help                - 显示此帮助信息");
    println!("  context             - 显示当前上下文");
    println!("  clear               - 清屏");
    println!("  exit/quit           - 退出 REPL");
    println!("  ls                  - 列出当前上下文相关项目");
    println!("  cd <id>             - 选择组 ID");
    println!("  select <type> <id>  - 选择文件、组或标签 (类型: file, group, tag)");
    println!("  new <type> <name>   - 创建新项目 (类型: file, group, tag)");
    println!("  rm <type> <id>      - 删除项目 (类型: file, group, tag)");
    println!("  run <file>          - 执行脚本文件");
    println!("  !cmd                - 执行系统命令");
    println!("\n标准命令 (使用完整格式):");
    println!("  file create --type <type> --path <path> --group_id <id>");
    println!("  file delete --id <id>");
    println!("  file list-interactive");
    println!("  file list-by-conditions -c <conditions> --order_by <order> --limit <n> --offset <n>");
    println!("  file list-by-group-id --group_id <id>");
    println!("  file update-by-id --id <id> [options]");
    println!("  file update-by-conditions -c <conditions> [options]");
    println!("  file delete-by-conditions -c <conditions>");
    println!("  group create --name <name>");
    println!("  group delete --id <id>");
    println!("  group list-interactive");
    println!("  group list-by-conditions -c <conditions> --order_by <order> --limit <n> --offset <n>");
    println!("  group list-by-file-id --file_id <id>");
    println!("  group list-by-tag-id --tag_id <id>");
    println!("  group update-by-id --id <id> [options]");
    println!("  group update-by-conditions -c <conditions> [options]");
    println!("  group delete-by-conditions -c <conditions>");
    println!("  tag create --name <name>");
    println!("  tag delete --id <id>");
    println!("  tag list-interactive");
    println!("  tag list-by-conditions -c <conditions> --order_by <order> --limit <n> --offset <n>");
    println!("  tag list-by-group-id --group_id <id>");
    println!("  tag update-by-id --id <id> [options]");
    println!("  tag update-by-conditions -c <conditions> [options]");
    println!("  tag delete-by-conditions -c <conditions>");
    println!("  file-group create --file_id <id> --group_id <id>");
    println!("  file-group delete --file_id <id> --group_id <id>");
    println!("  file-group list-interactive");
    println!("  file-group list-by-conditions -c <conditions> --order_by <order> --limit <n> --offset <n>");
    println!("  file-group delete-by-conditions -c <conditions>");
    println!("  group-tag create --group_id <id> --tag_id <id>");
    println!("  group-tag delete --group_id <id> --tag_id <id>");
    println!("  group-tag list-interactive");
    println!("  group-tag list-by-conditions -c <conditions> --order_by <order> --limit <n> --offset <n>");
    println!("  group-tag delete-by-conditions -c <conditions>");
}

/// 确认删除操作
pub fn confirm_deletion(prompt: &str) -> bool {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("y")
}

/// 获取用户输入
pub fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
