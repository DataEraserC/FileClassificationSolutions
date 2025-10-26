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

// 显示通知消息
function showMessage(message, type = 'success') {
    // 创建通知元素
    const messageElement = document.createElement('div');
    messageElement.className = `notification ${type}`;
    messageElement.innerText = message;
    
    // 添加样式
    messageElement.style.position = 'fixed';
    messageElement.style.top = '20px';
    messageElement.style.right = '20px';
    messageElement.style.padding = '15px 20px';
    messageElement.style.borderRadius = '4px';
    messageElement.style.color = 'white';
    messageElement.style.fontWeight = 'bold';
    messageElement.style.zIndex = '10000';
    messageElement.style.boxShadow = '0 2px 10px rgba(0,0,0,0.2)';
    
    // 根据类型设置背景色
    if (type === 'success') {
        messageElement.style.backgroundColor = '#28a745';
    } else if (type === 'error') {
        messageElement.style.backgroundColor = '#dc3545';
    } else if (type === 'warning') {
        messageElement.style.backgroundColor = '#ffc107';
        messageElement.style.color = '#212529';
    } else {
        messageElement.style.backgroundColor = '#17a2b8';
    }
    
    // 添加到页面
    document.body.appendChild(messageElement);
    
    // 3秒后自动移除
    setTimeout(() => {
        if (messageElement.parentNode) {
            messageElement.parentNode.removeChild(messageElement);
        }
    }, 3000);
}

// 关闭模态框
function closeModal() {
    document.getElementById('modal').style.display = 'none';
}

// ==================== 文件管理相关函数 ====================

function listFilesByFilter() {
    const fileId = getInputValue('file-id');
    const fileType = getInputValue('file-type');
    const filePath = getInputValue('file-path');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (fileId) params.append('id', fileId);
    if (fileType) params.append('type_', fileType);
    if (filePath) params.append('path', filePath);
    
    const url = `${BASE_URL}/api/files/filter?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            renderFileTable(data.data || []);
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('查询文件失败: ' + error.message, 'error');
        });
}

function renderFileTable(files) {
    const tableBody = document.querySelector('#files-table tbody');
    tableBody.innerHTML = '';
    
    files.forEach(file => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><input type="checkbox" class="file-checkbox" data-id="${file.id}"></td>
            <td>${file.id}</td>
            <td>${file.type_}</td>
            <td>${file.path}</td>
            <td>${file.reference_count}</td>
            <td>${file.group_id}</td>
            <td>
                <button class="action-button edit" onclick="openEditFileDialog(${file.id})">修改</button>
                <button class="action-button delete" onclick="deleteFile(${file.id})">删除</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

function getFileById() {
    const fileId = getInputValue('file-id');
    if (!fileId) {
        showMessage('请输入文件ID', 'warning');
        return;
    }
    
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                renderFileTable([data.data]);
            } else {
                showMessage('获取文件失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取文件失败: ' + error.message, 'error');
        });
}

function createFile() {
    const fileType = getInputValue('create-file-type');
    const filePath = getInputValue('create-file-path');
    const groupId = getInputValue('create-file-group-id');
    
    if (!fileType || !filePath || !groupId) {
        showMessage('请填写完整的文件信息', 'warning');
        return;
    }
    
    const fileData = {
        type_: fileType,
        path: filePath,
        group_id: parseInt(groupId)
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
        if (data.success) {
            showMessage('文件创建成功', 'success');
            closeModal();
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            showMessage('文件创建失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件创建失败: ' + error.message, 'error');
    });
}

function updateFile() {
    const fileId = getInputValue('edit-file-id');
    const fileType = getInputValue('edit-file-type');
    const filePath = getInputValue('edit-file-path');
    const groupId = getInputValue('edit-file-group-id');
    
    if (!fileId || !fileType || !filePath || !groupId) {
        showMessage('请填写完整的文件信息', 'warning');
        return;
    }
    
    const updateData = {
        type_: fileType,
        path: filePath,
        group_id: parseInt(groupId)
    };
    
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(updateData)
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            showMessage('文件更新成功', 'success');
            closeModal();
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            showMessage('文件更新失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件更新失败: ' + error.message, 'error');
    });
}

function deleteFile(fileId) {
    if (!confirm('确定要删除该文件吗？')) {
        return;
    }
    
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url, {
        method: 'DELETE'
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            showMessage('文件删除成功', 'success');
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            showMessage('文件删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件删除失败: ' + error.message, 'error');
    });
}

// 批量删除选中的文件
function deleteSelectedFiles() {
    const selectedCheckboxes = document.querySelectorAll('.file-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个文件进行删除', 'warning');
        return;
    }
    
    if (!confirm(`确定要删除这 ${selectedCheckboxes.length} 个文件吗？`)) {
        return;
    }
    
    const ids = Array.from(selectedCheckboxes).map(cb => parseInt(cb.getAttribute('data-id')));
    
    // 构造删除条件
    const conditions = ids.map(id => ({ Id: id }));
    
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
        if (data.success) {
            showMessage(`成功删除 ${data.count} 个文件`, 'success');
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            showMessage('文件批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件批量删除失败: ' + error.message, 'error');
    });
}

// 全选/取消全选文件
function toggleAllFiles(source) {
    const checkboxes = document.querySelectorAll('.file-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// ==================== 组管理相关函数 ====================

function listGroupsByFilter() {
    const groupId = getInputValue('group-id');
    const groupName = getInputValue('group-name');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (groupId) params.append('id', groupId);
    if (groupName) params.append('name', groupName);
    
    const url = `${BASE_URL}/api/groups/filter?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            renderGroupTable(data.data || []);
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('查询组失败: ' + error.message, 'error');
        });
}

function renderGroupTable(groups) {
    const tableBody = document.querySelector('#groups-table tbody');
    tableBody.innerHTML = '';
    
    groups.forEach(group => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><input type="checkbox" class="group-checkbox" data-id="${group.id}"></td>
            <td>${group.id}</td>
            <td>${group.name}</td>
            <td>${group.description}</td>
            <td>${group.reference_count}</td>
            <td>${group.parent_group_id}</td>
            <td>
                <button class="action-button edit" onclick="openEditGroupDialog(${group.id})">修改</button>
                <button class="action-button delete" onclick="deleteGroup(${group.id})">删除</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

function createGroup() {
    const groupName = getInputValue('create-group-name');
    const groupDescription = getInputValue('create-group-description');
    
    if (!groupName) {
        showMessage('请输入组名', 'warning');
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
        if (data.success) {
            showMessage('组创建成功', 'success');
            closeModal();
            // 重新加载组列表
            listGroupsByFilter();
        } else {
            showMessage('组创建失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组创建失败: ' + error.message, 'error');
    });
}

function updateGroup() {
    const groupId = getInputValue('edit-group-id');
    const groupName = getInputValue('edit-group-name');
    const groupDescription = getInputValue('edit-group-description');
    
    if (!groupId || !groupName) {
        showMessage('请填写完整的组信息', 'warning');
        return;
    }
    
    const updateData = {
        name: groupName,
        description: groupDescription || ''
    };
    
    const url = `${BASE_URL}/api/groups/${groupId}`;
    fetch(url, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(updateData)
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            showMessage('组更新成功', 'success');
            closeModal();
            // 重新加载组列表
            listGroupsByFilter();
        } else {
            showMessage('组更新失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组更新失败: ' + error.message, 'error');
    });
}

function deleteGroup(groupId) {
    if (!confirm('确定要删除该组吗？')) {
        return;
    }
    
    const url = `${BASE_URL}/api/groups/${groupId}`;
    fetch(url, {
        method: 'DELETE'
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            showMessage('组删除成功', 'success');
            // 重新加载组列表
            listGroupsByFilter();
        } else {
            showMessage('组删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组删除失败: ' + error.message, 'error');
    });
}

// 批量删除选中的组
function deleteSelectedGroups() {
    const selectedCheckboxes = document.querySelectorAll('.group-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个组进行删除', 'warning');
        return;
    }
    
    if (!confirm(`确定要删除这 ${selectedCheckboxes.length} 个组吗？`)) {
        return;
    }
    
    const ids = Array.from(selectedCheckboxes).map(cb => parseInt(cb.getAttribute('data-id')));
    
    // 构造删除条件
    const conditions = ids.map(id => ({ Id: id }));
    
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
        if (data.success) {
            showMessage(`成功删除 ${data.count} 个组`, 'success');
            // 重新加载组列表
            listGroupsByFilter();
        } else {
            showMessage('组批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组批量删除失败: ' + error.message, 'error');
    });
}

// 全选/取消全选组
function toggleAllGroups(source) {
    const checkboxes = document.querySelectorAll('.group-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// ==================== 标签管理相关函数 ====================

function listTagsByFilter() {
    const tagId = getInputValue('tag-id');
    const tagName = getInputValue('tag-name');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (tagId) params.append('id', tagId);
    if (tagName) params.append('name', tagName);
    
    const url = `${BASE_URL}/api/tags/filter?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            renderTagTable(data.data || []);
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('查询标签失败: ' + error.message, 'error');
        });
}

function renderTagTable(tags) {
    const tableBody = document.querySelector('#tags-table tbody');
    tableBody.innerHTML = '';
    
    tags.forEach(tag => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><input type="checkbox" class="tag-checkbox" data-id="${tag.id}"></td>
            <td>${tag.id}</td>
            <td>${tag.name}</td>
            <td>${tag.description}</td>
            <td>${tag.reference_count}</td>
            <td>
                <button class="action-button edit" onclick="openEditTagDialog(${tag.id})">修改</button>
                <button class="action-button delete" onclick="deleteTag(${tag.id})">删除</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

function createTag() {
    const tagName = getInputValue('create-tag-name');
    const tagDescription = getInputValue('create-tag-description');
    
    if (!tagName) {
        showMessage('请输入标签名', 'warning');
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
        if (data.success) {
            showMessage('标签创建成功', 'success');
            closeModal();
            // 重新加载标签列表
            listTagsByFilter();
        } else {
            showMessage('标签创建失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('标签创建失败: ' + error.message, 'error');
    });
}

function updateTag() {
    const tagId = getInputValue('edit-tag-id');
    const tagName = getInputValue('edit-tag-name');
    const tagDescription = getInputValue('edit-tag-description');
    
    if (!tagId || !tagName) {
        showMessage('请填写完整的标签信息', 'warning');
        return;
    }
    
    const updateData = {
        name: tagName,
        description: tagDescription || ''
    };
    
    const url = `${BASE_URL}/api/tags/${tagId}`;
    fetch(url, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(updateData)
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            showMessage('标签更新成功', 'success');
            closeModal();
            // 重新加载标签列表
            listTagsByFilter();
        } else {
            showMessage('标签更新失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('标签更新失败: ' + error.message, 'error');
    });
}

function deleteTag(tagId) {
    if (!confirm('确定要删除该标签吗？')) {
        return;
    }
    
    const url = `${BASE_URL}/api/tags/${tagId}`;
    fetch(url, {
        method: 'DELETE'
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            showMessage('标签删除成功', 'success');
            // 重新加载标签列表
            listTagsByFilter();
        } else {
            showMessage('标签删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('标签删除失败: ' + error.message, 'error');
    });
}

// 批量删除选中的标签
function deleteSelectedTags() {
    const selectedCheckboxes = document.querySelectorAll('.tag-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个标签进行删除', 'warning');
        return;
    }
    
    if (!confirm(`确定要删除这 ${selectedCheckboxes.length} 个标签吗？`)) {
        return;
    }
    
    const ids = Array.from(selectedCheckboxes).map(cb => parseInt(cb.getAttribute('data-id')));
    
    // 构造删除条件
    const conditions = ids.map(id => ({ Id: id }));
    
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
        if (data.success) {
            showMessage(`成功删除 ${data.count} 个标签`, 'success');
            // 重新加载标签列表
            listTagsByFilter();
        } else {
            showMessage('标签批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('标签批量删除失败: ' + error.message, 'error');
    });
}

// 全选/取消全选标签
function toggleAllTags(source) {
    const checkboxes = document.querySelectorAll('.tag-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// ==================== 文件组关联相关函数 ====================

function listFileGroupsByFilter() {
    const fileId = getInputValue('file-group-file-id');
    const groupId = getInputValue('file-group-group-id');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (fileId) params.append('file_id', fileId);
    if (groupId) params.append('group_id', groupId);
    
    const url = `${BASE_URL}/api/file-groups/filter?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            renderFileGroupTable(data.data || []);
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('查询文件组关联失败: ' + error.message, 'error');
        });
}

function renderFileGroupTable(fileGroups) {
    const tableBody = document.querySelector('#file-groups-table tbody');
    tableBody.innerHTML = '';
    
    fileGroups.forEach(fileGroup => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><input type="checkbox" class="file-group-checkbox" data-file-id="${fileGroup.file_id}" data-group-id="${fileGroup.group_id}"></td>
            <td>${fileGroup.file_id}</td>
            <td>${fileGroup.group_id}</td>
            <td>
                <button class="action-button delete" onclick="deleteFileGroup(${fileGroup.file_id}, ${fileGroup.group_id})">删除</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

function createFileGroup() {
    const fileId = getInputValue('create-file-group-file-id');
    const groupId = getInputValue('create-file-group-group-id');
    
    if (!fileId || !groupId) {
        showMessage('请填写完整的文件组关联信息', 'warning');
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
        if (data.success) {
            showMessage('文件组关联创建成功', 'success');
            closeModal();
            // 重新加载文件组列表
            listFileGroupsByFilter();
        } else {
            showMessage('文件组关联创建失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件组关联创建失败: ' + error.message, 'error');
    });
}

function deleteFileGroup(fileId, groupId) {
    if (!confirm('确定要删除该文件组关联吗？')) {
        return;
    }
    
    const fileGroupData = {
        file_id: fileId,
        group_id: groupId
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
        if (data.success) {
            showMessage('文件组关联删除成功', 'success');
            // 重新加载文件组列表
            listFileGroupsByFilter();
        } else {
            showMessage('文件组关联删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件组关联删除失败: ' + error.message, 'error');
    });
}

// 批量删除选中的文件组关联
function deleteSelectedFileGroups() {
    const selectedCheckboxes = document.querySelectorAll('.file-group-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个文件组关联进行删除', 'warning');
        return;
    }
    
    if (!confirm(`确定要删除这 ${selectedCheckboxes.length} 个文件组关联吗？`)) {
        return;
    }
    
    // 构造删除条件
    const conditions = Array.from(selectedCheckboxes).map(cb => {
        return {
            FileId: parseInt(cb.getAttribute('data-file-id')),
            GroupId: parseInt(cb.getAttribute('data-group-id'))
        };
    });
    
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
        if (data.success) {
            showMessage(`成功删除 ${data.count} 个文件组关联`, 'success');
            // 重新加载文件组列表
            listFileGroupsByFilter();
        } else {
            showMessage('文件组关联批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件组关联批量删除失败: ' + error.message, 'error');
    });
}

// 全选/取消全选文件组关联
function toggleAllFileGroups(source) {
    const checkboxes = document.querySelectorAll('.file-group-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// 重置文件组关联搜索表单
function resetFileGroupFilter() {
    document.getElementById('file-group-file-id').value = '';
    document.getElementById('file-group-group-id').value = '';
    listFileGroupsByFilter(); // 重置后重新搜索
}

// ==================== 组标签关联相关函数 ====================

function listGroupTagsByFilter() {
    const groupId = getInputValue('group-tag-group-id');
    const tagId = getInputValue('group-tag-tag-id');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (groupId) params.append('group_id', groupId);
    if (tagId) params.append('tag_id', tagId);
    
    const url = `${BASE_URL}/api/group-tags/filter?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            renderGroupTagTable(data.data || []);
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('查询组标签关联失败: ' + error.message, 'error');
        });
}

function renderGroupTagTable(groupTags) {
    const tableBody = document.querySelector('#group-tags-table tbody');
    tableBody.innerHTML = '';
    
    groupTags.forEach(groupTag => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><input type="checkbox" class="group-tag-checkbox" data-group-id="${groupTag.group_id}" data-tag-id="${groupTag.tag_id}"></td>
            <td>${groupTag.group_id}</td>
            <td>${groupTag.tag_id}</td>
            <td>
                <button class="action-button delete" onclick="deleteGroupTag(${groupTag.group_id}, ${groupTag.tag_id})">删除</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

function createGroupTag() {
    const groupId = getInputValue('create-group-tag-group-id');
    const tagId = getInputValue('create-group-tag-tag-id');
    
    if (!groupId || !tagId) {
        showMessage('请填写完整的组标签关联信息', 'warning');
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
        if (data.success) {
            showMessage('组标签关联创建成功', 'success');
            closeModal();
            // 重新加载组标签列表
            listGroupTagsByFilter();
        } else {
            showMessage('组标签关联创建失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组标签关联创建失败: ' + error.message, 'error');
    });
}

function deleteGroupTag(groupId, tagId) {
    if (!confirm('确定要删除该组标签关联吗？')) {
        return;
    }
    
    const groupTagData = {
        group_id: groupId,
        tag_id: tagId
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
        if (data.success) {
            showMessage('组标签关联删除成功', 'success');
            // 重新加载组标签列表
            listGroupTagsByFilter();
        } else {
            showMessage('组标签关联删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组标签关联删除失败: ' + error.message, 'error');
    });
}

// 批量删除选中的组标签关联
function deleteSelectedGroupTags() {
    const selectedCheckboxes = document.querySelectorAll('.group-tag-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个组标签关联进行删除', 'warning');
        return;
    }
    
    if (!confirm(`确定要删除这 ${selectedCheckboxes.length} 个组标签关联吗？`)) {
        return;
    }
    
    // 构造删除条件
    const conditions = Array.from(selectedCheckboxes).map(cb => {
        return {
            GroupId: parseInt(cb.getAttribute('data-group-id')),
            TagId: parseInt(cb.getAttribute('data-tag-id'))
        };
    });
    
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
        if (data.success) {
            showMessage(`成功删除 ${data.count} 个组标签关联`, 'success');
            // 重新加载组标签列表
            listGroupTagsByFilter();
        } else {
            showMessage('组标签关联批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组标签关联批量删除失败: ' + error.message, 'error');
    });
}

// 全选/取消全选组标签关联
function toggleAllGroupTags(source) {
    const checkboxes = document.querySelectorAll('.group-tag-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// 重置组标签关联搜索表单
function resetGroupTagFilter() {
    document.getElementById('group-tag-group-id').value = '';
    document.getElementById('group-tag-tag-id').value = '';
    listGroupTagsByFilter(); // 重置后重新搜索
}

// ==================== 组关系管理相关函数 ====================

function listGroupRelationsByFilter() {
    const firstId = getInputValue('group-relation-first-id');
    const secondId = getInputValue('group-relation-second-id');
    const relationType = getInputValue('group-relation-type');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (firstId) params.append('first_group_id', firstId);
    if (secondId) params.append('second_group_id', secondId);
    if (relationType) params.append('relation_type', relationType);
    
    const url = `${BASE_URL}/api/group-relations/filter?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            renderGroupRelationTable(data.data || []);
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('查询组关系失败: ' + error.message, 'error');
        });
}

function renderGroupRelationTable(groupRelations) {
    const tableBody = document.querySelector('#group-relations-table tbody');
    tableBody.innerHTML = '';
    
    groupRelations.forEach(groupRelation => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><input type="checkbox" class="group-relation-checkbox" data-first-id="${groupRelation.first_group_id}" data-second-id="${groupRelation.second_group_id}" data-relation-type="${groupRelation.relation_type}"></td>
            <td>${groupRelation.first_group_id}</td>
            <td>${groupRelation.second_group_id}</td>
            <td>${groupRelation.relation_type}</td>
            <td>
                <button class="action-button delete" onclick="deleteGroupRelation(${groupRelation.first_group_id}, ${groupRelation.second_group_id}, '${groupRelation.relation_type}')">删除</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

function createGroupRelation() {
    const firstId = getInputValue('create-group-relation-first-id');
    const secondId = getInputValue('create-group-relation-second-id');
    const relationType = getInputValue('create-group-relation-type');
    
    if (!firstId || !secondId || !relationType) {
        showMessage('请填写完整的组关系信息', 'warning');
        return;
    }
    
    const groupRelationData = {
        first_group_id: parseInt(firstId),
        second_group_id: parseInt(secondId),
        relation_type: relationType
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
        if (data.success) {
            showMessage('组关系创建成功', 'success');
            closeModal();
            // 重新加载组关系列表
            listGroupRelationsByFilter();
        } else {
            showMessage('组关系创建失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组关系创建失败: ' + error.message, 'error');
    });
}

function deleteGroupRelation(firstId, secondId, relationType) {
    if (!confirm('确定要删除该组关系吗？')) {
        return;
    }
    
    const groupRelationData = {
        first_group_id: firstId,
        second_group_id: secondId,
        relation_type: relationType
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
        if (data.success) {
            showMessage('组关系删除成功', 'success');
            // 重新加载组关系列表
            listGroupRelationsByFilter();
        } else {
            showMessage('组关系删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组关系删除失败: ' + error.message, 'error');
    });
}

// 批量删除选中的组关系
function deleteSelectedGroupRelations() {
    const selectedCheckboxes = document.querySelectorAll('.group-relation-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个组关系进行删除', 'warning');
        return;
    }
    
    if (!confirm(`确定要删除这 ${selectedCheckboxes.length} 个组关系吗？`)) {
        return;
    }
    
    // 构造删除条件
    const conditions = Array.from(selectedCheckboxes).map(cb => {
        return {
            FirstGroupId: parseInt(cb.getAttribute('data-first-id')),
            SecondGroupId: parseInt(cb.getAttribute('data-second-id')),
            RelationType: cb.getAttribute('data-relation-type')
        };
    });
    
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
        if (data.success) {
            showMessage(`成功删除 ${data.count} 个组关系`, 'success');
            // 重新加载组关系列表
            listGroupRelationsByFilter();
        } else {
            showMessage('组关系批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组关系批量删除失败: ' + error.message, 'error');
    });
}

// 全选/取消全选组关系
function toggleAllGroupRelations(source) {
    const checkboxes = document.querySelectorAll('.group-relation-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// 重置组关系搜索表单
function resetGroupRelationFilter() {
    document.getElementById('group-relation-first-id').value = '';
    document.getElementById('group-relation-second-id').value = '';
    document.getElementById('group-relation-type').value = '';
    listGroupRelationsByFilter(); // 重置后重新搜索
}

// 打开创建文件对话框
function openCreateFileDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>新增文件</h2>
        <form id="create-file-form">
            <div class="form-group">
                <label for="create-file-type">文件类型:</label>
                <input type="text" id="create-file-type" required>
            </div>
            <div class="form-group">
                <label for="create-file-path">文件路径:</label>
                <input type="text" id="create-file-path" required>
            </div>
            <div class="form-group">
                <label for="create-file-group-id">组ID:</label>
                <input type="number" id="create-file-group-id" required>
            </div>
            <button type="submit">创建</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-file-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createFile();
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 打开编辑文件对话框
function openEditFileDialog(fileId) {
    // 首先获取文件信息
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                const file = data.data;
                const modalBody = document.getElementById('modal-body');
                modalBody.innerHTML = `
                    <h2>编辑文件</h2>
                    <form id="edit-file-form">
                        <div class="form-group">
                            <label for="edit-file-id">文件ID:</label>
                            <input type="number" id="edit-file-id" value="${file.id}" readonly>
                        </div>
                        <div class="form-group">
                            <label for="edit-file-type">文件类型:</label>
                            <input type="text" id="edit-file-type" value="${file.type_}" required>
                        </div>
                        <div class="form-group">
                            <label for="edit-file-path">文件路径:</label>
                            <input type="text" id="edit-file-path" value="${file.path}" required>
                        </div>
                        <div class="form-group">
                            <label for="edit-file-group-id">组ID:</label>
                            <input type="number" id="edit-file-group-id" value="${file.group_id}" required>
                        </div>
                        <button type="submit">更新</button>
                        <button type="button" onclick="closeModal()">取消</button>
                    </form>
                `;
                
                // 绑定表单提交事件
                document.getElementById('edit-file-form').addEventListener('submit', function(e) {
                    e.preventDefault();
                    updateFile();
                });
                
                document.getElementById('modal').style.display = 'block';
            } else {
                showMessage('获取文件信息失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取文件信息失败: ' + error.message, 'error');
        });
}

// 打开创建组对话框
function openCreateGroupDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>新增组</h2>
        <form id="create-group-form">
            <div class="form-group">
                <label for="create-group-name">组名:</label>
                <input type="text" id="create-group-name" required>
            </div>
            <div class="form-group">
                <label for="create-group-description">描述:</label>
                <input type="text" id="create-group-description">
            </div>
            <button type="submit">创建</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-group-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createGroup();
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 打开编辑组对话框
function openEditGroupDialog(groupId) {
    // 首先获取组信息
    const url = `${BASE_URL}/api/groups/${groupId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                const group = data.data;
                const modalBody = document.getElementById('modal-body');
                modalBody.innerHTML = `
                    <h2>编辑组</h2>
                    <form id="edit-group-form">
                        <div class="form-group">
                            <label for="edit-group-id">组ID:</label>
                            <input type="number" id="edit-group-id" value="${group.id}" readonly>
                        </div>
                        <div class="form-group">
                            <label for="edit-group-name">组名:</label>
                            <input type="text" id="edit-group-name" value="${group.name}" required>
                        </div>
                        <div class="form-group">
                            <label for="edit-group-description">描述:</label>
                            <input type="text" id="edit-group-description" value="${group.description || ''}">
                        </div>
                        <button type="submit">更新</button>
                        <button type="button" onclick="closeModal()">取消</button>
                    </form>
                `;
                
                // 绑定表单提交事件
                document.getElementById('edit-group-form').addEventListener('submit', function(e) {
                    e.preventDefault();
                    updateGroup();
                });
                
                document.getElementById('modal').style.display = 'block';
            } else {
                showMessage('获取组信息失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取组信息失败: ' + error.message, 'error');
        });
}

// 打开创建标签对话框
function openCreateTagDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>新增标签</h2>
        <form id="create-tag-form">
            <div class="form-group">
                <label for="create-tag-name">标签名:</label>
                <input type="text" id="create-tag-name" required>
            </div>
            <div class="form-group">
                <label for="create-tag-description">描述:</label>
                <input type="text" id="create-tag-description">
            </div>
            <button type="submit">创建</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-tag-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createTag();
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 打开编辑标签对话框
function openEditTagDialog(tagId) {
    // 首先获取标签信息
    const url = `${BASE_URL}/api/tags/${tagId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                const tag = data.data;
                const modalBody = document.getElementById('modal-body');
                modalBody.innerHTML = `
                    <h2>编辑标签</h2>
                    <form id="edit-tag-form">
                        <div class="form-group">
                            <label for="edit-tag-id">标签ID:</label>
                            <input type="number" id="edit-tag-id" value="${tag.id}" readonly>
                        </div>
                        <div class="form-group">
                            <label for="edit-tag-name">标签名:</label>
                            <input type="text" id="edit-tag-name" value="${tag.name}" required>
                        </div>
                        <div class="form-group">
                            <label for="edit-tag-description">描述:</label>
                            <input type="text" id="edit-tag-description" value="${tag.description || ''}">
                        </div>
                        <button type="submit">更新</button>
                        <button type="button" onclick="closeModal()">取消</button>
                    </form>
                `;
                
                // 绑定表单提交事件
                document.getElementById('edit-tag-form').addEventListener('submit', function(e) {
                    e.preventDefault();
                    updateTag();
                });
                
                document.getElementById('modal').style.display = 'block';
            } else {
                showMessage('获取标签信息失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取标签信息失败: ' + error.message, 'error');
        });
}

// 打开创建文件组关联对话框
function openCreateFileGroupDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>新增文件组关联</h2>
        <form id="create-file-group-form">
            <div class="form-group">
                <label for="create-file-group-file-id">文件ID:</label>
                <input type="number" id="create-file-group-file-id" required>
            </div>
            <div class="form-group">
                <label for="create-file-group-group-id">组ID:</label>
                <input type="number" id="create-file-group-group-id" required>
            </div>
            <button type="submit">创建</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-file-group-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createFileGroup();
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 打开创建组标签关联对话框
function openCreateGroupTagDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>新增组标签关联</h2>
        <form id="create-group-tag-form">
            <div class="form-group">
                <label for="create-group-tag-group-id">组ID:</label>
                <input type="number" id="create-group-tag-group-id" required>
            </div>
            <div class="form-group">
                <label for="create-group-tag-tag-id">标签ID:</label>
                <input type="number" id="create-group-tag-tag-id" required>
            </div>
            <button type="submit">创建</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-group-tag-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createGroupTag();
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 打开创建组关系对话框
function openCreateGroupRelationDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>新增组关系</h2>
        <form id="create-group-relation-form">
            <div class="form-group">
                <label for="create-group-relation-first-id">第一组ID:</label>
                <input type="number" id="create-group-relation-first-id" required>
            </div>
            <div class="form-group">
                <label for="create-group-relation-second-id">第二组ID:</label>
                <input type="number" id="create-group-relation-second-id" required>
            </div>
            <div class="form-group">
                <label for="create-group-relation-type">关系类型:</label>
                <input type="text" id="create-group-relation-type" required>
            </div>
            <button type="submit">创建</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-group-relation-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createGroupRelation();
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 打开批量删除文件对话框
function openBatchDeleteFileDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>批量删除文件</h2>
        <form id="batch-delete-file-form">
            <div class="form-group">
                <label for="batch-delete-file-conditions">删除条件 (JSON格式):</label>
                <textarea id="batch-delete-file-conditions" rows="5" placeholder='[{"Id": 1}, {"Type": "txt"}]'></textarea>
            </div>
            <button type="submit">删除</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-file-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('batch-delete-file-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入删除条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            deleteFilesByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function deleteFilesByConditions(conditions) {
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
        if (data.success) {
            showMessage('文件批量删除成功', 'success');
            closeModal();
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            showMessage('文件批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件批量删除失败: ' + error.message, 'error');
    });
}

// 打开复杂查询文件对话框
function openComplexSearchFileDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询文件</h2>
        <div class="tabs">
            <button class="tab-button active" onclick="switchComplexSearchTab('visual')">可视化查询</button>
            <button class="tab-button" onclick="switchComplexSearchTab('json')">JSON查询</button>
        </div>
        <div id="visual-search" class="tab-content active">
            <form id="visual-file-search-form">
                <div class="form-group">
                    <label for="visual-search-field">查询字段:</label>
                    <select id="visual-search-field">
                        <option value="Id">ID</option>
                        <option value="Type">类型</option>
                        <option value="Path">路径</option>
                        <option value="ReferenceCount">引用计数</option>
                        <option value="GroupId">组ID</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-operator">操作符:</label>
                    <select id="visual-search-operator">
                        <option value="equal">等于</option>
                        <option value="like">包含</option>
                        <option value="greater">大于</option>
                        <option value="less">小于</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-value">值:</label>
                    <input type="text" id="visual-search-value">
                </div>
                <div class="form-group">
                    <button type="button" onclick="addVisualSearchCondition()">添加条件</button>
                </div>
                <div class="form-group">
                    <label>已添加的条件:</label>
                    <div id="visual-search-conditions"></div>
                </div>
                <button type="button" onclick="performVisualSearch()">查询</button>
            </form>
        </div>
        <div id="json-search" class="tab-content" style="display: none;">
            <form id="json-file-search-form">
                <div class="form-group">
                    <label for="complex-search-file-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-file-conditions" rows="5" placeholder='[{"Id": 1}, {"Type": "txt"}]'></textarea>
                </div>
                <button type="submit">查询</button>
            </form>
        </div>
        <button type="button" onclick="closeModal()">取消</button>
    `;
    
    // 绑定表单提交事件
    document.getElementById('json-file-search-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-file-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入查询条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            searchFilesByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 切换复杂查询标签
function switchComplexSearchTab(tab) {
    // 更新标签按钮状态
    document.querySelectorAll('.tab-button').forEach(button => {
        button.classList.remove('active');
    });
    event.target.classList.add('active');
    
    // 显示对应的标签内容
    if (tab === 'visual') {
        document.getElementById('visual-search').style.display = 'block';
        document.getElementById('json-search').style.display = 'none';
    } else {
        document.getElementById('visual-search').style.display = 'none';
        document.getElementById('json-search').style.display = 'block';
    }
}

// 添加可视化查询条件
function addVisualSearchCondition() {
    const field = document.getElementById('visual-search-field').value;
    const operator = document.getElementById('visual-search-operator').value;
    const value = document.getElementById('visual-search-value').value;
    
    if (!value) {
        showMessage('请输入值', 'warning');
        return;
    }
    
    const conditionsContainer = document.getElementById('visual-search-conditions');
    const conditionElement = document.createElement('div');
    conditionElement.className = 'condition-item';
    conditionElement.innerHTML = `
        <span>${field} ${operator} ${value}</span>
        <button type="button" onclick="this.parentElement.remove()">删除</button>
        <input type="hidden" class="condition-data" value='{"${field}": "${value}"}'>
    `;
    conditionsContainer.appendChild(conditionElement);
    
    // 清空输入
    document.getElementById('visual-search-value').value = '';
}

// 执行可视化查询
function performVisualSearch() {
    const conditionElements = document.querySelectorAll('.condition-data');
    const conditions = [];
    
    conditionElements.forEach(element => {
        try {
            const condition = JSON.parse(element.value);
            conditions.push(condition);
        } catch (e) {
            console.error('Error parsing condition:', e);
        }
    });
    
    if (conditions.length > 0) {
        searchFilesByConditions(conditions);
    } else {
        showMessage('请添加至少一个查询条件', 'warning');
    }
}

function searchFilesByConditions(conditions) {
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
        if (data.success) {
            closeModal();
            renderFileTable(data.data || []);
        } else {
            showMessage('文件查询失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件查询失败: ' + error.message, 'error');
    });
}

// 打开批量删除组对话框
function openBatchDeleteGroupDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>批量删除组</h2>
        <form id="batch-delete-group-form">
            <div class="form-group">
                <label for="batch-delete-group-conditions">删除条件 (JSON格式):</label>
                <textarea id="batch-delete-group-conditions" rows="5" placeholder='[{"Id": 1}, {"Name": "example"}]'></textarea>
            </div>
            <button type="submit">删除</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-group-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('batch-delete-group-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入删除条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            deleteGroupsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function deleteGroupsByConditions(conditions) {
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
        if (data.success) {
            showMessage('组批量删除成功', 'success');
            closeModal();
            // 重新加载组列表
            listGroupsByFilter();
        } else {
            showMessage('组批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组批量删除失败: ' + error.message, 'error');
    });
}

// 打开复杂查询组对话框
function openComplexSearchGroupDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询组</h2>
        <div class="tabs">
            <button class="tab-button active" onclick="switchComplexSearchTab('visual')">可视化查询</button>
            <button class="tab-button" onclick="switchComplexSearchTab('json')">JSON查询</button>
        </div>
        <div id="visual-search" class="tab-content active">
            <form id="visual-group-search-form">
                <div class="form-group">
                    <label for="visual-search-field">查询字段:</label>
                    <select id="visual-search-field">
                        <option value="Id">ID</option>
                        <option value="Name">名称</option>
                        <option value="Description">描述</option>
                        <option value="ReferenceCount">引用计数</option>
                        <option value="ParentGroupId">父组ID</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-operator">操作符:</label>
                    <select id="visual-search-operator">
                        <option value="equal">等于</option>
                        <option value="like">包含</option>
                        <option value="greater">大于</option>
                        <option value="less">小于</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-value">值:</label>
                    <input type="text" id="visual-search-value">
                </div>
                <div class="form-group">
                    <button type="button" onclick="addVisualSearchCondition()">添加条件</button>
                </div>
                <div class="form-group">
                    <label>已添加的条件:</label>
                    <div id="visual-search-conditions"></div>
                </div>
                <button type="button" onclick="performVisualSearch()">查询</button>
            </form>
        </div>
        <div id="json-search" class="tab-content" style="display: none;">
            <form id="json-group-search-form">
                <div class="form-group">
                    <label for="complex-search-group-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-group-conditions" rows="5" placeholder='[{"Id": 1}, {"Name": "example"}]'></textarea>
                </div>
                <button type="submit">查询</button>
            </form>
        </div>
        <button type="button" onclick="closeModal()">取消</button>
    `;
    
    // 绑定表单提交事件
    document.getElementById('json-group-search-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-group-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入查询条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            searchGroupsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function searchGroupsByConditions(conditions) {
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
        if (data.success) {
            closeModal();
            renderGroupTable(data.data || []);
        } else {
            showMessage('组查询失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组查询失败: ' + error.message, 'error');
    });
}

// 打开批量删除标签对话框
function openBatchDeleteTagDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>批量删除标签</h2>
        <form id="batch-delete-tag-form">
            <div class="form-group">
                <label for="batch-delete-tag-conditions">删除条件 (JSON格式):</label>
                <textarea id="batch-delete-tag-conditions" rows="5" placeholder='[{"Id": 1}, {"Name": "example"}]'></textarea>
            </div>
            <button type="submit">删除</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-tag-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('batch-delete-tag-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入删除条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            deleteTagsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function deleteTagsByConditions(conditions) {
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
        if (data.success) {
            showMessage('标签批量删除成功', 'success');
            closeModal();
            // 重新加载标签列表
            listTagsByFilter();
        } else {
            showMessage('标签批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('标签批量删除失败: ' + error.message, 'error');
    });
}

// 打开复杂查询标签对话框
function openComplexSearchTagDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询标签</h2>
        <div class="tabs">
            <button class="tab-button active" onclick="switchComplexSearchTab('visual')">可视化查询</button>
            <button class="tab-button" onclick="switchComplexSearchTab('json')">JSON查询</button>
        </div>
        <div id="visual-search" class="tab-content active">
            <form id="visual-tag-search-form">
                <div class="form-group">
                    <label for="visual-search-field">查询字段:</label>
                    <select id="visual-search-field">
                        <option value="Id">ID</option>
                        <option value="Name">名称</option>
                        <option value="Description">描述</option>
                        <option value="ReferenceCount">引用计数</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-operator">操作符:</label>
                    <select id="visual-search-operator">
                        <option value="equal">等于</option>
                        <option value="like">包含</option>
                        <option value="greater">大于</option>
                        <option value="less">小于</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-value">值:</label>
                    <input type="text" id="visual-search-value">
                </div>
                <div class="form-group">
                    <button type="button" onclick="addVisualSearchCondition()">添加条件</button>
                </div>
                <div class="form-group">
                    <label>已添加的条件:</label>
                    <div id="visual-search-conditions"></div>
                </div>
                <button type="button" onclick="performVisualSearch()">查询</button>
            </form>
        </div>
        <div id="json-search" class="tab-content" style="display: none;">
            <form id="json-tag-search-form">
                <div class="form-group">
                    <label for="complex-search-tag-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-tag-conditions" rows="5" placeholder='[{"Id": 1}, {"Name": "example"}]'></textarea>
                </div>
                <button type="submit">查询</button>
            </form>
        </div>
        <button type="button" onclick="closeModal()">取消</button>
    `;
    
    // 绑定表单提交事件
    document.getElementById('json-tag-search-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-tag-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入查询条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            searchTagsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function searchTagsByConditions(conditions) {
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
        if (data.success) {
            closeModal();
            renderTagTable(data.data || []);
        } else {
            showMessage('标签查询失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('标签查询失败: ' + error.message, 'error');
    });
}

// 打开批量删除文件组对话框
function openBatchDeleteFileGroupDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>批量删除文件组关联</h2>
        <form id="batch-delete-file-group-form">
            <div class="form-group">
                <label for="batch-delete-file-group-conditions">删除条件 (JSON格式):</label>
                <textarea id="batch-delete-file-group-conditions" rows="5" placeholder='[{"FileId": 1, "GroupId": 2}]'></textarea>
            </div>
            <button type="submit">删除</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-file-group-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('batch-delete-file-group-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入删除条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            deleteFileGroupsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function deleteFileGroupsByConditions(conditions) {
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
        if (data.success) {
            showMessage('文件组关联批量删除成功', 'success');
            closeModal();
            // 重新加载文件组列表
            listFileGroupsByFilter();
        } else {
            showMessage('文件组关联批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件组关联批量删除失败: ' + error.message, 'error');
    });
}

// 打开复杂查询文件组对话框
function openComplexSearchFileGroupDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询文件组关联</h2>
        <div class="tabs">
            <button class="tab-button active" onclick="switchComplexSearchTab('visual')">可视化查询</button>
            <button class="tab-button" onclick="switchComplexSearchTab('json')">JSON查询</button>
        </div>
        <div id="visual-search" class="tab-content active">
            <form id="visual-file-group-search-form">
                <div class="form-group">
                    <label for="visual-search-field">查询字段:</label>
                    <select id="visual-search-field">
                        <option value="FileId">文件ID</option>
                        <option value="GroupId">组ID</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-operator">操作符:</label>
                    <select id="visual-search-operator">
                        <option value="equal">等于</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-value">值:</label>
                    <input type="text" id="visual-search-value">
                </div>
                <div class="form-group">
                    <button type="button" onclick="addVisualSearchCondition()">添加条件</button>
                </div>
                <div class="form-group">
                    <label>已添加的条件:</label>
                    <div id="visual-search-conditions"></div>
                </div>
                <button type="button" onclick="performVisualSearch()">查询</button>
            </form>
        </div>
        <div id="json-search" class="tab-content" style="display: none;">
            <form id="json-file-group-search-form">
                <div class="form-group">
                    <label for="complex-search-file-group-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-file-group-conditions" rows="5" placeholder='[{"FileId": 1, "GroupId": 2}]'></textarea>
                </div>
                <button type="submit">查询</button>
            </form>
        </div>
        <button type="button" onclick="closeModal()">取消</button>
    `;
    
    // 绑定表单提交事件
    document.getElementById('json-file-group-search-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-file-group-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入查询条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            searchFileGroupsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function searchFileGroupsByConditions(conditions) {
    // 构造查询选项
    const options = {
        limit: 100,
        offset: 0
    };
    
    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/file-groups/search/by-conditions-with-options?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                closeModal();
                renderFileGroupTable(data.data || []);
            } else {
                showMessage('文件组关联查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('文件组关联查询失败: ' + error.message, 'error');
        });
}

// 打开批量删除组标签对话框
function openBatchDeleteGroupTagDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>批量删除组标签关联</h2>
        <form id="batch-delete-group-tag-form">
            <div class="form-group">
                <label for="batch-delete-group-tag-conditions">删除条件 (JSON格式):</label>
                <textarea id="batch-delete-group-tag-conditions" rows="5" placeholder='[{"GroupId": 1, "TagId": 2}]'></textarea>
            </div>
            <button type="submit">删除</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-group-tag-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('batch-delete-group-tag-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入删除条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            deleteGroupTagsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function deleteGroupTagsByConditions(conditions) {
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
        if (data.success) {
            showMessage('组标签关联批量删除成功', 'success');
            closeModal();
            // 重新加载组标签列表
            listGroupTagsByFilter();
        } else {
            showMessage('组标签关联批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组标签关联批量删除失败: ' + error.message, 'error');
    });
}

// 打开复杂查询组标签对话框
function openComplexSearchGroupTagDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询组标签关联</h2>
        <div class="tabs">
            <button class="tab-button active" onclick="switchComplexSearchTab('visual')">可视化查询</button>
            <button class="tab-button" onclick="switchComplexSearchTab('json')">JSON查询</button>
        </div>
        <div id="visual-search" class="tab-content active">
            <form id="visual-group-tag-search-form">
                <div class="form-group">
                    <label for="visual-search-field">查询字段:</label>
                    <select id="visual-search-field">
                        <option value="GroupId">组ID</option>
                        <option value="TagId">标签ID</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-operator">操作符:</label>
                    <select id="visual-search-operator">
                        <option value="equal">等于</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-value">值:</label>
                    <input type="text" id="visual-search-value">
                </div>
                <div class="form-group">
                    <button type="button" onclick="addVisualSearchCondition()">添加条件</button>
                </div>
                <div class="form-group">
                    <label>已添加的条件:</label>
                    <div id="visual-search-conditions"></div>
                </div>
                <button type="button" onclick="performVisualSearch()">查询</button>
            </form>
        </div>
        <div id="json-search" class="tab-content" style="display: none;">
            <form id="json-group-tag-search-form">
                <div class="form-group">
                    <label for="complex-search-group-tag-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-group-tag-conditions" rows="5" placeholder='[{"GroupId": 1, "TagId": 2}]'></textarea>
                </div>
                <button type="submit">查询</button>
            </form>
        </div>
        <button type="button" onclick="closeModal()">取消</button>
    `;
    
    // 绑定表单提交事件
    document.getElementById('json-group-tag-search-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-group-tag-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入查询条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            searchGroupTagsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function searchGroupTagsByConditions(conditions) {
    // 构造查询选项
    const options = {
        limit: 100,
        offset: 0
    };
    
    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/group-tags/search/by-conditions-with-options?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                closeModal();
                renderGroupTagTable(data.data || []);
            } else {
                showMessage('组标签关联查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组标签关联查询失败: ' + error.message, 'error');
        });
}

// 打开批量删除组关系对话框
function openBatchDeleteGroupRelationDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>批量删除组关系</h2>
        <form id="batch-delete-group-relation-form">
            <div class="form-group">
                <label for="batch-delete-group-relation-conditions">删除条件 (JSON格式):</label>
                <textarea id="batch-delete-group-relation-conditions" rows="5" placeholder='[{"FirstGroupId": 1, "SecondGroupId": 2, "RelationType": "parent"}]'></textarea>
            </div>
            <button type="submit">删除</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-group-relation-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('batch-delete-group-relation-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入删除条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            deleteGroupRelationsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function deleteGroupRelationsByConditions(conditions) {
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
        if (data.success) {
            showMessage('组关系批量删除成功', 'success');
            closeModal();
            // 重新加载组关系列表
            listGroupRelationsByFilter();
        } else {
            showMessage('组关系批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('组关系批量删除失败: ' + error.message, 'error');
    });
}

// 打开复杂查询组关系对话框
function openComplexSearchGroupRelationDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询组关系</h2>
        <div class="tabs">
            <button class="tab-button active" onclick="switchComplexSearchTab('visual')">可视化查询</button>
            <button class="tab-button" onclick="switchComplexSearchTab('json')">JSON查询</button>
        </div>
        <div id="visual-search" class="tab-content active">
            <form id="visual-group-relation-search-form">
                <div class="form-group">
                    <label for="visual-search-field">查询字段:</label>
                    <select id="visual-search-field">
                        <option value="FirstGroupId">第一组ID</option>
                        <option value="SecondGroupId">第二组ID</option>
                        <option value="RelationType">关系类型</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-operator">操作符:</label>
                    <select id="visual-search-operator">
                        <option value="equal">等于</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="visual-search-value">值:</label>
                    <input type="text" id="visual-search-value">
                </div>
                <div class="form-group">
                    <button type="button" onclick="addVisualSearchCondition()">添加条件</button>
                </div>
                <div class="form-group">
                    <label>已添加的条件:</label>
                    <div id="visual-search-conditions"></div>
                </div>
                <button type="button" onclick="performVisualSearch()">查询</button>
            </form>
        </div>
        <div id="json-search" class="tab-content" style="display: none;">
            <form id="json-group-relation-search-form">
                <div class="form-group">
                    <label for="complex-search-group-relation-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-group-relation-conditions" rows="5" placeholder='[{"FirstGroupId": 1, "SecondGroupId": 2, "RelationType": "parent"}]'></textarea>
                </div>
                <button type="submit">查询</button>
            </form>
        </div>
        <button type="button" onclick="closeModal()">取消</button>
    `;
    
    // 绑定表单提交事件
    document.getElementById('json-group-relation-search-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-group-relation-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入查询条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            searchGroupRelationsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function searchGroupRelationsByConditions(conditions) {
    // 构造查询选项
    const options = {
        limit: 100,
        offset: 0
    };
    
    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/group-relations/search/by-conditions-with-options?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                closeModal();
                renderGroupRelationTable(data.data || []);
            } else {
                showMessage('组关系查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组关系查询失败: ' + error.message, 'error');
        });
}