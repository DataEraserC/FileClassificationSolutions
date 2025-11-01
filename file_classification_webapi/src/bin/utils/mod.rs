pub mod database;
pub mod models;
pub mod logger;
pub mod cors;
pub mod static_files;
pub mod app_config;

pub mod server;

// 引入嵌入资源相关依赖
pub mod embedded {
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "static/"]
    pub struct Assets;
}