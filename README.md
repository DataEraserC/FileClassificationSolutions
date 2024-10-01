# File Classification Solutions

## Design

An innovative file classification storage technology designed to provide users with an efficient and convenient way to manage and retrieve various file types, such as stickers, text files, or other abstract data types. This technology establishes a non-file-system-based database architecture, allowing files to be categorized and quickly retrieved based on their content and attributes (such as tags and groups). Key features include:

1. **Database Structure**: Utilizes several core tables (files table, groups table, file-groups relationship table, tags table, and group-tags relationship table) to store file information, tags, and their many-to-many relationships.
2. **File Management**: Files are stored using a unique identifier (such as an integer ID) and keep track of their type and path.
3. **Grouping System**: Allows files to be categorized into different groups, supporting the identification of primary groups.
4. **Tagging System**: Organizes and classifies files through tags, enabling users to quickly find related files based on specific tags.
5. **Flexible Retrieval**: Users can search for files by tags or groups, facilitating efficient data retrieval.
6. **Extensibility**: This technology is not only suitable for stickers but can also be extended to other file types, providing a general file management and search solution.

### Database Table Structure Design

Based on your requirements, here’s the specific database table structure:

1. `files` table: Stores basic information about files.
2. `groups` table: Stores information about file groups.
3. `file_groups` table: Stores the many-to-many relationships between files and groups.
4. `tags` table: Stores information about tags.
5. `group_tags` table: Stores the many-to-many relationships between groups and tags.

Here are the SQL create table statements:

```sql
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
    name TEXT NOT NULL, -- 文件组名
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
```

### Data Operations

#### Inserting Data

Example of inserting files, groups, and tags:

```sql
-- Insert a file
INSERT INTO files (id, type, path)
VALUES (1, 'sticker', 'path/to/sticker');

-- Insert a group
INSERT INTO groups (id, name, is_primary, create_time, modify_time)
VALUES (1, 'Primary Group', TRUE, 1726384526489, 1726384566175);

-- Insert tags
INSERT INTO tags (id, name) VALUES (1, 'Honkai Impact'), (2, 'Songbird');
```

```sql
-- Insert the relationship between files and groups
INSERT INTO file_groups (file_id, group_id)
VALUES (1, 1);
```

```sql
-- Insert the relationship between groups and tags
INSERT INTO group_tags (group_id, tag_id)
VALUES (1, 1), (1, 2);
```

#### Querying Data

##### Query Files by Tag

```sql
SELECT f.id AS file_id, f.type, f.path
FROM files f
JOIN file_groups fg ON f.id = fg.file_id
JOIN group_tags gt ON fg.group_id = gt.group_id
JOIN tags t ON gt.tag_id = t.id
WHERE t.name = 'specified_tag_name';
```

##### Query Groups by File

```sql
SELECT g.id AS group_id, g.name
FROM groups g
JOIN file_groups fg ON g.id = fg.group_id
WHERE fg.file_id = 'specified_file_id';
```

### Future Considerations

1. **Support for Multiple File Types**: The design allows for support of various file types through the `type` field.
2. **Data Indexing**: Consider creating indexes on the `tags.name` and `groups.name` fields to improve query performance.
3. **Data Backup and Recovery**: Consider implementing data backup and recovery mechanisms to ensure data safety.
4. **User Permission Management**: If the system is open to multiple users, consider user permission management to ensure users can only access and operate on their files.
5. **File Version Control**: Consider adding version control features to track the modification history of files.
6. **File Search Optimization**: In addition to tag searches, implement full-text search capabilities based on file content to improve search accuracy.
