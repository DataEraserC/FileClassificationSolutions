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

// 切换复杂查询标签页
function switchComplexSearchTab(tab) {
    const visualTab = document.getElementById('visual-search');
    const jsonTab = document.getElementById('json-search');
    const buttons = document.querySelectorAll('.tab-button');

    if (tab === 'visual') {
        // 从 JSON 同步到可视化
        syncJsonToVisual();
        
        visualTab.style.display = 'block';
        jsonTab.style.display = 'none';
        buttons[0].classList.add('active');
        buttons[1].classList.remove('active');
    } else {
        // 从可视化同步到 JSON
        syncVisualToJson();
        
        visualTab.style.display = 'none';
        jsonTab.style.display = 'block';
        buttons[0].classList.remove('active');
        buttons[1].classList.add('active');
    }
}

// 将可视化条件同步到 JSON 文本框
function syncVisualToJson() {
    const conditions = compileVisualSearchConditions();
    const jsonArea = document.querySelector('#json-search textarea');
    if (jsonArea) {
        jsonArea.value = JSON.stringify(conditions, null, 2);
    }
}

// 将 JSON 文本框内容同步到可视化条件
function syncJsonToVisual() {
    const jsonArea = document.querySelector('#json-search textarea');
    if (!jsonArea || !jsonArea.value.trim()) return;

    try {
        const conditions = JSON.parse(jsonArea.value);
        if (Array.isArray(conditions)) {
            // 解析逻辑：这里只支持一级嵌套的 And/Or，或者平铺的条件
            // 这是一个简化的解析器
            const newConditions = [];
            let mainLogic = 'And';

            // 检查是否是单对象包装的全局逻辑，如 [{"Or": [...]}]
            let effectiveConditions = conditions;
            if (conditions.length === 1 && (conditions[0].Or || conditions[0].And)) {
                mainLogic = conditions[0].Or ? 'Or' : 'And';
                effectiveConditions = conditions[0].Or || conditions[0].And;
            }

            effectiveConditions.forEach(item => {
                if (item.Or || item.And) {
                    const logic = item.Or ? 'Or' : 'And';
                    const subConditions = (item.Or || item.And).map(c => parseJsonCondition(c)).filter(c => c);
                    newConditions.push({
                        type: 'group',
                        logic: logic,
                        conditions: subConditions
                    });
                } else {
                    const parsed = parseJsonCondition(item);
                    if (parsed) {
                        newConditions.push({
                            type: 'condition',
                            ...parsed
                        });
                    }
                }
            });

            visualSearchConditions = newConditions;
            currentSearchLogic = mainLogic;
            renderVisualSearchConditions();
        }
    } catch (e) {
        console.error('Failed to sync JSON to Visual:', e);
    }
}

// 解析单个 JSON 条件对象回到可视化格式
function parseJsonCondition(obj) {
    // 寻找匹配的 key，如 NameLike, IdGreaterThan 等
    const key = Object.keys(obj)[0];
    if (!key) return null;

    const value = obj[key];
    
    // 提取字段和操作符
    let field = '';
    let operator = 'equal';

    if (key.endsWith('GreaterThan')) {
        field = key.replace('GreaterThan', '');
        operator = 'greater';
    } else if (key.endsWith('LessThan')) {
        field = key.replace('LessThan', '');
        operator = 'less';
    } else if (key.endsWith('Like')) {
        field = key.replace('Like', '');
        operator = 'like';
    } else if (key.endsWith('In')) {
        field = key.replace('In', '');
        operator = 'in';
    } else {
        field = key;
        operator = 'equal';
    }

    return { field, operator, value: Array.isArray(value) ? value.join(', ') : value };
}

// 可视化查询状态
let visualSearchConditions = [];
let currentSearchLogic = 'And';

// 缓存上一次搜索的条件，以便展示和重新加载
let lastSearchState = {
    conditions: [],
    logic: 'And',
    active: false,
    target: '' // 'files', 'groups', 'tags'
};

// 获取当前活动页面标识
function getActivePageTarget() {
    const activeNav = document.querySelector('.sidebar ul li a.active');
    const navText = activeNav ? activeNav.querySelector('.nav-text').textContent : '';
    if (navText.includes('文件')) return 'files';
    if (navText.includes('分组')) return 'groups';
    if (navText.includes('标签')) return 'tags';
    return '';
}

// 渲染搜索结果上方的已激活条件展示
function renderActiveSearchConditions() {
    const target = getActivePageTarget();
    const containerId = `${target}-active-search-container`;
    let container = document.getElementById(containerId);

    // 如果容器不存在且有活跃搜索，尝试在标题后创建
    if (!container) {
        const section = document.querySelector('section.active h1');
        if (section) {
            container = document.createElement('div');
            container.id = containerId;
            container.className = 'active-search-conditions-bar';
            section.after(container);
        }
    }

    if (!container) return;

    if (!lastSearchState.active || lastSearchState.target !== target || lastSearchState.conditions.length === 0) {
        container.style.display = 'none';
        container.innerHTML = '';
        return;
    }

    container.style.display = 'flex';
    container.innerHTML = `
        <span class="active-search-label">当前搜索条件 (${lastSearchState.logic}):</span>
        <div class="active-search-tags">
            ${lastSearchState.conditions.map((item, idx) => {
                if (item.type === 'condition') {
                    return `<span class="active-tag">${getFieldLabel(item.field)} ${getOperatorLabel(item.operator)} ${item.value}</span>`;
                } else {
                    return `<span class="active-tag-group">${item.logic}(${item.conditions.length})</span>`;
                }
            }).join('')}
        </div>
        <button class="btn-clear-search" onclick="clearActiveSearch()">清除搜索</button>
    `;
}

// 清除当前搜索状态
function clearActiveSearch() {
    lastSearchState.active = false;
    renderActiveSearchConditions();
    
    const target = getActivePageTarget();
    if (target === 'files') listFilesByFilter();
    else if (target === 'groups') listGroupsByFilter();
    else if (target === 'tags') listTagsByFilter();
}

// 清空可视化查询条件
function clearVisualConditions() {
    visualSearchConditions = [];
    currentSearchLogic = 'And';
    renderVisualSearchConditions();
}

// 更新主逻辑
function updateMainLogic(logic) {
    currentSearchLogic = logic;
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

    const condition = {
        field: field,
        operator: operator,
        value: value,
        type: 'condition'
    };

    visualSearchConditions.push(condition);
    renderVisualSearchConditions();
    
    // 清空输入框
    document.getElementById('visual-search-value').value = '';
}

// 添加条件组 (通常用于 OR)
function addVisualSearchGroup() {
    const group = {
        type: 'group',
        logic: 'Or',
        conditions: []
    };
    visualSearchConditions.push(group);
    renderVisualSearchConditions();
}

// 移除组
function removeVisualGroup(index) {
    visualSearchConditions.splice(index, 1);
    renderVisualSearchConditions();
}

// 更新组逻辑
function updateGroupLogic(index, logic) {
    if (visualSearchConditions[index] && visualSearchConditions[index].type === 'group') {
        visualSearchConditions[index].logic = logic;
    }
}

// 显示向组添加条件的对话框 (简化版：直接添加当前选中的条件)
function addConditionToGroup(groupIndex) {
    const field = document.getElementById('visual-search-field').value;
    const operator = document.getElementById('visual-search-operator').value;
    const value = document.getElementById('visual-search-value').value;

    if (!value) {
        showMessage('请输入值以添加到组', 'warning');
        return;
    }

    const condition = {
        field: field,
        operator: operator,
        value: value
    };

    visualSearchConditions[groupIndex].conditions.push(condition);
    renderVisualSearchConditions();
}

// 获取字段显示名称
function getFieldLabel(field) {
    const labels = {
        'Id': 'ID',
        'Type_': '类型',
        'Path': '路径',
        'ReferenceCount': '引用计数',
        'GroupId': '组ID',
        'Description': '描述',
        'Name': '名称',
        'IsPrimary': '主分组',
        'ClickCount': '点击量',
        'ShareCount': '分享量',
        'CreateTime': '创建时间',
        'ModifyTime': '修改时间'
    };
    return labels[field] || field;
}

// 获取操作符显示名称
function getOperatorLabel(operator) {
    const labels = {
        'equal': '=',
        'like': '包含',
        'greater': '>',
        'less': '<',
        'in': '在集合中'
    };
    return labels[operator] || operator;
}

// 渲染可视化查询条件
function renderVisualSearchConditions() {
    const container = document.getElementById('visual-search-conditions');
    if (!container) return;

    container.innerHTML = '';
    
    if (visualSearchConditions.length === 0) {
        container.innerHTML = '<p style="color: #888; text-align: center; margin: 20px 0;">尚未添加任何查询条件</p>';
        return;
    }

    const visualContainer = document.createElement('div');
    visualContainer.className = 'visual-search-container';

    // 顶部控制栏
    const controls = document.createElement('div');
    controls.style.marginBottom = '10px';
    controls.style.display = 'flex';
    controls.style.justifyContent = 'space-between';
    controls.style.alignItems = 'center';
    controls.innerHTML = `
        <span>
            全局逻辑: 
            <select class="logic-operator-select" onchange="updateMainLogic(this.value)">
                <option value="And" ${currentSearchLogic === 'And' ? 'selected' : ''}>AND (全部满足)</option>
                <option value="Or" ${currentSearchLogic === 'Or' ? 'selected' : ''}>OR (满足其一)</option>
            </select>
        </span>
        <div>
            <button type="button" class="btn-secondary" style="padding: 2px 8px; font-size: 0.8rem;" onclick="addVisualSearchGroup()">+ 添加逻辑组</button>
            <button type="button" class="btn-secondary" style="padding: 2px 8px; font-size: 0.8rem;" onclick="clearVisualConditions()">清空</button>
        </div>
    `;
    visualContainer.appendChild(controls);

    visualSearchConditions.forEach((item, index) => {
        if (item.type === 'condition') {
            const tag = document.createElement('div');
            tag.className = 'condition-tag';
            tag.innerHTML = `
                <span><strong>${getFieldLabel(item.field)}</strong> ${getOperatorLabel(item.operator)} <em>${item.value}</em></span>
                <span class="remove-btn" onclick="visualSearchConditions.splice(${index}, 1); renderVisualSearchConditions();">&times;</span>
            `;
            visualContainer.appendChild(tag);
        } else if (item.type === 'group') {
            const groupEl = document.createElement('div');
            groupEl.className = 'condition-group';
            groupEl.setAttribute('data-logic', item.logic);
            
            const groupHeader = document.createElement('div');
            groupHeader.className = 'condition-group-header';
            groupHeader.innerHTML = `
                <span>
                    <span class="logic-badge ${item.logic.toLowerCase()}">${item.logic}</span>
                    逻辑组
                    <select class="logic-operator-select" style="margin-left: 5px;" onchange="updateGroupLogic(${index}, this.value); renderVisualSearchConditions();">
                        <option value="Or" ${item.logic === 'Or' ? 'selected' : ''}>OR</option>
                        <option value="And" ${item.logic === 'And' ? 'selected' : ''}>AND</option>
                    </select>
                </span>
                <button type="button" class="btn-secondary" style="padding: 0 5px;" onclick="removeVisualGroup(${index})">&times;</button>
            `;
            groupEl.appendChild(groupHeader);

            const groupContent = document.createElement('div');
            item.conditions.forEach((c, cIdx) => {
                const tag = document.createElement('div');
                tag.className = 'condition-tag';
                tag.innerHTML = `
                    <span><strong>${getFieldLabel(c.field)}</strong> ${getOperatorLabel(c.operator)} <em>${c.value}</em></span>
                    <span class="remove-btn" onclick="visualSearchConditions[${index}].conditions.splice(${cIdx}, 1); renderVisualSearchConditions();">&times;</span>
                `;
                groupContent.appendChild(tag);
            });

            if (item.conditions.length === 0) {
                const tip = document.createElement('p');
                tip.style.fontSize = '0.8rem';
                tip.style.color = '#888';
                tip.style.margin = '5px 10px';
                tip.textContent = '暂无子条件，点击下方按钮添加';
                groupContent.appendChild(tip);
            }

            const addBtn = document.createElement('button');
            addBtn.type = 'button';
            addBtn.className = 'btn-secondary';
            addBtn.style.display = 'block';
            addBtn.style.margin = '5px auto';
            addBtn.style.fontSize = '0.75rem';
            addBtn.style.padding = '2px 10px';
            addBtn.textContent = '+ 将上方选中的条件加入此组';
            addBtn.onclick = () => addConditionToGroup(index);

            groupEl.appendChild(groupContent);
            groupEl.appendChild(addBtn);
            visualContainer.appendChild(groupEl);
        }
    });

    container.appendChild(visualContainer);
}

// 将可视化条件转换为后端需要的 JSON 格式
function compileVisualSearchConditions() {
    if (visualSearchConditions.length === 0) return [];

    const result = visualSearchConditions.map(item => {
        if (item.type === 'condition') {
            return convertConditionToJson(item);
        } else if (item.type === 'group') {
            const groupConditions = item.conditions.map(c => convertConditionToJson(c));
            if (groupConditions.length === 0) return null;
            const obj = {};
            obj[item.logic] = groupConditions;
            return obj;
        }
    }).filter(item => item !== null);

    if (currentSearchLogic === 'Or') {
        return [{ "Or": result }];
    }
    
    return result;
}

// 转换单个条件为 JSON
function convertConditionToJson(item) {
    const field = item.field;
    const operator = item.operator;
    let value = item.value;

    // 根据字段类型转换值 (仅当不是 IN 操作符时)
    if (operator !== 'in') {
        if (field === 'Id' || field === 'ReferenceCount' || field === 'GroupId' || field === 'ClickCount' || field === 'ShareCount') {
            value = parseInt(value);
        } else if (field === 'IsPrimary') {
            value = (value.toLowerCase() === 'true' || value === '1');
        }
    }

    let variant = field;
    if (operator === 'greater') variant += 'GreaterThan';
    else if (operator === 'less') variant += 'LessThan';
    else if (operator === 'like') variant += 'Like';
    else if (operator === 'in') {
        variant += 'In';
        // 确保 value 是字符串再进行 split
        const strValue = String(item.value);
        value = strValue.split(',').map(v => {
            v = v.trim();
            if (field === 'Id' || field === 'ReferenceCount' || field === 'GroupId') return parseInt(v);
            return v;
        });
    }

    const obj = {};
    obj[variant] = value;
    return obj;
}

// 执行可视化搜索
function performVisualSearch() {
    const jsonTab = document.getElementById('json-search');
    // 如果当前在 JSON 标签页，先将 JSON 同步到可视化状态，确保 lastSearchState 保存的是最新修改
    if (jsonTab && jsonTab.style.display !== 'none') {
        syncJsonToVisual();
    }

    const conditions = compileVisualSearchConditions();
    if (conditions.length === 0) {
        showMessage('请添加至少一个查询条件', 'warning');
        return;
    }

    // 保存搜索状态
    lastSearchState.conditions = JSON.parse(JSON.stringify(visualSearchConditions));
    lastSearchState.logic = currentSearchLogic;
    lastSearchState.active = true;
    lastSearchState.target = getActivePageTarget();

    // 关闭模态框并渲染条件展示条
    closeModal();
    renderActiveSearchConditions();

    const target = lastSearchState.target;
    if (target === 'files' && typeof searchFilesByConditions === 'function') {
        searchFilesByConditions(conditions);
    } else if (target === 'groups' && typeof searchGroupsByConditions === 'function') {
        searchGroupsByConditions(conditions);
    } else if (target === 'tags' && typeof searchTagsByConditions === 'function') {
        searchTagsByConditions(conditions);
    } else {
        showMessage('无法确定搜索目标，请在对应页面执行搜索', 'error');
    }
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