pub mod database;
pub mod models;

// 引入嵌入资源相关依赖
pub mod embedded {
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "static/"]
    pub struct Assets;
}