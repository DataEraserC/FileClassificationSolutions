-- 回滚：恢复 groups 表的 parent_id 冗余列
ALTER TABLE "groups" ADD COLUMN parent_id INTEGER REFERENCES "groups"(id);
CREATE INDEX IF NOT EXISTS idx_groups_parent ON "groups"(parent_id);
