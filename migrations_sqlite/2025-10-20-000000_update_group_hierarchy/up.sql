-- 创建组关系表
CREATE TABLE `group_relations` (
    first_group_id INTEGER NOT NULL,
    second_group_id INTEGER NOT NULL,
    relation_type INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (first_group_id, second_group_id, relation_type),
    FOREIGN KEY (first_group_id) REFERENCES `groups`(id),
    FOREIGN KEY (second_group_id) REFERENCES `groups`(id)
);
-- SQLite 不支持字段注释，以下是注释信息供参考
-- 表名: group_relations (组关系表)
-- 字段说明:
--   first_group_id: 第一个组ID
--   second_group_id: 第二个组ID
--   relation_type: 关系类型

-- 扩展组表结构
ALTER TABLE `groups` ADD COLUMN parent_id INTEGER REFERENCES `groups`(id);
-- SQLite 不支持字段注释，parent_id字段表示父组ID

-- 扩展文件组关系表
ALTER TABLE `file_groups` ADD COLUMN relation_type INTEGER NOT NULL DEFAULT 1;
-- SQLite 不支持字段注释，relation_type字段表示关系类型

-- 创建查询优化索引
CREATE INDEX idx_group_relations_first ON `group_relations`(first_group_id);
CREATE INDEX idx_group_relations_second ON `group_relations`(second_group_id);
CREATE INDEX idx_groups_parent ON `groups`(parent_id);

-- 更新现有数据，为所有file_groups记录设置relation_type为1（主组关系）
UPDATE `file_groups` SET relation_type = 1;
