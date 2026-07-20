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

include template('common/header_common');
include template('common/header');

echo '<div id="pt" class="bm cl"><div class="z">';
echo '<a href="./" class="nvhm" title="首页">' . $_G['setting']['bbname'] . '</a><em>&raquo;</em>';
echo '<a href="plugin.php?id=pokemon:game&index=admin">宠物管理</a>';
echo '</div></div>';

echo '<div class="bm bw0"><div class="bm_h"><h2>宠物管理</h2></div><div class="bm_c">';

switch ($op) {
    case 'index':
        echo '<p>管理功能</p>';
        break;

    default:
        $file = __DIR__ . '/admin/routes/' . $op . '.php';
        if (file_exists($file)) {
            echo '<div style="padding:10px">';
            include $file;
            echo '</div>';
        } else {
            echo '<p class="alert_error">未知操作</p>';
        }
        break;
}

echo '</div></div>';

include template('common/footer');
