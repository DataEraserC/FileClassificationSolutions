//! 环境变量加载工具
//! 
//! 提供灵活的环境变量文件加载机制，支持自定义文件名和多重回退策略

use std::env;
use std::fs;
use std::path::Path;

/// 加载环境变量文件
/// 
/// 加载顺序：
/// 1. 通过 ENV_FILE 环境变量指定的文件
/// 2. 回退到 .file_classification_env
/// 3. 如果以上都不存在，则创建带有默认配置的 .file_classification_env
/// 
/// # Returns
/// 返回加载的环境变量文件路径，如果使用默认配置则返回 None
pub fn load_env_file() -> Result<Option<String>, Box<dyn std::error::Error>> {
    // 首先检查是否通过环境变量指定了env文件
    let env_file = env::var("ENV_FILE").unwrap_or_else(|_| ".file_classification_env".to_string());
    
    // 尝试加载指定的env文件
    if Path::new(&env_file).exists() {
        dotenvy::dotenv_override().ok();
        dotenvy::from_filename_override(&env_file)?;
        Ok(Some(env_file))
    } else if env_file != ".file_classification_env" && Path::new(".file_classification_env").exists() {
        // 如果指定了自定义env文件但不存在，回退到.file_classification_env
        dotenvy::dotenv_override().ok();
        dotenvy::from_filename_override(".file_classification_env")?;
        Ok(Some(".file_classification_env".to_string()))
    } else {
        // 如果文件都不存在，创建默认的.file_classification_env
        create_default_env_file()?;
        dotenvy::dotenv_override().ok();
        dotenvy::from_filename_override(".file_classification_env")?;
        Ok(None)
    }
}

/// 创建默认的环境变量配置文件
fn create_default_env_file() -> Result<(), Box<dyn std::error::Error>> {
    let default_content = r#"# File Classification 系统配置文件
# 数据库配置
DATABASE_URL=file_classification.db
DATABASE_TYPE=sqlite

# Web API 配置
BIND_ADDRESS=127.0.0.1
BIND_PORT=8082

# 日志配置
RUST_LOG=info
RUST_LOG_FILE=debug

# CORS 配置
CORS_ENABLED=true
CORS_ORIGIN=http://localhost:8082
"#;
    
    fs::write(".file_classification_env", default_content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;

    #[test]
    fn test_create_default_env_file() {
        // 创建临时目录进行测试
        let test_dir = tempfile::tempdir().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // 切换到临时目录
        env::set_current_dir(test_dir.path()).unwrap();
        
        // 测试创建默认env文件
        assert!(create_default_env_file().is_ok());
        assert!(Path::new(".file_classification_env").exists());
        
        // 检查文件内容
        let content = fs::read_to_string(".file_classification_env").unwrap();
        assert!(content.contains("DATABASE_URL=file_classification.db"));
        assert!(content.contains("DATABASE_TYPE=sqlite"));
        
        // 恢复原始目录
        env::set_current_dir(original_dir).unwrap();
    }
}