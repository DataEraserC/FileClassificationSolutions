use file_classification_core::service::groups::create_group;
use file_classification_core::utils::database::establish_connection;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();

    println!("Please input Group Name:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    let group = create_group(connection, name);
    match group {
        Ok(group) => {
            println!("Group created successfully!(Group<{:?}>)", group);
        }
        Err(e) => {
            println!("\nError creating group: {}", e);
        }
    }
}

#[allow(dead_code)]
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[allow(dead_code)]
#[cfg(windows)]
const EOF: &str = "CTRL+Z";
