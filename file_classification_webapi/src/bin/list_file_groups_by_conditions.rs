use file_classification_core::model::models::FileGroupCondition;
use file_classification_core::utils::database::establish_connection;
use std::io::{self, Write};
use file_classification_core::service::file_group::select_file_groups_by_conditions;

fn main() {
    let connection = &mut establish_connection();

    println!("文件组关联查询");
    println!("==============");

    loop {
        let conditions = get_user_conditions();

        if conditions.is_empty() {
            println!("未输入任何条件");
            // break;
        }

        match select_file_groups_by_conditions(connection, conditions, 20) {
            Ok(file_groups) => {
                println!("\n查询结果 (共 {} 条记录):", file_groups.len());
                println!("-------------------------");
                if file_groups.is_empty() {
                    println!("没有找到匹配的文件组关联。");
                } else {
                    for file_group_dto in file_groups {
                        println!("文件ID: {}, 组ID: {}", file_group_dto.file_id, file_group_dto.group_id);
                    }
                }
            }
            Err(e) => {
                eprintln!("查询出错: {}", e);
            }
        }

        println!("\n是否继续查询？(y/n): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        if input.trim().to_lowercase() != "y" {
            break;
        }
        println!("\n");
    }

    println!("感谢使用文件组关联查询！");
}

fn get_user_conditions() -> Vec<FileGroupCondition> {
    let mut conditions = Vec::new();

    println!("请输入查询条件（留空结束输入）:");

    loop {
        println!("\n可选条件类型:");
        println!("1. 文件ID等于");
        println!("2. 文件ID大于");
        println!("3. 文件ID小于");
        println!("4. 组ID等于");
        println!("5. 组ID大于");
        println!("6. 组ID小于");
        println!("7. AND组合条件");
        println!("8. OR组合条件");
        println!("9. NOT条件");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-9): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入文件ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileId(value));
                }
            }
            2 => {
                print!("请输入文件ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileIdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入文件ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileIdLessThan(value));
                }
            }
            4 => {
                print!("请输入组ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupId(value));
                }
            }
            5 => {
                print!("请输入组ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupIdGreaterThan(value));
                }
            }
            6 => {
                print!("请输入组ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupIdLessThan(value));
                }
            }
            7 => {
                println!("请输入AND组合条件 (输入0结束):");
                let sub_conditions = get_sub_conditions("AND");
                if !sub_conditions.is_empty() {
                    conditions.push(FileGroupCondition::And(sub_conditions));
                }
            }
            8 => {
                println!("请输入OR组合条件 (输入0结束):");
                let sub_conditions = get_sub_conditions("OR");
                if !sub_conditions.is_empty() {
                    conditions.push(FileGroupCondition::Or(sub_conditions));
                }
            }
            9 => {
                println!("请输入NOT条件:");
                if let Some(condition) = get_single_condition() {
                    conditions.push(FileGroupCondition::Not(Box::new(condition)));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_sub_conditions(condition_type: &str) -> Vec<FileGroupCondition> {
    let mut conditions = Vec::new();

    loop {
        println!("\n{}子条件类型:", condition_type);
        println!("1. 文件ID等于");
        println!("2. 文件ID大于");
        println!("3. 文件ID小于");
        println!("4. 组ID等于");
        println!("5. 组ID大于");
        println!("6. 组ID小于");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-6): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入文件ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileId(value));
                }
            }
            2 => {
                print!("请输入文件ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileIdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入文件ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::FileIdLessThan(value));
                }
            }
            4 => {
                print!("请输入组ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupId(value));
                }
            }
            5 => {
                print!("请输入组ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupIdGreaterThan(value));
                }
            }
            6 => {
                print!("请输入组ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileGroupCondition::GroupIdLessThan(value));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_single_condition() -> Option<FileGroupCondition> {
    println!("NOT条件类型:");
    println!("1. 文件ID等于");
    println!("2. 文件ID大于");
    println!("3. 文件ID小于");
    println!("4. 组ID等于");
    println!("5. 组ID大于");
    println!("6. 组ID小于");

    print!("请选择条件类型 (1-6): ");
    io::stdout().flush().unwrap();

    let choice = read_number();

    match choice {
        1 => {
            print!("请输入文件ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::FileId)
        }
        2 => {
            print!("请输入文件ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::FileIdGreaterThan)
        }
        3 => {
            print!("请输入文件ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::FileIdLessThan)
        }
        4 => {
            print!("请输入组ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::GroupId)
        }
        5 => {
            print!("请输入组ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::GroupIdGreaterThan)
        }
        6 => {
            print!("请输入组ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileGroupCondition::GroupIdLessThan)
        }
        _ => {
            println!("无效选择。");
            None
        }
    }
}

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().parse().unwrap_or(0)
}

fn read_number_optional() -> Option<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("输入无效，已忽略该条件。");
            None
        }
    }
}
