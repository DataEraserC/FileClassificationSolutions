// 主应用入口文件

// Tab切换功能
document.addEventListener('DOMContentLoaded', function() {
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
            document.getElementById(target).style.display = 'block';
            
            // 设置当前链接为活动状态
            this.classList.add('active');
            
            // 页面刚打开时自动搜索一次
            autoSearch(target);
        });
    });
    
    // 默认显示第一个tab并自动搜索
    if (tabContents.length > 0) {
        tabContents[0].style.display = 'block';
        navLinks[0].classList.add('active');
        openedTabs.add(navLinks[0].getAttribute('data-target'));
        
        // 自动执行首次搜索
        autoSearch(navLinks[0].getAttribute('data-target'));
    }
});

// 根据当前tab自动执行搜索
function autoSearch(tabName) {
    switch(tabName) {
        case 'files':
            listFilesByFilter();
            break;
        case 'groups':
            listGroupsByFilter();
            break;
        case 'tags':
            listTagsByFilter();
            break;
        case 'file-groups':
            listFileGroupsByFilter();
            break;
        case 'group-tags':
            listGroupTagsByFilter();
            break;
        case 'group-relations':
            listGroupRelationsByFilter();
            break;
    }
}