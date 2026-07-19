<?php

/**
 * 物品模块函数库
 *
 * 定义各种物品的使用效果
 * 每个物品函数返回 1 表示无法使用，返回其他值表示成功
 */

defined('IN_DISCUZ') || exit('Access Denied');

/**
 * 安详铃 - 提高亲密度
 */
function ocl($petid, $itemname)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    // 获取物品效果值
    $item = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE module = 'ocl' LIMIT 1"
    ));
    $good_increase = isset($item['sitemid']) ? (int)$item['sitemid'] : 15;

    $new_good = min(255, (int)$pet['good'] + $good_increase);

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET good = %d WHERE id = %d",
        $new_good,
        $petid
    ));

    return 0;
}

/**
 * 哞哞奶 - 解除饥饿状态并恢复HP
 */
function yypg($petid, $itemname)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    // 获取物品数据
    $item = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE module = 'yypg' LIMIT 1"
    ));

    $heal_amount = isset($item['addhp']) ? (int)$item['addhp'] : 50;

    // 饥饿状态是 5, 6
    $negative_states = [5, 6];
    $state = (int)$pet['state'];

    if (!in_array($state, $negative_states)) {
        // 非饥饿状态，但可以回复HP
        $max_hp = api_calculate_pokemon_max_hp($pet);
        $current_hp = (int)$pet['hp'];

        if ($current_hp >= $max_hp) {
            return 1; // 不需要使用
        }

        $new_hp = min($max_hp, $current_hp + $heal_amount);
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
            $new_hp,
            $petid
        ));
        return 0;
    }

    // 解除饥饿状态并恢复HP
    $max_hp = api_calculate_pokemon_max_hp($pet);
    $new_hp = min($max_hp, (int)$pet['hp'] + $heal_amount);

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET hp = %d, state = 1, statetime = %d WHERE id = %d",
        $new_hp,
        time(),
        $petid
    ));

    return 0;
}

/**
 * 釜炎仙贝 - 解除所有异常状态
 */
function sprite($petid, $itemname)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    $state = (int)$pet['state'];

    // 负面状态（需要治疗的异常状态，不包括濒危0）
    $negative_states = [2, 3, 4, 5, 6, 7, 11, 15];

    if (!in_array($state, $negative_states)) {
        return 1; // 没有异常状态不需要使用
    }

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET state = 1, statetime = %d WHERE id = %d",
        time(),
        $petid
    ));

    return 0;
}

/**
 * 元气之粉 - 治疗生病状态
 */
function disease($petid, $itemname)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    $state = (int)$pet['state'];

    // 生病状态 2, 3, 4
    $disease_states = [2, 3, 4];

    if (!in_array($state, $disease_states)) {
        return 1; // 没有生病不需要使用
    }

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET state = 1, statetime = %d WHERE id = %d",
        time(),
        $petid
    ));

    return 0;
}

/**
 * 心形甜品 - 解除饥饿状态
 */
function hunger($petid, $itemname)
{
    return yypg($petid, $itemname); // 与哞哞奶效果相同
}

/**
 * 毛绒尾玩偶 - 提高捕获率（无实际效果，返回成功）
 */
function cap($petid, $itemname)
{
    // 这个物品主要用于战斗中提高捕获率
    // 在背包中使用只是告知用户效果
    return 0;
}

/**
 * 品质重洗药 - 重洗品质
 */
function quality($petid, $itemname)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    // 随机生成新的品质 (1-31)
    $new_quality = rand(1, 31);

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET quality = %d WHERE id = %d",
        $new_quality,
        $petid
    ));

    return 0;
}

/**
 * 容量箱子 - 增加背包容量
 */
function box($petid, $itemname)
{
    global $_G;

    $uid = $_G['uid'];

    // 获取物品数据
    $item = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE module = 'box' LIMIT 1"
    ));

    if (!$item) {
        return 1;
    }

    $capacity_increase = isset($item['sitemid']) ? (int)$item['sitemid'] : 9;

    // 获取用户数据
    $user_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $uid
    ));

    if (!$user_data) {
        // 创建用户数据
        DB::query(pm_sql(
            "INSERT INTO " . pm_table('pm_usersdata') . " (uid, boxcapacity) VALUES (%d, %d)",
            $uid,
            $capacity_increase
        ));
    } else {
        // 增加容量
        $current_capacity = isset($user_data['boxcapacity']) ? (int)$user_data['boxcapacity'] : 0;
        $new_capacity = $current_capacity + $capacity_increase;

        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_usersdata') . " SET boxcapacity = %d WHERE uid = %d",
            $new_capacity,
            $uid
        ));
    }

    return 0;
}

/**
 * PP恢复 - 回复PP值
 */
function pp5($petid, $itemname, $pp_amount = 5)
{
    return _restore_pp($petid, $pp_amount);
}

function pp10($petid, $itemname, $pp_amount = 10)
{
    return _restore_pp($petid, $pp_amount);
}

function pp15($petid, $itemname, $pp_amount = 15)
{
    return _restore_pp($petid, $pp_amount);
}

function pp99($petid, $itemname)
{
    return _restore_pp($petid, 999); // 全满
}

/**
 * 内部函数：恢复PP值
 */
function _restore_pp($petid, $amount)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    // 查找PP未满的技能
    $skill = DB::fetch_first(pm_sql(
        "SELECT ms.*, s.num as max_pp
         FROM " . pm_table('pm_myskill') . " ms
         LEFT JOIN " . pm_table('pm_skill') . " s ON ms.skillid = s.id
         WHERE ms.uid = %d AND ms.petid = %d AND ms.skillnum < s.num
         LIMIT 1",
        $uid,
        $petid
    ));

    if (!$skill) {
        return 1; // 所有技能PP都已满
    }

    $max_pp = (int)$skill['max_pp'];
    $current_pp = (int)$skill['skillnum'];
    $restore_amount = $amount >= 999 ? ($max_pp - $current_pp) : min($amount, $max_pp - $current_pp);

    $new_pp = $current_pp + $restore_amount;

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_myskill') . " SET skillnum = %d WHERE skillid = %d AND uid = %d AND petid = %d",
        $new_pp,
        $skill['skillid'],
        $uid,
        $petid
    ));

    return 0;
}

/**
 * 怠惰药片 - 清空努力值
 */
function clean_equipment($petid, $itemname)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    // 清空努力值
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET expn = 0 WHERE id = %d",
        $petid
    ));

    return 0;
}

/**
 * 努力值提升剂类
 */
function hpn10($petid, $itemname, $ihp = 0)
{
    return _increase_effort_value($petid, 'hp', 10);
}

function atkn10($petid, $itemname, $iatk = 0)
{
    return _increase_effort_value($petid, 'atk', 10);
}

function defn10($petid, $itemname, $idef = 0)
{
    return _increase_effort_value($petid, 'def', 10);
}

function spatkn10($petid, $itemname, $ispatk = 0)
{
    return _increase_effort_value($petid, 'spatk', 10);
}

function spdefn10($petid, $itemname, $ispdef = 0)
{
    return _increase_effort_value($petid, 'spdef', 10);
}

function sdn10($petid, $itemname, $isd = 0)
{
    return _increase_effort_value($petid, 'speed', 10);
}

/**
 * 内部函数：增加努力值
 */
function _increase_effort_value($petid, $stat, $amount)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    // 获取当前努力值 (从 expn 字段解析)
    $expn = isset($pet['expn']) ? $pet['expn'] : '0,0,0,0,0,0';
    $evs = explode(',', $expn);

    $stat_index = [
        'hp' => 0,
        'atk' => 1,
        'def' => 2,
        'spatk' => 3,
        'spdef' => 4,
        'speed' => 5,
    ];

    $idx = $stat_index[$stat];
    $current_ev = isset($evs[$idx]) ? (int)$evs[$idx] : 0;

    // 检查总努力值上限 (510)
    $total_ev = array_sum(array_map('intval', $evs));

    if ($total_ev >= 510) {
        return 1; // 已达总上限
    }

    // 单项努力值上限 (252)
    if ($current_ev >= 252) {
        return 1; // 该项已达上限
    }

    // 计算实际增加值
    $actual_increase = min($amount, 252 - $current_ev, 510 - $total_ev);
    $evs[$idx] = (int)$evs[$idx] + $actual_increase;

    $new_expn = implode(',', $evs);

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET expn = %s WHERE id = %d",
        $new_expn,
        $petid
    ));

    return 0;
}

/**
 * 进化石类 - 需要通过进化系统使用
 */
function szs($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function czs($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function hzs($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function lzs($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function yzs($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function tys($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function bys($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function ll($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function cd($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function jswt($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function wzzz($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function txq($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function bhj($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function dlqdq($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function yjqdq($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function rlyc($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function rlzz($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function jxzs($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function mjwy($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function hazs($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function gzs($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function sjbd($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function dtd($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function dsl($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function sspg($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}
function ttpg($petid, $itemname, $itemid = 0)
{
    return _evolution_stone($petid, $itemname, $itemid);
}

/**
 * 内部函数：进化石处理
 */
function _evolution_stone($petid, $itemname, $itemid = 0)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    // 如果没有传递物品 ID，尝试通过名称查找
    if (!$itemid) {
        $item = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE name = %s LIMIT 1",
            $itemname
        ));
        if ($item) {
            $itemid = (int)$item['id'];
        }
    }

    if (!$itemid) {
        return 1;
    }

    // 查找对应的进化条件
    $evolution = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_up') . "
        WHERE pmid = %d AND cond = 'item' AND val = %d",
        $pet['pmno'],
        $itemid
    ));

    if (!$evolution) {
        return 1; // 该宠物不能使用此进化石
    }

    // 执行进化
    $new_type_id = $evolution['targetpmid'];
    $new_base_info = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d",
        $new_type_id
    ));

    if (!$new_base_info) {
        return 1;
    }

    $new_max_hp = api_calculate_pokemon_max_hp($pet, $new_base_info);

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET
            pmno = %d,
            hp = %d,
            pmname = %s,
            nowname = %s
            WHERE id = %d",
        $new_type_id,
        $new_max_hp,
        $new_base_info['name'],
        $new_base_info['name'],
        $petid
    ));

    return 0;
}

/**
 * 奇异糖果 - 等级提升
 */
function lvupitem($petid, $itemname)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    $current_level = (int)$pet['level'];

    if ($current_level >= 100) {
        return 1; // 已达最高等级
    }

    $new_level = min(100, $current_level + 1);

    // 更新等级
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET level = %d WHERE id = %d",
        $new_level,
        $petid
    ));

    // 重新获取宠物数据以获取新的等级
    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    // 计算新的最大HP（使用更新后的宠物数据）
    $new_max_hp = api_calculate_pokemon_max_hp($pet);

    // 更新HP
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
        $new_max_hp,
        $petid
    ));

    return 0;
}

/**
 * 强制升级盒 - 强制进化
 */
function jup($petid, $itemname)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    // 查找任何进化路线
    $evolution = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_up') . " WHERE pmid = %d LIMIT 1",
        $pet['pmno']
    ));

    if (!$evolution) {
        return 1; // 无法进化
    }

    // 执行进化
    $new_type_id = $evolution['targetpmid'];
    $new_base_info = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d",
        $new_type_id
    ));

    if (!$new_base_info) {
        return 1;
    }

    $new_max_hp = api_calculate_pokemon_max_hp($pet, $new_base_info);

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET
            pmno = %d,
            hp = %d,
            pmname = %s,
            nowname = %s
            WHERE id = %d",
        $new_type_id,
        $new_max_hp,
        $new_base_info['name'],
        $new_base_info['name'],
        $petid
    ));

    return 0;
}

/**
 * 精灵球 - 在背包中使用无效
 */
function jlq($petid, $itemname)
{
    return 1;
}
function cjq($petid, $itemname)
{
    return 1;
}
function gjq($petid, $itemname)
{
    return 1;
}
function sjq($petid, $itemname)
{
    return 1;
}
function ssq($petid, $itemname)
{
    return 1;
}
function slq($petid, $itemname)
{
    return 1;
}
function zxq($petid, $itemname)
{
    return 1;
}
function cfq($petid, $itemname)
{
    return 1;
}
function cxq($petid, $itemname)
{
    return 1;
}
function tmq($petid, $itemname)
{
    return 1;
}
function haq($petid, $itemname)
{
    return 1;
}
function ylq($petid, $itemname)
{
    return 1;
}
function zbq($petid, $itemname)
{
    return 1;
}
function ksq($petid, $itemname)
{
    return 1;
}
function dsq($petid, $itemname)
{
    return 1;
}
function zzq($petid, $itemname)
{
    return 1;
}
function djq($petid, $itemname)
{
    return 1;
}
function yyq($petid, $itemname)
{
    return 1;
}
function yuelq($petid, $itemname)
{
    return 1;
}
function axq($petid, $itemname)
{
    return 1;
}
function jsq($petid, $itemname)
{
    return 1;
}
function sxq($petid, $itemname)
{
    return 1;
}
function gyq($petid, $itemname)
{
    return 1;
}
function mjq($petid, $itemname)
{
    return 1;
}
function tail($petid, $itemname)
{
    return 1;
}

/**
 * 精灵球礼包 - 获得精灵球
 */
function libaochunj($petid, $itemname)
{
    global $_G;
    $uid = $_G['uid'];

    // 按照5:4:1的比例获得精灵球、超级球、高级球
    $items_to_add = [
        ['typeid' => 24, 'nums' => 5], // 精灵球
        ['typeid' => 25, 'nums' => 4], // 超级球
        ['typeid' => 26, 'nums' => 1], // 高级球
    ];

    foreach ($items_to_add as $item_info) {
        // 检查是否已拥有该物品
        $existing = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_myitem') . " WHERE uid = %d AND itemid = %d",
            $uid,
            $item_info['typeid']
        ));

        if ($existing) {
            // 更新数量
            DB::query(pm_sql(
                "UPDATE " . pm_table('pm_myitem') . " SET num = num + %d WHERE id = %d",
                $item_info['nums'],
                $existing['id']
            ));
        } else {
            // 插入新物品
            DB::query(pm_sql(
                "INSERT INTO " . pm_table('pm_myitem') . " (uid, itemid, num) VALUES (%d, %d, %d)",
                $uid,
                $item_info['typeid'],
                $item_info['nums']
            ));
        }
    }

    return 0;
}

/**
 * 装备道具类 - 不能直接使用，需要装备
 */
function zb01($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb02($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb03($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb04($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb05($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb06($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb07($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb08($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb09($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb10($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb11($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb12($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb13($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb14($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb15($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb16($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb17($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb18($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb19($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb20($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb21($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb22($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb23($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb24($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb25($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb26($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb27($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb28($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb29($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb30($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb31($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb32($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb33($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb34($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb35($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb36($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb37($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb38($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb39($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb40($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb41($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb42($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb43($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb44($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb45($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb46($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb47($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb48($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb49($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb50($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb51($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb52($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb53($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb54($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb55($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb56($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb57($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb58($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb59($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb60($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb61($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb62($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}
function zb63($petid, $itemname, $iatk = 0, $idef = 0, $ispatk = 0, $ispdef = 0, $isd = 0, $ihp = 0)
{
    return _equipment_item($petid, $itemname);
}

/**
 * 装备道具处理 - 提示用户在装备页面使用
 */
function _equipment_item($petid, $itemname)
{
    // 装备道具需要通过装备页面使用，这里返回错误提示
    return 1;
}

/**
 * 回复药类（直接回复HP的类型）
 */
function hp($petid, $itemname, $addhp = 20)
{
    return _heal_hp_item($petid, $addhp);
}

function hp1($petid, $itemname, $addhp = 50)
{
    return _heal_hp_item($petid, $addhp);
}

function hp2($petid, $itemname, $addhp = 100)
{
    return _heal_hp_item($petid, $addhp);
}

function hp3($petid, $itemname, $addhp = 200)
{
    return _heal_hp_item($petid, $addhp);
}

function hp4($petid, $itemname, $addhp = 999)
{
    return _heal_hp_item($petid, $addhp);
}

/**
 * 内部函数：回复HP
 */
function _heal_hp_item($petid, $addhp)
{
    global $_G;
    $uid = $_G['uid'];

    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $petid,
        $uid
    ));

    if (!$pet) {
        return 1;
    }

    $max_hp = api_calculate_pokemon_max_hp($pet);
    $current_hp = (int)$pet['hp'];

    if ($current_hp >= $max_hp) {
        return 1; // HP已满
    }

    $new_hp = min($max_hp, $current_hp + $addhp);

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
        $new_hp,
        $petid
    ));

    return 0;
}
