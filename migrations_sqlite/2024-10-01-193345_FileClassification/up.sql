-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `groups` (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    reference_count INTEGER NOT NULL DEFAULT 0,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    click_count INTEGER NOT NULL DEFAULT 0,
    share_count INTEGER NOT NULL DEFAULT 0,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modify_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- SQLite 不支持字段注释，以下是注释信息供参考
-- 表名: groups (组信息表)
-- 字段说明:
--   id: 组ID
--   name: 组名称
--   reference_count: 引用计数
--   is_primary: 是否为主组
--   click_count: 点击次数
--   share_count: 分享次数
--   create_time: 创建时间
--   modify_time: 修改时间

CREATE TABLE IF NOT EXISTS `tags` (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    reference_count INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL UNIQUE
);
-- SQLite 不支持字段注释，以下是注释信息供参考
-- 表名: tags (标签信息表)
-- 字段说明:
--   id: 标签ID
--   reference_count: 引用计数
--   name: 标签名称

CREATE TABLE IF NOT EXISTS `files` (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,
    path TEXT NOT NULL,
    reference_count INTEGER NOT NULL DEFAULT 0,
    group_id INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES `groups`(id)
);
-- SQLite 不支持字段注释，以下是注释信息供参考
-- 表名: files (文件信息表)
-- 字段说明:
--   id: 文件ID
--   type: 文件类型
--   path: 文件路径
--   reference_count: 引用计数
--   group_id: 主组ID

CREATE TABLE IF NOT EXISTS `file_groups` (
    file_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    PRIMARY KEY (file_id, group_id),
    FOREIGN KEY (file_id) REFERENCES `files`(id),
    FOREIGN KEY (group_id) REFERENCES `groups`(id)
);
-- SQLite 不支持字段注释，以下是注释信息供参考
-- 表名: file_groups (文件组关联表)
-- 字段说明:
--   file_id: 文件ID
--   group_id: 组ID

CREATE TABLE IF NOT EXISTS `group_tags` (
    group_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (group_id, tag_id),
    FOREIGN KEY (group_id) REFERENCES `groups`(id),
    FOREIGN KEY (tag_id) REFERENCES `tags`(id)
);
-- SQLite 不支持字段注释，以下是注释信息供参考
-- 表名: group_tags (组标签关联表)
-- 字段说明:
--   group_id: 组ID
--   tag_id: 标签ID
