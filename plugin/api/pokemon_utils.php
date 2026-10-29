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
 *   create_new_pokemon_data($pokemon_data, $level, $uid, $nickname, $ivs) - 创建新宠物完整数据
 *   pm_quality_of_pokemon($pm) - 宠物品质摘要（总个体值 + 档位）
 *   pm_roll_ivs_for_rank($rank) - 掷出指定品质档位的六项个体值
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
 * 宠物品质体系（依据六项个体值总和判定）
 *
 * 沿用旧版截图中的阶梯（总个体值满值 186 = 6 × 31）：
 *   E 0-9、D 10-69、C 70-99、B 100-119、A 120-149、S 150-169、
 *   SS 170-185、SS+ 恰好 186。
 *
 * 历史宠物的个体值可能突破单项 31 / 总和 186 的上限（例如活动发放的
 * 雪拉比），此时档位记为 9 = UnKnow（未知）。该档位只用于前台展示，
 * 后台发放与活动地图门槛仍限定在 1-8 档。
 *
 * 该体系同时供前台展示、后台发放与活动地图门槛使用；
 * Rust 侧 rust/utils/src/functions/quality.rs 维护了同一套阈值，
 * 两边改动必须同步。
 */

function pm_quality_grades()
{
    return [1 => 'E', 2 => 'D', 3 => 'C', 4 => 'B', 5 => 'A', 6 => 'S', 7 => 'SS', 8 => 'SS+', 9 => 'UnKnow'];
}

function pm_quality_thresholds()
{
    // 档位 => 进入该档所需的最低总个体值（UnKnow 无下界，由 pm_quality_rank() 单独判定）
    return [1 => 0, 2 => 10, 3 => 70, 4 => 100, 5 => 120, 6 => 150, 7 => 170, 8 => 186];
}

/**
 * 计算宠物实例（pm_mypm 行）的六项个体值总和
 */
function pm_iv_total($pm)
{
    return intval($pm['hpg']) + intval($pm['atkg']) + intval($pm['defg'])
        + intval($pm['spatkg']) + intval($pm['spdefg']) + intval($pm['sdg']);
}

/**
 * 计算宠物实例（pm_mypm 行）的六项努力值总和（上限 510）
 */
function pm_ev_total($pm)
{
    return intval($pm['hpn']) + intval($pm['atkn']) + intval($pm['defn'])
        + intval($pm['spatkn']) + intval($pm['spdefn']) + intval($pm['sdn']);
}

/**
 * 总个体值 -> 品质档位 (1=E ... 8=SS+，9=UnKnow)
 *
 * 超过满值 186 的历史个体值一律归入 UnKnow。
 */
function pm_quality_rank($iv_total)
{
    $iv_total = intval($iv_total);
    if ($iv_total > 186) {
        return 9;
    }
    $thresholds = pm_quality_thresholds();
    $rank = 1;
    foreach ($thresholds as $r => $min) {
        if ($iv_total >= $min) {
            $rank = $r;
        }
    }
    return $rank;
}

/**
 * 档位 -> 品质名；0 或越界返回空串（表示无品质要求）
 *
 * 1-8 依次为 E..SS+，9 为 UnKnow。
 */
function pm_quality_name($rank)
{
    $grades = pm_quality_grades();
    $rank = intval($rank);
    return isset($grades[$rank]) ? $grades[$rank] : '';
}

/**
 * 宠物实例的品质摘要
 *
 * @return array {iv_total, rank, grade}
 */
function pm_quality_of_pokemon($pm)
{
    $iv_total = pm_iv_total($pm);
    $rank = pm_quality_rank($iv_total);
    return [
        'iv_total' => $iv_total,
        'rank' => $rank,
        'grade' => pm_quality_name($rank),
    ];
}

/**
 * 掷出一组落在指定品质档位内的六项个体值 (0-31)
 *
 * 档位按旧版阶梯划分：E 0-9 ... SS 170-185、SS+ 恒为 186（六项全 31）。
 * 采用“定和分配 + 随机搬移”策略：先在档位总区间内取目标总和并均分，
 * 再在保持总和不变的前提下随机搬移若干点，保证结果一定落在目标档位。
 *
 * 档位参数会收敛到 1-8：UnKnow（9）只描述历史溢出数据，
 * 无法通过掷取生成，因此不会被发放。
 */
function pm_roll_ivs_for_rank($rank)
{
    $thresholds = pm_quality_thresholds();
    $rank = max(1, min(8, intval($rank)));
    $min = $thresholds[$rank];
    $max = ($rank < 8) ? $thresholds[$rank + 1] - 1 : 186;
    $target = rand($min, $max);

    $keys = ['hp', 'atk', 'def', 'spatk', 'spdef', 'speed'];
    $base = intval($target / 6);
    $ivs = [];
    foreach ($keys as $key) {
        $ivs[$key] = $base;
    }
    $remainder = $target - $base * 6;
    for ($i = 0; $i < $remainder; $i++) {
        $ivs[$keys[$i]]++;
    }

    // 随机搬移：在不越界的前提下把点数从一项搬到另一项，制造随机分布
    for ($round = 0; $round < 24; $round++) {
        $from = rand(0, 5);
        $to = rand(0, 5);
        if ($from === $to) {
            continue;
        }
        $delta = rand(1, 4);
        if ($ivs[$keys[$from]] - $delta < 0 || $ivs[$keys[$to]] + $delta > 31) {
            continue;
        }
        $ivs[$keys[$from]] -= $delta;
        $ivs[$keys[$to]] += $delta;
    }

    return $ivs;
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
 * @param array $ivs 指定个体值 (可选，键为 hp/atk/def/spatk/spdef/speed，缺省随机 0-31；
 *                   可配合 pm_roll_ivs_for_rank() 按品质档位发放)
 * @return array 完整的宠物数据，可直接用于插入数据库
 */
function create_new_pokemon_data($pokemon_data, $level, $uid, $nickname = null, $ivs = null)
{
    $level = max(1, min(100, intval($level)));
    $pokemon_id = intval($pokemon_data['id']);

    // 计算战斗属性
    $stats = calculate_pokemon_stats($pokemon_data, $level, $ivs);
    $max_hp = $stats['hp'];
    $ivs = $stats['ivs'];
    $evs = $stats['evs'];

    // pm_data.sex is the male weight per thousand; -1 alone is genderless.
    $sex_rand = rand(1, 1000);
    $pokemon_sex = intval($pokemon_data['sex']);
    if ($pokemon_sex >= 0) {
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
        'species_id' => $pokemon_id,
        'pmname' => $pokemon_data['name'],
        'nickname' => $name,
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
