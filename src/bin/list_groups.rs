use diesel::prelude::*;
use file_classification::models::*;
use file_classification::schema::groups::dsl::*;
use file_classification::establish_connection;

fn main() {
    let connection = &mut establish_connection();
    let results = groups
        .limit(5)
        .select(Group::as_select())
        .load(connection)
        .expect("Error loading groups");

    println!("Displaying {} groups", results.len());
    for group in results {
        let mut output = format!("ID: {}", group.id.unwrap_or_default());

        output.push_str(&format!(", Name: '{}'", group.name));

        output.push_str(&format!(", IsPrimary: '{}'", group.is_primary));

        if let Some(group_click_count) = group.click_count {
            output.push_str(&format!(", ClickCount: {}", group_click_count));
        }
        if let Some(group_share_count) = group.share_count {
            output.push_str(&format!(", ShareCount: {}", group_share_count));
        }
        output.push_str(&format!(", CreateTime: '{}'", group.create_time));

        output.push_str(&format!(", ModifyTime: '{}'", group.modify_time));

        println!("{}", output);
    }
}
