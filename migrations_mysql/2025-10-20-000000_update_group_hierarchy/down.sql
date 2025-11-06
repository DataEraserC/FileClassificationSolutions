-- 回滚数据库变更
DROP INDEX IF EXISTS idx_group_relations_first;
DROP INDEX IF EXISTS idx_group_relations_second;
DROP INDEX IF EXISTS idx_groups_parent;

ALTER TABLE `file_groups` DROP COLUMN relation_type;
ALTER TABLE `groups` DROP COLUMN parent_id;
DROP TABLE `group_relations`;
