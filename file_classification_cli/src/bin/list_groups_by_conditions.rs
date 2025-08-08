// 在 src/bin/list_groups_by_conditions.rs

use file_classification_core::model::models::GroupCondition;
use file_classification_core::service::groups::select_groups_by_conditions;
use file_classification_core::utils::database::establish_connection;
use std::io::{self, Write};

fn main() {
    let connection = &mut establish_connection();

    println!("组查询");
    println!("======");

    loop {
        let conditions = get_user_conditions();

        if conditions.is_empty() {
            println!("未输入任何条件");
            // break;
        }

        match select_groups_by_conditions(connection, conditions, Some(20)) {
            Ok(groups) => {
                println!("\n查询结果 (共 {} 条记录):", groups.len());
                println!("-------------------------");
                if groups.is_empty() {
                    println!("没有找到匹配的组。");
                } else {
                    for group in groups {
                        println!("{:?}", group);
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

    println!("感谢使用组查询！");
}

fn get_user_conditions() -> Vec<GroupCondition> {
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
        println!("9. 是否为主组");
        println!("10. 点击次数等于");
        println!("11. 点击次数大于");
        println!("12. 点击次数小于");
        println!("13. 分享次数等于");
        println!("14. 分享次数大于");
        println!("15. 分享次数小于");
        println!("16. AND组合条件");
        println!("17. OR组合条件");
        println!("18. NOT条件");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-18): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入组名称: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(GroupCondition::Name(value));
                }
            }
            5 => {
                print!("请输入名称匹配模式: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(GroupCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCountLessThan(value));
                }
            }
            9 => {
                print!("是否为主组 (true/false): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_bool_optional() {
                    conditions.push(GroupCondition::IsPrimary(value));
                }
            }
            10 => {
                print!("请输入点击次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCount(value));
                }
            }
            11 => {
                print!("请输入点击次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCountGreaterThan(value));
                }
            }
            12 => {
                print!("请输入点击次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCountLessThan(value));
                }
            }
            13 => {
                print!("请输入分享次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCount(value));
                }
            }
            14 => {
                print!("请输入分享次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCountGreaterThan(value));
                }
            }
            15 => {
                print!("请输入分享次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCountLessThan(value));
                }
            }
            16 => {
                println!("请输入AND组合条件 (输入0结束):");
                let sub_conditions = get_sub_conditions("AND");
                if !sub_conditions.is_empty() {
                    conditions.push(GroupCondition::And(sub_conditions));
                }
            }
            17 => {
                println!("请输入OR组合条件 (输入0结束):");
                let sub_conditions = get_sub_conditions("OR");
                if !sub_conditions.is_empty() {
                    conditions.push(GroupCondition::Or(sub_conditions));
                }
            }
            18 => {
                println!("请输入NOT条件:");
                if let Some(condition) = get_single_condition() {
                    conditions.push(GroupCondition::Not(Box::new(condition)));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_sub_conditions(condition_type: &str) -> Vec<GroupCondition> {
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
        println!("9. 是否为主组");
        println!("10. 点击次数等于");
        println!("11. 点击次数大于");
        println!("12. 点击次数小于");
        println!("13. 分享次数等于");
        println!("14. 分享次数大于");
        println!("15. 分享次数小于");
        println!("0. 结束输入");

        print!("请选择条件类型 (0-15): ");
        io::stdout().flush().unwrap();

        let choice = read_number();

        match choice {
            0 => break,
            1 => {
                print!("请输入ID值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::Id(value));
                }
            }
            2 => {
                print!("请输入ID最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::IdGreaterThan(value));
                }
            }
            3 => {
                print!("请输入ID最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::IdLessThan(value));
                }
            }
            4 => {
                print!("请输入组名称: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(GroupCondition::Name(value));
                }
            }
            5 => {
                print!("请输入名称匹配模式: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(GroupCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCountLessThan(value));
                }
            }
            9 => {
                print!("是否为主组 (true/false): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_bool_optional() {
                    conditions.push(GroupCondition::IsPrimary(value));
                }
            }
            10 => {
                print!("请输入点击次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCount(value));
                }
            }
            11 => {
                print!("请输入点击次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCountGreaterThan(value));
                }
            }
            12 => {
                print!("请输入点击次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ClickCountLessThan(value));
                }
            }
            13 => {
                print!("请输入分享次数: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCount(value));
                }
            }
            14 => {
                print!("请输入分享次数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCountGreaterThan(value));
                }
            }
            15 => {
                print!("请输入分享次数最大值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ShareCountLessThan(value));
                }
            }
            _ => {
                println!("无效选择，请重新输入。");
            }
        }
    }

    conditions
}

fn get_single_condition() -> Option<GroupCondition> {
    println!("NOT条件类型:");
    println!("1. ID等于");
    println!("2. ID大于");
    println!("3. ID小于");
    println!("4. 名称等于");
    println!("5. 名称匹配(like)");
    println!("6. 引用次数等于");
    println!("7. 引用次数大于");
    println!("8. 引用次数小于");
    println!("9. 是否为主组");
    println!("10. 点击次数等于");
    println!("11. 点击次数大于");
    println!("12. 点击次数小于");
    println!("13. 分享次数等于");
    println!("14. 分享次数大于");
    println!("15. 分享次数小于");

    print!("请选择条件类型 (1-15): ");
    io::stdout().flush().unwrap();

    let choice = read_number();

    match choice {
        1 => {
            print!("请输入ID值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::Id)
        }
        2 => {
            print!("请输入ID最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::IdGreaterThan)
        }
        3 => {
            print!("请输入ID最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::IdLessThan)
        }
        4 => {
            print!("请输入组名称: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(GroupCondition::Name)
        }
        5 => {
            print!("请输入名称匹配模式: ");
            io::stdout().flush().unwrap();
            read_string_optional().map(GroupCondition::NameLike)
        }
        6 => {
            print!("请输入引用次数: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ReferenceCount)
        }
        7 => {
            print!("请输入引用次数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ReferenceCountGreaterThan)
        }
        8 => {
            print!("请输入引用次数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ReferenceCountLessThan)
        }
        9 => {
            print!("是否为主组 (true/false): ");
            io::stdout().flush().unwrap();
            read_bool_optional().map(GroupCondition::IsPrimary)
        }
        10 => {
            print!("请输入点击次数: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ClickCount)
        }
        11 => {
            print!("请输入点击次数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ClickCountGreaterThan)
        }
        12 => {
            print!("请输入点击次数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ClickCountLessThan)
        }
        13 => {
            print!("请输入分享次数: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ShareCount)
        }
        14 => {
            print!("请输入分享次数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ShareCountGreaterThan)
        }
        15 => {
            print!("请输入分享次数最大值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ShareCountLessThan)
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

fn read_bool_optional() -> Option<bool> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let trimmed = input.trim().to_lowercase();
    match trimmed.as_str() {
        "true" | "t" | "yes" | "y" | "1" => Some(true),
        "false" | "f" | "no" | "n" | "0" => Some(false),
        _ => {
            println!("输入无效，已忽略该条件。");
            None
        }
    }
}
