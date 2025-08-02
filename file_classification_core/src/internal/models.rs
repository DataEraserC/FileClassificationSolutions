use super::schema::{file_groups, files, group_tags, groups, tags};
use chrono;
use diesel::prelude::*;

// File Related

#[derive(Queryable, Selectable)]
#[diesel(table_name = files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
	pub id: i32,
	pub type_: String,
	pub path: String,
	pub reference_count: i32,
	pub group_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = files)]
pub struct CreateFileDTO<'a, 'b> {
	pub type_: &'a str,
	pub path: &'b str,
	pub group_id: i32,
}

// NOTE: you may find File is like SearchFile
// it is bcs IDK how to unify them
pub struct FileFilter {
	pub id: Option<i32>,
	pub type_: Option<String>,
	pub path: Option<String>,
	pub reference_count: Option<i32>,
	pub group_id: Option<i32>,
}

struct FileSet {
	pub path: Option<String>,
	pub type_: Option<String>,
	pub reference_count: Option<i32>,
	pub group_id: Option<i32>,
}

pub struct UpdateFile {
	pub set: FileSet,
	pub filter: FileFilter,
}

// Group Related

#[derive(Queryable, Selectable)]
#[diesel(table_name = groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Group {
	pub id: i32,
	pub name: String,
	pub reference_count: i32,
	pub is_primary: bool,
	pub click_count: i32,
	pub share_count: i32,
	pub create_time: chrono::NaiveDateTime,
	pub modify_time: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = groups)]
pub struct NewGroup<'a> {
	pub name: &'a str,
}

// NOTE: you may find File is like SearchFile
// it is bcs IDK how to unify them
pub struct GroupFilter {
	pub id: Option<i32>,
	pub name: Option<String>,
	pub reference_count: Option<i32>,
	pub is_primary: Option<bool>,
	pub click_count: Option<i32>,
	pub share_count: Option<i32>,
	pub create_time: Option<chrono::NaiveDateTime>,
	pub modify_time: Option<chrono::NaiveDateTime>,
}

struct GroupSet {
	pub name: Option<String>,
	pub reference_count: Option<i32>,
	pub is_primary: Option<bool>,
	pub click_count: Option<i32>,
	pub share_count: Option<i32>,
	pub create_time: Option<chrono::NaiveDateTime>,
	pub modify_time: Option<chrono::NaiveDateTime>,
}

pub struct UpdateGroup {
	pub set: GroupSet,
	pub filter: GroupFilter,
}

// Tag Related

#[derive(Queryable, Selectable)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
	pub id: i32,
	pub name: String,
	pub reference_count: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct CreateTagDTO<'a> {
	pub name: &'a str,
}

pub struct TagFilter {
	pub id: Option<i32>,
	pub name: Option<String>,
	pub reference_count: Option<i32>,
}

pub struct TagSet {
	pub name: Option<String>,
	pub reference_count: Option<i32>,
}

pub struct UpdateTag {
	pub set: TagSet,
	pub filter: TagFilter,
}

/** FileGroup Related
 */

// NOTE: FileGroupDTO == CreateFileGroupDTO
#[derive(Insertable)]
#[diesel(table_name = file_groups)]
pub struct FileGroupDTO {
	pub file_id: i32,
	pub group_id: i32,
}

/** GroupTag Related
*/

// NOTE: GroupTagDTO == CreateGroupTagDTO
#[derive(Insertable)]
#[diesel(table_name = group_tags)]
pub struct GroupTagDTO {
	pub group_id: i32,
	pub tag_id: i32,
}
