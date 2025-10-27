// 文件管理相关函数

// 添加分页相关变量
let currentFilePage = 1;
let filePageSize = 10;
let totalFilePages = 1;
let currentFileConditions = null;

function listFilesByFilter() {
    const fileId = getInputValue('file-id');
    const fileType = getInputValue('file-type');
    const filePath = getInputValue('file-path');
    
    // 构造查询参数
    let params = new URLSearchParams();
    if (fileId) params.append('id', fileId);
    if (fileType) params.append('type_', fileType);
    if (filePath) params.append('path', filePath);
    
    // 构造分页参数
    const options = {
        page: currentFilePage,
        page_size: filePageSize
    };
    
    // 保存当前条件
    currentFileConditions = {};
    if (fileId) currentFileConditions.id = parseInt(fileId);
    if (fileType) currentFileConditions.type_ = fileType;
    if (filePath) currentFileConditions.path = filePath;
    
    // 构造查询参数 - 修复参数格式问题
    const searchParams = new URLSearchParams();
    searchParams.append('filter', JSON.stringify(currentFileConditions));
    searchParams.append('options', JSON.stringify(options));
    
    const url = `${BASE_URL}/api/files/search/by-filter-with-pagination?${searchParams.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success && data.data) {
                renderFileTable(data.data.data || []);
                // 更新分页信息
                totalFilePages = data.data.total_pages || 1;
                renderFilePagination(data.data);
            } else {
                renderFileTable([]);
                renderFilePagination({ page: 1, total_pages: 1, total: 0 });
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('查询文件失败: ' + error.message, 'error');
            renderFileTable([]);
            renderFilePagination({ page: 1, total_pages: 1, total: 0 });
        });
}

function renderFileTable(files) {
    const tableBody = document.querySelector('#files-table tbody');
    tableBody.innerHTML = '';
    
    files.forEach(file => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><input type="checkbox" class="file-checkbox" data-id="${file.id}"></td>
            <td>${file.id}</td>
            <td>${file.type_}</td>
            <td>${file.path}</td>
            <td>${file.reference_count}</td>
            <td>${file.group_id}</td>
            <td>
                <button class="action-button edit" onclick="openEditFileDialog(${file.id})">修改</button>
                <button class="action-button delete" onclick="deleteFile(${file.id})">删除</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

// 渲染分页控件
function renderFilePagination(paginationData) {
    const paginationContainer = document.getElementById('files-pagination');
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
                <button onclick="changeFilePage(1)" ${currentPage <= 1 ? 'disabled' : ''}>首页</button>
                <button onclick="changeFilePage(${currentPage - 1})" ${currentPage <= 1 ? 'disabled' : ''}>上一页</button>
                <span class="page-numbers">
    `;
    
    // 显示页码
    let startPage = Math.max(1, currentPage - 2);
    let endPage = Math.min(totalPages, currentPage + 2);
    
    if (startPage > 1) {
        paginationHTML += `<button onclick="changeFilePage(1)">1</button>`;
        if (startPage > 2) paginationHTML += `<span>...</span>`;
    }
    
    for (let i = startPage; i <= endPage; i++) {
        if (i === currentPage) {
            paginationHTML += `<button class="active">${i}</button>`;
        } else {
            paginationHTML += `<button onclick="changeFilePage(${i})">${i}</button>`;
        }
    }
    
    if (endPage < totalPages) {
        if (endPage < totalPages - 1) paginationHTML += `<span>...</span>`;
        paginationHTML += `<button onclick="changeFilePage(${totalPages})">${totalPages}</button>`;
    }
    
    paginationHTML += `
                </span>
                <button onclick="changeFilePage(${currentPage + 1})" ${currentPage >= totalPages ? 'disabled' : ''}>下一页</button>
                <button onclick="changeFilePage(${totalPages})" ${currentPage >= totalPages ? 'disabled' : ''}>末页</button>
            </div>
            <div class="pagination-size">
                每页显示: 
                <select onchange="changeFilePageSize(this.value)">
                    <option value="10" ${filePageSize === 10 ? 'selected' : ''}>10</option>
                    <option value="20" ${filePageSize === 20 ? 'selected' : ''}>20</option>
                    <option value="50" ${filePageSize === 50 ? 'selected' : ''}>50</option>
                    <option value="100" ${filePageSize === 100 ? 'selected' : ''}>100</option>
                </select>
            </div>
        </div>
    `;
    
    paginationContainer.innerHTML = paginationHTML;
}

// 改变页码
function changeFilePage(page) {
    if (page < 1 || page > totalFilePages) return;
    currentFilePage = page;
    // 检查是否有查询条件，如果有则使用conditions接口，否则使用filter接口
    if (currentFileConditions && Object.keys(currentFileConditions).length > 0) {
        searchFilesByConditions(currentFileConditions);
    } else {
        listFilesByFilter();
    }
}

// 改变每页大小
function changeFilePageSize(size) {
    filePageSize = parseInt(size);
    currentFilePage = 1; // 重置到第一页
    // 检查是否有查询条件，如果有则使用conditions接口，否则使用filter接口
    if (currentFileConditions && Object.keys(currentFileConditions).length > 0) {
        searchFilesByConditions(currentFileConditions);
    } else {
        listFilesByFilter();
    }
}

function getFileById() {
    const fileId = getInputValue('file-id');
    if (!fileId) {
        showMessage('请输入文件ID', 'warning');
        return;
    }
    
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                renderFileTable([data.data]);
            } else {
                showMessage('获取文件失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取文件失败: ' + error.message, 'error');
        });
}

function createFile() {
    const fileType = getInputValue('create-file-type');
    const filePath = getInputValue('create-file-path');
    const groupId = getInputValue('create-file-group-id');
    
    if (!fileType || !filePath || !groupId) {
        showMessage('请填写完整的文件信息', 'warning');
        return;
    }
    
    const fileData = {
        type_: fileType,
        path: filePath,
        group_id: parseInt(groupId)
    };
    
    const url = `${BASE_URL}/api/files`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(fileData)
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            showMessage('文件创建成功', 'success');
            closeModal();
            // 重新加载文件列表
            currentFilePage = 1;
            listFilesByFilter();
        } else {
            showMessage('文件创建失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件创建失败: ' + error.message, 'error');
    });
}

function updateFile() {
    const fileId = getInputValue('edit-file-id');
    const fileType = getInputValue('edit-file-type');
    const filePath = getInputValue('edit-file-path');
    const groupId = getInputValue('edit-file-group-id');
    
    if (!fileId || !fileType || !filePath || !groupId) {
        showMessage('请填写完整的文件信息', 'warning');
        return;
    }
    
    const updateData = {
        type_: fileType,
        path: filePath,
        group_id: parseInt(groupId)
    };
    
    const url = `${BASE_URL}/api/files/${fileId}`;
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
            showMessage('文件更新成功', 'success');
            closeModal();
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            showMessage('文件更新失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件更新失败: ' + error.message, 'error');
    });
}

function deleteFile(fileId) {
    if (!confirm('确定要删除该文件吗？')) {
        return;
    }
    
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url, {
        method: 'DELETE'
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            showMessage('文件删除成功', 'success');
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            showMessage('文件删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件删除失败: ' + error.message, 'error');
    });
}

// 批量删除选中的文件
function deleteSelectedFiles() {
    const selectedCheckboxes = document.querySelectorAll('.file-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个文件进行删除', 'warning');
        return;
    }
    
    if (!confirm(`确定要删除这 ${selectedCheckboxes.length} 个文件吗？`)) {
        return;
    }
    
    const ids = Array.from(selectedCheckboxes).map(cb => parseInt(cb.getAttribute('data-id')));
    
    // 使用新的delete by ids接口
    deleteFilesByIds(ids);
}

// 打开创建文件对话框
function openCreateFileDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>新增文件</h2>
        <form id="create-file-form">
            <div class="form-group">
                <label for="create-file-type">文件类型:</label>
                <input type="text" id="create-file-type" required>
            </div>
            <div class="form-group">
                <label for="create-file-path">文件路径:</label>
                <input type="text" id="create-file-path" required>
            </div>
            <div class="form-group">
                <label for="create-file-group-id">组ID:</label>
                <input type="number" id="create-file-group-id" required>
            </div>
            <button type="submit">创建</button>
            <button type="button" onclick="closeModal()">取消</button>
        </form>
    `;
    
    // 绑定表单提交事件
    document.getElementById('create-file-form').addEventListener('submit', function(e) {
        e.preventDefault();
        createFile();
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 打开编辑文件对话框
function openEditFileDialog(fileId) {
    // 首先获取文件信息
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                const file = data.data;
                const modalBody = document.getElementById('modal-body');
                modalBody.innerHTML = `
                    <h2>编辑文件</h2>
                    <form id="edit-file-form">
                        <div class="form-group">
                            <label for="edit-file-id">文件ID:</label>
                            <input type="number" id="edit-file-id" value="${file.id}" readonly>
                        </div>
                        <div class="form-group">
                            <label for="edit-file-type">文件类型:</label>
                            <input type="text" id="edit-file-type" value="${file.type_}" required>
                        </div>
                        <div class="form-group">
                            <label for="edit-file-path">文件路径:</label>
                            <input type="text" id="edit-file-path" value="${file.path}" required>
                        </div>
                        <div class="form-group">
                            <label for="edit-file-group-id">组ID:</label>
                            <input type="number" id="edit-file-group-id" value="${file.group_id}" required>
                        </div>
                        <button type="submit">更新</button>
                        <button type="button" onclick="closeModal()">取消</button>
                    </form>
                `;
                
                // 绑定表单提交事件
                document.getElementById('edit-file-form').addEventListener('submit', function(e) {
                    e.preventDefault();
                    updateFile();
                });
                
                document.getElementById('modal').style.display = 'block';
            } else {
                showMessage('获取文件信息失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取文件信息失败: ' + error.message, 'error');
        });
}

// 打开批量删除文件对话框
function openBatchDeleteFileDialog() {
    const modalBody = document.getElementById('modal-body');
    
    // 获取当前选中的文件ID
    const selectedFileCheckboxes = document.querySelectorAll('.file-checkbox:checked');
    const selectedFileIds = Array.from(selectedFileCheckboxes).map(cb => parseInt(cb.value)).map(id => parseInt(id));
    
    let formContent;
    if (selectedFileIds.length > 0) {
        formContent = `
            <h2>批量删除文件</h2>
            <p>已选择 ${selectedFileIds.length} 个文件</p>
            <form id="batch-delete-file-form">
                <input type="hidden" id="selected-file-ids" value='${JSON.stringify(selectedFileIds)}'>
                <button type="submit">删除选中文件</button>
                <button type="button" onclick="closeModal()">取消</button>
            </form>
        `;
    } else {
        formContent = `
            <h2>批量删除文件</h2>
            <form id="batch-delete-file-form">
                <div class="form-group">
                    <label for="batch-delete-file-conditions">删除条件 (JSON格式):</label>
                    <textarea id="batch-delete-file-conditions" rows="5" placeholder='[{"Id": 1}, {"Type": "txt"}]'></textarea>
                </div>
                <button type="submit">删除</button>
                <button type="button" onclick="closeModal()">取消</button>
            </form>
        `;
    }
    
    modalBody.innerHTML = formContent;
    
    // 绑定表单提交事件
    document.getElementById('batch-delete-file-form').addEventListener('submit', function(e) {
        e.preventDefault();
        
        // 如果有选中的文件ID，使用delete by ids
        const selectedIdsInput = document.getElementById('selected-file-ids');
        if (selectedIdsInput) {
            const fileIds = JSON.parse(selectedIdsInput.value);
            // 确保数值字段是数字类型
            const fixedFileIds = fileIds.map(id => parseInt(id));
            deleteFilesByIds(fixedFileIds);
            return;
        }
        
        // 否则使用条件删除（向后兼容）
        const conditionsJson = document.getElementById('batch-delete-file-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入删除条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            deleteFilesByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

// 批量删除文件（根据ID列表）
function deleteFilesByIds(fileIds) {
    const url = `${BASE_URL}/api/files/delete/by-ids`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(fileIds)
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            showMessage('文件批量删除成功', 'success');
            closeModal();
            // 重新加载文件列表
            listFilesByFilter();
        } else {
            showMessage('文件批量删除失败: ' + data.message, 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件批量删除失败: ' + error.message, 'error');
    });
}

// 打开复杂查询文件对话框
function openComplexSearchFileDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询文件</h2>
        <div class="tabs">
            <button class="tab-button active" onclick="switchComplexSearchTab('visual')">可视化查询</button>
            <button class="tab-button" onclick="switchComplexSearchTab('json')">JSON查询</button>
        </div>
        <div id="visual-search" class="tab-content active">
            <form id="visual-file-search-form">
                <div class="form-group">
                    <label for="visual-search-field">查询字段:</label>
                    <select id="visual-search-field">
                        <option value="Id">ID</option>
                        <option value="Type">类型</option>
                        <option value="Path">路径</option>
                        <option value="ReferenceCount">引用计数</option>
                        <option value="GroupId">组ID</option>
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
            <form id="json-file-search-form">
                <div class="form-group">
                    <label for="complex-search-file-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-file-conditions" rows="5" placeholder='[{"Id": 1}, {"Type": "txt"}]'></textarea>
                </div>
                <button type="submit">查询</button>
            </form>
        </div>
        <button type="button" onclick="closeModal()">取消</button>
    `;
    
    // 绑定表单提交事件
    document.getElementById('json-file-search-form').addEventListener('submit', function(e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-file-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入查询条件', 'warning');
            return;
        }
        
        try {
            const conditions = JSON.parse(conditionsJson);
            searchFilesByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });
    
    document.getElementById('modal').style.display = 'block';
}

function searchFilesByConditions(conditions) {
    // 构造查询选项
    const options = {
        page: currentFilePage,
        page_size: filePageSize
    };
    
    // 保存当前条件
    currentFileConditions = conditions;
    
    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });
    
    const url = `${BASE_URL}/api/files/search/by-conditions-with-pagination?${params.toString()}`;
    
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                closeModal();
                renderFileTable(data.data.data || []);
                // 更新分页信息
                totalFilePages = data.data.total_pages || 1;
                renderFilePagination(data.data);
            } else {
                showMessage('文件查询失败: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('文件查询失败: ' + error.message, 'error');
        });
}