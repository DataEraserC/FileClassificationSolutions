use file_classification_core::service::files::select_files_by_conditions;
use file_classification_core::utils::database::establish_connection;
use file_classification_core::model::models::{File, FileCondition, FileFilter};
// 引入select_files和SearchFile

fn main() {
    let connection = &mut establish_connection();

    // 定义一个空的 SearchFile 来进行无条件查询
    let mut fileConditions = Vec::<FileCondition>::new();
    fileConditions.push(FileCondition::TypeLike("file%type".to_string()));
    fileConditions.push(FileCondition::Or(vec![
        FileCondition::PathLike("file%path".to_string()),
        FileCondition::PathLike("fp".to_string()),
        FileCondition::Path("filePath".to_string()),
    ]));
    fileConditions.push(FileCondition::And(vec![
        FileCondition::IdGreaterThan(0),
        FileCondition::IdLessThan(100),
        FileCondition::Or(vec![
            FileCondition::Not(Box::new(
                FileCondition::TypeLike("p".to_string()),
            )),
        ]),
    ]));

    // 使用 select_files 函数进行查询
    let results = select_files_by_conditions(connection, fileConditions, 5).expect("Error loading files");

    println!("Displaying {} files", results.len());
    for file in results {
        println!("File<{:?}>\n", file);
    }
}
