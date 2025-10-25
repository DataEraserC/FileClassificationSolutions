// interactive.rs
// 所有交互式列表函数，使用dialoguer库

use dialoguer::{theme::ColorfulTheme, Select};
use file_classification_core::service;
use file_classification_core::utils::database::AnyConnection;

use crate::context::Context;

/// 交互式列出文件并选择
pub fn list_files_interactive(conn: &mut AnyConnection, context: &mut Context) {
	let files = service::files::select_files_by_conditions(conn, vec![], None);

	match files {
		Ok(files) => {
			if files.is_empty() {
				println!("没有找到任何文件。");
				return;
			}

			let items: Vec<String> = files
				.iter()
				.map(|f| format!("[{}] 类型: {}, 路径: {}, 组ID: {}", f.id, f.type_, f.path, f.group_id))
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

/// 交互式列出组并选择
pub fn list_groups_interactive(conn: &mut AnyConnection, context: &mut Context) {
	let groups =
		service::groups::select_groups_by_conditions_with_options(conn, vec![], Default::default());

	match groups {
		Ok(groups) => {
			if groups.is_empty() {
				println!("没有找到任何组。");
				return;
			}

			let items: Vec<String> = groups
				.iter()
				.map(|g| {
					format!(
						"[{}] 名称: {}, 主组: {}, 点击: {}, 分享: {}",
						g.id, g.name, g.is_primary, g.click_count, g.share_count
					)
				})
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

/// 交互式列出标签并选择
pub fn list_tags_interactive(conn: &mut AnyConnection, context: &mut Context) {
	let tags =
		service::tags::select_tags_by_conditions_with_options(conn, vec![], Default::default());

	match tags {
		Ok(tags) => {
			if tags.is_empty() {
				println!("没有找到任何标签。");
				return;
			}

			let items: Vec<String> = tags
				.iter()
				.map(|t| format!("[{}] 名称: {}, 引用计数: {}", t.id, t.name, t.reference_count))
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

/// 交互式列出文件组关联并选择
pub fn list_file_groups_interactive(conn: &mut AnyConnection, context: &mut Context) {
	let file_groups = service::file_group::select_file_groups_by_conditions_with_options(
		conn,
		vec![],
		Default::default(),
	);

	match file_groups {
		Ok(file_groups) => {
			if file_groups.is_empty() {
				println!("没有找到任何文件组关联。");
				return;
			}

			let items: Vec<String> = file_groups
				.iter()
				.map(|fg| format!("[文件ID: {}, 组ID: {}]", fg.file_id, fg.group_id))
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
					"已选择文件 ID: {} 和组 ID: {}",
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

/// 交互式列出组标签关联并选择
pub fn list_group_tags_interactive(conn: &mut AnyConnection, context: &mut Context) {
	let group_tags = service::group_tag::select_group_tags_by_conditions_with_options(
		conn,
		vec![],
		Default::default(),
	);

	match group_tags {
		Ok(group_tags) => {
			if group_tags.is_empty() {
				println!("没有找到任何组标签关联。");
				return;
			}

			let items: Vec<String> = group_tags
				.iter()
				.map(|gt| format!("[组ID: {}, 标签ID: {}]", gt.group_id, gt.tag_id))
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
					"已选择组 ID: {} 和标签 ID: {}",
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
