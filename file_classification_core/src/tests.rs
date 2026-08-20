// tests.rs
//! 核心库集成测试
//!
//! 基于 SQLite（内存数据库 + 开启外键约束）验证引用计数维护、
//! 级联删除与外键完整性。

use crate::internal::{file_group as file_group_dao, files as files_dao};
use crate::internal::{groups as groups_dao, tags as tags_dao};
use crate::model::models::{
  CreateFileDTO, CreateGroupDTO, CreateTagDTO, File, FileGroupDTO, Group, GroupFilter,
  GroupRelation, GroupTagDTO, RELATION_TYPE_PARENT_CHILD, Tag, UpdateFileDTO, UpdateGroupDTO,
  UpdateTagDTO,
};
use crate::service::{file_group as file_group_service, files as files_service};
use crate::service::{group_relations as group_relations_service, group_tag as group_tag_service};
use crate::service::{groups as groups_service, tags as tags_service};
use crate::utils::database::{AnyConnection, run_pending_migrations};
use crate::utils::errors::AppError;
use diesel::Connection;
use diesel::RunQueryDsl;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations_sqlite");

fn setup() -> AnyConnection {
  let mut sqlite = diesel::SqliteConnection::establish(":memory:").expect("连接 SQLite 失败");
  sqlite.run_pending_migrations(MIGRATIONS).expect("执行迁移失败");
  diesel::sql_query("PRAGMA foreign_keys = ON").execute(&mut sqlite).expect("开启外键失败");
  AnyConnection::Sqlite(sqlite)
}

fn create_group(conn: &mut AnyConnection, name: &str) -> i32 {
  groups_service::create_group(conn, &CreateGroupDTO { name: name.to_string(), description: None })
    .expect("创建分组失败")
}

fn create_file(conn: &mut AnyConnection, path: &str, group_id: i32) -> i32 {
  files_service::create_file(
    conn,
    CreateFileDTO { type_: "txt".to_string(), path: path.to_string(), group_id, description: None },
  )
  .expect("创建文件失败")
}

fn create_tag(conn: &mut AnyConnection, name: &str) -> i32 {
  tags_service::create_tag(conn, &CreateTagDTO { name: name.to_string(), description: None })
    .expect("创建标签失败")
}

fn find_group(conn: &mut AnyConnection, id: i32) -> Option<Group> {
  groups_dao::find_group_by_id(conn, id).expect("查询分组失败")
}

fn get_group(conn: &mut AnyConnection, id: i32) -> Group {
  groups_dao::get_group_by_id(conn, id).expect("查询分组失败")
}

fn get_file(conn: &mut AnyConnection, id: i32) -> File {
  files_service::get_file_by_id(conn, id).expect("查询文件失败")
}

fn get_tag(conn: &mut AnyConnection, id: i32) -> Tag {
  tags_dao::get_tag_by_id(conn, id).expect("查询标签失败")
}

/// 外键约束真实开启：绕过业务层直接删除被文件引用的主组应被数据库拦截
#[test]
fn test_foreign_keys_are_enforced() {
  let mut conn = setup();
  let group_id = create_group(&mut conn, "A");
  let file_id = create_file(&mut conn, "/a.txt", group_id);

  let res = groups_dao::delete_group_by_id(&mut conn, group_id);
  assert!(res.is_err(), "删除被文件引用的组应被外键拦截");

  let file = get_file(&mut conn, file_id);
  assert_eq!(file.id, file_id);
}

/// 创建文件后引用计数正确（主组关联 + 非主组关联 + 标签 + 父子关系）
#[test]
fn test_reference_counts_are_consistent() {
  let mut conn = setup();
  let g1 = create_group(&mut conn, "A");
  let g2 = create_group(&mut conn, "B");
  let tag = create_tag(&mut conn, "t");

  let file_id = create_file(&mut conn, "/a.txt", g1);
  assert_eq!(get_file(&mut conn, file_id).reference_count, 1);
  assert_eq!(get_group(&mut conn, g1).reference_count, 1);

  file_group_service::create_file_group(
    &mut conn,
    FileGroupDTO { file_id, group_id: g2, relation_type: 1 },
  )
  .unwrap();
  assert_eq!(get_file(&mut conn, file_id).reference_count, 2);
  assert_eq!(get_group(&mut conn, g2).reference_count, 1);

  group_tag_service::create_group_tag(&mut conn, GroupTagDTO { group_id: g1, tag_id: tag }).unwrap();
  assert_eq!(get_group(&mut conn, g1).reference_count, 2);
  assert_eq!(get_tag(&mut conn, tag).reference_count, 1);

  group_relations_service::create_group_relation(
    &mut conn,
    GroupRelation { first_group_id: g2, second_group_id: g1, relation_type: RELATION_TYPE_PARENT_CHILD },
  )
  .unwrap();
  assert_eq!(get_group(&mut conn, g2).reference_count, 2);
  assert_eq!(get_group(&mut conn, g1).reference_count, 3);
  // 父组关系由 group_relations 表维护，groups 表不再有 parent_id 列
  assert_eq!(group_relations_service::get_direct_parents_ids(&mut conn, g1).unwrap(), vec![g2]);
}

/// 删除文件：主组、文件组关联、组标签关系全部级联清理，引用计数归零
#[test]
fn test_delete_file_cascades() {
  let mut conn = setup();
  let g1 = create_group(&mut conn, "A");
  let g2 = create_group(&mut conn, "B");
  let tag = create_tag(&mut conn, "t");

  let file_id = create_file(&mut conn, "/a.txt", g1);
  file_group_service::create_file_group(
    &mut conn,
    FileGroupDTO { file_id, group_id: g2, relation_type: 1 },
  )
  .unwrap();
  group_tag_service::create_group_tag(&mut conn, GroupTagDTO { group_id: g1, tag_id: tag }).unwrap();

  files_service::delete_file(&mut conn, file_id).unwrap();

  assert!(find_group(&mut conn, g1).is_none(), "文件的主组应随文件删除");
  assert!(files_dao::find_file_by_id(&mut conn, file_id).unwrap().is_none());
  assert_eq!(get_group(&mut conn, g2).reference_count, 0);
  assert_eq!(get_tag(&mut conn, tag).reference_count, 0);

  // g2 计数归零后可复用为新主组
  let file_id2 = create_file(&mut conn, "/b.txt", g2);
  assert_eq!(get_file(&mut conn, file_id2).reference_count, 1);
}

/// 删除非主组：关联文件的引用计数回退，组本身删除
#[test]
fn test_delete_non_primary_group() {
  let mut conn = setup();
  let g1 = create_group(&mut conn, "A");
  let g2 = create_group(&mut conn, "B");
  let file_id = create_file(&mut conn, "/a.txt", g1);
  file_group_service::create_file_group(
    &mut conn,
    FileGroupDTO { file_id, group_id: g2, relation_type: 1 },
  )
  .unwrap();
  assert_eq!(get_file(&mut conn, file_id).reference_count, 2);

  groups_service::delete_group(&mut conn, g2).unwrap();

  assert!(find_group(&mut conn, g2).is_none());
  assert_eq!(get_file(&mut conn, file_id).reference_count, 1);

  files_service::delete_file(&mut conn, file_id).unwrap();
  assert!(find_group(&mut conn, g1).is_none(), "删除文件时其主组应级联删除");
}

/// 删除主组：级联删除文件、文件组关联、组标签、组关系，其余组计数归零
#[test]
fn test_delete_primary_group_cascades() {
  let mut conn = setup();
  let a = create_group(&mut conn, "A");
  let b = create_group(&mut conn, "B");
  let tag = create_tag(&mut conn, "t");

  let file_id = create_file(&mut conn, "/a.txt", a);
  file_group_service::create_file_group(
    &mut conn,
    FileGroupDTO { file_id, group_id: b, relation_type: 1 },
  )
  .unwrap();
  group_tag_service::create_group_tag(&mut conn, GroupTagDTO { group_id: a, tag_id: tag }).unwrap();
  group_relations_service::create_group_relation(
    &mut conn,
    GroupRelation { first_group_id: b, second_group_id: a, relation_type: RELATION_TYPE_PARENT_CHILD },
  )
  .unwrap();

  groups_service::delete_group(&mut conn, a).unwrap();

  assert!(find_group(&mut conn, a).is_none(), "主组应被删除");
  assert!(files_dao::find_file_by_id(&mut conn, file_id).unwrap().is_none());
  assert_eq!(get_group(&mut conn, b).reference_count, 0, "B 的父子关系与文件关联应全部释放");
  assert_eq!(get_tag(&mut conn, tag).reference_count, 0);

  // B 可复用为新主组
  let file_id2 = create_file(&mut conn, "/b.txt", b);
  assert_eq!(get_file(&mut conn, file_id2).reference_count, 1);
}

/// 父子关系生命周期：创建计数 +1、删除关系计数归零、删除子组自动清理关系
#[test]
fn test_group_relation_parent_id_lifecycle() {
  let mut conn = setup();
  let b = create_group(&mut conn, "B");
  let c = create_group(&mut conn, "C");

  group_relations_service::create_group_relation(
    &mut conn,
    GroupRelation { first_group_id: b, second_group_id: c, relation_type: RELATION_TYPE_PARENT_CHILD },
  )
  .unwrap();
  assert_eq!(group_relations_service::get_direct_parents_ids(&mut conn, c).unwrap(), vec![b]);
  assert_eq!(get_group(&mut conn, b).reference_count, 1);
  assert_eq!(get_group(&mut conn, c).reference_count, 1);

  // 删除父子关系：两端计数归零，组关系清空
  group_relations_service::delete_group_relation(
    &mut conn,
    &GroupRelation { first_group_id: b, second_group_id: c, relation_type: RELATION_TYPE_PARENT_CHILD },
  )
  .unwrap();
  assert!(group_relations_service::get_direct_parents_ids(&mut conn, c).unwrap().is_empty());
  assert_eq!(get_group(&mut conn, b).reference_count, 0);
  assert_eq!(get_group(&mut conn, c).reference_count, 0);

  // 重建父子关系后删除子组：关系被自动清理，父组计数归零
  group_relations_service::create_group_relation(
    &mut conn,
    GroupRelation { first_group_id: b, second_group_id: c, relation_type: RELATION_TYPE_PARENT_CHILD },
  )
  .unwrap();
  assert_eq!(get_group(&mut conn, b).reference_count, 1);

  groups_service::delete_group(&mut conn, c).unwrap();
  assert!(find_group(&mut conn, c).is_none());
  assert_eq!(get_group(&mut conn, b).reference_count, 0);

  // B 可复用为新主组
  let file_id = create_file(&mut conn, "/b.txt", b);
  assert_eq!(get_file(&mut conn, file_id).reference_count, 1);
}

/// 更新接口不允许直接修改业务维护字段（is_primary / reference_count）
#[test]
fn test_update_rejects_business_fields() {
  let mut conn = setup();
  let g1 = create_group(&mut conn, "A");

  // is_primary
  let res = groups_service::update_group_by_id(
    &mut conn,
    g1,
    UpdateGroupDTO { is_primary: Some(true), ..Default::default() },
  );
  assert_err_is(res, AppError::CannotModifyPrimaryStatus);

  // reference_count (group)
  let res = groups_service::update_group_by_id(
    &mut conn,
    g1,
    UpdateGroupDTO { reference_count: Some(10), ..Default::default() },
  );
  assert_err_is(res, AppError::CannotModifyReferenceCount);

  // reference_count (file)
  let file_id = create_file(&mut conn, "/a.txt", g1);
  let res = files_service::update_file_by_id(
    &mut conn,
    file_id,
    UpdateFileDTO { reference_count: Some(5), ..Default::default() },
  );
  assert!(matches!(res, Err(AppError::CannotModifyReferenceCount)));

  // reference_count (tag)
  let tag_id = create_tag(&mut conn, "t");
  let res = tags_service::update_tag_by_id(
    &mut conn,
    tag_id,
    UpdateTagDTO { reference_count: Some(3), ..Default::default() },
  );
  assert_err_is(res, AppError::CannotModifyReferenceCount);

  // 未设置业务字段的普通更新仍然可用
  let res = groups_service::update_group_by_id(
    &mut conn,
    g1,
    UpdateGroupDTO { name: Some("A2".to_string()), ..Default::default() },
  );
  assert_eq!(res.unwrap(), 1);
  assert_eq!(find_group(&mut conn, g1).unwrap().name, "A2");
}

fn assert_err_is(result: Result<usize, AppError>, expected: AppError) {
  match result {
    Err(e) => assert_eq!(e.code(), expected.code()),
    Ok(_) => panic!("期望业务错误，实际成功"),
  }
}

/// filter_with_limit 与其他 filter 变体语义一致：字符串字段 LIKE 匹配，且支持 description 过滤
#[test]
fn test_filter_limit_uses_like_and_description() {
  let mut conn = setup();

  groups_service::create_group(
    &mut conn,
    &CreateGroupDTO { name: "alpha".to_string(), description: Some("meme folder".to_string()) },
  )
  .unwrap();
  groups_service::create_group(
    &mut conn,
    &CreateGroupDTO { name: "alphabet".to_string(), description: None },
  )
  .unwrap();

  // name 子串匹配：LIKE %pha% 应同时命中 alpha 与 alphabet
  let by_name = groups_service::select_groups_by_filter_with_limit(
    &mut conn,
    GroupFilter { name: Some("%pha%".to_string()), ..Default::default() },
    None,
  )
  .unwrap();
  let names: Vec<String> = by_name.iter().map(|g| g.name.clone()).collect();
  assert!(names.contains(&"alpha".to_string()));
  assert!(names.contains(&"alphabet".to_string()));

  // description 过滤（此前 filter_with_limit 路径会静默忽略该字段，与其他变体不一致）
  let by_desc = groups_service::select_groups_by_filter_with_limit(
    &mut conn,
    GroupFilter { description: Some("%meme%".to_string()), ..Default::default() },
    None,
  )
  .unwrap();
  assert_eq!(by_desc.len(), 1);
  assert_eq!(by_desc[0].name, "alpha");
}
