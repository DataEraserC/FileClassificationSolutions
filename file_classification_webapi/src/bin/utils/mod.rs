pub mod app_config;
pub mod cors;
pub mod database;
pub mod logger;
pub mod models;
pub mod static_files;

pub mod server;

// 引入嵌入资源相关依赖
pub mod embedded {
  use rust_embed::RustEmbed;

  #[derive(RustEmbed)]
  #[folder = "static/"]
  pub struct Assets;
}
