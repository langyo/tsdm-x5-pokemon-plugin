<?php
defined('IN_DISCUZ') || exit('Access Denied');

$uid = isset($_GET['uid']) ? intval($_GET['uid']) : ($_G['uid'] ?? 0);
if (!$uid) {
    header('Content-Type: image/gif');
    echo base64_decode('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7');
    exit;
}

// Auto-assign medal if user has pokemon but no medal
$hasMedal = DB::result_first("SELECT COUNT(*) FROM pre_common_member_medal WHERE uid=%d AND medalid=99", [$uid]);
if (!$hasMedal) {
    $hasPokemon = DB::result_first("SELECT COUNT(*) FROM pm_mypm WHERE uid=%d AND site<=2", [$uid]);
    if ($hasPokemon) {
        DB::query("INSERT IGNORE INTO pre_common_member_medal (uid, medalid) VALUES (%d, 99)", [$uid]);
        DB::query("UPDATE pre_common_member_field_forum SET medals = IF(medals IS NULL OR medals='','99',CONCAT(medals,'\t99')) WHERE uid=%d", [$uid]);
    }
}

// Use small sprite (spm) instead of large battle sprite
$pet = DB::fetch_first("SELECT species_id FROM pm_mypm WHERE uid=%d AND site=1 LIMIT 1", [$uid]);
if (!$pet) {
    header('Content-Type: image/gif');
    echo base64_decode('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7');
    exit;
}

$imgDir = __DIR__ . '/images/spm';
foreach (['gif','png'] as $ext) {
    $f = "$imgDir/{$pet['species_id']}.$ext";
    if (file_exists($f)) {
        header('Content-Type: image/' . ($ext === 'png' ? 'png' : 'gif'));
        header('Cache-Control: public, max-age=300');
        readfile($f);
        exit;
    }
}
header('Content-Type: image/gif');
echo base64_decode('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7');
