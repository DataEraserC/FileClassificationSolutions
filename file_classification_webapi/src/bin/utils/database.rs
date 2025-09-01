use once_cell::sync::Lazy as SyncLazy;
use std::sync::Arc;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use once_cell::unsync::Lazy as UnsyncLazy;
use once_cell::unsync::Lazy;

// 创建连接池类型别名
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

// pub static DB_POOL: Lazy<Arc<Pool>> = Lazy::new(|| {
//     let manager = ConnectionManager::<SqliteConnection>::new("data.db");
//     let pool = r2d2::Pool::builder()
//         .max_size(10)
//         .build(manager)
//         .expect("无法创建连接池");
//
//     Arc::new(pool)
// });
//
// // 获取连接的辅助函数
// pub fn get_connection() -> Result<PooledConnection, r2d2::Error> {
//     DB_POOL.get()
// }

// 使用示例:
// let conn = get_connection().unwrap();
// do_something_with_connection(&conn);
