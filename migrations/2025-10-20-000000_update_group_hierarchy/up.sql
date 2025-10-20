-- 创建组关系表
CREATE TABLE group_relations (
    first_group_id INTEGER NOT NULL,
    second_group_id INTEGER NOT NULL,
    relation_type INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (first_group_id, second_group_id, relation_type),
    FOREIGN KEY (first_group_id) REFERENCES groups(id),
    FOREIGN KEY (second_group_id) REFERENCES groups(id)
);

-- 扩展组表结构
ALTER TABLE groups ADD COLUMN parent_id INTEGER REFERENCES groups(id);

-- 扩展文件组关系表
ALTER TABLE file_groups ADD COLUMN relation_type INTEGER NOT NULL DEFAULT 1;

-- 创建查询优化索引
CREATE INDEX idx_group_relations_first ON group_relations(first_group_id);
CREATE INDEX idx_group_relations_second ON group_relations(second_group_id);
CREATE INDEX idx_groups_parent ON groups(parent_id);