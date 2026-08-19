// database.rs
//! 数据库连接管理模块
//!
//! 提供数据库连接的建立和管理功能，支持多种数据库类型，
//! 并封装了通用的数据库连接类型。

use diesel::RunQueryDsl;
pub use diesel::{Connection, QueryResult};
use std::fmt;

/// 数据库连接错误类型
#[derive(Debug)]
pub struct ConnectionError {
  pub message: String,
  pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl fmt::Display for ConnectionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl std::error::Error for ConnectionError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
  }
}

impl ConnectionError {
  pub fn new(message: &str) -> Self {
    ConnectionError { message: message.to_string(), source: None }
  }

  pub fn with_source(message: &str, source: Box<dyn std::error::Error + Send + Sync>) -> Self {
    ConnectionError { message: message.to_string(), source: Some(source) }
  }
}

/// 通用数据库连接枚举
///
/// 使用 `diesel::MultiConnection` 宏定义的枚举类型，支持多种数据库连接类型。
#[derive(diesel::MultiConnection)]
pub enum AnyConnection {
  #[cfg(feature = "postgres")]
  /// PostgreSQL 数据库连接
  Postgresql(diesel::PgConnection),

  #[cfg(feature = "mysql")]
  /// MySQL 数据库连接
  Mysql(diesel::MysqlConnection),

  #[cfg(feature = "sqlite")]
  /// SQLite 数据库连接
  Sqlite(diesel::SqliteConnection),
}

/// 建立数据库连接
///
/// 根据提供的数据库URL和类型建立相应的数据库连接。
///
/// 参数：
/// - `database_url`: 数据库连接字符串
/// - `database_type`: 数据库类型
///
/// 返回值：
/// 成功时返回封装好的数据库连接对象，失败时返回 ConnectionError 错误
pub fn establish_connection(
  database_url: &str,
  database_type: &str,
) -> Result<AnyConnection, ConnectionError> {
  // 根据数据库类型建立相应的连接
  match database_type {
    #[cfg(feature = "sqlite")]
    "sqlite" => {
      let conn = diesel::SqliteConnection::establish(database_url).map_err(|e| {
        ConnectionError::with_source(
          &format!("Error connecting to SQLite database at {}", database_url),
          Box::new(e),
        )
      })?;

      let mut any_conn = AnyConnection::Sqlite(conn);

      // 显式开启外键约束（SQLite 默认关闭）
      diesel::sql_query("PRAGMA foreign_keys = ON").execute(&mut any_conn).map_err(|e| {
        ConnectionError::with_source("Error executing PRAGMA foreign_keys = ON", Box::new(e))
      })?;

      Ok(any_conn)
    }

    #[cfg(feature = "mysql")]
    "mysql" => {
      let conn = diesel::MysqlConnection::establish(database_url).map_err(|e| {
        ConnectionError::with_source(
          &format!("Error connecting to MySQL database at {}", database_url),
          Box::new(e),
        )
      })?;

      let mut any_conn = AnyConnection::Mysql(conn);

      Ok(any_conn)
    }

    #[cfg(feature = "postgres")]
    "postgres" => {
      let conn = diesel::PgConnection::establish(database_url).map_err(|e| {
        ConnectionError::with_source(
          &format!("Error connecting to PostgreSQL database at {}", database_url),
          Box::new(e),
        )
      })?;

      let mut any_conn = AnyConnection::Postgresql(conn);

      Ok(any_conn)
    }

    // 不支持的数据库类型
    _ => Err(ConnectionError::new(&format!(
      "Unsupported database type: {} or feature not enabled",
      database_type
    ))),
  }
}

/// 运行所有待处理的数据库迁移
///
/// 该函数会查找并运行所有尚未应用的数据库迁移脚本。
/// 迁移脚本位于项目中的 migrations 目录下。
///
/// 参数：
/// - `conn`: 数据库连接对象
/// - `database_type`: 数据库类型 ("sqlite", "mysql", "postgres")
///
/// 返回值：
/// 成功时返回迁移版本列表，失败时返回错误信息
pub fn run_pending_migrations<'a>(
  conn: &'a mut AnyConnection,
  database_type: &'a str,
) -> Result<Vec<diesel::migration::MigrationVersion<'a>>, Box<dyn std::error::Error + Send + Sync>>
{
  use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

  // 根据数据库类型运行对应迁移
  let migrations = match database_type {
    "sqlite" => {
      const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations_sqlite");
      MIGRATIONS
    }
    "mysql" => {
      const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations_mysql");
      MIGRATIONS
    }
    "postgres" => {
      const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations_postgres");
      MIGRATIONS
    }
    _ => {
      const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");
      MIGRATIONS
    }
  };

  conn.run_pending_migrations(migrations).map_err(|e| e.into())
}
