// 工具函数

// 获取输入框的值
function getInputValue(elementId) {
    const element = document.getElementById(elementId);
    return element ? element.value.trim() : '';
}

// 显示消息 - 支持多个实例
function showMessage(message, type = 'info') {
    const messageContainer = document.getElementById('message-container');
    if (messageContainer) {
        // 根据消息类型设置样式
        let messageTypeClass = 'message-info';
        if (type === 'success') messageTypeClass = 'message-success';
        if (type === 'error') messageTypeClass = 'message-error';
        if (type === 'warning') messageTypeClass = 'message-warning';

        // 创建消息元素
        const messageElement = document.createElement('div');
        messageElement.className = 'message';
        messageElement.innerHTML = `
            <div class="message ${messageTypeClass}">
                ${message}
                <button class="close-button" onclick="this.parentElement.parentElement.remove()">&times;</button>
            </div>
        `;

        // 添加到消息容器
        messageContainer.appendChild(messageElement);

        // 3秒后自动隐藏消息
        setTimeout(() => {
            if (messageElement.parentElement) {
                messageElement.remove();
            }
        }, 3000);
    }
}

// 打开模态框
function openModal() {
    const modal = document.getElementById('modal');
    if (modal) {
        modal.style.display = 'block';
    }
}

// 关闭模态框
function closeModal() {
    const modal = document.getElementById('modal');
    if (modal) {
        modal.style.display = 'none';
    }
}

// 重置文件过滤器
function resetFileFilter() {
    document.getElementById('file-id').value = '';
    document.getElementById('file-type').value = '';
    document.getElementById('file-path').value = '';
    // 重置分页参数
    currentFilePage = 1;
    listFilesByFilter(); // 重置后重新搜索
}

// 重置组过滤器
function resetGroupFilter() {
    document.getElementById('group-id').value = '';
    document.getElementById('group-name').value = '';
    // 重置分页参数
    currentGroupPage = 1;
    listGroupsByFilter(); // 重置后重新搜索
}

// 重置标签过滤器
function resetTagFilter() {
    document.getElementById('tag-id').value = '';
    document.getElementById('tag-name').value = '';
    // 重置分页参数
    currentTagPage = 1;
    listTagsByFilter(); // 重置后重新搜索
}

// 重置文件组过滤器
function resetFileGroupFilter() {
    document.getElementById('file-group-file-id').value = '';
    document.getElementById('file-group-group-id').value = '';
    // 重置分页参数
    currentFileGroupPage = 1;
    listFileGroupsByFilter(); // 重置后重新搜索
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

// 重置组标签过滤器
function resetGroupTagFilter() {
    document.getElementById('group-tag-group-id').value = '';
    document.getElementById('group-tag-tag-id').value = '';
    // 重置分页参数
    currentGroupTagPage = 1;
    listGroupTagsByFilter(); // 重置后重新搜索
}

// 切换全选文件
function toggleAllFiles(source) {
    const checkboxes = document.querySelectorAll('.file-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.checked = source.checked;
    });
}

// 切换全选组
function toggleAllGroups(source) {
    const checkboxes = document.querySelectorAll('.group-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.checked = source.checked;
    });
}

// 切换全选标签
function toggleAllTags(source) {
    const checkboxes = document.querySelectorAll('.tag-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.checked = source.checked;
    });
}

// 切换全选文件组
function toggleAllFileGroups(source) {
    const checkboxes = document.querySelectorAll('.file-group-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.checked = source.checked;
    });
}

// 切换全选组关系
function toggleAllGroupRelations(source) {
    const checkboxes = document.querySelectorAll('.group-relation-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.checked = source.checked;
    });
}

// 切换全选组标签
function toggleAllGroupTags(source) {
    const checkboxes = document.querySelectorAll('.group-tag-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.checked = source.checked;
    });
}

// 切换复杂查询标签
function switchComplexSearchTab(tab) {
    // 更新标签按钮状态
    document.querySelectorAll('.tab-button').forEach(button => {
        button.classList.remove('active');
    });
    event.target.classList.add('active');

    // 显示对应的标签内容
    if (tab === 'visual') {
        document.getElementById('visual-search').style.display = 'block';
        document.getElementById('json-search').style.display = 'none';
    } else {
        document.getElementById('visual-search').style.display = 'none';
        document.getElementById('json-search').style.display = 'block';
    }
}

// 添加可视化查询条件
function addVisualSearchCondition() {
    const field = document.getElementById('visual-search-field').value;
    const operator = document.getElementById('visual-search-operator').value;
    const value = document.getElementById('visual-search-value').value;

    if (!value) {
        showMessage('请输入值', 'warning');
        return;
    }

    const conditionsContainer = document.getElementById('visual-search-conditions');
    const conditionElement = document.createElement('div');
    conditionElement.className = 'condition-item';
    conditionElement.innerHTML = `
        <span>${field} ${operator} ${value}</span>
        <button type="button" onclick="this.parentElement.remove()">删除</button>
        <input type="hidden" class="condition-field" value="${field}">
        <input type="hidden" class="condition-operator" value="${operator}">
        <input type="hidden" class="condition-value" value="${value}">
    `;
    conditionsContainer.appendChild(conditionElement);
}

// 执行可视化搜索
function performVisualSearch() {
    // 这里应该根据添加的条件构造查询条件并执行搜索
    showMessage('可视化搜索功能待实现', 'info');
}

// 显示确认对话框
function showConfirmDialog(title, message, callback) {
    const modalBody = document.getElementById('modal-body');
    if (modalBody) {
        modalBody.innerHTML = `
            <h2>${title}</h2>
            <div class="confirm-dialog">
                <p>${message}</p>
                <div class="confirm-buttons">
                    <button type="button" class="btn-primary" onclick="handleConfirm(true)">确定</button>
                    <button type="button" class="btn-secondary" onclick="handleConfirm(false)">取消</button>
                </div>
            </div>
        `;

        // 保存回调函数
        window.confirmCallback = callback;

        // 打开模态框
        openModal();
    }
}

// 处理确认对话框的结果
function handleConfirm(result) {
    // 关闭模态框
    closeModal();

    // 执行回调函数
    if (window.confirmCallback && typeof window.confirmCallback === 'function') {
        window.confirmCallback(result);
    }

    // 清除回调函数
    window.confirmCallback = null;
}

// 处理API响应的通用函数
function handleApiResponse(response) {
    // 对于新的响应格式 (code/data/msg)
    if (response.code !== undefined) {
        if (response.code === "SUCCESS") {
            // 只有当msg存在且不为空时才显示消息
            if (response.msg && response.msg.trim() !== '') {
                showMessage(response.msg, 'success');
            }
            return { success: true, data: response.data };
        } else {
            // 显示错误消息
            showMessage(response.msg || '操作失败', 'error');
            return { success: false, data: response.data };
        }
    }
    // 对于旧的响应格式 (success/data/message)
    else if (response.success !== undefined) {
        if (response.success) {
            showMessage(response.message || '操作成功', 'success');
            return { success: true, data: response.data };
        } else {
            showMessage(response.message || '操作失败', 'error');
            return { success: false, data: response.data };
        }
    }
    // 默认处理
    else {
        if (response.error) {
            showMessage(response.error, 'error');
            return { success: false };
        } else {
            showMessage('操作成功', 'success');
            return { success: true, data: response };
        }
    }
}