// database.rs
//! 数据库连接管理模块
//!
//! 提供数据库连接的建立和管理功能，支持多种数据库类型（当前仅支持 SQLite），
//! 并封装了通用的数据库连接类型。

pub use diesel::{Connection, QueryResult};
use dotenvy::dotenv;
use std::env;

/// 通用数据库连接枚举
///
/// 使用 `diesel::MultiConnection` 宏定义的枚举类型，支持多种数据库连接类型。
/// 当前仅支持 SQLite 连接，但预留了扩展其他数据库类型的接口。
#[derive(diesel::MultiConnection)]
pub enum AnyConnection {
    // Postgresql(diesel::PgConnection),
    // Mysql(diesel::MysqlConnection),
    /// SQLite 数据库连接
    Sqlite(diesel::SqliteConnection),
}

/// 建立数据库连接
///
/// 从环境变量中读取数据库配置信息，并根据配置建立相应的数据库连接。
///
/// 环境变量要求：
/// - `DATABASE_URL`: 数据库连接字符串
/// - `DATABASE_TYPE`: 数据库类型（当前仅支持 "sqlite"）
///
/// 返回值：
/// 成功时返回封装好的数据库连接对象，失败时会 panic 并输出错误信息
pub fn establish_connection() -> AnyConnection {
    // 加载 .env 文件中的环境变量
    dotenv().ok();

    // 从环境变量中获取数据库连接URL和数据库类型
    // NOTE: from ./.env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_type = env::var("DATABASE_TYPE").expect("DATABASE_TYPE must be set");

    // 根据数据库类型建立相应的连接
    match database_type.as_str() {
        "sqlite" => AnyConnection::Sqlite(
            diesel::SqliteConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
        ),
        // 不支持的数据库类型直接 panic
        _ => panic!("Unsupported database type: {}", database_type),
    }
}

/// 运行所有待处理的数据库迁移
///
/// 该函数会查找并运行所有尚未应用的数据库迁移脚本。
/// 迁移脚本位于项目中的 migrations 目录下。
///
/// 参数：
/// - `conn`: 数据库连接对象
///
/// 返回值：
/// 成功时返回迁移版本列表，失败时返回错误信息
pub fn run_pending_migrations(
    conn: &mut AnyConnection,
) -> Result<Vec<diesel::migration::MigrationVersion>, Box<dyn std::error::Error + Send + Sync>> {
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(applied_migrations) => Ok(applied_migrations),
        Err(e) => Err(e),
    }
}
