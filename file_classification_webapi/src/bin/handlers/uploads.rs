use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use std::io::Write;
use std::path::Path;
use sanitize_filename;
use crate::utils::app_config::AppConfig;

/// 处理文件上传
///
/// 这是一个简单的文件上传接口，与数据库无关，只负责接收文件并返回访问链接
///
/// 请求路径: POST /api/uploads
#[post("/api/uploads")]
pub async fn upload_file(
    mut payload: Multipart,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // 获取第一个字段
    if let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Multipart error: {}", e))
        })?;

        // 先获取文件名并生成安全的文件名，在此之后不再使用field的content_disposition
        let safe_filename = {
            let temp_filename = if let Some(content_disposition) = field.content_disposition() {
                content_disposition.get_filename().unwrap_or("uploaded_file").to_owned()
            } else {
                "uploaded_file".to_owned()
            };
            
            // 生成安全的文件名
            sanitize_filename::sanitize(&temp_filename).into_owned()
        };
        
        let filepath = format!("{}/{}", app_config.upload_path, safe_filename);

        // 确保上传目录存在
        std::fs::create_dir_all(&app_config.upload_path).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to create upload directory: {}", e))
        })?;

        // 创建文件
        let mut file = std::fs::File::create(&filepath).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to create file: {}", e))
        })?;

        // 写入文件内容
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Failed to read chunk: {}", e))
            })?;
            file.write_all(&data).map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Failed to write file: {}", e))
            })?;
        }

        // 返回文件访问链接
        let file_url = format!("/api/uploads/{}", safe_filename);
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "url": file_url,
            "filename": safe_filename
        })))
    } else {
        Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No file provided"
        })))
    }
}

/// 处理文件下载
///
/// 根据文件名下载已上传的文件
///
/// 请求路径: GET /api/uploads/{filename}
#[get("/api/uploads/{filename}")]
pub async fn download_file(
    path: web::Path<String>,
    req: HttpRequest,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    let filename = path.into_inner();
    
    // 确保文件名是安全的
    let safe_filename = sanitize_filename::sanitize(&filename);
    
    // 构建文件路径
    let filepath = format!("{}/{}", app_config.upload_path, safe_filename);
    
    // 检查文件是否存在
    if !Path::new(&filepath).exists() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "File not found"
        })));
    }
    
    // 使用 NamedFile 提供文件下载
    let file = NamedFile::open(&filepath)?;
    Ok(file.into_response(&req))
}