use file_classification_core::model::models::{FileCondition, UpdateFileDTO};
use file_classification_core::service::files::update_files_by_conditions;
use file_classification_core::utils::database::establish_connection;
use std::io::{self, Write};

fn main() {
    let connection = &mut establish_connection();

    println!("文件条件更新工具");
    println!("==============");

    loop {
        println!("\n第一步：设置更新条件");
        let conditions = get_user_conditions();

        if conditions.is_empty() {
            println!("未设置任何条件，无法执行更新操作。");
            if !ask_continue() {
                break;
            }
            continue;
        }

        println!("\n第二步：设置要更新的字段");
        let update_set = get_update_fields();

        if update_set.path.is_none() &&
            update_set.type_.is_none() &&
            update_set.reference_count.is_none() &&
            update_set.group_id.is_none() {
            println!("未设置任何要更新的字段。");
            if !ask_continue() {
                break;
            }
            continue;
        }

        println!("\n确认更新操作：");
        println!("条件数量: {}", conditions.len());
        println!("更新字段:");
        if let Some(ref path) = update_set.path {
            println!("  路径: {}", path);
        }
        if let Some(ref type_) = update_set.type_ {
            println!("  类型: {}", type_);
        }
        if let Some(ref_count) = update_set.reference_count {
            println!("  引用计数: {}", ref_count);
        }
        if let Some(group_id) = update_set.group_id {
            println!("  组ID: {}", group_id);
        }

        print!("\n确认执行更新操作？(y/n): ");
        io::stdout().flush().unwrap();

        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).expect("读取输入失败");

        if confirm.trim().to_lowercase() == "y" {
            match update_files_by_conditions(connection, conditions, update_set) {
                Ok(updated_count) => {
                    println!("成功更新 {} 条记录。", updated_count);
                }
                Err(e) => {
                    eprintln!("更新失败: {}", e);
                }
            }
        } else {
            println!("操作已取消。");
        }

        if !ask_continue() {
            break;
        }
    }

    println!("感谢使用文件条件更新工具！");
}

fn get_user_conditions() -> Vec<FileCondition> {
    let mut conditions = Vec::new();

    println!("请输入查询条件（留空结束输入）:");

    loop {
        println!("\n可选条件类型:");
        println!("1. ID等于");
        println!("2. ID大于");
        println!("3. ID小于");
        println!("4. 类型等于");
        println!("5. 类型匹配模式");
        println!("6. 路径等于");
        println!("7. 路径匹配模式");
        println!("8. 引用计数等于");
        println!("9. 引用计数大于");
        println!("10. 引用计数小于");
        println!("11. 组ID等于");
        println!("12. 组ID大于");
        println!("13. 组ID小于");
        println!("14. AND组合条件");
        println!("15. OR组合条件");
        println!("16. NOT条件");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-16): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入文件ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入文件类型: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::Type(value));
                }
            }
            5 => {
                print!("请输入类型匹配模式 (如 '%.jpg'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::TypeLike(value));
                }
            }
            6 => {
                print!("请输入文件路径: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::Path(value));
                }
            }
            7 => {
                print!("请输入路径匹配模式: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::PathLike(value));
                }
            }
            8 => {
                print!("请输入引用计数值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCount(value));
                }
            }
            9 => {
                print!("请输入引用计数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCountGreaterThan(value));
                }
            }
            10 => {
                print!("请输入引用计数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCountLessThan(value));
                }
            }
            11 => {
                print!("请输入组ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupId(value));
                }
            }
            12 => {
                print!("请输入组ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupIdGreaterThan(value));
                }
            }
            13 => {
                print!("请输入组ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupIdLessThan(value));
                }
            }
            14 => {
                println!("请输入AND组合条件 (输入0结束):");
                let sub_conditions = get_sub_conditions("AND");
                if !sub_conditions.is_empty() {
                    conditions.push(FileCondition::And(sub_conditions));
                }
            }
            15 => {
                println!("请输入OR组合条件 (输入0结束):");
                let sub_conditions = get_sub_conditions("OR");
                if !sub_conditions.is_empty() {
                    conditions.push(FileCondition::Or(sub_conditions));
                }
            }
            16 => {
                println!("请输入NOT条件:");
                if let Some(condition) = get_single_condition() {
                    conditions.push(FileCondition::Not(Box::new(condition)));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_sub_conditions(condition_type: &str) -> Vec<FileCondition> {
    let mut conditions = Vec::new();

    loop {
        println!("\n{}子条件类型:", condition_type);
        println!("1. ID等于");
        println!("2. ID大于");
        println!("3. ID小于");
        println!("4. 类型等于");
        println!("5. 类型匹配模式");
        println!("6. 路径等于");
        println!("7. 路径匹配模式");
        println!("8. 引用计数等于");
        println!("9. 引用计数大于");
        println!("10. 引用计数小于");
        println!("11. 组ID等于");
        println!("12. 组ID大于");
        println!("13. 组ID小于");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-13): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入文件ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入文件类型: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::Type(value));
                }
            }
            5 => {
                print!("请输入类型匹配模式 (如 '%.jpg'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::TypeLike(value));
                }
            }
            6 => {
                print!("请输入文件路径: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::Path(value));
                }
            }
            7 => {
                print!("请输入路径匹配模式: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(FileCondition::PathLike(value));
                }
            }
            8 => {
                print!("请输入引用计数值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCount(value));
                }
            }
            9 => {
                print!("请输入引用计数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCountGreaterThan(value));
                }
            }
            10 => {
                print!("请输入引用计数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::ReferenceCountLessThan(value));
                }
            }
            11 => {
                print!("请输入组ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupId(value));
                }
            }
            12 => {
                print!("请输入组ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupIdGreaterThan(value));
                }
            }
            13 => {
                print!("请输入组ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(FileCondition::GroupIdLessThan(value));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_single_condition() -> Option<FileCondition> {
    println!("NOT条件类型:");
    println!("1. ID等于");
    println!("2. ID大于");
    println!("3. ID小于");
    println!("4. 类型等于");
    println!("5. 类型匹配模式");
    println!("6. 路径等于");
    println!("7. 路径匹配模式");
    println!("8. 引用计数等于");
    println!("9. 引用计数大于");
    println!("10. 引用计数小于");
    println!("11. 组ID等于");
    println!("12. 组ID大于");
    println!("13. 组ID小于");

    print!("请选择条件类型 (1-13): ");
    io::stdout().flush().unwrap();

    let choice = read_number();

    match choice {
        1 => {
            print!("请输入文件ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::Id)
        }
        2 => {
            print!("请输入ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::IdGreaterThan)
        }
        3 => {
            print!("请输入ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::IdLessThan)
        }
        4 => {
            print!("请输入文件类型: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(FileCondition::Type)
        }
        5 => {
            print!("请输入类型匹配模式 (如 '%.jpg'): ");
            io::stdout().flush().unwrap();
            read_string_optional().map(FileCondition::TypeLike)
        }
        6 => {
            print!("请输入文件路径: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(FileCondition::Path)
        }
        7 => {
            print!("请输入路径匹配模式: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(FileCondition::PathLike)
        }
        8 => {
            print!("请输入引用计数值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::ReferenceCount)
        }
        9 => {
            print!("请输入引用计数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::ReferenceCountGreaterThan)
        }
        10 => {
            print!("请输入引用计数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::ReferenceCountLessThan)
        }
        11 => {
            print!("请输入组ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::GroupId)
        }
        12 => {
            print!("请输入组ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::GroupIdGreaterThan)
        }
        13 => {
            print!("请输入组ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(FileCondition::GroupIdLessThan)
        }
        _ => {
            println!("无效选择。");
            None
        }
    }
}

fn get_update_fields() -> UpdateFileDTO {
    let path: Option<String>;
    let type_: Option<String>;
    let reference_count: Option<i32>;
    let group_id: Option<i32>;

    println!("请输入要更新的字段（留空跳过该字段）:");

    print!("文件路径: ");
    io::stdout().flush().unwrap();
    path = read_string_optional();

    print!("文件类型: ");
    io::stdout().flush().unwrap();
    type_ = read_string_optional();

    print!("引用计数: ");
    io::stdout().flush().unwrap();
    reference_count = read_number_optional();

    print!("组ID: ");
    io::stdout().flush().unwrap();
    group_id = read_number_optional();

    // 修改返回结构体字段为拥有所有权的 Option 值
    UpdateFileDTO {
        path,
        type_,
        reference_count,
        group_id,
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
            if input.trim().is_empty() {
                None
            } else {
                println!("输入无效，已忽略该字段。");
                None
            }
        }
    }
}

fn read_string_optional() -> Option<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn ask_continue() -> bool {
    print!("\n是否继续执行其他操作？(y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().to_lowercase() == "y"
}
