use std::collections::HashMap;

/// CLI上下文，用于存储会话中的变量和状态
#[derive(Debug, Default)]
pub struct Context {
    pub variables: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.variables.remove(key)
    }

    pub fn clear(&mut self) {
        self.variables.clear();
    }
}

/// 打印当前上下文中的变量
pub fn print_context(context: &Context) {
    println!("当前上下文变量:");
    if context.variables.is_empty() {
        println!("  (空)");
    } else {
        for (key, value) in &context.variables {
            println!("  {} = {}", key, value);
        }
    }
}