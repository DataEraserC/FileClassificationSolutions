use file_classification_core::establish_connection;
use file_classification_core::{select_files, models::SearchFile};  // 引入select_files和SearchFile

fn main() {
    let connection = &mut establish_connection();
    
    // 定义一个空的 SearchFile 来进行无条件查询
    let search_input = SearchFile {
        id: None,
        type_: None,
        path: None,
        reference_count: None,
        group_id: None,
    };
    
    // 使用 select_files 函数进行查询
    let results = select_files(connection, search_input, 5)
        .expect("Error loading files");

    println!("Displaying {} files", results.len());
    for file in results {
        let mut output = format!("ID: {}", file.id);

        output.push_str(&format!(", Type: '{}'", file.type_));

        output.push_str(&format!(", Path: '{}'", file.path));

        output.push_str(&format!(", ReferenceCount: {}", file.reference_count));

        output.push_str(&format!(", GroupID: {}", file.group_id));

        println!("{}", output);
    }
}
