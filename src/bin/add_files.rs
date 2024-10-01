use file_classification::establish_connection;
use file_classification::create_file;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut type_ = String::new();
    let mut path = String::new();

    println!("Please input File Type:");
    stdin().read_line(&mut type_).unwrap();
    let type_ = type_.trim_end(); // Remove the trailing newline

    println!("Please input File Path:");
    stdin().read_line(&mut path).unwrap();
    let path = path.trim_end(); // Remove the trailing newline

    let file = create_file(connection, path, &type_);
    if let Some(file_id) = file.id{
        println!("\nSaved file {path} with id {}", file_id);

    }
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";