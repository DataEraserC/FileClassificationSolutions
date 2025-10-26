// 文件组关联相关函数

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