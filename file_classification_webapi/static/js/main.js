// 主应用入口文件

// Tab切换功能
document.addEventListener('DOMContentLoaded', function() {
    // 延迟初始化，确保部分页面已加载完成
    setTimeout(initTabs, 100);
});

function initTabs() {
    const navLinks = document.querySelectorAll('.nav-link');
    const tabContents = document.querySelectorAll('.tab-content');
    
    // 存储已打开的标签页
    const openedTabs = new Set();
    
    navLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            
            const target = this.getAttribute('data-target');
            
            // 添加到已打开标签页集合
            openedTabs.add(target);
            
            // 隐藏所有tab内容
            tabContents.forEach(content => {
                content.style.display = 'none';
            });
            
            // 移除所有链接的活动状态
            navLinks.forEach(nav => {
                nav.classList.remove('active');
            });
            
            // 显示当前tab内容
            const targetElement = document.getElementById(target);
            if (targetElement) {
                targetElement.style.display = 'block';
            }
            
            // 设置当前链接为活动状态
            this.classList.add('active');
            
            // 页面刚打开时自动搜索一次
            autoSearch(target);
        });
    });
}

// 根据当前tab自动执行搜索
function autoSearch(tabName) {
    switch(tabName) {
        case 'files':
            if (typeof listFilesByFilter === 'function') {
                listFilesByFilter();
            }
            break;
        case 'groups':
            if (typeof listGroupsByFilter === 'function') {
                listGroupsByFilter();
            }
            break;
        case 'tags':
            if (typeof listTagsByFilter === 'function') {
                listTagsByFilter();
            }
            break;
        case 'file-groups':
            if (typeof listFileGroupsByFilter === 'function') {
                listFileGroupsByFilter();
            }
            break;
        case 'group-tags':
            if (typeof listGroupTagsByFilter === 'function') {
                listGroupTagsByFilter();
            }
            break;
        case 'group-relations':
            if (typeof listGroupRelationsByFilter === 'function') {
                listGroupRelationsByFilter();
            }
            break;
    }
}