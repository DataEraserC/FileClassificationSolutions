// handlers.rs
// 处理各种命令的核心逻辑，高内聚，每个子模块处理一类操作

use clap::Parser;
use file_classification_core::model::models;
use file_classification_core::service;
use file_classification_core::utils::database::AnyConnection;
use std::error::Error;
use std::io::{BufRead, Read};

use crate::cli::{self, Cli, Commands};
use crate::context::Context;
use crate::helpers::{confirm_deletion, get_input};
use crate::interactive::{
	list_file_groups_interactive, list_files_interactive, list_group_tags_interactive,
	list_groups_interactive, list_tags_interactive,
};
use crate::parsers::{
	parse_file_conditions, parse_file_group_conditions, parse_file_group_order_by,
	parse_file_order_by, parse_group_conditions, parse_group_order_by,
	parse_group_relation_conditions, parse_group_relation_order_by, parse_group_tag_conditions,
	parse_group_tag_order_by, parse_tag_conditions, parse_tag_order_by,
};
use crate::repl::run_repl;

/// 处理单个CLI命令
pub fn handle_command(
	cli: Cli,
	conn: &mut AnyConnection,
	context: &mut Context,
) -> Result<(), Box<dyn Error>> {
	match cli.command {
		Commands::Script { file } => handle_script(&file, conn, context),
		Commands::Repl => run_repl(conn, context),
		Commands::File { action } => handle_file_action(action, conn, context),
		Commands::Group { action } => handle_group_action(action, conn, context),
		Commands::Tag { action } => handle_tag_action(action, conn, context),
		Commands::FileGroup { action } => handle_file_group_action(action, conn, context),
		Commands::GroupTag { action } => handle_group_tag_action(action, conn, context),
		Commands::GroupRelation { action } => handle_group_relation_action(action, conn, context),
	}
}

/// 处理脚本执行命令
fn handle_script(
	file: &str,
	conn: &mut AnyConnection,
	context: &mut Context,
) -> Result<(), Box<dyn Error>> {
	let mut script_file = std::fs::File::open(file)?;
	let mut content = String::new();
	script_file.read_to_string(&mut content)?;

	println!("执行脚本: {}", file);
	for line in content.lines() {
		let line = line.trim();
		if line.is_empty() || line.starts_with('#') {
			continue; // 跳过空行和注释
		}

		println!("执行命令: {}", line);

		// 支持分号分隔的多命令
		let commands: Vec<&str> = line.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
		for command in commands {
			// 尝试解析为标准命令
			let args = shlex::split(command).unwrap_or_default();
			let cli_args = std::iter::once("file_classification_cli".to_string()).chain(args.into_iter());

			if let Ok(cmd) = cli::Cli::try_parse_from(cli_args) {
				if let Err(e) = handle_command(cmd, conn, context) {
					println!("命令执行错误: {}", e);
				}
			} else {
				// 尝试作为简化命令处理
				if !handle_simplified_command(command, conn, context) {
					println!("无法解析命令: {}", command);
				}
			}
		}
	}
	println!("脚本执行完成");
	Ok(())
}

/// 处理简化命令（如ls, cd等）
pub fn handle_simplified_command(
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
				if let Ok(cmd) = cli::Cli::try_parse_from(args) {
					let _ = handle_command(cmd, conn, context);
				}
			} else if let Some(group_id) = context.selected_group_id {
				println!("列出组ID {} 相关的文件:", group_id);
				let group_id_str = group_id.to_string();
				let args = vec!["file", "list-by-group-id", "--group_id", &group_id_str];
				if let Ok(cmd) = cli::Cli::try_parse_from(args) {
					let _ = handle_command(cmd, conn, context);
				}

				println!("列出组ID {} 相关的标签:", group_id);
				let group_id_str = group_id.to_string();
				let args = vec!["tag", "list-by-group-id", "--group_id", &group_id_str];
				if let Ok(cmd) = cli::Cli::try_parse_from(args) {
					let _ = handle_command(cmd, conn, context);
				}
			} else if let Some(tag_id) = context.selected_tag_id {
				println!("列出标签ID {} 相关的组:", tag_id);
				let tag_id_str = tag_id.to_string();
				let args = vec!["group", "list-by-tag-id", "--tag_id", &tag_id_str];
				if let Ok(cmd) = cli::Cli::try_parse_from(args) {
					let _ = handle_command(cmd, conn, context);
				}
			} else {
				println!("没有选中任何项目，列出所有组:");
				let args = vec!["group", "list-interactive"];
				if let Ok(cmd) = cli::Cli::try_parse_from(args) {
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

			// 假设是组ID
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
					if let Ok(cmd) = cli::Cli::try_parse_from(args) {
						let _ = handle_command(cmd, conn, context);
					}
				}
				"tag" => {
					let args = vec!["tag", "create", "--name", parts[2]];
					if let Ok(cmd) = cli::Cli::try_parse_from(args) {
						let _ = handle_command(cmd, conn, context);
					}
				}
				"file" => {
					let mut args = vec!["file", "create", "--path", parts[2], "--type", "regular"];
					let group_id_str;
					if let Some(group_id) = context.selected_group_id {
						args.push("--group_id");
						group_id_str = group_id.to_string();
						args.push(&group_id_str);
					}
					if let Ok(cmd) = cli::Cli::try_parse_from(args) {
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
					if let Ok(cmd) = cli::Cli::try_parse_from(args) {
						let _ = handle_command(cmd, conn, context);
					}
				}
				"group" => {
					let args = vec!["group", "delete", "--id", parts[2]];
					if let Ok(cmd) = cli::Cli::try_parse_from(args) {
						let _ = handle_command(cmd, conn, context);
					}
				}
				"tag" => {
					let args = vec!["tag", "delete", "--id", parts[2]];
					if let Ok(cmd) = cli::Cli::try_parse_from(args) {
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

/// 处理文件相关动作
fn handle_file_action(
	action: cli::FileActions,
	conn: &mut AnyConnection,
	context: &mut Context,
) -> Result<(), Box<dyn Error>> {
	match action {
		cli::FileActions::Create { type_, path, group_id } => {
			let type_ = type_.unwrap_or_else(|| get_input("请输入文件类型: "));
			let path = path.unwrap_or_else(|| get_input("请输入文件路径: "));
			let group_id =
				group_id.unwrap_or_else(|| get_input("请输入组 ID: ").parse().expect("无效的组 ID"));

			let dto = models::CreateFileDTO { type_, path, group_id };
			match service::files::create_file(conn, dto) {
				Ok(count) => println!("成功创建文件，影响 {} 行", count),
				Err(e) => eprintln!("创建文件失败: {:?}", e),
			}
		}
		cli::FileActions::Delete { id } => {
			let file_id = id
				.or(context.selected_file_id)
				.ok_or("错误：未提供文件 ID，也未在上下文中选中任何文件。")?;

			if confirm_deletion(&format!("确定要删除 ID 为 {} 的文件吗? (y/n): ", file_id)) {
				match service::files::delete_file(conn, file_id) {
					Ok(_) => {
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
		cli::FileActions::ListInteractive => list_files_interactive(conn, context),
		cli::FileActions::ListByConditions { conditions, order_by, limit, offset } => {
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
						if let Some(first_file) = results.first() {
							context.selected_file_id = Some(first_file.id);
							println!("\n提示：第一个文件的 ID ({}) 已被选中，可用于后续操作。", first_file.id);
						}
					}
				}
				Err(e) => eprintln!("查询文件失败: {:?}", e),
			}
		}
		cli::FileActions::ListByGroupId { group_id } => {
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
		cli::FileActions::UpdateById { id, path, type_, reference_count, group_id } => {
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

			let conditions = vec![models::FileCondition::Id(id)];
			match service::files::update_files_by_conditions(conn, conditions, changes) {
				Ok(count) => println!("成功更新 {} 个文件", count),
				Err(e) => eprintln!("更新文件失败: {:?}", e),
			}
		}
		cli::FileActions::UpdateByConditions { conditions, path, type_, reference_count, group_id } => {
			let update_conditions = if conditions.is_empty() {
				if let Some(selected_id) = context.selected_file_id {
					vec![format!("id={}", selected_id)]
				} else {
					return Err("错误：未提供更新条件，也未在上下文中选中任何文件。".into());
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
		cli::FileActions::DeleteByConditions { conditions } => {
			let conditions = parse_file_conditions(&conditions);
			match service::files::delete_files_by_conditions(conn, conditions) {
				Ok(count) => println!("成功删除 {} 条记录", count),
				Err(e) => eprintln!("删除失败: {:?}", e),
			}
		}
	}
	Ok(())
}

/// 处理组相关操作
fn handle_group_action(
	action: cli::GroupActions,
	conn: &mut AnyConnection,
	context: &mut Context,
) -> Result<(), Box<dyn Error>> {
	match action {
		cli::GroupActions::Create { name } => {
			let name = name.unwrap_or_else(|| get_input("请输入组名称: "));
			let create_dto = models::CreateGroupDTO { name };
			match service::groups::create_group(conn, &create_dto) {
				Ok(id) => println!("组创建成功，ID: {}", id),
				Err(e) => println!("组创建失败: {}", e),
			}
		}
		cli::GroupActions::Delete { id } => {
			let group_id =
				id.or(context.selected_group_id).ok_or("错误：未提供组 ID，也未在上下文中选中任何组。")?;

			if confirm_deletion(&format!("确定要删除 ID 为 {} 的组吗? (y/n): ", group_id)) {
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
		cli::GroupActions::ListInteractive => {
			list_groups_interactive(conn, context);
		}
		cli::GroupActions::ListByConditions { conditions, order_by, limit, offset } => {
			let conditions = parse_group_conditions(&conditions);
			let mut options = models::GroupQueryOptions::default();
			options.limit = limit;
			options.offset = offset;
			options.order_by = parse_group_order_by(&order_by);

			match service::groups::select_groups_by_conditions_with_options(conn, conditions, options) {
				Ok(results) => {
					if results.is_empty() {
						println!("未找到匹配的组。");
					} else {
						println!("查询结果:");
						for group in &results {
							println!("  - ID: {}, Name: {}", group.id, group.name);
						}
						if let Some(first_group) = results.first() {
							context.selected_group_id = Some(first_group.id);
							println!("\n提示：第一个组的 ID ({}) 已被选中，可用于后续操作。", first_group.id);
						}
					}
				}
				Err(e) => eprintln!("查询组失败: {:?}", e),
			}
		}
		cli::GroupActions::ListByFileId { file_id } => {
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
		cli::GroupActions::ListByTagId { tag_id } => {
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
		cli::GroupActions::GetTree { id } => match service::groups::get_group_tree(conn, id) {
			Ok(tree) => {
				println!("组树状结构:");
				print_group_tree(&tree, 0);
			}
			Err(e) => println!("获取组树状结构失败: {}", e),
		},
		cli::GroupActions::UpdateById {
			id,
			name,
			reference_count,
			is_primary,
			click_count,
			share_count,
		} => {
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

			let conditions = vec![models::GroupCondition::Id(id)];
			match service::groups::update_groups_by_conditions(conn, conditions, changes) {
				Ok(count) => println!("成功更新 {} 个组", count),
				Err(e) => eprintln!("更新组失败: {:?}", e),
			}
		}
		cli::GroupActions::UpdateByConditions {
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
					return Err("错误：未提供更新条件，也未在上下文中选中任何组。".into());
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
		cli::GroupActions::DeleteByConditions { conditions } => {
			let conditions = parse_group_conditions(&conditions);
			match service::groups::delete_groups_by_conditions(conn, conditions) {
				Ok(count) => println!("成功删除 {} 条记录", count),
				Err(e) => eprintln!("删除失败: {:?}", e),
			}
		}
	}
	Ok(())
}

/// 处理标签相关动作
fn handle_tag_action(
	action: cli::TagActions,
	conn: &mut AnyConnection,
	context: &mut Context,
) -> Result<(), Box<dyn Error>> {
	match action {
		cli::TagActions::Create { name } => {
			let name = name.unwrap_or_else(|| get_input("请输入标签名称: "));

			match service::tags::create_tag_by_name(conn, &name) {
				Ok(tag) => println!("成功创建标签: {:?}", tag),
				Err(e) => eprintln!("创建标签失败: {:?}", e),
			}
		}
		cli::TagActions::Delete { id } => {
			let tag_id = id;

			if confirm_deletion(&format!("确定要删除 ID 为 {} 的标签吗? (y/n): ", tag_id)) {
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
		cli::TagActions::ListInteractive => list_tags_interactive(conn, context),
		cli::TagActions::ListByConditions { conditions, order_by, limit, offset } => {
			let conditions = parse_tag_conditions(&conditions);
			let mut options = models::TagQueryOptions::default();
			options.limit = limit;
			options.offset = offset;
			options.order_by = parse_tag_order_by(&order_by);

			match service::tags::select_tags_by_conditions_with_options(conn, conditions, options) {
				Ok(tags) => {
					println!("查询结果 (共 {} 条记录):", tags.len());
					for tag in tags {
						println!("{:?}", tag);
					}
				}
				Err(e) => eprintln!("查询失败: {:?}", e),
			}
		}
		cli::TagActions::ListByGroupId { group_id } => {
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
		cli::TagActions::UpdateById { id, name, reference_count } => {
			let update_dto = models::UpdateTagDTO { name, reference_count };

			let conditions = vec![models::TagCondition::Id(id)];
			match service::tags::update_tags_by_conditions(conn, conditions, update_dto) {
				Ok(count) => println!("成功更新 {} 条记录", count),
				Err(e) => eprintln!("更新失败: {:?}", e),
			}
		}
		cli::TagActions::UpdateByConditions { conditions, name, reference_count } => {
			let conditions = if conditions.is_empty() {
				if let Some(selected_id) = context.selected_tag_id {
					vec![models::TagCondition::Id(selected_id)]
				} else {
					return Err(
						"没有活动的标签，请先运行 'tag list' 或 'tag list-by-conditions' 选择一个标签".into(),
					);
				}
			} else {
				parse_tag_conditions(&conditions)
			};

			let name = name.unwrap_or_else(|| get_input("请输入新的标签名称 (留空则不修改): "));

			let update_dto = models::UpdateTagDTO {
				name: if name.is_empty() { None } else { Some(name) },
				reference_count,
			};
			match service::tags::update_tags_by_conditions(conn, conditions, update_dto) {
				Ok(count) => println!("成功更新 {} 条记录", count),
				Err(e) => eprintln!("更新失败: {:?}", e),
			}
		}
		cli::TagActions::DeleteByConditions { conditions } => {
			let conditions = parse_tag_conditions(&conditions);
			match service::tags::delete_tags_by_conditions(conn, conditions) {
				Ok(count) => println!("成功删除 {} 条记录", count),
				Err(e) => eprintln!("删除失败: {:?}", e),
			}
		}
	}
	Ok(())
}

/// 处理文件组关联相关动作
fn handle_file_group_action(
	action: cli::FileGroupActions,
	conn: &mut AnyConnection,
	context: &mut Context,
) -> Result<(), Box<dyn Error>> {
	match action {
		cli::FileGroupActions::Create { file_id, group_id } => {
			let file_id =
				file_id.unwrap_or_else(|| get_input("请输入文件 ID: ").parse().expect("无效的文件 ID"));
			let group_id = group_id
				.or(context.selected_group_id)
				.unwrap_or_else(|| get_input("请输入组 ID: ").parse().expect("无效的组 ID"));

			let dto = models::FileGroupDTO { file_id, group_id, relation_type: 1 };
			match service::file_group::create_file_group(conn, dto) {
				Ok(dto) => println!("成功创建文件组关联: {:?}", dto),
				Err(e) => eprintln!("创建文件组关联失败: {:?}", e),
			}
		}
		cli::FileGroupActions::Delete { file_id, group_id } => {
			let file_id = file_id
				.or(context.selected_file_id)
				.unwrap_or_else(|| get_input("请输入文件 ID: ").parse().expect("无效的文件 ID"));
			let group_id = group_id
				.or(context.selected_group_id)
				.unwrap_or_else(|| get_input("请输入组 ID: ").parse().expect("无效的组 ID"));

			if confirm_deletion(&format!(
				"确定要删除文件 ID 为 {} 和组 ID 为 {} 的关联吗? (y/n): ",
				file_id, group_id
			)) {
				let dto = models::FileGroupDTO { file_id, group_id, relation_type: 1 };
				match service::file_group::delete_file_group_by_dto(conn, &dto) {
					Ok(count) => println!("成功删除 {} 个文件组关联", count),
					Err(e) => eprintln!("删除文件组关联失败: {:?}", e),
				}
			} else {
				println!("操作已取消");
			}
		}
		cli::FileGroupActions::ListInteractive => list_file_groups_interactive(conn, context),
		cli::FileGroupActions::ListByConditions { conditions, order_by, limit, offset } => {
			let conditions = parse_file_group_conditions(&conditions);
			let mut options = models::FileGroupQueryOptions::default();
			options.limit = limit;
			options.offset = offset;
			options.order_by = parse_file_group_order_by(&order_by);

			match service::file_group::select_file_groups_by_conditions_with_options(
				conn, conditions, options,
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
		cli::FileGroupActions::DeleteByConditions { conditions } => {
			let conditions = parse_file_group_conditions(&conditions);
			match service::file_group::delete_file_groups_by_conditions(conn, conditions) {
				Ok(count) => println!("成功删除 {} 条记录", count),
				Err(e) => eprintln!("删除失败: {:?}", e),
			}
		}
	}
	Ok(())
}

/// 处理组标签关联相关动作
fn handle_group_tag_action(
	action: cli::GroupTagActions,
	conn: &mut AnyConnection,
	context: &mut Context,
) -> Result<(), Box<dyn Error>> {
	match action {
		cli::GroupTagActions::Create { group_id, tag_id } => {
			let group_id = group_id
				.or(context.selected_group_id)
				.unwrap_or_else(|| get_input("请输入组 ID: ").parse().expect("无效的组 ID"));
			let tag_id =
				tag_id.unwrap_or_else(|| get_input("请输入标签 ID: ").parse().expect("无效的标签 ID"));

			let dto = models::GroupTagDTO { group_id, tag_id };
			match service::group_tag::create_group_tag(conn, dto) {
				Ok(dto) => println!("成功创建组标签关联: {:?}", dto),
				Err(e) => eprintln!("创建组标签关联失败: {:?}", e),
			}
		}
		cli::GroupTagActions::Delete { group_id, tag_id } => {
			let group_id = group_id
				.or(context.selected_group_id)
				.unwrap_or_else(|| get_input("请输入组 ID: ").parse().expect("无效的组 ID"));
			let tag_id = tag_id
				.or(context.selected_tag_id)
				.unwrap_or_else(|| get_input("请输入标签 ID: ").parse().expect("无效的标签 ID"));

			if confirm_deletion(&format!(
				"确定要删除组 ID 为 {} 和标签 ID 为 {} 的关联吗? (y/n): ",
				group_id, tag_id
			)) {
				let dto = models::GroupTagDTO { group_id, tag_id };
				match service::group_tag::delete_group_tag_by_dto(conn, &dto) {
					Ok(count) => println!("成功删除 {} 个组标签关联", count),
					Err(e) => eprintln!("删除组标签关联失败: {:?}", e),
				}
			} else {
				println!("操作已取消");
			}
		}
		cli::GroupTagActions::ListInteractive => list_group_tags_interactive(conn, context),
		cli::GroupTagActions::ListByConditions { conditions, order_by, limit, offset } => {
			let conditions = parse_group_tag_conditions(&conditions);
			let mut options = models::GroupTagQueryOptions::default();
			options.limit = limit;
			options.offset = offset;
			options.order_by = parse_group_tag_order_by(&order_by);

			match service::group_tag::select_group_tags_by_conditions_with_options(
				conn, conditions, options,
			) {
				Ok(group_tags) => {
					println!("查询结果 (共 {} 条记录):", group_tags.len());
					for gt in &group_tags {
						println!("{:?}", gt);
					}

					if let Some(first_gt) = group_tags.first() {
						context.selected_group_id = Some(first_gt.group_id);
						context.selected_tag_id = Some(first_gt.tag_id);
						println!(
							"\n提示：第一个组标签关联的组 ID ({}) 和标签 ID ({}) 已被选中，可用于后续操作。",
							first_gt.group_id, first_gt.tag_id
						);
					}
				}
				Err(e) => eprintln!("查询失败: {:?}", e),
			}
		}
		cli::GroupTagActions::DeleteByConditions { conditions } => {
			let conditions = parse_group_tag_conditions(&conditions);
			match service::group_tag::delete_group_tags_by_conditions(conn, conditions) {
				Ok(count) => println!("成功删除 {} 条记录", count),
				Err(e) => eprintln!("删除失败: {:?}", e),
			}
		}
	}
	Ok(())
}

/// 处理组关系相关动作
fn handle_group_relation_action(
	action: cli::GroupRelationActions,
	conn: &mut AnyConnection,
	context: &mut Context,
) -> Result<(), Box<dyn Error>> {
	match action {
		cli::GroupRelationActions::Create { first_group_id, second_group_id, relation_type } => {
			let first_group_id = first_group_id
				.unwrap_or_else(|| get_input("请输入第一个组 ID: ").parse().expect("无效的组 ID"));
			let second_group_id = second_group_id
				.unwrap_or_else(|| get_input("请输入第二个组 ID: ").parse().expect("无效的组 ID"));
			let relation_type = relation_type.unwrap_or(1); // 默认为父子关系

			let dto = models::GroupRelation { first_group_id, second_group_id, relation_type };

			match service::group_relations::create_group_relation(conn, dto) {
				Ok(dto) => println!("成功创建组关系: {:?}", dto),
				Err(e) => eprintln!("创建组关系失败: {:?}", e),
			}
		}
		cli::GroupRelationActions::Delete { first_group_id, second_group_id, relation_type } => {
			let first_group_id = first_group_id
				.unwrap_or_else(|| get_input("请输入第一个组 ID: ").parse().expect("无效的组 ID"));
			let second_group_id = second_group_id
				.unwrap_or_else(|| get_input("请输入第二个组 ID: ").parse().expect("无效的组 ID"));
			let relation_type = relation_type.unwrap_or(1); // 默认为父子关系

			if confirm_deletion(&format!(
				"确定要删除组 ID 为 {} 和组 ID 为 {} 的关系吗? (y/n): ",
				first_group_id, second_group_id
			)) {
				let dto = models::GroupRelation { first_group_id, second_group_id, relation_type };

				match service::group_relations::delete_group_relation(conn, &dto) {
					Ok(count) => println!("成功删除 {} 个组关系", count),
					Err(e) => eprintln!("删除组关系失败: {:?}", e),
				}
			} else {
				println!("操作已取消");
			}
		}
		cli::GroupRelationActions::ListInteractive => {
			println!("暂不支持交互式列出组关系");
		}
		cli::GroupRelationActions::ListByConditions { conditions, order_by, limit, offset } => {
			let conditions = parse_group_relation_conditions(&conditions);
			let mut options = models::GroupRelationQueryOptions::default();
			options.limit = limit;
			options.offset = offset;
			options.order_by = parse_group_relation_order_by(&order_by);

			match service::group_relations::select_group_relations_by_conditions_with_options(
				conn, conditions, options,
			) {
				Ok(relations) => {
					println!("查询结果 (共 {} 条记录):", relations.len());
					for relation in &relations {
						println!("{:?}", relation);
					}

					if let Some(first_relation) = relations.first() {
						context.selected_group_id = Some(first_relation.first_group_id);
						println!(
							"\n提示：第一个组关系的第一个组 ID ({}) 已被选中，可用于后续操作。",
							first_relation.first_group_id
						);
					}
				}
				Err(e) => eprintln!("查询失败: {:?}", e),
			}
		}
		cli::GroupRelationActions::DeleteByConditions { conditions } => {
			let conditions = parse_group_relation_conditions(&conditions);
			match service::group_relations::delete_group_relations_by_conditions(conn, conditions) {
				Ok(count) => println!("成功删除 {} 条记录", count),
				Err(e) => eprintln!("删除失败: {:?}", e),
			}
		}
	}
	Ok(())
}

/// 打印组树状结构
fn print_group_tree(node: &models::GroupTreeNode, depth: usize) {
	let indent = "  ".repeat(depth);
	println!(
		"{}- {} (ID: {}, 引用数: {}, 主组: {}, 点击数: {}, 分享数: {})",
		indent,
		node.group.name,
		node.group.id,
		node.group.reference_count,
		node.group.is_primary,
		node.group.click_count,
		node.group.share_count
	);

	for child in &node.children {
		print_group_tree(child, depth + 1);
	}
}
