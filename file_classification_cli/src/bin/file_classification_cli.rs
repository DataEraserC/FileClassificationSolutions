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
        #[clap(long)] // 改为只使用长选项
        type_: Option<String>,
        #[clap(long)] // 改为只使用长选项
        reference_count: Option<i32>,
        #[clap(long)] // 改为只使用长选项
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
        #[clap(long)] // 改为只使用长选项
        reference_count: Option<i32>,
        #[clap(long)] // 改为只使用长选项
        is_primary: Option<bool>,
        #[clap(long)] // 改为只使用长选项
        click_count: Option<i32>,
        #[clap(long)] // 改为只使用长选项
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
        #[clap(long)] // 改为只使用长选项
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
            FileActions::ListByConditions { conditions, order_by, limit, offset } => {
                let conditions = parse_file_conditions(conditions);
                let mut options = FileQueryOptions::default();
                options.limit = *limit;
                options.offset = *offset;
                options.order_by = parse_file_order_by(order_by);

                match files::select_files_by_conditions_with_options(&mut conn, conditions, options) {
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
            FileActions::DeleteByConditions { conditions } => {
                let conditions = parse_file_conditions(conditions);
                match files::delete_files_by_conditions(&mut conn, conditions) {
                    Ok(count) => println!("成功删除 {} 条记录", count),
                    Err(e) => eprintln!("删除失败: {:?}", e),
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
                    Ok(count) => println!("成功删除组，影响 {} 行", count),
                    Err(e) => eprintln!("删除组失败: {:?}", e),
                }
            }
            GroupActions::ListInteractive => {
                list_groups_interactive(&mut conn);
            }
            GroupActions::ListByConditions { conditions, order_by, limit, offset } => {
                let conditions = parse_group_conditions(conditions);
                let mut options = GroupQueryOptions::default();
                options.limit = *limit;
                options.offset = *offset;
                options.order_by = parse_group_order_by(order_by);

                match groups::select_groups_by_conditions_with_options(&mut conn, conditions, options) {
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
            GroupActions::DeleteByConditions { conditions } => {
                let conditions = parse_group_conditions(conditions);
                match groups::delete_groups_by_conditions(&mut conn, conditions) {
                    Ok(count) => println!("成功删除 {} 条记录", count),
                    Err(e) => eprintln!("删除失败: {:?}", e),
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
                    Ok(count) => println!("成功删除标签，影响 {} 行", count),
                    Err(e) => eprintln!("删除标签失败: {:?}", e),
                }
            }
            TagActions::ListInteractive => {
                list_tags_interactive(&mut conn);
            }
            TagActions::ListByConditions { conditions, order_by, limit, offset } => {
                let conditions = parse_tag_conditions(conditions);
                let mut options = TagQueryOptions::default();
                options.limit = *limit;
                options.offset = *offset;
                options.order_by = parse_tag_order_by(order_by);

                match tags::select_tags_by_conditions_with_options(&mut conn, conditions, options) {
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
            TagActions::DeleteByConditions { conditions } => {
                let conditions = parse_tag_conditions(conditions);
                match tags::delete_tags_by_conditions(&mut conn, conditions) {
                    Ok(count) => println!("成功删除 {} 条记录", count),
                    Err(e) => eprintln!("删除失败: {:?}", e),
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
            FileGroupActions::ListByConditions { conditions, order_by, limit, offset } => {
                let conditions = parse_file_group_conditions(conditions);
                let mut options = FileGroupQueryOptions::default();
                options.limit = *limit;
                options.offset = *offset;
                options.order_by = parse_file_group_order_by(order_by);

                match file_group::select_file_groups_by_conditions_with_options(&mut conn, conditions, options) {
                    Ok(file_groups) => {
                        println!("查询结果 (共 {} 条记录):", file_groups.len());
                        for fg in file_groups {
                            println!("{:?}", fg);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            FileGroupActions::DeleteByConditions { conditions } => {
                let conditions = parse_file_group_conditions(conditions);
                match file_group::delete_file_groups_by_conditions(&mut conn, conditions) {
                    Ok(count) => println!("成功删除 {} 条记录", count),
                    Err(e) => eprintln!("删除失败: {:?}", e),
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
            GroupTagActions::ListByConditions { conditions, order_by, limit, offset } => {
                let conditions = parse_group_tag_conditions(conditions);
                let mut options = GroupTagQueryOptions::default();
                options.limit = *limit;
                options.offset = *offset;
                options.order_by = parse_group_tag_order_by(order_by);

                match group_tag::select_group_tags_by_conditions_with_options(&mut conn, conditions, options) {
                    Ok(group_tags) => {
                        println!("查询结果 (共 {} 条记录):", group_tags.len());
                        for gt in group_tags {
                            println!("{:?}", gt);
                        }
                    }
                    Err(e) => eprintln!("查询失败: {:?}", e),
                }
            }
            GroupTagActions::DeleteByConditions { conditions } => {
                let conditions = parse_group_tag_conditions(conditions);
                match group_tag::delete_group_tags_by_conditions(&mut conn, conditions) {
                    Ok(count) => println!("成功删除 {} 条记录", count),
                    Err(e) => eprintln!("删除失败: {:?}", e),
                }
            }
        },
    }

    Ok(())
}

// 解析文件条件参数
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
            "id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(FileCondition::IdIn(values));
                }
                i += 2;
            }
            "type" if i + 1 < args.len() => {
                conditions.push(FileCondition::Type(args[i + 1].clone()));
                i += 2;
            }
            "type_like" if i + 1 < args.len() => {
                conditions.push(FileCondition::TypeLike(args[i + 1].clone()));
                i += 2;
            }
            "type_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1].split(',').map(|s| s.to_string()).collect();
                conditions.push(FileCondition::TypeIn(values));
                i += 2;
            }
            "path" if i + 1 < args.len() => {
                conditions.push(FileCondition::Path(args[i + 1].clone()));
                i += 2;
            }
            "path_like" if i + 1 < args.len() => {
                conditions.push(FileCondition::PathLike(args[i + 1].clone()));
                i += 2;
            }
            "path_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1].split(',').map(|s| s.to_string()).collect();
                conditions.push(FileCondition::PathIn(values));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "ref_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::ReferenceCountGreaterThan(value));
                }
                i += 2;
            }
            "ref_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::ReferenceCountLessThan(value));
                }
                i += 2;
            }
            "ref_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(FileCondition::ReferenceCountIn(values));
                }
                i += 2;
            }
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::GroupId(value));
                }
                i += 2;
            }
            "group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::GroupIdGreaterThan(value));
                }
                i += 2;
            }
            "group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileCondition::GroupIdLessThan(value));
                }
                i += 2;
            }
            "group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(FileCondition::GroupIdIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 解析组条件参数
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
            "id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::IdGreaterThan(value));
                }
                i += 2;
            }
            "id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::IdLessThan(value));
                }
                i += 2;
            }
            "id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(GroupCondition::IdIn(values));
                }
                i += 2;
            }
            "name" if i + 1 < args.len() => {
                conditions.push(GroupCondition::Name(args[i + 1].clone()));
                i += 2;
            }
            "name_like" if i + 1 < args.len() => {
                conditions.push(GroupCondition::NameLike(args[i + 1].clone()));
                i += 2;
            }
            "name_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1].split(',').map(|s| s.to_string()).collect();
                conditions.push(GroupCondition::NameIn(values));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "ref_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::ReferenceCountGreaterThan(value));
                }
                i += 2;
            }
            "ref_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::ReferenceCountLessThan(value));
                }
                i += 2;
            }
            "ref_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(GroupCondition::ReferenceCountIn(values));
                }
                i += 2;
            }
            "is_primary" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<bool>() {
                    conditions.push(GroupCondition::IsPrimary(value));
                }
                i += 2;
            }
            "click_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::ClickCount(value));
                }
                i += 2;
            }
            "click_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::ClickCountGreaterThan(value));
                }
                i += 2;
            }
            "click_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::ClickCountLessThan(value));
                }
                i += 2;
            }
            "click_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(GroupCondition::ClickCountIn(values));
                }
                i += 2;
            }
            "share_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::ShareCount(value));
                }
                i += 2;
            }
            "share_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::ShareCountGreaterThan(value));
                }
                i += 2;
            }
            "share_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupCondition::ShareCountLessThan(value));
                }
                i += 2;
            }
            "share_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(GroupCondition::ShareCountIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 解析标签条件参数
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
            "id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(TagCondition::IdGreaterThan(value));
                }
                i += 2;
            }
            "id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(TagCondition::IdLessThan(value));
                }
                i += 2;
            }
            "id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(TagCondition::IdIn(values));
                }
                i += 2;
            }
            "name" if i + 1 < args.len() => {
                conditions.push(TagCondition::Name(args[i + 1].clone()));
                i += 2;
            }
            "name_like" if i + 1 < args.len() => {
                conditions.push(TagCondition::NameLike(args[i + 1].clone()));
                i += 2;
            }
            "name_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1].split(',').map(|s| s.to_string()).collect();
                conditions.push(TagCondition::NameIn(values));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(TagCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "ref_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(TagCondition::ReferenceCountGreaterThan(value));
                }
                i += 2;
            }
            "ref_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(TagCondition::ReferenceCountLessThan(value));
                }
                i += 2;
            }
            "ref_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(TagCondition::ReferenceCountIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 解析文件组关联条件参数
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
            "file_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileGroupCondition::FileIdGreaterThan(value));
                }
                i += 2;
            }
            "file_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileGroupCondition::FileIdLessThan(value));
                }
                i += 2;
            }
            "file_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(FileGroupCondition::FileIdIn(values));
                }
                i += 2;
            }
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileGroupCondition::GroupId(value));
                }
                i += 2;
            }
            "group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileGroupCondition::GroupIdGreaterThan(value));
                }
                i += 2;
            }
            "group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(FileGroupCondition::GroupIdLessThan(value));
                }
                i += 2;
            }
            "group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(FileGroupCondition::GroupIdIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 解析组标签关联条件参数
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
            "group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupTagCondition::GroupIdGreaterThan(value));
                }
                i += 2;
            }
            "group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupTagCondition::GroupIdLessThan(value));
                }
                i += 2;
            }
            "group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(GroupTagCondition::GroupIdIn(values));
                }
                i += 2;
            }
            "tag_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupTagCondition::TagId(value));
                }
                i += 2;
            }
            "tag_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupTagCondition::TagIdGreaterThan(value));
                }
                i += 2;
            }
            "tag_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(GroupTagCondition::TagIdLessThan(value));
                }
                i += 2;
            }
            "tag_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1]
                    .split(',')
                    .map(|s| s.parse::<i32>())
                    .collect();
                if let Ok(values) = values {
                    conditions.push(GroupTagCondition::TagIdIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

// 解析文件排序参数
fn parse_file_order_by(args: &[String]) -> Vec<FileOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => OrderDirection::Asc,
            "desc" => OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "id" => FileOrderBy::Id(direction),
            "type" => FileOrderBy::Type(direction),
            "path" => FileOrderBy::Path(direction),
            "ref_count" => FileOrderBy::ReferenceCount(direction),
            "group_id" => FileOrderBy::GroupId(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

// 解析组排序参数
fn parse_group_order_by(args: &[String]) -> Vec<GroupOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => OrderDirection::Asc,
            "desc" => OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "id" => GroupOrderBy::Id(direction),
            "name" => GroupOrderBy::Name(direction),
            "ref_count" => GroupOrderBy::ReferenceCount(direction),
            "is_primary" => GroupOrderBy::IsPrimary(direction),
            "click_count" => GroupOrderBy::ClickCount(direction),
            "share_count" => GroupOrderBy::ShareCount(direction),
            "create_time" => GroupOrderBy::CreateTime(direction),
            "modify_time" => GroupOrderBy::ModifyTime(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

// 解析标签排序参数
fn parse_tag_order_by(args: &[String]) -> Vec<TagOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => OrderDirection::Asc,
            "desc" => OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "id" => TagOrderBy::Id(direction),
            "name" => TagOrderBy::Name(direction),
            "ref_count" => TagOrderBy::ReferenceCount(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

// 解析文件组关联排序参数
fn parse_file_group_order_by(args: &[String]) -> Vec<FileGroupOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => OrderDirection::Asc,
            "desc" => OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "file_id" => FileGroupOrderBy::FileId(direction),
            "group_id" => FileGroupOrderBy::GroupId(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

// 解析组标签关联排序参数
fn parse_group_tag_order_by(args: &[String]) -> Vec<GroupTagOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => OrderDirection::Asc,
            "desc" => OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "group_id" => GroupTagOrderBy::GroupId(direction),
            "tag_id" => GroupTagOrderBy::TagId(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

// 交互式查询函数（示例）
fn list_files_interactive(conn: &mut AnyConnection) {
    println!("交互式文件查询功能待实现");
}

fn list_groups_interactive(conn: &mut AnyConnection) {
    println!("交互式组查询功能待实现");
}

fn list_tags_interactive(conn: &mut AnyConnection) {
    println!("交互式标签查询功能待实现");
}

fn list_file_groups_interactive(conn: &mut AnyConnection) {
    println!("交互式文件组关联查询功能待实现");
}

fn list_group_tags_interactive(conn: &mut AnyConnection) {
    println!("交互式组标签关联查询功能待实现");
}
