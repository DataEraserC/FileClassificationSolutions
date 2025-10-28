// context.rs
// 管理REPL和命令中的上下文状态，如选中的ID

/// 上下文结构体，用于跟踪当前选中的文件、组、标签ID
pub struct Context {
    pub selected_file_id: Option<i32>,
    pub selected_group_id: Option<i32>,
    pub selected_tag_id: Option<i32>,
}

impl Context {
    /// 创建一个新的空上下文
    pub fn new() -> Self {
        Self { selected_file_id: None, selected_group_id: None, selected_tag_id: None }
    }
}
