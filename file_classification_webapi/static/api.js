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
            alert('查询文件失败: ' + error.message);
        });
}

function renderFileTable(files) {
    const tableBody = document.querySelector('#files-table tbody');
    tableBody.innerHTML = '';
    
    files.forEach(file => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td>${file.id}</td>
            <td>${file.type_}</td>
            <td>${file.path}</td>
            <td>${file.reference_count}</td>
            <td>${file.group_id}</td>
            <td>
                <button class="action-button edit" onclick="editFile(${file.id})">修改</button>
                <button class="action-button delete" onclick="deleteFile(${file.id})">删除</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

function getFileById() {
    const fileId = getInputValue('file-id');
    if (!fileId) {
        alert('请输入文件ID');
        return;
    }
    
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                renderFileTable([data.data]);
            } else {
                alert('获取文件失败: ' + data.message);
            }
        })
        .catch(error => {
            console.error('Error:', error);
            alert('获取文件失败: ' + error.message);
        });
}

function createFile() {
    const fileType = getInputValue('create-file-type');
    const filePath = getInputValue('create-file-path');
    const groupId = getInputValue('create-file-group-id');
    
    if (!fileType || !filePath || !groupId) {
        alert('请填写完整的文件信息');
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
            alert('文件创建成功');
            closeModal();
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            alert('文件创建失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('文件创建失败: ' + error.message);
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
            alert('文件删除成功');
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            alert('文件删除失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('文件删除失败: ' + error.message);
    });
}

function editFile(fileId) {
    // 这里可以实现编辑功能
    alert('编辑功能待实现，文件ID: ' + fileId);
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
            alert('查询组失败: ' + error.message);
        });
}

function renderGroupTable(groups) {
    const tableBody = document.querySelector('#groups-table tbody');
    tableBody.innerHTML = '';
    
    groups.forEach(group => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td>${group.id}</td>
            <td>${group.name}</td>
            <td>${group.description}</td>
            <td>${group.reference_count}</td>
            <td>${group.parent_group_id}</td>
            <td>
                <button class="action-button edit" onclick="editGroup(${group.id})">修改</button>
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
        alert('请输入组名');
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
            alert('组创建成功');
            closeModal();
            // 重新加载组列表
            listGroupsByFilter();
        } else {
            alert('组创建失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('组创建失败: ' + error.message);
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
            alert('组删除成功');
            // 重新加载组列表
            listGroupsByFilter();
        } else {
            alert('组删除失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('组删除失败: ' + error.message);
    });
}

function editGroup(groupId) {
    // 这里可以实现编辑功能
    alert('编辑功能待实现，组ID: ' + groupId);
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
            alert('查询标签失败: ' + error.message);
        });
}

function renderTagTable(tags) {
    const tableBody = document.querySelector('#tags-table tbody');
    tableBody.innerHTML = '';
    
    tags.forEach(tag => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td>${tag.id}</td>
            <td>${tag.name}</td>
            <td>${tag.description}</td>
            <td>${tag.reference_count}</td>
            <td>
                <button class="action-button edit" onclick="editTag(${tag.id})">修改</button>
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
        alert('请输入标签名');
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
            alert('标签创建成功');
            closeModal();
            // 重新加载标签列表
            listTagsByFilter();
        } else {
            alert('标签创建失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('标签创建失败: ' + error.message);
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
            alert('标签删除成功');
            // 重新加载标签列表
            listTagsByFilter();
        } else {
            alert('标签删除失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('标签删除失败: ' + error.message);
    });
}

function editTag(tagId) {
    // 这里可以实现编辑功能
    alert('编辑功能待实现，标签ID: ' + tagId);
}

// ==================== 文件组关联相关函数 ====================

// ==================== 组标签关联相关函数 ====================

// ==================== 组关系管理相关函数 ====================

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
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-file-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createFile();
    });
    
    document.getElementById('modal').style.display = 'block';
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
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-group-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createGroup();
    });
    
    document.getElementById('modal').style.display = 'block';
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
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-tag-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createTag();
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
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-file-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('batch-delete-file-conditions').value;
        if (!conditionsJson) {
            alert('请输入删除条件');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            deleteFilesByConditions(conditions);
        } catch (e) {
            alert('JSON格式错误: ' + e.message);
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
            alert('文件批量删除成功');
            closeModal();
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            alert('文件批量删除失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('文件批量删除失败: ' + error.message);
    });
}

// 打开复杂查询文件对话框
function openComplexSearchFileDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询文件</h2>
        <form id="complex-search-file-form">
            <div class="form-group">
                <label for="complex-search-file-conditions">查询条件 (JSON格式):</label>
                <textarea id="complex-search-file-conditions" rows="5" placeholder='[{"Id": 1}, {"Type": "txt"}]'></textarea>
            </div>
            <button type="submit">查询</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('complex-search-file-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-file-conditions').value;
        if (!conditionsJson) {
            alert('请输入查询条件');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            searchFilesByConditions(conditions);
        } catch (e) {
            alert('JSON格式错误: ' + e.message);
        }
    });
    
    document.getElementById('modal').style.display = 'block';
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
            alert('文件查询失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('文件查询失败: ' + error.message);
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
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-group-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('batch-delete-group-conditions').value;
        if (!conditionsJson) {
            alert('请输入删除条件');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            deleteGroupsByConditions(conditions);
        } catch (e) {
            alert('JSON格式错误: ' + e.message);
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
            alert('组批量删除成功');
            closeModal();
            // 重新加载组列表
            listGroupsByFilter();
        } else {
            alert('组批量删除失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('组批量删除失败: ' + error.message);
    });
}

// 打开复杂查询组对话框
function openComplexSearchGroupDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询组</h2>
        <form id="complex-search-group-form">
            <div class="form-group">
                <label for="complex-search-group-conditions">查询条件 (JSON格式):</label>
                <textarea id="complex-search-group-conditions" rows="5" placeholder='[{"Id": 1}, {"Name": "example"}]'></textarea>
            </div>
            <button type="submit">查询</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('complex-search-group-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-group-conditions').value;
        if (!conditionsJson) {
            alert('请输入查询条件');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            searchGroupsByConditions(conditions);
        } catch (e) {
            alert('JSON格式错误: ' + e.message);
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
            alert('组查询失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('组查询失败: ' + error.message);
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
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-tag-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('batch-delete-tag-conditions').value;
        if (!conditionsJson) {
            alert('请输入删除条件');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            deleteTagsByConditions(conditions);
        } catch (e) {
            alert('JSON格式错误: ' + e.message);
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
            alert('标签批量删除成功');
            closeModal();
            // 重新加载标签列表
            listTagsByFilter();
        } else {
            alert('标签批量删除失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('标签批量删除失败: ' + error.message);
    });
}

// 打开复杂查询标签对话框
function openComplexSearchTagDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询标签</h2>
        <form id="complex-search-tag-form">
            <div class="form-group">
                <label for="complex-search-tag-conditions">查询条件 (JSON格式):</label>
                <textarea id="complex-search-tag-conditions" rows="5" placeholder='[{"Id": 1}, {"Name": "example"}]'></textarea>
            </div>
            <button type="submit">查询</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('complex-search-tag-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-tag-conditions').value;
        if (!conditionsJson) {
            alert('请输入查询条件');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            searchTagsByConditions(conditions);
        } catch (e) {
            alert('JSON格式错误: ' + e.message);
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
            alert('标签查询失败: ' + data.message);
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('标签查询失败: ' + error.message);
    });
}