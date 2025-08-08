// 在 src/bin/list_tags_by_conditions.rs

use file_classification_core::model::models::TagCondition;
use file_classification_core::service::tags::select_tags_by_conditions;
use file_classification_core::utils::database::establish_connection;
use std::io::{self, Write};

fn main() {
    let connection = &mut establish_connection();

    println!("标签查询");
    println!("========");

    loop {
        let conditions = get_user_conditions();

        if conditions.is_empty() {
            println!("未输入任何条件");
            // break;
        }

        match select_tags_by_conditions(connection, conditions, Some(20)) {
            Ok(tags) => {
                println!("\n查询结果 (共 {} 条记录):", tags.len());
                println!("-------------------------");
                if tags.is_empty() {
                    println!("没有找到匹配的标签。");
                } else {
                    for tag in tags {
                        println!("{:?}", tag);
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

    println!("感谢使用标签查询！");
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
        println!("5. 名称匹配(like)");
        println!("6. 引用次数等于");
        println!("7. 引用次数大于");
        println!("8. 引用次数小于");
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
                print!("请输入ID值: ");
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
                print!("请输入名称匹配模式: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(TagCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用次数最大值: ");
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
        println!("5. 名称匹配(like)");
        println!("6. 引用次数等于");
        println!("7. 引用次数大于");
        println!("8. 引用次数小于");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-8): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入ID值: ");
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
                print!("请输入名称匹配模式: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(TagCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(TagCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用次数最大值: ");
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
    println!("5. 名称匹配(like)");
    println!("6. 引用次数等于");
    println!("7. 引用次数大于");
    println!("8. 引用次数小于");

    print!("请选择条件类型 (1-8): ");
    io::stdout().flush().unwrap();

    let choice = read_number();

    match choice {
        1 => {
            print!("请输入ID值: ");
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
            print!("请输入名称匹配模式: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(TagCondition::NameLike)
        }
        6 => {
            print!("请输入引用次数: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::ReferenceCount)
        }
        7 => {
            print!("请输入引用次数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::ReferenceCountGreaterThan)
        }
        8 => {
            print!("请输入引用次数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(TagCondition::ReferenceCountLessThan)
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

fn read_string_optional() -> Option<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let trimmed = input.trim();
    if trimmed.is_empty() {
        println!("输入为空，已忽略该条件。");
        None
    } else {
        Some(trimmed.to_string())
    }
}
