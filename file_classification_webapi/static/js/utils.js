// 工具函数

// 获取输入框的值
function getInputValue(elementId) {
    const element = document.getElementById(elementId);
    return element ? element.value.trim() : '';
}

// 显示消息
function showMessage(message, type = 'info') {
    const messageContainer = document.getElementById('message-container');
    if (messageContainer) {
        messageContainer.innerHTML = `
            <div class="message message-${type}">
                ${message}
                <button class="close-button" onclick="this.parentElement.style.display='none'">&times;</button>
            </div>
        `;
        messageContainer.style.display = 'block';
        
        // 3秒后自动隐藏消息
        setTimeout(() => {
            messageContainer.style.display = 'none';
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