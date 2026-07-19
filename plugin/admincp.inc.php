<?php
defined('IN_DISCUZ') || exit('Access Denied');

if (!defined('IN_ADMINCP')) {
    showmessage(lang('plugin/pokemon', 'admin_only'));
}

loadcache('plugin');
$settings = $_G['cache']['plugin']['pokemon'] ?? [];

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
