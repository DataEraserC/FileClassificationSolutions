use file_classification_core::establish_connection;
use file_classification_core::{models::SearchGroup, select_groups}; // 引入select_groups和SearchGroup

fn main() {
	let connection = &mut establish_connection();

	// 定义一个空的 SearchGroup 来进行无条件查询
	let search_input = SearchGroup {
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
		let mut output = format!("ID: {}", group.id);

		output.push_str(&format!(", Name: '{}'", group.name));

		output.push_str(&format!(", ReferenceCount: {}", group.reference_count));
		output.push_str(&format!(", IsPrimary: '{}'", group.is_primary));

		output.push_str(&format!(", ClickCount: {}", group.click_count));
		output.push_str(&format!(", ShareCount: {}", group.share_count));
		output.push_str(&format!(", CreateTime: '{}'", group.create_time));

		output.push_str(&format!(", ModifyTime: '{}'", group.modify_time));

		println!("{}", output);
	}
}
