<?php
defined('IN_DISCUZ') || exit('Access Denied');

loadcache('plugin');
$settings = $_G['cache']['plugin']['pokemon'] ?? [];

$index = isset($_GET['index']) ? preg_replace('/[^a-z_]/', '', $_GET['index']) : 'game';

if (isset($_GET['endpoint'])) {
    $endpoint = $_GET['endpoint'];
    if (preg_match('/^[a-z_]+$/', $endpoint)) {
        if ($endpoint !== 'badge') {
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
            'badge' => 'badge.php',
            'badges' => 'badges.php',
        ];

        if (isset($api_files[$endpoint])) {
            $api_file = __DIR__ . '/api/' . $api_files[$endpoint];
            if (file_exists($api_file)) {
                define('API_ROUTED', true);
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

if (empty($settings['is_open']) && $index !== 'admin') {
    $gmarray = explode(',', $settings['poke_smgly'] ?? '');
    if (!in_array($_G['username'], $gmarray)) {
        showmessage(lang('plugin/pokemon', 'system_closed'));
    }
}

$allowed_routes = ['game', 'admin'];
if (!in_array($index, $allowed_routes)) {
    showmessage(lang('plugin/pokemon', 'invalid_route'));
}

if ($index === 'game') {
    include_once __DIR__ . '/game.inc.php';
    return;
}

if ($index === 'admin') {
    include_once __DIR__ . '/admincp.inc.php';
    return;
}
