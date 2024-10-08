use file_classification_core::delete_file;
use file_classification_core::establish_connection;
use std::io;
fn main() {
    let connection = &mut establish_connection();

    println!("Please input File ID:");
    let mut file_id = String::new();
    io::stdin()
        .read_line(&mut file_id)
        .expect("Failed to read line");
    let file_id: i32 = file_id.trim().parse().expect("Please type a number!");
    println!("Deleting file with ID: {}", file_id);
    delete_file(connection, file_id).expect("Error deleting file");
}
