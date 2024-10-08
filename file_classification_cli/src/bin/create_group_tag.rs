use file_classification_core::create_group_tag;
use file_classification_core::establish_connection;
use std::io::{stdin, stdout, Write};

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
    if result.is_ok() {
        println!("\nTag ID {} added to Group ID {}", tag_id, group_id);
    } else {
        println!("\nFailed to add Tag ID {} to Group ID {}", tag_id, group_id);
    }
}

#[allow(dead_code)]
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[allow(dead_code)]
#[cfg(windows)]
const EOF: &str = "CTRL+Z";
