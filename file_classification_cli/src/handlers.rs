use crate::cli_types::{Cli, Commands, FileActions, GroupActions, TagActions, FileGroupActions, GroupTagActions};
use crate::context::Context;
use crate::eval_interpreter::{FileService, GroupService, TagService};
use crate::file_group::FileGroupService;
use crate::group_tag::GroupTagService;
use file_classification_core::model::models::*;
use file_classification_core::service::*;
use file_classification_core::utils::database::AnyConnection;
use std::fs::File;
use std::io::Read;
use std::error::Error;

/// 处理命令行命令
pub fn handle_command(command: Cli, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    match command.command {
        Commands::Script { file } => {
            let mut script_file = File::open(&file)?;
            let mut content = String::new();
            script_file.read_to_string(&mut content)?;
            
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                // 处理变量赋值
                if line.contains('=') {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim();
                        let value = parts[1].trim();
                        context.set(key, value);
                    }
                    continue;
                }
                
                // 处理命令
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                let cmd = parts[0].trim();
                let args = if parts.len() > 1 { parts[1].trim() } else { "" };
                
                match cmd {
                    "file" => handle_file_command(args, conn, context)?,
                    "group" => handle_group_command(args, conn, context)?,
                    "tag" => handle_tag_command(args, conn, context)?,
                    "file_group" => handle_file_group_command(args, conn, context)?,
                    "group_tag" => handle_group_tag_command(args, conn, context)?,
                    _ => println!("未知命令: {}", cmd),
                }
            }
            
            Ok(())
        },
        Commands::File{action} => handle_file_action(action, conn, context),
        Commands::Group{action} => handle_group_action(action, conn, context),
        Commands::Tag{action} => handle_tag_action(action, conn, context),
        Commands::FileGroup{action} => handle_file_group_action(action, conn, context),
        Commands::GroupTag{action} => handle_group_tag_action(action, conn, context),
        Commands::Repl => {
            // REPL模式在其他地方处理
            Ok(())
        },
    }
}

fn handle_file_command(args: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.is_empty() {
        return Err("文件命令需要指定操作".into());
    }
    
    let action = parts[0];
    let params = if parts.len() > 1 { parts[1] } else { "" };
    
    match action {
        "create" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.is_empty() {
                return Err("创建文件需要提供路径".into());
            }
            
            let path = params[0];
            let type_ = if params.len() > 1 { Some(params[1]) } else { None };
            let group_id = if params.len() > 2 { 
                Some(params[2].parse::<i32>()?) 
            } else { 
                None 
            };
            
            let mut file_service = FileService::new(conn);
            let file = file_service.create_file(path, type_.as_deref(), group_id)?;
            println!("文件创建成功: ID={}, 路径={}", file.id, file.path);
        },
        "get" => {
            let id = params.parse::<i32>()?;
            let mut file_service = FileService::new(conn);
            let file = file_service.get_file(id)?;
            println!("ID: {}", file.id);
            println!("路径: {}", file.path);
            println!("类型: {}", file.type_);
            println!("引用计数: {}", file.reference_count);
            println!("组ID: {}", match file.group_id {
                Some(id) => id.to_string(),
                None => "无".to_string()
            });
        },
        "list" => {
            let mut file_service = FileService::new(conn);
            let files = file_service.list_files(None, None, None, None)?;
            
            if files.is_empty() {
                println!("没有文件");
            } else {
                println!("ID\t类型\t引用计数\t路径");
                for file in files {
                    println!("{}\t{}\t{}\t{}", 
                        file.id, 
                        file.type_, 
                        file.reference_count, 
                        file.path);
                }
            }
        },
        "update" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("更新文件需要提供ID和要更新的字段".into());
            }
            
            let id = params[0].parse::<i32>()?;
            let field = params[1];
            let value = if params.len() > 2 { Some(params[2]) } else { None };
            
            let mut file_service = FileService::new(conn);
            match field {
                "type" => {
                    file_service.update_files(&format!("id={}", id), None, value.as_deref(), None, None)?;
                    println!("文件类型已更新");
                },
                "group" => {
                    let group_id = value.map(|v| v.parse::<i32>()).transpose()?;
                    file_service.update_files(&format!("id={}", id), None, None, None, group_id)?;
                    println!("文件组ID已更新");
                },
                _ => return Err(format!("不支持更新字段: {}", field).into()),
            }
        },
        "delete" => {
            let id = params.parse::<i32>()?;
            let mut file_service = FileService::new(conn);
            file_service.delete_file(id)?;
            println!("文件已删除");
        },
        _ => return Err(format!("未知的文件操作: {}", action).into()),
    }
    
    Ok(())
}

fn handle_group_command(args: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.is_empty() {
        return Err("组命令需要指定操作".into());
    }
    
    let action = parts[0];
    let params = if parts.len() > 1 { parts[1] } else { "" };
    
    match action {
        "create" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.is_empty() {
                return Err("创建组需要提供名称".into());
            }
            
            let name = params[0];
            let is_primary = if params.len() > 1 { 
                params[1].parse::<bool>()? 
            } else { 
                false 
            };
            
            let mut group_service = GroupService::new(conn);
            let group = group_service.create_group(name)?;
            println!("组创建成功: ID={}, 名称={}", group.id, group.name);
        },
        "get" => {
            let id = params.parse::<i32>()?;
            let mut group_service = GroupService::new(conn);
            let group = group_service.get_group(id)?;
            println!("ID: {}", group.id);
            println!("名称: {}", group.name);
            println!("引用计数: {}", group.reference_count);
            println!("是否主组: {}", group.is_primary);
            println!("点击计数: {}", group.click_count);
            println!("分享计数: {}", group.share_count);
            println!("创建时间: {}", group.create_time);
            println!("修改时间: {}", group.modify_time);
        },
        "list" => {
            let mut group_service = GroupService::new(conn);
            let groups = group_service.list_groups(None, None, None)?;
            
            if groups.is_empty() {
                println!("没有组");
            } else {
                println!("ID\t名称\t引用计数\t是否主组");
                for group in groups {
                    println!("{}\t{}\t{}\t{}", 
                        group.id, 
                        group.name, 
                        group.reference_count, 
                        group.is_primary);
                }
            }
        },
        "update" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("更新组需要提供ID和要更新的字段".into());
            }
            
            let id = params[0].parse::<i32>()?;
            let field = params[1];
            let value = if params.len() > 2 { Some(params[2]) } else { None };
            
            let mut group_service = GroupService::new(conn);
            match field {
                "name" => {
                    let name = value.ok_or("更新名称需要提供值")?;
                    group_service.update_group(id, Some(name))?;
                    println!("组名称已更新");
                },
                "is_primary" => {
                    // 由于update_group方法只支持更新名称，需要使用core库的方法直接更新
                    let is_primary = value.map(|v| v.parse::<bool>()).transpose()?.ok_or("更新主组状态需要提供值")?;
                    let mut changes = file_classification_core::model::models::UpdateGroupDTO::default();
                    changes.is_primary = Some(is_primary);
                    file_classification_core::service::groups::update_group_by_id(conn, id, changes)?;
                    println!("组主组状态已更新");
                },
                "click_count" => {
                    // 由于update_group方法只支持更新名称，需要使用core库的方法直接更新
                    let click_count = value.map(|v| v.parse::<i32>()).transpose()?.ok_or("更新点击计数需要提供值")?;
                    let mut changes = file_classification_core::model::models::UpdateGroupDTO::default();
                    changes.click_count = Some(click_count);
                    file_classification_core::service::groups::update_group_by_id(conn, id, changes)?;
                    println!("组点击计数已更新");
                },
                "share_count" => {
                    // 由于update_group方法只支持更新名称，需要使用core库的方法直接更新
                    let share_count = value.map(|v| v.parse::<i32>()).transpose()?.ok_or("更新分享计数需要提供值")?;
                    let mut changes = file_classification_core::model::models::UpdateGroupDTO::default();
                    changes.share_count = Some(share_count);
                    file_classification_core::service::groups::update_group_by_id(conn, id, changes)?;
                    println!("组分享计数已更新");
                },
                _ => return Err(format!("不支持更新字段: {}", field).into()),
            }
        },
        "delete" => {
            let id = params.parse::<i32>()?;
            let mut group_service = GroupService::new(conn);
            group_service.delete_group(id)?;
            println!("组已删除");
        },
        "files" => {
            let id = params.parse::<i32>()?;
            let mut file_group_service = FileGroupService::new(conn);
            let files = file_group_service.list_files_in_group(id)?;
            
            if files.is_empty() {
                println!("组 {} 没有文件", id);
            } else {
                println!("ID\t类型\t引用计数\t路径");
                for file in files {
                    println!("{}\t{}\t{}\t{}", 
                        file.id, 
                        file.type_, 
                        file.reference_count, 
                        file.path);
                }
            }
        },
        "tags" => {
            let id = params.parse::<i32>()?;
            let mut group_tag_service = GroupTagService::new(conn);
            let tags = group_tag_service.list_tags_for_group(id)?;
            
            if tags.is_empty() {
                println!("组 {} 没有标签", id);
            } else {
                println!("ID\t名称\t引用计数");
                for tag in tags {
                    println!("{}\t{}\t{}", 
                        tag.id, 
                        tag.name, 
                        tag.reference_count);
                }
            }
        },
        _ => return Err(format!("未知的组操作: {}", action).into()),
    }
    
    Ok(())
}

fn handle_tag_command(args: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.is_empty() {
        return Err("标签命令需要指定操作".into());
    }
    
    let action = parts[0];
    let params = if parts.len() > 1 { parts[1] } else { "" };
    
    match action {
        "create" => {
            let name = params;
            if name.is_empty() {
                return Err("创建标签需要提供名称".into());
            }
            
            let mut tag_service = TagService::new(conn);
            let tag = tag_service.create_tag(name, None)?;
            println!("标签创建成功: ID={}, 名称={}", tag.id, tag.name);
        },
        "get" => {
            let id = params.parse::<i32>()?;
            let mut tag_service = TagService::new(conn);
            let tag = tag_service.get_tag(id)?;
            println!("ID: {}", tag.id);
            println!("名称: {}", tag.name);
            println!("引用计数: {}", tag.reference_count);
        },
        "list" => {
            let mut tag_service = TagService::new(conn);
            let tags = tag_service.list_tags(None, None, None)?;
            
            if tags.is_empty() {
                println!("没有标签");
            } else {
                println!("ID\t名称\t引用计数");
                for tag in tags {
                    println!("{}\t{}\t{}", 
                        tag.id, 
                        tag.name, 
                        tag.reference_count);
                }
            }
        },
        "update" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("更新标签需要提供ID和新名称".into());
            }
            
            let id = params[0].parse::<i32>()?;
            let name = Some(params[1].to_string());
            
            let mut tag_service = TagService::new(conn);
            tag_service.update_tag(id, name, None)?;
            println!("标签已更新");
        },
        "delete" => {
            let id = params.parse::<i32>()?;
            let mut tag_service = TagService::new(conn);
            tag_service.delete_tag(id)?;
            println!("标签已删除");
        },
        "groups" => {
            let id = params.parse::<i32>()?;
            let mut group_tag_service = GroupTagService::new(conn);
            let groups = group_tag_service.list_groups_with_tag(id)?;
            
            if groups.is_empty() {
                println!("标签 {} 没有关联的组", id);
            } else {
                println!("ID\t名称\t引用计数\t是否主组");
                for group in groups {
                    println!("{}\t{}\t{}\t{}", 
                        group.id, 
                        group.name, 
                        group.reference_count, 
                        group.is_primary);
                }
            }
        },
        _ => return Err(format!("未知的标签操作: {}", action).into()),
    }
    
    Ok(())
}

fn handle_file_group_command(args: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.is_empty() {
        return Err("文件组关联命令需要指定操作".into());
    }
    
    let action = parts[0];
    let params = if parts.len() > 1 { parts[1] } else { "" };
    
    match action {
        "link" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("链接文件到组需要提供文件ID和组ID".into());
            }
            
            let file_id = params[0].parse::<i32>()?;
            let group_id = params[1].parse::<i32>()?;
            
            let mut file_group_service = FileGroupService::new(conn);
            file_group_service.link_file_to_group(file_id, group_id)?;
            println!("文件 {} 已链接到组 {}", file_id, group_id);
        },
        "unlink" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("从组解除链接文件需要提供文件ID和组ID".into());
            }
            
            let file_id = params[0].parse::<i32>()?;
            let group_id = params[1].parse::<i32>()?;
            
            let mut file_group_service = FileGroupService::new(conn);
            file_group_service.unlink_file_from_group(file_id, group_id)?;
            println!("文件 {} 已从组 {} 解除链接", file_id, group_id);
        },
        _ => return Err(format!("未知的文件组关联操作: {}", action).into()),
    }
    
    Ok(())
}

fn handle_group_tag_command(args: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.is_empty() {
        return Err("组标签关联命令需要指定操作".into());
    }
    
    let action = parts[0];
    let params = if parts.len() > 1 { parts[1] } else { "" };
    
    match action {
        "link" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("链接组到标签需要提供组ID和标签ID".into());
            }
            
            let group_id = params[0].parse::<i32>()?;
            let tag_id = params[1].parse::<i32>()?;
            
            let mut group_tag_service = GroupTagService::new(conn);
            group_tag_service.link_tag_to_group(group_id, tag_id)?;
            println!("组 {} 已链接到标签 {}", group_id, tag_id);
        },
        "unlink" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("从标签解除链接组需要提供组ID和标签ID".into());
            }
            
            let group_id = params[0].parse::<i32>()?;
            let tag_id = params[1].parse::<i32>()?;
            
            let mut group_tag_service = GroupTagService::new(conn);
            group_tag_service.unlink_tag_from_group(group_id, tag_id)?;
            println!("组 {} 已从标签 {} 解除链接", group_id, tag_id);
        },
        _ => return Err(format!("未知的组标签关联操作: {}", action).into()),
    }
    
    Ok(())
}

fn handle_file_action(action: FileActions, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    match action {
        FileActions::Create { type_, path, group_id } => {
            let path = path.ok_or("需要提供文件路径")?;
            let mut file_service = FileService::new(conn);
            let file = file_service.create_file(&path, type_.as_deref(), group_id)?;
            println!("文件创建成功: ID={}, 路径={}", file.id, file.path);
        },
        FileActions::Get { id } => {
            let mut file_service = FileService::new(conn);
            let file = file_service.get_file(id)?;
            println!("ID: {}", file.id);
            println!("路径: {}", file.path);
            println!("类型: {}", file.type_);
            println!("引用计数: {}", file.reference_count);
            println!("组ID: {}", file.group_id.map_or("无".to_string(), |id| id.to_string()));
        },
        FileActions::List => {
            let mut file_service = FileService::new(conn);
            let files = file_service.list_files(None, None, None)?;
            
            if files.is_empty() {
                println!("没有文件");
            } else {
                println!("ID\t类型\t引用计数\t路径");
                for file in files {
                    println!("{}\t{}\t{}\t{}", 
                        file.id, 
                        file.type_, 
                        file.reference_count, 
                        file.path);
                }
            }
        },
        FileActions::ListByConditions { conditions, order_by, limit, offset } => {
            let mut file_service = FileService::new(conn);
            let files = file_service.list_files(
                if conditions.is_empty() { None } else { Some(&conditions.join(" ")) },
                limit,
                offset,
            )?;
            
            if files.is_empty() {
                println!("没有符合条件的文件");
            } else {
                println!("ID\t类型\t引用计数\t路径");
                for file in files {
                    println!("{}\t{}\t{}\t{}", 
                        file.id, 
                        file.type_, 
                        file.reference_count, 
                        file.path);
                }
            }
        },
        FileActions::Update { id, type_, group_id } => {
            let mut file_service = FileService::new(conn);
            file_service.update_file(id, None, type_.as_deref(), group_id)?;
            println!("文件已更新");
        },
        FileActions::Delete { id } => {
            let mut file_service = FileService::new(conn);
            file_service.delete_file(id)?;
            println!("文件已删除");
        },
    }
    
    Ok(())
}

fn handle_group_action(action: GroupActions, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    match action {
        GroupActions::Create { name, is_primary } => {
            let mut group_service = GroupService::new(conn);
            let group = group_service.create_group(&name, is_primary)?;
            println!("组创建成功: ID={}, 名称={}", group.id, group.name);
        },
        GroupActions::Get { id } => {
            let mut group_service = GroupService::new(conn);
            let group = group_service.get_group(id)?;
            println!("ID: {}", group.id);
            println!("名称: {}", group.name);
            println!("引用计数: {}", group.reference_count);
            println!("是否主组: {}", group.is_primary);
            println!("点击计数: {}", group.click_count);
            println!("分享计数: {}", group.share_count);
            println!("创建时间: {}", group.create_time);
            println!("修改时间: {}", group.modify_time);
        },
        GroupActions::List => {
            let mut group_service = GroupService::new(conn);
            let groups = group_service.list_groups(None, None, None)?;
            
            if groups.is_empty() {
                println!("没有组");
            } else {
                println!("ID\t名称\t引用计数\t是否主组");
                for group in groups {
                    println!("{}\t{}\t{}\t{}", 
                        group.id, 
                        group.name, 
                        group.reference_count, 
                        group.is_primary);
                }
            }
        },
        GroupActions::ListByConditions { conditions, order_by, limit, offset } => {
            let mut group_service = GroupService::new(conn);
            let groups = group_service.list_groups(
                if conditions.is_empty() { None } else { Some(&conditions.join(" ")) },
                limit,
                offset,
            )?;
            
            if groups.is_empty() {
                println!("没有符合条件的组");
            } else {
                println!("ID\t名称\t引用计数\t是否主组");
                for group in groups {
                    println!("{}\t{}\t{}\t{}", 
                        group.id, 
                        group.name, 
                        group.reference_count, 
                        group.is_primary);
                }
            }
        },
        GroupActions::Update { id, name, is_primary, click_count, share_count } => {
            let mut group_service = GroupService::new(conn);
            group_service.update_group(id, name.as_deref(), is_primary, click_count, share_count, None)?;
            println!("组已更新");
        },
        GroupActions::Delete { id } => {
            let mut group_service = GroupService::new(conn);
            group_service.delete_group(id)?;
            println!("组已删除");
        },
        GroupActions::ListFiles { id } => {
            let mut file_group_service = FileGroupService::new(conn);
            let files = file_group_service.list_files_in_group(id)?;
            
            if files.is_empty() {
                println!("组 {} 没有文件", id);
            } else {
                println!("ID\t类型\t引用计数\t路径");
                for file in files {
                    println!("{}\t{}\t{}\t{}", 
                        file.id, 
                        file.type_, 
                        file.reference_count, 
                        file.path);
                }
            }
        },
        GroupActions::ListTags { id } => {
            let mut group_tag_service = GroupTagService::new(conn);
            let tags = group_tag_service.list_tags_for_group(id)?;
            
            if tags.is_empty() {
                println!("组 {} 没有标签", id);
            } else {
                println!("ID\t名称\t引用计数");
                for tag in tags {
                    println!("{}\t{}\t{}", 
                        tag.id, 
                        tag.name, 
                        tag.reference_count);
                }
            }
        },
    }
    
    Ok(())
}

fn handle_tag_action(action: TagActions, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    match action {
        TagActions::Create { name } => {
            let mut tag_service = TagService::new(conn);
            let tag = tag_service.create_tag(&name, None)?;
            println!("标签创建成功: ID={}, 名称={}", tag.id, tag.name);
        },
        TagActions::Get { id } => {
            let mut tag_service = TagService::new(conn);
            let tag = tag_service.get_tag(id)?;
            println!("ID: {}", tag.id);
            println!("名称: {}", tag.name);
            println!("引用计数: {}", tag.reference_count);
        },
        TagActions::List => {
            let mut tag_service = TagService::new(conn);
            let tags = tag_service.list_tags(None, None, None)?;
            
            if tags.is_empty() {
                println!("没有标签");
            } else {
                println!("ID\t名称\t引用计数");
                for tag in tags {
                    println!("{}\t{}\t{}", 
                        tag.id, 
                        tag.name, 
                        tag.reference_count);
                }
            }
        },
        TagActions::ListByConditions { conditions, order_by, limit, offset } => {
            let mut tag_service = TagService::new(conn);
            let tags = tag_service.list_tags(
                if conditions.is_empty() { None } else { Some(&conditions.join(" ")) },
                limit,
                offset,
            )?;
            
            if tags.is_empty() {
                println!("没有符合条件的标签");
            } else {
                println!("ID\t名称\t引用计数");
                for tag in tags {
                    println!("{}\t{}\t{}", 
                        tag.id, 
                        tag.name, 
                        tag.reference_count);
                }
            }
        },
        TagActions::Update { id, name } => {
            let mut tag_service = TagService::new(conn);
            tag_service.update_tag(id, name.as_deref(), None)?;
            println!("标签已更新");
        },
        TagActions::Delete { id } => {
            let mut tag_service = TagService::new(conn);
            tag_service.delete_tag(id)?;
            println!("标签已删除");
        },
        TagActions::ListGroups { id } => {
            let mut group_tag_service = GroupTagService::new(conn);
            let groups = group_tag_service.list_groups_with_tag(id)?;
            
            if groups.is_empty() {
                println!("标签 {} 没有关联的组", id);
            } else {
                println!("ID\t名称\t引用计数\t是否主组");
                for group in groups {
                    println!("{}\t{}\t{}\t{}", 
                        group.id, 
                        group.name, 
                        group.reference_count, 
                        group.is_primary);
                }
            }
        },
    }
    
    Ok(())
}

fn handle_file_group_action(action: FileGroupActions, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    match action {
        FileGroupActions::Link { file_id, group_id } => {
            let mut file_group_service = FileGroupService::new(conn);
            file_group_service.link_file_to_group(file_id, group_id)?;
            println!("文件 {} 已链接到组 {}", file_id, group_id);
        },
        FileGroupActions::Unlink { file_id, group_id } => {
            let mut file_group_service = FileGroupService::new(conn);
            file_group_service.unlink_file_from_group(file_id, group_id)?;
            println!("文件 {} 已从组 {} 解除链接", file_id, group_id);
        },
    }
    
    Ok(())
}

fn handle_group_tag_action(action: GroupTagActions, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    match action {
        GroupTagActions::Link { group_id, tag_id } => {
            let mut group_tag_service = GroupTagService::new(conn);
            group_tag_service.link_tag_to_group(group_id, tag_id)?;
            println!("组 {} 已链接到标签 {}", group_id, tag_id);
        },
        GroupTagActions::Unlink { group_id, tag_id } => {
            let mut group_tag_service = GroupTagService::new(conn);
            group_tag_service.unlink_tag_from_group(group_id, tag_id)?;
            println!("组 {} 已从标签 {} 解除链接", group_id, tag_id);
        },
    }
    
    Ok(())
}