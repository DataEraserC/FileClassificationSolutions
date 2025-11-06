-- 创建组关系表
CREATE TABLE `group_relations` (
    first_group_id INTEGER NOT NULL,
    second_group_id INTEGER NOT NULL,
    relation_type INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (first_group_id, second_group_id, relation_type),
    FOREIGN KEY (first_group_id) REFERENCES `groups`(id),
    FOREIGN KEY (second_group_id) REFERENCES `groups`(id)
);
-- 表和字段注释 (仅在支持的数据库中有效)
-- COMMENT ON TABLE `group_relations` IS '组关系表';
-- COMMENT ON COLUMN `group_relations`.first_group_id IS '第一个组ID';
-- COMMENT ON COLUMN `group_relations`.second_group_id IS '第二个组ID';
-- COMMENT ON COLUMN `group_relations`.relation_type IS '关系类型';

-- 扩展组表结构
ALTER TABLE `groups` ADD COLUMN parent_id INTEGER REFERENCES `groups`(id);
-- COMMENT ON COLUMN `groups`.parent_id IS '父组ID';

-- 扩展文件组关系表
ALTER TABLE `file_groups` ADD COLUMN relation_type INTEGER NOT NULL DEFAULT 1;
-- COMMENT ON COLUMN `file_groups`.relation_type IS '关系类型';

-- 创建查询优化索引
CREATE INDEX idx_group_relations_first ON `group_relations`(first_group_id);
CREATE INDEX idx_group_relations_second ON `group_relations`(second_group_id);
CREATE INDEX idx_groups_parent ON `groups`(parent_id);

-- 更新现有数据，为所有file_groups记录设置relation_type为1（主组关系）
UPDATE `file_groups` SET relation_type = 1;
