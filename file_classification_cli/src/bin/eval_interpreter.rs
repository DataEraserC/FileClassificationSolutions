use regex::Regex;
use std::collections::HashMap;
use rusqlite::Connection;
use file_classification_core::model::models::{
    CreateFileDTO, CreateGroupDTO, CreateTagDTO,
    FileCondition, GroupCondition, TagCondition,
    FileOrderBy, GroupOrderBy, TagOrderBy,
    FileQueryOptions, GroupQueryOptions, TagQueryOptions,
    OrderDirection, FileGroupDTO, GroupTagDTO
};
use file_classification_core::service::{files, groups, tags, file_group, group_tag};
use file_classification_core::utils::errors::AppError;
use file_classification_core::utils::database::AnyConnection;
pub struct Context {
    selected_file_id: Option<i32>,
    selected_group_id: Option<i32>,
    selected_tag_id: Option<i32>,
}

impl Context {
    fn new() -> Self {
        Self { selected_file_id: None, selected_group_id: None, selected_tag_id: None }
    }
}

pub fn eval_expression(expr: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<String, String> {
    let re = Regex::new(r"^(\w+)\s+(\w+)(?:\s+(.+))?$").unwrap();
    if let Some(caps) = re.captures(expr) {
        let entity_type = caps.get(1).map_or("", |m| m.as_str());
        let action = caps.get(2).map_or("", |m| m.as_str());
        let params = caps.get(3).map_or("", |m| m.as_str());

        match entity_type {
            "file" => eval_file_operation(action, params, conn, context),
            "group" => eval_group_operation(action, params, conn, context),
            "tag" => eval_tag_operation(action, params, conn, context),
            "link" => eval_link_operation(action, params, conn, context),
            _ => Err(format!("未知的实体类型: {}", entity_type)),
        }
    } else {
        Err("无效的表达式格式".to_string())
    }
}

// 解析文件条件的辅助函数
fn parse_file_conditions(conditions: &[String]) -> Vec<FileCondition> {
    let mut result = Vec::new();
    for condition in conditions {
        if condition.contains('=') {
            let parts: Vec<&str> = condition.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                match key {
                    "id" => if let Ok(id) = value.parse::<i32>() {
                        result.push(FileCondition::Id(id));
                    },
                    "path" => result.push(FileCondition::Path(value.to_string())),
                    "type" => result.push(FileCondition::Type(value.to_string())),
                    _ => {}
                }
            }
        }
    }
    result
}

// 解析组条件的辅助函数
fn parse_group_conditions(conditions: &[String]) -> Vec<GroupCondition> {
    let mut result = Vec::new();
    for condition in conditions {
        if condition.contains('=') {
            let parts: Vec<&str> = condition.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                match key {
                    "id" => if let Ok(id) = value.parse::<i32>() {
                        result.push(GroupCondition::Id(id));
                    },
                    "name" => result.push(GroupCondition::Name(value.to_string())),
                    _ => {}
                }
            }
        }
    }
    result
}

// 解析标签条件的辅助函数
fn parse_tag_conditions(conditions: &[String]) -> Vec<TagCondition> {
    let mut result = Vec::new();
    for condition in conditions {
        if condition.contains('=') {
            let parts: Vec<&str> = condition.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                match key {
                    "id" => if let Ok(id) = value.parse::<i32>() {
                        result.push(TagCondition::Id(id));
                    },
                    "name" => result.push(TagCondition::Name(value.to_string())),
                    _ => {}
                }
            }
        }
    }
    result
}

fn eval_file_operation(action: &str, params: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<String, String> {
    let params: Vec<&str> = params.split_whitespace().collect();
    
    match action {
        "list" => {
            let mut conditions = Vec::new();
            let mut order_by = None;
            let mut limit = None;
            let mut offset = None;
            
            // 解析参数
            let mut i = 0;
            while i < params.len() {
                match params[i] {
                    "--where" if i + 1 < params.len() => {
                        conditions.push(params[i + 1].to_string());
                        i += 2;
                    }
                    "--order-by" if i + 1 < params.len() => {
                        order_by = Some(params[i + 1].to_string());
                        i += 2;
                    }
                    "--limit" if i + 1 < params.len() => {
                        if let Ok(l) = params[i + 1].parse::<i64>() {
                            limit = Some(l);
                        }
                        i += 2;
                    }
                    "--offset" if i + 1 < params.len() => {
                        if let Ok(o) = params[i + 1].parse::<i64>() {
                            offset = Some(o);
                        }
                        i += 2;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            
            let order_direction = OrderDirection::Asc;
            let mut options = FileQueryOptions {
                limit,
                offset,
                order_by: Vec::new(),
            };
            
            if let Some(order) = order_by {
                match order.as_str() {
                    "id" => options.order_by.push(FileOrderBy::Id(order_direction)),
                    "path" => options.order_by.push(FileOrderBy::Path(order_direction)),
                    "type" => options.order_by.push(FileOrderBy::Type(order_direction)),
                    "reference_count" => options.order_by.push(FileOrderBy::ReferenceCount(order_direction)),
                    "group_id" => options.order_by.push(FileOrderBy::GroupId(order_direction)),
                    _ => options.order_by.push(FileOrderBy::Id(order_direction)),
                }
            } else {
                options.order_by.push(FileOrderBy::Id(order_direction));
            }
            
            let conditions_vec: Vec<String> = conditions.iter().map(|s| s.to_string()).collect();
            match files::select_files_by_conditions_with_options(conn, parse_file_conditions(&conditions_vec), options) {
                Ok(files) => {
                    let mut result = String::new();
                    for file in files {
                        result.push_str(&format!("ID: {}, Path: {}, Type: {}\n", file.id, file.path, file.type_));
                    }
                    Ok(result)
                }
                Err(e) => Err(format!("列出文件时出错: {:?}", e)),
            }
        }
        "get" => {
            if params.is_empty() {
                return Err("缺少文件ID".to_string());
            }
            
            let id = match params[0].parse::<i32>() {
                Ok(id) => id,
                Err(_) => return Err("无效的文件ID".to_string()),
            };
            
            match files::get_file_by_id(conn, id) {
                Ok(file) => Ok(format!("ID: {}, Path: {}, Type: {}", file.id, file.path, file.type_)),
                Err(e) => Err(format!("获取文件时出错: {:?}", e)),
            }
        }
        "create" => {
            if params.len() < 2 {
                return Err("缺少必要参数。用法: file create <path> <type> [group_id]".to_string());
            }
            
            let path = params[0];
            let type_ = params[1];
            let group_id = if params.len() > 2 {
                match params[2].parse::<i32>() {
                    Ok(id) => Some(id),
                    Err(_) => return Err("无效的组ID".to_string()),
                }
            } else {
                None
            };
            
            let file_dto = CreateFileDTO {
                type_,
                path,
                group_id: group_id.unwrap_or(0),
            };
            
            match files::create_file(conn, file_dto) {
                Ok(id) => Ok(format!("文件创建成功，ID: {}", id)),
                Err(e) => Err(format!("创建文件时出错: {:?}", e)),
            }
        }
        _ => Err(format!("未知的文件操作: {}", action)),
    }
}

fn eval_group_operation(action: &str, params: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<String, String> {
    let params: Vec<&str> = params.split_whitespace().collect();
    
    match action {
        "list" => {
            let mut conditions = Vec::new();
            let mut order_by = None;
            let mut limit = None;
            let mut offset = None;
            
            // 解析参数
            let mut i = 0;
            while i < params.len() {
                match params[i] {
                    "--where" if i + 1 < params.len() => {
                        conditions.push(params[i + 1].to_string());
                        i += 2;
                    }
                    "--order-by" if i + 1 < params.len() => {
                        order_by = Some(params[i + 1].to_string());
                        i += 2;
                    }
                    "--limit" if i + 1 < params.len() => {
                        if let Ok(l) = params[i + 1].parse::<i64>() {
                            limit = Some(l);
                        }
                        i += 2;
                    }
                    "--offset" if i + 1 < params.len() => {
                        if let Ok(o) = params[i + 1].parse::<i64>() {
                            offset = Some(o);
                        }
                        i += 2;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            
            let order_direction = OrderDirection::Asc;
            let mut options = GroupQueryOptions {
                limit,
                offset,
                order_by: Vec::new(),
            };
            
            if let Some(order) = order_by {
                match order.as_str() {
                    "id" => options.order_by.push(GroupOrderBy::Id(order_direction)),
                    "name" => options.order_by.push(GroupOrderBy::Name(order_direction)),
                    "reference_count" => options.order_by.push(GroupOrderBy::ReferenceCount(order_direction)),
                    _ => options.order_by.push(GroupOrderBy::Id(order_direction)),
                }
            } else {
                options.order_by.push(GroupOrderBy::Id(order_direction));
            }
            
            let conditions_vec: Vec<String> = conditions.iter().map(|s| s.to_string()).collect();
            match groups::select_groups_by_conditions_with_options(conn, parse_group_conditions(&conditions_vec), options) {
                Ok(groups) => {
                    let mut result = String::new();
                    for group in groups {
                        result.push_str(&format!("ID: {}, Name: {}\n", group.id, group.name));
                    }
                    Ok(result)
                }
                Err(e) => Err(format!("列出组时出错: {:?}", e)),
            }
        }
        "get" => {
            if params.is_empty() {
                return Err("缺少组ID".to_string());
            }
            
            let id = match params[0].parse::<i32>() {
                Ok(id) => id,
                Err(_) => return Err("无效的组ID".to_string()),
            };
            
            match groups::get_group_by_id(conn, id) {
                Ok(group) => Ok(format!("ID: {}, Name: {}", group.id, group.name)),
                Err(e) => Err(format!("获取组时出错: {:?}", e)),
            }
        }
        "create" => {
            if params.is_empty() {
                return Err("缺少组名".to_string());
            }
            
            let name = params[0];
            let group_dto = CreateGroupDTO { name };
            
            match groups::create_group(conn, &group_dto) {
                Ok(id) => Ok(format!("组创建成功，ID: {}", id)),
                Err(e) => Err(format!("创建组时出错: {:?}", e)),
            }
        }
        _ => Err(format!("未知的组操作: {}", action)),
    }
}

fn eval_tag_operation(action: &str, params: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<String, String> {
    let params: Vec<&str> = params.split_whitespace().collect();
    
    match action {
        "list" => {
            let mut conditions = Vec::new();
            let mut order_by = None;
            let mut limit = None;
            let mut offset = None;
            
            // 解析参数
            let mut i = 0;
            while i < params.len() {
                match params[i] {
                    "--where" if i + 1 < params.len() => {
                        conditions.push(params[i + 1].to_string());
                        i += 2;
                    }
                    "--order-by" if i + 1 < params.len() => {
                        order_by = Some(params[i + 1].to_string());
                        i += 2;
                    }
                    "--limit" if i + 1 < params.len() => {
                        if let Ok(l) = params[i + 1].parse::<i64>() {
                            limit = Some(l);
                        }
                        i += 2;
                    }
                    "--offset" if i + 1 < params.len() => {
                        if let Ok(o) = params[i + 1].parse::<i64>() {
                            offset = Some(o);
                        }
                        i += 2;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            
            let order_direction = OrderDirection::Asc;
            let mut options = TagQueryOptions {
                limit,
                offset,
                order_by: Vec::new(),
            };
            
            if let Some(order) = order_by {
                match order.as_str() {
                    "id" => options.order_by.push(TagOrderBy::Id(order_direction)),
                    "name" => options.order_by.push(TagOrderBy::Name(order_direction)),
                    "reference_count" => options.order_by.push(TagOrderBy::ReferenceCount(order_direction)),
                    _ => options.order_by.push(TagOrderBy::Id(order_direction)),
                }
            } else {
                options.order_by.push(TagOrderBy::Id(order_direction));
            }
            
            let conditions_vec: Vec<String> = conditions.iter().map(|s| s.to_string()).collect();
            match tags::select_tags_by_conditions_with_options(conn, parse_tag_conditions(&conditions_vec), options) {
                Ok(tags) => {
                    let mut result = String::new();
                    for tag in tags {
                        result.push_str(&format!("ID: {}, Name: {}\n", tag.id, tag.name));
                    }
                    Ok(result)
                }
                Err(e) => Err(format!("列出标签时出错: {:?}", e)),
            }
        }
        "get" => {
            if params.is_empty() {
                return Err("缺少标签ID".to_string());
            }
            
            let id = match params[0].parse::<i32>() {
                Ok(id) => id,
                Err(_) => return Err("无效的标签ID".to_string()),
            };
            
            match tags::get_tag_by_id(conn, id) {
                Ok(tag) => Ok(format!("ID: {}, Name: {}", tag.id, tag.name)),
                Err(e) => Err(format!("获取标签时出错: {:?}", e)),
            }
        }
        "create" => {
            if params.is_empty() {
                return Err("缺少标签名".to_string());
            }
            
            let name = params[0];
            let tag_dto = CreateTagDTO { name };
            
            match tags::create_tag(conn, &tag_dto) {
                Ok(id) => Ok(format!("标签创建成功，ID: {}", id.id)),
                Err(e) => Err(format!("创建标签时出错: {:?}", e)),
            }
        }
        _ => Err(format!("未知的标签操作: {}", action)),
    }
}

fn eval_link_operation(action: &str, params: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<String, String> {
    let params: Vec<&str> = params.split_whitespace().collect();
    
    match action {
        "file-group" => {
            if params.len() < 2 {
                return Err("缺少必要参数。用法: link file-group <file_id> <group_id>".to_string());
            }
            
            let file_id = match params[0].parse::<i32>() {
                Ok(id) => id,
                Err(_) => return Err("无效的文件ID".to_string()),
            };
            
            let group_id = match params[1].parse::<i32>() {
                Ok(id) => id,
                Err(_) => return Err("无效的组ID".to_string()),
            };
            
            match file_group::create_file_group(conn, FileGroupDTO { file_id, group_id }) {
                Ok(_) => Ok("文件-组关联创建成功".to_string()),
                Err(e) => Err(format!("创建文件-组关联时出错: {:?}", e)),
            }
        }
        "group-tag" => {
            if params.len() < 2 {
                return Err("缺少必要参数。用法: link group-tag <group_id> <tag_id>".to_string());
            }
            
            let group_id = match params[0].parse::<i32>() {
                Ok(id) => id,
                Err(_) => return Err("无效的组ID".to_string()),
            };
            
            let tag_id = match params[1].parse::<i32>() {
                Ok(id) => id,
                Err(_) => return Err("无效的标签ID".to_string()),
            };
            
            match group_tag::create_group_tag(conn, GroupTagDTO { group_id, tag_id }) {
                Ok(_) => Ok("组-标签关联创建成功".to_string()),
                Err(e) => Err(format!("创建组-标签关联时出错: {:?}", e)),
            }
        }
        _ => Err(format!("未知的链接操作: {}", action)),
    }
}