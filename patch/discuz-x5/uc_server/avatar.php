<?php
// uc_server 位于 webroot 下一级，因此 webroot = __DIR__ . "/.."。
$uid = isset($_GET["uid"]) ? intval($_GET["uid"]) : 0;
$size = isset($_GET["size"]) ? $_GET["size"] : "small";
$sizes = ["small" => "small", "middle" => "middle", "big" => "big"];
$size = isset($sizes[$size]) ? $sizes[$size] : "small";

$uid_sprintf = sprintf("%09d", $uid);
$subdir = substr($uid_sprintf, 0, 3) . "/" . substr($uid_sprintf, 3, 2) . "/" . substr($uid_sprintf, 5, 2);

// 老站（X2/X3 迁移）的头像文件名是 uid 的末两位（如 001/53/53/10_avatar_middle.jpg），
// 标准命名则是完整九位 UID；两种都试。
$avatar_files = [
    __DIR__ . "/../data/avatar/{$subdir}/{$uid_sprintf}_avatar_{$size}.jpg",
    __DIR__ . "/../data/avatar/{$subdir}/" . ($uid % 100) . "_avatar_{$size}.jpg",
];

// 回退链：用户头像 → 站点默认 → 插件自带默认（随插件分发，始终存在）。
$fallbacks = [
    __DIR__ . "/../data/avatar/noavatar.svg",
    __DIR__ . "/../source/plugin/pokemon/images/site/noavatar.svg",
];

$serve = null;
$type = null;
foreach (array_merge($avatar_files, $fallbacks) as $i => $candidate) {
    if (is_readable($candidate)) {
        $serve = $candidate;
        $type = $i < count($avatar_files) ? "image/jpeg" : "image/svg+xml";
        break;
    }
}

if ($serve === null) {
    // 理论上不会走到这里：插件自带占位图随包分发。
    header("HTTP/1.1 404 Not Found");
    exit;
}

header("Content-Type: {$type}");
header("Cache-Control: public, max-age=86400");
readfile($serve);
