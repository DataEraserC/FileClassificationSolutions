// @generated automatically by Diesel CLI.

diesel::table! {
    file_groups (file_id, group_id) {
        file_id -> Integer,
        group_id -> Integer,
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
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        reference_count -> Integer,
        name -> Text,
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
    group_tags,
    groups,
    tags,
);
