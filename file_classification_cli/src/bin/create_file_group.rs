use file_classification_core::create_file_group;
use file_classification_core::establish_connection;
use std::io::{stdin, stdout, Write};

fn main() {
	let connection = &mut establish_connection();

	let mut file_id_input = String::new();
	let mut group_id_input = String::new();

	print!("Please input File ID: ");
	stdout().flush().unwrap();
	stdin().read_line(&mut file_id_input).unwrap();
	let file_id: i32 = file_id_input.trim().parse().expect("Invalid File ID");

	print!("Please input Group ID: ");
	stdout().flush().unwrap();
	stdin().read_line(&mut group_id_input).unwrap();
	let group_id: i32 = group_id_input.trim().parse().expect("Invalid Group ID");

	let result = create_file_group(connection, file_id, group_id);
	if result.is_ok() {
		println!("\nFile ID {} added to Group ID {}", file_id, group_id);
	} else {
		println!("\nFailed to add File ID {} to Group ID {}", file_id, group_id);
	}
}

#[allow(dead_code)]
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[allow(dead_code)]
#[cfg(windows)]
const EOF: &str = "CTRL+Z";
