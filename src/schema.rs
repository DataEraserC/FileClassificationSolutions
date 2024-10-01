// @generated automatically by Diesel CLI.

diesel::table! {
    file_groups (file_id, group_id) {
        file_id -> Nullable<Integer>,
        group_id -> Nullable<Integer>,
    }
}

diesel::table! {
    files (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Text,
        path -> Text,
        reference_count -> Nullable<Integer>,
        group_id -> Nullable<Integer>,
    }
}

diesel::table! {
    group_tags (group_id, tag_id) {
        group_id -> Nullable<Integer>,
        tag_id -> Nullable<Integer>,
    }
}

diesel::table! {
    groups (id) {
        id -> Nullable<Integer>,
        name -> Text,
        is_primary -> Bool,
        click_count -> Nullable<Integer>,
        share_count -> Nullable<Integer>,
        create_time -> BigInt,
        modify_time -> BigInt,
    }
}

diesel::table! {
    tags (id) {
        id -> Nullable<Integer>,
        reference_count -> Nullable<Integer>,
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
