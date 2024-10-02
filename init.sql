CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,   -- 文件类型
    path TEXT NOT NULL, -- 文件存储位置
    reference_count INTEGER DEFAULT 0, -- 引用计数
    group_id INTEGER, -- 默认的文件组ID
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

CREATE TABLE IF NOT EXISTS groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE, -- 文件组名
    is_primary BOOLEAN NOT NULL DEFAULT 0, -- 是否为本命文件组，0表示否，1表示是
    click_count INTEGER DEFAULT 0, -- 点击次数
    share_count INTEGER DEFAULT 0, -- 分享次数
    create_time BigInt NOT NULL, -- 创建时间
    modify_time BigInt NOT NULL  -- 修改时间
);

CREATE TABLE IF NOT EXISTS file_groups (
    file_id INTEGER,
    group_id INTEGER,
    PRIMARY KEY (file_id, group_id),
    FOREIGN KEY (file_id) REFERENCES files(id),
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    reference_count INTEGER DEFAULT 0, -- 引用计数
    name TEXT NOT NULL UNIQUE -- 标签名称，唯一
);

CREATE TABLE IF NOT EXISTS group_tags (
    group_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (group_id, tag_id),
    FOREIGN KEY (group_id) REFERENCES groups(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);