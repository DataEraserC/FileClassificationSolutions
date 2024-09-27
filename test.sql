-- 1. 用户上传文件 {a.jpg, a.mp4} -Q版青雀 #青雀 #崩坏星穷铁道 #Q版
-- 创建文件组
SELECT 1.0;
INSERT INTO groups (name, is_primary, createTime, modifyTime) VALUES ('Q版青雀', 1, datetime('now'), datetime('now'));

-- 获取刚创建的文件组ID
SELECT 1.1;
SELECT last_insert_rowid() AS group_id FROM groups;

-- 插入标签
SELECT 1.2;
INSERT INTO tags (name) VALUES ('青雀'), ('崩坏星穷铁道'), ('Q版');

-- 上传文件并获取刚上传的文件ID
SELECT 1.3;
-- 上传第一个文件 a.jpg
INSERT INTO files (type, location) VALUES ('jpg', 'path/to/a.jpg');

-- 获取第一个文件的ID
SELECT last_insert_rowid() AS file_id_1 FROM files;

-- 上传第二个文件 a.mp4
INSERT INTO files (type, location) VALUES ('mp4', 'path/to/a.mp4');

-- 获取第二个文件的ID
SELECT last_insert_rowid() AS file_id_2 FROM files;

-- 将文件与文件组关联
SELECT 1.4;
-- 注意：以下语句中的 [FILE_ID_1] 和 [FILE_ID_2] 应替换为实际的文件ID，[GROUP_ID] 替换为实际的文件组ID
-- INSERT INTO file_groups (file_id, group_id) VALUES ([FILE_ID_1], [GROUP_ID]), ([FILE_ID_2], [GROUP_ID]);
INSERT INTO file_groups (file_id, group_id) VALUES (1, 1), (2, 1);

-- 将标签与文件组关联
SELECT 1.5;
-- 注意：以下语句中的 [TAG_ID_1], [TAG_ID_2], [TAG_ID_3] 应替换为实际的标签ID
-- INSERT INTO group_tags (group_id, tag_id) VALUES ([GROUP_ID], [TAG_ID_1]), ([GROUP_ID], [TAG_ID_2]), ([GROUP_ID], [TAG_ID_3]);
INSERT INTO group_tags (group_id, tag_id) VALUES (1, 1), (1, 2), (1, 3);

-- 2. 用户上传文件 {b.jpg} -符玄教训青雀 #符玄

-- 创建文件组
SELECT 2.1;
INSERT INTO groups (name, createTime, modifyTime) VALUES ('符玄教训青雀', datetime('now'), datetime('now'));

-- 获取刚创建的文件组ID
SELECT 2.2;
SELECT last_insert_rowid() AS group_id FROM groups;

-- 插入标签
SELECT 2.3;
INSERT INTO tags (name) VALUES ('符玄');

-- 上传文件
SELECT 2.4;
INSERT INTO files (type, location) VALUES ('jpg', 'path/to/b.jpg');

-- 获取刚上传的文件ID
SELECT 2.5;
SELECT last_insert_rowid() AS file_id FROM files;

-- 将文件与文件组关联
SELECT 2.6;
-- 注意：以下语句中的 [FILE_ID] 和 [GROUP_ID] 应替换为实际的文件ID和文件组ID
-- INSERT INTO file_groups (file_id, group_id) VALUES ([FILE_ID], [GROUP_ID]);
INSERT INTO file_groups (file_id, group_id) VALUES (3, 2);

-- 将标签与文件组关联
SELECT 2.7;
-- 注意：以下语句中的 [GROUP_ID] 和 [TAG_ID] 应替换为实际的文件组ID和标签ID
-- INSERT INTO group_tags (group_id, tag_id) VALUES ([GROUP_ID], [TAG_ID]);
INSERT INTO group_tags (group_id, tag_id) VALUES (2, 4);


-- 3. 用户搜索文件 #符玄
SELECT 3.1;
SELECT f.id AS file_id, f.type AS file_type, f.location AS file_location
FROM files f
JOIN file_groups fg ON f.id = fg.file_id
JOIN group_tags gt ON fg.group_id = gt.group_id
JOIN tags t ON gt.tag_id = t.id
WHERE t.name = '符玄';

-- 4. 用户选择文件组2
-- 此步骤不需要SQL，因为它是用户界面操作
SELECT 4.1;


-- 5. 用户添加tag #崩坏星穹铁道
SELECT 5.1;
INSERT INTO tags (name) VALUES ('崩坏星穹铁道');

-- 获取刚插入的标签ID
SELECT 5.2;
SELECT last_insert_rowid() AS tag_id FROM tags;

-- 将标签与文件组关联
SELECT 5.3;
-- 注意：以下语句中的 [GROUP_ID] 和 [TAG_ID] 应替换为实际的文件组ID和标签ID
-- INSERT INTO group_tags (group_id, tag_id) VALUES ([GROUP_ID], [TAG_ID]);
INSERT INTO group_tags (group_id, tag_id) VALUES (2, 5);

-- 6. 用户添加tag #青雀
-- 将标签与文件组关联
SELECT 6.1;
-- 注意：以下语句中的 [GROUP_ID] 和 [TAG_ID] 应替换为实际的文件组ID和标签ID
-- INSERT INTO group_tags (group_id, tag_id) VALUES ([GROUP_ID], [TAG_ID]);
INSERT INTO group_tags (group_id, tag_id) VALUES (2, 1);


-- 7. 用户搜索文件及其tags -Q版青雀
SELECT 7.1;
SELECT 
    printf('%-7d|%-9s|%-13s|%-8d|%-14s|%-16d|%-10s|%-10s|%-8s', 
        f.id, 
        f.type, 
        f.location, 
        g.id, 
        g.name, 
        g.is_primary, 
        g.createTime, 
        g.modifyTime, 
        t.name
    ) AS 'file_id|file_type|file_location|group_id|group_name|is_primary_group|createTime|modifyTime|tag_name'
FROM 
    files f
JOIN 
    file_groups fg ON f.id = fg.file_id
JOIN 
    groups g ON fg.group_id = g.id
JOIN 
    group_tags gt ON g.id = gt.group_id
JOIN 
    tags t ON gt.tag_id = t.id
WHERE 
    g.name = 'Q版青雀';

-- 8. 用户选择文件组1
-- 此步骤不需要SQL，因为它是用户界面操作
SELECT 8.1;

-- 9. 用户修改tag名为崩坏星穹铁道
-- 检查是否存在同名的tag
SELECT 9.1;
SELECT id AS tag_id, name AS tag_name FROM tags WHERE name = '崩坏星穹铁道';
-- 如果不存在同名的tag，则执行以下更新操作，这里需要输入要更新的标签ID
-- 注意：以下语句中的 [TAG_ID] 应替换为实际的标签ID
-- UPDATE tags SET name = '崩坏星穹铁道' WHERE id = [TAG_ID];

-- 10. 用户搜索tag #崩坏星穹铁道
SELECT 10.1;
SELECT id AS tag_id FROM tags WHERE name = '崩坏星穹铁道';

-- 11. 用户选择tag #崩坏星穹铁道
-- 此步骤不需要SQL，因为它是用户界面操作
SELECT 11.1;

-- 12. 用户点击按tag搜索
SELECT 12.1;
-- 获取tag ID后，执行以下查询，这里需要输入tag ID
-- 注意：以下语句中的 [TAG_ID] 应替换为实际的标签ID
-- SELECT f.id AS file_id, f.type AS file_type, f.location AS file_location
-- FROM files f
-- JOIN file_groups fg ON f.id = fg.file_id
-- JOIN group_tags gt ON fg.group_id = gt.group_id
-- JOIN tags t ON gt.tag_id = t.id
-- WHERE t.id = [TAG_ID];

SELECT f.id AS file_id, f.type AS file_type, f.location AS file_location
FROM files f
JOIN file_groups fg ON f.id = fg.file_id
JOIN group_tags gt ON fg.group_id = gt.group_id
JOIN tags t ON gt.tag_id = t.id
WHERE t.id = 5;

-- 13. 用户手动按tag搜索 #青雀
SELECT 13.1;
SELECT f.id AS file_id, f.type AS file_type, f.location AS file_location
FROM files f
JOIN file_groups fg ON f.id = fg.file_id
JOIN group_tags gt ON fg.group_id = gt.group_id
JOIN tags t ON gt.tag_id = t.id
WHERE t.name = '青雀';

-- 14. 用户按精确文件组名搜索 -符玄教训青雀
SELECT 14.1;
SELECT g.id AS group_id, g.name AS group_name
FROM groups g
WHERE g.name = '符玄教训青雀';


-- 原要求
-- 我需要你模拟用户操作给出指定的sql语句  
-- （#表示tag）(-表示组名)  
  
-- 1. 用户上传文件 {a.jpg a.mp4} -Q版青雀 #青雀 #崩坏星穷铁道 #Q版 (假定数据库指定的id为1)  
-- 2. 用户上传文件 {b.jpg} -符玄教训青雀 #符玄 (假定数据库指定的id为2)  
-- 3. 用户搜索文件#符玄  
-- 4. 用户选择文件组2  
-- 5. 用户添加tag#崩坏星穹铁道  
-- 6. 用户添加tag#青雀  
-- 7. 用户搜索文件-Q版青雀 （注意：这里似乎重复了，原注释为6，但内容不同，已按7处理）  
-- 8. 用户选择文件组1内的tag#崩坏星穷铁道  
-- 9. 用户修改tag名为崩坏星穹铁道(此时搜索数据库内是否存在同tag, 若存在则失败并提示删除当前tag并手动添加tag)  
-- 10. 用户搜索tag#崩坏星穹铁道  
-- 11. 用户选择tag#崩坏星穹铁道  
-- 12. 用户点击按tag搜索  
  
-- 13. 用户手动按tag搜索#青雀  
-- 14. 用户按精确文件组名搜索-符玄教训青雀