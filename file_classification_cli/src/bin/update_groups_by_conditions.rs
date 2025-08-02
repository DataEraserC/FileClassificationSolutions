use file_classification_core::model::models::{GroupCondition, UpdateGroupDTO};
use file_classification_core::utils::database::establish_connection;
use std::io::{self, Write};
use chrono;
use file_classification_core::service::groups::update_groups_by_conditions;

fn main() {
    let connection = &mut establish_connection();

    println!("组条件更新工具");
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
           update_set.reference_count.is_none() &&
           update_set.is_primary.is_none() &&
           update_set.click_count.is_none() &&
           update_set.share_count.is_none() &&
           update_set.create_time.is_none() &&
           update_set.modify_time.is_none() {
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
        if let Some(is_primary) = update_set.is_primary {
            println!("  是否为主组: {}", is_primary);
        }
        if let Some(click_count) = update_set.click_count {
            println!("  点击次数: {}", click_count);
        }
        if let Some(share_count) = update_set.share_count {
            println!("  分享次数: {}", share_count);
        }

        print!("\n确认执行更新操作？(y/n): ");
        io::stdout().flush().unwrap();

        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).expect("读取输入失败");

        if confirm.trim().to_lowercase() == "y" {
            match update_groups_by_conditions(connection, conditions, update_set) {
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

    println!("感谢使用组条件更新工具！");
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
        println!("5. 名称匹配模式");
        println!("6. 引用计数等于");
        println!("7. 引用计数大于");
        println!("8. 引用计数小于");
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
                print!("请输入组ID值: ");
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
                print!("请输入名称匹配模式 (如 '%图片%'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(GroupCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用计数值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用计数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用计数最大值: ");
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
        println!("5. 名称匹配模式");
        println!("6. 引用计数等于");
        println!("7. 引用计数大于");
        println!("8. 引用计数小于");
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
                print!("请输入组ID值: ");
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
                print!("请输入名称匹配模式 (如 '%图片%'): ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_string_optional() {
                    conditions.push(GroupCondition::NameLike(value));
                }
            }
            6 => {
                print!("请输入引用计数值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCount(value));
                }
            }
            7 => {
                print!("请输入引用计数最小值: ");
                io::stdout().flush().unwrap();
                if let Some(value) = read_number_optional() {
                    conditions.push(GroupCondition::ReferenceCountGreaterThan(value));
                }
            }
            8 => {
                print!("请输入引用计数最大值: ");
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
    println!("5. 名称匹配模式");
    println!("6. 引用计数等于");
    println!("7. 引用计数大于");
    println!("8. 引用计数小于");
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
            print!("请输入组ID值: ");
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
            print!("请输入名称匹配模式 (如 '%图片%'): ");
            io::stdout().flush().unwrap();
            read_string_optional().map(GroupCondition::NameLike)
        }
        6 => {
            print!("请输入引用计数值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ReferenceCount)
        }
        7 => {
            print!("请输入引用计数最小值: ");
            io::stdout().flush().unwrap();
            read_number_optional().map(GroupCondition::ReferenceCountGreaterThan)
        }
        8 => {
            print!("请输入引用计数最大值: ");
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

fn get_update_fields() -> UpdateGroupDTO {
    let name: Option<String>;
    let reference_count: Option<i32>;
    let is_primary: Option<bool>;
    let click_count: Option<i32>;
    let share_count: Option<i32> ;
    let create_time: Option<chrono::NaiveDateTime> = None;
    let modify_time: Option<chrono::NaiveDateTime> = Some(chrono::Local::now().naive_local());

    println!("请输入要更新的字段（留空跳过该字段）:");

    print!("组名称: ");
    io::stdout().flush().unwrap();
    name = read_string_optional();

    print!("引用计数: ");
    io::stdout().flush().unwrap();
    reference_count = read_number_optional();

    print!("是否为主组 (true/false): ");
    io::stdout().flush().unwrap();
    is_primary = read_bool_optional();

    print!("点击次数: ");
    io::stdout().flush().unwrap();
    click_count = read_number_optional();

    print!("分享次数: ");
    io::stdout().flush().unwrap();
    share_count = read_number_optional();

    // 对于时间字段，可以考虑添加输入逻辑，这里简化处理
    UpdateGroupDTO {
        id: None, // ID通常不更新
        name,
        reference_count,
        is_primary,
        click_count,
        share_count,
        create_time,
        modify_time,
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

fn read_bool_optional() -> Option<bool> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let trimmed = input.trim().to_lowercase();
    if trimmed.is_empty() {
        None
    } else {
        match trimmed.as_str() {
            "true" | "1" | "yes" | "y" => Some(true),
            "false" | "0" | "no" | "n" => Some(false),
            _ => {
                println!("输入无效，已忽略该字段。");
                None
            }
        }
    }
}

fn ask_continue() -> bool {
    print!("\n是否继续执行其他操作？(y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().to_lowercase() == "y"
}
