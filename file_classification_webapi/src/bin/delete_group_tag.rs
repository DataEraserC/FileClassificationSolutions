use file_classification_core::service::group_tag::delete_group_tag;
use file_classification_core::utils::database::establish_connection;
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

    let result = delete_group_tag(connection, group_id, tag_id);
    match result {
        Ok(deleted_group_tag_num) => {
            if deleted_group_tag_num == 0 {
                println!("No GroupTag deleted!");
            } else {
                println!("GroupTag deleted successfully!({:?} line changed)", deleted_group_tag_num)
            }
        }
        Err(e) => eprintln!("Error deleting GroupTag: {}", e),
    }
}

#[allow(dead_code)]
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[allow(dead_code)]
#[cfg(windows)]
const EOF: &str = "CTRL+Z";
