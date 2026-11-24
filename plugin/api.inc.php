<?php

defined('IN_DISCUZ') || exit('Access Denied');

header('Content-Type: application/json; charset=utf-8');
header('Cache-Control: no-store, no-cache, must-revalidate');

// 致命错误与异常只记服务端日志，响应里不带 file/line/原始消息，
// 避免向客户端泄露路径、SQL 片段等内部细节。
register_shutdown_function(function() {
    $error = error_get_last();
    if ($error !== null && in_array($error['type'], [E_ERROR, E_PARSE, E_CORE_ERROR, E_COMPILE_ERROR])) {
        error_log(sprintf(
            '[Pokemon API Fatal] %s:%d - %s',
            $error['file'],
            $error['line'],
            $error['message']
        ));
        echo json_encode([
            'success' => false,
            'error' => 'Internal Server Error',
            'timestamp' => time()
        ], JSON_UNESCAPED_UNICODE);
    }
});

function api_send_error($message, $code = 500, $extra = []) {
    http_response_code($code);
    $response = array_merge([
        'success' => false,
        'error' => $message,
        'code' => $code,
        'timestamp' => time()
    ], $extra);
    echo json_encode($response, JSON_UNESCAPED_UNICODE);
    exit;
}

set_exception_handler(function($e) {
    error_log(sprintf(
        '[Pokemon API Exception] %s:%d - %s',
        $e->getFile(),
        $e->getLine(),
        $e->getMessage()
    ));
    api_send_error('Internal Server Error', 500);
});

set_error_handler(function($errno, $errstr, $errfile, $errline) {
    if (!(error_reporting() & $errno)) {
        return false;
    }
    error_log("[Pokemon API Error] $errfile:$errline - [$errno] $errstr");
    api_send_error('Internal Server Error', 500);
});

$endpoint = isset($_GET['endpoint']) ? $_GET['endpoint'] : '';

if (!preg_match('/^[a-z_]+$/', $endpoint)) {
    api_send_error('Invalid API endpoint', 400);
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
];

if (isset($api_files[$endpoint])) {
    $api_file = __DIR__ . '/api/' . $api_files[$endpoint];

    if (file_exists($api_file)) {
        define('API_ROUTED', true);
        include_once $api_file;
        exit;
    }

    api_send_error("API file not found: {$api_files[$endpoint]}", 404);
}

api_send_error("Unknown endpoint: {$endpoint}", 404);
