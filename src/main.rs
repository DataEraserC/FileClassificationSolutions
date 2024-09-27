use crate::database::*;
use rusqlite::{Connection, Result};
use std::path::Path;
mod database; // 声明本地模块

fn main() -> Result<()> {
    let conn = match Connection::open(Path::new("sqlite.db")) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to open database: {}", e);
            return Err(e);
        }
    };

    if let Err(e) = database::schema::init(&conn) {
        eprintln!("Failed to initialize database schema: {}", e);
        // return Err(e);
    }
    test(conn);
    Ok(())
}
fn test(conn: Connection) -> Result<()> {
    
    // 1. 用户上传文件 {青雀.jpg, 青雀跳舞.mp4} -Q版青雀 #青雀 #崩坏星穷铁道 #Q版
    println!("1. 用户上传文件 {{青雀.jpg, 青雀跳舞.mp4}} -Q版青雀 #青雀 #崩坏星穷铁道 #Q版");

    let group = match database::queries::create_group(&conn, "Q版青雀", true) {
        Ok(group) => group,
        Err(e) => {
            eprintln!("Failed to create group: {}", e);
            None // return Err(e);
        }
    };

    let tags = match database::queries::create_tags(&conn, vec!["青雀", "崩坏星穷铁道", "Q版"])
    {
        Ok(tags) => tags,
        Err(e) => {
            eprintln!("Failed to create tags: {}", e);
            vec![] // return Err(e);
        }
    };

    let file_a = match database::queries::upload_file(&conn, "jpg", "path/to/青雀.jpg") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to upload file 青雀.jpg: {}", e);
            None // return Err(e);
        }
    };

    let file_b = match database::queries::upload_file(&conn, "mp4", "path/to/青雀跳舞.mp4") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to upload file 青雀跳舞.mp4: {}", e);
            None // return Err(e);
        }
    };
    if let Some(file_a) = file_a {
        if let Some(ref group) = group {
            if let Err(e) = database::queries::associate_file_with_group(&conn, file_a.id, group.id) {
            eprintln!("Failed to associate file 青雀.jpg with group: {}", e);
            () // return Err(e);
        }}
    }
    if let Some(file_b) = file_b {
        if let Some(ref group) = group {
        if let Err(e) = database::queries::associate_file_with_group(&conn, file_b.id, group.id) {
            eprintln!("Failed to associate file 青雀跳舞.mp4 with group: {}", e);
            // return Err(e);
        }
    }
}
    if let Some(group) = group {
        if let Err(e) = database::queries::associate_tags_with_group(
            &conn,
            group.id,
            &tags.iter().map(|t| t.id).collect::<Vec<_>>(),
        ) {
            eprintln!("Failed to associate tags with group: {}", e);
            // return Err(e);
        }
    }
    // 2. 用户上传文件 {青雀被符玄教训.jpg} -符玄教训青雀 #符玄
    println!("2. 用户上传文件 {{青雀被符玄教训.jpg}} -符玄教训青雀 #符玄");
    let group = match database::queries::create_group(&conn, "符玄教训青雀", false) {
        Ok(group) => group,
        Err(e) => {
            eprintln!("Failed to create group: {}", e);
            None // return Err(e);
        }
    };

    let tags = match database::queries::create_tags(&conn, vec!["符玄"]) {
        Ok(tags) => tags,
        Err(e) => {
            eprintln!("Failed to create tags: {}", e);
            vec![] // return Err(e);
        }
    };

    let file = match database::queries::upload_file(&conn, "jpg", "path/to/青雀被符玄教训.jpg")
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to upload file 青雀被符玄教训.jpg: {}", e);
            None // return Err(e);
        }
    };
    if let Some(file) = file {
        if let Some(ref group) = group {
            if let Err(e) = database::queries::associate_file_with_group(&conn, file.id, group.id) {
                eprintln!(
                    "Failed to associate file 青雀被符玄教训.jpg with group: {}",
                    e
                );
                // return Err(e);
            }
        }
    }
    if let Some(group) = group {
        if let Err(e) = database::queries::associate_tags_with_group(
            &conn,
            group.id,
            &tags.iter().map(|tag| tag.id).collect::<Vec<_>>(),
        ) {
            eprintln!("Failed to associate tags with group: {}", e);
            // return Err(e);
        }
    }
    // 3. 用户搜索文件 #符玄
    println!("3. 用户搜索文件 #符玄");
    if let Err(e) = database::queries::search_files_by_tag(&conn, "符玄") {
        eprintln!("Failed to search files by tag #符玄: {}", e);
        // return Err(e);
    }

    // 4. 用户选择文件组2
    println!("4. 用户选择文件组2");
    // This step is a UI operation, no SQL needed.

    // 5. 用户添加tag #崩坏星穹铁道
    println!("5. 用户添加tag #崩坏星穹铁道");
    let tags = match database::queries::create_tags(&conn, vec!["崩坏星穹铁道"]) {
        Ok(tags) => tags,
        Err(e) => {
            eprintln!("Failed to create tag #崩坏星穹铁道: {}", e);
            vec![] // return Err(e);
        }
    };
    if let Some(first_tag) = tags.first() {
        if let Err(e) = database::queries::associate_tags_with_group(&conn, 2, &[first_tag.id]) {
            eprintln!("Failed to associate tag #崩坏星穹铁道 with group: {}", e);
            // return Err(e);
        }
    } else {
        eprintln!("No tags were created.");
        return Err(rusqlite::Error::QueryReturnedNoRows); // Assuming this is the appropriate error.
    }

    // 6. 用户添加tag #青雀
    println!("6. 用户添加tag #青雀");
    let tag_id = match database::queries::get_tag_id(&conn, "青雀") {
        Ok(tag_id) => tag_id,
        Err(e) => {
            eprintln!("Failed to get tag ID for #青雀: {}", e);
            None // return Err(e);
        }
    };
    if let Some(tag_id) = tag_id {
        if let Err(e) = database::queries::associate_tags_with_group(&conn, 2, &vec![tag_id]) {
            eprintln!("Failed to associate tag #青雀 with group: {}", e);
            // return Err(e);
        }
    }

    // 7. 用户搜索文件-Q版青雀
    println!("7. 用户搜索文件-Q版青雀");
    if let Err(e) = database::queries::search_files_by_group_name(&conn, "Q版青雀") {
        eprintln!("Failed to search files by group name Q版青雀: {}", e);
        // return Err(e);
    }

    // 8. 用户选择文件组1内的tag#崩坏星穷铁道
    println!("8. 用户选择文件组1内的tag#崩坏星穷铁道");
    // This step is a UI operation, no SQL needed.

    // 9. 用户修改tag名为崩坏星穹铁道
    println!("9. 用户修改tag名为崩坏星穹铁道");
    let tag_id = match database::queries::get_tag_id(&conn, "崩坏星穹铁道") {
        Ok(tag_id) => tag_id,
        Err(e) => {
            eprintln!("Failed to get tag ID for 崩坏星穹铁道: {}", e);
            None // return Err(e);
        }
    };
    if let Some(tag_id) = tag_id {
        match database::queries::update_tag_name(&conn, tag_id, "崩坏星穹铁道") {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to update tag name for 崩坏星穹铁道: {}", e);
                // return Err(e);
            }
        }
    }

    // 10. 用户搜索tag#崩坏星穹铁道
    println!("10. 用户搜索tag#崩坏星穹铁道");
    if let Err(e) = database::queries::search_tag(&conn, "崩坏星穹铁道") {
        eprintln!("Failed to search tag 崩坏星穹铁道: {}", e);
        // return Err(e);
    }

    // 11. 用户选择tag#崩坏星穹铁道
    println!("11. 用户选择tag#崩坏星穹铁道");
    // This step is a UI operation, no SQL needed.

    // 12. 用户点击按tag搜索
    println!("12. 用户点击按tag搜索");
    let tag_id = match database::queries::get_tag_id(&conn, "崩坏星穹铁道") {
        Ok(tag_id) => tag_id,
        Err(e) => {
            eprintln!("Failed to get tag ID for 崩坏星穹铁道: {}", e);
            None // return Err(e);
        }
    };
    if let Some(tag_id) = tag_id {
        if let Err(e) = database::queries::search_files_by_tag_id(&conn, tag_id) {
            eprintln!("Failed to search files by tag ID: {}", e);
            // return Err(e);
        }
    }

    // 13. 用户手动按tag搜索#青雀
    println!("13. 用户手动按tag搜索#青雀");
    if let Err(e) = database::queries::search_files_by_tag(&conn, "青雀") {
        eprintln!("Failed to search files by tag 青雀: {}", e);
        // return Err(e);
    }

    // 14. 用户按精确文件组名搜索-符玄教训青雀
    println!("14. 用户按精确文件组名搜索-符玄教训青雀");
    match database::queries::search_files_by_group_name(&conn, "符玄教训青雀") {
        Ok(files) => {
            // 成功找到文件，输出文件信息
            for file in files {
                println!(
                    "File ID: {}, Name: {}, Group Name: {}",
                    file.id, file.file_type, file.location
                );
            }
        }
        Err(e) => {
            // 处理错误
            eprintln!("An error occurred: {}", e);
            // return Err(e);
        }
    }
    Ok(())
}