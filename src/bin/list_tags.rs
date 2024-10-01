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
        let mut output = format!("ID: {}", tag.id.unwrap_or_default());

        output.push_str(&format!(", Name: '{}'", tag.name));

        if let Some(tag_reference_count) = tag.reference_count {
            output.push_str(&format!(", ReferenceCount: {}", tag_reference_count));
        }

        println!("{}", output);
    }
}
