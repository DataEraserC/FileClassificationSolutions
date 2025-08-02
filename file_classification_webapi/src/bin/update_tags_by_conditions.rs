use file_classification_core::model::models::{TagCondition, UpdateTagDTO};
use file_classification_core::utils::database::establish_connection;
use std::io::{self, Write};
use file_classification_core::service::tags::update_tags_by_conditions;

fn main() {
    let connection = &mut establish_connection();

    println!("标签条件更新工具");
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

        if update_set.name.is_none() &&
           update_set.reference_count.is_none() {
            println!("未设置任何要更新的字段。");
            if !ask_continue() {
                break;
            }
            continue;
        }

        println!("\n确认更新操作：");
        println!("条件数量: {}", conditions.len());
        println!("更新字段:");
        if let Some(ref name) = update_set.name {
            println!("  名称: {}", name);
        }
        if let Some(ref_count) = update_set.reference_count {
            println!("  引用计数: {}", ref_count);
        }

        print!("\n确认执行更新操作？(y/n): ");
        io::stdout().flush().unwrap();

        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).expect("读取输入失败");

        if confirm.trim().to_lowercase() == "y" {
            match update_tags_by_conditions(connection, conditions, update_set) {
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

    println!("感谢使用标签条件更新工具！");
}

fn get_user_conditions() -> Vec<TagCondition> {
    let mut conditions = Vec::new();

    println!("请输入查询条件（留空结束输入）:");

    loop {
        println!("\n可选条件类型:");
        println!("1. ID等于");
        println!("2. ID大于");
        println!("3. ID小于");
        println!("4. 名称等于");
        println!("5. 名称匹配模式");
        println!("6. 引用计数等于");
        println!("7. 引用计数大于");
        println!("8. 引用计数小于");
        println!("9. AND组合条件");
        println!("10. OR组合条件");
        println!("11. NOT条件");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-11): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入标签ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入标签名称: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(TagCondition::Name(value));
                }
            }
            5 => {
                print!("请输入名称匹配模式 (如 '%重要%'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(TagCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用计数值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用计数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用计数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCountLessThan(value));
                }
            }
            9 => {
                println!("请输入AND组合条件 (输入0结束):");
                let sub_conditions = get_sub_conditions("AND");
                if !sub_conditions.is_empty() {
                    conditions.push(TagCondition::And(sub_conditions));
                }
            }
            10 => {
                println!("请输入OR组合条件 (输入0结束):");
                let sub_conditions = get_sub_conditions("OR");
                if !sub_conditions.is_empty() {
                    conditions.push(TagCondition::Or(sub_conditions));
                }
            }
            11 => {
                println!("请输入NOT条件:");
                if let Some(condition) = get_single_condition() {
                    conditions.push(TagCondition::Not(Box::new(condition)));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_sub_conditions(condition_type: &str) -> Vec<TagCondition> {
    let mut conditions = Vec::new();

    loop {
        println!("\n{}子条件类型:", condition_type);
        println!("1. ID等于");
        println!("2. ID大于");
        println!("3. ID小于");
        println!("4. 名称等于");
        println!("5. 名称匹配模式");
        println!("6. 引用计数等于");
        println!("7. 引用计数大于");
        println!("8. 引用计数小于");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-8): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入标签ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入标签名称: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(TagCondition::Name(value));
                }
            }
            5 => {
                print!("请输入名称匹配模式 (如 '%重要%'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(TagCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用计数值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用计数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用计数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCountLessThan(value));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_single_condition() -> Option<TagCondition> {
    println!("NOT条件类型:");
    println!("1. ID等于");
    println!("2. ID大于");
    println!("3. ID小于");
    println!("4. 名称等于");
    println!("5. 名称匹配模式");
    println!("6. 引用计数等于");
    println!("7. 引用计数大于");
    println!("8. 引用计数小于");

    print!("请选择条件类型 (1-8): ");
    io::stdout().flush().unwrap();

    let choice = read_number();

    match choice {
        1 => {
            print!("请输入标签ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::Id)
        }
        2 => {
            print!("请输入ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::IdGreaterThan)
        }
        3 => {
            print!("请输入ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::IdLessThan)
        }
        4 => {
            print!("请输入标签名称: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(TagCondition::Name)
        }
        5 => {
            print!("请输入名称匹配模式 (如 '%重要%'): ");
            io::stdout().flush().unwrap();
            read_string_optional().map(TagCondition::NameLike)
        }
        6 => {
            print!("请输入引用计数值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::ReferenceCount)
        }
        7 => {
            print!("请输入引用计数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::ReferenceCountGreaterThan)
        }
        8 => {
            print!("请输入引用计数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::ReferenceCountLessThan)
        }
        _ => {
            println!("无效选择。");
            None
        }
    }
}

fn get_update_fields() -> UpdateTagDTO {
    let name: Option<String>;
    let reference_count: Option<i32>;

    println!("请输入要更新的字段（留空跳过该字段）:");

    print!("标签名称: ");
    io::stdout().flush().unwrap();
    name = read_string_optional();

    print!("引用计数: ");
    io::stdout().flush().unwrap();
    reference_count = read_number_optional();

    UpdateTagDTO {
        name,
        reference_count,
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
