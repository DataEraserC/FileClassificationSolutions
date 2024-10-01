use file_classification::create_tag;
use file_classification::establish_connection;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();

    println!("Please input Tag Name:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    let tag = create_tag(connection, name);
    if let Some(group_id) = tag.id {
        println!("\nSaved tag {name} with id {}", group_id);
    }
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
