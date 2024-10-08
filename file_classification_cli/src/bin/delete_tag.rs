use file_classification_core::delete_tag;
use file_classification_core::establish_connection;
use std::io;
fn main() {
	let connection = &mut establish_connection();

	println!("Please input Tag ID:");
	let mut tag_id = String::new();
	io::stdin().read_line(&mut tag_id).expect("Failed to read line");
	let tag_id: i32 = tag_id.trim().parse().expect("Please type a number!");
	println!("Deleting tag with ID: {}", tag_id);
	delete_tag(connection, tag_id).expect("Error deleting tag");
}
