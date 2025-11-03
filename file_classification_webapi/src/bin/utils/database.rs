pub use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use file_classification_common::env_loader::load_env_file;
use file_classification_core::utils::database::AnyConnection;
use std::env;
use diesel::RunQueryDsl;

// 定义连接池类型
pub type DbPool = Pool<ConnectionManager<AnyConnection>>;
// 定义池化连接类型
pub type DbPooledConnection = PooledConnection<ConnectionManager<AnyConnection>>;

/// 自定义连接创建器，用于在创建连接后执行 PRAGMA 命令
#[derive(Debug)]
struct ConnectionCreator;

impl diesel::r2d2::CustomizeConnection<AnyConnection, diesel::r2d2::Error> for ConnectionCreator {
    fn on_acquire(&self, conn: &mut AnyConnection) -> Result<(), diesel::r2d2::Error> {
        // 只有 SQLite 连接才这样关闭外键约束
        match conn {
            AnyConnection::Sqlite(_) => {
                // 关闭外键约束检查
                diesel::sql_query("PRAGMA foreign_keys = OFF")
                    .execute(conn)
                    .map_err(diesel::r2d2::Error::QueryError)?;
            }
        }
        Ok(())
    }
}

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

    Pool::builder()
        .connection_customizer(Box::new(ConnectionCreator))
        .build(manager)
        .expect("Failed to create pool.")
}