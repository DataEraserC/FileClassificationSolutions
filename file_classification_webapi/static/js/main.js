// 主应用入口文件

// Tab切换功能
document.addEventListener('DOMContentLoaded', function () {
    // 延迟初始化，确保部分页面已加载完成
    setTimeout(initTabs, 100);

    // 添加ESC键关闭模态框功能
    document.addEventListener('keydown', function (event) {
        if (event.key === 'Escape') {
            // 尝试关闭主模态框
            const modal = document.getElementById('modal');
            if (modal && modal.style.display === 'block') {
                modal.style.display = 'none';
            }

            // 尝试关闭组树模态框
            const groupTreeModal = document.getElementById('group-tree-modal');
            if (groupTreeModal && groupTreeModal.style.display === 'block') {
                groupTreeModal.style.display = 'none';
            }
        }
    });

    // 添加点击模态框背景关闭功能
    document.addEventListener('click', function (event) {
        // 处理主模态框
        const modal = document.getElementById('modal');
        if (modal && event.target === modal) {
            modal.style.display = 'none';
        }

        // 处理组树模态框
        const groupTreeModal = document.getElementById('group-tree-modal');
        if (groupTreeModal && event.target === groupTreeModal) {
            groupTreeModal.style.display = 'none';
        }
    });
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
document.addEventListener('DOMContentLoaded', function () {
    // 初始化主题切换功能
    waitForThemeToggle();

    // 初始化侧边栏切换功能
    waitForSidebarToggle();
    
    // 移动端底部导航适配
    handleMobileBottomNav();
});

function initTabs() {
    const navLinks = document.querySelectorAll('.nav-link');
    const tabContents = document.querySelectorAll('.tab-content');

    // 存储已打开的标签页
    const openedTabs = new Set();

    navLinks.forEach(link => {
        link.addEventListener('click', function (e) {
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

            // 页面刚打开时自动搜索一次，但主页不需要搜索
            if (target !== 'home') {
                autoSearch(target);
            }
            
            // 在移动设备上，点击导航项后滚动到内容顶部
            if (window.innerWidth <= 768) {
                window.scrollTo({ top: 0, behavior: 'smooth' });
            }
        });
    });
}

// 根据当前tab自动执行搜索
function autoSearch(tabName) {
    switch (tabName) {
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
    themeToggle.addEventListener('click', function () {
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
    sidebarToggle.addEventListener('click', function () {
        sidebar.classList.toggle('collapsed');

        // 保存状态到本地存储
        if (sidebar.classList.contains('collapsed')) {
            localStorage.setItem('sidebarState', 'collapsed');
        } else {
            localStorage.setItem('sidebarState', 'expanded');
        }

        // 触发窗口大小调整事件，确保响应式设计正常工作
        window.dispatchEvent(new Event('resize'));
    });
}

// 处理移动端底部导航
function handleMobileBottomNav() {
    // 检测是否为移动设备
    const isMobile = window.innerWidth <= 768;
    
    if (isMobile) {
        const sidebar = document.querySelector('.sidebar');
        // 确保侧边栏在底部
        sidebar.style.position = 'fixed';
        sidebar.style.bottom = '0';
        sidebar.style.top = 'auto';
        
        // 监听窗口大小变化
        window.addEventListener('resize', function() {
            const currentSidebar = document.querySelector('.sidebar');
            if (window.innerWidth <= 768) {
                currentSidebar.style.position = 'fixed';
                currentSidebar.style.bottom = '0';
                currentSidebar.style.top = 'auto';
            } else {
                currentSidebar.style.position = '';
                currentSidebar.style.bottom = '';
            }
        });
    }
}