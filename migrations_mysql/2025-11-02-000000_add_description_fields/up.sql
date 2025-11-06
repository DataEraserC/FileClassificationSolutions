-- 为files表添加description字段
ALTER TABLE `files` ADD COLUMN description TEXT COMMENT '文件描述';

-- 为groups表添加description字段
ALTER TABLE `groups` ADD COLUMN description TEXT COMMENT '组描述';

-- 为tags表添加description字段
ALTER TABLE `tags` ADD COLUMN description TEXT COMMENT '标签描述';
