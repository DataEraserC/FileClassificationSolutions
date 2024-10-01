use crate::schema::{files, groups, tags,file_groups, group_tags};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: Option<i32>,
    pub type_: String,
    pub path: String,
    pub reference_count: Option<i32>,
    pub group_id: Option<i32>,
}
#[derive(Insertable)]
#[diesel(table_name = files)]
pub struct NewFile<'a> {
    pub type_: &'a str,
    pub path: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Group {
    pub id: Option<i32>,
    pub name: String,
    pub is_primary: bool,
    pub click_count: Option<i32>,
    pub share_count: Option<i32>,
    pub create_time: i64,
    pub modify_time: i64,
}
#[derive(Insertable)]
#[diesel(table_name = groups)]
pub struct NewGroup<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: Option<i32>,
    pub name: String,
    pub reference_count: Option<i32>,
}
#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag<'a> {
    pub name: &'a str,
}


#[derive(Insertable)]
#[diesel(table_name = file_groups)]
pub struct NewFileGroup {
    pub file_id: Option<i32>,
    pub group_id: Option<i32>,
}
#[derive(Insertable)]
#[diesel(table_name = group_tags)]
pub struct NewGroupTag {
    pub group_id: Option<i32>,
    pub tag_id: Option<i32>,
}