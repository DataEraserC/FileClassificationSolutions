# FileClassificationSolutions 贡献代码规范

## 目录

- [1. 代码风格规范](#1-代码风格规范)
  - [1.1 Rust 代码格式化](#11-rust-代码格式化)
  - [1.2 JavaScript 代码格式化](#12-javascript-代码格式化)
  - [1.3 命名规范](#13-命名规范)
  - [1.4 注释规范](#14-注释规范)
- [2. 项目架构规范](#2-项目架构规范)
  - [2.1 分层架构](#21-分层架构)
  - [2.2 模块组织](#22-模块组织)
  - [2.3 依赖管理](#23-依赖管理)
- [3. 开发流程规范](#3-开发流程规范)
  - [3.1 Git 工作流](#31-git-工作流)
  - [3.2 提交信息规范](#32-提交信息规范)
  - [3.3 测试规范](#33-测试规范)
- [4. 数据库规范](#4-数据库规范)
  - [4.1 Migration 规范](#41-migration-规范)
  - [4.2 SQL 编写规范](#42-sql-编写规范)
- [5. API 设计规范](#5-api-设计规范)
  - [5.1 RESTful 规范](#51-restful-规范)
  - [5.2 错误处理](#52-错误处理)
- [6. 文档规范](#6-文档规范)
- [7. 安全检查清单](#7-安全检查清单)

---

## 1. 代码风格规范

### 1.1 Rust 代码格式化

项目使用 `rustfmt` 进行代码格式化，配置文件为 `rustfmt.toml`：

```toml
edition = "2024"
style_edition = "2024"
tab_spaces = 2
hard_tabs = false
newline_style = "Unix"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
reorder_modules = true
reorder_impl_items = true
use_field_init_shorthand = true
use_try_shorthand = true
wrap_comments = true
normalize_comments = true
```

**强制要求：**
- 使用 2 个空格缩进
- 导入语句按组排序：标准库 → 外部 crate → 内部模块
- 模块声明需要重新排序
- 使用字段初始化简写语法
- 使用 try 简写语法（`?` 操作符）
- 注释自动换行和规范化

**使用方法：**
```bash
cargo fmt
```

### 1.2 JavaScript 代码格式化

前端 JavaScript 相关文件推荐使用 Prettier 进行格式化，建议配置文件为 `.prettierrc`：

```json
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5",
  "printWidth": 100
}
```

**强制要求：**
- 使用 2 个空格缩进
- 语句末尾添加分号
- 字符串使用单引号
- 行宽限制 100 字符

**使用方法：**
```bash
# 安装 Prettier
npm install --save-dev prettier

# 格式化 JavaScript 文件
npx prettier --write static/js/
```

### 1.3 命名规范

#### 变量和函数命名
- **变量名**：使用 `snake_case`（蛇形命名法）
  ```rust
  let file_id = 123;
  let group_name = "Documents";
  ```

- **函数名**：使用 `snake_case`
  ```rust
  pub fn create_group() { }
  pub fn delete_file_by_id() { }
  ```

- **类型和结构体**：使用 `PascalCase`（驼峰命名法）
  ```rust
  pub struct File { }
  pub struct CreateGroupDTO { }
  ```

- **枚举**：使用 `PascalCase`
  ```rust
  pub enum GroupCondition { }
  pub enum OrderDirection { }
  ```

- **常量**：使用 `SCREAMING_SNAKE_CASE`
  ```rust
  pub const RELATION_TYPE_PARENT_CHILD: i32 = 1;
  ```

#### 文件命名
- Rust 源文件：小写字母 + 下划线分隔
  - ✅ `file_groups.rs`, `group_relations.rs`
  - ❌ `FileGroups.rs`, `filegroups.rs`

- 测试文件：与被测试模块同名，放在 `tests/` 目录
  ```
  file_classification_cli/
  ├── src/
  │   └── cli.rs
  └── tests/
      └── cli.rs
  ```

### 1.4 注释规范

#### 模块级文档注释
每个模块文件开头必须包含模块级文档注释，说明模块用途：

```rust
// groups.rs
//! 分组服务模块
//!
//! 提供分组相关的业务逻辑处理，包括分组的创建、删除、查询和更新操作，
//! 并处理分组与其关联文件、标签等资源的引用计数和级联删除。
```

#### 函数文档注释
公共函数必须包含完整的文档注释：

```rust
/// 通过名称创建分组
///
/// 参数:
/// - `conn`: 数据库连接对象
/// - `create_group_dto`: 包含分组信息的 DTO 对象
///
/// 返回值:
/// 成功时返回插入记录的 ID，失败则返回相应的错误
pub fn create_group(
  conn: &mut AnyConnection,
  create_group_dto: &CreateGroupDTO,
) -> Result<i32, Error> {
  // ...
}
```

#### 内联注释
- 使用 `//` 添加单行注释，注释前留一个空格
- 复杂逻辑需要添加注释说明意图
- 避免无意义的注释（代码已经很清晰的情况）

```rust
// ✅ 好的注释
// 对于非主组，需要先减少关联文件的引用计数
for file in &files_associated_with_group {
  files_dao::decrease_file_reference_count_by_id(conn, file.id)?;
}

// ❌ 避免的注释
// 循环遍历文件
for file in &files {
  // 删除文件
  delete_file(file)?;
}
```

#### 语言要求
- **所有注释必须使用中文**（特殊业务逻辑可用英文术语）
- 文档字符串保持中英文双语版本（如 README）

---

## 2. 项目架构规范

### 2.1 分层架构

项目采用清晰的三层架构：

```
┌─────────────────────────────────────┐
│     Presentation Layer (Web/CLI)    │
│  - handlers/ (API 请求处理器)         │
│  - cli.rs (命令行解析)                │
│  - repl.rs (交互式界面)               │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│      Business Logic Layer (Core)    │
│  - service/ (业务逻辑层)              │
│  - model/ (数据模型层)                │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│      Data Access Layer (Core)       │
│  - internal/ (DAO 数据访问层)         │
│  - utils/database.rs (数据库工具)     │
└─────────────────────────────────────┘
```

**层级依赖规则：**
- ✅ Web/CLI → Core (service → model → internal)
- ❌ internal → service (禁止反向依赖)
- ❌ CLI ↔ WebAPI (禁止横向依赖)

### 2.2 模块组织

#### Core 模块结构
```
file_classification_core/
├── src/
│   ├── internal/          # 数据访问层（私有）
│   │   ├── files.rs       # 文件 DAO
│   │   ├── groups.rs      # 分组 DAO
│   │   ├── tags.rs        # 标签 DAO
│   │   ├── file_group.rs  # 文件 - 分组关联 DAO
│   │   ├── group_tag.rs   # 分组 - 标签关联 DAO
│   │   └── group_relations.rs  # 分组关系 DAO
│   ├── model/             # 数据模型层（公开）
│   │   ├── models.rs      # 所有模型定义
│   │   └── schema.rs      # Diesel Schema
│   ├── service/           # 业务逻辑层（公开）
│   │   ├── files.rs       # 文件服务
│   │   ├── groups.rs      # 分组服务
│   │   └── ...
│   ├── utils/             # 工具模块
│   │   ├── database.rs    # 数据库连接
│   │   └── errors.rs      # 错误处理
│   └── lib.rs             # 库入口
```

#### 可见性规则
- `internal/` 模块标记为 `mod`（私有），不对外暴露
- `model/` 和 `service/` 标记为 `pub mod`（公开）
- 外部只能通过 `service/` 提供的接口访问数据

```rust
// lib.rs
mod internal;        // ❌ 外部不可见
pub mod model;       // ✅ 外部可见
pub mod service;     // ✅ 外部可见
pub mod utils;       // ✅ 外部可见
```

### 2.3 依赖管理

#### Workspace 配置
```toml
[workspace]
resolver = "2"
members = ["file_classification_*", "common"]
```

#### 依赖版本管理
统一在根 `Cargo.toml` 的 `[workspace.dependencies]` 中定义：

```toml
[workspace.dependencies]
diesel = { version = "2.2", features = ["chrono", "r2d2"] }
diesel_migrations = "2.2"
dotenvy = "0.15"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0.219", features = ["derive"] }
```

子项目引用方式：
```toml
[dependencies]
diesel.workspace = true
chrono.workspace = true
```

#### 禁止事项
- ❌ 禁止在子项目中直接指定版本
- ❌ 禁止循环依赖
- ❌ 禁止引入未使用的依赖

---

## 3. 开发流程规范

### 3.1 Git 工作流

#### 分支命名
- `main` / `master`: 主分支，用于发布
- `feature/*`: 功能分支
  ```bash
  git checkout -b feature/add-file-upload
  git checkout -b feature/group-hierarchy
  ```
- `bugfix/*`: 修复分支
  ```bash
  git checkout -b bugfix/primary-group-delete
  ```
- `release/*`: 发布分支
  ```bash
  git checkout -b release/v1.2.0
  ```

#### 提交流程
```bash
# 1. 从主分支创建功能分支
git checkout main
git pull origin main
git checkout -b feature/your-feature

# 2. 开发并提交（遵循提交规范）
git add .
git commit -m "feat: add new file upload endpoint"

# 3. 同步主分支变更
git fetch origin
git rebase origin/main

# 4. 推送并创建 Pull Request
git push origin feature/your-feature
```

### 3.2 提交信息规范

采用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### Type 类型
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整（不影响功能）
- `refactor`: 重构（既不是新功能也不是修复）
- `test`: 添加或修改测试
- `chore`: 构建过程或辅助工具变动

#### Scope 范围
- `core`: 核心库
- `cli`: 命令行工具
- `webapi`: Web API
- `db`: 数据库相关
- `ui`: 前端界面

#### 示例
```bash
# ✅ 好的提交
feat(core): add group tree structure support
fix(cli): resolve primary group deletion issue
docs(readme): update Chinese translation
refactor(webapi): optimize error handling logic
test(core): add unit tests for file service

# ❌ 不好的提交
update code
fix bug
add new feature
```

### 3.3 测试规范

#### 单元测试
为所有公共函数编写单元测试：

```rust
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_group() {
    // Arrange
    let mut conn = setup_test_db();
    let dto = CreateGroupDTO { name: "Test".to_string() };

    // Act
    let result = create_group(&mut conn, &dto);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
  }
}
```

#### 集成测试
在 `tests/` 目录添加集成测试：

```rust
// tests/cli.rs
#[test]
fn test_file_commands() {
  assert!(true)
}
```

#### 运行测试
```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test --package file_classification_core

# 运行带过滤的测试
cargo test test_create_group
```

#### CI/CD 要求
所有 PR 必须通过 GitHub Actions 的自动化测试：
- ✅ Linux (Ubuntu) 测试通过
- ✅ Windows 测试通过
- ✅ macOS 测试通过
- ✅ 代码格式化检查通过

---

## 4. 数据库规范

### 4.1 Migration 规范

#### 目录结构
支持多种数据库，每个数据库有独立的 migration 目录：
```
migrations/                # Diesel 默认（SQLite）
migrations_mysql/          # MySQL
migrations_postgres/       # PostgreSQL
migrations_sqlite/         # SQLite
```

#### Migration 命名
格式：`YYYY-MM-DD-HHMMSS_description`

```bash
✅ 2024-10-01-193345_FileClassification
✅ 2025-10-20-000000_update_group_hierarchy
✅ 2025-11-02-000000_add_description_fields
```

#### 文件要求
每个 migration 必须包含：
- `up.sql`: 升级脚本
- `down.sql`: 降级脚本（可回滚）

```sql
-- up.sql
CREATE TABLE IF NOT EXISTS `groups` (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- down.sql
DROP TABLE IF EXISTS `groups`;
```

#### 多数据库兼容
不同数据库的 SQL 语法差异需要在对应目录分别维护：

```bash
# SQLite (migrations_sqlite/)
CREATE TABLE IF NOT EXISTS `groups` (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT
);

# MySQL (migrations_mysql/)
CREATE TABLE IF NOT EXISTS `groups` (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY
);

# PostgreSQL (migrations_postgres/)
CREATE TABLE IF NOT EXISTS groups (
    id SERIAL PRIMARY KEY
);
```

### 4.2 SQL 编写规范

#### 表名和字段名
- 使用 `snake_case`
- 表名使用复数形式
- 外键使用单数形式 + `_id` 后缀

```sql
-- ✅ 好的命名
CREATE TABLE files (
    id INTEGER PRIMARY KEY,
    type TEXT NOT NULL,
    group_id INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

-- ❌ 避免的命名
CREATE TABLE FileList (
    FileID INTEGER PRIMARY KEY,
    FileType TEXT,
    GroupID INTEGER
);
```

#### 索引优化
为频繁查询的字段添加索引：

```sql
CREATE INDEX idx_files_group_id ON files(group_id);
CREATE INDEX idx_groups_name ON groups(name);
```

#### 注释规范
SQL 文件中添加表和字段注释：

```sql
CREATE TABLE `groups` (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);
-- COMMENT ON TABLE `groups` IS '组信息表';
-- COMMENT ON COLUMN `groups`.id IS '组 ID';
-- COMMENT ON COLUMN `groups`.name IS '组名称';
```

---

## 5. API 设计规范

### 5.1 RESTful 规范

#### 资源命名
- 使用名词复数形式
- 使用小写字母 + 连字符

```
✅ /api/files
✅ /api/groups
✅ /api/file-groups
✅ /api/group-tags

❌ /api/file
❌ /api/createGroup
❌ /api/getFiles
```

#### HTTP 方法语义
```rust
GET    /api/groups          // 获取分组列表
GET    /api/groups/{id}     // 获取单个分组
POST   /api/groups          // 创建分组
PUT    /api/groups/{id}     // 更新分组
DELETE /api/groups/{id}     // 删除分组
```

#### 查询参数设计
使用查询选项对象：

```rust
// ✅ 好的设计
GET /api/groups/search/by-filter-with-options?
    name=Documents&
    page=1&
    page_size=20&
    order_by=name_asc

// ❌ 避免的设计
GET /api/getGroupsByNameAndPage?name=Documents&page=1
```

#### Handler 命名规范
```rust
#[get("/api/groups")]
async fn api_list_groups(pool: web::Data<DbPool>) -> Result<HttpResponse> { }

#[get("/api/groups/{id}")]
async fn api_get_group_by_id(path: web::Path<i32>) -> Result<HttpResponse> { }

#[post("/api/groups")]
async fn api_create_group(payload: web::Json<CreateGroupDTO>) -> Result<HttpResponse> { }

#[put("/api/groups/update/by-conditions")]
async fn api_update_groups_by_conditions(
    payload: web::Json<(Vec<GroupCondition>, UpdateGroupDTO)>
) -> Result<HttpResponse> { }
```

### 5.2 错误处理

#### 统一响应格式
```rust
// 成功响应
{
  "success": true,
  "data": { /* 数据内容 */ },
  "count": 10  // 可选，列表查询时返回
}

// 错误响应
{
  "success": false,
  "error": {
    "code": "GROUP_NOT_FOUND",
    "message": "分组未找到"
  }
}
```

#### 错误码规范
使用大写字母 + 下划线分隔：

```rust
// 分组相关
GROUP_NOT_FOUND
CREATE_GROUP_FAILED
DELETE_GROUP_FAILED

// 文件相关
FILE_NOT_FOUND
CREATE_FILE_FAILED

// 验证错误
VALIDATION_ERROR

// 数据库错误
DATABASE_ERROR
```

#### 错误处理实现
```rust
#[derive(Debug)]
pub enum AppError {
  GroupNotFound,
  CreateGroupFailed(String),
  ValidationError(String),
  DieselError(DieselError),
}

impl Display for AppError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.message())
  }
}
```

---

## 6. 文档规范

### 6.1 README 文档
项目根目录和维护中英双语版本：
- `README.md` - 英文版本
- `README_cn.md` - 中文版本

### 6.2 项目文档
所有文档放在 `docs/` 目录：

```
docs/
├── README.md           # 英文概述
├── README_cn.md        # 中文概述
├── project_structure.md      # 英文项目结构
├── project_structure_cn.md   # 中文项目结构
└── CONTRIBUTING.md     # 贡献指南（本文件）
```

### 6.3 API 文档
使用 OpenAPI/Swagger 规范：

```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::api_list_groups,
        handlers::api_get_group_by_id,
    ),
    components(
        schemas(Group, CreateGroupDTO, UpdateGroupDTO)
    )
)]
pub struct ApiDoc;
```

访问路径：`/api-docs/openapi.json`

---

## 7. 安全检查清单

在提交代码前，请确认以下事项：

### 代码质量
- [ ] 代码已通过 `cargo fmt` 格式化
- [ ] 代码已通过 `cargo clippy` 检查
- [ ] 所有单元测试通过 (`cargo test`)
- [ ] 没有编译警告

### 功能完整性
- [ ] 新功能已添加单元测试
- [ ] 已有功能的回归测试通过
- [ ] 错误处理完善（考虑了各种异常情况）
- [ ] 事务使用正确（保证数据一致性）

### 代码规范
- [ ] 遵循命名规范（snake_case / PascalCase）
- [ ] 添加了必要的注释
- [ ] 公共函数有完整的文档注释
- [ ] 模块有模块级文档注释

### 数据安全
- [ ] SQL 注入防护（使用参数化查询）
- [ ] 输入验证完整
- [ ] 敏感信息未硬编码（使用环境变量）
- [ ] 引用计数更新正确（无内存泄漏风险）

### 文档更新
- [ ] 更新了相关文档（如适用）
- [ ] 提交了中英文文档（如适用）
- [ ] API 变更已更新 OpenAPI 文档

### Git 规范
- [ ] 提交信息符合 Conventional Commits 规范
- [ ] 分支命名正确
- [ ] 已与主分支同步

---

## 附录：快速参考

### 常用命令
```bash
# 格式化代码
cargo fmt
stylua static/js/

# 运行测试
cargo test

# 代码检查
cargo clippy -- -D warnings

# 构建项目
cargo build --release

# 运行 CLI
cargo run --bin file_classification_cli -- repl

# 运行 WebAPI
cargo run --bin file_classification_webapi

# 数据库迁移
diesel migration run
```

### 项目结构速查
```
FileClassificationSolutions/
├── common/                          # 公共工具库
├── file_classification_core/        # 核心业务逻辑
│   ├── internal/                    # 数据访问层（私有）
│   ├── model/                       # 数据模型（公开）
│   ├── service/                     # 业务服务（公开）
│   └── utils/                       # 工具函数
├── file_classification_cli/         # 命令行工具
├── file_classification_webapi/      # Web API
│   ├── handlers/                    # API 处理器
│   └── static/                      # 前端静态资源
└── migrations_*/                    # 数据库迁移
```

### 联系与支持
如有问题，请通过以下方式联系：
- 提交 Issue: https://github.com/DataEraserC/FileClassificationSolutions/issues
- 讨论区：https://github.com/DataEraserC/FileClassificationSolutions/discussions

---

**最后更新日期**: 2026-03-25  
**文档版本**: 1.0.0
