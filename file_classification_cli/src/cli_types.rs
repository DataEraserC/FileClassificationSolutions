use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "文件分类系统", version = "1.0", author = "Developer")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 文件相关操作
    File {
        #[clap(subcommand)]
        action: FileActions,
    },
    /// 组相关操作
    Group {
        #[clap(subcommand)]
        action: GroupActions,
    },
    /// 标签相关操作
    Tag {
        #[clap(subcommand)]
        action: TagActions,
    },
    /// 文件组关联操作
    FileGroup {
        #[clap(subcommand)]
        action: FileGroupActions,
    },
    /// 组标签关联操作
    GroupTag {
        #[clap(subcommand)]
        action: GroupTagActions,
    },
    /// 进入 REPL 模式
    Repl,
    /// 执行脚本文件
    Script {
        #[clap(short, long)]
        file: String,
    },
}

#[derive(Subcommand)]
pub enum FileActions {
    /// 创建文件
    Create {
        #[clap(short, long)]
        type_: Option<String>,
        #[clap(short, long)]
        path: Option<String>,
        #[clap(short, long)]
        group_id: Option<i32>,
    },
    /// 删除文件
    Delete {
        #[clap(short, long)]
        id: Option<i32>,
    },
    /// 查询文件（交互式）
    ListInteractive,
    /// 根据条件查询文件
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
        order_by: Vec<String>,
        #[clap(long)]
        limit: Option<i64>,
        #[clap(long)]
        offset: Option<i64>,
    },
    /// 根据组ID查询文件
    ListByGroupId {
        #[clap(short, long)]
        group_id: i64,
    },
    /// 更新文件
    UpdateByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(short, long)]
        path: Option<String>,
        #[clap(long)]
        type_: Option<String>,
        #[clap(long)]
        reference_count: Option<i32>,
        #[clap(long)]
        group_id: Option<i32>,
    },
    /// 删除文件（按条件）
    DeleteByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum GroupActions {
    /// 创建组
    Create {
        #[clap(short, long)]
        name: Option<String>,
    },
    /// 删除组
    Delete {
        #[clap(short, long)]
        id: Option<i32>,
    },
    /// 查询组（交互式）
    ListInteractive,
    /// 根据条件查询组
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
        order_by: Vec<String>,
        #[clap(long)]
        limit: Option<i64>,
        #[clap(long)]
        offset: Option<i64>,
    },
    /// 根据文件ID查询组
    ListByFileId {
        #[clap(short, long)]
        file_id: i32,
    },
    /// 根据标签ID查询组
    ListByTagId {
        #[clap(short, long)]
        tag_id: i32,
    },
    /// 更新组
    UpdateByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(short, long)]
        name: Option<String>,
        #[clap(long)]
        reference_count: Option<i32>,
        #[clap(long)]
        is_primary: Option<bool>,
        #[clap(long)]
        click_count: Option<i32>,
        #[clap(long)]
        share_count: Option<i32>,
    },
    /// 删除组（按条件）
    DeleteByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum TagActions {
    /// 创建标签
    Create {
        #[clap(short, long)]
        name: Option<String>,
    },
    /// 删除标签
    Delete {
        #[clap(short, long)]
        id: i32,
    },
    /// 查询标签（交互式）
    ListInteractive,
    /// 根据条件查询标签
    ListByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
        order_by: Vec<String>,
        #[clap(long)]
        limit: Option<i64>,
        #[clap(long)]
        offset: Option<i64>,
    },
    /// 根据组ID查询标签
    ListByGroupId {
        #[clap(short, long)]
        group_id: i64,
    },
    /// 更新标签
    UpdateByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
        #[clap(short, long)]
        name: Option<String>,
        #[clap(long)]
        reference_count: Option<i32>,
    },
    /// 删除标签（按条件）
    DeleteByConditions {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        conditions: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum FileGroupActions {
    /// 关联文件和组
    Link {
        #[clap(short, long)]
        file_id: i32,
        #[clap(short, long)]
        group_id: i32,
    },
    /// 解除文件和组的关联
    Unlink {
        #[clap(short, long)]
        file_id: i32,
        #[clap(short, long)]
        group_id: i32,
    },
}

#[derive(Subcommand)]
pub enum GroupTagActions {
    /// 关联组和标签
    Link {
        #[clap(short, long)]
        group_id: i32,
        #[clap(short, long)]
        tag_id: i32,
    },
    /// 解除组和标签的关联
    Unlink {
        #[clap(short, long)]
        group_id: i32,
        #[clap(short, long)]
        tag_id: i32,
    },
}