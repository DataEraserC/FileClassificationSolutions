-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "groups" (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    reference_count INTEGER NOT NULL DEFAULT 0,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    click_count INTEGER NOT NULL DEFAULT 0,
    share_count INTEGER NOT NULL DEFAULT 0,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modify_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
COMMENT ON TABLE "groups" IS '组信息表';
COMMENT ON COLUMN "groups".id IS '组ID';
COMMENT ON COLUMN "groups".name IS '组名称';
COMMENT ON COLUMN "groups".reference_count IS '引用计数';
COMMENT ON COLUMN "groups".is_primary IS '是否为主组';
COMMENT ON COLUMN "groups".click_count IS '点击次数';
COMMENT ON COLUMN "groups".share_count IS '分享次数';
COMMENT ON COLUMN "groups".create_time IS '创建时间';
COMMENT ON COLUMN "groups".modify_time IS '修改时间';

CREATE TABLE IF NOT EXISTS "tags" (
    id SERIAL PRIMARY KEY,
    reference_count INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL UNIQUE
);
COMMENT ON TABLE "tags" IS '标签信息表';
COMMENT ON COLUMN "tags".id IS '标签ID';
COMMENT ON COLUMN "tags".reference_count IS '引用计数';
COMMENT ON COLUMN "tags".name IS '标签名称';

CREATE TABLE IF NOT EXISTS "files" (
    id SERIAL PRIMARY KEY,
    type TEXT NOT NULL,
    path TEXT NOT NULL,
    reference_count INTEGER NOT NULL DEFAULT 0,
    group_id INTEGER NOT NULL REFERENCES "groups"(id)
);
COMMENT ON TABLE "files" IS '文件信息表';
COMMENT ON COLUMN "files".id IS '文件ID';
COMMENT ON COLUMN "files".type IS '文件类型';
COMMENT ON COLUMN "files".path IS '文件路径';
COMMENT ON COLUMN "files".reference_count IS '引用计数';
COMMENT ON COLUMN "files".group_id IS '主组ID';

CREATE TABLE IF NOT EXISTS "file_groups" (
    file_id INTEGER NOT NULL REFERENCES "files"(id),
    group_id INTEGER NOT NULL REFERENCES "groups"(id),
    PRIMARY KEY (file_id, group_id)
);
COMMENT ON TABLE "file_groups" IS '文件组关联表';
COMMENT ON COLUMN "file_groups".file_id IS '文件ID';
COMMENT ON COLUMN "file_groups".group_id IS '组ID';

CREATE TABLE IF NOT EXISTS "group_tags" (
    group_id INTEGER NOT NULL REFERENCES "groups"(id),
    tag_id INTEGER NOT NULL REFERENCES "tags"(id),
    PRIMARY KEY (group_id, tag_id)
);
COMMENT ON TABLE "group_tags" IS '组标签关联表';
COMMENT ON COLUMN "group_tags".group_id IS '组ID';
COMMENT ON COLUMN "group_tags".tag_id IS '标签ID';
