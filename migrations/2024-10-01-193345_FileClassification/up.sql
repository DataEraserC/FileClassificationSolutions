-- Your SQL goes here
CREATE TABLE IF NOT EXISTS files (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,
    path TEXT NOT NULL,
    reference_count INTEGER NOT NULL DEFAULT 0,
    group_id INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

CREATE TABLE IF NOT EXISTS groups (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    is_primary BOOLEAN NOT NULL DEFAULT 0,
    click_count INTEGER NOT NULL DEFAULT 0,
    share_count INTEGER NOT NULL DEFAULT 0,
    create_time BigInt NOT NULL,
    modify_time BigInt NOT NULL
);

CREATE TABLE IF NOT EXISTS file_groups (
    file_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    PRIMARY KEY (file_id, group_id),
    FOREIGN KEY (file_id) REFERENCES files(id),
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

CREATE TABLE IF NOT EXISTS tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    reference_count INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS group_tags (
    group_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (group_id, tag_id),
    FOREIGN KEY (group_id) REFERENCES groups(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);
