<?php

/**
 * API 工具函数库
 *
 * 从 data/function.func.php 迁移并重构
 * 修复 SQL 注入漏洞，添加输入验证
 */

defined('IN_DISCUZ') || exit('Access Denied');

/**
 * 状态修正系数
 * 从 data/abilityrevise.inc.php 迁移
 */
$statehp = array('1' => '1', '2' => '0.9', '3' => '0.8', '4' => '0.5', '5' => '0.9', '6' => '0.8', '7' => '0.5', '8' => '1.1', '9' => '1.2', '10' => '2', '11' => '0.8', '12' => '1.3', '13' => '1.1', '14' => '1.3', '15' => '0.7', '16' => '1', '17' => '1.5', '18' => '1', '19' => '1', '20' => '1');
$stateatk = array('1' => '1', '2' => '0.9', '3' => '0.8', '4' => '0.5', '5' => '0.9', '6' => '0.8', '7' => '0.5', '8' => '1.2', '9' => '1.5', '10' => '2', '11' => '1', '12' => '1', '13' => '1', '14' => '1', '15' => '0.7', '16' => '0.8', '17' => '0.6', '18' => '1.5', '19' => '2.5', '20' => '1');
$statespatk = array('1' => '1', '2' => '0.9', '3' => '0.8', '4' => '0.5', '5' => '0.9', '6' => '0.8', '7' => '0.5', '8' => '1.2', '9' => '1.5', '10' => '2', '11' => '1', '12' => '1', '13' => '1', '14' => '1', '15' => '0.7', '16' => '0.8', '17' => '0.6', '18' => '1.5', '19' => '2.5', '20' => '1',);
$statedef = array('1' => '1', '2' => '0.9', '3' => '0.8', '4' => '0.5', '5' => '0.9', '6' => '0.8', '7' => '0.5', '8' => '1', '9' => '0.5', '10' => '2', '11' => '1', '12' => '1', '13' => '1', '14' => '1.2', '15' => '0.7', '16' => '1.2', '17' => '1.5', '18' => '0.8', '19' => '0.3', '20' => '1',);
$statespdef = array('1' => '1', '2' => '0.9', '3' => '0.8', '4' => '0.5', '5' => '0.9', '6' => '0.8', '7' => '0.5', '8' => '1', '9' => '0.5', '10' => '2', '11' => '1', '12' => '1', '13' => '1', '14' => '1.2', '15' => '0.7', '16' => '1.2', '17' => '1.5', '18' => '0.8', '19' => '0.3', '20' => '1',);
$statesd = array('1' => '1', '2' => '0.9', '3' => '0.8', '4' => '0.5', '5' => '0.9', '6' => '0.8', '7' => '0.5', '8' => '1', '9' => '1.2', '10' => '2', '11' => '1', '12' => '1', '13' => '1.2', '14' => '1.6', '15' => '0.7', '16' => '1', '17' => '1', '18' => '1', '19' => '1', '20' => '1',);

/**
 * 获取物品的功能模块名（item_modules.php 中的函数名）
 *
 * X2 旧库与 X5 导出/种子数据中模块名不在 module 列：
 * 强化/PP 类道具在 sitemname（如 pp5、hpn10），其余在 tpname（如 szs、lvupitem）。
 * 这里按 module → sitemname → tpname 的顺序回退，保证旧数据物品仍能正确分发。
 *
 * @param array $item_data pm_itemdata 行
 * @return string 模块名（可能为空字符串）
 */
function api_get_item_module($item_data)
{
    if (!is_array($item_data)) {
        return '';
    }

    foreach (['module', 'sitemname', 'tpname'] as $col) {
        $val = isset($item_data[$col]) ? trim(strval($item_data[$col])) : '';
        if ($val !== '' && preg_match('/^[a-z][a-z0-9_]*$/i', $val)) {
            return $val;
        }
    }

    return '';
}

/**
 * 治疗物品回复HP计算
 * 原型：get_item_healed_hp
 *
 * @param array &$pet 宠物数据（引用）
 * @param int &$heal 治疗量（引用）
 * @param int $max 最大HP
 * @return int 新的HP值
 */
function api_get_item_healed_hp(&$pet, &$heal, $max)
{
    if (!is_numeric($heal)) {
        $heal = 0;
    }
    if (!is_numeric($max)) {
        $max = 0;
    }

    $hp = intval($pet['hp']);
    $max = intval($max);
    $heal = intval($heal);

    if ($hp > $max) {
        $hp = $max;
    }
    $_hp = $max - $hp;

    if ($heal == ITEM_HEAL_MAX_HP || $heal >= $_hp) {
        $hp = $max;
        $heal = $_hp;
    } else {
        $hp += $heal;
    }

    $pet['hp'] = strval($hp);
    return $hp;
}

/**
 * 获取用户扩展数据（安全版本）
 * 原型：my_usersdata
 *
 * @param int $uid 用户ID
 * @return array|false 用户数据，失败返回 false
 */
function api_my_usersdata($uid)
{
    $uid = intval($uid);
    if ($uid <= 0) {
        return false;
    }

    $data = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d", $uid));
    if (!$data) {
        // 初始化用户数据（移除未使用的 PVP 相关字段）
        DB::query(pm_sql("INSERT INTO " . pm_table('pm_usersdata') . " (
            uid, npcid, hpg, hp, allure, capture
        ) VALUES (
            %d, '', '', '', '', ''
        )", $uid));
        $data = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d", $uid));
    }
    return $data;
}

/**
 * 获取用户首位宠物（安全版本）
 * 原型：my_pokemon
 *
 * @param string $username 用户名
 * @return array|false 宠物数据，失败返回 false
 */
function api_my_pokemon($username)
{
    global $_G;

    // 通过 username 查找 uid（避免直接使用用户名）
    $userinfo = DB::fetch_first(pm_sql("SELECT uid FROM " . DB::table('common_member') . " WHERE username = %s", $username));
    if (!$userinfo) {
        return false;
    }

    $uid = intval($userinfo['uid']);
    return DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_mypm') . " WHERE uid = %d AND site = 1", $uid));
}

/**
 * 获取指定宠物数据（安全版本）
 * 原型：my_pokemon_data
 *
 * @param int $pmid 宠物ID
 * @return array|false 宠物数据，失败返回 false
 */
function api_my_pokemon_data($pmid)
{
    $pmid = intval($pmid);
    if ($pmid <= 0) {
        return false;
    }
    return DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d", $pmid));
}

/**
 * 解析宠物装备物品并计算加成（安全版本）
 * 原型：parse_pet_wear_items
 *
 * @param array &$pet 宠物数据（引用）
 * @param bool $html 是否返回HTML格式
 * @param mixed &$max_hp 最大HP引用
 * @return array|void 装备加成或HTML
 */
function api_parse_pet_wear_items(&$pet, $html, &$max_hp = null)
{
    $arr = ($max_hp !== null) ? [&$max_hp, 0, 0, 0, 0, 0] : [];

    for ($i = 1; $i <= 4; ++$i) {
        $iid = isset($pet["equipmentid$i"]) ? intval($pet["equipmentid$i"]) : 0;
        if ($iid > 0) {
            $item = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_myitem') . " WHERE id = %d", $iid));
            if (!$item) {
                continue;
            }

            $data = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_itemdata') . " WHERE id = %d", intval($item['itemid'])));
            if (!$data) {
                continue;
            }

            if ($html) {
                // HTML 模式已弃用，不再使用
                continue;
            }

            // 计算装备加成
            if ($max_hp !== null) {
                $equipment = json_decode($data['equipment'], true) ?: [];
                $arr[0] += intval(isset($equipment['hp']) ? $equipment['hp'] : 0);
                $arr[1] += intval(isset($equipment['atk']) ? $equipment['atk'] : 0);
                $arr[2] += intval(isset($equipment['def']) ? $equipment['def'] : 0);
                $arr[3] += intval(isset($equipment['spatk']) ? $equipment['spatk'] : 0);
                $arr[4] += intval(isset($equipment['spdef']) ? $equipment['spdef'] : 0);
                $arr[5] += intval(isset($equipment['spd']) ? $equipment['spd'] : 0);
            }
        }
    }

    if ($max_hp !== null) {
        return $arr;
    }
}

/**
 * 根据经验值计算等级
 * 原型：get_pet_exp_level
 *
 * @param string|int|array $no_or_data 宝可梦编号或数据
 * @param int $exp 经验值
 * @return int 等级 (0-100)
 */
function api_get_pet_exp_level($no_or_data, $exp)
{
    $exp = intval($exp);
    if ($exp <= 0) {
        return 0;
    }

    // 获取经验表数据
    if (!is_array($no_or_data)) {
        $no_or_data = api_get_pet_exp_max_data($no_or_data);
    }

    $level = 0;
    foreach ($no_or_data as $lv => $v) {
        if ($exp <= $v) {
            $level = $lv;
            if ($exp == $v && !($lv == $v && $v < 1)) {
                ++$level;
            }
            break;
        }
    }

    $level = intval($level);
    return max(0, min($level, 100));
}

/**
 * 获取宝可梦经验表数据
 * 原型：get_pet_exp_max_data
 *
 * 注意：由于 expdata/ 已被删除，此函数现在返回默认经验表
 * 如果需要特定的经验表，需要重新实现或从数据库读取
 *
 * @param string $no 宝可梦编号
 * @return array 经验表 [等级 => 经验值]
 */
function api_get_pet_exp_max_data($no)
{
    // 默认经验表（标准/快速经验曲线）
    // 这是一个简化版本，实际应该根据宝可梦类型使用不同的经验表
    static $default_exp_table = [
        1 => 0,
        2 => 10,
        3 => 33,
        4 => 80,
        5 => 156,
        6 => 276,
        7 => 453,
        8 => 706,
        9 => 1061,
        10 => 1539,
        11 => 2137,
        12 => 2831,
        13 => 3647,
        14 => 4562,
        15 => 5559,
        16 => 6675,
        17 => 7906,
        18 => 9169,
        19 => 10499,
        20 => 11920,
        21 => 13398,
        22 => 14948,
        23 => 16570,
        24 => 18268,
        25 => 20052,
        26 => 21913,
        27 => 23859,
        28 => 25890,
        29 => 28008,
        30 => 30217,
        31 => 32523,
        32 => 34931,
        33 => 37436,
        34 => 40041,
        35 => 42747,
        36 => 45556,
        37 => 48469,
        38 => 51488,
        39 => 54614,
        40 => 57838,
        41 => 61172,
        42 => 64617,
        43 => 68173,
        44 => 71818,
        45 => 75558,
        46 => 79424,
        47 => 83409,
        48 => 87495,
        49 => 91689,
        50 => 95995,
        51 => 100414,
        52 => 104946,
        53 => 109593,
        54 => 114355,
        55 => 119212,
        56 => 124196,
        57 => 129296,
        58 => 134514,
        59 => 139850,
        60 => 145306,
        61 => 150883,
        62 => 156579,
        63 => 162394,
        64 => 168328,
        65 => 174381,
        66 => 180556,
        67 => 186851,
        68 => 193267,
        69 => 199801,
        70 => 206456,
        71 => 213239,
        72 => 220141,
        73 => 227171,
        74 => 234332,
        75 => 241625,
        76 => 249053,
        77 => 256616,
        78 => 264313,
        79 => 272146,
        80 => 280114,
        81 => 288220,
        82 => 296463,
        83 => 304842,
        84 => 313357,
        85 => 322009,
        86 => 330794,
        87 => 339718,
        88 => 348781,
        89 => 357985,
        90 => 367432,
        91 => 377020,
        92 => 386749,
        93 => 396612,
        94 => 406619,
        95 => 416770,
        96 => 427078,
        97 => 437543,
        98 => 448155,
        99 => 458965,
        100 => 469987,
    ];

    // 慢速经验曲线（部分宝可梦使用）
    static $slow_exp_table = [
        1 => 0,
        2 => 15,
        3 => 52,
        4 => 122,
        5 => 237,
        6 => 406,
        7 => 640,
        8 => 953,
        9 => 1351,
        10 => 1854,
        11 => 2471,
        12 => 3208,
        13 => 4078,
        14 => 5082,
        15 => 6222,
        16 => 7505,
        17 => 8935,
        18 => 10518,
        19 => 12212,
        20 => 14028,
        21 => 15971,
        22 => 18045,
        23 => 20250,
        24 => 22558,
        25 => 24972,
        26 => 27507,
        27 => 30168,
        28 => 32947,
        29 => 35852,
        30 => 38839,
        // ... 省略中间等级
        100 => 1640000,
    ];

    // TODO: 根据宝可梦编号返回正确的经验表
    // 目前使用标准经验表
    return $default_exp_table;
}

/**
 * 获取指定等级的最大经验值
 * 原型：get_pet_exp_max
 *
 * @param array $data 经验表数据
 * @param int $level 等级
 * @return int 该等级所需经验值
 */
function api_get_pet_exp_max($data, $level)
{
    $level = intval($level);
    if ($level < 1) {
        return 0;
    }
    if ($level > 100) {
        $level = 100;
    }
    return isset($data[$level]) ? intval($data[$level]) : 0;
}

/**
 * 获取状态修正系数
 *
 * @param string $state 状态编号
 * @param string $stat 属性类型 (hp, atk, spatk, def, spdef, spd)
 * @return float 修正系数
 */
function api_get_state_modifier($state, $stat = 'hp')
{
    global ${'state' . $stat};
    $var_name = 'state' . $stat;

    if (isset(${$var_name}[$state])) {
        return floatval(${$var_name}[$state]);
    }
    return 1.0;
}

/**
 * 实用工具：从数组获取多个键值
 * 原型：pkg_array_gets
 *
 * @param array $arr 源数组
 * @param string ...$keys 键名
 * @return array 提取的键值对
 */
function api_array_gets($arr, ...$keys)
{
    $result = [];
    foreach ($keys as $key) {
        $result[$key] = isset($arr[$key]) ? $arr[$key] : null;
    }
    return $result;
}

/**
 * 计算宠物的最大 HP（统一函数）
 *
 * 这是唯一正确的 max_hp 计算方式，所有其他代码都应该使用这个函数
 * 而不是自己计算（pm_mypm 表中不存储 maxhp，以此函数计算结果为准）
 *
 * @param array $pm 宠物数据（来自 pm_mypm 表）
 * @param array|null $pm_data 宠物基础数据（来自 pm_data 表），如果为 null 则自动查询
 * @return int 最大 HP
 */
function api_calculate_pokemon_max_hp($pm, $pm_data = null, $include_equipment = true)
{
    global $statehp;

    $pmno = (int) $pm['species_id'];

    if ($pm_data === null) {
        $pm_data = DB::fetch_first(pm_sql(
            "SELECT hp FROM " . pm_table('pm_data') . " WHERE id = %d",
            $pmno
        ));
    }

    $base_hp = $pm_data ? (int) $pm_data['hp'] : 50;

    $level = (int) $pm['level'];
    $hpg = (int) $pm['hpg'];
    $hpn = (int) $pm['hpn'];
    $state = (int) $pm['state'];
    $sg = (int) $pm['is_shiny'];

    $state_multiplier = isset($statehp[$state]) ? (float)$statehp[$state] : 1.0;

    $flash_boost = ($sg == 1) ? 2 : 1;

    $boost = (10 + $level) * $flash_boost;
    $max_hp = (int) floor(((2 * $base_hp + $hpg + $hpn / 4) * $level / 100 + $boost) * $state_multiplier);

    if ($include_equipment) {
        $equipment_hp = 0;
        for ($i = 1; $i <= 4; $i++) {
            $iid = isset($pm["equipmentid$i"]) ? (int) $pm["equipmentid$i"] : 0;
            if ($iid > 0) {
                $item = DB::fetch_first(pm_sql(
                    "SELECT i.equipment FROM " . pm_table('pm_myitem') . " m "
                    . "LEFT JOIN " . pm_table('pm_itemdata') . " i ON m.itemid=i.id "
                    . "WHERE m.id=%d", $iid
                ));
                if ($item) {
                    $equipment = json_decode($item['equipment'], true) ?: [];
                    $equipment_hp += (int) (isset($equipment['hp']) ? $equipment['hp'] : 0);
                }
            }
        }
        $max_hp += $equipment_hp;
    }

    return max(1, $max_hp);
}

/**
 * 验证并纠正宠物血量
 * 确保 HP 在 [0, max_hp] 范围内，如果超出则立即纠正到数据库
 *
 * @param array $pm 宠物数据（必须包含 id, uid 字段）
 * @param int|null $current_hp 当前 HP（如果为 null 则从 $pm 读取）
 * @param int|null $max_hp 最大 HP（如果为 null 则自动计算）
 * @param array|null $pm_data 宠物基础数据（如果为 null 则自动查询）
 * @return array 纠正后的数据 ['hp' => 纠正后的HP, 'max_hp' => 最大HP, 'corrected' => 是否被纠正]
 */
function api_validate_and_correct_hp(&$pm, $current_hp = null, $max_hp = null, $pm_data = null)
{
    $pet_id = isset($pm['id']) ? (int) $pm['id'] : 0;
    $uid = isset($pm['uid']) ? (int) $pm['uid'] : 0;

    if ($pet_id <= 0) {
        return ['hp' => 0, 'max_hp' => 1, 'corrected' => false];
    }

    // 获取当前 HP
    if ($current_hp === null) {
        $current_hp = isset($pm['hp']) ? (int) $pm['hp'] : 0;
    }

    // 计算最大 HP
    if ($max_hp === null) {
        $max_hp = api_calculate_pokemon_max_hp($pm, $pm_data);
    }

    $corrected = false;
    $corrected_hp = $current_hp;

    // 检查并纠正 HP
    if ($corrected_hp < 0) {
        $corrected_hp = 0;
        $corrected = true;
    } elseif ($corrected_hp > $max_hp) {
        $corrected_hp = $max_hp;
        $corrected = true;
    }

    // 如果需要纠正，更新数据库
    if ($corrected) {
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
            $corrected_hp,
            $pet_id
        ));
        // 更新传入的宠物数据
        $pm['hp'] = strval($corrected_hp);
    }

    return [
        'hp' => $corrected_hp,
        'max_hp' => $max_hp,
        'corrected' => $corrected
    ];
}
