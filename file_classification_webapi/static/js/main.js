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

    // 页面加载时只显示首页内容
    const homeTab = document.getElementById('home');
    if (homeTab) {
        homeTab.style.display = 'block';
        // 主页也需要执行一次搜索以显示默认数据
        autoSearch('home');
    }

    // 隐藏除主页外的所有标签页内容
    tabContents.forEach(content => {
        if (content.id !== 'home') {
            content.style.display = 'none';
        }
    });

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
        case 'home':
            // 主页可能需要执行某些初始化操作
            break;
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
        // 确保侧边栏元素存在
        if (sidebar) {
            // 确保侧边栏在底部
            sidebar.style.position = 'fixed';
            sidebar.style.bottom = '0';
            sidebar.style.top = 'auto';
        }
        
        // 监听窗口大小变化
        window.addEventListener('resize', function() {
            const currentSidebar = document.querySelector('.sidebar');
            if (currentSidebar) {
                if (window.innerWidth <= 768) {
                    currentSidebar.style.position = 'fixed';
                    currentSidebar.style.bottom = '0';
                    currentSidebar.style.top = 'auto';
                } else {
                    currentSidebar.style.position = '';
                    currentSidebar.style.bottom = '';
                }
            }
        });
    }
}

// main.js - 主程序入口文件

// 页面加载完成后初始化
document.addEventListener('DOMContentLoaded', function () {
    // 加载所有部分页面
    loadPartials();

    // 初始化路由
    initRouter();

    // 监听窗口大小变化，处理响应式布局
    window.addEventListener('resize', handleResponsiveLayout);
    
    // 初始检查屏幕大小
    handleResponsiveLayout();
});

// 加载所有部分页面
function loadPartials() {
    // 定义需要加载的部分页面
    const partials = [
        { placeholder: 'header-placeholder', file: 'partials/header.html' },
        { placeholder: 'sidebar-placeholder', file: 'partials/sidebar.html' },
        { placeholder: 'home-placeholder', file: 'partials/home.html' },
        { placeholder: 'files-placeholder', file: 'partials/files.html' },
        { placeholder: 'groups-placeholder', file: 'partials/groups.html' },
        { placeholder: 'tags-placeholder', file: 'partials/tags.html' },
        { placeholder: 'file-groups-placeholder', file: 'partials/file-groups.html' },
        { placeholder: 'group-tags-placeholder', file: 'partials/group-tags.html' },
        { placeholder: 'group-relations-placeholder', file: 'partials/group-relations.html' },
        { placeholder: 'modal-placeholder', file: 'partials/modal.html' }
    ];

    // 逐个加载部分页面
    partials.forEach(partial => {
        fetch(partial.file)
            .then(response => response.text())
            .then(data => {
                document.getElementById(partial.placeholder).innerHTML = data;
            })
            .catch(error => {
                console.error(`加载 ${partial.file} 失败:`, error);
            });
    });
}

// 初始化路由
function initRouter() {
    // 监听浏览器前进后退事件
    window.addEventListener('popstate', function (event) {
        navigateToFragment();
    });

    // 初始导航
    navigateToFragment();

    // 绑定导航链接事件
    document.addEventListener('click', function (event) {
        const navLink = event.target.closest('.nav-link');
        if (navLink) {
            event.preventDefault();
            const target = navLink.getAttribute('href').substring(1); // 移除 # 前缀
            navigateTo(target);
            
            // 在移动端点击导航链接后隐藏导航栏
            hideMobileNavbar();
        }
    });
}

// 导航到指定片段
function navigateTo(fragment) {
    // 更新浏览器历史记录
    history.pushState(null, '', `#${fragment}`);

    // 执行导航
    navigateToFragment();
}

// 执行实际的导航操作
function navigateToFragment() {
    // 隐藏所有内容区域
    const contentSections = document.querySelectorAll('.tab-content');
    contentSections.forEach(section => {
        section.style.display = 'none';
    });

    // 获取目标片段
    const fragment = window.location.hash.substring(1) || 'home';

    // 显示目标内容区域
    const targetSection = document.getElementById(fragment);
    if (targetSection) {
        targetSection.style.display = 'block';
    }

    // 更新导航链接的活动状态
    const navLinks = document.querySelectorAll('.nav-link');
    navLinks.forEach(link => {
        link.classList.remove('active');
        if (link.getAttribute('href') === `#${fragment}`) {
            link.classList.add('active');
        }
    });
}

// 处理响应式布局
function handleResponsiveLayout() {
    const isMobile = window.innerWidth <= 768;
    
    // 获取侧边栏和移动端导航栏元素
    const sidebar = document.querySelector('.sidebar');
    const mobileNavbar = document.querySelector('.mobile-navbar');
    
    if (isMobile) {
        // 移动端：隐藏侧边栏，显示底部导航栏
        if (sidebar) {
            sidebar.style.display = 'none';
        }
        if (mobileNavbar) {
            mobileNavbar.style.display = 'block';
        }
    } else {
        // 桌面端：显示侧边栏，隐藏底部导航栏
        if (sidebar) {
            sidebar.style.display = 'flex';
        }
        if (mobileNavbar) {
            mobileNavbar.style.display = 'none';
        }
    }
}

// 隐藏移动端导航栏（用于点击导航项后自动隐藏）
function hideMobileNavbar() {
    const mobileNavbar = document.querySelector('.mobile-navbar');
    if (mobileNavbar) {
        // 可以添加动画效果，这里简单地隐藏
        mobileNavbar.style.display = 'none';
        
        // 延迟一段时间后重新显示，确保用户仍然可以看到导航栏
        setTimeout(() => {
            if (window.innerWidth <= 768) {
                mobileNavbar.style.display = 'block';
            }
        }, 100);
    }
}
