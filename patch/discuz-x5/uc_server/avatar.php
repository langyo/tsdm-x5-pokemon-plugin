<?php
$uid = isset($_GET["uid"]) ? intval($_GET["uid"]) : 0;
$size = isset($_GET["size"]) ? $_GET["size"] : "small";
$sizes = ["small" => "small", "middle" => "middle", "big" => "big"];
$size = isset($sizes[$size]) ? $sizes[$size] : "small";

$uid_sprintf = sprintf("%09d", $uid);
$subdir = substr($uid_sprintf, 0, 3) . "/" . substr($uid_sprintf, 3, 2) . "/" . substr($uid_sprintf, 5, 2);
$avatar_file = __DIR__ . "/../../data/avatar/{$subdir}/{$uid_sprintf}_avatar_{$size}.jpg";

if (file_exists($avatar_file)) {
    header("Content-Type: image/jpeg");
    header("Cache-Control: public, max-age=86400");
    readfile($avatar_file);
} else {
    header("Content-Type: image/svg+xml");
    header("Cache-Control: public, max-age=86400");
    readfile(__DIR__ . "/../../data/avatar/noavatar.svg");
}
