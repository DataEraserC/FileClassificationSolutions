use file_classification_core::model::models::TagFilter;
use file_classification_core::service::tags::select_tags;
use file_classification_core::utils::database::establish_connection;
// 引入select_tags和SearchTag

fn main() {
    let connection = &mut establish_connection();

    // 定义一个空的 SearchTag 来进行无条件查询
    let search_input = TagFilter { id: None, name: None, reference_count: None };

    // 使用 select_tags 函数进行查询
    let results = select_tags(connection, search_input, 5).expect("Error loading tags");

    println!("Displaying {} tags", results.len());
    for tag in results {
        println!("Tag<{:?}>\n", tag);
    }
}
