use super::schema::{file_groups, files, group_tags, groups, tags};
use chrono;
use diesel::prelude::*;
use serde::Serialize;

/** File Related
*/

#[derive(Queryable, Selectable, AsChangeset, Serialize)]
#[diesel(table_name = files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(treat_none_as_null = true)]
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

pub enum FileCondition {
    Id(i32),
    Type(String),
    Path(String),
    ReferenceCount(i32),
    GroupId(i32),

    IdGreaterThan(i32),
    IdLessThan(i32),
    TypeLike(String),
    PathLike(String),
    ReferenceCountGreaterThan(i32),
    ReferenceCountLessThan(i32),
    GroupIdGreaterThan(i32),
    GroupIdLessThan(i32),

    And(Vec<FileCondition>),
    Or(Vec<FileCondition>),
    Not(Box<FileCondition>),
}

#[derive(AsChangeset)]
#[diesel(table_name = files)]
pub struct UpdateFileDTO {
    pub path: Option<String>,
    pub type_: Option<String>,
    pub reference_count: Option<i32>,
    pub group_id: Option<i32>,
}

/** Group Related
*/

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
pub struct CreateGroupDTO<'a> {
    pub name: &'a str,
}

pub enum GroupCondition {
    Id(i32),
    Name(String),
    ReferenceCount(i32),
    IsPrimary(bool),
    ClickCount(i32),
    ShareCount(i32),
    CreateTime(chrono::NaiveDateTime),
    ModifyTime(chrono::NaiveDateTime),

    IdGreaterThan(i32),
    IdLessThan(i32),
    NameLike(String),
    ReferenceCountGreaterThan(i32),
    ReferenceCountLessThan(i32),
    ClickCountGreaterThan(i32),
    ClickCountLessThan(i32),
    ShareCountGreaterThan(i32),
    ShareCountLessThan(i32),
    CreateTimeGreaterThan(chrono::NaiveDateTime),
    CreateTimeLessThan(chrono::NaiveDateTime),
    ModifyTimeGreaterThan(chrono::NaiveDateTime),
    ModifyTimeLessThan(chrono::NaiveDateTime),

    And(Vec<GroupCondition>),
    Or(Vec<GroupCondition>),
    Not(Box<GroupCondition>),
}

#[derive(AsChangeset)]
#[diesel(table_name = groups)]
pub struct UpdateGroupDTO {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub reference_count: Option<i32>,
    pub is_primary: Option<bool>,
    pub click_count: Option<i32>,
    pub share_count: Option<i32>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub modify_time: Option<chrono::NaiveDateTime>,
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

pub struct GroupSet {
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

/** Tag Related
*/

#[derive(Queryable, Selectable, Serialize)]
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

pub enum TagCondition {
    Id(i32),
    Name(String),
    ReferenceCount(i32),

    IdGreaterThan(i32),
    IdLessThan(i32),
    NameLike(String),
    ReferenceCountGreaterThan(i32),
    ReferenceCountLessThan(i32),

    And(Vec<TagCondition>),
    Or(Vec<TagCondition>),
    Not(Box<TagCondition>),
}

#[derive(AsChangeset)]
#[diesel(table_name = tags)]
pub struct UpdateTagDTO {
    pub name: Option<String>,
    pub reference_count: Option<i32>,
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
#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = file_groups)]
pub struct FileGroupDTO {
    pub file_id: i32,
    pub group_id: i32,
}

pub enum FileGroupCondition {
    FileId(i32),
    GroupId(i32),

    FileIdGreaterThan(i32),
    FileIdLessThan(i32),
    GroupIdGreaterThan(i32),
    GroupIdLessThan(i32),

    And(Vec<FileGroupCondition>),
    Or(Vec<FileGroupCondition>),
    Not(Box<FileGroupCondition>),
}

/** GroupTag Related
*/

// NOTE: GroupTagDTO == CreateGroupTagDTO
#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = group_tags)]
pub struct GroupTagDTO {
    pub group_id: i32,
    pub tag_id: i32,
}

pub enum GroupTagCondition {
    GroupId(i32),
    TagId(i32),

    GroupIdGreaterThan(i32),
    GroupIdLessThan(i32),
    TagIdGreaterThan(i32),
    TagIdLessThan(i32),

    And(Vec<GroupTagCondition>),
    Or(Vec<GroupTagCondition>),
    Not(Box<GroupTagCondition>),
}