use file_classification_core::database::establish_connection;
use file_classification_core::group_tag::create_group_tag;
use std::io::{Write, stdin, stdout};

fn main() {
	let connection = &mut establish_connection();

	let mut group_id_input = String::new();
	let mut tag_id_input = String::new();

	print!("Please input Group ID: ");
	stdout().flush().unwrap();
	stdin().read_line(&mut group_id_input).unwrap();
	let group_id: i32 = group_id_input.trim().parse().expect("Invalid Group ID");

	print!("Please input Tag ID: ");
	stdout().flush().unwrap();
	stdin().read_line(&mut tag_id_input).unwrap();
	let tag_id: i32 = tag_id_input.trim().parse().expect("Invalid Tag ID");

	let result = create_group_tag(connection, group_id, tag_id);
	match result {
		Ok(new_group_tag) => {
			println!("GroupTag created successfully!GroupTag<{:?}>", new_group_tag)
		}
		Err(e) => eprintln!("Error creating GroupTag: {}", e),
	}
}

#[allow(dead_code)]
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[allow(dead_code)]
#[cfg(windows)]
const EOF: &str = "CTRL+Z";
