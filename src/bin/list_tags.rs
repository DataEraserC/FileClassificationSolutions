use diesel::prelude::*;
use file_classification::establish_connection;
use file_classification::models::*;
use file_classification::schema::tags::dsl::*;

fn main() {
    let connection = &mut establish_connection();
    let results = tags
        .limit(5)
        .select(Tag::as_select())
        .load(connection)
        .expect("Error loading tags");

    println!("Displaying {} tags", results.len());
    for tag in results {
        let mut output = format!("ID: {}", tag.id);

        output.push_str(&format!(", Name: '{}'", tag.name));

        output.push_str(&format!(", ReferenceCount: {}", tag.reference_count));

        println!("{}", output);
    }
}
