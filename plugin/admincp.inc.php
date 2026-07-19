<?php
defined('IN_DISCUZ') || exit('Access Denied');

$is_admin = ($_G['adminid'] == 1 || $_G['groupid'] == 1);
$is_moderator = false;
if (!$is_admin && $_G['uid']) {
    $modcheck = DB::result_first("SELECT COUNT(*) FROM " . DB::table('forum_moderator') . " WHERE uid=%d", [$_G['uid']]);
    $is_moderator = $modcheck > 0;
}
if (!$is_admin && !$is_moderator) {
    showmessage(lang('plugin/pokemon', 'admin_only'));
}

$op = isset($_GET['op']) ? preg_replace('/[^a-z_]/', '', $_GET['op']) : 'index';

switch ($op) {
    case 'index':
        showtableheader();
        showtablerow('', '', lang('plugin/pokemon', 'name'));
        showtablefooter();
        break;

    default:
        $file = __DIR__ . '/admin/routes/' . $op . '.php';
        if (file_exists($file)) {
            include $file;
        } else {
            cpmsg('undefined_action', '', 'error');
        }
        break;
}
