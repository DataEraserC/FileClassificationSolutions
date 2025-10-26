// 组关系管理相关函数

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