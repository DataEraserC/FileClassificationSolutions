-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `groups` (
    id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT COMMENT '组ID',
    name VARCHAR(255) NOT NULL UNIQUE COMMENT '组名称',
    reference_count INTEGER NOT NULL DEFAULT 0 COMMENT '引用计数',
    is_primary BOOLEAN NOT NULL DEFAULT false COMMENT '是否为主组',
    click_count INTEGER NOT NULL DEFAULT 0 COMMENT '点击次数',
    share_count INTEGER NOT NULL DEFAULT 0 COMMENT '分享次数',
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    modify_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '修改时间'
) COMMENT='组信息表';

CREATE TABLE IF NOT EXISTS `tags` (
    id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT COMMENT '标签ID',
    reference_count INTEGER NOT NULL DEFAULT 0 COMMENT '引用计数',
    name VARCHAR(100) NOT NULL UNIQUE COMMENT '标签名称'
) COMMENT='标签信息表';

CREATE TABLE IF NOT EXISTS `files` (
    id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT COMMENT '文件ID',
    type TEXT NOT NULL COMMENT '文件类型',
    path TEXT NOT NULL COMMENT '文件路径',
    reference_count INTEGER NOT NULL DEFAULT 0 COMMENT '引用计数',
    group_id INTEGER NOT NULL COMMENT '主组ID',
    FOREIGN KEY (group_id) REFERENCES `groups`(id)
) COMMENT='文件信息表';

CREATE TABLE IF NOT EXISTS `file_groups` (
    file_id INTEGER NOT NULL COMMENT '文件ID',
    group_id INTEGER NOT NULL COMMENT '组ID',
    PRIMARY KEY (file_id, group_id),
    FOREIGN KEY (file_id) REFERENCES `files`(id),
    FOREIGN KEY (group_id) REFERENCES `groups`(id)
) COMMENT='文件组关联表';

CREATE TABLE IF NOT EXISTS `group_tags` (
    group_id INTEGER NOT NULL COMMENT '组ID',
    tag_id INTEGER NOT NULL COMMENT '标签ID',
    PRIMARY KEY (group_id, tag_id),
    FOREIGN KEY (group_id) REFERENCES `groups`(id),
    FOREIGN KEY (tag_id) REFERENCES `tags`(id)
) COMMENT='组标签关联表';
