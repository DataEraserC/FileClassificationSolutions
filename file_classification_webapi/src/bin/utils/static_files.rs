use actix_web::HttpResponse;
use rust_embed::RustEmbed;
use std::path::Path;
use std::path::PathBuf;
use log;

// 嵌入静态资源
#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

// 处理嵌入的文件资源
fn handle_embedded_file(path: &str) -> HttpResponse {
    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime.as_ref())
                .body(content.data)
        }
        None => {
            // Avoid recursive call by directly getting index.html
            if path == "index.html" {
                return HttpResponse::NotFound().body("404 Not Found");
            }
            handle_embedded_file("index.html")
        }
    }
}

/// 查找静态文件目录，支持从不同目录运行程序
pub fn find_static_directory() -> std::io::Result<PathBuf> {
    // 首先尝试从可执行文件所在目录查找
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let static_dir = exe_dir.join("static");
            if Path::new(&static_dir).exists() {
                log::info!("找到静态资源目录: {:?}", static_dir);
                return Ok(static_dir);
            }
        }
    }

    // 然后尝试从当前工作目录查找
    if let Ok(current_dir) = std::env::current_dir() {
        let static_dir = current_dir.join("static");
        if Path::new(&static_dir).exists() {
            log::info!("找到静态资源目录: {:?}", static_dir);
            return Ok(static_dir);
        }

        // 尝试从当前工作目录的子目录 file_classification_webapi 中查找
        let static_dir = current_dir.join("file_classification_webapi").join("static");
        if Path::new(&static_dir).exists() {
            log::info!("找到静态资源目录: {:?}", static_dir);
            return Ok(static_dir);
        }
    }

    // 如果都没找到，则返回默认路径并让后续逻辑处理错误
    if let Ok(current_dir) = std::env::current_dir() {
        let static_dir = current_dir.join("static");
        log::warn!("静态文件目录不存在: {:?}", static_dir);
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("静态文件目录不存在: {:?}", static_dir)))
    } else {
        log::error!("无法确定静态文件目录位置");
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "无法确定静态文件目录位置"))
    }
}

pub async fn index_handler() -> HttpResponse {
    // 首先尝试从物理目录提供文件
    match find_static_directory() {
        Ok(static_dir) => {
            let index_path = static_dir.join("index.html");
            if index_path.exists() {
                log::info!("从物理目录提供 index.html 文件: {:?}", index_path);
                return HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(std::fs::read(index_path).unwrap_or_else(|_| Vec::new()));
            }
        }
        Err(_) => {
            // 物理目录不存在，回退到嵌入资源
            log::info!("物理目录中未找到 index.html，回退到嵌入资源");
        }
    }

    // 回退到嵌入资源
    log::info!("从嵌入资源提供 index.html 文件");
    handle_embedded_file("index.html")
}

pub async fn static_handler(path: actix_web::web::Path<String>) -> HttpResponse {
    let path = path.into_inner();

    // 首先尝试从物理目录提供文件
    match find_static_directory() {
        Ok(static_dir) => {
            let file_path = static_dir.join(&path);
            log::debug!("尝试从物理目录提供文件: {:?}, 请求路径: {}", file_path, path);

            if file_path.exists() && file_path.is_file() {
                // 确保请求的文件在 static 目录内，防止路径遍历攻击
                if let Ok(abs_file_path) = file_path.canonicalize() {
                    if abs_file_path.starts_with(&static_dir.canonicalize().unwrap_or(static_dir.clone())) {
                        log::info!("从物理目录提供文件: {:?}", file_path);
                        let content = std::fs::read(&file_path);
                        match content {
                            Ok(data) => {
                                let mime = mime_guess::from_path(&path).first_or_octet_stream();
                                return HttpResponse::Ok()
                                    .content_type(mime.as_ref())
                                    .body(data);
                            }
                            Err(e) => {
                                log::error!("读取文件失败 {:?}: {}", file_path, e);
                            }
                        }
                    } else {
                        log::warn!("文件路径不在静态目录内: {:?}", file_path);
                    }
                }
            } else {
                log::debug!("文件不存在或不是文件: {:?}", file_path);
            }
        }
        Err(e) => {
            // 物理目录不存在，回退到嵌入资源
            log::info!("查找物理目录失败: {}，回退到嵌入资源，请求路径: {}", e, path);
        }
    }

    // 回退到嵌入资源
    log::info!("从嵌入资源提供文件: {}", path);
    handle_embedded_file(&path)
}