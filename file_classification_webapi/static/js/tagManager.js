// 标签管理相关函数

// 添加分页相关变量
let currentTagPage = 1;
let tagPageSize = 10;
let totalTagPages = 1;
let currentTagConditions = null;
let currentTagQueryType = null; // 'filter' or 'conditions'

function listTagsByFilter() {
    const tagId = getInputValue('tag-id');
    const tagName = getInputValue('tag-name');

    // 构造查询参数
    let params = new URLSearchParams();
    if (tagId) params.append('id', tagId);
    if (tagName) params.append('name', tagName);

    // 构造分页参数
    const options = {
        page: currentTagPage,
        page_size: tagPageSize
    };

    // 保存当前条件
    currentTagConditions = {};
    if (tagId) currentTagConditions.id = parseInt(tagId);
    if (tagName) currentTagConditions.name = tagName;

    // 标记使用filter查询
    currentTagQueryType = 'filter';

    // 构造查询参数
    const searchParams = new URLSearchParams({
        filter: JSON.stringify(currentTagConditions),
        options: JSON.stringify(options)
    });

    const url = `${BASE_URL}/api/tags/search/by-filter-with-pagination?${searchParams.toString()}`;

    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success && result.data) {
                renderTagTable(result.data.data || []);
                // 更新分页信息
                totalTagPages = result.data.total_pages || 1;
                renderTagPagination(result.data);
            } else {
                renderTagTable([]);
                renderTagPagination({page: 1, total_pages: 1, total: 0});
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('查询标签失败: ' + error.message, 'error');
            renderTagTable([]);
            renderTagPagination({page: 1, total_pages: 1, total: 0});
        });
}

function renderTagTable(tags) {
    const tableBody = document.querySelector('#tags-table tbody');
    tableBody.innerHTML = '';

    tags.forEach(tag => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><input type="checkbox" class="tag-checkbox" data-id="${tag.id}"></td>
            <td>${tag.id}</td>
            <td>${tag.name}</td>
            <td>${tag.description === null ? '无' : (tag.description || '')}</td>
            <td>
                <span class="info-icon" data-tag='${JSON.stringify(tag).replace(/"/g, '&quot;')}' onmouseover="showTagTooltip(event)" onmouseout="hideTagTooltip()" title="悬停查看详细信息">ℹ️</span>
                <button class="action-button info" onclick="showTagInfo(${tag.id})" title="查看详细信息">详情</button>
                <button class="action-button edit" onclick="openEditTagDialog(${tag.id})">修改</button>
                <button class="action-button delete" onclick="deleteTag(${tag.id})">删除</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

// 渲染分页控件
function renderTagPagination(paginationData) {
    const paginationContainer = document.getElementById('tags-pagination');
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
                <button onclick="changeTagPage(1)" ${currentPage <= 1 ? 'disabled' : ''}>首页</button>
                <button onclick="changeTagPage(${currentPage - 1})" ${currentPage <= 1 ? 'disabled' : ''}>上一页</button>
                <span class="page-numbers">
    `;

    // 显示页码
    let startPage = Math.max(1, currentPage - 2);
    let endPage = Math.min(totalPages, currentPage + 2);

    if (startPage > 1) {
        paginationHTML += `<button onclick="changeTagPage(1)">1</button>`;
        if (startPage > 2) paginationHTML += `<span>...</span>`;
    }

    for (let i = startPage; i <= endPage; i++) {
        if (i === currentPage) {
            paginationHTML += `<button class="active">${i}</button>`;
        } else {
            paginationHTML += `<button onclick="changeTagPage(${i})">${i}</button>`;
        }
    }

    if (endPage < totalPages) {
        if (endPage < totalPages - 1) paginationHTML += `<span>...</span>`;
        paginationHTML += `<button onclick="changeTagPage(${totalPages})">${totalPages}</button>`;
    }

    paginationHTML += `
                </span>
                <button onclick="changeTagPage(${currentPage + 1})" ${currentPage >= totalPages ? 'disabled' : ''}>下一页</button>
                <button onclick="changeTagPage(${totalPages})" ${currentPage >= totalPages ? 'disabled' : ''}>末页</button>
            </div>
            <div class="pagination-size">
                每页显示: 
                <select onchange="changeTagPageSize(this.value)">
                    <option value="10" ${tagPageSize === 10 ? 'selected' : ''}>10</option>
                    <option value="20" ${tagPageSize === 20 ? 'selected' : ''}>20</option>
                    <option value="50" ${tagPageSize === 50 ? 'selected' : ''}>50</option>
                    <option value="100" ${tagPageSize === 100 ? 'selected' : ''}>100</option>
                </select>
            </div>
        </div>
    `;

    paginationContainer.innerHTML = paginationHTML;
}

// 改变页码
function changeTagPage(page) {
    if (page < 1 || page > totalTagPages) return;
    currentTagPage = page;
    // 根据查询类型选择接口
    if (currentTagQueryType === 'conditions') {
        searchTagsByConditions(currentTagConditions);
    } else {
        listTagsByFilter();
    }
}

// 改变每页大小
function changeTagPageSize(size) {
    tagPageSize = parseInt(size);
    currentTagPage = 1; // 重置到第一页
    // 根据查询类型选择接口
    if (currentTagQueryType === 'conditions') {
        searchTagsByConditions(currentTagConditions);
    } else {
        listTagsByFilter();
    }
}

function createTag() {
    const tagName = getInputValue('create-tag-name');
    const tagDescription = getInputValue('create-tag-description');

    if (!tagName) {
        showMessage('请输入标签名', 'warning');
        return;
    }

    const tagData = {
        name: tagName,
        description: tagDescription || ''
    };

    const url = `${BASE_URL}/api/tags`;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(tagData)
    })
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('标签创建成功', 'success');
                closeModal();
                // 重新加载标签列表
                currentTagPage = 1;
                listTagsByFilter();
            } else {
                showMessage('标签创建失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('标签创建失败: ' + error.message, 'error');
        });
}

function updateTag() {
    const tagId = getInputValue('edit-tag-id');
    const tagName = getInputValue('edit-tag-name');
    const tagDescription = getInputValue('edit-tag-description');

    if (!tagId || !tagName) {
        showMessage('请填写完整的标签信息', 'warning');
        return;
    }

    const updateData = {
        name: tagName,
        description: tagDescription || ''
    };

    const url = `${BASE_URL}/api/tags/${tagId}`;
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
                showMessage('标签更新成功', 'success');
                closeModal();
                // 重新加载标签列表
                listTagsByFilter();
            } else {
                showMessage('标签更新失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('标签更新失败: ' + error.message, 'error');
        });
}

function deleteTag(tagId) {
    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', '确定要删除该标签吗？', function (result) {
        if (result) {
            const url = `${BASE_URL}/api/tags/${tagId}`;
            fetch(url, {
                method: 'DELETE'
            })
                .then(response => response.json())
                .then(data => {
                    const result = handleApiResponse(data);
                    if (result.success) {
                        showMessage('标签删除成功', 'success');
                        // 重新加载标签列表
                        listTagsByFilter();
                    } else {
                        showMessage('标签删除失败: ' + (result.data?.message || '未知错误'), 'error');
                    }
                })
                .catch(error => {
                    console.error('Error:', error);
                    showMessage('标签删除失败: ' + error.message, 'error');
                });
        }
    });
}

// 批量删除选中的标签
function deleteSelectedTags() {
    const selectedCheckboxes = document.querySelectorAll('.tag-checkbox:checked');
    if (selectedCheckboxes.length === 0) {
        showMessage('请至少选择一个标签进行删除', 'warning');
        return;
    }

    // 使用页面弹窗替换原生confirm
    showConfirmDialog('确认删除', `确定要删除这 ${selectedCheckboxes.length} 个标签吗？`, function (result) {
        if (result) {
            const ids = Array.from(selectedCheckboxes).map(cb => parseInt(cb.getAttribute('data-id')));

            // 使用新的delete by ids接口
            deleteTagsByIds(ids);
        }
    });
}

// 打开创建标签对话框
function openCreateTagDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>新增标签</h2>
        <form id="create-tag-form">
            <div class="form-group">
                <label for="create-tag-name">标签名:</label>
                <input type="text" id="create-tag-name" required>
            </div>
            <div class="form-group">
                <label for="create-tag-description">描述:</label>
                <input type="text" id="create-tag-description">
            </div>
            <button type="submit" class="btn-primary">创建</button>
            <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
        </form>
    `;

    // 绑定表单提交事件
    document.getElementById('create-tag-form').addEventListener('submit', function (e) {
        e.preventDefault();
        createTag();
    });

    document.getElementById('modal').style.display = 'block';
}

// 打开编辑标签对话框
function openEditTagDialog(tagId) {
    // 首先获取标签信息
    const url = `${BASE_URL}/api/tags/${tagId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                const tag = result.data;
                const modalBody = document.getElementById('modal-body');
                modalBody.innerHTML = `
                    <h2>编辑标签</h2>
                    <form id="edit-tag-form">
                        <div class="form-group">
                            <label for="edit-tag-id">标签ID:</label>
                            <input type="number" id="edit-tag-id" value="${tag.id}" readonly>
                        </div>
                        <div class="form-group">
                            <label for="edit-tag-name">标签名:</label>
                            <input type="text" id="edit-tag-name" value="${tag.name}" required>
                        </div>
                        <div class="form-group">
                            <label for="edit-tag-description">描述:</label>
                            <input type="text" id="edit-tag-description" value="${tag.description || ''}">
                        </div>
                        <button type="submit" class="btn-primary">更新</button>
                        <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
                    </form>
                `;

                // 绑定表单提交事件
                document.getElementById('edit-tag-form').addEventListener('submit', function (e) {
                    e.preventDefault();
                    updateTag();
                });

                document.getElementById('modal').style.display = 'block';
            } else {
                showMessage('获取标签信息失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取标签信息失败: ' + error.message, 'error');
        });
}

// 打开批量删除标签对话框
function openBatchDeleteTagDialog() {
    const modalBody = document.getElementById('modal-body');

    // 获取当前选中的标签ID
    const selectedTagCheckboxes = document.querySelectorAll('.tag-checkbox:checked');
    const selectedTagIds = Array.from(selectedTagCheckboxes).map(cb => parseInt(cb.value)).map(id => parseInt(id));

    let formContent;
    if (selectedTagIds.length > 0) {
        formContent = `
            <h2>批量删除标签</h2>
            <p>已选择 ${selectedTagIds.length} 个标签</p>
            <form id="batch-delete-tag-form">
                <input type="hidden" id="selected-tag-ids" value='${JSON.stringify(selectedTagIds)}'>
                <button type="submit">删除选中标签</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        `;
    } else {
        formContent = `
            <h2>批量删除标签</h2>
            <form id="batch-delete-tag-form">
                <div class="form-group">
                    <label for="batch-delete-tag-conditions">删除条件 (JSON格式):</label>
                    <textarea id="batch-delete-tag-conditions" rows="5" placeholder='[{"Id": 1}, {"Name": "example"}]'></textarea>
                </div>
                <button type="submit">删除</button>
                <button type="button" class="btn-secondary" onclick="closeModal()">取消</button>
            </form>
        `;
    }

    modalBody.innerHTML = formContent;

    // 绑定表单提交事件
    document.getElementById('batch-delete-tag-form').addEventListener('submit', function (e) {
        e.preventDefault();

        // 如果有选中的标签ID，使用delete by ids
        const selectedIdsInput = document.getElementById('selected-tag-ids');
        if (selectedIdsInput) {
            const tagIds = JSON.parse(selectedIdsInput.value);
            // 确保数值字段是数字类型
            const fixedTagIds = tagIds.map(id => parseInt(id));
            deleteTagsByIds(fixedTagIds);
            return;
        }

        // 否则使用条件删除（向后兼容）
        const conditionsJson = document.getElementById('batch-delete-tag-conditions').value;
        if (!conditionsJson) {
            showMessage('请输入删除条件', 'warning');
            return;
        }

        try {
            const conditions = JSON.parse(conditionsJson);
            deleteTagsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });

    document.getElementById('modal').style.display = 'block';
}

// 批量删除标签（根据ID列表）
function deleteTagsByIds(tagIds) {
    const url = `${BASE_URL}/api/tags/delete/by-ids`;
    fetch(url, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(tagIds)
    })
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                showMessage('标签批量删除成功', 'success');
                closeModal();
                // 重新加载标签列表
                listTagsByFilter();
            } else {
                showMessage('标签批量删除失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('标签批量删除失败: ' + error.message, 'error');
        });
}

function deleteTagsByConditions(conditions) {
    const url = `${BASE_URL}/api/tags/delete/by-conditions`;
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
                showMessage('标签批量删除成功', 'success');
                closeModal();
                // 重新加载标签列表
                listTagsByFilter();
            } else {
                showMessage('标签批量删除失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('标签批量删除失败: ' + error.message, 'error');
        });
}

// 打开复杂查询标签对话框
function openComplexSearchTagDialog() {
    const modalBody = document.getElementById('modal-body');
    modalBody.innerHTML = `
        <h2>复杂查询标签</h2>
        <div class="tabs">
            <button class="tab-button active" onclick="switchComplexSearchTab('visual')">可视化查询</button>
            <button class="tab-button" onclick="switchComplexSearchTab('json')">JSON查询</button>
        </div>
        <div id="visual-search" class="tab-content active">
            <form id="visual-tag-search-form">
                <div class="form-group-container" style="display: flex; gap: 10px; align-items: flex-end; flex-wrap: wrap;">
                    <div class="form-group" style="margin-bottom: 0;">
                        <label for="visual-search-field">查询字段:</label>
                        <select id="visual-search-field">
                            <option value="Id">ID</option>
                            <option value="Name">名称</option>
                            <option value="Description">描述</option>
                            <option value="ReferenceCount">引用计数</option>
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
            <form id="json-tag-search-form">
                <div class="form-group">
                    <label for="complex-search-tag-conditions">查询条件 (JSON格式):</label>
                    <textarea id="complex-search-tag-conditions" rows="5" placeholder='[{"Id": 1}, {"Name": "example"}]'></textarea>
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
    document.getElementById('json-tag-search-form').addEventListener('submit', function (e) {
        e.preventDefault();
        const conditionsJson = document.getElementById('complex-search-tag-conditions').value;
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

            searchTagsByConditions(conditions);
        } catch (e) {
            showMessage('JSON格式错误: ' + e.message, 'error');
        }
    });

    document.getElementById('modal').style.display = 'block';
}


function searchTagsByConditions(conditions) {
    // 构造查询选项
    const options = {
        page: currentTagPage,
        page_size: tagPageSize
    };

    // 保存当前条件
    currentTagConditions = conditions;

    // 标记使用conditions查询
    currentTagQueryType = 'conditions';

    // 构造查询参数
    const params = new URLSearchParams({
        conditions: JSON.stringify(conditions),
        options: JSON.stringify(options)
    });

    const url = `${BASE_URL}/api/tags/search/by-conditions-with-pagination?${params.toString()}`;

    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                closeModal();
                renderTagTable(result.data.data || []);
                // 更新分页信息
                totalTagPages = result.data.total_pages || 1;
                renderTagPagination(result.data);
            } else {
                showMessage('标签查询失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('标签查询失败: ' + error.message, 'error');
        });
}

// 显示标签悬浮窗
function showTagTooltip(event) {
    // 移除已存在的悬浮窗
    hideTagTooltip();
    
    // 获取标签数据
    const tagData = JSON.parse(event.target.getAttribute('data-tag').replace(/&quot;/g, '"'));
    
    // 创建悬浮窗
    const tooltip = document.createElement('div');
    tooltip.id = 'tag-tooltip';
    tooltip.className = 'tooltip';
    
    // 构建悬浮窗内容
    tooltip.innerHTML = `
        <ul class="tooltip-content">
            <li><span class="label">ID:</span> <span class="value">${tagData.id}</span></li>
            <li><span class="label">名称:</span> <span class="value">${tagData.name}</span></li>
            <li><span class="label">描述:</span> <span class="value">${tagData.description || '无'}</span></li>
            <li><span class="label">引用计数:</span> <span class="value">${tagData.reference_count}</span></li>
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

// 隐藏标签悬浮窗
function hideTagTooltip() {
    const existingTooltip = document.getElementById('tag-tooltip');
    if (existingTooltip) {
        existingTooltip.remove();
    }
}

// 显示标签详细信息
function showTagInfo(tagId) {
    const url = `${BASE_URL}/api/tags/${tagId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                const tag = result.data;
                const modalBody = document.getElementById('modal-body');
                modalBody.innerHTML = `
                    <h2>标签详细信息</h2>
                    <div class="tag-details">
                        <div class="form-group">
                            <label><strong>ID:</strong></label>
                            <span>${tag.id}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>名称:</strong></label>
                            <span>${tag.name}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>描述:</strong></label>
                            <span>${tag.description || '无'}</span>
                        </div>
                        <div class="form-group">
                            <label><strong>引用计数:</strong></label>
                            <span>${tag.reference_count}</span>
                        </div>
                    </div>
                    <button type="button" class="btn-secondary" onclick="closeModal()">关闭</button>
                `;
                document.getElementById('modal').style.display = 'block';
            } else {
                showMessage('获取标签信息失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取标签信息失败: ' + error.message, 'error');
        });
}

// 根据组ID获取标签列表
function listTagsByGroupId() {
    const groupId = getInputValue('tag-group-id');
    if (!groupId) {
        showMessage('请输入组ID', 'warning');
        return;
    }

    const url = `${BASE_URL}/api/tags/group/${groupId}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            const result = handleApiResponse(data);
            if (result.success) {
                renderTagTable(result.data || []);
            } else {
                showMessage('获取标签失败: ' + (result.data?.message || '未知错误'), 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showMessage('获取标签失败: ' + error.message, 'error');
        });
}