// 动态加载HTML内容的工具函数
document.addEventListener('DOMContentLoaded', function () {
    // 加载所有部分页面内容
    loadPartials();
});

function loadPartials() {
    // 定义需要加载的部分页面
    const partials = [
        {id: 'header-placeholder', file: 'partials/header.html'},
        {id: 'sidebar-placeholder', file: 'partials/sidebar.html'},
        {id: 'home-placeholder', file: 'partials/home.html'},
        {id: 'files-placeholder', file: 'partials/files.html'},
        {id: 'groups-placeholder', file: 'partials/groups.html'},
        {id: 'tags-placeholder', file: 'partials/tags.html'},
        {id: 'file-groups-placeholder', file: 'partials/file-groups.html'},
        {id: 'group-tags-placeholder', file: 'partials/group-tags.html'},
        {id: 'group-relations-placeholder', file: 'partials/group-relations.html'},
        {id: 'modal-placeholder', file: 'partials/modal.html'}
    ];

    // 加载每个部分页面
    partials.forEach(partial => {
        fetch(partial.file)
            .then(response => response.text())
            .then(html => {
                const placeholder = document.getElementById(partial.id);
                if (placeholder) {
                    placeholder.innerHTML = html;
                }

                // 当所有内容加载完成后，初始化应用
                if (partial.id === 'modal-placeholder') {
                    initializeApp();
                }
            })
            .catch(error => {
                console.error(`加载 ${partial.file} 失败:`, error);
            });
    });
}

function initializeApp() {
    // 初始化应用功能
    // 这里可以添加一些初始化逻辑

    // 默认显示主页
    const homeTab = document.getElementById('home');
    if (homeTab) {
        homeTab.style.display = 'block';
    }
}