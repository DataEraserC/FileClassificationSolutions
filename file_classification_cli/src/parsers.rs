// parsers.rs
// 所有条件和排序解析函数，独立模块，便于维护

use file_classification_core::model::models;

/// 解析文件条件
pub fn parse_file_conditions(args: &[String]) -> Vec<models::FileCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::Id(value));
                }
                i += 2;
            }
            "id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::IdGreaterThan(value));
                }
                i += 2;
            }
            "id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::IdLessThan(value));
                }
                i += 2;
            }
            "id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::FileCondition::IdIn(values));
                }
                i += 2;
            }
            "type" if i + 1 < args.len() => {
                conditions.push(models::FileCondition::Type(args[i + 1].clone()));
                i += 2;
            }
            "type_like" if i + 1 < args.len() => {
                conditions.push(models::FileCondition::TypeLike(args[i + 1].clone()));
                i += 2;
            }
            "type_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1].split(',').map(|s| s.to_string()).collect();
                conditions.push(models::FileCondition::TypeIn(values));
                i += 2;
            }
            "path" if i + 1 < args.len() => {
                conditions.push(models::FileCondition::Path(args[i + 1].clone()));
                i += 2;
            }
            "path_like" if i + 1 < args.len() => {
                conditions.push(models::FileCondition::PathLike(args[i + 1].clone()));
                i += 2;
            }
            "path_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1].split(',').map(|s| s.to_string()).collect();
                conditions.push(models::FileCondition::PathIn(values));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "ref_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::ReferenceCountGreaterThan(value));
                }
                i += 2;
            }
            "ref_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::ReferenceCountLessThan(value));
                }
                i += 2;
            }
            "ref_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::FileCondition::ReferenceCountIn(values));
                }
                i += 2;
            }
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::GroupId(value));
                }
                i += 2;
            }
            "group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::GroupIdGreaterThan(value));
                }
                i += 2;
            }
            "group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileCondition::GroupIdLessThan(value));
                }
                i += 2;
            }
            "group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::FileCondition::GroupIdIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

/// 解析组条件
pub fn parse_group_conditions(args: &[String]) -> Vec<models::GroupCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::Id(value));
                }
                i += 2;
            }
            "id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::IdGreaterThan(value));
                }
                i += 2;
            }
            "id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::IdLessThan(value));
                }
                i += 2;
            }
            "id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupCondition::IdIn(values));
                }
                i += 2;
            }
            "name" if i + 1 < args.len() => {
                conditions.push(models::GroupCondition::Name(args[i + 1].clone()));
                i += 2;
            }
            "name_like" if i + 1 < args.len() => {
                conditions.push(models::GroupCondition::NameLike(args[i + 1].clone()));
                i += 2;
            }
            "name_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1].split(',').map(|s| s.to_string()).collect();
                conditions.push(models::GroupCondition::NameIn(values));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "ref_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ReferenceCountGreaterThan(value));
                }
                i += 2;
            }
            "ref_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ReferenceCountLessThan(value));
                }
                i += 2;
            }
            "ref_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupCondition::ReferenceCountIn(values));
                }
                i += 2;
            }
            "is_primary" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<bool>() {
                    conditions.push(models::GroupCondition::IsPrimary(value));
                }
                i += 2;
            }
            "click_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ClickCount(value));
                }
                i += 2;
            }
            "click_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ClickCountGreaterThan(value));
                }
                i += 2;
            }
            "click_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ClickCountLessThan(value));
                }
                i += 2;
            }
            "click_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupCondition::ClickCountIn(values));
                }
                i += 2;
            }
            "share_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ShareCount(value));
                }
                i += 2;
            }
            "share_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ShareCountGreaterThan(value));
                }
                i += 2;
            }
            "share_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupCondition::ShareCountLessThan(value));
                }
                i += 2;
            }
            "share_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupCondition::ShareCountIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

/// 解析标签条件
pub fn parse_tag_conditions(args: &[String]) -> Vec<models::TagCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::Id(value));
                }
                i += 2;
            }
            "id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::IdGreaterThan(value));
                }
                i += 2;
            }
            "id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::IdLessThan(value));
                }
                i += 2;
            }
            "id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::TagCondition::IdIn(values));
                }
                i += 2;
            }
            "name" if i + 1 < args.len() => {
                conditions.push(models::TagCondition::Name(args[i + 1].clone()));
                i += 2;
            }
            "name_like" if i + 1 < args.len() => {
                conditions.push(models::TagCondition::NameLike(args[i + 1].clone()));
                i += 2;
            }
            "name_in" if i + 1 < args.len() => {
                let values: Vec<String> = args[i + 1].split(',').map(|s| s.to_string()).collect();
                conditions.push(models::TagCondition::NameIn(values));
                i += 2;
            }
            "ref_count" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::ReferenceCount(value));
                }
                i += 2;
            }
            "ref_count_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::ReferenceCountGreaterThan(value));
                }
                i += 2;
            }
            "ref_count_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::TagCondition::ReferenceCountLessThan(value));
                }
                i += 2;
            }
            "ref_count_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::TagCondition::ReferenceCountIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

/// 解析文件组条件
pub fn parse_file_group_conditions(args: &[String]) -> Vec<models::FileGroupCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "file_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::FileId(value));
                }
                i += 2;
            }
            "file_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::FileIdGreaterThan(value));
                }
                i += 2;
            }
            "file_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::FileIdLessThan(value));
                }
                i += 2;
            }
            "file_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::FileGroupCondition::FileIdIn(values));
                }
                i += 2;
            }
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::GroupId(value));
                }
                i += 2;
            }
            "group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::GroupIdGreaterThan(value));
                }
                i += 2;
            }
            "group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::GroupIdLessThan(value));
                }
                i += 2;
            }
            "group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::FileGroupCondition::GroupIdIn(values));
                }
                i += 2;
            }
            "relation_type" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::RelationType(value));
                }
                i += 2;
            }
            "relation_type_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::RelationTypeGreaterThan(value));
                }
                i += 2;
            }
            "relation_type_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::FileGroupCondition::RelationTypeLessThan(value));
                }
                i += 2;
            }
            "relation_type_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::FileGroupCondition::RelationTypeIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

/// 解析组标签条件
pub fn parse_group_tag_conditions(args: &[String]) -> Vec<models::GroupTagCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::GroupId(value));
                }
                i += 2;
            }
            "group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::GroupIdGreaterThan(value));
                }
                i += 2;
            }
            "group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::GroupIdLessThan(value));
                }
                i += 2;
            }
            "group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupTagCondition::GroupIdIn(values));
                }
                i += 2;
            }
            "tag_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::TagId(value));
                }
                i += 2;
            }
            "tag_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::TagIdGreaterThan(value));
                }
                i += 2;
            }
            "tag_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupTagCondition::TagIdLessThan(value));
                }
                i += 2;
            }
            "tag_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupTagCondition::TagIdIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

/// 解析组关系条件
pub fn parse_group_relation_conditions(args: &[String]) -> Vec<models::GroupRelationCondition> {
    let mut conditions = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "first_group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupRelationCondition::FirstGroupId(value));
                }
                i += 2;
            }
            "first_group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupRelationCondition::FirstGroupIdGreaterThan(value));
                }
                i += 2;
            }
            "first_group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupRelationCondition::FirstGroupIdLessThan(value));
                }
                i += 2;
            }
            "first_group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupRelationCondition::FirstGroupIdIn(values));
                }
                i += 2;
            }
            "second_group_id" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupRelationCondition::SecondGroupId(value));
                }
                i += 2;
            }
            "second_group_id_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupRelationCondition::SecondGroupIdGreaterThan(value));
                }
                i += 2;
            }
            "second_group_id_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupRelationCondition::SecondGroupIdLessThan(value));
                }
                i += 2;
            }
            "second_group_id_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupRelationCondition::SecondGroupIdIn(values));
                }
                i += 2;
            }
            "relation_type" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupRelationCondition::RelationType(value));
                }
                i += 2;
            }
            "relation_type_gt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupRelationCondition::RelationTypeGreaterThan(value));
                }
                i += 2;
            }
            "relation_type_lt" if i + 1 < args.len() => {
                if let Ok(value) = args[i + 1].parse::<i32>() {
                    conditions.push(models::GroupRelationCondition::RelationTypeLessThan(value));
                }
                i += 2;
            }
            "relation_type_in" if i + 1 < args.len() => {
                let values: Result<Vec<i32>, _> = args[i + 1].split(',').map(|s| s.parse::<i32>()).collect();
                if let Ok(values) = values {
                    conditions.push(models::GroupRelationCondition::RelationTypeIn(values));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    conditions
}

/// 解析文件排序
pub fn parse_file_order_by(args: &[String]) -> Vec<models::FileOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => models::OrderDirection::Asc,
            "desc" => models::OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "id" => models::FileOrderBy::Id(direction),
            "type" => models::FileOrderBy::Type(direction),
            "path" => models::FileOrderBy::Path(direction),
            "ref_count" => models::FileOrderBy::ReferenceCount(direction),
            "group_id" => models::FileOrderBy::GroupId(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

/// 解析组排序
pub fn parse_group_order_by(args: &[String]) -> Vec<models::GroupOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => models::OrderDirection::Asc,
            "desc" => models::OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "id" => models::GroupOrderBy::Id(direction),
            "name" => models::GroupOrderBy::Name(direction),
            "ref_count" => models::GroupOrderBy::ReferenceCount(direction),
            "is_primary" => models::GroupOrderBy::IsPrimary(direction),
            "click_count" => models::GroupOrderBy::ClickCount(direction),
            "share_count" => models::GroupOrderBy::ShareCount(direction),
            "create_time" => models::GroupOrderBy::CreateTime(direction),
            "modify_time" => models::GroupOrderBy::ModifyTime(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

/// 解析标签排序
pub fn parse_tag_order_by(args: &[String]) -> Vec<models::TagOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => models::OrderDirection::Asc,
            "desc" => models::OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "id" => models::TagOrderBy::Id(direction),
            "name" => models::TagOrderBy::Name(direction),
            "ref_count" => models::TagOrderBy::ReferenceCount(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

/// 解析文件组排序
pub fn parse_file_group_order_by(args: &[String]) -> Vec<models::FileGroupOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => models::OrderDirection::Asc,
            "desc" => models::OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "file_id" => models::FileGroupOrderBy::FileId(direction),
            "group_id" => models::FileGroupOrderBy::GroupId(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

/// 解析组标签排序
pub fn parse_group_tag_order_by(args: &[String]) -> Vec<models::GroupTagOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let field = parts[0];
        let direction = match parts[1].to_lowercase().as_str() {
            "asc" => models::OrderDirection::Asc,
            "desc" => models::OrderDirection::Desc,
            _ => continue,
        };

        let order_by = match field {
            "group_id" => models::GroupTagOrderBy::GroupId(direction),
            "tag_id" => models::GroupTagOrderBy::TagId(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}

/// 解析组关系排序
pub fn parse_group_relation_order_by(args: &[String]) -> Vec<models::GroupRelationOrderBy> {
    let mut order_bys = Vec::new();

    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.is_empty() || parts.len() > 2 {
            continue;
        }

        let field = parts[0];
        let direction = if parts.len() == 2 && parts[1] == "desc" {
            models::OrderDirection::Desc
        } else {
            models::OrderDirection::Asc
        };

        let order_by = match field {
            "first_group_id" => models::GroupRelationOrderBy::FirstGroupId(direction),
            "second_group_id" => models::GroupRelationOrderBy::SecondGroupId(direction),
            "relation_type" => models::GroupRelationOrderBy::RelationType(direction),
            _ => continue,
        };

        order_bys.push(order_by);
    }

    order_bys
}
