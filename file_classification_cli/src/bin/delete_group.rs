use file_classification_core::delete_group;
use file_classification_core::establish_connection;
use std::io;
fn main() {
	let connection = &mut establish_connection();

	println!("Please input Group ID:");
	let mut group_id = String::new();
	io::stdin().read_line(&mut group_id).expect("Failed to read line");
	let group_id: i32 = group_id.trim().parse().expect("Please type a number!");
	println!("Deleting group with ID: {}", group_id);
	delete_group(connection, group_id).expect("Error deleting group");
}
