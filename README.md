# FileClassificationSolutions

## Design

An innovative file classification storage technology designed to provide users with an efficient and convenient way to manage and retrieve various types of file data, such as stickers, text files, or other abstract data types. This technology establishes a non-file system based database architecture, allowing files to be categorized and quickly retrieved based on their content and attributes (such as tags). The main features include:

1. **Database Structure**: Utilizes three core tables (file table, tag table, and file-tag relationship table) to store file information, tags, and the many-to-many relationships between them.
2. **File Management**: Files are stored with unique identifiers (such as UUIDs) and record their type, location, name, and access statistics (such as click count and share count).
3. **Tagging System**: Organizes and categorizes files through tags, enabling users to quickly find related files based on specific tags.
4. **Flexible Retrieval**: Users can retrieve files by tags or query all related tags for a specific file, achieving efficient data retrieval.
5. **Scalability**: This technology is applicable not only to stickers but can also be extended to other file types, providing a universal file management and search solution.

### Database Table Structure Design

Based on your requirements and the sample data provided, we can design the following database table structure:

1. `files` table: to store basic information about files.
2. `tags` table: to store information about tags.
3. `file_tags` table: to store the many-to-many relationship between files and tags.

Here are the specific SQL create table statements:

```sql
CREATE TABLE files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,   -- 文件类型
    location TEXT NOT NULL, -- 文件存储位置
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
```

### Data Operations

#### Inserting Data

Example of inserting files and tags:

```sql
-- Inserting a file
INSERT INTO files (id, type, location, name, clickCount, shareCount, createTime, modifyTime)
VALUES ('0fa89015-73dd-4fd6-9e50-370ae0a97ea2', 'sticker', 'path/to/sticker', 'Songbird', 3, 0, 1726384526489, 1726384566175);

-- Inserting tags
INSERT INTO tags (name) VALUES ('Honkai Impact 3rd'), ('Songbird');
```

#### Querying Data

##### Retrieving Files by Tag

```sql
SELECT f.id AS file_id, f.type, f.location, f.name
FROM files f
JOIN file_tags ft ON f.id = ft.file_id
JOIN tags t ON ft.tag_id = t.id
WHERE t.name = 'specified_tag_name';
```

##### Retrieving Tags by File

```sql
SELECT t.id AS tag_id, t.name AS tag_name
FROM files f
JOIN file_tags ft ON f.id = ft.file_id
JOIN tags t ON ft.tag_id = t.id
WHERE f.id = 'specified_file_id';
```

### Updated Ideas

1. **Support for Multiple File Types**: Your idea has been extended to support various file types, which can be implemented through the `type` field.
2. **Data Indexing**: To improve query efficiency, consider creating indexes on the `tags.name` and `files.type` fields.
3. **Data Backup and Recovery**: When designing the system, consider implementing mechanisms for data backup and recovery to ensure data security.
4. **User Permissions Management**: If the system is open to multiple users, consider implementing user permissions management to ensure users can only access and manipulate their own files.
5. **File Version Control**: You could add version control functionality to keep track of file modification history.
6. **File Search Optimization**: In addition to tag-based search, you could implement full-text search based on file content to enhance search accuracy.

## API Design
