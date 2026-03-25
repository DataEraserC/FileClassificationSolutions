# FileClassificationSolutions Contributing Guidelines

## Table of Contents

- [1. Code Style Guidelines](#1-code-style-guidelines)
  - [1.1 Rust Code Formatting](#11-rust-code-formatting)
  - [1.2 JavaScript Code Formatting](#12-javascript-code-formatting)
  - [1.3 Naming Conventions](#13-naming-conventions)
  - [1.4 Comment Guidelines](#14-comment-guidelines)
- [2. Project Architecture](#2-project-architecture)
  - [2.1 Layered Architecture](#21-layered-architecture)
  - [2.2 Module Organization](#22-module-organization)
  - [2.3 Dependency Management](#23-dependency-management)
- [3. Development Workflow](#3-development-workflow)
  - [3.1 Git Workflow](#31-git-workflow)
  - [3.2 Commit Message Convention](#32-commit-message-convention)
  - [3.3 Testing Guidelines](#33-testing-guidelines)
- [4. Database Guidelines](#4-database-guidelines)
  - [4.1 Migration Guidelines](#41-migration-guidelines)
  - [4.2 SQL Writing Standards](#42-sql-writing-standards)
- [5. API Design Guidelines](#5-api-design-guidelines)
  - [5.1 RESTful Standards](#51-restful-standards)
  - [5.2 Error Handling](#52-error-handling)
- [6. Documentation Guidelines](#6-documentation-guidelines)
- [7. Security Checklist](#7-security-checklist)

---

## 1. Code Style Guidelines

### 1.1 Rust Code Formatting

The project uses `rustfmt` for code formatting, configured in `rustfmt.toml`:

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

**Requirements:**
- Use 2 spaces for indentation
- Import statements sorted by groups: std → external crates → internal modules
- Module declarations must be reordered
- Use field init shorthand syntax
- Use try shorthand syntax (`?` operator)
- Comments auto-wrap and normalize

**Usage:**
```bash
cargo fmt
```

### 1.2 JavaScript Code Formatting

JavaScript files use Prettier for formatting, recommended configuration in `.prettierrc`:

```json
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5",
  "printWidth": 100
}
```

**Requirements:**
- Use 2 spaces for indentation
- Add semicolons at the end of statements
- Use single quotes for strings
- Line width limit 100 characters

**Usage:**
```bash
# Install Prettier
npm install --save-dev prettier

# Format JavaScript files
npx prettier --write static/js/
```

### 1.3 Naming Conventions

#### Variables and Functions
- **Variables**: Use `snake_case`
  ```rust
  let file_id = 123;
  let group_name = "Documents";
  ```

- **Functions**: Use `snake_case`
  ```rust
  pub fn create_group() { }
  pub fn delete_file_by_id() { }
  ```

- **Types and Structs**: Use `PascalCase`
  ```rust
  pub struct File { }
  pub struct CreateGroupDTO { }
  ```

- **Enums**: Use `PascalCase`
  ```rust
  pub enum GroupCondition { }
  pub enum OrderDirection { }
  ```

- **Constants**: Use `SCREAMING_SNAKE_CASE`
  ```rust
  pub const RELATION_TYPE_PARENT_CHILD: i32 = 1;
  ```

#### File Naming
- Rust source files: lowercase with underscores
  - ✅ `file_groups.rs`, `group_relations.rs`
  - ❌ `FileGroups.rs`, `filegroups.rs`

- Test files: Same name as the module being tested, placed in `tests/` directory
  ```
  file_classification_cli/
  ├── src/
  │   └── cli.rs
  └── tests/
      └── cli.rs
  ```

### 1.4 Comment Guidelines

#### Module-level Documentation
Each module file must include module-level documentation at the beginning:

```rust
// groups.rs
//! Group Service Module
//!
//! Provides business logic handling for groups, including creation, deletion,
//! query and update operations, and handles reference counting and cascading
//! deletion with related files and tags.
```

#### Function Documentation
Public functions must include complete documentation:

```rust
/// Create a group by name
///
/// Parameters:
/// - `conn`: Database connection object
/// - `create_group_dto`: DTO object containing group information
///
/// Returns:
/// On success, returns the ID of the inserted record; on failure, returns the corresponding error
pub fn create_group(
  conn: &mut AnyConnection,
  create_group_dto: &CreateGroupDTO,
) -> Result<i32, Error> {
  // ...
}
```

#### Inline Comments
- Use `//` for single-line comments, with one space before the comment
- Complex logic requires comments explaining the intent
- Avoid meaningless comments (when the code is already clear)

```rust
// ✅ Good comment
// For non-primary groups, we need to decrease the reference count of associated files first
for file in &files_associated_with_group {
  files_dao::decrease_file_reference_count_by_id(conn, file.id)?;
}

// ❌ Avoid comments like this
// Loop through files
for file in &files {
  // Delete file
  delete_file(file)?;
}
```

#### Language Requirements
- **All comments must be in English** (English terminology allowed for special business logic)
- Maintain bilingual Chinese-English versions for documentation strings (e.g., README)

---

## 2. Project Architecture

### 2.1 Layered Architecture

The project adopts a clear three-layer architecture:

```
┌─────────────────────────────────────┐
│     Presentation Layer (Web/CLI)    │
│  - handlers/ (API Request Handlers) │
│  - cli.rs (Command Line Parsing)    │
│  - repl.rs (Interactive Interface)  │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│      Business Logic Layer (Core)    │
│  - service/ (Business Logic Layer)  │
│  - model/ (Data Model Layer)        │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│      Data Access Layer (Core)       │
│  - internal/ (DAO Layer)            │
│  - utils/database.rs (DB Utils)     │
└─────────────────────────────────────┘
```

**Layer Dependency Rules:**
- ✅ Web/CLI → Core (service → model → internal)
- ❌ internal → service (No reverse dependencies)
- ❌ CLI ↔ WebAPI (No horizontal dependencies)

### 2.2 Module Organization

#### Core Module Structure
```
file_classification_core/
├── src/
│   ├── internal/          # Data access layer (private)
│   │   ├── files.rs       # File DAO
│   │   ├── groups.rs      # Group DAO
│   │   ├── tags.rs        # Tag DAO
│   │   ├── file_group.rs  # File-Group Association DAO
│   │   ├── group_tag.rs   # Group-Tag Association DAO
│   │   └── group_relations.rs  # Group Relations DAO
│   ├── model/             # Data model layer (public)
│   │   ├── models.rs      # All model definitions
│   │   └── schema.rs      # Diesel Schema
│   ├── service/           # Business logic layer (public)
│   │   ├── files.rs       # File service
│   │   ├── groups.rs      # Group service
│   │   └── ...
│   ├── utils/             # Utility modules
│   │   ├── database.rs    # Database connection
│   │   └── errors.rs      # Error handling
│   └── lib.rs             # Library entry point
```

#### Visibility Rules
- `internal/` module marked as `mod` (private), not exposed externally
- `model/` and `service/` marked as `pub mod` (public)
- External access only through interfaces provided by `service/`

```rust
// lib.rs
mod internal;        // ❌ Not accessible externally
pub mod model;       // ✅ Externally visible
pub mod service;     // ✅ Externally visible
pub mod utils;       // ✅ Externally visible
```

### 2.3 Dependency Management

#### Workspace Configuration
```toml
[workspace]
resolver = "2"
members = ["file_classification_*", "common"]
```

#### Dependency Version Management
Define uniformly in root `Cargo.toml`'s `[workspace.dependencies]`:

```toml
[workspace.dependencies]
diesel = { version = "2.2", features = ["chrono", "r2d2"] }
diesel_migrations = "2.2"
dotenvy = "0.15"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0.219", features = ["derive"] }
```

Sub-project references:
```toml
[dependencies]
diesel.workspace = true
chrono.workspace = true
```

#### Prohibitions
- ❌ No direct version specification in sub-projects
- ❌ No circular dependencies
- ❌ No unused dependencies

---

## 3. Development Workflow

### 3.1 Git Workflow

#### Branch Naming
- `main` / `master`: Main branch for releases
- `feature/*`: Feature branches
  ```bash
  git checkout -b feature/add-file-upload
  git checkout -b feature/group-hierarchy
  ```
- `bugfix/*`: Bug fix branches
  ```bash
  git checkout -b bugfix/primary-group-delete
  ```
- `release/*`: Release branches
  ```bash
  git checkout -b release/v1.2.0
  ```

#### Commit Workflow
```bash
# 1. Create feature branch from main
git checkout main
git pull origin main
git checkout -b feature/your-feature

# 2. Develop and commit (follow commit convention)
git add .
git commit -m "feat: add new file upload endpoint"

# 3. Sync main branch changes
git fetch origin
git rebase origin/main

# 4. Push and create Pull Request
git push origin feature/your-feature
```

### 3.2 Commit Message Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### Type Categories
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation updates
- `style`: Code formatting adjustments (no functional change)
- `refactor`: Refactoring (neither feature nor fix)
- `test`: Adding or modifying tests
- `chore`: Build process or auxiliary tool changes

#### Scope Categories
- `core`: Core library
- `cli`: Command-line tool
- `webapi`: Web API
- `db`: Database related
- `ui`: Frontend UI

#### Examples
```bash
# ✅ Good commits
feat(core): add group tree structure support
fix(cli): resolve primary group deletion issue
docs(readme): update Chinese translation
refactor(webapi): optimize error handling logic
test(core): add unit tests for file service

# ❌ Bad commits
update code
fix bug
add new feature
```

### 3.3 Testing Guidelines

#### Unit Tests
Write unit tests for all public functions:

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

#### Integration Tests
Add integration tests in `tests/` directory:

```rust
// tests/cli.rs
#[test]
fn test_file_commands() {
  assert!(true)
}
```

#### Running Tests
```bash
# Run all tests
cargo test

# Run specific package tests
cargo test --package file_classification_core

# Run tests with filter
cargo test test_create_group
```

#### CI/CD Requirements
All PRs must pass GitHub Actions automated tests:
- ✅ Linux (Ubuntu) tests pass
- ✅ Windows tests pass
- ✅ macOS tests pass
- ✅ Code formatting checks pass

---

## 4. Database Guidelines

### 4.1 Migration Guidelines

#### Directory Structure
Support multiple databases with independent migration directories:
```
migrations/                # Diesel default (SQLite)
migrations_mysql/          # MySQL
migrations_postgres/       # PostgreSQL
migrations_sqlite/         # SQLite
```

#### Migration Naming
Format: `YYYY-MM-DD-HHMMSS_description`

```bash
✅ 2024-10-01-193345_FileClassification
✅ 2025-10-20-000000_update_group_hierarchy
✅ 2025-11-02-000000_add_description_fields
```

#### File Requirements
Each migration must include:
- `up.sql`: Upgrade script
- `down.sql`: Downgrade script (rollbackable)

```sql
-- up.sql
CREATE TABLE IF NOT EXISTS `groups` (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- down.sql
DROP TABLE IF EXISTS `groups`;
```

#### Multi-Database Compatibility
SQL syntax differences between databases must be maintained separately:

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

### 4.2 SQL Writing Standards

#### Table and Column Names
- Use `snake_case`
- Table names in plural form
- Foreign keys in singular form + `_id` suffix

```sql
-- ✅ Good naming
CREATE TABLE files (
    id INTEGER PRIMARY KEY,
    type TEXT NOT NULL,
    group_id INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

-- ❌ Avoid naming
CREATE TABLE FileList (
    FileID INTEGER PRIMARY KEY,
    FileType TEXT,
    GroupID INTEGER
);
```

#### Index Optimization
Add indexes for frequently queried columns:

```sql
CREATE INDEX idx_files_group_id ON files(group_id);
CREATE INDEX idx_groups_name ON groups(name);
```

#### Comment Standards
Add table and column comments in SQL files:

```sql
CREATE TABLE `groups` (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);
-- COMMENT ON TABLE `groups` IS 'Group information table';
-- COMMENT ON COLUMN `groups`.id IS 'Group ID';
-- COMMENT ON COLUMN `groups`.name IS 'Group name';
```

---

## 5. API Design Guidelines

### 5.1 RESTful Standards

#### Resource Naming
- Use plural nouns
- Use lowercase letters with hyphens

```
✅ /api/files
✅ /api/groups
✅ /api/file-groups
✅ /api/group-tags

❌ /api/file
❌ /api/createGroup
❌ /api/getFiles
```

#### HTTP Method Semantics
```rust
GET    /api/groups          // Get group list
GET    /api/groups/{id}     // Get single group
POST   /api/groups          // Create group
PUT    /api/groups/{id}     // Update group
DELETE /api/groups/{id}     // Delete group
```

#### Query Parameter Design
Use query options objects:

```rust
// ✅ Good design
GET /api/groups/search/by-filter-with-options?
    name=Documents&
    page=1&
    page_size=20&
    order_by=name_asc

// ❌ Avoid design
GET /api/getGroupsByNameAndPage?name=Documents&page=1
```

#### Handler Naming Convention
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

### 5.2 Error Handling

#### Unified Response Format
```rust
// Success response
{
  "success": true,
  "data": { /* data content */ },
  "count": 10  // optional, returned for list queries
}

// Error response
{
  "success": false,
  "error": {
    "code": "GROUP_NOT_FOUND",
    "message": "Group not found"
  }
}
```

#### Error Code Convention
Use uppercase letters with underscores:

```rust
// Group related
GROUP_NOT_FOUND
CREATE_GROUP_FAILED
DELETE_GROUP_FAILED

// File related
FILE_NOT_FOUND
CREATE_FILE_FAILED

// Validation errors
VALIDATION_ERROR

// Database errors
DATABASE_ERROR
```

#### Error Handling Implementation
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

## 6. Documentation Guidelines

### 6.1 README Documentation
Maintain both English and Chinese versions in project root:
- `README.md` - English version
- `README_cn.md` - Chinese version

### 6.2 Project Documentation
All documentation goes in `docs/` directory:

```
docs/
├── README.md                    # English overview
├── README_cn.md                 # Chinese overview
├── project_structure.md         # English project structure
├── project_structure_cn.md      # Chinese project structure
└── CONTRIBUTING.md              # Contributing guidelines (this file)
```

### 6.3 API Documentation
Use OpenAPI/Swagger specification:

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

Access path: `/api-docs/openapi.json`

---

## 7. Security Checklist

Before submitting code, please confirm the following items:

### Code Quality
- [ ] Code formatted with `cargo fmt`
- [ ] Code passes `cargo clippy` checks
- [ ] All unit tests pass (`cargo test`)
- [ ] No compilation warnings

### Functional Completeness
- [ ] Unit tests added for new features
- [ ] Regression tests pass for existing features
- [ ] Error handling is complete (all edge cases considered)
- [ ] Transactions used correctly (data consistency guaranteed)

### Code Standards
- [ ] Naming conventions followed (snake_case / PascalCase)
- [ ] Necessary comments added
- [ ] Complete documentation for public functions
- [ ] Module-level documentation present

### Data Security
- [ ] SQL injection prevention (parameterized queries used)
- [ ] Input validation complete
- [ ] No hardcoded sensitive information (use environment variables)
- [ ] Reference count updates correct (no memory leak risks)

### Documentation Updates
- [ ] Related documentation updated (if applicable)
- [ ] Both Chinese and English versions submitted (if applicable)
- [ ] API changes reflected in OpenAPI documentation

### Git Standards
- [ ] Commit messages follow Conventional Commits
- [ ] Branch naming is correct
- [ ] Synchronized with main branch

---

## Appendix: Quick Reference

### Common Commands
```bash
# Format code
cargo fmt
stylua static/js/

# Run tests
cargo test

# Code inspection
cargo clippy -- -D warnings

# Build project
cargo build --release

# Run CLI
cargo run --bin file_classification_cli -- repl

# Run WebAPI
cargo run --bin file_classification_webapi

# Database migration
diesel migration run
```

### Project Structure Quick Reference
```
FileClassificationSolutions/
├── common/                          # Common utilities
├── file_classification_core/        # Core business logic
│   ├── internal/                    # Data access layer (private)
│   ├── model/                       # Data models (public)
│   ├── service/                     # Business services (public)
│   └── utils/                       # Utility functions
├── file_classification_cli/         # Command-line tool
├── file_classification_webapi/      # Web API
│   ├── handlers/                    # API handlers
│   └── static/                      # Frontend static assets
└── migrations_*/                    # Database migrations
```

### Contact and Support
For questions, please contact via:
- Submit Issue: https://github.com/DataEraserC/FileClassificationSolutions/issues
- Discussions: https://github.com/DataEraserC/FileClassificationSolutions/discussions

---

**Last Updated**: 2026-03-25  
**Document Version**: 1.0.0
