<?php
/**
 * 头像接口 — 替代 X5 已不存在的 uc_server/avatar.php。
 *
 * 兼容两种头像文件命名：
 * - 标准：  data/avatar/{前3}/{中2}/{后2}/{九位UID}_avatar_{size}.jpg
 * - 老站：  data/avatar/{前3}/{中2}/{后2}/{UID末两位}_avatar_{size}.jpg
 * 都缺失时回退到站点默认占位图，最后是插件自带的占位 SVG。
 */

if (!defined('IN_DISCUZ') && !defined('API_ROUTED')) exit;

$uid = isset($_GET['uid']) ? intval($_GET['uid']) : 0;
$size = isset($_GET['size']) ? $_GET['size'] : 'middle';
$sizes = ['small' => 'small', 'middle' => 'middle', 'big' => 'big'];
$size = isset($sizes[$size]) ? $sizes[$size] : 'small';

$webroot = dirname(__DIR__, 4);
$serve = null;
$type = null;

if ($uid > 0) {
    $uid_sprintf = sprintf('%09d', $uid);
    $subdir = substr($uid_sprintf, 0, 3) . '/' . substr($uid_sprintf, 3, 2) . '/' . substr($uid_sprintf, 5, 2);
    $avatar_files = [
        [$webroot . "/data/avatar/{$subdir}/{$uid_sprintf}_avatar_{$size}.jpg", 'image/jpeg'],
        [$webroot . "/data/avatar/{$subdir}/" . ($uid % 100) . "_avatar_{$size}.jpg", 'image/jpeg'],
    ];
    foreach ($avatar_files as [$file, $mime]) {
        if (is_readable($file)) {
            $serve = $file;
            $type = $mime;
            break;
        }
    }
}

if ($serve === null) {
    $serve = __DIR__ . '/../images/site/noavatar.svg';
    $type = 'image/svg+xml';
}

header('Content-Type: ' . $type);
header('Cache-Control: public, max-age=86400');
readfile($serve);
