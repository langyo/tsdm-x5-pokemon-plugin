<?php

/**
 * 全局配置API
 *
 * 端点:
 * - GET ?action=global_config 获取全局配置
 */

// 如果通过路由访问，加载API辅助函数
if (defined('API_ROUTED')) {
    require_once __DIR__ . '/index.php';
} elseif (!defined('IN_DISCUZ')) {
    require_once __DIR__ . '/bootstrap.php';
    require_once __DIR__ . '/index.php';
}

global $_G;

$action = get_param('action', '');

switch ($action) {
    case 'global_config':
        api_get_global_config();
        break;

    default:
        api_error('Invalid action', 400);
}

/**
 * 获取全局配置
 */
function api_get_global_config()
{
    $config = array();

    $rows = DB::fetch_all("SELECT * FROM pm_config");
    foreach ($rows as $obj) {
        $value = null;
        switch ($obj['data_type']) {
            case "string":
                $value = strval($obj['value']);
                // 对所有字符串字段使用 stripslashes 还原
                $value = stripslashes($value);
                // 对于 news_announcements，解析 JSON 字符串为数组
                if ($obj['key'] === 'news_announcements') {
                    $decoded = json_decode($value, true);
                    if (is_array($decoded)) {
                        $value = $decoded;
                    } else {
                        $value = array(); // JSON 解析失败，返回空数组
                    }
                }
                break;
            case "integer":
                $value = intval($obj['value']);
                break;
            case "boolean":
                $value = boolval($obj['value']);
                break;
        }
        $config[$obj['key']] = $value;
    }

    // 如果 news_announcements 不存在或为空，自动创建默认值
    if (!isset($config['news_announcements']) || !is_array($config['news_announcements'])) {
        $config['news_announcements'] = array();
        // 同步到数据库
        $json_value = json_encode($config['news_announcements']);
        $escaped_value = addslashes($json_value);
        $existing = DB::fetch_first("SELECT * FROM pm_config WHERE `key` = 'news_announcements'");
        if ($existing) {
            DB::query(pm_sql(
                "UPDATE pm_config SET `value` = %s, `data_type` = 'string' WHERE `key` = 'news_announcements'",
                $escaped_value
            ));
        } else {
            DB::query(pm_sql(
                "INSERT INTO pm_config (`key`, `value`, `data_type`) VALUES ('news_announcements', %s, 'string')",
                $escaped_value
            ));
        }
    }

    api_success($config);
}
