use super::database::SqliteConnection;
use crate::service::file_group::create_file_group;
use crate::service::groups::create_group;
use crate::service::AppError;
use crate::{
    internal::{
        files,
        groups::{find_group_by_name, mark_group_as_primary},
    },
    model::models::{CreateFileDTO, File, FileFilter, Group},
};
use crate::model::models::FileCondition;

pub fn raw_create_file(
    conn: &mut SqliteConnection,
    type_: &str,
    path_: &str,
    group_id: i32,
) -> Result<File, AppError> {
    let new_file = CreateFileDTO { type_, path: path_, group_id };
    files::create_file(conn, &new_file)
}
pub fn create_file(
    conn: &mut SqliteConnection,
    name: &str,
    type_: &str,
    path_: &str,
) -> Result<(File, Group), diesel::result::Error> {
    let mut is_primary = false;
    let group = match find_group_by_name(conn, name)? {
        Some(existing_group) => existing_group,
        _ => {
            let group = create_group(conn, name)?;
            is_primary = true;
            group
        }
    };

    let file = raw_create_file(conn, type_, path_, group.id)?;

    create_file_group(conn, file.id, group.id)?;
    if is_primary {
        mark_group_as_primary(conn, group.id)?;
    }

    Ok((file, group))
}
pub fn delete_file(conn: &mut SqliteConnection, file_id: i32) -> Result<(), diesel::result::Error> {
    files::delete_file(conn, file_id)
}
pub fn select_files(
    conn: &mut SqliteConnection,
    search_input: FileFilter,
    limit: i64,
) -> Result<Vec<File>, diesel::result::Error> {
    files::select_files(conn, search_input, limit)
}

pub fn select_files_by_conditions(
    conn: &mut SqliteConnection,
    condition: Vec<FileCondition>,
    limit: i64,
) -> Result<Vec<File>, diesel::result::Error> {
    files::select_files_by_conditions(conn, condition, limit)
}

// pub fn update_file(
// 	conn: &mut SqliteConnection,
// 	update_input: FileFilter,
// ) -> Result<File, diesel::result::Error> {
// 	files::update_file(conn, update_input)
// }
