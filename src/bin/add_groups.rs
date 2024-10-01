use file_classification::establish_connection;
use file_classification::create_group;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();

    println!("Please input Group Name:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    let group = create_group(connection, name);
    if let Some(group_id) = group.id {
        println!("\nSaved group {name} with id {}", group_id);
    }
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
