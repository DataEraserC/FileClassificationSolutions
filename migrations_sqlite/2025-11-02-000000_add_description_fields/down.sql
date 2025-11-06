-- 删除tags表的description字段
CREATE TABLE tags_new (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    reference_count INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL UNIQUE
);
INSERT INTO tags_new SELECT id, reference_count, name FROM tags;
DROP TABLE tags;
ALTER TABLE tags_new RENAME TO tags;

-- 删除groups表的description字段
CREATE TABLE groups_new (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    reference_count INTEGER NOT NULL DEFAULT 0,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    click_count INTEGER NOT NULL DEFAULT 0,
    share_count INTEGER NOT NULL DEFAULT 0,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modify_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    parent_id INTEGER REFERENCES groups(id)
);
INSERT INTO groups_new SELECT id, name, reference_count, is_primary, click_count, share_count, create_time, modify_time, parent_id FROM groups;
DROP TABLE groups;
ALTER TABLE groups_new RENAME TO groups;

-- 删除files表的description字段
CREATE TABLE files_new (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,
    path TEXT NOT NULL,
    reference_count INTEGER NOT NULL DEFAULT 0,
    group_id INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES groups(id)
);
INSERT INTO files_new SELECT id, type, path, reference_count, group_id FROM files;
DROP TABLE files;
ALTER TABLE files_new RENAME TO files;
