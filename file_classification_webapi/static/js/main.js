// 主应用入口文件

// Tab切换功能
document.addEventListener('DOMContentLoaded', function() {
    // 延迟初始化，确保部分页面已加载完成
    setTimeout(initTabs, 100);
});

// 等待主题切换按钮加载完成后再初始化
function waitForThemeToggle() {
    const themeToggle = document.getElementById('themeToggle');
    if (themeToggle) {
        initThemeToggle();
    } else {
        // 如果还没找到元素，稍后再试
        setTimeout(waitForThemeToggle, 100);
    }
}

// 等待侧边栏切换按钮加载完成后再初始化
function waitForSidebarToggle() {
    const sidebarToggle = document.getElementById('sidebarToggle');
    if (sidebarToggle) {
        initSidebarToggle();
    } else {
        // 如果还没找到元素，稍后再试
        setTimeout(waitForSidebarToggle, 100);
    }
}

// 监听页面内容加载完成事件
document.addEventListener('DOMContentLoaded', function() {
    // 初始化主题切换功能
    waitForThemeToggle();
    
    // 初始化侧边栏切换功能
    waitForSidebarToggle();
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

// 初始化主题切换功能
function initThemeToggle() {
    const themeToggle = document.getElementById('themeToggle');
    const body = document.body;
    
    // 检查本地存储中的主题偏好
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme === 'dark') {
        body.classList.add('dark-mode');
        themeToggle.textContent = '☀️';
    } else {
        themeToggle.textContent = '🌙';
    }
    
    // 添加点击事件监听器
    themeToggle.addEventListener('click', function() {
        body.classList.toggle('dark-mode');
        
        // 更新按钮图标
        if (body.classList.contains('dark-mode')) {
            themeToggle.textContent = '☀️';
            localStorage.setItem('theme', 'dark');
        } else {
            themeToggle.textContent = '🌙';
            localStorage.setItem('theme', 'light');
        }
    });
}

// 初始化侧边栏切换功能
function initSidebarToggle() {
    const sidebarToggle = document.getElementById('sidebarToggle');
    const sidebar = document.querySelector('.sidebar');
    
    // 检查本地存储中的侧边栏状态
    const savedSidebarState = localStorage.getItem('sidebarState');
    if (savedSidebarState === 'collapsed') {
        sidebar.classList.add('collapsed');
    }
    
    // 添加点击事件监听器
    sidebarToggle.addEventListener('click', function() {
        sidebar.classList.toggle('collapsed');
        
        // 保存状态到本地存储
        if (sidebar.classList.contains('collapsed')) {
            localStorage.setItem('sidebarState', 'collapsed');
        } else {
            localStorage.setItem('sidebarState', 'expanded');
        }
    });
}