// 组管理相关函数

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
                <button class="action-button view-tree" onclick="showGroupTree(${group.id})">查看树</button>
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

// 显示组的树状结构
function showGroupTree(groupId) {
    const url = `${BASE_URL}/api/groups/${groupId}/tree`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                renderGroupTree(data.data);
                document.getElementById('group-tree-modal').style.display = 'block';
            } else {
                showMessage('获取组树失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取组树失败: ' + error.message, 'error');
        });
}

// 渲染组树状结构
function renderGroupTree(treeNode, container = null, level = 0) {
    if (!container) {
        container = document.getElementById('group-tree-container');
        container.innerHTML = '';
    }
    
    const nodeElement = document.createElement('div');
    nodeElement.className = 'tree-node';
    nodeElement.style.marginLeft = (level * 20) + 'px';
    
    nodeElement.innerHTML = `
        <div class="tree-node-content">
            <span class="tree-node-name">${treeNode.group.name}</span>
            <span class="tree-node-info">(ID: ${treeNode.group.id})</span>
        </div>
    `;
    
    container.appendChild(nodeElement);
    
    // 递归渲染子节点
    if (treeNode.children && treeNode.children.length > 0) {
        treeNode.children.forEach(child => {
            renderGroupTree(child, container, level + 1);
        });
    }
}

// 关闭组树模态框
function closeGroupTreeModal() {
    document.getElementById('group-tree-modal').style.display = 'none';
}