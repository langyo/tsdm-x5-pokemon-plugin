<?php
defined('IN_DISCUZ') || exit('Access Denied');

// 后台与 JSON API 共用的请求安全检查（formhash）：游戏页与管理页在 fetch 外包一层，
// 同源请求自动带 X-Pm-Formhash 头；跨站页面无法设置自定义头（会触发 CORS 预检失败），
// 也拿不到 formhash。

if (!function_exists('pm_request_formhash')) {
    function pm_request_formhash()
    {
        if (!empty($_SERVER['HTTP_X_PM_FORMHASH'])) {
            return (string) $_SERVER['HTTP_X_PM_FORMHASH'];
        }
        if (isset($_GET['formhash']) && is_string($_GET['formhash'])) {
            return $_GET['formhash'];
        }
        if (isset($_POST['formhash']) && is_string($_POST['formhash'])) {
            return $_POST['formhash'];
        }
        return '';
    }

    function pm_formhash_ok()
    {
        $hash = pm_request_formhash();
        return $hash !== '' && hash_equals(formhash(), $hash);
    }
}

if (!function_exists('pm_public_readonly_request')) {
    /**
     * 是否为豁免 formhash 的"公开只读"请求。
     *
     * 帖子侧栏宠物卡片（endpoint=pokemon&action=card）在 viewthread 页面由
     * 弹窗直接 GET 拉取：徽章对未登录访客也公开展示，该请求天然不带
     * formhash；端点不改写任何数据（HP 只读钳制），CSRF 防护对只读 GET
     * 无意义。新增豁免必须是严格只读的 GET，写操作一律不得进入此函数。
     */
    function pm_public_readonly_request()
    {
        if (isset($_SERVER['REQUEST_METHOD']) && $_SERVER['REQUEST_METHOD'] !== 'GET') {
            return false;
        }
        if (!isset($_GET['endpoint']) || !is_string($_GET['endpoint']) || $_GET['endpoint'] !== 'pokemon') {
            return false;
        }
        return isset($_GET['action']) && is_string($_GET['action']) && $_GET['action'] === 'card';
    }
}
