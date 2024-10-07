use file_classification::establish_connection;
use file_classification::{select_tags, models::SearchTag};  // 引入select_tags和SearchTag

fn main() {
    let connection = &mut establish_connection();
    
    // 定义一个空的 SearchTag 来进行无条件查询
    let search_input = SearchTag {
        id: None,
        name: None,
        reference_count: None,
    };

    // 使用 select_tags 函数进行查询
    let results = select_tags(connection, search_input, 5)
        .expect("Error loading tags");

    println!("Displaying {} tags", results.len());
    for tag in results {
        let mut output = format!("ID: {}", tag.id);

        output.push_str(&format!(", Name: '{}'", tag.name));

        output.push_str(&format!(", ReferenceCount: {}", tag.reference_count));

        println!("{}", output);
    }
}
