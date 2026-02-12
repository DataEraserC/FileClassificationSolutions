// API基础URL - 自动获取当前访问的域名和端口，以支持跨设备访问
const BASE_URL = window.location.protocol + '//' + window.location.hostname + (window.location.port ? ':' + window.location.port : '');

// 导出配置
window.APP_CONFIG = {
    BASE_URL
};