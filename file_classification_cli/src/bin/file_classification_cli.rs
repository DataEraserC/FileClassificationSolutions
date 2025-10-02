use clap::{Parser, Subcommand};
use file_classification_core::model::models::*;
use file_classification_core::service::*;
use file_classification_core::utils::database::{establish_connection, AnyConnection};
use std::io::{self, Write};

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
}

#[derive(Subcommand)]
enum FileActions {
    /// 创建文件
    Create {
        #[clap(short, long)]
        type_: String,
        #[clap(short, long)]
        path: String,
        #[clap(short, long)]
        group_id: i32,
    },
    /// 删除文件
    Delete {
        #[clap(short, long)]
        id: i32,
    },
    /// 查询文件（交互式）
    ListInteractive,
    /// 根据条件查询文件
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
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
        #[clap(short, long)]
        type_: Option<String>,
        #[clap(short, long)]
        reference_count: Option<i32>,
        #[clap(short, long)]
        group_id: Option<i32>,
    },
}

#[derive(Subcommand)]
enum GroupActions {
    /// 创建组
    Create {
        #[clap(short, long)]
        name: String,
    },
    /// 删除组
    Delete {
        #[clap(short, long)]
        id: i32,
    },
    /// 查询组（交互式）
    ListInteractive,
    /// 根据条件查询组
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
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
        #[clap(short, long)]
        reference_count: Option<i32>,
        #[clap(short, long)]
        is_primary: Option<bool>,
        #[clap(short, long)]
        click_count: Option<i32>,
        #[clap(short, long)]
        share_count: Option<i32>,
    },
}

#[derive(Subcommand)]
enum TagActions {
    /// 创建标签
    Create {
        #[clap(short, long)]
        name: String,
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
        #[clap(short, long)]
        reference_count: Option<i32>,
    },
}

#[derive(Subcommand)]
enum FileGroupActions {
    /// 创建文件组关联
    Create {
        #[clap(short, long)]
        file_id: i32,
        #[clap(short, long)]
        group_id: i32,
    },
    /// 删除文件组关联
    Delete {
        #[clap(short, long)]
        file_id: i32,
        #[clap(short, long)]
        group_id: i32,
    },
    /// 查询文件组关联（交互式）
    ListInteractive,
    /// 根据条件查询文件组关联
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
    },
}

#[derive(Subcommand)]
enum GroupTagActions {
    /// 创建组标签关联
    Create {
        #[clap(short, long)]
        group_id: i32,
        #[clap(short, long)]
        tag_id: i32,
    },
    /// 删除组标签关联
    Delete {
        #[clap(short, long)]
        group_id: i32,
        #[clap(short, long)]
        tag_id: i32,
    },
    /// 查询组标签关联（交互式）
    ListInteractive,
    /// 根据条件查询组标签关联
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut conn = establish_connection();

    match &cli.command {
        Commands::File { action } => match action {
            FileActions::Create { type_, path, group_id } => {
                let dto = CreateFileDTO {
                    type_: type_,
                    path,
                    group_id: *group_id,
                };
                match files::create_file(&mut conn, dto) {
                    Ok(count) => println!("成功创建文件，影响 {} 行", count),
                    Err(e) => eprintln!("创建文件失败: {:?}", e),
                }
            }
            FileActions::Delete { id } => {
                match files::delete_file(&mut conn, *id) {
                    Ok(()) => println!("成功删除文件"),
                    Err(e) => eprintln!("删除文件失败: {:?}", e),
                }
            }
            FileActions::ListInteractive => {
                list_files_interactive(&mut conn);
            }
            FileActions::ListByConditions { conditions } => {
                let conditions = parse_file_conditions(conditions);
                match files::select_files_by_conditions(&mut conn, conditions, Some(20)) {
                    Ok(files) => {
                        println!("查询结果 (共 {} 条记录):", files.len());
                        for file in files {
                            println!("{:?}", file);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            FileActions::ListByGroupId { group_id } => {
                match files::select_file_by_group_id(&mut conn, *group_id) {
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
                let conditions = parse_file_conditions(conditions);
                let update_dto = UpdateFileDTO {
                    path: path.clone(),
                    type_: type_.clone(),
                    reference_count: *reference_count,
                    group_id: *group_id,
                };
                match files::update_files_by_conditions(&mut conn, conditions, update_dto) {
                    Ok(count) => println!("成功更新 {} 条记录", count),
                    Err(e) => eprintln!("更新失败: {:?}", e),
                }
            }
        },
        Commands::Group { action } => match action {
            GroupActions::Create { name } => {
                match groups::create_group(&mut conn, name) {
                    Ok(count) => println!("成功创建组，影响 {} 行", count),
                    Err(e) => eprintln!("创建组失败: {:?}", e),
                }
            }
            GroupActions::Delete { id } => {
                match groups::delete_group(&mut conn, *id) {
                    Ok(count) => println!("成功删除 {} 个组", count),
                    Err(e) => eprintln!("删除组失败: {:?}", e),
                }
            }
            GroupActions::ListInteractive => {
                list_groups_interactive(&mut conn);
            }
            GroupActions::ListByConditions { conditions } => {
                let conditions = parse_group_conditions(conditions);
                match groups::select_groups_by_conditions(&mut conn, conditions, Some(20)) {
                    Ok(groups) => {
                        println!("查询结果 (共 {} 条记录):", groups.len());
                        for group in groups {
                            println!("{:?}", group);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            GroupActions::ListByFileId { file_id } => {
                match groups::select_group_by_file_id(&mut conn, *file_id) {
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
                match groups::select_group_by_tag_id(&mut conn, *tag_id) {
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
                let conditions = parse_group_conditions(conditions);
                let update_dto = UpdateGroupDTO {
                    id: None,
                    name: name.clone(),
                    reference_count: *reference_count,
                    is_primary: *is_primary,
                    click_count: *click_count,
                    share_count: *share_count,
                    create_time: None,
                    modify_time: None,
                };
                match groups::update_groups_by_conditions(&mut conn, conditions, update_dto) {
                    Ok(count) => println!("成功更新 {} 条记录", count),
                    Err(e) => eprintln!("更新失败: {:?}", e),
                }
            }
        },
        Commands::Tag { action } => match action {
            TagActions::Create { name } => {
                match tags::create_tag(&mut conn, name) {
                    Ok(tag) => println!("成功创建标签: {:?}", tag),
                    Err(e) => eprintln!("创建标签失败: {:?}", e),
                }
            }
            TagActions::Delete { id } => {
                match tags::delete_tag(&mut conn, *id) {
                    Ok(count) => println!("成功删除 {} 个标签", count),
                    Err(e) => eprintln!("删除标签失败: {:?}", e),
                }
            }
            TagActions::ListInteractive => {
                list_tags_interactive(&mut conn);
            }
            TagActions::ListByConditions { conditions } => {
                let conditions = parse_tag_conditions(conditions);
                match tags::select_tags_by_conditions(&mut conn, conditions, Some(20)) {
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
                match tags::select_tag_by_group_id(&mut conn, *group_id) {
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
                let conditions = parse_tag_conditions(conditions);
                let update_dto = UpdateTagDTO {
                    name: name.clone(),
                    reference_count: *reference_count,
                };
                match tags::update_tags_by_conditions(&mut conn, conditions, update_dto) {
                    Ok(count) => println!("成功更新 {} 条记录", count),
                    Err(e) => eprintln!("更新失败: {:?}", e),
                }
            }
        },
        Commands::FileGroup { action } => match action {
            FileGroupActions::Create { file_id, group_id } => {
                let dto = FileGroupDTO {
                    file_id: *file_id,
                    group_id: *group_id,
                };
                match file_group::create_file_group(&mut conn, dto) {
                    Ok(dto) => println!("成功创建文件组关联: {:?}", dto),
                    Err(e) => eprintln!("创建文件组关联失败: {:?}", e),
                }
            }
            FileGroupActions::Delete { file_id, group_id } => {
                let dto = FileGroupDTO {
                    file_id: *file_id,
                    group_id: *group_id,
                };
                match file_group::delete_file_group(&mut conn, dto) {
                    Ok(count) => println!("成功删除 {} 个文件组关联", count),
                    Err(e) => eprintln!("删除文件组关联失败: {:?}", e),
                }
            }
            FileGroupActions::ListInteractive => {
                list_file_groups_interactive(&mut conn);
            }
            FileGroupActions::ListByConditions { conditions } => {
                let conditions = parse_file_group_conditions(conditions);
                match file_group::select_file_groups_by_conditions(&mut conn, conditions, Some(20)) {
                    Ok(file_groups) => {
                        println!("查询结果 (共 {} 条记录):", file_groups.len());
                        for fg in file_groups {
                            println!("{:?}", fg);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
        },
        Commands::GroupTag { action } => match action {
            GroupTagActions::Create { group_id, tag_id } => {
                let dto = GroupTagDTO {
                    group_id: *group_id,
                    tag_id: *tag_id,
                };
                match group_tag::create_group_tag(&mut conn, dto) {
                    Ok(dto) => println!("成功创建组标签关联: {:?}", dto),
                    Err(e) => eprintln!("创建组标签关联失败: {:?}", e),
                }
            }
            GroupTagActions::Delete { group_id, tag_id } => {
                let dto = GroupTagDTO {
                    group_id: *group_id,
                    tag_id: *tag_id,
                };
                match group_tag::delete_group_tag_by_id(&mut conn, dto) {
                    Ok(count) => println!("成功删除 {} 个组标签关联", count),
                    Err(e) => eprintln!("删除组标签关联失败: {:?}", e),
                }
            }
            GroupTagActions::ListInteractive => {
                list_group_tags_interactive(&mut conn);
            }
            GroupTagActions::ListByConditions { conditions } => {
                let conditions = parse_group_tag_conditions(conditions);
                match group_tag::select_group_tags_by_conditions(&mut conn, conditions, Some(20)) {
                    Ok(group_tags) => {
                        println!("查询结果 (共 {} 条记录):", group_tags.len());
                        for gt in group_tags {
                            println!("{:?}", gt);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
        },
    }

    Ok(())
}

// 交互式查询函数
fn list_files_interactive(conn: &mut AnyConnection) {
    println!("文件查询");
    println!("============");

    loop {
        let conditions = get_file_conditions_interactive();

        if conditions.is_empty() {
            println!("未输入任何条件");
        }

        match files::select_files_by_conditions(conn, conditions, Some(20)) {
            Ok(files) => {
                println!("\n查询结果 (共 {} 条记录):", files.len());
                println!("-------------------------");
                if files.is_empty() {
                    println!("没有找到匹配的文件。");
                } else {
                    for file in files {
                        println!("{:?}", file);
                    }
                }
            }
            Err(e) => {
                eprintln!("查询出错: {}", e);
            }
        }

        println!("\n是否继续查询？(y/n): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        if input.trim().to_lowercase() != "y" {
            break;
        }
        println!("\n");
    }

    println!("感谢使用文件查询！");
}

fn list_groups_interactive(conn: &mut AnyConnection) {
    println!("组查询");
    println!("============");

    loop {
        let conditions = get_group_conditions_interactive();

        if conditions.is_empty() {
            println!("未输入任何条件");
        }

        match groups::select_groups_by_conditions(conn, conditions, Some(20)) {
            Ok(groups) => {
                println!("\n查询结果 (共 {} 条记录):", groups.len());
                println!("-------------------------");
                if groups.is_empty() {
                    println!("没有找到匹配的组。");
                } else {
                    for group in groups {
                        println!("{:?}", group);
                    }
                }
            }
            Err(e) => {
                eprintln!("查询出错: {}", e);
            }
        }

        println!("\n是否继续查询？(y/n): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        if input.trim().to_lowercase() != "y" {
            break;
        }
        println!("\n");
    }

    println!("感谢使用组查询！");
}

fn list_tags_interactive(conn: &mut AnyConnection) {
    println!("标签查询");
    println!("============");

    loop {
        let conditions = get_tag_conditions_interactive();

        if conditions.is_empty() {
            println!("未输入任何条件");
        }

        match tags::select_tags_by_conditions(conn, conditions, Some(20)) {
            Ok(tags) => {
                println!("\n查询结果 (共 {} 条记录):", tags.len());
                println!("-------------------------");
                if tags.is_empty() {
                    println!("没有找到匹配的标签。");
                } else {
                    for tag in tags {
                        println!("{:?}", tag);
                    }
                }
            }
            Err(e) => {
                eprintln!("查询出错: {}", e);
            }
        }

        println!("\n是否继续查询？(y/n): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        if input.trim().to_lowercase() != "y" {
            break;
        }
        println!("\n");
    }

    println!("感谢使用标签查询！");
}

fn list_file_groups_interactive(conn: &mut AnyConnection) {
    println!("文件组关联查询");
    println!("==============");

    loop {
        let conditions = get_file_group_conditions_interactive();

        if conditions.is_empty() {
            println!("未输入任何条件");
        }

        match file_group::select_file_groups_by_conditions(conn, conditions, Some(20)) {
            Ok(file_groups) => {
                println!("\n查询结果 (共 {} 条记录):", file_groups.len());
                println!("-------------------------");
                if file_groups.is_empty() {
                    println!("没有找到匹配的文件组关联。");
                } else {
                    for fg in file_groups {
                        println!("文件ID: {}, 组ID: {}", fg.file_id, fg.group_id);
                    }
                }
            }
            Err(e) => {
                eprintln!("查询出错: {}", e);
            }
        }

        println!("\n是否继续查询？(y/n): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        if input.trim().to_lowercase() != "y" {
            break;
        }
        println!("\n");
    }

    println!("感谢使用文件组关联查询！");
}

fn list_group_tags_interactive(conn: &mut AnyConnection) {
    println!("组标签关联查询");
    println!("==============");

    loop {
        let conditions = get_group_tag_conditions_interactive();

        if conditions.is_empty() {
            println!("未输入任何条件");
        }

        match group_tag::select_group_tags_by_conditions(conn, conditions, Some(20)) {
            Ok(group_tags) => {
                println!("\n查询结果 (共 {} 条记录):", group_tags.len());
                println!("-------------------------");
                if group_tags.is_empty() {
                    println!("没有找到匹配的组标签关联。");
                } else {
                    for gt in group_tags {
                        println!("组ID: {}, 标签ID: {}", gt.group_id, gt.tag_id);
                    }
                }
            }
            Err(e) => {
                eprintln!("查询出错: {}", e);
            }
        }

        println!("\n是否继续查询？(y/n): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        if input.trim().to_lowercase() != "y" {
            break;
        }
        println!("\n");
    }

    println!("感谢使用组标签关联查询！");
}

// 交互式条件输入函数
fn get_file_conditions_interactive() -> Vec<FileCondition> {
    let mut conditions = Vec::new();

    println!("请输入查询条件（留空结束输入）:");

    loop {
        println!("\n可选条件类型:");
        println!("1. ID等于");
        println!("2. ID大于");
        println!("3. ID小于");
        println!("4. 类型等于");
        println!("5. 类型匹配(like)");
        println!("6. 路径等于");
        println!("7. 路径匹配(like)");
        println!("8. 引用次数等于");
        println!("9. 引用次数大于");
        println!("10. 引用次数小于");
        println!("11. 组ID等于");
        println!("12. 组ID大于");
        println!("13. 组ID小于");
        println!("14. AND组合条件");
        println!("15. OR组合条件");
        println!("16. NOT条件");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-16): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入文件类型: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::Type(value));
                }
            }
            5 => {
                print!("请输入类型匹配模式 (如 '%.jpg'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::TypeLike(value));
                }
            }
            6 => {
                print!("请输入文件路径: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::Path(value));
                }
            }
            7 => {
                print!("请输入路径匹配模式: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::PathLike(value));
                }
            }
            8 => {
                print!("请输入引用次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCount(value));
                }
            }
            9 => {
                print!("请输入引用次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCountGreaterThan(value));
                }
            }
            10 => {
                print!("请输入引用次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCountLessThan(value));
                }
            }
            11 => {
                print!("请输入组ID: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupId(value));
                }
            }
            12 => {
                print!("请输入组ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupIdGreaterThan(value));
                }
            }
            13 => {
                print!("请输入组ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupIdLessThan(value));
                }
            }
            14 => {
                println!("请输入AND组合条件 (输入0结束):");
                let sub_conditions = get_file_sub_conditions("AND");
                if !sub_conditions.is_empty() {
                    conditions.push(FileCondition::And(sub_conditions));
                }
            }
            15 => {
                println!("请输入OR组合条件 (输入0结束):");
                let sub_conditions = get_file_sub_conditions("OR");
                if !sub_conditions.is_empty() {
                    conditions.push(FileCondition::Or(sub_conditions));
                }
            }
            16 => {
                println!("请输入NOT条件:");
                if let Some(condition) = get_file_single_condition() {
                    conditions.push(FileCondition::Not(Box::new(condition)));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_file_sub_conditions(condition_type: &str) -> Vec<FileCondition> {
    let mut conditions = Vec::new();

    loop {
        println!("\n{}子条件类型:", condition_type);
        println!("1. ID等于");
        println!("2. ID大于");
        println!("3. ID小于");
        println!("4. 类型等于");
        println!("5. 类型匹配(like)");
        println!("6. 路径等于");
        println!("7. 路径匹配(like)");
        println!("8. 引用次数等于");
        println!("9. 引用次数大于");
        println!("10. 引用次数小于");
        println!("11. 组ID等于");
        println!("12. 组ID大于");
        println!("13. 组ID小于");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-13): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入文件类型: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::Type(value));
                }
            }
            5 => {
                print!("请输入类型匹配模式 (如 '%.jpg'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::TypeLike(value));
                }
            }
            6 => {
                print!("请输入文件路径: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::Path(value));
                }
            }
            7 => {
                print!("请输入路径匹配模式: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::PathLike(value));
                }
            }
            8 => {
                print!("请输入引用次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCount(value));
                }
            }
            9 => {
                print!("请输入引用次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCountGreaterThan(value));
                }
            }
            10 => {
                print!("请输入引用次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCountLessThan(value));
                }
            }
            11 => {
                print!("请输入组ID: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupId(value));
                }
            }
            12 => {
                print!("请输入组ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupIdGreaterThan(value));
                }
            }
            13 => {
                print!("请输入组ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupIdLessThan(value));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_file_single_condition() -> Option<FileCondition> {
    println!("NOT条件类型:");
    println!("1. ID等于");
    println!("2. ID大于");
    println!("3. ID小于");
    println!("4. 类型等于");
    println!("5. 类型匹配模式");
    println!("6. 路径等于");
    println!("7. 路径匹配模式");
    println!("8. 引用计数等于");
    println!("9. 引用计数大于");
    println!("10. 引用计数小于");
    println!("11. 组ID等于");
    println!("12. 组ID大于");
    println!("13. 组ID小于");

    print!("请选择条件类型 (1-13): ");
    io::stdout().flush().unwrap();

    let choice = read_number();

    match choice {
        1 => {
            print!("请输入ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::Id)
        }
        2 => {
            print!("请输入ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::IdGreaterThan)
        }
        3 => {
            print!("请输入ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::IdLessThan)
        }
        4 => {
            print!("请输入文件类型: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(FileCondition::Type)
        }
        5 => {
            print!("请输入类型匹配模式: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(FileCondition::TypeLike)
        }
        6 => {
            print!("请输入文件路径: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(FileCondition::Path)
        }
        7 => {
            print!("请输入路径匹配模式: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(FileCondition::PathLike)
        }
        8 => {
            print!("请输入引用计数值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::ReferenceCount)
        }
        9 => {
            print!("请输入引用计数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::ReferenceCountGreaterThan)
        }
        10 => {
            print!("请输入引用计数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::ReferenceCountLessThan)
        }
        11 => {
            print!("请输入组ID: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::GroupId)
        }
        12 => {
            print!("请输入组ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::GroupIdGreaterThan)
        }
        13 => {
            print!("请输入组ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::GroupIdLessThan)
        }
        _ => {
            println!("无效选择。");
            None
        }
    }
}

fn get_group_conditions_interactive() -> Vec<GroupCondition> {
    let mut conditions = Vec::new();

    println!("请输入查询条件（留空结束输入）:");

    loop {
        println!("\n可选条件类型:");
        println!("1. ID等于");
        println!("2. ID大于");
        println!("3. ID小于");
        println!("4. 名称等于");
        println!("5. 名称匹配模式");
        println!("6. 引用次数等于");
        println!("7. 引用次数大于");
        println!("8. 引用次数小于");
        println!("9. 是否为主组");
        println!("10. 点击次数等于");
        println!("11. 点击次数大于");
        println!("12. 点击次数小于");
        println!("13. 分享次数等于");
        println!("14. 分享次数大于");
        println!("15. 分享次数小于");
        println!("16. AND组合条件");
        println!("17. OR组合条件");
        println!("18. NOT条件");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-18): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入组ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入组名称: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(GroupCondition::Name(value));
                }
            }
            5 => {
                print!("请输入名称匹配模式 (如 '%图片%'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(GroupCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用计数值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用计数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用计数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCountLessThan(value));
                }
            }
            9 => {
                print!("是否为主组 (true/false): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_bool_optional() {
                    conditions.push(GroupCondition::IsPrimary(value));
                }
            }
            10 => {
                print!("请输入点击次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCount(value));
                }
            }
            11 => {
                print!("请输入点击次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCountGreaterThan(value));
                }
            }
            12 => {
                print!("请输入点击次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCountLessThan(value));
                }
            }
            13 => {
                print!("请输入分享次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCount(value));
                }
            }
            14 => {
                print!("请输入分享次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCountGreaterThan(value));
                }
            }
            15 => {
                print!("请输入分享次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCountLessThan(value));
                }
            }
            16 => {
                println!("请输入AND组合条件 (输入0结束):");
                let sub_conditions = get_group_sub_conditions("AND");
                if !sub_conditions.is_empty() {
                    conditions.push(GroupCondition::And(sub_conditions));
                }
            }
            17 => {
                println!("请输入OR组合条件 (输入0结束):");
                let sub_conditions = get_group_sub_conditions("OR");
                if !sub_conditions.is_empty() {
                    conditions.push(GroupCondition::Or(sub_conditions));
                }
            }
            18 => {
                println!("请输入NOT条件:");
                if let Some(condition) = get_group_single_condition() {
                    conditions.push(GroupCondition::Not(Box::new(condition)));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_group_sub_conditions(condition_type: &str) -> Vec<GroupCondition> {
    let mut conditions = Vec::new();

    loop {
        println!("\n{}子条件类型:", condition_type);
        println!("1. ID等于");
        println!("2. ID大于");
        println!("3. ID小于");
        println!("4. 名称等于");
        println!("5. 名称匹配(like)");
        println!("6. 引用次数等于");
        println!("7. 引用次数大于");
        println!("8. 引用次数小于");
        println!("9. 是否为主组");
        println!("10. 点击次数等于");
        println!("11. 点击次数大于");
        println!("12. 点击次数小于");
        println!("13. 分享次数等于");
        println!("14. 分享次数大于");
        println!("15. 分享次数小于");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-15): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入组ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入组名称: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(GroupCondition::Name(value));
                }
            }
            5 => {
                print!("请输入名称匹配模式 (如 '%图片%'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(GroupCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCountLessThan(value));
                }
            }
            9 => {
                print!("是否为主组 (true/false): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_bool_optional() {
                    conditions.push(GroupCondition::IsPrimary(value));
                }
            }
            10 => {
                print!("请输入点击次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCount(value));
                }
            }
            11 => {
                print!("请输入点击次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCountGreaterThan(value));
                }
            }
            12 => {
                print!("请输入点击次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCountLessThan(value));
                }
            }
            13 => {
                print!("请输入分享次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCount(value));
                }
            }
            14 => {
                print!("请输入分享次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCountGreaterThan(value));
                }
            }
            15 => {
                print!("请输入分享次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCountLessThan(value));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_group_single_condition() -> Option<GroupCondition> {
    println!("NOT条件类型:");
    println!("1. ID等于");
    println!("2. ID大于");
    println!("3. ID小于");
    println!("4. 名称等于");
    println!("5. 名称匹配模式");
    println!("6. 引用计数等于");
    println!("7. 引用计数大于");
    println!("8. 引用计数小于");
    println!("9. 是否为主组");
    println!("10. 点击次数等于");
    println!("11. 点击次数大于");
    println!("12. 点击次数小于");
    println!("13. 分享次数等于");
    println!("14. 分享次数大于");
    println!("15. 分享次数小于");

    print!("请选择条件类型 (1-15): ");
    io::stdout().flush().unwrap();

    let choice = read_number();

    match choice {
        1 => {
            print!("请输入组ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::Id)
        }
        2 => {
            print!("请输入ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::IdGreaterThan)
        }
        3 => {
            print!("请输入ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::IdLessThan)
        }
        4 => {
            print!("请输入组名称: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(GroupCondition::Name)
        }
        5 => {
            print!("请输入名称匹配模式: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(GroupCondition::NameLike)
        }
        6 => {
            print!("请输入引用计数值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ReferenceCount)
        }
        7 => {
            print!("请输入引用计数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ReferenceCountGreaterThan)
        }
        8 => {
            print!("请输入引用计数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ReferenceCountLessThan)
        }
        9 => {
            print!("是否为主组 (true/false): ");
            io::stdout().flush().unwrap();
            read_bool_optional().map(GroupCondition::IsPrimary)
        }
        10 => {
            print!("请输入点击次数: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ClickCount)
        }
        11 => {
            print!("请输入点击次数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ClickCountGreaterThan)
        }
        12 => {
            print!("请输入点击次数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ClickCountLessThan)
        }
        13 => {
            print!("请输入分享次数: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ShareCount)
        }
        14 => {
            print!("请输入分享次数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ShareCountGreaterThan)
        }
        15 => {
            print!("请输入分享次数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ShareCountLessThan)
        }
        _ => {
            println!("无效选择。");
            None
        }
    }
}

fn get_tag_conditions_interactive() -> Vec<TagCondition> {
    let mut conditions = Vec::new();

    println!("请输入查询条件（留空结束输入）:");

    loop {
        println!("\n可选条件类型:");
        println!("1. ID等于");
        println!("2. ID大于");
        println!("3. ID小于");
        println!("4. 名称等于");
        println!("5. 名称匹配(like)");
        println!("6. 引用次数等于");
        println!("7. 引用次数大于");
        println!("8. 引用次数小于");
        println!("9. AND组合条件");
        println!("10. OR组合条件");
        println!("11. NOT条件");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-11): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入标签名称: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(TagCondition::Name(value));
                }
            }
            5 => {
                print!("请输入名称匹配模式 (如 '%重要%'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(TagCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用计数值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用计数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用计数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCountLessThan(value));
                }
            }
            9 => {
                println!("请输入AND组合条件 (输入0结束):");
                let sub_conditions = get_tag_sub_conditions("AND");
                if !sub_conditions.is_empty() {
                    conditions.push(TagCondition::And(sub_conditions));
                }
            }
            10 => {
                println!("请输入OR组合条件 (输入0结束):");
                let sub_conditions = get_tag_sub_conditions("OR");
                if !sub_conditions.is_empty() {
                    conditions.push(TagCondition::Or(sub_conditions));
                }
            }
            11 => {
                println!("请输入NOT条件:");
                if let Some(condition) = get_tag_single_condition() {
                    conditions.push(TagCondition::Not(Box::new(condition)));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_tag_sub_conditions(condition_type: &str) -> Vec<TagCondition> {
    let mut conditions = Vec::new();

    loop {
        println!("\n{}子条件类型:", condition_type);
        println!("1. ID等于");
        println!("2. ID大于");
        println!("3. ID小于");
        println!("4. 名称等于");
        println!("5. 名称匹配(like)");
        println!("6. 引用次数等于");
        println!("7. 引用次数大于");
        println!("8. 引用次数小于");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-8): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入标签名称: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(TagCondition::Name(value));
                }
            }
            5 => {
                print!("请输入名称匹配模式 (如 '%重要%'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(TagCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCountLessThan(value));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_tag_single_condition() -> Option<TagCondition> {
    println!("NOT条件类型:");
    println!("1. ID等于");
    println!("2. ID大于");
    println!("3. ID小于");
    println!("4. 名称等于");
    println!("5. 名称匹配模式");
    println!("6. 引用计数等于");
    println!("7. 引用计数大于");
    println!("8. 引用计数小于");

    print!("请选择条件类型 (1-8): ");
    io::stdout().flush().unwrap();

    let choice = read_number();

    match choice {
        1 => {
            print!("请输入ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::Id)
        }
        2 => {
            print!("请输入ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::IdGreaterThan)
        }
        3 => {
            print!("请输入ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::IdLessThan)
        }
        4 => {
            print!("请输入标签名称: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(TagCondition::Name)
        }
        5 => {
            print!("请输入名称匹配模式: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(TagCondition::NameLike)
        }
        6 => {
            print!("请输入引用计数值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::ReferenceCount)
        }
        7 => {
            print!("请输入引用计数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::ReferenceCountGreaterThan)
        }
        8 => {
            print!("请输入引用计数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::ReferenceCountLessThan)
        }
        _ => {
            println!("无效选择。");
            None
        }
    }
}

fn get_file_group_conditions_interactive() -> Vec<FileGroupCondition> {
    let mut conditions = Vec::new();

    println!("请输入查询条件（留空结束输入）:");

    loop {
        println!("\n可选条件类型:");
        println!("1. 文件ID等于");
        println!("2. 文件ID大于");
        println!("3. 文件ID小于");
        println!("4. 组ID等于");
        println!("5. 组ID大于");
        println!("6. 组ID小于");
        println!("7. AND组合条件");
        println!("8. OR组合条件");
        println!("9. NOT条件");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-9): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入文件ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileId(value));
                }
            }
            2 => {
                print!("请输入文件ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileIdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入文件ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileIdLessThan(value));
                }
            }
            4 => {
                print!("请输入组ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupId(value));
                }
            }
            5 => {
                print!("请输入组ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupIdGreaterThan(value));
                }
            }
            6 => {
                print!("请输入组ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupIdLessThan(value));
                }
            }
            7 => {
                println!("请输入AND组合条件 (输入0结束):");
                let sub_conditions = get_file_group_sub_conditions("AND");
                if !sub_conditions.is_empty() {
                    conditions.push(FileGroupCondition::And(sub_conditions));
                }
            }
            8 => {
                println!("请输入OR组合条件 (输入0结束):");
                let sub_conditions = get_file_group_sub_conditions("OR");
                if !sub_conditions.is_empty() {
                    conditions.push(FileGroupCondition::Or(sub_conditions));
                }
            }
            9 => {
                println!("请输入NOT条件:");
                if let Some(condition) = get_file_group_single_condition() {
                    conditions.push(FileGroupCondition::Not(Box::new(condition)));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_file_group_sub_conditions(condition_type: &str) -> Vec<FileGroupCondition> {
    let mut conditions = Vec::new();

    loop {
        println!("\n{}子条件类型:", condition_type);
        println!("1. 文件ID等于");
        println!("2. 文件ID大于");
        println!("3. 文件ID小于");
        println!("4. 组ID等于");
        println!("5. 组ID大于");
        println!("6. 组ID小于");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-6): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入文件ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileId(value));
                }
            }
            2 => {
                print!("请输入文件ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileIdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入文件ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileIdLessThan(value));
                }
            }
            4 => {
                print!("请输入组ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupId(value));
                }
            }
            5 => {
                print!("请输入组ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupIdGreaterThan(value));
                }
            }
            6 => {
                print!("请输入组ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupIdLessThan(value));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_file_group_single_condition() -> Option<FileGroupCondition> {
    println!("NOT条件类型:");
    println!("1. 文件ID等于");
    println!("2. 文件ID大于");
    println!("3. 文件ID小于");
    println!("4. 组ID等于");
    println!("5. 组ID大于");
    println!("6. 组ID小于");

    print!("请选择条件类型 (1-6): ");
    io::stdout().flush().unwrap();

    let choice = read_number();

    match choice {
        1 => {
            print!("请输入文件ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::FileId)
        }
        2 => {
            print!("请输入文件ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::FileIdGreaterThan)
        }
        3 => {
            print!("请输入文件ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::FileIdLessThan)
        }
        4 => {
            print!("请输入组ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::GroupId)
        }
        5 => {
            print!("请输入组ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::GroupIdGreaterThan)
        }
        6 => {
            print!("请输入组ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::GroupIdLessThan)
        }
        _ => {
            println!("无效选择。");
            None
        }
    }
}

fn get_group_tag_conditions_interactive() -> Vec<GroupTagCondition> {
    let mut conditions = Vec::new();

    println!("请输入查询条件（留空结束输入）:");

    loop {
        println!("\n可选条件类型:");
        println!("1. 组ID等于");
        println!("2. 组ID大于");
        println!("3. 组ID小于");
        println!("4. 标签ID等于");
        println!("5. 标签ID大于");
        println!("6. 标签ID小于");
        println!("7. AND组合条件");
        println!("8. OR组合条件");
        println!("9. NOT条件");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-9): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入组ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::GroupId(value));
                }
            }
            2 => {
                print!("请输入组ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::GroupIdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入组ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::GroupIdLessThan(value));
                }
            }
            4 => {
                print!("请输入标签ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::TagId(value));
                }
            }
            5 => {
                print!("请输入标签ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::TagIdGreaterThan(value));
                }
            }
            6 => {
                print!("请输入标签ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::TagIdLessThan(value));
                }
            }
            7 => {
                println!("请输入AND组合条件 (输入0结束):");
                let sub_conditions = get_group_tag_sub_conditions("AND");
                if !sub_conditions.is_empty() {
                    conditions.push(GroupTagCondition::And(sub_conditions));
                }
            }
            8 => {
                println!("请输入OR组合条件 (输入0结束):");
                let sub_conditions = get_group_tag_sub_conditions("OR");
                if !sub_conditions.is_empty() {
                    conditions.push(GroupTagCondition::Or(sub_conditions));
                }
            }
            9 => {
                println!("请输入NOT条件:");
                if let Some(condition) = get_group_tag_single_condition() {
                    conditions.push(GroupTagCondition::Not(Box::new(condition)));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_group_tag_sub_conditions(condition_type: &str) -> Vec<GroupTagCondition> {
    let mut conditions = Vec::new();

    loop {
        println!("\n{}子条件类型:", condition_type);
        println!("1. 组ID等于");
        println!("2. 组ID大于");
        println!("3. 组ID小于");
        println!("4. 标签ID等于");
        println!("5. 标签ID大于");
        println!("6. 标签ID小于");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-6): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入组ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::GroupId(value));
                }
            }
            2 => {
                print!("请输入组ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::GroupIdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入组ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::GroupIdLessThan(value));
                }
            }
            4 => {
                print!("请输入标签ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::TagId(value));
                }
            }
            5 => {
                print!("请输入标签ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::TagIdGreaterThan(value));
                }
            }
            6 => {
                print!("请输入标签ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupTagCondition::TagIdLessThan(value));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_group_tag_single_condition() -> Option<GroupTagCondition> {
    println!("NOT条件类型:");
    println!("1. 组ID等于");
    println!("2. 组ID大于");
    println!("3. 组ID小于");
    println!("4. 标签ID等于");
    println!("5. 标签ID大于");
    println!("6. 标签ID小于");

    print!("请选择条件类型 (1-6): ");
    io::stdout().flush().unwrap();

    let choice = read_number();

    match choice {
        1 => {
            print!("请输入组ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupTagCondition::GroupId)
        }
        2 => {
            print!("请输入组ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupTagCondition::GroupIdGreaterThan)
        }
        3 => {
            print!("请输入组ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupTagCondition::GroupIdLessThan)
        }
        4 => {
            print!("请输入标签ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupTagCondition::TagId)
        }
        5 => {
            print!("请输入标签ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupTagCondition::TagIdGreaterThan)
        }
        6 => {
            print!("请输入标签ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupTagCondition::TagIdLessThan)
        }
        _ => {
            println!("无效选择。");
            None
        }
    }
}

// 条件解析函数
fn parse_file_conditions(args: &[String]) -> Vec<FileCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::Id(value));
                }
                i += 2;
            }
            "id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::IdGreaterThan(value));
                }
                i += 2;
            }
            "id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::IdLessThan(value));
                }
                i += 2;
            }
            "type" if i + 1 < args.len() => {
                conditions.push(FileCondition::Type(args[i + 1].clone()));
                i += 2;
            }
            "path" if i + 1 < args.len() => {
                conditions.push(FileCondition::Path(args[i + 1].clone()));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::GroupId(value));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

fn parse_group_conditions(args: &[String]) -> Vec<GroupCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::Id(value));
                }
                i += 2;
            }
            "name" if i + 1 < args.len() => {
                conditions.push(GroupCondition::Name(args[i + 1].clone()));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "is_primary" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<bool>() {
                    conditions.push(GroupCondition::IsPrimary(value));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

fn parse_tag_conditions(args: &[String]) -> Vec<TagCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(TagCondition::Id(value));
                }
                i += 2;
            }
            "name" if i + 1 < args.len() => {
                conditions.push(TagCondition::Name(args[i + 1].clone()));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(TagCondition::ReferenceCount(value));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

fn parse_file_group_conditions(args: &[String]) -> Vec<FileGroupCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "file_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileGroupCondition::FileId(value));
                }
                i += 2;
            }
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileGroupCondition::GroupId(value));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

fn parse_group_tag_conditions(args: &[String]) -> Vec<GroupTagCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupTagCondition::GroupId(value));
                }
                i += 2;
            }
            "tag_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupTagCondition::TagId(value));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 输入读取辅助函数
fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().parse().unwrap_or(0)
}

fn read_number_optional() -> Option<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            if input.trim().is_empty() {
                None
            } else {
                println!("输入无效，已忽略该字段。");
                None
            }
        }
    }
}

fn read_string_optional() -> Option<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn read_bool_optional() -> Option<bool> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let trimmed = input.trim().to_lowercase();
    if trimmed.is_empty() {
        None
    } else {
        match trimmed.as_str() {
            "true" | "1" | "yes" | "y" => Some(true),
            "false" | "0" | "no" | "n" => Some(false),
            _ => {
                println!("输入无效，已忽略该字段。");
                None
            }
        }
    }
}
