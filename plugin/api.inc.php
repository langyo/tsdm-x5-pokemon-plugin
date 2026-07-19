<?php

defined('IN_DISCUZ') || exit('Access Denied');

ob_clean();
header('Content-Type: application/json; charset=utf-8');
header('Cache-Control: no-store, no-cache, must-revalidate');

register_shutdown_function(function() {
    $error = error_get_last();
    if ($error !== null && in_array($error['type'], [E_ERROR, E_PARSE, E_CORE_ERROR, E_COMPILE_ERROR])) {
        echo json_encode([
            'success' => false,
            'error' => 'Fatal Error: ' . $error['message'],
            'file' => basename($error['file']),
            'line' => $error['line'],
            'timestamp' => time()
        ], JSON_UNESCAPED_UNICODE);
    }
});

function api_send_error($message, $code = 500, $extra = []) {
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
    api_send_error('Internal Server Error: ' . $e->getMessage(), 500, [
        'file' => basename($e->getFile()),
        'line' => $e->getLine()
    ]);
});

set_error_handler(function($errno, $errstr, $errfile, $errline) {
    if (!(error_reporting() & $errno)) {
        return false;
    }
    api_send_error("PHP Error [$errno]: $errstr", 500, [
        'file' => basename($errfile),
        'line' => $errline
    ]);
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
