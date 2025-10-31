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
            const result = handleApiResponse(data);
            if (result.success && result.data) {
                renderGroupRelationTable(result.data.data || []);
                // 更新分页信息
                totalGroupRelationPages = result.data.total_pages || 1;
                renderGroupRelationPagination(result.data);
            } else {
                showMessage('组关系查询失败: ' + (result.data?.message || '未知错误'), 'error');
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

    tbody.innerHTML = groupRelations.map(relation => {
        // 根据关系类型显示对应的文本
        let relationTypeText = relation.relation_type;
        if (relation.relation_type === 1) {
            relationTypeText = '父与子关系';
        }
        
        return `
        <tr>
            <td><input type="checkbox" class="group-relation-checkbox" data-first-id="${relation.first_group_id}" data-second-id="${relation.second_group_id}" data-relation-type="${relation.relation_type}"></td>
            <td>${relation.first_group_id}</td>
            <td>${relation.second_group_id}</td>
            <td>${relationTypeText}</td>
            <td>
                <div class="table-actions">
                    <button class="action-button delete" onclick="deleteGroupRelation(${relation.first_group_id}, ${relation.second_group_id}, ${relation.relation_type})">删除</button>
                </div>
            </td>
        </tr>
    `}).join('');
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
            const result = handleApiResponse(data);
            if (result.success && result.data) {
                renderGroupRelationTable(result.data.data || []);
                // 更新分页信息
                totalGroupRelationPages = result.data.total_pages || 1;
                renderGroupRelationPagination(result.data);
            } else {
                showMessage('组关系查询失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组关系查询失败: ' + error.message, 'error');
        });
}

// 在页面加载完成后绑定分页控件事件
document.addEventListener('DOMContentLoaded', function () {
    // 绑定分页控件事件
    const groupRelationPagination = document.getElementById('group-relations-pagination');
    if (groupRelationPagination) {
        groupRelationPagination.addEventListener('click', function (event) {
            const target = event.target;
            if (target.tagName === 'BUTTON' && !target.disabled) {
                const page = parseInt(target.textContent);
                if (!isNaN(page)) {
                    changeGroupRelationPage(page);
                }
            }
        });
    }
});

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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('组关系创建成功', 'success');
                closeModal();
                // 重新加载组关系列表
                currentGroupRelationPage = 1;
                searchGroupRelationsByFilter();
            } else {
                showMessage('组关系创建失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组关系创建失败: ' + error.message, 'error');
        });
}

function deleteGroupRelation(firstId, secondId, relationType) {
    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', '确定要删除该组关系吗？', function (result) {
        if (result) {
            const url = `${BASE_URL}/api/group-relations`;
            fetch(url, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ 
                    first_group_id: firstId, 
                    second_group_id: secondId, 
                    relation_type: relationType 
                })
            })
                .then(response => response.json())
                .then(data => {
                    const result = handleApiResponse(data);
                    if (result.success) {
                        showMessage('组关系删除成功', 'success');
                        // 重新加载组关系列表
                        searchGroupRelationsByFilter();
                    } else {
                        showMessage('组关系删除失败: ' + (result.data?.message || '未知错误'), 'error');
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
    showConfirmDialog('确认删除', `确定要删除这 ${selectedCheckboxes.length} 个组关系吗？`, function (result) {
        if (result) {
            // 构造DTO数组
            const dtos = Array.from(selectedCheckboxes).map(cb => ({
                first_group_id: parseInt(cb.getAttribute('data-first-id')),
                second_group_id: parseInt(cb.getAttribute('data-second-id')),
                relation_type: parseInt(cb.getAttribute('data-relation-type'))
            }));

            // 使用新的delete by dtos接口
            deleteGroupRelationsByDtos(dtos);
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
                <label for="create-group-relation-first-id">第一组ID:</label>
                <input type="number" id="create-group-relation-first-id" required>
            </div>
            <div class="form-group">
                <label for="create-group-relation-second-id">第二组ID:</label>
                <input type="number" id="create-group-relation-second-id" required>
            </div>
            <div class="form-group">
                <label for="create-group-relation-type">关系类型:</label>
                <input type="number" id="create-group-relation-type" required>
                <div class="form-help">1 = 父与子关系</div>
            </div>
            <button type="submit" class="btn-primary">创建</button>
            <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
        </form>
    `;

    // 绑定表单提交事件
    document.getElementById('create-group-relation-form').addEventListener('submit', function (e) {
        e.preventDefault();
        createGroupRelation();
    });

    document.getElementById('modal').style.display = 'block';
}

// 打开批量删除组关系对话框
function openBatchDeleteGroupRelationDialog() {
    const modalBody = document.getElementById('modal-body');

    // 获取当前选中的组关系ID
    const selectedGroupRelationCheckboxes = document.querySelectorAll('.group-relation-checkbox:checked');
    const selectedGroupRelationIds = Array.from(selectedGroupRelationCheckboxes).map(cb => ({
        first_group_id: parseInt(cb.getAttribute('data-first-id')),
        second_group_id: parseInt(cb.getAttribute('data-second-id')),
        relation_type: parseInt(cb.getAttribute('data-relation-type'))
    }));

    let formContent;
    if (selectedGroupRelationIds.length > 0) {
        formContent = `
            <h2>批量删除组关系</h2>
            <p>已选择 ${selectedGroupRelationIds.length} 个组关系</p>
            <form id="batch-delete-group-relation-form">
                <input type="hidden" id="selected-group-relation-ids" value='${JSON.stringify(selectedGroupRelationIds)}'>
                <button type="submit">删除选中组关系</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        `;
    } else {
        formContent = `
            <h2>批量删除组关系</h2>
            <form id="batch-delete-group-relation-form">
                <div class="form-group">
                    <label for="batch-delete-group-relation-conditions">删除条件 (JSON格式):</label>
                    <textarea id="batch-delete-group-relation-conditions" rows="5" placeholder='[{"FirstGroupId": 1}, {"SecondGroupId": 1}]'></textarea>
                </div>
                <button type="submit">删除</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        `;
    }

    modalBody.innerHTML = formContent;

    // 绑定表单提交事件
    document.getElementById('batch-delete-group-relation-form').addEventListener('submit', function (e) {
        e.preventDefault();

        // 如果有选中的组关系ID，使用delete by dtos
        const selectedIdsInput = document.getElementById('selected-group-relation-ids');
        if (selectedIdsInput) {
            const groupRelationDtos = JSON.parse(selectedIdsInput.value);
            // 确保数值字段是数字类型
            const fixedGroupRelationDtos = groupRelationDtos.map(dto => ({
                first_group_id: parseInt(dto.first_group_id),
                second_group_id: parseInt(dto.second_group_id),
                relation_type: parseInt(dto.relation_type)
            }));
            deleteGroupRelationsByDtos(fixedGroupRelationDtos);
            return;
        }

        // 否则使用条件删除（向后兼容）
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

// 根据DTO列表批量删除组关系（根据ID列表）
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('组关系批量删除成功', 'success');
                closeModal();
                // 重新加载组关系列表
                searchGroupRelationsByFilter();
            } else {
                showMessage('组关系批量删除失败: ' + (result.data?.message || '未知错误'), 'error');
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('组关系批量删除成功', 'success');
                closeModal();
                // 重新加载组关系列表
                searchGroupRelationsByFilter();
            } else {
                showMessage('组关系批量删除失败: ' + (result.data?.message || '未知错误'), 'error');
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
                    <textarea id="complex-search-group-relation-conditions" rows="5" placeholder='[{"FirstGroupId": 1}, {"SecondGroupId": 1}]'></textarea>
                </div>
                <button type="submit">查询</button>
            </form>
        </div>
        <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
    `;

    // 绑定表单提交事件
    document.getElementById('json-group-relation-search-form').addEventListener('submit', function (e) {
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
            const result = handleApiResponse(data);
            if (result.success) {
                closeModal();
                renderGroupRelationTable(result.data.data || []);
                // 更新分页信息
                totalGroupRelationPages = result.data.total_pages || 1;
                renderGroupRelationPagination(result.data);
            } else {
                showMessage('组关系查询失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组关系查询失败: ' + error.message, 'error');
        });
}