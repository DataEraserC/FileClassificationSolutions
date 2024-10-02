use file_classification::create_file;
use file_classification::establish_connection;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();
    let mut type_ = String::new();
    let mut path = String::new();

    println!("Please input File Name:");
    stdin().read_line(&mut type_).unwrap();
    let type_ = type_.trim_end();

    println!("Please input File Type:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    println!("Please input File Path:");
    stdin().read_line(&mut path).unwrap();
    let path = path.trim_end();

    let result = create_file(connection, name, path, &type_);
    match result {
        Ok((file, group)) => {
            if let Some(file_id) = file.id {
                println!("\nSaved file {path} with id {}", file_id);
            }
            if let Some(group_id) = group.id {
                println!("Saved group {type_} with id {}", group_id);
            }
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e);
        }
    }
}

#[allow(dead_code)]
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[allow(dead_code)]
#[cfg(windows)]
const EOF: &str = "CTRL+Z";
