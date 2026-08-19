use diesel::RunQueryDsl;
pub use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use file_classification_common::env_loader::load_env_file;
use file_classification_core::utils::database::AnyConnection;
use std::env;

// 定义连接池类型
pub type DbPool = Pool<ConnectionManager<AnyConnection>>;
// 定义池化连接类型
pub type DbPooledConnection = PooledConnection<ConnectionManager<AnyConnection>>;

/// 自定义连接定制器，用于在获取连接后执行 PRAGMA 命令
#[derive(Debug)]
struct ConnectionCustomizer;

impl diesel::r2d2::CustomizeConnection<AnyConnection, diesel::r2d2::Error>
  for ConnectionCustomizer
{
  fn on_acquire(&self, conn: &mut AnyConnection) -> Result<(), diesel::r2d2::Error> {
    match conn {
      #[cfg(feature = "sqlite")]
      // SQLite 外键约束默认关闭，需要显式开启
      AnyConnection::Sqlite(_) => {
        diesel::sql_query("PRAGMA foreign_keys = ON")
          .execute(conn)
          .map_err(diesel::r2d2::Error::QueryError)?;
      }
      _ => {}
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
    #[cfg(feature = "sqlite")]
    "sqlite" => ConnectionManager::<AnyConnection>::new(&database_url),

    #[cfg(feature = "mysql")]
    "mysql" => ConnectionManager::<AnyConnection>::new(&database_url),

    #[cfg(feature = "postgres")]
    "postgres" => ConnectionManager::<AnyConnection>::new(&database_url),

    _ => panic!("Unsupported database type: {} or feature not enabled", database_type),
  };

  Pool::builder()
    .connection_customizer(Box::new(ConnectionCustomizer))
    .build(manager)
    .expect("Failed to create pool.")
}
