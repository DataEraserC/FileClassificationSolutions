CREATE TABLE files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,   -- 文件类型
    path TEXT NOT NULL, -- 文件存储位置
    reference_count INTEGER DEFAULT 0, -- 引用计数
    group_id INTEGER, -- 默认的文件组ID
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    reference_count INTEGER DEFAULT 0, -- 引用计数
    name TEXT NOT NULL UNIQUE -- 标签名称，唯一
);

CREATE TABLE group_tags (
    group_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (group_id, tag_id),
    FOREIGN KEY (group_id) REFERENCES groups(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

CREATE TABLE groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL, -- 文件组名
    is_primary BOOLEAN NOT NULL DEFAULT 0, -- 是否为本命文件组，0表示否，1表示是
    clickCount INTEGER DEFAULT 0, -- 点击次数
    shareCount INTEGER DEFAULT 0, -- 分享次数
    createTime INTEGER NOT NULL, -- 创建时间
    modifyTime INTEGER NOT NULL  -- 修改时间
);

CREATE TABLE file_groups (
    file_id INTEGER,
    group_id INTEGER,
    PRIMARY KEY (file_id, group_id),
    FOREIGN KEY (file_id) REFERENCES files(id),
    FOREIGN KEY (group_id) REFERENCES groups(id)
);