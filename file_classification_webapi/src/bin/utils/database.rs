pub use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
pub use diesel::Connection;
use file_classification_common::env_loader::load_env_file;
use file_classification_core::utils::database::AnyConnection;
use std::env;

// 定义连接池类型
pub type DbPool = Pool<ConnectionManager<AnyConnection>>;
// 定义池化连接类型
pub type DbPooledConnection = PooledConnection<ConnectionManager<AnyConnection>>;

pub fn establish_connection_pool() -> DbPool {
    // 加载环境变量文件
    if let Err(e) = load_env_file() {
        eprintln!("加载环境变量文件失败: {}", e);
    }
    
    // NOTE: from ./.env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_type = env::var("DATABASE_TYPE").expect("DATABASE_TYPE must be set");

    let manager = match database_type.as_str() {
        "sqlite" => ConnectionManager::<AnyConnection>::new(&database_url),
        _ => panic!("Unsupported database type: {}", database_type),
    };

    Pool::builder().build(manager).expect("Failed to create pool.")
}