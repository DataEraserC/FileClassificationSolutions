-- 先删除引用该列的索引，否则 SQLite 不允许 DROP COLUMN
DROP INDEX IF EXISTS idx_groups_parent;
-- 移除 groups 表的 parent_id 冗余列（父组关系由 group_relations 表维护）
ALTER TABLE `groups` DROP COLUMN parent_id;
