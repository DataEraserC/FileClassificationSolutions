use file_classification_core::model::models::{Group, Tag};
use file_classification_core::utils::database::AnyConnection;
use file_classification_core::service::group_tags::{create_group_tag, delete_group_tag, get_tags_by_group_id, get_groups_by_tag_id};
use std::error::Error;

pub struct GroupTagService<'a> {
    conn: &'a mut AnyConnection,
}

impl<'a> GroupTagService<'a> {
    pub fn new(conn: &'a mut AnyConnection) -> Self {
        Self { conn }
    }

    /// 将标签链接到组
    pub fn link_tag_to_group(&mut self, group_id: i32, tag_id: i32) -> Result<(), Box<dyn Error>> {
        create_group_tag(self.conn, group_id, tag_id)?;
        Ok(())
    }

    /// 从组中解除标签链接
    pub fn unlink_tag_from_group(&mut self, group_id: i32, tag_id: i32) -> Result<(), Box<dyn Error>> {
        delete_group_tag(self.conn, group_id, tag_id)?;
        Ok(())
    }

    /// 列出组中的所有标签
    pub fn list_tags_for_group(&mut self, group_id: i32) -> Result<Vec<Tag>, Box<dyn Error>> {
        Ok(get_tags_by_group_id(self.conn, group_id)?)
    }

    /// 列出包含特定标签的所有组
    pub fn list_groups_with_tag(&mut self, tag_id: i32) -> Result<Vec<Group>, Box<dyn Error>> {
        Ok(get_groups_by_tag_id(self.conn, tag_id)?)
    }
}
