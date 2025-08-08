use file_classification_core::model::models::GroupFilter;
#[allow(deprecated)]
use file_classification_core::service::groups::select_groups;
use file_classification_core::utils::database::establish_connection;
// 引入select_groups和SearchGroup

#[allow(deprecated)]
fn main() {
    let connection = &mut establish_connection();

    // 定义一个空的 SearchGroup 来进行无条件查询
    let search_input = GroupFilter {
        id: None,
        name: None,
        reference_count: None,
        is_primary: None,
        click_count: None,
        share_count: None,
        create_time: None,
        modify_time: None,
    };

    // 使用 select_groups 函数进行查询
    let results = select_groups(connection, search_input, 5).expect("Error loading groups");

    println!("Displaying {} groups", results.len());
    for group in results {
        println!("Group<{:?}>\n", group);
    }
}
