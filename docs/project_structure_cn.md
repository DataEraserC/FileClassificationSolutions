# FileClassificationSolutions 项目结构

这是一个基于 Rust 的文件分类解决方案，采用模块化架构设计，包含核心库、命令行界面和 Web API 等多个组件。

## 项目整体结构

```
FileClassificationSolutions/
├── .github/                    # GitHub 相关配置
│   └── workflows/             # CI/CD 工作流配置
├── file_classification_cli/   # 命令行界面应用
│   ├── src/bin/               # 各个独立的命令行工具
│   └── Cargo.toml             # CLI 包配置
├── file_classification_core/  # 核心库
│   ├── src/
│   │   ├── internal/          # 数据访问层
│   │   ├── model/             # 数据模型和数据库模式
│   │   ├── service/           # 业务逻辑层
│   │   └── utils/             # 工具函数
│   └── Cargo.toml             # 核心库包配置
├── file_classification_webapi/ # Web API
│   ├── src/
│   │   ├── handlers/          # API 请求处理函数
│   │   ├── group_tags.rs      # 组标签关联 API 处理
│   │   ├── groups.rs          # 组 API 处理
│   │   ├── mod.rs             # 模块声明
│   │   └── tags.rs            # 标签 API 处理
│   ├── utils/                 # Web API 工具函数
│   │   ├── database.rs        # 数据库连接池
│   │   ├── mod.rs             # 模块声明
│   │   └── models.rs          # API 数据传输对象
│   └── Cargo.toml             # Web API 包配置
├── migrations/                # 数据库迁移脚本
└── nix/                      # Nix 包管理配置
```


## 核心模块详解

### file_classification_core (核心库)

这是整个项目的业务逻辑核心，包含了数据模型、数据库访问层和业务服务。

#### 目录结构
```
file_classification_core/
├── src/
│   ├── internal/              # 数据访问层 (DAO)
│   │   ├── file_group.rs      # 文件组关联数据访问
│   │   ├── files.rs           # 文件数据访问
│   │   ├── group_tag.rs       # 组标签关联数据访问
│   │   ├── groups.rs          # 组数据访问
│   │   └── tags.rs            # 标签数据访问
│   ├── model/                 # 数据模型
│   │   ├── models.rs          # 所有数据结构定义
│   │   └── schema.rs          # 数据库模式 (由 Diesel 生成)
│   ├── service/               # 业务逻辑层
│   │   ├── file_group.rs      # 文件组关联业务逻辑
│   │   ├── files.rs           # 文件业务逻辑
│   │   ├── group_tag.rs       # 组标签关联业务逻辑
│   │   ├── groups.rs          # 组业务逻辑
│   │   └── tags.rs            # 标签业务逻辑
│   └── utils/                 # 工具函数
│       ├── database.rs        # 数据库连接管理
│       └── errors.rs          # 错误处理
└── Cargo.toml                 # 包配置文件
```


#### 核心概念
- **Files（文件）**: 代表系统中的具体文件，包含类型、路径、引用计数等属性
- **Groups（组）**: 用于对文件进行分类和组织，具有名称、引用计数、主组标识等属性
- **Tags（标签）**: 为组提供额外的元数据描述，增强分类能力
- **FileGroups（文件组关联）**: 建立文件与组之间的多对多关系
- **GroupTags（组标签关联）**: 建立组与标签之间的多对多关系

#### 设计原则
1. 使用引用计数来追踪实体之间的关联关系
2. 支持复杂的条件查询，包括等于、大于、小于、LIKE模式匹配等
3. 实现了完整的CRUD操作，包括通过条件批量操作
4. 采用事务确保数据一致性，特别是在处理引用计数时
5. 区分主组和普通组，主组与文件具有一对一关系

### file_classification_cli (命令行界面)

提供命令行工具来操作文件分类系统。

#### 目录结构
```
file_classification_cli/
├── src/bin/                   # 各个独立的命令行工具
│   ├── create_file_group.rs   # 创建文件组关联
│   ├── create_group.rs        # 创建组
│   ├── create_group_tag.rs    # 创建组标签关联
│   ├── create_tag.rs          # 创建标签
│   ├── delete_file.rs         # 删除文件
│   ├── delete_file_group.rs   # 删除文件组关联
│   ├── delete_group.rs        # 删除组
│   ├── delete_group_tag.rs    # 删除组标签关联
│   ├── delete_tag.rs          # 删除标签
│   ├── list_file_groups_by_conditions.rs  # 条件查询文件组关联
│   ├── list_files.rs          # 列出文件
│   ├── list_files_by_conditions.rs        # 条件查询文件
│   ├── list_group_tags_by_conditions.rs   # 条件查询组标签关联
│   ├── list_groups.rs         # 列出组
│   ├── list_groups_by_conditions.rs       # 条件查询组
│   ├── list_tags.rs           # 列出标签
│   ├── list_tags_by_conditions.rs         # 条件查询标签
│   ├── update_files_by_conditions.rs      # 条件更新文件
│   ├── update_groups_by_conditions.rs     # 条件更新组
│   └── update_tags_by_conditions.rs       # 条件更新标签
└── Cargo.toml                 # 包配置文件
```


#### 功能特点
- 提供交互式命令行界面
- 支持复杂的条件查询和批量操作
- 包含完整的增删改查功能
- 支持组合条件查询（AND、OR、NOT）

### file_classification_webapi (Web API)

基于 Actix-web 框架构建的 RESTful API 服务。

#### 目录结构
```
file_classification_webapi/
├── src/
│   ├── handlers/              # API 请求处理函数
│   │   ├── file_groups.rs     # 文件组关联 API 处理
│   │   ├── files.rs           # 文件 API 处理
│   │   ├── group_tags.rs      # 组标签关联 API 处理
│   │   ├── groups.rs          # 组 API 处理
│   │   ├── mod.rs             # 模块声明
│   │   └── tags.rs            # 标签 API 处理
│   ├── utils/                 # Web API 工具函数
│   │   ├── database.rs        # 数据库连接池
│   │   ├── mod.rs             # 模块声明
│   │   └── models.rs          # API 数据传输对象
│   └── main.rs                # 应用入口点
└── Cargo.toml                 # 包配置文件
```


#### API 端点
- **文件管理**: `/api/files`
- **组管理**: `/api/groups`
- **标签管理**: `/api/tags`
- **文件组关联**: `/api/file-groups`
- **组标签关联**: `/api/group-tags`

### 数据库迁移

```
migrations/
└── 2024-10-01-193345_FileClassification/
    ├── up.sql                 # 数据库表创建脚本
    └── down.sql               # 数据库表删除脚本
```


数据库包含以下表：
- `files`: 存储文件信息
- `groups`: 存储文件组信息
- `file_groups`: 文件和组的多对多关联关系
- `tags`: 存储标签信息
- `group_tags`: 组和标签的多对多关联关系

### 数据模型关系

```mermaid
erDiagram
    files ||--o{ file_groups : has
    groups ||--o{ file_groups : has
    groups ||--o{ group_tags : has
    tags ||--o{ group_tags : has
    groups ||--o{ group_relations : has
    groups ||--o{ group_relations : has
    
    files {
        int id PK
        string type
        string path
        int reference_count
        int group_id FK
    }
    
    groups {
        int id PK
        string name
        int reference_count
        boolean is_primary
        int click_count
        int share_count
        timestamp create_time
        timestamp modify_time
        int parent_id FK
    }
    
    file_groups {
        int file_id PK,FK
        int group_id PK,FK
        int relation_type
    }
    
    tags {
        int id PK
        string name
        int reference_count
    }
    
    group_tags {
        int group_id PK,FK
        int tag_id PK,FK
    }
    
    group_relations {
        int first_group_id PK,FK
        int second_group_id PK,FK
        int relation_type
    }
```


## 架构特点

1. **分层架构**: 项目采用清晰的分层架构，将数据访问、业务逻辑和表示层分离
2. **模块化设计**: 不同的功能模块被组织在独立的 crate 中
3. **多种访问方式**: 提供 CLI 和 Web API 两种访问方式
4. **强类型安全**: 利用 Rust 的类型系统保证代码安全
5. **错误处理**: 统一的错误处理机制
6. **数据库抽象**: 使用 Diesel ORM 进行数据库操作
7. **可扩展性**: 易于添加新的功能模块和访问接口

这个项目结构设计支持对文件进行分类管理，通过组和标签的方式组织文件，并提供多种访问接口以适应不同使用场景。