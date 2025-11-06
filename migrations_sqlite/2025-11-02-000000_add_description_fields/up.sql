-- 为files表添加description字段
ALTER TABLE `files` ADD COLUMN description TEXT;
-- SQLite 不支持字段注释，description字段表示文件描述

-- 为groups表添加description字段
ALTER TABLE `groups` ADD COLUMN description TEXT;
-- SQLite 不支持字段注释，description字段表示组描述

-- 为tags表添加description字段
ALTER TABLE `tags` ADD COLUMN description TEXT;
-- SQLite 不支持字段注释，description字段表示标签描述
