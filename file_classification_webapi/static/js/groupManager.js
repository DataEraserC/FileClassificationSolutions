// 组管理相关函数

// 添加分页相关变量
let currentGroupPage = 1;
let groupPageSize = 10;
let totalGroupPages = 1;
let currentGroupConditions = null;
let currentGroupQueryType = null; // 'filter' or 'conditions'

function listGroupsByFilter() {
    const groupId = getInputValue('group-id');
    const groupName = getInputValue('group-name');

    // 构造查询参数
    let params = new URLSearchParams();
    if (groupId) params.append('id', groupId);
    if (groupName) params.append('name', groupName);

    // 构造分页参数
    const options = {
        page: currentGroupPage,
        page_size: groupPageSize
    };

    // 保存当前条件
    currentGroupConditions = {};
    if (groupId) currentGroupConditions.id = parseInt(groupId);
    if (groupName) currentGroupConditions.name = groupName;

    // 标记使用filter查询
    currentGroupQueryType = 'filter';

    // 构造查询参数
    const searchParams = new URLSearchParams({
        filter: JSON.stringify(currentGroupConditions),
        options: JSON.stringify(options)
    });

    const url = `${BASE_URL}/api/groups/search/by-filter-with-pagination?${searchParams.toString()}`;

    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success && result.data) {
                renderGroupTable(result.data.data || []);
                // 更新分页信息
                totalGroupPages = result.data.total_pages || 1;
                renderGroupPagination(result.data);
            } else {
                renderGroupTable([]);
                renderGroupPagination({page: 1, total_pages: 1, total: 0});
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('查询组失败: ' + error.message, 'error');
            renderGroupTable([]);
            renderGroupPagination({page: 1, total_pages: 1, total: 0});
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
            <td>${group.description === null ? '无' : (group.description || '')}</td>
            <td>
                <span class="info-icon" data-group='${JSON.stringify(group).replace(/"/g, '&quot;')}' onmouseover="showGroupTooltip(event)" onmouseout="hideGroupTooltip()" title="悬停查看详细信息">ℹ️</span>
                <button class="action-button info" onclick="showGroupInfo(${group.id})" title="查看详细信息">详情</button>
                <button class="action-button view-tree" onclick="showGroupTree(${group.id})">查看树</button>
                <button class="action-button edit" onclick="openEditGroupDialog(${group.id})">修改</button>
                <button class="action-button delete" onclick="deleteGroup(${group.id})">删除</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

// 渲染分页控件
function renderGroupPagination(paginationData) {
    const paginationContainer = document.getElementById('groups-pagination');
    if (!paginationContainer) return;

    const currentPage = paginationData.page || 1;
    const totalPages = paginationData.total_pages || 1;
    const totalRecords = paginationData.total || 0;

    let paginationHTML = `
        <div class="pagination-container">
            <div class="pagination-info">
                共 ${totalRecords} 条记录，第 ${currentPage} 页/共 ${totalPages} 页
            </div>
            <div class="pagination-controls">
                <button onclick="changeGroupPage(1)" ${currentPage <= 1 ? 'disabled' : ''}>首页</button>
                <button onclick="changeGroupPage(${currentPage - 1})" ${currentPage <= 1 ? 'disabled' : ''}>上一页</button>
                <span class="page-numbers">
    `;

    // 显示页码
    let startPage = Math.max(1, currentPage - 2);
    let endPage = Math.min(totalPages, currentPage + 2);

    if (startPage > 1) {
        paginationHTML += `<button onclick="changeGroupPage(1)">1</button>`;
        if (startPage > 2) paginationHTML += `<span>...</span>`;
    }

    for (let i = startPage; i <= endPage; i++) {
        if (i === currentPage) {
            paginationHTML += `<button class="active">${i}</button>`;
        } else {
            paginationHTML += `<button onclick="changeGroupPage(${i})">${i}</button>`;
        }
    }

    if (endPage < totalPages) {
        if (endPage < totalPages - 1) paginationHTML += `<span>...</span>`;
        paginationHTML += `<button onclick="changeGroupPage(${totalPages})">${totalPages}</button>`;
    }

    paginationHTML += `
                </span>
                <button onclick="changeGroupPage(${currentPage + 1})" ${currentPage >= totalPages ? 'disabled' : ''}>下一页</button>
                <button onclick="changeGroupPage(${totalPages})" ${currentPage >= totalPages ? 'disabled' : ''}>末页</button>
            </div>
            <div class="pagination-size">
                每页显示: 
                <select onchange="changeGroupPageSize(this.value)">
                    <option value="10" ${groupPageSize === 10 ? 'selected' : ''}>10</option>
                    <option value="20" ${groupPageSize === 20 ? 'selected' : ''}>20</option>
                    <option value="50" ${groupPageSize === 50 ? 'selected' : ''}>50</option>
                    <option value="100" ${groupPageSize === 100 ? 'selected' : ''}>100</option>
                </select>
            </div>
        </div>
    `;

    paginationContainer.innerHTML = paginationHTML;
}

// 改变页码
function changeGroupPage(page) {
    if (page < 1 || page > totalGroupPages) return;
    currentGroupPage = page;
    // 根据查询类型选择接口
    if (currentGroupQueryType === 'conditions') {
        searchGroupsByConditions(currentGroupConditions);
    } else {
        listGroupsByFilter();
    }
}

// 改变每页大小
function changeGroupPageSize(size) {
    groupPageSize = parseInt(size);
    currentGroupPage = 1; // 重置到第一页
    // 根据查询类型选择接口
    if (currentGroupQueryType === 'conditions') {
        searchGroupsByConditions(currentGroupConditions);
    } else {
        listGroupsByFilter();
    }
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('组创建成功', 'success');
                closeModal();
                // 重新加载组列表
                currentGroupPage = 1;
                listGroupsByFilter();
            } else {
                showMessage('组创建失败: ' + (result.data?.message || '未知错误'), 'error');
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('组更新成功', 'success');
                closeModal();
                // 重新加载组列表
                listGroupsByFilter();
            } else {
                showMessage('组更新失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组更新失败: ' + error.message, 'error');
        });
}

function deleteGroup(groupId) {
    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', '确定要删除该组吗？', function (result) {
        if (result) {
            const url = `${BASE_URL}/api/groups/${groupId}`;
            fetch(url, {
                method: 'DELETE'
            })
                .then(response => response.json())
                .then(data => {
                    const result = handleApiResponse(data);
                    if (result.success) {
                        showMessage('组删除成功', 'success');
                        // 重新加载组列表
                        listGroupsByFilter();
                    } else {
                        showMessage('组删除失败: ' + (result.data?.message || '未知错误'), 'error');
                    }
                })
                .catch(error => {
                    console.error('Error:', error);
                    showMessage('组删除失败: ' + error.message, 'error');
                });
        }
    });
}

// 批量删除选中的组
function deleteSelectedGroups() {
    const selectedCheckboxes = document.querySelectorAll('.group-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个组进行删除', 'warning');
        return;
    }

    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', `确定要删除这 ${selectedCheckboxes.length} 个组吗？`, function (result) {
        if (result) {
            const ids = Array.from(selectedCheckboxes).map(cb => parseInt(cb.getAttribute('data-id')));

            // 使用新的delete by ids接口
            deleteGroupsByIds(ids);
        }
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
            <button type="submit" class="btn-primary">创建</button>
            <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
        </form>
    `;

    // 绑定表单提交事件
    document.getElementById('create-group-form').addEventListener('submit', function (e) {
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
            const result = handleApiResponse(data);
            if (result.success) {
                const group = result.data;
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
                        <button type="submit" class="btn-primary">更新</button>
                        <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
                    </form>
                `;

                // 绑定表单提交事件
                document.getElementById('edit-group-form').addEventListener('submit', function (e) {
                    e.preventDefault();
                    updateGroup();
                });

                document.getElementById('modal').style.display = 'block';
            } else {
                showMessage('获取组信息失败: ' + (result.data?.message || '未知错误'), 'error');
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

    // 获取当前选中的组ID
    const selectedGroupCheckboxes = document.querySelectorAll('.group-checkbox:checked');
    const selectedGroupIds = Array.from(selectedGroupCheckboxes).map(cb => parseInt(cb.value)).map(id => parseInt(id));

    let formContent;
    if (selectedGroupIds.length > 0) {
        formContent = `
            <h2>批量删除组</h2>
            <p>已选择 ${selectedGroupIds.length} 个组</p>
            <form id="batch-delete-group-form">
                <input type="hidden" id="selected-group-ids" value='${JSON.stringify(selectedGroupIds)}'>
                <button type="submit">删除选中组</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        `;
    } else {
        formContent = `
            <h2>批量删除组</h2>
            <form id="batch-delete-group-form">
                <div class="form-group">
                    <label for="batch-delete-group-conditions">删除条件 (JSON格式):</label>
                    <textarea id="batch-delete-group-conditions" rows="5" placeholder='[{"Id": 1}, {"Name": "example"}]'></textarea>
                </div>
                <button type="submit">删除</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        `;
    }

    modalBody.innerHTML = formContent;

    // 绑定表单提交事件
    document.getElementById('batch-delete-group-form').addEventListener('submit', function (e) {
        e.preventDefault();

        // 如果有选中的组ID，使用delete by ids
        const selectedIdsInput = document.getElementById('selected-group-ids');
        if (selectedIdsInput) {
            const groupIds = JSON.parse(selectedIdsInput.value);
            // 确保数值字段是数字类型
            const fixedGroupIds = groupIds.map(id => parseInt(id));
            deleteGroupsByIds(fixedGroupIds);
            return;
        }

        // 否则使用条件删除（向后兼容）
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

// 批量删除组（根据ID列表）
function deleteGroupsByIds(groupIds) {
    const url = `${BASE_URL}/api/groups/delete/by-ids`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(groupIds)
    })
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('组批量删除成功', 'success');
                closeModal();
                // 重新加载组列表
                listGroupsByFilter();
            } else {
                showMessage('组批量删除失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组批量删除失败: ' + error.message, 'error');
        });
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('组批量删除成功', 'success');
                closeModal();
                // 重新加载组列表
                listGroupsByFilter();
            } else {
                showMessage('组批量删除失败: ' + (result.data?.message || '未知错误'), 'error');
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
        <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
    `;

    // 绑定表单提交事件
    document.getElementById('json-group-search-form').addEventListener('submit', function (e) {
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
    // 构造查询选项
    const options = {
        page: currentGroupPage,
        page_size: groupPageSize
    };

    // 保存当前条件
    currentGroupConditions = conditions;

    // 标记使用conditions查询
    currentGroupQueryType = 'conditions';

    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });

    const url = `${BASE_URL}/api/groups/search/by-conditions-with-pagination?${params.toString()}`;

    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                closeModal();
                renderGroupTable(result.data.data || []);
                // 更新分页信息
                totalGroupPages = result.data.total_pages || 1;
                renderGroupPagination(result.data);
            } else {
                showMessage('组查询失败: ' + (result.data?.message || '未知错误'), 'error');
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
            const result = handleApiResponse(data);
            if (result.success) {
                renderGroupTree(result.data);
                document.getElementById('group-tree-modal').style.display = 'block';
            } else {
                showMessage('获取组树失败: ' + (result.data?.message || '未知错误'), 'error');
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
        if (!container) {
            console.error('无法找到组树容器');
            return;
        }
        container.innerHTML = '';
    }

    const nodeElement = document.createElement('div');
    nodeElement.className = 'tree-node';
    nodeElement.style.marginLeft = (level * 20) + 'px';

    nodeElement.innerHTML = `
        <div class="tree-node-content">
            <span class="tree-node-name">${treeNode.group.name}</span>
            <span class="tree-node-info">(ID: ${treeNode.group.id}, 引用数: ${treeNode.group.reference_count})</span>
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
    const modal = document.getElementById('group-tree-modal');
    if (modal) {
        modal.style.display = 'none';
    }
}

// 显示组悬浮窗
function showGroupTooltip(event) {
    // 移除已存在的悬浮窗
    hideGroupTooltip();
    
    // 获取组数据
    const groupData = JSON.parse(event.target.getAttribute('data-group').replace(/&quot;/g, '"'));
    
    // 创建悬浮窗
    const tooltip = document.createElement('div');
    tooltip.id = 'group-tooltip';
    tooltip.className = 'tooltip';
    
    // 构建悬浮窗内容
    tooltip.innerHTML = `
        <ul class="tooltip-content">
            <li><span class="label">ID:</span> <span class="value">${groupData.id}</span></li>
            <li><span class="label">名称:</span> <span class="value">${groupData.name}</span></li>
            <li><span class="label">描述:</span> <span class="value">${groupData.description || '无'}</span></li>
            <li><span class="label">引用计数:</span> <span class="value">${groupData.reference_count}</span></li>
            <li><span class="label">主组:</span> <span class="value">${groupData.is_primary ? '是' : '否'}</span></li>
            <li><span class="label">点击次数:</span> <span class="value">${groupData.click_count}</span></li>
            <li><span class="label">分享次数:</span> <span class="value">${groupData.share_count}</span></li>
            <li><span class="label">父组ID:</span> <span class="value">${groupData.parent_id === null ? '无' : groupData.parent_id}</span></li>
        </ul>
    `;
    
    // 添加到文档中
    document.body.appendChild(tooltip);
    
    // 定位悬浮窗
    const rect = event.target.getBoundingClientRect();
    tooltip.style.left = rect.left + (rect.width / 2) - (tooltip.offsetWidth / 2) + 'px';
    tooltip.style.top = (rect.top - tooltip.offsetHeight - 10) + 'px';
    
    // 确保悬浮窗不会超出视窗边界
    const tooltipRect = tooltip.getBoundingClientRect();
    if (tooltipRect.left < 0) {
        tooltip.style.left = '10px';
    } else if (tooltipRect.right > window.innerWidth) {
        tooltip.style.left = (window.innerWidth - tooltip.offsetWidth - 10) + 'px';
    }
}

// 隐藏组悬浮窗
function hideGroupTooltip() {
    const existingTooltip = document.getElementById('group-tooltip');
    if (existingTooltip) {
        existingTooltip.remove();
    }
}

// 显示组详细信息
function showGroupInfo(groupId) {
    const url = `${BASE_URL}/api/groups/${groupId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                const group = result.data;
                const modalBody = document.getElementById('modal-body');
                modalBody.innerHTML = `
                    <h2>组详细信息</h2>
                    <div class="group-details">
                        <div class="form-group">
                            <label><strong>ID:</strong></label>
                            <span>${group.id}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>名称:</strong></label>
                            <span>${group.name}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>描述:</strong></label>
                            <span>${group.description || '无'}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>引用计数:</strong></label>
                            <span>${group.reference_count}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>主组:</strong></label>
                            <span>${group.is_primary ? '是' : '否'}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>点击次数:</strong></label>
                            <span>${group.click_count}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>分享次数:</strong></label>
                            <span>${group.share_count}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>父组ID:</strong></label>
                            <span>${group.parent_id === null ? '无' : group.parent_id}</span>
                        </div>
                    </div>
                    <button type="button" class="btn-secondary" onclick="closeModal()">关闭</button>
                `;
                document.getElementById('modal').style.display = 'block';
            } else {
                showMessage('获取组信息失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取组信息失败: ' + error.message, 'error');
        });
}
