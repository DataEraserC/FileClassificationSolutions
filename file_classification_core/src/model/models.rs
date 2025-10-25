// models.rs
//! 数据模型定义模块
//!
//! 定义了应用程序中使用的所有数据库实体模型、数据传输对象(DTO)、查询条件和更新对象，
//! 包括文件、分组、标签以及它们之间关联关系的相关结构。

use std::fmt::{Debug, Formatter, Result as fmtResult};
use super::schema::{file_groups, files, group_tags, groups, tags};
use chrono;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/** File Related
 文件相关数据模型
*/

/// 文件实体模型
/// 
/// 对应数据库中的 `files` 表，表示系统中的一个文件记录
#[derive(Queryable, Selectable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(treat_none_as_null = true)]
pub struct File {
    /// 文件ID，主键
    pub id: i32,
    /// 文件类型/扩展名
    pub type_: String,
    /// 文件路径
    pub path: String,
    /// 引用计数，表示有多少个分组关联了该文件
    pub reference_count: i32,
    /// 主分组ID，表示该文件属于哪个分组
    pub group_id: i32,
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "File {{ id: {}, type_: {}, path: {}, reference_count: {}, group_id: {} }}",
            self.id, self.type_, self.path, self.reference_count, self.group_id
        )
    }
}

/// 创建文件的DTO对象
/// 
/// 用于向数据库插入新文件记录时的数据传输对象
#[derive(Insertable, Deserialize)]
#[diesel(table_name = files)]
pub struct CreateFileDTO {
    /// 文件类型/扩展名
    pub type_: String,
    /// 文件路径
    pub path: String,
    /// 主分组ID
    pub group_id: i32,
}

/// 文件查询条件枚举
/// 
/// 定义了可以用于查询文件的各种条件类型，支持复合条件查询
#[derive(serde::Deserialize, Clone)]
pub enum FileCondition {
    // 基本相等条件
    /// 根据文件ID查询
    Id(i32),
    /// 根据文件类型查询
    Type(String),
    /// 根据文件路径查询
    Path(String),
    /// 根据引用计数查询
    ReferenceCount(i32),
    /// 根据分组ID查询
    GroupId(i32),

    // 范围比较条件
    /// 文件ID大于指定值
    IdGreaterThan(i32),
    /// 文件ID小于指定值
    IdLessThan(i32),
    /// 文件类型模糊匹配
    TypeLike(String),
    /// 文件路径模糊匹配
    PathLike(String),
    /// 引用计数大于指定值
    ReferenceCountGreaterThan(i32),
    /// 引用计数小于指定值
    ReferenceCountLessThan(i32),
    /// 分组ID大于指定值
    GroupIdGreaterThan(i32),
    /// 分组ID小于指定值
    GroupIdLessThan(i32),

    // 集合包含条件
    /// 文件ID在指定集合中
    IdIn(Vec<i32>),
    /// 文件类型在指定集合中
    TypeIn(Vec<String>),
    /// 文件路径在指定集合中
    PathIn(Vec<String>),
    /// 引用计数在指定集合中
    ReferenceCountIn(Vec<i32>),
    /// 分组ID在指定集合中
    GroupIdIn(Vec<i32>),

    // 逻辑组合条件
    /// AND逻辑组合多个条件
    And(Vec<FileCondition>),
    /// OR逻辑组合多个条件
    Or(Vec<FileCondition>),
    /// NOT逻辑取反条件
    Not(Box<FileCondition>),
}

/// 文件查询选项结构体
/// 
/// 定义了文件查询的可选参数，如分页、排序等
#[derive(Default, Deserialize)]
pub struct FileQueryOptions {
    /// 查询结果限制数量
    pub limit: Option<i64>,
    /// 查询结果偏移量（用于分页）
    pub offset: Option<i64>,
    /// 排序条件列表
    pub order_by: Vec<FileOrderBy>,
}

/// 文件排序字段枚举
/// 
/// 定义了可以用于文件查询结果排序的字段
#[derive(Deserialize)]
pub enum FileOrderBy {
    /// 按文件ID排序
    Id(OrderDirection),
    /// 按文件类型排序
    Type(OrderDirection),
    /// 按文件路径排序
    Path(OrderDirection),
    /// 按引用计数排序
    ReferenceCount(OrderDirection),
    /// 按分组ID排序
    GroupId(OrderDirection),
}

/// 更新文件的DTO对象
/// 
/// 用于更新文件记录时的数据传输对象，所有字段都是可选的
#[derive(AsChangeset, serde::Deserialize, Default)]
#[diesel(table_name = files)]
pub struct UpdateFileDTO {
    /// 文件路径（可选）
    pub path: Option<String>,
    /// 文件类型（可选）
    pub type_: Option<String>,
    /// 引用计数（可选）
    pub reference_count: Option<i32>,
    /// 分组ID（可选）
    pub group_id: Option<i32>,
}

/// 文件过滤条件结构体
/// 
/// 用于简单文件查询的过滤条件，各字段都是可选的
#[derive(serde::Deserialize)]
pub struct FileFilter {
    /// 文件ID过滤条件
    pub id: Option<i32>,
    /// 文件类型过滤条件
    pub type_: Option<String>,
    /// 文件路径过滤条件
    pub path: Option<String>,
    /// 引用计数过滤条件
    pub reference_count: Option<i32>,
    /// 分组ID过滤条件
    pub group_id: Option<i32>,
}

/// 文件更新字段集合
/// 
/// 定义了可以更新的文件字段，各字段都是可选的
pub struct FileSet {
    /// 文件路径（可选）
    pub path: Option<String>,
    /// 文件类型（可选）
    pub type_: Option<String>,
    /// 引用计数（可选）
    pub reference_count: Option<i32>,
    /// 分组ID（可选）
    pub group_id: Option<i32>,
}

/// 文件更新结构体
/// 
/// 包含了文件更新的字段集合和过滤条件
pub struct UpdateFile {
    /// 要更新的字段集合
    pub set: FileSet,
    /// 更新的过滤条件
    pub filter: FileFilter,
}

/** Group Related
 分组相关数据模型
*/

/// 分组实体模型
/// 
/// 对应数据库中的 `groups` 表，表示系统中的一个分组记录
#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Group {
    /// 分组ID，主键
    pub id: i32,
    /// 分组名称
    pub name: String,
    /// 引用计数，表示有多少个文件属于该分组
    pub reference_count: i32,
    /// 是否为主分组
    pub is_primary: bool,
    /// 点击次数
    pub click_count: i32,
    /// 分享次数
    pub share_count: i32,
    /// 创建时间
    pub create_time: chrono::NaiveDateTime,
    /// 修改时间
    pub modify_time: chrono::NaiveDateTime,
}

impl Debug for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "Group {{ id: {}, name: {}, reference_count: {}, is_primary: {}, click_count: {}, share_count: {}, create_time: {}, modify_time: {} }}",
            self.id,
            self.name,
            self.reference_count,
            self.is_primary,
            self.click_count,
            self.share_count,
            self.create_time,
            self.modify_time
        )
    }
}

/// 创建分组的DTO对象
/// 
/// 用于向数据库插入新分组记录时的数据传输对象
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = groups)]
pub struct CreateGroupDTO {
    /// 分组名称
    pub name: String,
}

/// 分组查询条件枚举
/// 
/// 定义了可以用于查询分组的各种条件类型，支持复合条件查询
#[derive(Deserialize,Clone)]
pub enum GroupCondition {
    // 基本相等条件
    /// 根据分组ID查询
    Id(i32),
    /// 根据分组名称查询
    Name(String),
    /// 根据引用计数查询
    ReferenceCount(i32),
    /// 根据是否为主分组查询
    IsPrimary(bool),
    /// 根据点击次数查询
    ClickCount(i32),
    /// 根据分享次数查询
    ShareCount(i32),
    /// 根据创建时间查询
    CreateTime(chrono::NaiveDateTime),
    /// 根据修改时间查询
    ModifyTime(chrono::NaiveDateTime),

    // 范围比较条件
    /// 分组ID大于指定值
    IdGreaterThan(i32),
    /// 分组ID小于指定值
    IdLessThan(i32),
    /// 分组名称模糊匹配
    NameLike(String),
    /// 引用计数大于指定值
    ReferenceCountGreaterThan(i32),
    /// 引用计数小于指定值
    ReferenceCountLessThan(i32),
    /// 点击次数大于指定值
    ClickCountGreaterThan(i32),
    /// 点击次数小于指定值
    ClickCountLessThan(i32),
    /// 分享次数大于指定值
    ShareCountGreaterThan(i32),
    /// 分享次数小于指定值
    ShareCountLessThan(i32),
    /// 创建时间晚于指定时间
    CreateTimeGreaterThan(chrono::NaiveDateTime),
    /// 创建时间早于指定时间
    CreateTimeLessThan(chrono::NaiveDateTime),
    /// 修改时间晚于指定时间
    ModifyTimeGreaterThan(chrono::NaiveDateTime),
    /// 修改时间早于指定时间
    ModifyTimeLessThan(chrono::NaiveDateTime),

    // 集合包含条件
    /// 分组ID在指定集合中
    IdIn(Vec<i32>),
    /// 分组名称在指定集合中
    NameIn(Vec<String>),
    /// 引用计数在指定集合中
    ReferenceCountIn(Vec<i32>),
    /// 点击次数在指定集合中
    ClickCountIn(Vec<i32>),
    /// 分享次数在指定集合中
    ShareCountIn(Vec<i32>),
    /// 创建时间在指定集合中
    CreateTimeIn(Vec<chrono::NaiveDateTime>),
    /// 修改时间在指定集合中
    ModifyTimeIn(Vec<chrono::NaiveDateTime>),

    // 逻辑组合条件
    /// AND逻辑组合多个条件
    And(Vec<GroupCondition>),
    /// OR逻辑组合多个条件
    Or(Vec<GroupCondition>),
    /// NOT逻辑取反条件
    Not(Box<GroupCondition>),
}

/// 分组查询选项结构体
/// 
/// 定义了分组查询的可选参数，如分页、排序等
#[derive(Default, Deserialize)]
pub struct GroupQueryOptions {
    /// 查询结果限制数量
    pub limit: Option<i64>,
    /// 查询结果偏移量（用于分页）
    pub offset: Option<i64>,
    /// 排序条件列表
    pub order_by: Vec<GroupOrderBy>,
}

/// 分组排序字段枚举
/// 
/// 定义了可以用于分组查询结果排序的字段
#[derive(Deserialize)]
pub enum GroupOrderBy {
    /// 按分组ID排序
    Id(OrderDirection),
    /// 按分组名称排序
    Name(OrderDirection),
    /// 按引用计数排序
    ReferenceCount(OrderDirection),
    /// 按是否为主分组排序
    IsPrimary(OrderDirection),
    /// 按点击次数排序
    ClickCount(OrderDirection),
    /// 按分享次数排序
    ShareCount(OrderDirection),
    /// 按创建时间排序
    CreateTime(OrderDirection),
    /// 按修改时间排序
    ModifyTime(OrderDirection),
}

/// 排序方向枚举
/// 
/// 定义了排序的方向，升序或降序
#[derive(Deserialize)]
pub enum OrderDirection {
    /// 升序排列
    Asc,
    /// 降序排列
    Desc,
}

/// 更新分组的DTO对象
/// 
/// 用于更新分组记录时的数据传输对象，所有字段都是可选的
#[derive(AsChangeset, Deserialize, Default, Debug)]
#[diesel(table_name = groups)]
pub struct UpdateGroupDTO {
    /// 分组ID（可选）
    pub id: Option<i32>,
    /// 分组名称（可选）
    pub name: Option<String>,
    /// 引用计数（可选）
    pub reference_count: Option<i32>,
    /// 是否为主分组（可选）
    pub is_primary: Option<bool>,
    /// 点击次数（可选）
    pub click_count: Option<i32>,
    /// 分享次数（可选）
    pub share_count: Option<i32>,
    /// 创建时间（可选）
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 修改时间（可选）
    pub modify_time: Option<chrono::NaiveDateTime>,
}

/// 分组过滤条件结构体
/// 
/// 用于简单分组查询的过滤条件，各字段都是可选的
#[derive(Deserialize)]
pub struct GroupFilter {
    /// 分组ID过滤条件
    pub id: Option<i32>,
    /// 分组名称过滤条件
    pub name: Option<String>,
    /// 引用计数过滤条件
    pub reference_count: Option<i32>,
    /// 是否为主分组过滤条件
    pub is_primary: Option<bool>,
    /// 点击次数过滤条件
    pub click_count: Option<i32>,
    /// 分享次数过滤条件
    pub share_count: Option<i32>,
    /// 创建时间过滤条件
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 修改时间过滤条件
    pub modify_time: Option<chrono::NaiveDateTime>,
}

/// 分组更新字段集合
/// 
/// 定义了可以更新的分组字段，各字段都是可选的
pub struct GroupSet {
    /// 分组名称（可选）
    pub name: Option<String>,
    /// 引用计数（可选）
    pub reference_count: Option<i32>,
    /// 是否为主分组（可选）
    pub is_primary: Option<bool>,
    /// 点击次数（可选）
    pub click_count: Option<i32>,
    /// 分享次数（可选）
    pub share_count: Option<i32>,
    /// 创建时间（可选）
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 修改时间（可选）
    pub modify_time: Option<chrono::NaiveDateTime>,
}

/// 分组更新结构体
/// 
/// 包含了分组更新的字段集合和过滤条件
pub struct UpdateGroup {
    /// 要更新的字段集合
    pub set: GroupSet,
    /// 更新的过滤条件
    pub filter: GroupFilter,
}

/** Tag Related
 标签相关数据模型
*/

/// 标签实体模型
/// 
/// 对应数据库中的 `tags` 表，表示系统中的一个标签记录
#[derive(Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    /// 标签ID，主键
    pub id: i32,
    /// 标签名称
    pub name: String,
    /// 引用计数，表示有多少个分组关联了该标签
    pub reference_count: i32,
}

impl Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "Tag {{ id: {}, name: {}, reference_count: {} }}",
            self.id, self.name, self.reference_count
        )
    }
}

/// 创建标签的DTO对象
/// 
/// 用于向数据库插入新标签记录时的数据传输对象
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = tags)]
pub struct CreateTagDTO {
    /// 标签名称
    pub name: String,
}

/// 标签查询条件枚举
/// 
/// 定义了可以用于查询标签的各种条件类型，支持复合条件查询
#[derive(Deserialize, Clone)]
pub enum TagCondition {
    // 基本相等条件
    /// 根据标签ID查询
    Id(i32),
    /// 根据标签名称查询
    Name(String),
    /// 根据引用计数查询
    ReferenceCount(i32),

    // 范围比较条件
    /// 标签ID大于指定值
    IdGreaterThan(i32),
    /// 标签ID小于指定值
    IdLessThan(i32),
    /// 标签名称模糊匹配
    NameLike(String),
    /// 引用计数大于指定值
    ReferenceCountGreaterThan(i32),
    /// 引用计数小于指定值
    ReferenceCountLessThan(i32),

    // 集合包含条件
    /// 标签ID在指定集合中
    IdIn(Vec<i32>),
    /// 标签名称在指定集合中
    NameIn(Vec<String>),
    /// 引用计数在指定集合中
    ReferenceCountIn(Vec<i32>),

    // 逻辑组合条件
    /// AND逻辑组合多个条件
    And(Vec<TagCondition>),
    /// OR逻辑组合多个条件
    Or(Vec<TagCondition>),
    /// NOT逻辑取反条件
    Not(Box<TagCondition>),
}

/// 标签查询选项结构体
/// 
/// 定义了标签查询的可选参数，如分页、排序等
#[derive(Default, Deserialize)]
pub struct TagQueryOptions {
    /// 查询结果限制数量
    pub limit: Option<i64>,
    /// 查询结果偏移量（用于分页）
    pub offset: Option<i64>,
    /// 排序条件列表
    pub order_by: Vec<TagOrderBy>,
}

/// 标签排序字段枚举
/// 
/// 定义了可以用于标签查询结果排序的字段
#[derive(Deserialize)]
pub enum TagOrderBy {
    /// 按标签ID排序
    Id(OrderDirection),
    /// 按标签名称排序
    Name(OrderDirection),
    /// 按引用计数排序
    ReferenceCount(OrderDirection),
}

/// 更新标签的DTO对象
/// 
/// 用于更新标签记录时的数据传输对象，所有字段都是可选的
#[derive(AsChangeset, Deserialize, Default, Debug)]
#[diesel(table_name = tags)]
pub struct UpdateTagDTO {
    /// 标签名称（可选）
    pub name: Option<String>,
    /// 引用计数（可选）
    pub reference_count: Option<i32>,
}

/// 标签过滤条件结构体
/// 
/// 用于简单标签查询的过滤条件，各字段都是可选的
#[derive(Deserialize)]
pub struct TagFilter {
    /// 标签ID过滤条件
    pub id: Option<i32>,
    /// 标签名称过滤条件
    pub name: Option<String>,
    /// 引用计数过滤条件
    pub reference_count: Option<i32>,
}

/// 标签更新字段集合
/// 
/// 定义了可以更新的标签字段，各字段都是可选的
pub struct TagSet {
    /// 标签名称（可选）
    pub name: Option<String>,
    /// 引用计数（可选）
    pub reference_count: Option<i32>,
}

/// 标签更新结构体
/// 
/// 包含了标签更新的字段集合和过滤条件
pub struct UpdateTag {
    /// 要更新的字段集合
    pub set: TagSet,
    /// 更新的过滤条件
    pub filter: TagFilter,
}

/** FileGroup Related
 文件-分组关联关系相关数据模型
 */

/// 文件-分组关联实体模型
/// 
/// 对应数据库中的 `file_groups` 表，表示文件和分组之间的多对多关联关系
#[derive(Queryable, Selectable, Insertable, Serialize, serde::Deserialize)]
#[diesel(table_name = file_groups)]
pub struct FileGroupDTO {
    /// 文件ID，外键关联 `files` 表
    pub file_id: i32,
    /// 分组ID，外键关联 `groups` 表
    pub group_id: i32,
    /// 关联类型，1表示主分组关系
    pub relation_type: i32,
}

impl Debug for FileGroupDTO {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "FileGroup {{ file_id: {}, group_id: {}, relation_type: {} }}", self.file_id, self.group_id, self.relation_type)
    }
}

/// 文件-分组关联查询条件枚举
/// 
/// 定义了可以用于查询文件-分组关联关系的各种条件类型
#[derive(serde::Deserialize, Clone)]
pub enum FileGroupCondition {
    // 基本相等条件
    /// 根据文件ID查询
    FileId(i32),
    /// 根据分组ID查询
    GroupId(i32),
    /// 根据关联类型查询
    RelationType(i32),

    // 范围比较条件
    /// 文件ID大于指定值
    FileIdGreaterThan(i32),
    /// 文件ID小于指定值
    FileIdLessThan(i32),
    /// 分组ID大于指定值
    GroupIdGreaterThan(i32),
    /// 分组ID小于指定值
    GroupIdLessThan(i32),
    /// 关联类型大于指定值
    RelationTypeGreaterThan(i32),
    /// 关联类型小于指定值
    RelationTypeLessThan(i32),

    // 集合包含条件
    /// 文件ID在指定集合中
    FileIdIn(Vec<i32>),
    /// 分组ID在指定集合中
    GroupIdIn(Vec<i32>),
    /// 关联类型在指定集合中
    RelationTypeIn(Vec<i32>),

    // 逻辑组合条件
    /// AND逻辑组合多个条件
    And(Vec<FileGroupCondition>),
    /// OR逻辑组合多个条件
    Or(Vec<FileGroupCondition>),
    /// NOT逻辑取反条件
    Not(Box<FileGroupCondition>),
}

/// 文件-分组关联查询选项结构体
/// 
/// 定义了文件-分组关联查询的可选参数，如分页、排序等
#[derive(Default, Deserialize)]
pub struct FileGroupQueryOptions {
    /// 查询结果限制数量
    pub limit: Option<i64>,
    /// 查询结果偏移量（用于分页）
    pub offset: Option<i64>,
    /// 排序条件列表
    pub order_by: Vec<FileGroupOrderBy>,
}

/// 文件-分组关联排序字段枚举
/// 
/// 定义了可以用于文件-分组关联查询结果排序的字段
#[derive(Deserialize)]
pub enum FileGroupOrderBy {
    /// 按文件ID排序
    FileId(OrderDirection),
    /// 按分组ID排序
    GroupId(OrderDirection),
}

/** GroupTag Related
 分组-标签关联关系相关数据模型
*/

/// 分组-标签关联实体模型
/// 
/// 对应数据库中的 `group_tags` 表，表示分组和标签之间的多对多关联关系
#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = group_tags)]
pub struct GroupTagDTO {
    /// 分组ID，外键关联 `groups` 表
    pub group_id: i32,
    /// 标签ID，外键关联 `tags` 表
    pub tag_id: i32,
}

impl Debug for GroupTagDTO {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "GroupTag {{ group_id: {}, tag_id: {} }}", self.group_id, self.tag_id)
    }
}

/// 分组-标签关联查询条件枚举
/// 
/// 定义了可以用于查询分组-标签关联关系的各种条件类型
#[derive(serde::Deserialize, Clone)]
pub enum GroupTagCondition {
    // 基本相等条件
    /// 根据分组ID查询
    GroupId(i32),
    /// 根据标签ID查询
    TagId(i32),

    // 范围比较条件
    /// 分组ID大于指定值
    GroupIdGreaterThan(i32),
    /// 分组ID小于指定值
    GroupIdLessThan(i32),
    /// 标签ID大于指定值
    TagIdGreaterThan(i32),
    /// 标签ID小于指定值
    TagIdLessThan(i32),

    // 集合包含条件
    /// 分组ID在指定集合中
    GroupIdIn(Vec<i32>),
    /// 标签ID在指定集合中
    TagIdIn(Vec<i32>),

    // 逻辑组合条件
    /// AND逻辑组合多个条件
    And(Vec<GroupTagCondition>),
    /// OR逻辑组合多个条件
    Or(Vec<GroupTagCondition>),
    /// NOT逻辑取反条件
    Not(Box<GroupTagCondition>),
}

/// 分组-标签关联查询选项结构体
/// 
/// 定义了分组-标签关联查询的可选参数，如分页、排序等
#[derive(Default, Deserialize)]
pub struct GroupTagQueryOptions {
    /// 查询结果限制数量
    pub limit: Option<i64>,
    /// 查询结果偏移量（用于分页）
    pub offset: Option<i64>,
    /// 排序条件列表
    pub order_by: Vec<GroupTagOrderBy>,
}

/// 分组-标签关联排序字段枚举
/// 
/// 定义了可以用于分组-标签关联查询结果排序的字段
#[derive(Deserialize)]
pub enum GroupTagOrderBy {
    /// 按分组ID排序
    GroupId(OrderDirection),
    /// 按标签ID排序
    TagId(OrderDirection),
}

/** GroupRelation Related
 组关系相关数据模型
*/

/// 组关系实体模型
/// 
/// 对应数据库中的 `group_relations` 表，表示组和组之间的关联关系
#[derive(Queryable, Selectable, Insertable, Serialize, serde::Deserialize, Clone)]
#[diesel(table_name = crate::model::schema::group_relations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GroupRelation {
    /// 第一个组ID，外键关联 `groups` 表
    pub first_group_id: i32,
    /// 第二个组ID，外键关联 `groups` 表
    pub second_group_id: i32,
    /// 关系类型
    pub relation_type: i32,
}

impl Debug for GroupRelation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "GroupRelation {{ first_group_id: {}, second_group_id: {}, relation_type: {} }}", 
               self.first_group_id, self.second_group_id, self.relation_type)
    }
}

/// 组关系查询条件枚举
/// 
/// 定义了可以用于查询组关系的各种条件类型，支持复合条件查询
#[derive(serde::Deserialize, Clone)]
pub enum GroupRelationCondition {
    // 基本相等条件
    /// 根据第一个组ID查询
    FirstGroupId(i32),
    /// 根据第二个组ID查询
    SecondGroupId(i32),
    /// 根据关系类型查询
    RelationType(i32),

    // 范围比较条件
    /// 第一个组ID大于指定值
    FirstGroupIdGreaterThan(i32),
    /// 第一个组ID小于指定值
    FirstGroupIdLessThan(i32),
    /// 第二个组ID大于指定值
    SecondGroupIdGreaterThan(i32),
    /// 第二个组ID小于指定值
    SecondGroupIdLessThan(i32),
    /// 关系类型大于指定值
    RelationTypeGreaterThan(i32),
    /// 关系类型小于指定值
    RelationTypeLessThan(i32),

    // 集合包含条件
    /// 第一个组ID在指定集合中
    FirstGroupIdIn(Vec<i32>),
    /// 第二个组ID在指定集合中
    SecondGroupIdIn(Vec<i32>),
    /// 关系类型在指定集合中
    RelationTypeIn(Vec<i32>),

    // 逻辑组合条件
    /// AND逻辑组合多个条件
    And(Vec<GroupRelationCondition>),
    /// OR逻辑组合多个条件
    Or(Vec<GroupRelationCondition>),
    /// NOT逻辑取反条件
    Not(Box<GroupRelationCondition>),
}

/// 组关系查询选项结构体
/// 
/// 定义了组关系查询的可选参数，如分页、排序等
#[derive(Default, Deserialize)]
pub struct GroupRelationQueryOptions {
    /// 查询结果限制数量
    pub limit: Option<i64>,
    /// 查询结果偏移量（用于分页）
    pub offset: Option<i64>,
    /// 排序条件列表
    pub order_by: Vec<GroupRelationOrderBy>,
}

/// 组关系排序字段枚举
/// 
/// 定义了可以用于组关系查询结果排序的字段
#[derive(Deserialize)]
pub enum GroupRelationOrderBy {
    /// 按第一个组ID排序
    FirstGroupId(OrderDirection),
    /// 按第二个组ID排序
    SecondGroupId(OrderDirection),
    /// 按关系类型排序
    RelationType(OrderDirection),
}

/// 树节点结构，用于表示组的层级结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupTreeNode {
    /// 组信息
    pub group: Group,
    /// 子节点列表
    pub children: Vec<GroupTreeNode>,
}
