// 工具函数

function displayResult(elementId, data) {
    const resultElement = document.getElementById(elementId);
    resultElement.innerText = JSON.stringify(data, null, 2);
}

// 工具函数：获取输入值
function getInputValue(id) {
    return document.getElementById(id).value;
}

// 工具函数：设置输入值
function setInputValue(id, value) {
    document.getElementById(id).value = value;
}

// 工具函数：获取文本域值
function getTextValue(id) {
    return document.getElementById(id).value;
}

// 工具函数：设置文本域值
function setTextValue(id, value) {
    document.getElementById(id).value = value;
}

// 显示通知消息
function showMessage(message, type = 'success') {
    // 创建通知元素
    const messageElement = document.createElement('div');
    messageElement.className = `notification ${type}`;
    messageElement.innerText = message;
    
    // 添加样式
    messageElement.style.position = 'fixed';
    messageElement.style.top = '20px';
    messageElement.style.right = '20px';
    messageElement.style.padding = '15px 20px';
    messageElement.style.borderRadius = '4px';
    messageElement.style.color = 'white';
    messageElement.style.fontWeight = 'bold';
    messageElement.style.zIndex = '10000';
    messageElement.style.boxShadow = '0 2px 10px rgba(0,0,0,0.2)';
    
    // 根据类型设置背景色
    if (type === 'success') {
        messageElement.style.backgroundColor = '#28a745';
    } else if (type === 'error') {
        messageElement.style.backgroundColor = '#dc3545';
    } else if (type === 'warning') {
        messageElement.style.backgroundColor = '#ffc107';
        messageElement.style.color = '#212529';
    } else {
        messageElement.style.backgroundColor = '#17a2b8';
    }
    
    // 添加到页面
    document.body.appendChild(messageElement);
    
    // 3秒后自动移除
    setTimeout(() => {
        if (messageElement.parentNode) {
            messageElement.parentNode.removeChild(messageElement);
        }
    }, 3000);
}

// 关闭模态框
function closeModal() {
    document.getElementById('modal').style.display = 'none';
}

// 全选/取消全选文件
function toggleAllFiles(source) {
    const checkboxes = document.querySelectorAll('.file-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// 全选/取消全选组
function toggleAllGroups(source) {
    const checkboxes = document.querySelectorAll('.group-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// 全选/取消全选标签
function toggleAllTags(source) {
    const checkboxes = document.querySelectorAll('.tag-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// 全选/取消全选文件组关联
function toggleAllFileGroups(source) {
    const checkboxes = document.querySelectorAll('.file-group-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// 全选/取消全选组标签关联
function toggleAllGroupTags(source) {
    const checkboxes = document.querySelectorAll('.group-tag-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// 全选/取消全选组关系
function toggleAllGroupRelations(source) {
    const checkboxes = document.querySelectorAll('.group-relation-checkbox');
    for (let i = 0; i < checkboxes.length; i++) {
        checkboxes[i].checked = source.checked;
    }
}

// 重置文件搜索表单
function resetFileFilter() {
    document.getElementById('file-id').value = '';
    document.getElementById('file-type').value = '';
    document.getElementById('file-path').value = '';
    listFilesByFilter(); // 重置后重新搜索
}

// 重置组搜索表单
function resetGroupFilter() {
    document.getElementById('group-id').value = '';
    document.getElementById('group-name').value = '';
    listGroupsByFilter(); // 重置后重新搜索
}

// 重置标签搜索表单
function resetTagFilter() {
    document.getElementById('tag-id').value = '';
    document.getElementById('tag-name').value = '';
    listTagsByFilter(); // 重置后重新搜索
}

// 重置文件组关联搜索表单
function resetFileGroupFilter() {
    document.getElementById('file-group-file-id').value = '';
    document.getElementById('file-group-group-id').value = '';
    listFileGroupsByFilter(); // 重置后重新搜索
}

// 重置组标签关联搜索表单
function resetGroupTagFilter() {
    document.getElementById('group-tag-group-id').value = '';
    document.getElementById('group-tag-tag-id').value = '';
    listGroupTagsByFilter(); // 重置后重新搜索
}

// 重置组关系搜索表单
function resetGroupRelationFilter() {
    document.getElementById('group-relation-first-id').value = '';
    document.getElementById('group-relation-second-id').value = '';
    document.getElementById('group-relation-type').value = '';
    listGroupRelationsByFilter(); // 重置后重新搜索
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
        <input type="hidden" class="condition-data" value='{"${field}": "${value}"}'>
    `;
    conditionsContainer.appendChild(conditionElement);
    
    // 清空输入
    document.getElementById('visual-search-value').value = '';
}

// 执行可视化查询
function performVisualSearch() {
    const conditionElements = document.querySelectorAll('.condition-data');
    const conditions = [];
    
    conditionElements.forEach(element => {
        try {
            const condition = JSON.parse(element.value);
            conditions.push(condition);
        } catch (e) {
            console.error('Error parsing condition:', e);
        }
    });
    
    if (conditions.length > 0) {
        searchFilesByConditions(conditions);
    } else {
        showMessage('请添加至少一个查询条件', 'warning');
    }
}