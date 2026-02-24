// @generated automatically by Diesel CLI.

diesel::table! {
    file_groups (file_id, group_id) {
        file_id -> Integer,
        group_id -> Integer,
        relation_type -> Integer,
    }
}

diesel::table! {
    files (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        path -> Text,
        reference_count -> Integer,
        group_id -> Integer,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    group_relations (first_group_id, second_group_id, relation_type) {
        first_group_id -> Integer,
        second_group_id -> Integer,
        relation_type -> Integer,
    }
}

diesel::table! {
    group_tags (group_id, tag_id) {
        group_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    groups (id) {
        id -> Integer,
        name -> Text,
        reference_count -> Integer,
        is_primary -> Bool,
        click_count -> Integer,
        share_count -> Integer,
        create_time -> Timestamp,
        modify_time -> Timestamp,
        parent_id -> Nullable<Integer>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        reference_count -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(file_groups -> files (file_id));
diesel::joinable!(file_groups -> groups (group_id));
diesel::joinable!(files -> groups (group_id));
diesel::joinable!(group_tags -> groups (group_id));
diesel::joinable!(group_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
  file_groups,
  files,
  group_relations,
  group_tags,
  groups,
  tags,
);
