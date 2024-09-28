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
    Ok(())
}
