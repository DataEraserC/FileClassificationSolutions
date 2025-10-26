// API基础URL
const BASE_URL = 'http://127.0.0.1:8082';

// 工具函数：显示结果
function displayResult(elementId, data) {
    const resultElement = document.getElementById(elementId);
    resultElement.innerText = JSON.stringify(data, null, 2);
}

// 工具函数：获取输入值
function getInputValue(id) {
    return document.getElementById(id).value;
}

// 工具函数：设置输入值
function setInputValue(id, value) {
    document.getElementById(id).value = value;
}

// 工具函数：获取文本域值
function getTextValue(id) {
    return document.getElementById(id).value;
}

// 工具函数：设置文本域值
function setTextValue(id, value) {
    document.getElementById(id).value = value;
}

// ==================== 文件管理相关函数 ====================

function listFilesByFilter() {
    const fileId = getInputValue('file-id');
    const fileName = getInputValue('file-name');
    const filePath = getInputValue('file-path');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (fileId) params.append('id', fileId);
    if (fileName) params.append('name', fileName);
    if (filePath) params.append('path', filePath);
    
    const url = `${BASE_URL}/api/files/filter?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('files-result', data);
        })
        .catch(error => {
            displayResult('files-result', { error: error.message });
        });
}

function getFileById() {
    const fileId = getInputValue('file-id');
    if (!fileId) {
        displayResult('files-result', { error: '请输入文件ID' });
        return;
    }
    
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('files-result', data);
        })
        .catch(error => {
            displayResult('files-result', { error: error.message });
        });
}

function listFilesByConditions() {
    const conditionsJson = getTextValue('file-conditions');
    if (!conditionsJson) {
        displayResult('files-result', { error: '请输入查询条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('files-result', { error: '查询条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/files/search/by-conditions`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('files-result', data);
    })
    .catch(error => {
        displayResult('files-result', { error: error.message });
    });
}

function listFilesByGroupId() {
    const groupId = getInputValue('file-group-id');
    if (!groupId) {
        displayResult('files-result', { error: '请输入组ID' });
        return;
    }
    
    const url = `${BASE_URL}/api/files/group/${groupId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('files-result', data);
        })
        .catch(error => {
            displayResult('files-result', { error: error.message });
        });
}

function createFile() {
    const fileName = getInputValue('file-name');
    const filePath = getInputValue('file-path');
    
    if (!fileName || !filePath) {
        displayResult('files-result', { error: '请输入文件名和文件路径' });
        return;
    }
    
    const fileData = {
        name: fileName,
        path: filePath
    };
    
    const url = `${BASE_URL}/api/files`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(fileData)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('files-result', data);
    })
    .catch(error => {
        displayResult('files-result', { error: error.message });
    });
}

function updateFilesByConditions() {
    const conditionsJson = getTextValue('file-conditions');
    const updateDataJson = getTextValue('file-update-data');
    
    if (!conditionsJson || !updateDataJson) {
        displayResult('files-result', { error: '请输入查询条件和更新数据' });
        return;
    }
    
    let conditions, updateData;
    try {
        conditions = JSON.parse(conditionsJson);
        updateData = JSON.parse(updateDataJson);
    } catch (e) {
        displayResult('files-result', { error: 'JSON格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/files/update/by-conditions`;
    fetch(url, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify([conditions, updateData])
    })
    .then(response => response.json())
    .then(data => {
        displayResult('files-result', data);
    })
    .catch(error => {
        displayResult('files-result', { error: error.message });
    });
}

function deleteFilesByConditions() {
    const conditionsJson = getTextValue('file-conditions');
    if (!conditionsJson) {
        displayResult('files-result', { error: '请输入删除条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('files-result', { error: '删除条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/files/delete/by-conditions`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('files-result', data);
    })
    .catch(error => {
        displayResult('files-result', { error: error.message });
    });
}

// ==================== 组管理相关函数 ====================

function listGroupsByFilter() {
    const groupId = getInputValue('group-id');
    const groupName = getInputValue('group-name');
    const groupDescription = getInputValue('group-description');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (groupId) params.append('id', groupId);
    if (groupName) params.append('name', groupName);
    if (groupDescription) params.append('description', groupDescription);
    
    const url = `${BASE_URL}/api/groups/filter?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('groups-result', data);
        })
        .catch(error => {
            displayResult('groups-result', { error: error.message });
        });
}

function getGroupById() {
    const groupId = getInputValue('group-id');
    if (!groupId) {
        displayResult('groups-result', { error: '请输入组ID' });
        return;
    }
    
    const url = `${BASE_URL}/api/groups/${groupId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('groups-result', data);
        })
        .catch(error => {
            displayResult('groups-result', { error: error.message });
        });
}

function listGroupsByConditions() {
    const conditionsJson = getTextValue('group-conditions');
    if (!conditionsJson) {
        displayResult('groups-result', { error: '请输入查询条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('groups-result', { error: '查询条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/groups/search/by-conditions`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('groups-result', data);
    })
    .catch(error => {
        displayResult('groups-result', { error: error.message });
    });
}

function listGroupsByFileId() {
    const fileId = getInputValue('group-file-id');
    if (!fileId) {
        displayResult('groups-result', { error: '请输入文件ID' });
        return;
    }
    
    const url = `${BASE_URL}/api/groups/file/${fileId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('groups-result', data);
        })
        .catch(error => {
            displayResult('groups-result', { error: error.message });
        });
}

function listGroupsByTagId() {
    const tagId = getInputValue('group-tag-id');
    if (!tagId) {
        displayResult('groups-result', { error: '请输入标签ID' });
        return;
    }
    
    const url = `${BASE_URL}/api/groups/tag/${tagId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('groups-result', data);
        })
        .catch(error => {
            displayResult('groups-result', { error: error.message });
        });
}

function createGroup() {
    const groupName = getInputValue('group-name');
    const groupDescription = getInputValue('group-description');
    
    if (!groupName) {
        displayResult('groups-result', { error: '请输入组名' });
        return;
    }
    
    const groupData = {
        name: groupName,
        description: groupDescription || ''
    };
    
    const url = `${BASE_URL}/api/groups`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(groupData)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('groups-result', data);
    })
    .catch(error => {
        displayResult('groups-result', { error: error.message });
    });
}

function updateGroupsByConditions() {
    const conditionsJson = getTextValue('group-conditions');
    const updateDataJson = getTextValue('group-update-data');
    
    if (!conditionsJson || !updateDataJson) {
        displayResult('groups-result', { error: '请输入查询条件和更新数据' });
        return;
    }
    
    let conditions, updateData;
    try {
        conditions = JSON.parse(conditionsJson);
        updateData = JSON.parse(updateDataJson);
    } catch (e) {
        displayResult('groups-result', { error: 'JSON格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/groups/update/by-conditions`;
    fetch(url, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify([conditions, updateData])
    })
    .then(response => response.json())
    .then(data => {
        displayResult('groups-result', data);
    })
    .catch(error => {
        displayResult('groups-result', { error: error.message });
    });
}

function deleteGroupsByConditions() {
    const conditionsJson = getTextValue('group-conditions');
    if (!conditionsJson) {
        displayResult('groups-result', { error: '请输入删除条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('groups-result', { error: '删除条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/groups/delete/by-conditions`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('groups-result', data);
    })
    .catch(error => {
        displayResult('groups-result', { error: error.message });
    });
}

function getGroupTree() {
    const groupId = getInputValue('group-id');
    if (!groupId) {
        displayResult('groups-result', { error: '请输入组ID' });
        return;
    }
    
    const url = `${BASE_URL}/api/groups/${groupId}/tree`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('groups-result', data);
        })
        .catch(error => {
            displayResult('groups-result', { error: error.message });
        });
}

// ==================== 标签管理相关函数 ====================

function listTagsByFilter() {
    const tagId = getInputValue('tag-id');
    const tagName = getInputValue('tag-name');
    const tagDescription = getInputValue('tag-description');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (tagId) params.append('id', tagId);
    if (tagName) params.append('name', tagName);
    if (tagDescription) params.append('description', tagDescription);
    
    const url = `${BASE_URL}/api/tags/filter?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('tags-result', data);
        })
        .catch(error => {
            displayResult('tags-result', { error: error.message });
        });
}

function getTagById() {
    const tagId = getInputValue('tag-id');
    if (!tagId) {
        displayResult('tags-result', { error: '请输入标签ID' });
        return;
    }
    
    const url = `${BASE_URL}/api/tags/${tagId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('tags-result', data);
        })
        .catch(error => {
            displayResult('tags-result', { error: error.message });
        });
}

function listTagsByConditions() {
    const conditionsJson = getTextValue('tag-conditions');
    if (!conditionsJson) {
        displayResult('tags-result', { error: '请输入查询条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('tags-result', { error: '查询条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/tags/search/by-conditions`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('tags-result', data);
    })
    .catch(error => {
        displayResult('tags-result', { error: error.message });
    });
}

function listTagsByGroupId() {
    const groupId = getInputValue('tag-group-id');
    if (!groupId) {
        displayResult('tags-result', { error: '请输入组ID' });
        return;
    }
    
    const url = `${BASE_URL}/api/tags/group/${groupId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            displayResult('tags-result', data);
        })
        .catch(error => {
            displayResult('tags-result', { error: error.message });
        });
}

function createTag() {
    const tagName = getInputValue('tag-name');
    const tagDescription = getInputValue('tag-description');
    
    if (!tagName) {
        displayResult('tags-result', { error: '请输入标签名' });
        return;
    }
    
    const tagData = {
        name: tagName,
        description: tagDescription || ''
    };
    
    const url = `${BASE_URL}/api/tags`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(tagData)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('tags-result', data);
    })
    .catch(error => {
        displayResult('tags-result', { error: error.message });
    });
}

function updateTagsByConditions() {
    const conditionsJson = getTextValue('tag-conditions');
    const updateDataJson = getTextValue('tag-update-data');
    
    if (!conditionsJson || !updateDataJson) {
        displayResult('tags-result', { error: '请输入查询条件和更新数据' });
        return;
    }
    
    let conditions, updateData;
    try {
        conditions = JSON.parse(conditionsJson);
        updateData = JSON.parse(updateDataJson);
    } catch (e) {
        displayResult('tags-result', { error: 'JSON格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/tags/update/by-conditions`;
    fetch(url, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify([conditions, updateData])
    })
    .then(response => response.json())
    .then(data => {
        displayResult('tags-result', data);
    })
    .catch(error => {
        displayResult('tags-result', { error: error.message });
    });
}

function deleteTagsByConditions() {
    const conditionsJson = getTextValue('tag-conditions');
    if (!conditionsJson) {
        displayResult('tags-result', { error: '请输入删除条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('tags-result', { error: '删除条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/tags/delete/by-conditions`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('tags-result', data);
    })
    .catch(error => {
        displayResult('tags-result', { error: error.message });
    });
}

// ==================== 文件组关联相关函数 ====================

function listFileGroupsByConditions() {
    const conditionsJson = getTextValue('file-group-conditions');
    if (!conditionsJson) {
        displayResult('file-groups-result', { error: '请输入查询条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('file-groups-result', { error: '查询条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/file-groups/search/by-conditions`;
    fetch(url, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('file-groups-result', data);
    })
    .catch(error => {
        displayResult('file-groups-result', { error: error.message });
    });
}

function createFileGroup() {
    const fileId = getInputValue('file-group-file-id');
    const groupId = getInputValue('file-group-group-id');
    
    if (!fileId || !groupId) {
        displayResult('file-groups-result', { error: '请输入文件ID和组ID' });
        return;
    }
    
    const fileGroupData = {
        file_id: parseInt(fileId),
        group_id: parseInt(groupId)
    };
    
    const url = `${BASE_URL}/api/file-groups`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(fileGroupData)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('file-groups-result', data);
    })
    .catch(error => {
        displayResult('file-groups-result', { error: error.message });
    });
}

function deleteFileGroup() {
    const fileId = getInputValue('file-group-file-id');
    const groupId = getInputValue('file-group-group-id');
    
    if (!fileId || !groupId) {
        displayResult('file-groups-result', { error: '请输入文件ID和组ID' });
        return;
    }
    
    const fileGroupData = {
        file_id: parseInt(fileId),
        group_id: parseInt(groupId)
    };
    
    const url = `${BASE_URL}/api/file-groups`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(fileGroupData)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('file-groups-result', data);
    })
    .catch(error => {
        displayResult('file-groups-result', { error: error.message });
    });
}

function deleteFileGroupsByConditions() {
    const conditionsJson = getTextValue('file-group-conditions');
    if (!conditionsJson) {
        displayResult('file-groups-result', { error: '请输入删除条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('file-groups-result', { error: '删除条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/file-groups/delete/by-conditions`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('file-groups-result', data);
    })
    .catch(error => {
        displayResult('file-groups-result', { error: error.message });
    });
}

// ==================== 组标签关联相关函数 ====================

function listGroupTagsByConditions() {
    const conditionsJson = getTextValue('group-tag-conditions');
    if (!conditionsJson) {
        displayResult('group-tags-result', { error: '请输入查询条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('group-tags-result', { error: '查询条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/group-tags/search/by-conditions`;
    fetch(url, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('group-tags-result', data);
    })
    .catch(error => {
        displayResult('group-tags-result', { error: error.message });
    });
}

function createGroupTag() {
    const groupId = getInputValue('group-tag-group-id');
    const tagId = getInputValue('group-tag-tag-id');
    
    if (!groupId || !tagId) {
        displayResult('group-tags-result', { error: '请输入组ID和标签ID' });
        return;
    }
    
    const groupTagData = {
        group_id: parseInt(groupId),
        tag_id: parseInt(tagId)
    };
    
    const url = `${BASE_URL}/api/group-tags`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(groupTagData)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('group-tags-result', data);
    })
    .catch(error => {
        displayResult('group-tags-result', { error: error.message });
    });
}

function deleteGroupTag() {
    const groupId = getInputValue('group-tag-group-id');
    const tagId = getInputValue('group-tag-tag-id');
    
    if (!groupId || !tagId) {
        displayResult('group-tags-result', { error: '请输入组ID和标签ID' });
        return;
    }
    
    const groupTagData = {
        group_id: parseInt(groupId),
        tag_id: parseInt(tagId)
    };
    
    const url = `${BASE_URL}/api/group-tags`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(groupTagData)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('group-tags-result', data);
    })
    .catch(error => {
        displayResult('group-tags-result', { error: error.message });
    });
}

function deleteGroupTagsByConditions() {
    const conditionsJson = getTextValue('group-tag-conditions');
    if (!conditionsJson) {
        displayResult('group-tags-result', { error: '请输入删除条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('group-tags-result', { error: '删除条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/group-tags/delete/by-conditions`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('group-tags-result', data);
    })
    .catch(error => {
        displayResult('group-tags-result', { error: error.message });
    });
}

// ==================== 组关系管理相关函数 ====================

function listGroupRelationsByConditions() {
    const conditionsJson = getTextValue('group-relation-conditions');
    if (!conditionsJson) {
        displayResult('group-relations-result', { error: '请输入查询条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('group-relations-result', { error: '查询条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/group-relations/search/by-conditions`;
    fetch(url, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('group-relations-result', data);
    })
    .catch(error => {
        displayResult('group-relations-result', { error: error.message });
    });
}

function createGroupRelation() {
    const firstGroupId = getInputValue('group-relation-first-id');
    const secondGroupId = getInputValue('group-relation-second-id');
    const relationType = getInputValue('group-relation-type');
    
    if (!firstGroupId || !secondGroupId) {
        displayResult('group-relations-result', { error: '请输入第一组ID和第二组ID' });
        return;
    }
    
    const groupRelationData = {
        first_group_id: parseInt(firstGroupId),
        second_group_id: parseInt(secondGroupId),
        relation_type: relationType || ''
    };
    
    const url = `${BASE_URL}/api/group-relations`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(groupRelationData)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('group-relations-result', data);
    })
    .catch(error => {
        displayResult('group-relations-result', { error: error.message });
    });
}

function deleteGroupRelation() {
    const firstGroupId = getInputValue('group-relation-first-id');
    const secondGroupId = getInputValue('group-relation-second-id');
    const relationType = getInputValue('group-relation-type');
    
    if (!firstGroupId || !secondGroupId) {
        displayResult('group-relations-result', { error: '请输入第一组ID和第二组ID' });
        return;
    }
    
    const groupRelationData = {
        first_group_id: parseInt(firstGroupId),
        second_group_id: parseInt(secondGroupId),
        relation_type: relationType || ''
    };
    
    const url = `${BASE_URL}/api/group-relations`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(groupRelationData)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('group-relations-result', data);
    })
    .catch(error => {
        displayResult('group-relations-result', { error: error.message });
    });
}

function deleteGroupRelationsByConditions() {
    const conditionsJson = getTextValue('group-relation-conditions');
    if (!conditionsJson) {
        displayResult('group-relations-result', { error: '请输入删除条件' });
        return;
    }
    
    let conditions;
    try {
        conditions = JSON.parse(conditionsJson);
    } catch (e) {
        displayResult('group-relations-result', { error: '删除条件格式错误: ' + e.message });
        return;
    }
    
    const url = `${BASE_URL}/api/group-relations/delete/by-conditions`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(conditions)
    })
    .then(response => response.json())
    .then(data => {
        displayResult('group-relations-result', data);
    })
    .catch(error => {
        displayResult('group-relations-result', { error: error.message });
    });
}