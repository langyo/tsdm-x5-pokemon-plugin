<?php
defined('IN_DISCUZ') || exit('Access Denied');

loadcache('plugin');
$settings = $_G['cache']['plugin']['pokemon'] ?? [];
require_once __DIR__ . '/game_access.php';

$game_staff = pm_game_is_staff($settings);
$index = isset($_GET['index']) ? preg_replace('/[^a-z_]/', '', $_GET['index']) : 'game';

if (isset($_GET['endpoint'])) {
    $endpoint = $_GET['endpoint'];
    if (preg_match('/^[a-z_]+$/', $endpoint)) {
        // badge/avatar 是图片端点，不能套 JSON 响应头。
        if ($endpoint !== 'badge' && $endpoint !== 'avatar') {
            @header('Content-Type: application/json; charset=utf-8');
            @header('Cache-Control: no-store, no-cache, must-revalidate');
        }

        $api_files = [
            'pokemon' => 'pokemon.php',
            'battle' => 'battle.php',
            'shop' => 'shop.php',
            'evolution' => 'evolution.php',
            'user' => 'user.php',
            'topics' => 'topics.php',
            'admin' => 'admin.php',
            'config' => 'config.php',
            'boss' => 'boss.php',
            'badge' => 'badge.php',
            'badges' => 'badges.php',
            'avatar' => 'avatar.php',
        ];

        if (isset($api_files[$endpoint])) {
            $api_file = __DIR__ . '/api/' . $api_files[$endpoint];
            if (file_exists($api_file)) {
                define('API_ROUTED', true);
                // 标记当前路由的 endpoint 名，供 boss.php 等函数被其他端点
                // 复用的文件区分"作为端点被路由"与"被 battle.php 引入"
                define('API_ENDPOINT', $endpoint);
                // 公告和图片仍可读取，管理入口继续执行自己的权限检查。
                // index=admin 不能豁免 endpoint=battle 等游戏操作。
                if (!$game_staff && !in_array($endpoint, ['admin', 'config', 'badge', 'badges', 'avatar'], true)
                    && pm_game_is_closed($settings)) {
                    require_once __DIR__ . '/api/index.php';
                    api_error(lang('plugin/pokemon', 'system_closed'), 503, null, 'game_closed');
                }
                include_once $api_file;
                return;
            }
        }
    }

    echo json_encode([
        'success' => false,
        'error' => 'Invalid API endpoint',
        'code' => 400,
        'timestamp' => time()
    ], JSON_UNESCAPED_UNICODE);
    return;
}

if ($index !== 'admin' && !$game_staff && pm_game_is_closed($settings)) {
    showmessage(lang('plugin/pokemon', 'system_closed'));
}

$allowed_routes = ['game', 'admin'];
if (!in_array($index, $allowed_routes)) {
    showmessage(lang('plugin/pokemon', 'invalid_route'));
}

if ($index === 'game') {
    // 迁移期双前端：&ui=v2 走 tairitsu 版（game_v2.inc.php），默认仍是
    // Dioxus 版。两者共享同一套 API 与 Discuz 外壳，便于逐页行为对照。
    if (isset($_GET['ui']) && $_GET['ui'] === 'v2') {
        include_once __DIR__ . '/game_v2.inc.php';
        return;
    }
    include_once __DIR__ . '/game.inc.php';
    return;
}

if ($index === 'admin') {
    include_once __DIR__ . '/admincp.inc.php';
    return;
}
