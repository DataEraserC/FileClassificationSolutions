use actix_web::{post, web, HttpResponse, Result};
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use std::io::Write;
use sanitize_filename;

/// 处理文件上传
///
/// 这是一个简单的文件上传接口，与数据库无关，只负责接收文件并返回访问链接
///
/// 请求路径: POST /api/uploads
#[post("/api/uploads")]
pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse> {
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
        
        let filepath = format!("uploads/{}", safe_filename);

        // 确保上传目录存在
        std::fs::create_dir_all("uploads").map_err(|e| {
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
        let file_url = format!("/{}", filepath.replace("\\", "/"));
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