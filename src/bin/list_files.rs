use diesel::prelude::*;
use file_classification::establish_connection;
use file_classification::models::*;
use file_classification::schema::files::dsl::*;

fn main() {
    let connection = &mut establish_connection();
    let results = files
        .limit(5)
        .select(File::as_select())
        .load(connection)
        .expect("Error loading files");

    println!("Displaying {} files", results.len());
    for file in results {
        let mut output = format!("ID: {}", file.id);

        output.push_str(&format!(", Type: '{}'", file.type_));

        output.push_str(&format!(", Path: '{}'", file.path));

            output.push_str(&format!(", ReferenceCount: {}", file.reference_count));

            output.push_str(&format!(", GroupID: {}", file.group_id));

        println!("{}", output);
    }
}
