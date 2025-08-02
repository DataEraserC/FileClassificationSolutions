use file_classification_core::service::files::select_files;
use file_classification_core::utils::database::establish_connection;
use file_classification_core::model::models::FileFilter;
// 引入select_files和SearchFile

fn main() {
    let connection = &mut establish_connection();

    // 定义一个空的 SearchFile 来进行无条件查询
    let search_input =
        FileFilter { id: None, type_: None, path: None, reference_count: None, group_id: None };

    // 使用 select_files 函数进行查询
    let results = select_files(connection, search_input, 5).expect("Error loading files");

    println!("Displaying {} files", results.len());
    for file in results {
        println!("File<{:?}>\n", file);
    }
}
