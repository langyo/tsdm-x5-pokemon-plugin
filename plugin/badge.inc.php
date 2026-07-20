<?php
defined('IN_DISCUZ') || exit('Access Denied');

$uid = isset($_GET['uid']) ? intval($_GET['uid']) : 0;
if (!$uid) {
    header('HTTP/1.0 404 Not Found');
    exit;
}

$imgDir = dirname(__DIR__) . '/images/pm';

// Get user's first pokemon
$pet = DB::fetch_first(
    "SELECT pmno, level, nowname FROM pm_mypm WHERE uid=%d AND site=1 LIMIT 1",
    [$uid]
);

if (!$pet) {
    // No pokemon - return transparent pixel
    header('Content-Type: image/gif');
    header('Cache-Control: no-cache');
    echo base64_decode('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7');
    exit;
}

$pngFile = "$imgDir/{$pet['pmno']}.png";
$gifFile = "$imgDir/{$pet['pmno']}.gif";

if (file_exists($pngFile)) {
    header('Content-Type: image/png');
    header('Cache-Control: public, max-age=300');
    readfile($pngFile);
} elseif (file_exists($gifFile)) {
    header('Content-Type: image/gif');
    header('Cache-Control: public, max-age=300');
    readfile($gifFile);
} else {
    header('Content-Type: image/gif');
    header('Cache-Control: no-cache');
    echo base64_decode('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7');
}
