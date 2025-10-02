# FileClassificationSolutions

## Project Overview

FileClassificationSolutions is an innovative file classification and storage system developed in the Rust programming language. It aims to provide users with an efficient and convenient way to manage and retrieve various types of file data, such as memes, text files, or other abstract data types. The system establishes a non-file-system-like database architecture that allows files to be categorized and quickly retrieved based on their content and attributes (such as tags and groups).

## Core Features

- **Multi-dimensional Classification**: Achieve multi-dimensional file management through groups (`groups`) and tags (`tags`)
- **Reference Counting Mechanism**: Automatically track associations between files, groups, and tags to ensure data consistency
- **Flexible Querying**: Support complex queries based on multiple conditions including equals, greater than, less than, LIKE pattern matching, etc.
- **Batch Operations**: Support batch updates and deletions based on conditions
- **Primary Group Concept**: Distinguish between primary groups and ordinary groups, where primary groups have a one-to-one relationship with files
- **Multiple Access Methods**: Provide both Command Line Interface (CLI) and Web API access methods

## System Architecture

The project adopts a modular architecture design, including the following core components:

### Core Library (file_classification_core)

This is the business logic core of the entire project, containing data models, database access layer, and business services.

### Command Line Interface (file_classification_cli)

Provides an interactive command-line tool for operating the file classification system, including complete CRUD functionality.

### Web API (file_classification_webapi)

A RESTful API service built on the Actix-web framework that provides HTTP interfaces for data operations.

## Database Design

### Core Table Structure

The system uses the following 5 core tables to store data:

1. `files` table: Stores basic file information
2. `groups` table: Stores file group information
3. `file_groups` table: Stores many-to-many relationships between files and groups
4. `tags` table: Stores tag information
5. `group_tags` table: Stores many-to-many relationships between groups and tags

### Table Structure Details

```sql
CREATE TABLE IF NOT EXISTS files (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,   -- File type
    path TEXT NOT NULL, -- File storage location
    reference_count INTEGER NOT NULL DEFAULT 0, -- Reference count
    group_id INTEGER NOT NULL, -- Default file group ID
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

CREATE TABLE IF NOT EXISTS groups (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE, -- Group name
    reference_count INTEGER NOT NULL DEFAULT 0, -- Reference count
    is_primary BOOLEAN NOT NULL DEFAULT false, -- Whether it's a primary group, false for no, true for yes
    click_count INTEGER NOT NULL DEFAULT 0, -- Click count
    share_count INTEGER NOT NULL DEFAULT 0, -- Share count
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- Creation time
    modify_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP  -- Modification time
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
    reference_count INTEGER NOT NULL DEFAULT 0, -- Reference count
    name TEXT NOT NULL UNIQUE -- Tag name, unique
);

CREATE TABLE IF NOT EXISTS group_tags (
    group_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (group_id, tag_id),
    FOREIGN KEY (group_id) REFERENCES groups(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);
```


## Function Details

### File Management

- Files are stored with unique identifiers (ID)
- Records file type (`type`) and storage path (`path`)
- Automatically maintains reference count (`reference_count`)
- Establishes association relationships with groups

### Group System

- Supports creating file groups with unique names
- Distinguishes between primary groups (`is_primary`) and ordinary groups
- Each file has a primary file group that records detailed information
- Tracks group usage statistics (click count, share count)
- Automatically maintains reference count
- Records creation and modification times

### Tag System

- Supports creating tags with unique names
- Organizes and classifies file groups through tags
- Automatically maintains reference count

### Query Functionality

#### Basic Queries

Supports precise queries based on various fields:
- Files: ID, type, path, reference count, group ID
- Groups: ID, name, reference count, whether it's a primary group, click count, share count, creation time, modification time
- Tags: ID, name, reference count

#### Advanced Queries

Supports complex conditional queries:
- Range queries: greater than, less than
- Pattern matching: LIKE queries
- Combined conditions: AND, OR, NOT logical combinations
- Batch queries: IN conditions

#### Association Queries

- Query related file groups by tag
- Query groups that files belong to
- Query associated tags by group

### Update Functionality

Supports batch updates based on conditions:
- Files: path, type, reference count, group ID
- Groups: name, reference count, whether it's a primary group, click count, share count, timestamps
- Tags: name, reference count

### Delete Functionality

Supports safe data deletion:
- Automatically handles reduction of reference counts
- Cascading deletion of associated data
- Automatic cleanup of associated files when deleting primary groups

## Usage

### Command Line Interface

Provides multiple independent command-line tools:

```bash
# File operations
list_files                           # List files
list_files_by_conditions            # Query files by conditions
create_file                         # Create file
delete_file                         # Delete file
update_files_by_conditions          # Update files by conditions

# Group operations
list_groups                         # List groups
list_groups_by_conditions           # Query groups by conditions
create_group                        # Create group
delete_group                        # Delete group
update_groups_by_conditions         # Update groups by conditions

# Tag operations
list_tags                           # List tags
list_tags_by_conditions             # Query tags by conditions
create_tag                          # Create tag
delete_tag                          # Delete tag
update_tags_by_conditions           # Update tags by conditions

# Association operations
list_file_groups_by_conditions      # Query file-group associations by conditions
create_file_group                   # Create file-group association
delete_file_group                   # Delete file-group association
list_group_tags_by_conditions       # Query group-tag associations by conditions
create_group_tag                    # Create group-tag association
delete_group_tag                    # Delete group-tag association
```


### Web API

Provides RESTful API interfaces:

```
# File related
GET    /api/files                  # Query files
POST   /api/files/search           # Query files by conditions
PUT    /api/files                  # Update files by conditions
DELETE /api/files/{id}             # Delete file

# Group related
GET    /api/groups                 # Query groups
POST   /api/groups/search          # Query groups by conditions
POST   /api/groups                 # Create group
PUT    /api/groups                 # Update groups by conditions
DELETE /api/groups/{id}            # Delete group

# Tag related
GET    /api/tags                   # Query tags
POST   /api/tags/search            # Query tags by conditions
POST   /api/tags                   # Create tag
PUT    /api/tags                   # Update tags by conditions
DELETE /api/tags/{id}              # Delete tag

# File-group association related
GET    /api/file-groups            # Query file-group associations by conditions
POST   /api/file-groups            # Create file-group association
DELETE /api/file-groups            # Delete file-group association

# Group-tag association related
GET    /api/group-tags             # Query group-tag associations by conditions
POST   /api/group-tags             # Create group-tag association
DELETE /api/group-tags             # Delete group-tag association
```


## Technical Features

### Architectural Advantages

1. **Layered Architecture**: Clear separation of data access layer, business logic layer, and presentation layer
2. **Modular Design**: Different functional modules organized in independent crates
3. **Strong Type Safety**: Utilizes Rust's type system to ensure code safety
4. **Error Handling**: Unified error handling mechanism
5. **Database Abstraction**: Uses Diesel ORM for database operations
6. **Scalability**: Easy to add new functional modules and access interfaces

### Data Consistency

1. **Transaction Processing**: Critical operations use database transactions to ensure consistency
2. **Reference Counting**: Automatically maintains references between entities
3. **Cascading Operations**: Automatically cleans up associated data during deletion operations

### Performance Optimization

1. **Query Optimization**: Supports complex conditional queries and sorting
2. **Batch Operations**: Supports batch updates and deletions
3. **Connection Pooling**: Web API uses database connection pooling to improve performance

## Future Plans

1. **Multiple File Type Support**: Extend support for more file types
2. **Data Index Optimization**: Create indexes on frequently queried fields
3. **Data Backup and Recovery**: Implement data backup and recovery mechanisms
4. **User Permission Management**: Support multi-user and permission control
5. **File Version Control**: Add file version management functionality
6. **Full-text Search**: Implement full-text search based on file content
7. **Multiple Database Compatibility**: Support more database types
8. **Enhanced Batch Operations**: Provide richer batch operation functionality

## Application Scenarios

- Meme management
- Document classification storage
- Image asset management
- Code snippet organization
- General file management systems

This system is not only suitable for specific types of file management but can also be extended as a general file management and search solution.