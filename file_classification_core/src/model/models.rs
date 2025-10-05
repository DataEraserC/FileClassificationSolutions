use super::schema::{file_groups, files, group_tags, groups, tags};
use chrono;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/** File Related
*/

#[derive(Queryable, Selectable, AsChangeset, Serialize, Deserialize, Clone)]
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
pub struct CreateFileDTO {
    pub type_: String,
    pub path: String,
    pub group_id: i32,
}

#[derive(serde::Deserialize, Clone)]
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

    IdIn(Vec<i32>),
    TypeIn(Vec<String>),
    PathIn(Vec<String>),
    ReferenceCountIn(Vec<i32>),
    GroupIdIn(Vec<i32>),

    And(Vec<FileCondition>),
    Or(Vec<FileCondition>),
    Not(Box<FileCondition>),
}

#[derive(Default)]
pub struct FileQueryOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: Vec<FileOrderBy>,
}

pub enum FileOrderBy {
    Id(OrderDirection),
    Type(OrderDirection),
    Path(OrderDirection),
    ReferenceCount(OrderDirection),
    GroupId(OrderDirection),
}

#[derive(AsChangeset, serde::Deserialize, Default)]
#[diesel(table_name = files)]
pub struct UpdateFileDTO {
    pub path: Option<String>,
    pub type_: Option<String>,
    pub reference_count: Option<i32>,
    pub group_id: Option<i32>,
}

// NOTE: you may find File is like SearchFile
// it is bcs IDK how to unify them,
// and it was used to select or update or delete
#[derive(serde::Deserialize)]
pub struct FileFilter {
    pub id: Option<i32>,
    pub type_: Option<String>,
    pub path: Option<String>,
    pub reference_count: Option<i32>,
    pub group_id: Option<i32>,
}

pub struct FileSet {
    pub path: Option<String>,
    pub type_: Option<String>,
    pub reference_count: Option<i32>,
    pub group_id: Option<i32>,
}

 pub struct UpdateFile {
    pub set: FileSet,
    pub filter: FileFilter,
}

/** Group Related
*/

#[derive(Queryable, Selectable, Serialize, Clone)]
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

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = groups)]
pub struct CreateGroupDTO {
    pub name: String,
}


#[derive(Deserialize,Clone)]
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

    IdIn(Vec<i32>),
    NameIn(Vec<String>),
    ReferenceCountIn(Vec<i32>),
    ClickCountIn(Vec<i32>),
    ShareCountIn(Vec<i32>),
    CreateTimeIn(Vec<chrono::NaiveDateTime>),
    ModifyTimeIn(Vec<chrono::NaiveDateTime>),

    And(Vec<GroupCondition>),
    Or(Vec<GroupCondition>),
    Not(Box<GroupCondition>),
}

#[derive(Default)]
pub struct GroupQueryOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: Vec<GroupOrderBy>,
}

pub enum GroupOrderBy {
    Id(OrderDirection),
    Name(OrderDirection),
    ReferenceCount(OrderDirection),
    IsPrimary(OrderDirection),
    ClickCount(OrderDirection),
    ShareCount(OrderDirection),
    CreateTime(OrderDirection),
    ModifyTime(OrderDirection),
}

pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(AsChangeset, Deserialize, Default, Debug)]
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
// it is bcs IDK how to unify them,
// and it was used to select or update or delete
#[derive(Deserialize)]
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

#[derive(Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub reference_count: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = tags)]
pub struct CreateTagDTO {
    pub name: String,
}

#[derive(Deserialize, Clone)]
pub enum TagCondition {
    Id(i32),
    Name(String),
    ReferenceCount(i32),

    IdGreaterThan(i32),
    IdLessThan(i32),
    NameLike(String),
    ReferenceCountGreaterThan(i32),
    ReferenceCountLessThan(i32),

    IdIn(Vec<i32>),
    NameIn(Vec<String>),
    ReferenceCountIn(Vec<i32>),

    And(Vec<TagCondition>),
    Or(Vec<TagCondition>),
    Not(Box<TagCondition>),

    // GroupId(i32),
}

#[derive(Default)]
pub struct TagQueryOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: Vec<TagOrderBy>,
}

pub enum TagOrderBy {
    Id(OrderDirection),
    Name(OrderDirection),
    ReferenceCount(OrderDirection),
}

#[derive(AsChangeset, Deserialize, Default, Debug)]
#[diesel(table_name = tags)]
pub struct UpdateTagDTO {
    pub name: Option<String>,
    pub reference_count: Option<i32>,
}

#[derive(Deserialize)]
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
#[derive(Queryable, Insertable, Serialize, serde::Deserialize)]
#[diesel(table_name = file_groups)]
pub struct FileGroupDTO {
    pub file_id: i32,
    pub group_id: i32,
}

#[derive(serde::Deserialize, Clone)]
pub enum FileGroupCondition {
    FileId(i32),
    GroupId(i32),

    FileIdGreaterThan(i32),
    FileIdLessThan(i32),
    GroupIdGreaterThan(i32),
    GroupIdLessThan(i32),

    FileIdIn(Vec<i32>),
    GroupIdIn(Vec<i32>),

    And(Vec<FileGroupCondition>),
    Or(Vec<FileGroupCondition>),
    Not(Box<FileGroupCondition>),
}

#[derive(Default)]
pub struct FileGroupQueryOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: Vec<FileGroupOrderBy>,
}

pub enum FileGroupOrderBy {
    FileId(OrderDirection),
    GroupId(OrderDirection),
}

/** GroupTag Related
*/

// NOTE: GroupTagDTO == CreateGroupTagDTO
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = group_tags)]
pub struct GroupTagDTO {
    pub group_id: i32,
    pub tag_id: i32,
}

#[derive(serde::Deserialize, Clone)]
pub enum GroupTagCondition {
    GroupId(i32),
    TagId(i32),

    GroupIdGreaterThan(i32),
    GroupIdLessThan(i32),
    TagIdGreaterThan(i32),
    TagIdLessThan(i32),

    GroupIdIn(Vec<i32>),
    TagIdIn(Vec<i32>),

    And(Vec<GroupTagCondition>),
    Or(Vec<GroupTagCondition>),
    Not(Box<GroupTagCondition>),
}

#[derive(Default)]
pub struct GroupTagQueryOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: Vec<GroupTagOrderBy>,
}

pub enum GroupTagOrderBy {
    GroupId(OrderDirection),
    TagId(OrderDirection),
}