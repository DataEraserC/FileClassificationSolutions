// 组标签关联相关函数

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
    
    // 构造删除DTO列表
    const dtos = Array.from(selectedCheckboxes).map(cb => {
        return {
            group_id: parseInt(cb.getAttribute('data-group-id')),
            tag_id: parseInt(cb.getAttribute('data-tag-id'))
        };
    });
    
    // 使用新的delete by dtos接口
    deleteGroupTagsByDtos(dtos);
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

// 打开批量删除组标签对话框
function openBatchDeleteGroupTagDialog() {
    const modalBody = document.getElementById('modal-body');
    
    // 获取当前选中的组标签关联
    const selectedGroupTagCheckboxes = document.querySelectorAll('.group-tag-checkbox:checked');
    const selectedGroupTagDtos = Array.from(selectedGroupTagCheckboxes).map(cb => ({
        group_id: parseInt(cb.getAttribute('data-group-id')),
        tag_id: parseInt(cb.getAttribute('data-tag-id'))
    }));
    
    let formContent;
    if (selectedGroupTagDtos.length > 0) {
        formContent = `
            <h2>批量删除组标签关联</h2>
            <p>已选择 ${selectedGroupTagDtos.length} 个组标签关联</p>
            <form id="batch-delete-group-tag-form">
                <input type="hidden" id="selected-group-tag-dtos" value='${JSON.stringify(selectedGroupTagDtos)}'>
                <button type="submit">删除选中组标签关联</button>
                <button type="button" onclick="closeModal()">取消</button>
            </form>
        `;
    } else {
        formContent = `
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
    }
    
    modalBody.innerHTML = formContent;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-group-tag-form').addEventListener('submit', function(e) {
        e.preventDefault();
        
        // 如果有选中的组标签关联，使用delete by dtos
        const selectedDtosInput = document.getElementById('selected-group-tag-dtos');
        if (selectedDtosInput) {
            const dtos = JSON.parse(selectedDtosInput.value);
            deleteGroupTagsByDtos(dtos);
            return;
        }
        
        // 否则使用条件删除（向后兼容）
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

// 批量删除组标签关联（根据DTO列表）
function deleteGroupTagsByDtos(dtos) {
    const url = `${BASE_URL}/api/group-tags/delete/by-dtos`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(dtos)
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