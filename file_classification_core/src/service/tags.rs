use crate::model::models::{GroupTagCondition, TagCondition, TagQueryOptions, UpdateTagDTO};
use crate::{
    internal::tags,
    model::models::{CreateTagDTO, Tag, TagFilter},
};
use diesel::{Connection, JoinOnDsl};
use crate::utils::database::AnyConnection;

pub fn create_tag_by_name<S>(conn: &mut AnyConnection, name: S) -> Result<Tag, diesel::result::Error>
where
    S: Into<String>,
{
    let new_tag = CreateTagDTO { name: name.into() };
    tags::create_tag(conn, &new_tag)
}

pub fn create_tag(conn: &mut AnyConnection, create_tag_dto: &CreateTagDTO) -> Result<Tag, diesel::result::Error> {
    tags::create_tag(conn, create_tag_dto)
}

pub fn delete_tag(conn: &mut AnyConnection, tag_id: i32) -> Result<usize, diesel::result::Error> {
    // 使用事务确保数据一致性
    conn.transaction::<usize, diesel::result::Error, _>(|conn| {
        // 查找与该标签关联的所有组标签关系
        let group_tags = crate::internal::group_tag::select_group_tags_by_conditions(
            conn,
            vec![GroupTagCondition::TagId(tag_id)],
            None,
        )?;

        // 对于每个关联的组，减少其引用计数
        for group_tag in &group_tags {
            crate::internal::groups::decrease_group_reference_count(conn, group_tag.group_id)?;
        }

        // 删除与该标签关联的所有组标签关系
        crate::internal::group_tag::delete_group_tags_by_conditions(
            conn,
            vec![GroupTagCondition::TagId(tag_id)]
        )?;

        // 删除标签本身
        tags::delete_tag(conn, tag_id)
    })
}

pub fn select_tags(
    conn: &mut AnyConnection,
    search_input: TagFilter,
    limit: i64,
) -> Result<Vec<Tag>, diesel::result::Error> {
    tags::select_tags(conn, search_input, limit)
}

pub fn select_tags_by_conditions(
    conn: &mut AnyConnection,
    condition: Vec<TagCondition>,
    limit: Option<i64>,
) -> Result<Vec<Tag>, diesel::result::Error> {
    tags::select_tags_by_conditions(conn, condition, limit)
}
pub fn select_tags_by_conditions_with_options(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
    options: TagQueryOptions,
) -> Result<Vec<Tag>, diesel::result::Error> {
    tags::select_tags_by_conditions_with_options(conn, conditions, options)
}

pub fn update_tags_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
    update_set: UpdateTagDTO,
) -> Result<usize, diesel::result::Error> {
    tags::update_tags_by_conditions(conn, conditions, update_set)
}


// NOTE: 这个方法在core里不应该有用法
// 要暴露给用户使用的话 应当改为先select再delete_by_id
// 防止引用计算问题
pub fn delete_tags_by_conditions(
    conn: &mut AnyConnection,
    conditions: Vec<TagCondition>,
) -> Result<usize, diesel::result::Error> {
    // 首先查询将要删除的标签
    let tags_to_delete = select_tags_by_conditions(conn, conditions.clone(), None)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => diesel::result::Error::NotFound,
            _ => e,
        })?;

    // 使用事务确保数据一致性
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        let mut total_deleted = 0;

        // 对于每个要删除的标签，处理相关的引用关系和关联数据
        for tag in &tags_to_delete {
            // 查找与该标签关联的所有组标签关系
            let group_tags = crate::internal::group_tag::select_group_tags_by_conditions(
                conn,
                vec![GroupTagCondition::TagId(tag.id)],
                None,
            )?;

            // 对于每个关联的组，减少其引用计数
            for group_tag in &group_tags {
                crate::internal::groups::decrease_group_reference_count(conn, group_tag.group_id)?;
            }

            // 删除与该标签关联的所有组标签关系
            crate::internal::group_tag::delete_group_tags_by_conditions(
                conn,
                vec![GroupTagCondition::TagId(tag.id)]
            )?;

            // 删除标签本身
            let deleted_count = tags::delete_tag(conn, tag.id)?;
            total_deleted += deleted_count;
        }

        Ok(total_deleted)
    })
}

pub fn select_tag_by_group_id(
    conn: &mut AnyConnection,
    group_id: i64,
) -> Result<Vec<Tag>, diesel::result::Error> {
    crate::internal::tags::select_tag_by_group_id(conn, group_id)
}

pub fn get_tag_by_id(
    conn: &mut AnyConnection,
    tag_id: i32,
) -> Result<Tag, diesel::result::Error> {
    tags::get_tag_by_id(conn, tag_id)
}

pub fn update_tag_by_id(
    conn: &mut AnyConnection,
    tag_id: i32,
    update_set: UpdateTagDTO,
) -> Result<usize, diesel::result::Error> {
    tags::update_tag_by_id(conn, tag_id, update_set)
}
