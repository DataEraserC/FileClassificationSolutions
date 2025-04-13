use file_classification_core::tags::create_tag;
use file_classification_core::database::establish_connection;
use std::io::stdin;

fn main() {
	let connection = &mut establish_connection();

	let mut name = String::new();

	println!("Please input Tag Name:");
	stdin().read_line(&mut name).unwrap();
	let name = name.trim_end();

	let tag = create_tag(connection, name);
	match tag {
		Ok(tag) => {
			println!("Tag created successfully!(Tag<{:?}>)", tag);
		}
		Err(e) => {
			println!("\nError creating tag: {}", e);
		}
	}
}

#[allow(dead_code)]
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[allow(dead_code)]
#[cfg(windows)]
const EOF: &str = "CTRL+Z";
