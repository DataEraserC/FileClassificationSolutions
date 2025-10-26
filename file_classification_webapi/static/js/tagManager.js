// 标签管理相关函数

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