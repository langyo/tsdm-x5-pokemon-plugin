<?php

/**
 * Pokemon Plugin - 宠物数值计算工具函数
 *
 * 提供统一的接口计算宠物属性：血量、六维、经验值等
 *
 * 使用方法:
 *   require_once __DIR__ . '/pokemon_utils.php';
 *
 * 函数:
 *   calculate_pokemon_stats($pokemon_data, $level) - 计算宠物战斗六维数值
 *   calculate_pokemon_max_hp($pokemon_data, $level) - 计算宠物最大血量
 *   get_pokemon_exp_threshold($pokemon_id, $level) - 获取指定等级的经验阈值
 *   create_new_pokemon_data($pokemon_data, $level, $uid, $nickname) - 创建新宠物完整数据
 */

// 禁止直接访问
if (!defined('IN_DISCUZ') && !defined('API_ROUTED')) {
    define('IN_DISCUZ', true);
}

/**
 * 获取宠物经验值表类型
 *
 * @param int $pokemon_id 宠物ID
 * @return int 经验值表类型 (60, 80, 100, 105, 125, 164)
 */
function get_pokemon_exp_type($pokemon_id)
{
    static $exp_types = null;

    if ($exp_types === null) {
        // expdata/ 已被删除，返回空数组，将使用默认类型 100
        $exp_types = [];
    }

    // 同时检查整数键和字符串键
    if (isset($exp_types[$pokemon_id])) {
        return $exp_types[$pokemon_id];
    }
    if (isset($exp_types[(string)$pokemon_id])) {
        return $exp_types[(string)$pokemon_id];
    }
    return 100;
}

/**
 * 获取宠物经验值表数据
 *
 * @param int $pokemon_id 宠物ID
 * @return array 经验值数组 [level => exp_needed]
 */
function get_pokemon_exp_table($pokemon_id)
{
    static $cached_tables = [];

    $type = get_pokemon_exp_type($pokemon_id);

    if (isset($cached_tables[$type])) {
        return $cached_tables[$type];
    }

    // expdata/ 已被删除，使用 api/utils.php 中的默认经验表
    if (function_exists('api_get_pet_exp_max_data')) {
        $maxexp = api_get_pet_exp_max_data($pokemon_id);
    } else {
        // 备用：返回空数组
        $maxexp = [];
    }

    $cached_tables[$type] = $maxexp;
    return $maxexp;
}

/**
 * 获取指定等级需要的总经验值
 *
 * @param int $pokemon_id 宠物ID
 * @param int $level 等级 (0-100)
 * @return int 升级到该等级需要的总经验值
 */
function get_pokemon_exp_threshold($pokemon_id, $level)
{
    $exp_table = get_pokemon_exp_table($pokemon_id);

    if ($level < 0) {
        return 0;
    }
    if ($level > 100) {
        $level = 100;
    }

    return isset($exp_table[$level]) ? $exp_table[$level] : 0;
}

/**
 * 计算宠物战斗六维数值（使用标准宝可梦公式）
 *
 * 公式:
 *   能力值 = floor(((2 * 基础值 + IV + EV/4) * 等级 / 100 + boost) * 状态修正)
 *   其中 boost = (10 + level) 对于 HP，5 对于其他属性
 *   状态修正默认 state=1 乘数为 1.0，新宠物创建时不含闪光加成和装备加成
 *   完整计算（含状态/闪光/装备）请使用 calculate_pokemon_full_stats() 或 battle_calc_my_stats()
 *
 * @param array $pokemon_data 宠物基础数据 (来自 pm_data 表)
 * @param int $level 等级
 * @param array $ivs IV值数组 (可选，默认随机生成)
 * @param array $evs EV值数组 (可选，默认随机生成)
 * @return array [hp, atk, def, spatk, spdef, speed]
 */
function calculate_pokemon_stats($pokemon_data, $level, $ivs = null, $evs = null)
{
    $level = max(1, min(100, intval($level)));

    // 基础属性
    $base_hp = intval($pokemon_data['hp']);
    $base_atk = intval($pokemon_data['atk']);
    $base_def = intval($pokemon_data['def']);
    $base_spatk = intval($pokemon_data['spatk']);
    $base_spdef = intval($pokemon_data['spdef']);
    $base_speed = intval($pokemon_data['speed']);

    // IV 值 (Individual Values)
    if ($ivs === null) {
        $iv_hp = rand(0, 31);
        $iv_atk = rand(0, 31);
        $iv_def = rand(0, 31);
        $iv_spatk = rand(0, 31);
        $iv_spdef = rand(0, 31);
        $iv_speed = rand(0, 31);
    } else {
        $iv_hp = isset($ivs['hp']) ? $ivs['hp'] : rand(0, 31);
        $iv_atk = isset($ivs['atk']) ? $ivs['atk'] : rand(0, 31);
        $iv_def = isset($ivs['def']) ? $ivs['def'] : rand(0, 31);
        $iv_spatk = isset($ivs['spatk']) ? $ivs['spatk'] : rand(0, 31);
        $iv_spdef = isset($ivs['spdef']) ? $ivs['spdef'] : rand(0, 31);
        $iv_speed = isset($ivs['speed']) ? $ivs['speed'] : rand(0, 31);
    }

    // EV 值 (Effort Values)
    if ($evs === null) {
        $ev_hp = min(85, intval(rand(0, 85) * $level / 100));
        $ev_atk = min(85, intval(rand(0, 85) * $level / 100));
        $ev_def = min(85, intval(rand(0, 85) * $level / 100));
        $ev_spatk = min(85, intval(rand(0, 85) * $level / 100));
        $ev_spdef = min(85, intval(rand(0, 85) * $level / 100));
        $ev_speed = min(85, intval(rand(0, 85) * $level / 100));
    } else {
        $ev_hp = isset($evs['hp']) ? min(85, $evs['hp']) : 0;
        $ev_atk = isset($evs['atk']) ? min(85, $evs['atk']) : 0;
        $ev_def = isset($evs['def']) ? min(85, $evs['def']) : 0;
        $ev_spatk = isset($evs['spatk']) ? min(85, $evs['spatk']) : 0;
        $ev_spdef = isset($evs['spdef']) ? min(85, $evs['spdef']) : 0;
        $ev_speed = isset($evs['speed']) ? min(85, $evs['speed']) : 0;
    }

    // 计算 HP (HP公式不同: +10 + level 而非 +5)
    $hp = floor(((2 * $base_hp + $iv_hp + $ev_hp / 4) * $level / 100 + 10 + $level));

    // 计算其他属性
    $atk = floor(((2 * $base_atk + $iv_atk + $ev_atk / 4) * $level / 100 + 5));
    $def = floor(((2 * $base_def + $iv_def + $ev_def / 4) * $level / 100 + 5));
    $spatk = floor(((2 * $base_spatk + $iv_spatk + $ev_spatk / 4) * $level / 100 + 5));
    $spdef = floor(((2 * $base_spdef + $iv_spdef + $ev_spdef / 4) * $level / 100 + 5));
    $speed = floor(((2 * $base_speed + $iv_speed + $ev_speed / 4) * $level / 100 + 5));

    return [
        'hp' => $hp,
        'atk' => $atk,
        'def' => $def,
        'spatk' => $spatk,
        'spdef' => $spdef,
        'speed' => $speed,
        'ivs' => [
            'hp' => $iv_hp,
            'atk' => $iv_atk,
            'def' => $iv_def,
            'spatk' => $iv_spatk,
            'spdef' => $iv_spdef,
            'speed' => $iv_speed,
        ],
        'evs' => [
            'hp' => $ev_hp,
            'atk' => $ev_atk,
            'def' => $ev_def,
            'spatk' => $ev_spatk,
            'spdef' => $ev_spdef,
            'speed' => $ev_speed,
        ],
    ];
}

/**
 * 计算宠物最大血量
 *
 * @param array $pokemon_data 宠物基础数据
 * @param int $level 等级
 * @return int 最大HP
 */
function calculate_pokemon_max_hp($pokemon_data, $level)
{
    $stats = calculate_pokemon_stats($pokemon_data, $level);
    return $stats['hp'];
}

/**
 * 计算宠物初始经验值
 *
 * @param int $pokemon_id 宠物ID
 * @param int $level 目标等级
 * @return int 初始经验值
 */
function calculate_initial_exp($pokemon_id, $level)
{
    return get_pokemon_exp_threshold($pokemon_id, $level - 1);
}

/**
 * 创建新宠物的完整数据
 *
 * 用于新用户注册、捕捉宠物等场景
 *
 * 数据库字段说明 (pm_mypm 表):
 * - hp: 当前HP（最大HP通过 api_calculate_pokemon_max_hp() 计算）
 * - hpg, atkg, defg, spatkg, spdefg, sdg: IV值 (Individual Values, 0-31)
 * - hpn, atkn, defn, spatkn, spdefn, sdn: EV值 (Effort Values, 0-255)
 * - level, exp: 等级和经验值
 * - sex: 性别 (0=无, 1=雄, 2=雌)
 * - sx: 宠物属性
 * - good: 亲密度 (0-255)
 * - state: 状态 (1=正常)
 * - site: 位置 (1=首位/主力)
 *
 * @param array $pokemon_data 宠物基础数据 (来自 pm_data 表)
 * @param int $level 初始等级
 * @param int $uid 用户ID
 * @param string $nickname 昵称 (可选，默认使用宠物名)
 * @return array 完整的宠物数据，可直接用于插入数据库
 */
function create_new_pokemon_data($pokemon_data, $level, $uid, $nickname = null)
{
    $level = max(1, min(100, intval($level)));
    $pokemon_id = intval($pokemon_data['id']);

    // 计算战斗属性
    $stats = calculate_pokemon_stats($pokemon_data, $level);
    $max_hp = $stats['hp'];
    $ivs = $stats['ivs'];
    $evs = $stats['evs'];

    // 随机性别
    $sex_rand = rand(1, 100);
    $pokemon_sex = intval($pokemon_data['sex']);
    if ($pokemon_sex > 0) {
        $sex = ($sex_rand <= $pokemon_sex) ? 1 : 2;
    } else {
        $sex = 0;
    }

    // 初始经验值
    $initial_exp = calculate_initial_exp($pokemon_id, $level);

    // 昵称
    $name = !empty($nickname) ? $nickname : $pokemon_data['name'];

    return [
        'uid' => $uid,
        'pmno' => $pokemon_id,
        'pmname' => $pokemon_data['name'],
        'nowname' => $name,
        'sex' => $sex,
        'sx' => $pokemon_data['xs'],
        'level' => $level,
        'exp' => $initial_exp,
        'hp' => $max_hp,
        // IV值 (g后缀字段)
        'hpg' => $ivs['hp'],
        'atkg' => $ivs['atk'],
        'defg' => $ivs['def'],
        'spatkg' => $ivs['spatk'],
        'spdefg' => $ivs['spdef'],
        'sdg' => $ivs['speed'],
        // EV值 (n后缀字段)
        'hpn' => $evs['hp'],
        'atkn' => $evs['atk'],
        'defn' => $evs['def'],
        'spatkn' => $evs['spatk'],
        'spdefn' => $evs['spdef'],
        'sdn' => $evs['speed'],
        // 其他属性
        'good' => 70,
        'state' => 1,
    ];
}

/**
 * 将宠物数据数组转换为 SQL INSERT 语句
 *
 * @param array $pokemon_data create_new_pokemon_data() 返回的数据
 * @param int $site 位置 (1=首位出战, 2+=后备)
 * @param string $table_name 表名（可选，默认 pm_mypm）
 * @return string SQL INSERT 语句
 */
function build_pokemon_insert_sql($pokemon_data, $site = 1, $table_name = 'pm_mypm')
{
    $pokemon_data['site'] = $site;

    $field_list = implode(', ', array_keys($pokemon_data));
    $value_list = implode(', ', array_map(function ($v) {
        return is_numeric($v) ? $v : "'" . addslashes($v) . "'";
    }, $pokemon_data));

    // 如果 pm_table 函数存在，使用它；否则直接使用表名
    if (function_exists('pm_table')) {
        return "INSERT INTO " . pm_table($table_name) . " ($field_list) VALUES ($value_list)";
    } else {
        return "INSERT INTO `$table_name` ($field_list) VALUES ($value_list)";
    }
}
