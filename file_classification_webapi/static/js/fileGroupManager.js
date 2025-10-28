// 文件组关联相关函数

// 添加分页相关变量
let currentFileGroupPage = 1;
let fileGroupPageSize = 10;
let totalFileGroupPages = 1;
let currentFileGroupConditions = null;
let currentFileGroupQueryType = null; // 'filter' or 'conditions'

function listFileGroupsByFilter() {
    const fileId = getInputValue('file-group-file-id');
    const groupId = getInputValue('file-group-group-id');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (fileId) params.append('file_id', fileId);
    if (groupId) params.append('group_id', groupId);
    
    // 构造分页参数
    const options = {
        page: currentFileGroupPage,
        page_size: fileGroupPageSize
    };
    
    // 保存当前条件
    currentFileGroupConditions = {};
    if (fileId) currentFileGroupConditions.file_id = fileId;
    if (groupId) currentFileGroupConditions.group_id = groupId;
    
    // 标记使用filter查询
    currentFileGroupQueryType = 'filter';
    
    // 构造查询参数
    const searchParams = new URLSearchParams({
        filter: JSON.stringify(currentFileGroupConditions),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/file-groups/search/by-filter-with-pagination?${searchParams.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success && data.data) {
                renderFileGroupTable(data.data.data || []);
                // 更新分页信息
                totalFileGroupPages = data.data.total_pages || 1;
                renderFileGroupPagination(data.data);
            } else {
                showMessage('文件组关联查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('文件组关联查询失败: ' + error.message, 'error');
        });
}

// 渲染文件组表格
function renderFileGroupTable(fileGroups) {
    const tbody = document.querySelector('#file-groups-table tbody');
    if (!tbody) return;
    
    if (!fileGroups || fileGroups.length === 0) {
        tbody.innerHTML = '<tr><td colspan="5">暂无数据</td></tr>';
        return;
    }
    
    tbody.innerHTML = fileGroups.map(fg => `
        <tr>
            <td><input type="checkbox" class="file-group-checkbox" data-file-id="${fg.file_id}" data-group-id="${fg.group_id}"></td>
            <td>${fg.file_id}</td>
            <td>${fg.group_id}</td>
            <td>
                <div class="table-actions">
                    <button class="action-button delete" onclick="deleteFileGroup(${fg.file_id}, ${fg.group_id})">删除</button>
                </div>
            </td>
        </tr>
    `).join('');
}

// 渲染文件组分页控件
function renderFileGroupPagination(data) {
    const paginationContainer = document.getElementById('file-groups-pagination');
    if (!paginationContainer) return;
    
    const currentPage = data.page || currentFileGroupPage;
    const totalPages = data.total_pages || totalFileGroupPages;
    const totalRecords = data.total || 0;
    const pageSize = data.page_size || fileGroupPageSize;
    
    let paginationHTML = `
        <div class="pagination-container">
            <div class="pagination-info">
                共 ${totalRecords} 条记录，第 ${currentPage} 页/共 ${totalPages} 页
            </div>
            <div class="pagination-controls">
                <button onclick="changeFileGroupPage(1)" ${currentPage <= 1 ? 'disabled' : ''}>首页</button>
                <button onclick="changeFileGroupPage(${currentPage - 1})" ${currentPage <= 1 ? 'disabled' : ''}>上一页</button>
                <span class="page-numbers">
    `;
    
    // 显示页码
    let startPage = Math.max(1, currentPage - 2);
    let endPage = Math.min(totalPages, currentPage + 2);
    
    if (startPage > 1) {
        paginationHTML += `<button onclick="changeFileGroupPage(1)">1</button>`;
        if (startPage > 2) paginationHTML += `<span>...</span>`;
    }
    
    for (let i = startPage; i <= endPage; i++) {
        if (i === currentPage) {
            paginationHTML += `<button class="active">${i}</button>`;
        } else {
            paginationHTML += `<button onclick="changeFileGroupPage(${i})">${i}</button>`;
        }
    }
    
    if (endPage < totalPages) {
        if (endPage < totalPages - 1) paginationHTML += `<span>...</span>`;
        paginationHTML += `<button onclick="changeFileGroupPage(${totalPages})">${totalPages}</button>`;
    }
    
    paginationHTML += `
                </span>
                <button onclick="changeFileGroupPage(${currentPage + 1})" ${currentPage >= totalPages ? 'disabled' : ''}>下一页</button>
                <button onclick="changeFileGroupPage(${totalPages})" ${currentPage >= totalPages ? 'disabled' : ''}>末页</button>
            </div>
            <div class="pagination-size">
                每页显示: 
                <select onchange="changeFileGroupPageSize(this.value)">
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
function changeFileGroupPage(page) {
    if (page < 1 || page > totalFileGroupPages) return;
    currentFileGroupPage = page;
    // 根据查询类型选择接口
    if (currentFileGroupQueryType === 'conditions') {
        searchFileGroupsByConditions(currentFileGroupConditions);
    } else {
        searchFileGroupsByFilter();
    }
}

// 使用filter方式搜索文件组（用于分页）
function searchFileGroupsByFilter() {
    // 构造查询参数
    let params = new URLSearchParams();
    if (currentFileGroupConditions && currentFileGroupConditions.file_id) params.append('file_id', currentFileGroupConditions.file_id);
    if (currentFileGroupConditions && currentFileGroupConditions.group_id) params.append('group_id', currentFileGroupConditions.group_id);
    
    // 构造分页参数
    const options = {
        page: currentFileGroupPage,
        page_size: fileGroupPageSize
    };
    
    // 构造查询参数
    const searchParams = new URLSearchParams({
        filter: JSON.stringify(currentFileGroupConditions || {}),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/file-groups/search/by-filter-with-pagination?${searchParams.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success && data.data) {
                renderFileGroupTable(data.data.data || []);
                // 更新分页信息
                totalFileGroupPages = data.data.total_pages || 1;
                renderFileGroupPagination(data.data);
            } else {
                showMessage('文件组关联查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('文件组关联查询失败: ' + error.message, 'error');
        });
}

// 在页面加载完成后绑定分页控件事件
document.addEventListener('DOMContentLoaded', function() {
    // 使用事件委托处理分页按钮点击
    document.addEventListener('click', function(e) {
        // 处理文件组分页按钮点击
        if (e.target.closest('#file-groups-pagination') && e.target.tagName === 'BUTTON') {
            const button = e.target;
            if (button.hasAttribute('onclick')) {
                // 防止重复绑定事件
                return;
            }
            
            const pageMatch = button.textContent.match(/(\d+)/);
            if (button.textContent === '首页') {
                changeFileGroupPage(1);
            } else if (button.textContent === '上一页') {
                changeFileGroupPage(currentFileGroupPage - 1);
            } else if (button.textContent === '下一页') {
                changeFileGroupPage(currentFileGroupPage + 1);
            } else if (button.textContent === '末页') {
                changeFileGroupPage(totalFileGroupPages);
            } else if (pageMatch) {
                changeFileGroupPage(parseInt(pageMatch[1]));
            }
        }
    });
});

// 改变每页大小
function changeFileGroupPageSize(size) {
    fileGroupPageSize = parseInt(size);
    currentFileGroupPage = 1; // 重置到第一页
    if (currentFileGroupConditions) {
        searchFileGroupsByConditions(currentFileGroupConditions);
    } else {
        searchFileGroupsByFilter();
    }
}

function createFileGroup() {
    const fileId = getInputValue('create-file-group-file-id');
    const groupId = getInputValue('create-file-group-group-id');
    
    if (!fileId || !groupId) {
        showMessage('请填写完整的文件组信息', 'warning');
        return;
    }
    
    const fileGroupData = {
        file_id: parseInt(fileId),
        group_id: parseInt(groupId),
        relation_type: 1
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
    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', `确定要删除文件组关联 [文件ID: ${fileId}, 组ID: ${groupId}] 吗？`, function(result) {
        if (result) {
            const fileGroupData = {
                file_id: fileId,
                group_id: groupId,
                relation_type: 1
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
    });
}

// 批量删除选中的文件组关联
function deleteSelectedFileGroups() {
    const selectedCheckboxes = document.querySelectorAll('.file-group-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个文件组关联进行删除', 'warning');
        return;
    }

    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', `确定要删除这 ${selectedCheckboxes.length} 个文件组关联吗？`, function(result) {
        if (result) {
            const fileGroups = Array.from(selectedCheckboxes).map(cb => {
                return {
                    file_id: parseInt(cb.getAttribute('data-file-id')),
                    group_id: parseInt(cb.getAttribute('data-group-id')),
                    relation_type: 1
                };
            });

            // 使用新的delete by dtos接口
            deleteFileGroupsByDtos(fileGroups);
        }
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
            <button type="submit" class="btn-primary">创建</button>
            <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-file-group-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createFileGroup();
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 批量删除文件组关联（根据DTO列表）
function deleteFileGroupsByDtos(dtos) {
    const url = `${BASE_URL}/api/file-groups/delete/by-dtos`;
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
                <div id="visual-search-conditions"></div>
                <button type="submit" class="btn-primary">查询</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        </div>
        <div id="json-search" class="tab-content">
            <form id="json-file-group-search-form">
                <div class="form-group">
                    <label for="complex-search-file-group-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-file-group-conditions" rows="5" placeholder='[{"FileId": 1}]'></textarea>
                </div>
                <button type="submit" class="btn-primary">查询</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        </div>
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
        page: currentFileGroupPage,
        page_size: fileGroupPageSize
    };
    
    // 保存当前条件
    currentFileGroupConditions = conditions;
    
    // 标记使用conditions查询
    currentFileGroupQueryType = 'conditions';
    
    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/file-groups/search/by-conditions-with-pagination?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                closeModal();
                renderFileGroupTable(data.data.data || []);
                // 更新分页信息
                totalFileGroupPages = data.data.total_pages || 1;
                renderFileGroupPagination(data.data);
            } else {
                showMessage('文件组关联查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('文件组关联查询失败: ' + error.message, 'error');
        });
}

// 重置文件组过滤器
function resetFileGroupFilter() {
    document.getElementById('file-group-file-id').value = '';
    document.getElementById('file-group-group-id').value = '';
    // 重置分页参数
    currentFileGroupPage = 1;
    listFileGroupsByFilter(); // 重置后重新搜索
}

// 切换全选文件组
function toggleAllFileGroups(source) {
    const checkboxes = document.querySelectorAll('.file-group-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.checked = source.checked;
    });
}