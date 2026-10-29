<?php
defined('IN_DISCUZ') || exit('Access Denied');

global $_G;

$uid = (int)$_G['uid'];

if (!$uid) {
    showmessage('错误: 没有登录');
}

$hide = !empty($_GET['hide']);

DB::update('common_member_field_forum', [
    'pokemon' => $hide ? '' : serialize(get_my_pm_data()),
], ['uid' => $uid]);

showmessage('已' . ($hide ? '隐藏' : '刷新') . '状态栏', '', [], [
    'alert' => 'right'
]);

function get_my_pm_data()
{
    global $_G;
    // Plugin tables use fixed pm_* names, matching install.php and the game API.
    $query = DB::query('SELECT m.id, m.species_id, m.nickname, m.level, m.site, m.is_shiny, COALESCE(d.is_legendary, 0) AS is_legendary FROM pm_mypm m LEFT JOIN pm_data d ON m.species_id = d.id WHERE m.uid=%d AND m.site < 3', [
        (int)$_G['uid']
    ], true);
    if ($query === false) {
        showmessage('无法读取宠物数据，请联系管理员检查插件数据表及数据库配置；原状态栏已保留');
    }
    $data = [];
    $creeps = [];
    while ($pet = DB::fetch($query)) {
        if ($pet['site'] == 1) {
            $data['first'] = $pet;
        } else {
            $creeps[] = $pet;
        }
    }
    DB::free_result($query);
    $data['creeps'] = $creeps;
    return $data;
}
