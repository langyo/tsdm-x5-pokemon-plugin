<?php
/**
 * Static asset proxy — serves old pokemon_system/ paths from the new location.
 * Caddy routes /source/plugin/pokemon/pokemon_system/* through this file.
 */

$request_uri = $_SERVER['REQUEST_URI'];
$base = '/source/plugin/pokemon/pokemon_system/';
$rel = substr($request_uri, strpos($request_uri, $base) + strlen($base));

$new_base = __DIR__ . '/../../';
$file = $new_base . $rel;

if (!file_exists($file) || is_dir($file)) {
    http_response_code(404);
    exit;
}

$ext = strtolower(pathinfo($file, PATHINFO_EXTENSION));
$mime_types = [
    'gif' => 'image/gif', 'png' => 'image/png', 'jpg' => 'image/jpeg',
    'jpeg' => 'image/jpeg', 'svg' => 'image/svg+xml', 'ico' => 'image/x-icon',
    'css' => 'text/css', 'js' => 'application/javascript',
    'wasm' => 'application/wasm', 'ttf' => 'font/ttf', 'otf' => 'font/otf',
    'woff' => 'font/woff', 'woff2' => 'font/woff2',
    'bmp' => 'image/bmp', 'json' => 'application/json',
];

$mime = $mime_types[$ext] ?? 'application/octet-stream';
header("Content-Type: $mime");
header("Content-Length: " . filesize($file));
header("Cache-Control: public, max-age=86400");

readfile($file);
