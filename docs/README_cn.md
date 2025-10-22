# FileClassificationSolutions

## 项目概述

FileClassificationSolutions 是一个基于 Rust 语言开发的创新文件分类存储系统，旨在为用户提供高效、便捷的方式来管理和检索各类文件数据，如表情包、文本文件或其他抽象数据类型。该系统通过建立一个非文件系统类的数据库架构，使得文件可以根据其内容和属性（如标签和分组）进行分类和快速检索。

## 核心特性

- **多维度分类**: 通过组（`groups`）和标签（`tags`）实现文件的多维度分类管理
- **引用计数机制**: 自动跟踪文件、组和标签之间的关联关系，确保数据一致性
- **灵活查询**: 支持基于多种条件的复杂查询，包括等于、大于、小于、LIKE 模式匹配等
- **批量操作**: 支持基于条件的批量更新和删除操作
- **主组概念**: 区分主组和普通组，主组与文件具有一对一关系
- **多种访问方式**: 提供命令行界面（CLI）和 Web API 两种访问方式

## 系统架构

项目采用模块化架构设计，包含以下核心组件：

### 核心库 (file_classification_core)

这是整个项目的业务逻辑核心，包含了数据模型、数据库访问层和业务服务。

### 命令行界面 (file_classification_cli)

提供交互式命令行工具来操作文件分类系统，包含完整的增删改查功能。

### Web API (file_classification_webapi)

基于 Actix-web 框架构建的 RESTful API 服务，提供 HTTP 接口进行数据操作。

## 数据库设计

### 核心表结构

系统使用以下 6 个核心表来存储数据：

1. `files` 表：存储文件的基本信息
2. `groups` 表：存储文件分组信息
3. `file_groups` 表：存储文件与分组的多对多关系
4. `tags` 表：存储标签信息
5. `group_tags` 表：存储分组与标签的多对多关系
6. `group_relations` 表：存储分组间的层级关系

### 表结构详情

```sql
CREATE TABLE IF NOT EXISTS files (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,   -- 文件类型
    path TEXT NOT NULL, -- 文件存储位置
    reference_count INTEGER NOT NULL DEFAULT 0, -- 引用计数
    group_id INTEGER NOT NULL, -- 默认的文件组ID
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

CREATE TABLE IF NOT EXISTS groups (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE, -- 文件组名
    reference_count INTEGER NOT NULL DEFAULT 0, -- 引用计数
    is_primary BOOLEAN NOT NULL DEFAULT false, -- 是否为主文件组，false表示否，true表示是 主文件组代表文件元数据
    click_count INTEGER NOT NULL DEFAULT 0, -- 点击次数
    share_count INTEGER NOT NULL DEFAULT 0, -- 分享次数
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    modify_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 修改时间
    parent_id INTEGER REFERENCES groups(id)  -- 父组ID，用于层级结构
);

CREATE TABLE IF NOT EXISTS file_groups (
    file_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    relation_type INTEGER NOT NULL DEFAULT 1, -- 关系类型：1表示主组关系
    PRIMARY KEY (file_id, group_id),
    FOREIGN KEY (file_id) REFERENCES files(id),
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

CREATE TABLE IF NOT EXISTS tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    reference_count INTEGER NOT NULL DEFAULT 0, -- 引用计数
    name TEXT NOT NULL UNIQUE -- 标签名称，唯一
);

CREATE TABLE IF NOT EXISTS group_tags (
    group_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (group_id, tag_id),
    FOREIGN KEY (group_id) REFERENCES groups(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

CREATE TABLE IF NOT EXISTS group_relations (
    first_group_id INTEGER NOT NULL,
    second_group_id INTEGER NOT NULL,
    relation_type INTEGER NOT NULL DEFAULT 1, -- 关系类型：1表示父子关系
    PRIMARY KEY (first_group_id, second_group_id, relation_type),
    FOREIGN KEY (first_group_id) REFERENCES groups(id),
    FOREIGN KEY (second_group_id) REFERENCES groups(id)
);
```


## 功能详解

### 文件管理

- 文件通过唯一标识符（ID）进行存储
- 记录文件类型（`type`）和存储路径（`path`）
- 自动维护引用计数（`reference_count`）
- 与组建立关联关系

### 分组系统

- 支持创建具有唯一名称的文件组
- 区分主组（`is_primary`）和普通组
- 每个文件有记录详细信息的主文件组
- 跟踪组的使用统计（点击次数、分享次数）
- 自动维护引用计数
- 记录创建和修改时间

### 标签系统

- 支持创建具有唯一名称的标签
- 通过标签来组织和分类文件组
- 自动维护引用计数

### 查询功能

#### 基本查询

支持基于各种字段的精确查询：
- 文件：ID、类型、路径、引用计数、组ID
- 组：ID、名称、引用计数、是否为主组、点击次数、分享次数、创建时间、修改时间
- 标签：ID、名称、引用计数

#### 高级查询

支持复杂的条件查询：
- 范围查询：大于、小于
- 模式匹配：LIKE 查询
- 组合条件：AND、OR、NOT 逻辑组合
- 批量查询：IN 条件

#### 关联查询

- 根据标签查询相关文件组
- 根据文件查询所属分组
- 根据组查询关联标签

### 更新功能

支持基于条件的批量更新：
- 文件：路径、类型、引用计数、组ID
- 组：名称、引用计数、是否为主组、点击次数、分享次数、时间戳
- 标签：名称、引用计数

### 删除功能

支持安全的数据删除：
- 自动处理引用计数的减少
- 级联删除相关联的数据
- 主组删除时自动清理关联文件

## 使用方式

### 命令行界面

提供多种独立的命令行工具：

```bash
# 文件操作
list_files                           # 列出文件
list_files_by_conditions            # 条件查询文件
create_file                         # 创建文件
delete_file                         # 删除文件
update_files_by_conditions          # 条件更新文件

# 组操作
list_groups                         # 列出组
list_groups_by_conditions           # 条件查询组
create_group                        # 创建组
delete_group                        # 删除组
update_groups_by_conditions         # 条件更新组

# 标签操作
list_tags                           # 列出标签
list_tags_by_conditions             # 条件查询标签
create_tag                          # 创建标签
delete_tag                          # 删除标签
update_tags_by_conditions           # 条件更新标签

# 关联操作
list_file_groups_by_conditions      # 条件查询文件组关联
create_file_group                   # 创建文件组关联
delete_file_group                   # 删除文件组关联
list_group_tags_by_conditions       # 条件查询组标签关联
create_group_tag                    # 创建组标签关联
delete_group_tag                    # 删除组标签关联
```


### Web API

提供 RESTful API 接口：

```
# 文件相关
GET    /api/files                  # 查询文件
POST   /api/files/search           # 条件查询文件
PUT    /api/files                  # 条件更新文件
DELETE /api/files/{id}             # 删除文件

# 组相关
GET    /api/groups                 # 查询组
POST   /api/groups/search          # 条件查询组
POST   /api/groups                 # 创建组
PUT    /api/groups                 # 条件更新组
DELETE /api/groups/{id}            # 删除组

# 标签相关
GET    /api/tags                   # 查询标签
POST   /api/tags/search            # 条件查询标签
POST   /api/tags                   # 创建标签
PUT    /api/tags                   # 条件更新标签
DELETE /api/tags/{id}              # 删除标签

# 文件组关联相关
GET    /api/file-groups            # 条件查询文件组关联
POST   /api/file-groups            # 创建文件组关联
DELETE /api/file-groups            # 删除文件组关联

# 组标签关联相关
GET    /api/group-tags             # 条件查询组标签关联
POST   /api/group-tags             # 创建组标签关联
DELETE /api/group-tags             # 删除组标签关联
```


## 技术特点

### 架构优势

1. **分层架构**: 清晰分离数据访问层、业务逻辑层和表示层
2. **模块化设计**: 不同功能模块组织在独立的 crate 中
3. **强类型安全**: 利用 Rust 的类型系统保证代码安全
4. **错误处理**: 统一的错误处理机制
5. **数据库抽象**: 使用 Diesel ORM 进行数据库操作
6. **可扩展性**: 易于添加新的功能模块和访问接口

### 数据一致性

1. **事务处理**: 关键操作使用数据库事务确保一致性
2. **引用计数**: 自动维护实体间的引用关系
3. **级联操作**: 删除操作时自动清理关联数据

### 性能优化

1. **查询优化**: 支持复杂条件查询和排序
2. **批量操作**: 支持批量更新和删除
3. **连接池**: Web API 使用数据库连接池提高性能

## 未来规划

1. **多种文件类型支持**: 扩展支持更多文件类型
2. **数据索引优化**: 对常用查询字段建立索引
3. **数据备份与恢复**: 实现数据的备份和恢复机制
4. **用户权限管理**: 支持多用户和权限控制
5. **文件版本控制**: 增加文件版本管理功能
6. **全文搜索**: 实现基于文件内容的全文搜索
7. **多种数据库兼容**: 支持更多数据库类型
8. **批量操作增强**: 提供更丰富的批量操作功能

## 适用场景

- 表情包管理
- 文档分类存储
- 图片素材管理
- 代码片段组织
- 通用文件管理系统

该系统不仅适用于特定类型的文件管理，还可扩展为通用的文件管理和搜索解决方案。