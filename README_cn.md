# FileClassificationSolutions

## 设计

一种创新的文件分类存储技术，旨在为用户提供一个高效、便捷的方式来管理和检索各类文件数据，如表情包、文本文件或其他抽象数据类型。该技术通过建立一个非文件系统类的数据库架构，使得文件可以根据其内容和属性（如标签和分组）进行分类和快速检索。主要特点包括：

1. **数据库结构**：使用多个核心表（文件表、分组表、文件-分组关系表、标签表和分组-标签关系表）来存储文件信息、标签以及它们之间的多对多关系。
2. **文件管理**：文件通过唯一标识符（如整数ID）进行存储，并记录其类型和位置。
3. **分组系统**：允许将文件分类到不同的分组中，并支持主分组的标识。
4. **标签系统**：通过标签来组织和分类文件，使得用户可以根据特定标签快速找到相关文件。
5. **灵活检索**：用户可以通过标签或分组检索文件，实现高效的数据检索。
6. **扩展性**：该技术不仅适用于表情包，还可扩展到其他文件类型，提供通用的文件管理和搜索解决方案。

### 数据库表结构设计

基于您的需求，以下是具体的数据库表结构：

1. `files` 表：用于存储文件的基本信息。
2. `groups` 表：用于存储文件分组信息。
3. `file_groups` 表：用于存储文件与分组的多对多关系。
4. `tags` 表：用于存储标签信息。
5. `group_tags` 表：用于存储分组与标签的多对多关系。

以下是具体的 SQL 创建表语句：

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
```

### 数据操作

#### 插入数据

插入文件、分组和标签的示例：

```sql
-- 插入文件
INSERT INTO files (id, type, path)
VALUES (1, 'sticker', 'path/to/sticker');

-- 插入分组
INSERT INTO groups (id, name, is_primary, create_time, modify_time)
VALUES (1, '主分组', TRUE, 1726384526489, 1726384566175);

-- 插入标签
INSERT INTO tags (id, name) VALUES (1, '崩坏三'), (2, '松雀');
```

```sql
-- 插入文件和分组的关系
INSERT INTO file_groups (file_id, group_id)
VALUES (1, 1);
```

```sql
-- 插入分组和标签的关系
INSERT INTO group_tags (group_id, tag_id)
VALUES (1, 1), (1, 2);
```

#### 查询数据

##### 根据标签查询文件

```sql
SELECT f.id AS file_id, f.type, f.path
FROM files f
JOIN file_groups fg ON f.id = fg.file_id
JOIN group_tags gt ON fg.group_id = gt.group_id
JOIN tags t ON gt.tag_id = t.id
WHERE t.name = 'specified_tag_name';
```

##### 根据文件查询分组

```sql
SELECT g.id AS group_id, g.name
FROM groups g
JOIN file_groups fg ON g.id = fg.group_id
WHERE fg.file_id = 'specified_file_id';
```

### 更新想法

1. **支持多种文件类型**：扩展支持多种文件类型，通过 `type` 字段实现。
2. **数据索引**：可以考虑对 `tags.name` 和 `groups.name` 字段建立索引，以提高查询效率。
3. **数据备份与恢复**：在设计系统时，应考虑数据的备份和恢复机制，以保证数据的安全。
4. **用户权限管理**：如果系统对多用户开放，需要考虑用户权限管理，确保用户只能访问和操作自己的文件。
5. **文件版本控制**：可以增加版本控制功能，记录文件的修改历史。
6. **文件搜索优化**：除了标签搜索，还可以实现基于文件内容的全文搜索，提高搜索的准确性。
