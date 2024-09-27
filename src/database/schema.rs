use rusqlite::{params, Connection, Result};
pub fn init(conn: &Connection) -> Result<()> {
    // Create tables if they don't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY,
            type TEXT NOT NULL,
            location TEXT NOT NULL
        )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS groups (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            is_primary BOOLEAN NOT NULL,
            create_time INTEGER NOT NULL,
            modify_time INTEGER NOT NULL
        )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS file_groups (
            file_id INTEGER,
            group_id INTEGER,
            PRIMARY KEY (file_id, group_id),
            FOREIGN KEY (file_id) REFERENCES files(id),
            FOREIGN KEY (group_id) REFERENCES groups(id)
        )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS group_tags (
            group_id INTEGER,
            tag_id INTEGER,
            PRIMARY KEY (group_id, tag_id),
            FOREIGN KEY (group_id) REFERENCES groups(id),
            FOREIGN KEY (tag_id) REFERENCES tags(id)
        )",
        params![],
    )?;

    // // Example usage
    // let group_id = create_group(&conn, "Q版青雀", true)?;
    // let file_id_a = upload_file(&conn, "jpg", "a.jpg")?;
    // let file_id_b = upload_file(&conn, "mp4", "a.mp4")?;

    // associate_file_with_group(&conn, file_id_a, group_id)?;
    // associate_file_with_group(&conn, file_id_b, group_id)?;

    // let tag_ids = vec![
    //     create_tag(&conn, "#青雀")?,
    //     create_tag(&conn, "#崩坏星穷铁道")?,
    //     create_tag(&conn, "#Q版")?,
    // ];

    // for tag_id in tag_ids {
    //     associate_tag_with_group(&conn, group_id, tag_id)?;
    // }

    // // Search for files by tag
    // search_files_by_tag(&conn, "#Q版")?;

    Ok(())
}
