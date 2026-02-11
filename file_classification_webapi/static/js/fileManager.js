// 文件管理相关函数

// 添加分页相关变量
let currentFilePage = 1;
let filePageSize = 10;
let totalFilePages = 1;
let currentFileConditions = null;
let currentFileQueryType = null; // 'filter' or 'conditions'

function listFilesByFilter() {
    const fileId = getInputValue('file-id');
    const fileType = getInputValue('file-type');
    const filePath = getInputValue('file-path');
    const fileDescription = getInputValue('file-description');

    // 构造查询参数
    let params = new URLSearchParams();
    if (fileId) params.append('id', fileId);
    if (fileType) params.append('type_', fileType);
    if (filePath) params.append('path', filePath);
    if (fileDescription) params.append('description', fileDescription);

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
    if (fileDescription) currentFileConditions.description = fileDescription;

    // 标记使用filter查询
    currentFileQueryType = 'filter';

    // 构造查询参数 - 修复参数格式问题
    const searchParams = new URLSearchParams();
    searchParams.append('filter', JSON.stringify(currentFileConditions));
    searchParams.append('options', JSON.stringify(options));

    const url = `${BASE_URL}/api/files/search/by-filter-with-pagination?${searchParams.toString()}`;

    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success && result.data) {
                renderFileTable(result.data.data || []);
                // 更新分页信息
                totalFilePages = result.data.total_pages || 1;
                renderFilePagination(result.data);
            } else {
                renderFileTable([]);
                renderFilePagination({page: 1, total_pages: 1, total: 0});
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('查询文件失败: ' + error.message, 'error');
            renderFileTable([]);
            renderFilePagination({page: 1, total_pages: 1, total: 0});
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
            <td>${file.path}</td>
            <td>${file.type_}</td>
            <td>${file.description === null ? '无' : (file.description || '')}</td>
            <td>
                <span class="info-icon" data-file='${JSON.stringify(file).replace(/"/g, '&quot;')}' onmouseover="showFileTooltip(event)" onmouseout="hideFileTooltip()" title="悬停查看详细信息">ℹ️</span>
                <button class="action-button info" onclick="showFileInfo(${file.id})" title="查看详细信息">详情</button>
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
    // 根据查询类型选择接口
    if (currentFileQueryType === 'conditions') {
        searchFilesByConditions(currentFileConditions);
    } else {
        listFilesByFilter();
    }
}

// 改变每页大小
function changeFilePageSize(size) {
    filePageSize = parseInt(size);
    currentFilePage = 1; // 重置到第一页
    // 根据查询类型选择接口
    if (currentFileQueryType === 'conditions') {
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
            const result = handleApiResponse(data);
            if (result.success) {
                renderFileTable([result.data]);
            } else {
                showMessage('获取文件失败: ' + (result.data?.message || '未知错误'), 'error');
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
    const fileDescription = getInputValue('create-file-description');

    if (!fileType || !filePath || !groupId) {
        showMessage('请填写完整的文件信息', 'warning');
        return;
    }

    const fileData = {
        type_: fileType,
        path: filePath,
        group_id: parseInt(groupId),
        description: fileDescription || null
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('文件创建成功', 'success');
                closeModal();
                // 重新加载文件列表
                currentFilePage = 1;
                listFilesByFilter();
            } else {
                showMessage('文件创建失败: ' + (result.data?.message || '未知错误'), 'error');
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
    const fileDescription = getInputValue('edit-file-description');

    if (!fileId || !fileType || !filePath || !groupId) {
        showMessage('请填写完整的文件信息', 'warning');
        return;
    }

    const updateData = {
        type_: fileType,
        path: filePath,
        group_id: parseInt(groupId),
        description: fileDescription || null
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('文件更新成功', 'success');
                closeModal();
                // 重新加载文件列表
                listFilesByFilter();
            } else {
                showMessage('文件更新失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('文件更新失败: ' + error.message, 'error');
        });
}

function deleteFile(fileId) {
    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', '确定要删除该文件吗？', function (result) {
        if (result) {
            const url = `${BASE_URL}/api/files/${fileId}`;
            fetch(url, {
                method: 'DELETE'
            })
                .then(response => response.json())
                .then(data => {
                    const result = handleApiResponse(data);
                    if (result.success) {
                        showMessage('文件删除成功', 'success');
                        // 重新加载文件列表
                        listFilesByFilter();
                    } else {
                        showMessage('文件删除失败: ' + (result.data?.message || '未知错误'), 'error');
                    }
                })
                .catch(error => {
                    console.error('Error:', error);
                    showMessage('文件删除失败: ' + error.message, 'error');
                });
        }
    });
}

// 批量删除选中的文件
function deleteSelectedFiles() {
    const selectedCheckboxes = document.querySelectorAll('.file-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个文件进行删除', 'warning');
        return;
    }

    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', `确定要删除这 ${selectedCheckboxes.length} 个文件吗？`, function (result) {
        if (result) {
            const ids = Array.from(selectedCheckboxes).map(cb => parseInt(cb.getAttribute('data-id')));

            // 使用新的delete by ids接口
            deleteFilesByIds(ids);
        }
    });
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
                <label for="create-file-upload">或上传文件:</label>
                <input type="file" id="create-file-upload">
            </div>
            <div class="form-group">
                <label for="create-file-group-id">组ID:</label>
                <input type="number" id="create-file-group-id" required>
            </div>
            <div class="form-group">
                <label for="create-file-description">文件描述:</label>
                <input type="text" id="create-file-description">
            </div>
            <button type="submit" class="btn-primary">创建</button>
            <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
        </form>
    `;

    // 绑定文件选择事件
    document.getElementById('create-file-upload').addEventListener('change', function(e) {
        const fileInput = e.target;
        const pathInput = document.getElementById('create-file-path');
        if (fileInput.files.length > 0) {
            // 如果选择了文件，禁用路径输入框
            pathInput.disabled = true;
        } else {
            // 如果没有选择文件，启用路径输入框
            pathInput.disabled = false;
        }
    });

    // 绑定表单提交事件
    document.getElementById('create-file-form').addEventListener('submit', function (e) {
        e.preventDefault();
        
        // 检查是否选择了上传文件
        const fileUpload = document.getElementById('create-file-upload');
        if (fileUpload.files.length > 0) {
            // 如果选择了文件，先上传文件
            uploadAndCreateFile(fileUpload.files[0]);
        } else {
            // 否则直接创建文件
            createFile();
        }
    });

    document.getElementById('modal').style.display = 'block';
}

// 上传文件并创建文件记录
function uploadAndCreateFile(file) {
    const formData = new FormData();
    formData.append('file', file);

    // 上传文件
    fetch(`${BASE_URL}/api/uploads`, {
        method: 'POST',
        body: formData
    })
    .then(response => response.json())
    .then(data => {
        if (data.url) {
            // 将上传后的文件URL设置为路径
            document.getElementById('create-file-path').value = data.url;
            
            // 创建文件记录
            createFile();
        } else {
            showMessage('文件上传失败: ' + (data.error || '未知错误'), 'error');
        }
    })
    .catch(error => {
        console.error('Error:', error);
        showMessage('文件上传失败: ' + error.message, 'error');
    });
}

// 打开编辑文件对话框
function openEditFileDialog(fileId) {
    // 首先获取文件信息
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                const file = result.data;
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
                        <div class="form-group">
                            <label for="edit-file-description">文件描述:</label>
                            <input type="text" id="edit-file-description" value="${file.description || ''}">
                        </div>
                        <button type="submit" class="btn-primary">更新</button>
                        <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
                    </form>
                `;

                // 绑定表单提交事件
                document.getElementById('edit-file-form').addEventListener('submit', function (e) {
                    e.preventDefault();
                    updateFile();
                });

                document.getElementById('modal').style.display = 'block';
            } else {
                showMessage('获取文件信息失败: ' + (result.data?.message || '未知错误'), 'error');
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
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        `;
    } else {
        formContent = `
            <h2>批量删除文件</h2>
            <form id="batch-delete-file-form">
                <div class="form-group">
                    <label for="batch-delete-file-conditions">删除条件 (JSON格式):</label>
                    <textarea id="batch-delete-file-conditions" rows="5" placeholder='[{"Id": 1}, {"Type_": "example"}]'></textarea>
                </div>
                <button type="submit">删除</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        `;
    }

    modalBody.innerHTML = formContent;

    // 绑定表单提交事件
    document.getElementById('batch-delete-file-form').addEventListener('submit', function (e) {
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
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('文件批量删除成功', 'success');
                closeModal();
                // 重新加载文件列表
                listFilesByFilter();
            } else {
                showMessage('文件批量删除失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('文件批量删除失败: ' + error.message, 'error');
        });
}

function deleteFilesByConditions(conditions) {
    const url = `${BASE_URL}/api/files/delete/by-conditions`;
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
                showMessage('文件批量删除成功', 'success');
                closeModal();
                // 重新加载文件列表
                listFilesByFilter();
            } else {
                showMessage('文件批量删除失败: ' + (result.data?.message || '未知错误'), 'error');
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
                <div class="form-group-container" style="display: flex; gap: 10px; align-items: flex-end; flex-wrap: wrap;">
                    <div class="form-group" style="margin-bottom: 0;">
                        <label for="visual-search-field">查询字段:</label>
                        <select id="visual-search-field">
                            <option value="Id">ID</option>
                            <option value="Type_">类型</option>
                            <option value="Path">路径</option>
                            <option value="ReferenceCount">引用计数</option>
                            <option value="GroupId">组ID</option>
                            <option value="Description">描述</option>
                        </select>
                    </div>
                    <div class="form-group" style="margin-bottom: 0;">
                        <label for="visual-search-operator">操作符:</label>
                        <select id="visual-search-operator">
                            <option value="equal">等于</option>
                            <option value="like">包含</option>
                            <option value="greater">大于</option>
                            <option value="less">小于</option>
                            <option value="in">在集合中(逗号分隔)</option>
                        </select>
                    </div>
                    <div class="form-group" style="margin-bottom: 0; flex: 1; min-width: 150px;">
                        <label for="visual-search-value">值:</label>
                        <input type="text" id="visual-search-value" placeholder="输入查询值...">
                    </div>
                    <div class="form-group" style="margin-bottom: 0;">
                        <button type="button" class="btn-primary" onclick="addVisualSearchCondition()">添加</button>
                    </div>
                </div>
                
                <div id="visual-search-conditions" class="visual-search-container">
                    <!-- 条件标签将在这里显示 -->
                </div>
                
                <div style="margin-top: 15px; display: flex; justify-content: flex-end; gap: 10px;">
                    <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
                    <button type="button" class="btn-primary" onclick="performVisualSearch()">开始查询</button>
                </div>
            </form>
        </div>
        <div id="json-search" class="tab-content" style="display: none;">
            <form id="json-file-search-form">
                <div class="form-group">
                    <label for="complex-search-file-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-file-conditions" rows="5" placeholder='[{"Id": 1}, {"Type_": "example"}]'></textarea>
                </div>
                <div style="display: flex; justify-content: flex-end; gap: 10px;">
                    <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
                    <button type="submit" class="btn-primary">查询</button>
                </div>
            </form>
        </div>
    `;

    // 初始化可视化查询状态：如果有上一次的搜索内容且目标一致，则恢复
    const currentTarget = getActivePageTarget();
    if (lastSearchState.active && lastSearchState.target === currentTarget) {
        visualSearchConditions = JSON.parse(JSON.stringify(lastSearchState.conditions));
        currentSearchLogic = lastSearchState.logic;
        renderVisualSearchConditions();
    } else {
        clearVisualConditions();
    }

    // 绑定表单提交事件
    document.getElementById('json-file-search-form').addEventListener('submit', function (e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-file-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入查询条件', 'warning');
            return;
        }

        try {
            const conditions = JSON.parse(conditionsJson);
            
            // 同步到可视化状态以便持久化
            syncJsonToVisual();
            lastSearchState.conditions = JSON.parse(JSON.stringify(visualSearchConditions));
            lastSearchState.logic = currentSearchLogic;
            lastSearchState.active = true;
            lastSearchState.target = getActivePageTarget();
            renderActiveSearchConditions();

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

    // 标记使用conditions查询
    currentFileQueryType = 'conditions';

    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });

    const url = `${BASE_URL}/api/files/search/by-conditions-with-pagination?${params.toString()}`;

    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                closeModal();
                renderFileTable(result.data.data || []);
                // 更新分页信息
                totalFilePages = result.data.total_pages || 1;
                renderFilePagination(result.data);
            } else {
                showMessage('文件查询失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('文件查询失败: ' + error.message, 'error');
        });
}

// 显示文件详细信息
function showFileInfo(fileId) {
    const url = `${BASE_URL}/api/files/${fileId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                const file = result.data;
                const modalBody = document.getElementById('modal-body');
                modalBody.innerHTML = `
                    <h2>文件详细信息</h2>
                    <div class="file-details">
                        <div class="form-group">
                            <label><strong>ID:</strong></label>
                            <span>${file.id}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>类型:</strong></label>
                            <span>${file.type_}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>路径:</strong></label>
                            <span>${file.path}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>描述:</strong></label>
                            <span>${file.description || '无'}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>组ID:</strong></label>
                            <span>${file.group_id}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>引用计数:</strong></label>
                            <span>${file.reference_count}</span>
                        </div>
                    </div>
                    <button type="button" class="btn-secondary" onclick="closeModal()">关闭</button>
                `;
                document.getElementById('modal').style.display = 'block';
            } else {
                showMessage('获取文件信息失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取文件信息失败: ' + error.message, 'error');
        });
}

// 创建悬浮窗
function createTooltip(element, content) {
    // 移除已存在的悬浮窗
    removeTooltip();
    
    // 创建新的悬浮窗
    const tooltip = document.createElement('div');
    tooltip.className = 'tooltip';
    tooltip.id = 'file-tooltip';
    tooltip.innerHTML = content;
    
    // 添加到文档中
    document.body.appendChild(tooltip);
    
    // 定位悬浮窗
    const rect = element.getBoundingClientRect();
    tooltip.style.left = rect.left + (rect.width / 2) - (tooltip.offsetWidth / 2) + 'px';
    tooltip.style.top = (rect.top - tooltip.offsetHeight - 10) + 'px';
    
    // 确保悬浮窗不会超出视窗边界
    const tooltipRect = tooltip.getBoundingClientRect();
    if (tooltipRect.left < 0) {
        tooltip.style.left = '10px';
    } else if (tooltipRect.right > window.innerWidth) {
        tooltip.style.left = (window.innerWidth - tooltip.offsetWidth - 10) + 'px';
    }
    
    return tooltip;
}

// 移除悬浮窗
function removeTooltip() {
    const existingTooltip = document.getElementById('file-tooltip');
    if (existingTooltip) {
        existingTooltip.remove();
    }
}

// 为信息图标添加悬浮事件
function addInfoIconHover() {
    document.addEventListener('mouseover', function(e) {
        if (e.target.classList.contains('info-icon')) {
            const fileId = e.target.getAttribute('data-file-id');
            if (fileId) {
                // 获取文件详细信息并显示悬浮窗
                const url = `${BASE_URL}/api/files/${fileId}`;
                fetch(url)
                    .then(response => response.json())
                    .then(data => {
                        const result = handleApiResponse(data);
                        if (result.success) {
                            const file = result.data;
                            const content = `
                                <ul class="tooltip-content">
                                    <li><span class="label">ID:</span> <span class="value">${file.id}</span></li>
                                    <li><span class="label">类型:</span> <span class="value">${file.type_}</span></li>
                                    <li><span class="label">路径:</span> <span class="value">${file.path}</span></li>
                                    <li><span class="label">描述:</span> <span class="value">${file.description || '无'}</span></li>
                                    <li><span class="label">组ID:</span> <span class="value">${file.group_id}</span></li>
                                    <li><span class="label">引用计数:</span> <span class="value">${file.reference_count}</span></li>
                                </ul>
                            `;
                            createTooltip(e.target, content);
                        }
                    })
                    .catch(error => {
                        console.error('Error:', error);
                    });
            }
        }
    });
    
    document.addEventListener('mouseout', function(e) {
        if (e.target.classList.contains('info-icon')) {
            // 延迟移除悬浮窗，避免鼠标移动到悬浮窗时立即消失
            setTimeout(() => {
                removeTooltip();
            }, 100);
        }
    });
}

// 页面加载完成后初始化信息图标悬浮事件
document.addEventListener('DOMContentLoaded', function() {
    addInfoIconHover();
});

// 显示文件悬浮窗
function showFileTooltip(event) {
    // 移除已存在的悬浮窗
    hideFileTooltip();
    
    // 获取文件数据
    const fileData = JSON.parse(event.target.getAttribute('data-file').replace(/&quot;/g, '"'));
    
    // 创建悬浮窗
    const tooltip = document.createElement('div');
    tooltip.id = 'file-tooltip';
    tooltip.className = 'tooltip';
    
    // 构建悬浮窗内容
    tooltip.innerHTML = `
        <ul class="tooltip-content">
            <li><span class="label">ID:</span> <span class="value">${fileData.id}</span></li>
            <li><span class="label">类型:</span> <span class="value">${fileData.type_}</span></li>
            <li><span class="label">路径:</span> <span class="value">${fileData.path}</span></li>
            <li><span class="label">描述:</span> <span class="value">${fileData.description || '无'}</span></li>
            <li><span class="label">组ID:</span> <span class="value">${fileData.group_id}</span></li>
            <li><span class="label">引用计数:</span> <span class="value">${fileData.reference_count}</span></li>
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

// 隐藏文件悬浮窗
function hideFileTooltip() {
    const existingTooltip = document.getElementById('file-tooltip');
    if (existingTooltip) {
        existingTooltip.remove();
    }
}

// 根据组ID获取文件列表
function listFilesByGroupId() {
    const groupId = getInputValue('file-group-id');
    if (!groupId) {
        showMessage('请输入组ID', 'warning');
        return;
    }

    const url = `${BASE_URL}/api/files/group/${groupId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                renderFileTable(result.data || []);
            } else {
                showMessage('获取文件失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取文件失败: ' + error.message, 'error');
        });
}