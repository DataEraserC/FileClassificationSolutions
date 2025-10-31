// 组标签关联相关函数

// 添加分页相关变量
let currentGroupTagPage = 1;
let groupTagPageSize = 10;
let totalGroupTagPages = 1;
let currentGroupTagConditions = null;
let currentGroupTagQueryType = null; // 'filter' or 'conditions'

function listGroupTagsByFilter() {
    const groupId = getInputValue('group-tag-group-id');
    const tagId = getInputValue('group-tag-tag-id');

    // 构造查询参数
    let params = new URLSearchParams();
    if (groupId) params.append('group_id', parseInt(groupId));
    if (tagId) params.append('tag_id', parseInt(tagId));

    // 构造分页参数
    const options = {
        page: currentGroupTagPage,
        page_size: groupTagPageSize
    };

    // 保存当前条件
    currentGroupTagConditions = {};
    if (groupId) currentGroupTagConditions.group_id = parseInt(groupId);
    if (tagId) currentGroupTagConditions.tag_id = parseInt(tagId);

    // 标记使用filter查询
    currentGroupTagQueryType = 'filter';

    // 构造查询参数
    const searchParams = new URLSearchParams({
        filter: JSON.stringify(currentGroupTagConditions),
        options: JSON.stringify(options)
    });

    const url = `${BASE_URL}/api/group-tags/search/by-filter-with-pagination?${searchParams.toString()}`;

    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success && result.data) {
                renderGroupTagTable(result.data.data || []);
                // 更新分页信息
                totalGroupTagPages = result.data.total_pages || 1;
                renderGroupTagPagination(result.data);
            } else {
                showMessage('组标签关联查询失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组标签关联查询失败: ' + error.message, 'error');
        });
}

// 渲染组标签表格
function renderGroupTagTable(groupTags) {
    const tbody = document.querySelector('#group-tags-table tbody');
    if (!tbody) return;

    if (!groupTags || groupTags.length === 0) {
        tbody.innerHTML = '<tr><td colspan="4">暂无数据</td></tr>';
        return;
    }

    tbody.innerHTML = groupTags.map(gt => `
        <tr>
            <td><input type="checkbox" class="group-tag-checkbox" data-group-id="${gt.group_id}" data-tag-id="${gt.tag_id}"></td>
            <td>${gt.group_id}</td>
            <td>${gt.tag_id}</td>
            <td>
                <div class="table-actions">
                    <button class="action-button delete" onclick="deleteGroupTag(${gt.group_id}, ${gt.tag_id})">删除</button>
                </div>
            </td>
        </tr>
    `).join('');
}

// 渲染组标签分页控件
function renderGroupTagPagination(data) {
    const paginationContainer = document.getElementById('group-tags-pagination');
    if (!paginationContainer) return;

    const currentPage = data.page || currentGroupTagPage;
    const totalPages = data.total_pages || totalGroupTagPages;
    const totalRecords = data.total || 0;
    const pageSize = data.page_size || groupTagPageSize;

    let paginationHTML = `
        <div class="pagination-container">
            <div class="pagination-info">
                共 ${totalRecords} 条记录，第 ${currentPage} 页/共 ${totalPages} 页
            </div>
            <div class="pagination-controls">
                <button onclick="changeGroupTagPage(1)" ${currentPage <= 1 ? 'disabled' : ''}>首页</button>
                <button onclick="changeGroupTagPage(${currentPage - 1})" ${currentPage <= 1 ? 'disabled' : ''}>上一页</button>
                <span class="page-numbers">
    `;

    // 显示页码
    let startPage = Math.max(1, currentPage - 2);
    let endPage = Math.min(totalPages, currentPage + 2);

    if (startPage > 1) {
        paginationHTML += `<button onclick="changeGroupTagPage(1)">1</button>`;
        if (startPage > 2) paginationHTML += `<span>...</span>`;
    }

    for (let i = startPage; i <= endPage; i++) {
        if (i === currentPage) {
            paginationHTML += `<button class="active">${i}</button>`;
        } else {
            paginationHTML += `<button onclick="changeGroupTagPage(${i})">${i}</button>`;
        }
    }

    if (endPage < totalPages) {
        if (endPage < totalPages - 1) paginationHTML += `<span>...</span>`;
        paginationHTML += `<button onclick="changeGroupTagPage(${totalPages})">${totalPages}</button>`;
    }

    paginationHTML += `
                </span>
                <button onclick="changeGroupTagPage(${currentPage + 1})" ${currentPage >= totalPages ? 'disabled' : ''}>下一页</button>
                <button onclick="changeGroupTagPage(${totalPages})" ${currentPage >= totalPages ? 'disabled' : ''}>末页</button>
            </div>
            <div class="pagination-size">
                每页显示: 
                <select onchange="changeGroupTagPageSize(this.value)">
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
function changeGroupTagPage(page) {
    if (page < 1 || page > totalGroupTagPages) return;
    currentGroupTagPage = page;
    // 根据查询类型选择接口
    if (currentGroupTagQueryType === 'conditions') {
        searchGroupTagsByConditions(currentGroupTagConditions);
    } else {
        searchGroupTagsByFilter();
    }
}

// 使用filter方式搜索组标签（用于分页）
function searchGroupTagsByFilter() {
    // 构造查询参数
    let params = new URLSearchParams();
    if (currentGroupTagConditions && currentGroupTagConditions.group_id) params.append('group_id', currentGroupTagConditions.group_id);
    if (currentGroupTagConditions && currentGroupTagConditions.tag_id) params.append('tag_id', currentGroupTagConditions.tag_id);

    // 构造分页参数
    const options = {
        page: currentGroupTagPage,
        page_size: groupTagPageSize
    };

    // 构造查询参数
    const searchParams = new URLSearchParams({
        filter: JSON.stringify(currentGroupTagConditions || {}),
        options: JSON.stringify(options)
    });

    const url = `${BASE_URL}/api/group-tags/search/by-filter-with-pagination?${searchParams.toString()}`;

    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success && result.data) {
                renderGroupTagTable(result.data.data || []);
                // 更新分页信息
                totalGroupTagPages = result.data.total_pages || 1;
                renderGroupTagPagination(result.data);
            } else {
                showMessage('组标签关联查询失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组标签关联查询失败: ' + error.message, 'error');
        });
}

// 在页面加载完成后绑定分页控件事件
document.addEventListener('DOMContentLoaded', function () {
    // 绑定分页控件事件
    const groupTagPagination = document.getElementById('group-tags-pagination');
    if (groupTagPagination) {
        groupTagPagination.addEventListener('click', function (event) {
            const target = event.target;
            if (target.tagName === 'BUTTON' && !target.disabled) {
                const page = parseInt(target.textContent);
                if (!isNaN(page)) {
                    changeGroupTagPage(page);
                }
            }
        });
    }
});

// 重置组标签过滤器
function resetGroupTagFilter() {
    document.getElementById('group-tag-group-id').value = '';
    document.getElementById('group-tag-tag-id').value = '';
    // 重置分页参数
    currentGroupTagPage = 1;
    listGroupTagsByFilter(); // 重置后重新搜索
}

// 切换全选组标签
function toggleAllGroupTags(source) {
    const checkboxes = document.querySelectorAll('.group-tag-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.checked = source.checked;
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('组标签关联创建成功', 'success');
                closeModal();
                // 重新加载组标签列表
                currentGroupTagPage = 1;
                searchGroupTagsByFilter();
            } else {
                showMessage('组标签关联创建失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组标签关联创建失败: ' + error.message, 'error');
        });
}

function deleteGroupTag(groupId, tagId) {
    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', '确定要删除该组标签关联吗？', function (result) {
        if (result) {
            const url = `${BASE_URL}/api/group-tags`;
            fetch(url, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ group_id: groupId, tag_id: tagId })
            })
                .then(response => response.json())
                .then(data => {
                    const result = handleApiResponse(data);
                    if (result.success) {
                        showMessage('组标签关联删除成功', 'success');
                        // 重新加载组标签列表
                        searchGroupTagsByFilter();
                    } else {
                        showMessage('组标签关联删除失败: ' + (result.data?.message || '未知错误'), 'error');
                    }
                })
                .catch(error => {
                    console.error('Error:', error);
                    showMessage('组标签关联删除失败: ' + error.message, 'error');
                });
        }
    });
}

// 批量删除选中的组标签
function deleteSelectedGroupTags() {
    const selectedCheckboxes = document.querySelectorAll('.group-tag-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个组标签关联进行删除', 'warning');
        return;
    }

    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', `确定要删除这 ${selectedCheckboxes.length} 个组标签关联吗？`, function (result) {
        if (result) {
            // 构造DTO数组
            const dtos = Array.from(selectedCheckboxes).map(cb => ({
                group_id: parseInt(cb.getAttribute('data-group-id')),
                tag_id: parseInt(cb.getAttribute('data-tag-id'))
            }));

            // 使用新的delete by dtos接口
            deleteGroupTagsByDtos(dtos);
        }
    });
}

// 打开创建组标签对话框
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
            <button type="submit" class="btn-primary">创建</button>
            <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
        </form>
    `;

    // 绑定表单提交事件
    document.getElementById('create-group-tag-form').addEventListener('submit', function (e) {
        e.preventDefault();
        createGroupTag();
    });

    document.getElementById('modal').style.display = 'block';
}

// 打开批量删除组标签对话框
function openBatchDeleteGroupTagDialog() {
    const modalBody = document.getElementById('modal-body');

    // 获取当前选中的组标签ID
    const selectedGroupTagCheckboxes = document.querySelectorAll('.group-tag-checkbox:checked');
    const selectedGroupTagIds = Array.from(selectedGroupTagCheckboxes).map(cb => ({
        group_id: parseInt(cb.getAttribute('data-group-id')),
        tag_id: parseInt(cb.getAttribute('data-tag-id'))
    }));

    let formContent;
    if (selectedGroupTagIds.length > 0) {
        formContent = `
            <h2>批量删除组标签关联</h2>
            <p>已选择 ${selectedGroupTagIds.length} 个组标签关联</p>
            <form id="batch-delete-group-tag-form">
                <input type="hidden" id="selected-group-tag-ids" value='${JSON.stringify(selectedGroupTagIds)}'>
                <button type="submit">删除选中组标签关联</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        `;
    } else {
        formContent = `
            <h2>批量删除组标签关联</h2>
            <form id="batch-delete-group-tag-form">
                <div class="form-group">
                    <label for="batch-delete-group-tag-conditions">删除条件 (JSON格式):</label>
                    <textarea id="batch-delete-group-tag-conditions" rows="5" placeholder='[{"GroupId": 1}, {"TagId": 1}]'></textarea>
                </div>
                <button type="submit">删除</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        `;
    }

    modalBody.innerHTML = formContent;

    // 绑定表单提交事件
    document.getElementById('batch-delete-group-tag-form').addEventListener('submit', function (e) {
        e.preventDefault();

        // 如果有选中的组标签ID，使用delete by dtos
        const selectedIdsInput = document.getElementById('selected-group-tag-ids');
        if (selectedIdsInput) {
            const groupTagDtos = JSON.parse(selectedIdsInput.value);
            // 确保数值字段是数字类型
            const fixedGroupTagDtos = groupTagDtos.map(dto => ({
                group_id: parseInt(dto.group_id),
                tag_id: parseInt(dto.tag_id)
            }));
            deleteGroupTagsByDtos(fixedGroupTagDtos);
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

// 根据DTO列表批量删除组标签（根据ID列表）
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('组标签关联批量删除成功', 'success');
                closeModal();
                // 重新加载组标签列表
                searchGroupTagsByFilter();
            } else {
                showMessage('组标签关联批量删除失败: ' + (result.data?.message || '未知错误'), 'error');
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('组标签关联批量删除成功', 'success');
                closeModal();
                // 重新加载组标签列表
                searchGroupTagsByFilter();
            } else {
                showMessage('组标签关联批量删除失败: ' + (result.data?.message || '未知错误'), 'error');
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
        <h2>复杂查询组标签</h2>
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
                    <textarea id="complex-search-group-tag-conditions" rows="5" placeholder='[{"GroupId": 1}, {"TagId": 1}]'></textarea>
                </div>
                <button type="submit">查询</button>
            </form>
        </div>
        <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
    `;

    // 绑定表单提交事件
    document.getElementById('json-group-tag-search-form').addEventListener('submit', function (e) {
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
        page: currentGroupTagPage,
        page_size: groupTagPageSize
    };

    // 保存当前条件
    currentGroupTagConditions = conditions;

    // 标记使用conditions查询
    currentGroupTagQueryType = 'conditions';

    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });

    const url = `${BASE_URL}/api/group-tags/search/by-conditions-with-pagination?${params.toString()}`;

    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                closeModal();
                renderGroupTagTable(result.data.data || []);
                // 更新分页信息
                totalGroupTagPages = result.data.total_pages || 1;
                renderGroupTagPagination(result.data);
            } else {
                showMessage('组标签关联查询失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组标签关联查询失败: ' + error.message, 'error');
        });
}