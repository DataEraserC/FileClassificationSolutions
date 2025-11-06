-- 删除tags表的description字段
CREATE TABLE tags_new (LIKE tags INCLUDING ALL);
ALTER TABLE tags_new DROP COLUMN description;
INSERT INTO tags_new SELECT id, reference_count, name FROM tags;
DROP TABLE tags;
ALTER TABLE tags_new RENAME TO tags;

-- 删除groups表的description字段
CREATE TABLE groups_new (LIKE groups INCLUDING ALL);
ALTER TABLE groups_new DROP COLUMN description;
INSERT INTO groups_new SELECT id, name, reference_count, is_primary, click_count, share_count, create_time, modify_time, parent_id FROM groups;
DROP TABLE groups;
ALTER TABLE groups_new RENAME TO groups;

-- 删除files表的description字段
CREATE TABLE files_new (LIKE files INCLUDING ALL);
ALTER TABLE files_new DROP COLUMN description;
INSERT INTO files_new SELECT id, type, path, reference_count, group_id FROM files;
DROP TABLE files;
ALTER TABLE files_new RENAME TO files;
