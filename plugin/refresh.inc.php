<?php
defined('IN_DISCUZ') || exit('Access Denied');

global $_G;

$uid = (int)$_G['uid'];

if (!$uid) {
    showmessage('错误: 没有登录');
}

$hide = (bool)$_GET['hide'];

DB::update('common_member_field_forum', [
    'pokemon' => $hide ? '' : serialize(get_my_pm_data()),
], "uid=%d", [$uid]);

showmessage('已' . ($hide ? '隐藏' : '刷新') . '状态栏', '', [], [
    'alert' => 'right'
]);

function get_my_pm_data()
{
    global $_G;
    $rows = DB::fetch_all('SELECT id, pmno, nowname, level, site, sg FROM %t WHERE uid=%d AND site < 3', [
        'pm_mypm', $_G['uid']
    ]);
    $data = [];
    $creeps = [];
    foreach ($rows as $pet) {
        if ($pet['site'] == 1) {
            $data['first'] = $pet;
        } else {
            $creeps[] = $pet;
        }
    }
    $data['creeps'] = $creeps;
    return $data;
}
