-- Your SQL goes here
CREATE TABLE IF NOT EXISTS files (
    id SERIAL PRIMARY KEY,
    type TEXT NOT NULL,
    path TEXT NOT NULL,
    reference_count INTEGER NOT NULL DEFAULT 0,
    group_id INTEGER NOT NULL REFERENCES groups(id)
);

CREATE TABLE IF NOT EXISTS groups (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    reference_count INTEGER NOT NULL DEFAULT 0,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    click_count INTEGER NOT NULL DEFAULT 0,
    share_count INTEGER NOT NULL DEFAULT 0,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modify_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS file_groups (
    file_id INTEGER NOT NULL REFERENCES files(id),
    group_id INTEGER NOT NULL REFERENCES groups(id),
    PRIMARY KEY (file_id, group_id)
);

CREATE TABLE IF NOT EXISTS tags (
    id SERIAL PRIMARY KEY,
    reference_count INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS group_tags (
    group_id INTEGER NOT NULL REFERENCES groups(id),
    tag_id INTEGER NOT NULL REFERENCES tags(id),
    PRIMARY KEY (group_id, tag_id)
);
