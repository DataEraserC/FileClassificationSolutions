// 组关系管理相关函数

// 添加分页相关变量
let currentGroupRelationPage = 1;
let groupRelationPageSize = 10;
let totalGroupRelationPages = 1;
let currentGroupRelationConditions = null;
let currentGroupRelationQueryType = null; // 'filter' or 'conditions'

function listGroupRelationsByFilter() {
    const firstId = getInputValue('group-relation-first-id');
    const secondId = getInputValue('group-relation-second-id');
    const relationType = getInputValue('group-relation-type');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (firstId) params.append('first_group_id', firstId);
    if (secondId) params.append('second_group_id', secondId);
    if (relationType) params.append('relation_type', relationType);
    
    // 构造分页参数
    const options = {
        page: currentGroupRelationPage,
        page_size: groupRelationPageSize
    };
    
    // 保存当前条件
    currentGroupRelationConditions = {};
    if (firstId) currentGroupRelationConditions.first_group_id = firstId;
    if (secondId) currentGroupRelationConditions.second_group_id = secondId;
    if (relationType) currentGroupRelationConditions.relation_type = relationType;
    
    // 标记使用filter查询
    currentGroupRelationQueryType = 'filter';
    
    // 构造查询参数
    const searchParams = new URLSearchParams({
        filter: JSON.stringify(currentGroupRelationConditions),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/group-relations/search/by-filter-with-pagination?${searchParams.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success && data.data) {
                renderGroupRelationTable(data.data.data || []);
                // 更新分页信息
                totalGroupRelationPages = data.data.total_pages || 1;
                renderGroupRelationPagination(data.data);
            } else {
                showMessage('组关系查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组关系查询失败: ' + error.message, 'error');
        });
}

// 渲染组关系表格
function renderGroupRelationTable(groupRelations) {
    const tbody = document.querySelector('#group-relations-table tbody');
    if (!tbody) return;
    
    if (!groupRelations || groupRelations.length === 0) {
        tbody.innerHTML = '<tr><td colspan="5">暂无数据</td></tr>';
        return;
    }
    
    tbody.innerHTML = groupRelations.map(relation => `
        <tr>
            <td><input type="checkbox" class="group-relation-checkbox" data-first-id="${relation.first_group_id}" data-second-id="${relation.second_group_id}" data-relation-type="${relation.relation_type}"></td>
            <td>${relation.first_group_id}</td>
            <td>${relation.second_group_id}</td>
            <td>${relation.relation_type}</td>
            <td>
                <div class="table-actions">
                    <button class="action-button delete" onclick="deleteGroupRelation(${relation.first_group_id}, ${relation.second_group_id}, ${relation.relation_type})">删除</button>
                </div>
            </td>
        </tr>
    `).join('');
}

// 渲染组关系分页控件
function renderGroupRelationPagination(data) {
    const paginationContainer = document.getElementById('group-relations-pagination');
    if (!paginationContainer) return;
    
    const currentPage = data.page || currentGroupRelationPage;
    const totalPages = data.total_pages || totalGroupRelationPages;
    const totalRecords = data.total || 0;
    const pageSize = data.page_size || groupRelationPageSize;
    
    let paginationHTML = `
        <div class="pagination-container">
            <div class="pagination-info">
                共 ${totalRecords} 条记录，第 ${currentPage} 页/共 ${totalPages} 页
            </div>
            <div class="pagination-controls">
                <button onclick="changeGroupRelationPage(1)" ${currentPage <= 1 ? 'disabled' : ''}>首页</button>
                <button onclick="changeGroupRelationPage(${currentPage - 1})" ${currentPage <= 1 ? 'disabled' : ''}>上一页</button>
                <span class="page-numbers">
    `;
    
    // 显示页码
    let startPage = Math.max(1, currentPage - 2);
    let endPage = Math.min(totalPages, currentPage + 2);
    
    if (startPage > 1) {
        paginationHTML += `<button onclick="changeGroupRelationPage(1)">1</button>`;
        if (startPage > 2) paginationHTML += `<span>...</span>`;
    }
    
    for (let i = startPage; i <= endPage; i++) {
        if (i === currentPage) {
            paginationHTML += `<button class="active">${i}</button>`;
        } else {
            paginationHTML += `<button onclick="changeGroupRelationPage(${i})">${i}</button>`;
        }
    }
    
    if (endPage < totalPages) {
        if (endPage < totalPages - 1) paginationHTML += `<span>...</span>`;
        paginationHTML += `<button onclick="changeGroupRelationPage(${totalPages})">${totalPages}</button>`;
    }
    
    paginationHTML += `
                </span>
                <button onclick="changeGroupRelationPage(${currentPage + 1})" ${currentPage >= totalPages ? 'disabled' : ''}>下一页</button>
                <button onclick="changeGroupRelationPage(${totalPages})" ${currentPage >= totalPages ? 'disabled' : ''}>末页</button>
            </div>
            <div class="pagination-size">
                每页显示: 
                <select onchange="changeGroupRelationPageSize(this.value)">
                    <option value="10" ${pageSize === 10 ? 'selected' : ''}>10</option>
                    <option value="20" ${pageSize === 20 ? 'selected' : ''}>20</option>
                    <option value="50" ${pageSize === 50 ? 'selected' : ''}>50</option>
                    <option value="100" ${pageSize === 100 ? 'selected' : ''}>100</option>
                </select>
            </div>
        </div>
    `;
    
    paginationContainer.innerHTML = paginationHTML;
}

// 改变页码
function changeGroupRelationPage(page) {
    if (page < 1 || page > totalGroupRelationPages) return;
    currentGroupRelationPage = page;
    // 根据查询类型选择接口
    if (currentGroupRelationQueryType === 'conditions') {
        searchGroupRelationsByConditions(currentGroupRelationConditions);
    } else {
        searchGroupRelationsByFilter();
    }
}

// 使用filter方式搜索组关系（用于分页）
function searchGroupRelationsByFilter() {
    // 构造查询参数
    let params = new URLSearchParams();
    if (currentGroupRelationConditions && currentGroupRelationConditions.first_group_id) params.append('first_group_id', currentGroupRelationConditions.first_group_id);
    if (currentGroupRelationConditions && currentGroupRelationConditions.second_group_id) params.append('second_group_id', currentGroupRelationConditions.second_group_id);
    if (currentGroupRelationConditions && currentGroupRelationConditions.relation_type) params.append('relation_type', currentGroupRelationConditions.relation_type);
    
    // 构造分页参数
    const options = {
        page: currentGroupRelationPage,
        page_size: groupRelationPageSize
    };
    
    // 构造查询参数
    const searchParams = new URLSearchParams({
        filter: JSON.stringify(currentGroupRelationConditions || {}),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/group-relations/search/by-filter-with-pagination?${searchParams.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success && data.data) {
                renderGroupRelationTable(data.data.data || []);
                // 更新分页信息
                totalGroupRelationPages = data.data.total_pages || 1;
                renderGroupRelationPagination(data.data);
            } else {
                showMessage('组关系查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组关系查询失败: ' + error.message, 'error');
        });
}

// 在页面加载完成后绑定分页控件事件
document.addEventListener('DOMContentLoaded', function() {
    // 使用事件委托处理分页按钮点击
    document.addEventListener('click', function(e) {
        // 处理组关系分页按钮点击
        if (e.target.closest('#group-relations-pagination') && e.target.tagName === 'BUTTON') {
            const button = e.target;
            if (button.hasAttribute('onclick')) {
                // 防止重复绑定事件
                return;
            }
            
            const pageMatch = button.textContent.match(/(\d+)/);
            if (button.textContent === '首页') {
                changeGroupRelationPage(1);
            } else if (button.textContent === '上一页') {
                changeGroupRelationPage(currentGroupRelationPage - 1);
            } else if (button.textContent === '下一页') {
                changeGroupRelationPage(currentGroupRelationPage + 1);
            } else if (button.textContent === '末页') {
                changeGroupRelationPage(totalGroupRelationPages);
            } else if (pageMatch) {
                changeGroupRelationPage(parseInt(pageMatch[1]));
            }
        }
    });
});

// 改变每页大小
function changeGroupRelationPageSize(size) {
    groupRelationPageSize = parseInt(size);
    currentGroupRelationPage = 1; // 重置到第一页
    if (currentGroupRelationConditions) {
        searchGroupRelationsByConditions(currentGroupRelationConditions);
    } else {
        searchGroupRelationsByFilter();
    }
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
        relation_type: parseInt(relationType)
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
    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', `确定要删除组关系 [第一个组ID: ${firstId}, 第二个组ID: ${secondId}, 关系类型: ${relationType}] 吗？`, function(result) {
        if (result) {
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
    });
}

// 批量删除选中的组关系
function deleteSelectedGroupRelations() {
    const selectedCheckboxes = document.querySelectorAll('.group-relation-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个组关系进行删除', 'warning');
        return;
    }

    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', `确定要删除这 ${selectedCheckboxes.length} 个组关系吗？`, function(result) {
        if (result) {
            const groupRelations = Array.from(selectedCheckboxes).map(cb => {
                return {
                    first_group_id: parseInt(cb.getAttribute('data-first-id')),
                    second_group_id: parseInt(cb.getAttribute('data-second-id')),
                    relation_type: parseInt(cb.getAttribute('data-relation-type'))
                };
            });

            // 使用新的delete by dtos接口
            deleteGroupRelationsByDtos(groupRelations);
        }
    });
}

// 打开创建组关系对话框
function openCreateGroupRelationDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>新增组关系</h2>
        <form id="create-group-relation-form">
            <div class="form-group">
                <label for="create-group-relation-first-id">第一个组ID:</label>
                <input type="number" id="create-group-relation-first-id" required>
            </div>
            <div class="form-group">
                <label for="create-group-relation-second-id">第二个组ID:</label>
                <input type="number" id="create-group-relation-second-id" required>
            </div>
            <div class="form-group">
                <label for="create-group-relation-type">关系类型:</label>
                <input type="number" id="create-group-relation-type" value="1" required>
            </div>
            <button type="submit" class="btn-primary">创建</button>
            <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-group-relation-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createGroupRelation();
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 批量删除组关系（根据DTO列表）
function deleteGroupRelationsByDtos(dtos) {
    const url = `${BASE_URL}/api/group-relations/delete/by-dtos`;
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
                <div id="visual-search-conditions"></div>
                <button type="submit" class="btn-primary">查询</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        </div>
        <div id="json-search" class="tab-content">
            <form id="json-group-relation-search-form">
                <div class="form-group">
                    <label for="complex-search-group-relation-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-group-relation-conditions" rows="5" placeholder='[{"FirstGroupId": 1}]'></textarea>
                </div>
                <button type="submit" class="btn-primary">查询</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        </div>
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
        page: currentGroupRelationPage,
        page_size: groupRelationPageSize
    };
    
    // 保存当前条件
    currentGroupRelationConditions = conditions;
    
    // 标记使用conditions查询
    currentGroupRelationQueryType = 'conditions';
    
    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/group-relations/search/by-conditions-with-pagination?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                closeModal();
                renderGroupRelationTable(data.data.data || []);
                // 更新分页信息
                totalGroupRelationPages = data.data.total_pages || 1;
                renderGroupRelationPagination(data.data);
            } else {
                showMessage('组关系查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组关系查询失败: ' + error.message, 'error');
        });
}

// 重置组关系过滤器
function resetGroupRelationFilter() {
    document.getElementById('group-relation-first-id').value = '';
    document.getElementById('group-relation-second-id').value = '';
    document.getElementById('group-relation-type').value = '';
    // 重置分页参数
    currentGroupRelationPage = 1;
    listGroupRelationsByFilter(); // 重置后重新搜索
}

// 切换全选组关系
function toggleAllGroupRelations(source) {
    const checkboxes = document.querySelectorAll('.group-relation-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.checked = source.checked;
    });
}