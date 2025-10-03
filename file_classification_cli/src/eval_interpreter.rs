use regex::Regex;
use crate::context::Context;
use crate::file_group::FileGroupService;
use crate::group_tag::GroupTagService;
use file_classification_core::model::models::*;
use file_classification_core::service::*;
use file_classification_core::utils::database::AnyConnection;
use std::error::Error;

// 定义服务结构体
pub struct FileService<'a> {
    conn: &'a mut AnyConnection,
}

pub struct GroupService<'a> {
    conn: &'a mut AnyConnection,
}

pub struct TagService<'a> {
    conn: &'a mut AnyConnection,
}

// FileGroupService和GroupTagService已移至独立模块

// 实现服务方法
impl<'a> FileService<'a> {
    pub fn new(conn: &'a mut AnyConnection) -> Self {
        Self { conn }
    }
    
    pub fn list_files(&mut self, conditions: Option<&str>, limit: Option<i64>, offset: Option<i64>, _order_by: Option<&str>) -> Result<Vec<File>, Box<dyn Error>> {
        let conditions = match conditions {
            Some(c) => parse_file_conditions(&vec![c.to_string()]),
            None => vec![],
        };
        let mut options = FileQueryOptions::default();
        options.limit = limit;
        options.offset = offset;
        
        Ok(files::select_files_by_conditions_with_options(self.conn, conditions, options)?)
    }
    
    pub fn get_file(&mut self, id: i32) -> Result<File, Box<dyn Error>> {
        Ok(files::get_file_by_id(self.conn, id)?)
    }
    
    pub fn create_file(&mut self, path: &str, type_: Option<&str>, group_id: Option<i32>) -> Result<File, Box<dyn Error>> {
        let dto = CreateFileDTO {
            type_: type_.unwrap_or("regular"),
            path,
            group_id: group_id.unwrap_or(0),
        };
        files::create_file(self.conn, dto)?;
        
        // 返回创建的文件
        let conditions = vec![FileCondition::Path(path.to_string())];
        let files = files::select_files_by_conditions(self.conn, conditions, None)?;
        if let Some(file) = files.first() {
            Ok(file.clone())
        } else {
            Err("文件创建后无法检索".into())
        }
    }
    
    pub fn update_files(&mut self, conditions: &str, path: Option<&str>, type_: Option<&str>, reference_count: Option<i32>, group_id: Option<i32>) -> Result<usize, Box<dyn Error>> {
        let conditions = parse_file_conditions(&vec![conditions.to_string()]);
        let mut changes = UpdateFileDTO::default();
        if let Some(path) = path { changes.path = Some(path.to_string()); }
        if let Some(type_) = type_ { changes.type_ = Some(type_.to_string()); }
        if let Some(reference_count) = reference_count { changes.reference_count = Some(reference_count); }
        if let Some(group_id) = group_id { changes.group_id = Some(group_id); }
        
        Ok(files::update_files_by_conditions(self.conn, conditions, changes)?)
    }
    
    pub fn delete_file(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        files::delete_file(self.conn, id)?;
        Ok(())
    }
}

impl<'a> GroupService<'a> {
    pub fn new(conn: &'a mut AnyConnection) -> Self {
        Self { conn }
    }
    
    pub fn list_groups(&mut self, conditions: Option<&str>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Group>, Box<dyn Error>> {
        let conditions = match conditions {
            Some(c) => parse_group_conditions(&vec![c.to_string()]),
            None => vec![],
        };
        let mut options = GroupQueryOptions::default();
        options.limit = limit;
        options.offset = offset;
        
        Ok(groups::select_groups_by_conditions_with_options(self.conn, conditions, options)?)
    }
    
    pub fn get_group(&mut self, id: i32) -> Result<Group, Box<dyn Error>> {
        Ok(groups::get_group_by_id(self.conn, id)?)
    }
    
    pub fn create_group(&mut self, name: &str) -> Result<Group, Box<dyn Error>> {
        let dto = CreateGroupDTO {
            name: name,
        };
        groups::create_group(self.conn, &dto)?;
        
        // 返回创建的组
        let conditions = vec![GroupCondition::Name(name.to_string())];
        let groups = groups::select_groups_by_conditions(self.conn, conditions, None)?;
        if let Some(group) = groups.first() {
            Ok(group.clone())
        } else {
            Err("组创建后无法检索".into())
        }
    }
    
    pub fn update_group(&mut self, id: i32, name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let mut changes = UpdateGroupDTO::default();
        if let Some(name) = name { changes.name = Some(name.to_string()); }
        
        groups::update_group_by_id(self.conn, id, changes)?;
        Ok(())
    }
    
    pub fn delete_group(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        groups::delete_group(self.conn, id)?;
        Ok(())
    }
}

impl<'a> TagService<'a> {
    pub fn new(conn: &'a mut AnyConnection) -> Self {
        Self { conn }
    }
    
    pub fn list_tags(&mut self, conditions: Option<&str>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Tag>, Box<dyn Error>> {
        let conditions = match conditions {
            Some(c) => parse_tag_conditions(&vec![c.to_string()]),
            None => vec![],
        };
        let mut options = TagQueryOptions::default();
        options.limit = limit;
        options.offset = offset;
        
        Ok(tags::select_tags_by_conditions_with_options(self.conn, conditions, options)?)
    }
    
    pub fn get_tag(&mut self, id: i32) -> Result<Tag, Box<dyn Error>> {
        Ok(tags::get_tag_by_id(self.conn, id)?)
    }
    
    pub fn create_tag(&mut self, name: &str, _description: Option<&str>) -> Result<Tag, Box<dyn Error>> {
        let dto = CreateTagDTO {
            name: name,
        };
        tags::create_tag(self.conn, &dto)?;
        
        // 返回创建的标签
        let conditions = vec![TagCondition::Name(name.to_string())];
        let tags = tags::select_tags_by_conditions(self.conn, conditions, None)?;
        if let Some(tag) = tags.first() {
            Ok(tag.clone())
        } else {
            Err("标签创建后无法检索".into())
        }
    }
    
    pub fn update_tag(&mut self, id: i32, name: Option<&str>, _description: Option<&str>) -> Result<(), Box<dyn Error>> {
        let mut changes = UpdateTagDTO::default();
        if let Some(name) = name { changes.name = Some(name.to_string()); }
        
        tags::update_tag_by_id(self.conn, id, changes)?;
        Ok(())
    }
    
    pub fn delete_tag(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        tags::delete_tag(self.conn, id)?;
        Ok(())
    }
}

pub fn eval_expression(expr: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    // 解析表达式
    let re = Regex::new(r"^(\w+)\.(\w+)\((.*)\)$")?;
    if let Some(caps) = re.captures(expr) {
        let entity_type = caps.get(1).map_or("", |m| m.as_str());
        let action = caps.get(2).map_or("", |m| m.as_str());
        let params = caps.get(3).map_or("", |m| m.as_str());

        match entity_type {
            "file" => handle_file_expression(action, params, conn, context),
            "group" => handle_group_expression(action, params, conn, context),
            "tag" => handle_tag_expression(action, params, conn, context),
            _ => Err(format!("未知的实体类型: {}", entity_type).into()),
        }
    } else {
        Err("无效的表达式格式".into())
    }
}

fn handle_file_expression(action: &str, params: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let mut file_service = FileService::new(conn);
    
    match action {
        "list" => {
            let files = file_service.list_files(None, None, None, None)?;
            if files.is_empty() {
                println!("没有找到文件");
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
        }
        "get" => {
            let id = params.parse::<i32>()?;
            match file_service.get_file(id) {
                Ok(file) => {
                    println!("ID: {}", file.id);
                    println!("路径: {}", file.path);
                    println!("类型: {}", file.type_);
                    println!("引用计数: {}", file.reference_count);
                    // 保存到上下文
                    context.set("selected_file_id", &id.to_string());
                }
                Err(e) => return Err(format!("获取文件失败: {}", e).into()),
            }
        }
        "create" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 1 {
                return Err("创建文件需要提供路径".into());
            }
            
            let path = params[0].trim_matches('"');
            let type_ = if params.len() > 1 { Some(params[1].trim_matches('"')) } else { None };
            let group_id = if params.len() > 2 { params[2].parse::<i32>().ok() } else { None };
            
            match file_service.create_file(path, type_, group_id) {
                Ok(file) => {
                    println!("文件创建成功: ID={}, 路径={}", file.id, file.path);
                    // 保存到上下文
                    context.set("selected_file_id", &file.id.to_string());
                }
                Err(e) => return Err(format!("创建文件失败: {}", e).into()),
            }
        }
        "update" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("更新文件需要提供ID和至少一个要更新的字段".into());
            }
            
            let id = params[0].parse::<i32>()?;
            let path = if params.len() > 1 && !params[1].is_empty() { Some(params[1].trim_matches('"')) } else { None };
            let type_ = if params.len() > 2 && !params[2].is_empty() { Some(params[2].trim_matches('"')) } else { None };
            
            let conditions = format!("id={}", id);
            let count = file_service.update_files(&conditions, path, type_, None, None)?;
            println!("更新了 {} 个文件", count);
        }
        "delete" => {
            let id = params.parse::<i32>()?;
            match file_service.delete_file(id) {
                Ok(_) => println!("文件删除成功: ID={}", id),
                Err(e) => return Err(format!("删除文件失败: {}", e).into()),
            }
        }
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
        }
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
        }
        _ => return Err(format!("未知的文件操作: {}", action).into()),
    }
    
    Ok(())
}

fn handle_group_expression(action: &str, params: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let mut group_service = GroupService::new(conn);
    
    match action {
        "list" => {
            let groups = group_service.list_groups(None, None, None)?;
            if groups.is_empty() {
                println!("没有找到组");
            } else {
                println!("ID\t名称\t描述");
                for group in groups {
                    println!("{}\t{}", 
                        group.id, 
                        group.name);
                }
            }
        }
        "get" => {
            let id = params.parse::<i32>()?;
            match group_service.get_group(id) {
                Ok(group) => {
                    println!("ID: {}", group.id);
                    println!("名称: {}", group.name);
                    println!("引用计数: {}", group.reference_count);
                    // 保存到上下文
                    context.set("selected_group_id", &id.to_string());
                }
                Err(e) => return Err(format!("获取组失败: {}", e).into()),
            }
        }
        "create" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.is_empty() {
                return Err("创建组需要提供名称".into());
            }
            
            let name = params[0].trim_matches('"');
            
            match group_service.create_group(name) {
                Ok(group) => {
                    println!("组创建成功: ID={}, 名称={}", group.id, group.name);
                    // 保存到上下文
                    context.set("selected_group_id", &group.id.to_string());
                }
                Err(e) => return Err(format!("创建组失败: {}", e).into()),
            }
        }
        "update" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("缺少参数: 组ID和新名称".into());
            }
            let id = params[0].parse::<i32>().map_err(|_| "组ID必须是数字")?;
            let name = if !params[1].is_empty() { Some(params[1].trim_matches('"')) } else { None };
            
            group_service.update_group(id, name)?;
            println!("组更新成功: ID={}", id);
        }
        "delete" => {
            let id = params.parse::<i32>()?;
            match group_service.delete_group(id) {
                Ok(_) => println!("组删除成功: ID={}", id),
                Err(e) => return Err(format!("删除组失败: {}", e).into()),
            }
        }
        "files" => {
            let id = params.parse::<i32>()?;
            let mut file_group_service = FileGroupService::new(conn);
            let files = file_group_service.list_files_in_group(id)?;
            
            if files.is_empty() {
                println!("组 {} 中没有文件", id);
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
        }
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
        }
        "link_tag" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("链接标签到组需要提供组ID和标签ID".into());
            }
            
            let group_id = params[0].parse::<i32>()?;
            let tag_id = params[1].parse::<i32>()?;
            
            let mut group_tag_service = GroupTagService::new(conn);
            group_tag_service.link_tag_to_group(group_id, tag_id)?;
            println!("标签 {} 已链接到组 {}", tag_id, group_id);
        }
        "unlink_tag" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("从组解除链接标签需要提供组ID和标签ID".into());
            }
            
            let group_id = params[0].parse::<i32>()?;
            let tag_id = params[1].parse::<i32>()?;
            
            let mut group_tag_service = GroupTagService::new(conn);
            group_tag_service.unlink_tag_from_group(group_id, tag_id)?;
            println!("标签 {} 已从组 {} 解除链接", tag_id, group_id);
        }
        _ => return Err(format!("未知的组操作: {}", action).into()),
    }
    
    Ok(())
}

fn handle_tag_expression(action: &str, params: &str, conn: &mut AnyConnection, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let mut tag_service = TagService::new(conn);
    
    match action {
        "list" => {
            let tags = tag_service.list_tags(None, None, None)?;
            if tags.is_empty() {
                println!("没有找到标签");
            } else {
                println!("ID\t名称\t描述");
                for tag in tags {
                    println!("{}\t{}", 
                        tag.id, 
                        tag.name);
                }
            }
        }
        "get" => {
            let id = params.parse::<i32>()?;
            match tag_service.get_tag(id) {
                Ok(tag) => {
                    println!("ID: {}", tag.id);
                     println!("名称: {}", tag.name);
                     println!("引用计数: {}", tag.reference_count);
                    // 保存到上下文
                    context.set("selected_tag_id", &id.to_string());
                }
                Err(e) => return Err(format!("获取标签失败: {}", e).into()),
            }
        }
        "create" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.is_empty() {
                return Err("创建标签需要提供名称".into());
            }
            
            let name = params[0].trim_matches('"');
            let description = if params.len() > 1 { Some(params[1].trim_matches('"')) } else { None };
            
            match tag_service.create_tag(name, description) {
                Ok(tag) => {
                    println!("标签创建成功: ID={}, 名称={}", tag.id, tag.name);
                    // 保存到上下文
                    context.set("selected_tag_id", &tag.id.to_string());
                }
                Err(e) => return Err(format!("创建标签失败: {}", e).into()),
            }
        }
        "update" => {
            let params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
            if params.len() < 2 {
                return Err("更新标签需要提供ID和至少一个要更新的字段".into());
            }
            
            let id = params[0].parse::<i32>()?;
            let name = if params.len() > 1 && !params[1].is_empty() { Some(params[1].trim_matches('"')) } else { None };
            let description = if params.len() > 2 && !params[2].is_empty() { Some(params[2].trim_matches('"')) } else { None };
            
            tag_service.update_tag(id, name, description)?;
            println!("标签更新成功: ID={}", id);
        }
        "delete" => {
            let id = params.parse::<i32>()?;
            match tag_service.delete_tag(id) {
                Ok(_) => println!("标签删除成功: ID={}", id),
                Err(e) => return Err(format!("删除标签失败: {}", e).into()),
            }
        }
        "groups" => {
            let id = params.parse::<i32>()?;
            let mut group_tag_service = GroupTagService::new(conn);
            let groups = group_tag_service.list_groups_with_tag(id)?;
            
            if groups.is_empty() {
                println!("标签 {} 没有关联的组", id);
            } else {
                println!("ID\t名称\t描述");
                for group in groups {
                    println!("{}\t{}", 
                        group.id, 
                        group.name);
                }
            }
        }
        _ => return Err(format!("未知的标签操作: {}", action).into()),
    }
    
    Ok(())
}

// 解析条件函数
pub fn parse_file_conditions(conditions: &Vec<String>) -> Vec<FileCondition> {
    let mut result = Vec::new();
    
    for condition in conditions {
        if let Some(id) = condition.strip_prefix("id=") {
            if let Ok(id) = id.parse::<i32>() {
                result.push(FileCondition::Id(id));
            }
        } else if let Some(path) = condition.strip_prefix("path=") {
            result.push(FileCondition::Path(path.to_string()));
        } else if let Some(type_) = condition.strip_prefix("type=") {
            result.push(FileCondition::Type(type_.to_string()));
        } else if let Some(group_id) = condition.strip_prefix("group_id=") {
            if let Ok(group_id) = group_id.parse::<i32>() {
                result.push(FileCondition::GroupId(group_id));
            }
        }
    }
    
    result
}

pub fn parse_group_conditions(conditions: &Vec<String>) -> Vec<GroupCondition> {
    let mut result = Vec::new();
    
    for condition in conditions {
        if let Some(id) = condition.strip_prefix("id=") {
            if let Ok(id) = id.parse::<i32>() {
                result.push(GroupCondition::Id(id));
            }
        } else if let Some(name) = condition.strip_prefix("name=") {
            result.push(GroupCondition::Name(name.to_string()));
        }
    }
    
    result
}

pub fn parse_tag_conditions(conditions: &Vec<String>) -> Vec<TagCondition> {
    let mut result = Vec::new();
    
    for condition in conditions {
        if let Some(id) = condition.strip_prefix("id=") {
            if let Ok(id) = id.parse::<i32>() {
                result.push(TagCondition::Id(id));
            }
        } else if let Some(name) = condition.strip_prefix("name=") {
            result.push(TagCondition::Name(name.to_string()));
        }
    }
    
    result
}