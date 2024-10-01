use chrono::Utc;
use rusqlite::{params, Connection, Result};

#[allow(dead_code)]
pub struct File {
    pub id: i64,
    pub file_type: String,
    pub path: String,
}

#[allow(dead_code)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub is_primary: bool,
    pub create_time: String,
    pub modify_time: String,
}

#[allow(dead_code)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[allow(dead_code)]
pub struct GroupFile {
    pub file_id: i64,
    pub group_id: i64,
}

#[allow(dead_code)]
pub struct GroupTag {
    pub group_id: i64,
    pub tag_id: i64,
}

#[allow(dead_code)]
pub fn get_last_insert_rowid(conn: &Connection) -> Result<i64> {
    let row_id = conn.last_insert_rowid();
    Ok(row_id)
}

#[allow(dead_code)]
pub fn upload_file(conn: &Connection, file_type: &str, path: &str) -> Result<Option<File>> {
    conn.execute(
        "INSERT INTO files (type, path) VALUES (?1, ?2)",
        params![file_type, path],
    )?;
    let file_id = get_last_insert_rowid(&conn)?;
    let file = File {
        id: file_id,
        file_type: file_type.to_string(),
        path: path.to_string(),
    };
    Ok(Some(file))
}

#[allow(dead_code)]
pub fn associate_file_with_group(conn: &Connection, file_id: i64, group_id: i64) -> Result<()> {
    conn.execute(
        "INSERT INTO file_groups (file_id, group_id) VALUES (?1, ?2)",
        params![file_id, group_id],
    )?;
    Ok(())
}

#[allow(dead_code)]
pub fn associate_tags_with_group(conn: &Connection, group_id: i64, tag_ids: &[i64]) -> Result<()> {
    for tag_id in tag_ids {
        conn.execute(
            "INSERT INTO group_tags (group_id, tag_id) VALUES (?1, ?2)",
            params![group_id, tag_id],
        )?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn search_files_by_tag_name(
    conn: &Connection,
    tag_name: &str,
) -> Result<Vec<File>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT f.id AS file_id, f.type AS file_type, f.path AS file_path
         FROM files f
         JOIN file_groups fg ON f.id = fg.file_id
         JOIN group_tags gt ON fg.group_id = gt.group_id
         JOIN tags t ON gt.tag_id = t.id
         WHERE t.name = ?1",
    )?;
    let file_iter = stmt.query_map(params![tag_name], |row| {
        Ok(File {
            id: row.get(0)?,
            file_type: row.get(1)?,
            path: row.get(2)?,
        })
    })?;

    let mut files = Vec::new();
    for file in file_iter {
        files.push(file?);
    }

    Ok(files)
}

#[allow(dead_code)]
pub fn search_files_by_tag_id(
    conn: &Connection,
    tag_id: i64,
) -> Result<Vec<File>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT f.id AS file_id, f.type AS file_type, f.path AS file_path
         FROM files f
         JOIN file_groups fg ON f.id = fg.file_id
         JOIN group_tags gt ON fg.group_id = gt.group_id
         WHERE gt.tag_id = ?1",
    )?;
    let file_iter = stmt.query_map(params![tag_id], |row| {
        Ok(File {
            id: row.get(0)?,
            file_type: row.get(1)?,
            path: row.get(2)?,
        })
    })?;

    let mut files = Vec::new();
    for file in file_iter {
        files.push(file?);
    }

    Ok(files)
}

#[allow(dead_code)]
pub fn search_files_by_group_name(
    conn: &Connection,
    group_name: &str,
) -> Result<Vec<File>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT f.id AS file_id, f.type AS file_type, f.path AS file_path
         FROM files f
         JOIN file_groups fg ON f.id = fg.file_id
         JOIN groups g ON fg.group_id = g.id
         WHERE g.name = ?1",
    )?;
    let file_iter = stmt.query_map(params![group_name], |row| {
        Ok(File {
            id: row.get("file_id")?,
            file_type: row.get("file_type")?,
            path: row.get("file_path")?,
        })
    })?;

    let mut files = Vec::new();
    for file in file_iter {
        files.push(file?);
    }

    Ok(files)
}

#[allow(dead_code)]
pub fn get_tag_id(conn: &Connection, tag_name: &str) -> Result<Option<i64>> {
    let mut stmt = conn.prepare("SELECT id FROM tags WHERE name = ?1")?;
    let tag_id = stmt.query_map(params![tag_name], |row| row.get(0))?.next();

    match tag_id {
        Some(Ok(tag_id)) => Ok(Some(tag_id)),
        _ => Ok(None),
    }
}

#[allow(dead_code)]
pub fn update_tag_name(conn: &Connection, tag_id: i64, new_name: &str) -> Result<()> {
    conn.execute(
        "UPDATE tags SET name = ?1 WHERE id = ?2",
        params![new_name, tag_id],
    )?;
    Ok(())
}

#[allow(dead_code)]
pub fn search_tag(conn: &Connection, tag_name: &str) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare("SELECT id AS tag_id FROM tags WHERE name = ?1")?;
    let tag_iter = stmt.query_map(params![tag_name], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: tag_name.to_string(),
        })
    })?;

    let mut tags = Vec::new();
    for tag in tag_iter {
        tags.push(tag?);
    }
    Ok(tags)
}

#[allow(dead_code)]
pub fn search_group_by_name(
    conn: &Connection,
    group_name: &str,
) -> Result<Vec<Group>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name, is_primary, create_time, modify_time FROM groups WHERE name = ?1",
    )?;
    let group_iter = stmt.query_map(params![group_name], |row| {
        Ok(Group {
            id: row.get(0)?,
            name: row.get(1)?,
            is_primary: row.get(2)?,
            create_time: row.get(3)?,
            modify_time: row.get(4)?,
        })
    })?;

    let mut groups = Vec::new();
    for group in group_iter {
        groups.push(group?);
    }

    Ok(groups)
}

#[allow(dead_code)]
pub fn create_group(conn: &Connection, name: &str, is_primary: bool) -> Result<Option<Group>> {
    conn.execute(
        "INSERT INTO groups (name, is_primary, create_time, modify_time) VALUES (?1, ?2, datetime('now'), datetime('now'))",
        params![name, is_primary],
    )?;
    let group_id = get_last_insert_rowid(&conn)?;
    let group = Group {
        id: group_id,
        name: name.to_string(),
        is_primary,
        create_time: Utc::now().to_string(),
        modify_time: Utc::now().to_string(),
    };
    Ok(Some(group))
}

#[allow(dead_code)]
pub fn create_tags(conn: &Connection, names: Vec<&str>) -> Result<Vec<Tag>> {
    let mut tags = Vec::new();
    for name in names {
        conn.execute("INSERT INTO tags (name) VALUES (?1)", params![name])?;
        let tag_id = get_last_insert_rowid(&conn)?;
        let tag = Tag {
            id: tag_id,
            name: name.to_string(),
        };
        tags.push(tag);
    }
    Ok(tags)
}
