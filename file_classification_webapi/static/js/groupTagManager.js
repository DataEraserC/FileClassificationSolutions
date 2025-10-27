// 组标签关联相关函数

// 添加分页相关变量
let currentGroupTagPage = 1;
let groupTagPageSize = 10;
let totalGroupTagPages = 1;
let currentGroupTagConditions = null;

function listGroupTagsByFilter() {
    const groupId = getInputValue('group-tag-group-id');
    const tagId = getInputValue('group-tag-tag-id');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (groupId) params.append('group_id', groupId);
    if (tagId) params.append('tag_id', tagId);
    
    // 构造分页参数
    const options = {
        page: currentGroupTagPage,
        page_size: groupTagPageSize
    };
    
    // 保存当前条件
    currentGroupTagConditions = {};
    if (groupId) currentGroupTagConditions.group_id = groupId;
    if (tagId) currentGroupTagConditions.tag_id = tagId;
    
    // 构造查询参数
    const searchParams = new URLSearchParams({
        filter: JSON.stringify(currentGroupTagConditions),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/group-tags/search/by-filter-with-pagination?${searchParams.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success && data.data) {
                renderGroupTagTable(data.data.data || []);
                // 更新分页信息
                totalGroupTagPages = data.data.total_pages || 1;
                renderGroupTagPagination(data.data);
            } else {
                showMessage('组标签关联查询失败: ' + data.message, 'error');
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
    if (currentGroupTagConditions) {
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
            if (data.success && data.data) {
                renderGroupTagTable(data.data.data || []);
                // 更新分页信息
                totalGroupTagPages = data.data.total_pages || 1;
                renderGroupTagPagination(data.data);
            } else {
                showMessage('组标签关联查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组标签关联查询失败: ' + error.message, 'error');
        });
}

// 在页面加载完成后绑定分页控件事件
document.addEventListener('DOMContentLoaded', function() {
    // 使用事件委托处理分页按钮点击
    document.addEventListener('click', function(e) {
        // 处理组标签分页按钮点击
        if (e.target.closest('#group-tags-pagination') && e.target.tagName === 'BUTTON') {
            const button = e.target;
            if (button.hasAttribute('onclick')) {
                // 防止重复绑定事件
                return;
            }
            
            const pageMatch = button.textContent.match(/(\d+)/);
            if (button.textContent === '首页') {
                changeGroupTagPage(1);
            } else if (button.textContent === '上一页') {
                changeGroupTagPage(currentGroupTagPage - 1);
            } else if (button.textContent === '下一页') {
                changeGroupTagPage(currentGroupTagPage + 1);
            } else if (button.textContent === '末页') {
                changeGroupTagPage(totalGroupTagPages);
            } else if (pageMatch) {
                changeGroupTagPage(parseInt(pageMatch[1]));
            }
        }
    });
});

// 改变每页大小
function changeGroupTagPageSize(size) {
    groupTagPageSize = parseInt(size);
    currentGroupTagPage = 1; // 重置到第一页
    if (currentGroupTagConditions) {
        searchGroupTagsByConditions(currentGroupTagConditions);
    } else {
        searchGroupTagsByFilter();
    }
}

function createGroupTag() {
    const groupId = getInputValue('create-group-tag-group-id');
    const tagId = getInputValue('create-group-tag-tag-id');
    
    if (!groupId || !tagId) {
        showMessage('请填写完整的组标签信息', 'warning');
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
    if (!confirm(`确定要删除组标签关联 [组ID: ${groupId}, 标签ID: ${tagId}] 吗？`)) {
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
    
    const groupTags = Array.from(selectedCheckboxes).map(cb => {
        return {
            group_id: parseInt(cb.getAttribute('data-group-id')),
            tag_id: parseInt(cb.getAttribute('data-tag-id'))
        };
    });
    
    // 使用新的delete by dtos接口
    deleteGroupTagsByDtos(groupTags);
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
                <div id="visual-search-conditions"></div>
                <button type="submit">查询</button>
                <button type="button" onclick="closeModal()">取消</button>
            </form>
        </div>
        <div id="json-search" class="tab-content">
            <form id="json-group-tag-search-form">
                <div class="form-group">
                    <label for="complex-search-group-tag-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-group-tag-conditions" rows="5" placeholder='[{"GroupId": 1}]'></textarea>
                </div>
                <button type="submit">查询</button>
                <button type="button" onclick="closeModal()">取消</button>
            </form>
        </div>
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
        page: currentGroupTagPage,
        page_size: groupTagPageSize
    };
    
    // 保存当前条件
    currentGroupTagConditions = conditions;
    
    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/group-tags/search/by-conditions-with-pagination?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                closeModal();
                renderGroupTagTable(data.data.data || []);
                // 更新分页信息
                totalGroupTagPages = data.data.total_pages || 1;
                renderGroupTagPagination(data.data);
            } else {
                showMessage('组标签关联查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('组标签关联查询失败: ' + error.message, 'error');
        });
}

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