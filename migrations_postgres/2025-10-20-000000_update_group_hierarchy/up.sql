-- 创建组关系表
CREATE TABLE group_relations (
    first_group_id INTEGER NOT NULL REFERENCES groups(id),
    second_group_id INTEGER NOT NULL REFERENCES groups(id),
    relation_type INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (first_group_id, second_group_id, relation_type)
);

-- 扩展组表结构
ALTER TABLE groups ADD COLUMN parent_id INTEGER REFERENCES groups(id);

-- 扩展文件组关系表
ALTER TABLE file_groups ADD COLUMN relation_type INTEGER NOT NULL DEFAULT 1;

-- 创建查询优化索引
CREATE INDEX IF NOT EXISTS idx_group_relations_first ON group_relations(first_group_id);
CREATE INDEX IF NOT EXISTS idx_group_relations_second ON group_relations(second_group_id);
CREATE INDEX IF NOT EXISTS idx_groups_parent ON groups(parent_id);

-- 更新现有数据，为所有file_groups记录设置relation_type为1（主组关系）
UPDATE file_groups SET relation_type = 1;
