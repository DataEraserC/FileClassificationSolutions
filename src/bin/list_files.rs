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
        let mut output = format!("ID: {}", file.id.unwrap_or_default());

        output.push_str(&format!(", Type: '{}'", file.type_));

        output.push_str(&format!(", Path: '{}'", file.path));

        if let Some(file_reference_count) = file.reference_count {
            output.push_str(&format!(", ReferenceCount: {}", file_reference_count));
        }

        if let Some(file_group_id) = file.group_id {
            output.push_str(&format!(", GroupID: {}", file_group_id));
        }

        println!("{}", output);
    }
}
