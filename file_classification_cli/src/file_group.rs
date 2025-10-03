use file_classification_core::model::models::{File, Group};
use file_classification_core::utils::database::AnyConnection;
use file_classification_core::service::file_groups::{create_file_group, delete_file_group, get_files_by_group_id, get_groups_by_file_id};
use std::error::Error;

pub struct FileGroupService<'a> {
    conn: &'a mut AnyConnection,
}

impl<'a> FileGroupService<'a> {
    pub fn new(conn: &'a mut AnyConnection) -> Self {
        Self { conn }
    }

    /// 将文件链接到组
    pub fn link_file_to_group(&mut self, file_id: i32, group_id: i32) -> Result<(), Box<dyn Error>> {
        create_file_group(self.conn, file_id, group_id)?;
        Ok(())
    }

    /// 从组中解除文件链接
    pub fn unlink_file_from_group(&mut self, file_id: i32, group_id: i32) -> Result<(), Box<dyn Error>> {
        delete_file_group(self.conn, file_id, group_id)?;
        Ok(())
    }

    /// 列出组中的所有文件
    pub fn list_files_in_group(&mut self, group_id: i32) -> Result<Vec<File>, Box<dyn Error>> {
        Ok(get_files_by_group_id(self.conn, group_id)?)
    }

    /// 列出文件所属的所有组
    pub fn list_groups_for_file(&mut self, file_id: i32) -> Result<Vec<Group>, Box<dyn Error>> {
        Ok(get_groups_by_file_id(self.conn, file_id)?)
    }
}
