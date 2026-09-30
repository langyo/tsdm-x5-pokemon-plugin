<?php

/**
 * 宝可梦相关API
 *
 * 端点:
 * - GET  ?action=list      获取用户宠物列表
 * - GET  ?action=detail&id=xxx  获取宠物详情
 * - POST ?action=rename    重命名宠物
 * - POST ?action=release   放生宠物
 * - GET  ?action=update_state  更新宠物状态（随机触发）
 */

// 加载Discuz环境
if (!defined('IN_DISCUZ')) {
    require_once __DIR__ . '/bootstrap.php';
}

// 加载API辅助函数
require_once __DIR__ . '/index.php';

// 加载API工具函数（包含经验计算、状态修正等）
require_once __DIR__ . '/utils.php';

// 加载Discuz和插件核心
global $_G;

$action = get_param('action', 'list');

switch ($action) {
    case 'list':
        api_get_pokemon_list();
        break;

    case 'detail':
        api_get_pokemon_detail();
        break;

    case 'rename':
        api_rename_pokemon();
        break;

    case 'release':
        api_release_pokemon();
        break;

    case 'learnable_skills':
        api_get_learnable_skills();
        break;

    case 'forget_skill':
        api_forget_skill();
        break;

    case 'learn_skill':
        api_learn_skill();
        break;

    case 'equipment':
        api_get_equipment();
        break;

    case 'equip_item':
        api_equip_item();
        break;

    case 'unequip_item':
        api_unequip_item();
        break;

    case 'update_state':
        api_update_pokemon_state();
        break;

    case 'move_pokemon':
        api_move_pokemon();
        break;

    case 'swap_pokemon':
        api_swap_pokemon();
        break;

    case 'set_first':
        api_set_first_pokemon();
        break;

    default:
        api_error('Invalid action: ' . $action, 400);
}

/**
 * 获取宠物状态文本
 */
function get_pokemon_state_text($state)
{
    switch ((int) $state) {
        case 1:
            return '正常';
        case 2:
            return '生病';
        case 3:
            return '生病';
        case 4:
            return '生病';
        case 5:
            return '饥饿';
        case 6:
            return '饥饿';
        case 7:
            return '疲惫';
        case 8:
            return '兴奋';
        case 9:
            return '兴奋';
        case 10:
            return '兴奋';
        case 11:
            return '受伤';
        case 12:
            return '开心';
        case 13:
            return '开心';
        case 14:
            return '开心';
        case 15:
            return '惊慌';
        case 16:
            return '自恋';
        case 17:
            return '自恋';
        case 18:
            return '愤怒';
        case 19:
            return '愤怒';
        case 20:
            return '虚弱';
        case 21:
            return '虚弱';
        case 22:
            return '虚弱';
        case 0:
        default:
            return '濒危';
    }
}

/**
 * 获取宠物状态 CSS 类名
 */
function get_pokemon_state_class($state)
{
    switch ((int) $state) {
        case 1:
            return 'normal';
        case 2:
        case 3:
        case 4:
            return 'sick';
        case 5:
        case 6:
            return 'hungry';
        case 7:
            return 'tired';
        case 8:
        case 9:
        case 10:
            return 'excited';
        case 11:
            return 'hurt';
        case 12:
        case 13:
        case 14:
            return 'happy';
        case 15:
            return 'shock';
        case 16:
        case 17:
            return 'self-love';
        case 18:
        case 19:
            return 'angry';
        case 20:
        case 21:
        case 22:
            return 'weak';
        case 0:
        default:
            return 'critical';
    }
}

/**
 * 检查并更新宠物的濒危状态
 * 
 * 规则：
 * - 当 HP <= 0 时，自动转为濒危状态（state = 0）
 * - 当 HP > 0 且当前是濒危状态时，自动解除濒危，转为虚弱状态（state = 20）
 * 
 * @param array $pm 宠物数据
 * @return array ['state_changed' => bool, 'new_state' => int, 'old_state' => int]
 */
function check_and_update_critical_state($pm)
{
    $pet_id = (int) $pm['id'];
    $uid = (int) $pm['uid'];
    $current_hp = (int) $pm['hp'];
    $current_state = (int) $pm['state'];
    $timestamp = time();

    $state_changed = false;
    $new_state = $current_state;

    if ($current_hp <= 0 && $current_state !== 0) {
        // HP <= 0，自动转为濒危状态
        $new_state = 0;
        $state_changed = true;
        DB::query("UPDATE " . pm_table('pm_mypm') . " 
            SET state = 0, statetime = $timestamp 
            WHERE id = $pet_id AND uid = $uid");
    } elseif ($current_hp > 0 && $current_state === 0) {
        // HP > 0 且当前是濒危，自动解除濒危，转为虚弱状态
        $new_state = 20;
        $state_changed = true;
        DB::query("UPDATE " . pm_table('pm_mypm') . " 
            SET state = 20, statetime = $timestamp 
            WHERE id = $pet_id AND uid = $uid");
    }

    return [
        'state_changed' => $state_changed,
        'new_state' => $new_state,
        'old_state' => $current_state
    ];
}

/**
 * 虚弱状态降级
 * 
 * 虚弱状态等级：20(最高) -> 21 -> 22 -> 1(正常)
 * 每次使用血瓶会降低一个等级
 * 
 * @param int $current_state 当前状态
 * @return int 新状态
 */
function downgrade_weak_state($current_state)
{
    switch ((int) $current_state) {
        case 20:
            return 21;
        case 21:
            return 22;
        case 22:
            return 1;
        default:
            return $current_state;
    }
}

/**
 * 获取宠物经验值表类型
 * 注意：expdata/ 已被删除，现在统一使用 api_get_pet_exp_max_data()
 */
function get_pokemon_exp_type_local($pokemon_id)
{
    static $exp_types = null;

    if ($exp_types === null) {
        // expdata/ 已被删除，返回空数组，将使用默认类型 100
        $exp_types = [];
    }

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
 * 注意：expdata/ 已被删除，现在使用 api/utils.php 中的默认经验表
 */
function get_pokemon_exp_table_local($pokemon_id)
{
    static $cached_tables = [];

    $type = get_pokemon_exp_type_local($pokemon_id);

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
 * 计算宠物的完整属性（基础属性 + 装备加成）
 * 
 * @param array $pm 宠物数据（来自 pm_mypm 表）
 * @param array|null $pm_data 宠物基础数据（来自 pm_data 表），如果为 null 则自动查询
 * @return array 包含 base_hp, total_hp, equipment_hp, atk, def, spatk, spdef, speed
 */
function calculate_pokemon_full_stats($pm, $pm_data = null)
{
    global $statehp, $stateatk, $statedef, $statespatk, $statespdef, $statespeed;

    $pmno = (int) $pm['species_id'];
    $level = (int) $pm['level'];

    if ($pm_data === null) {
        $pm_data = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d", $pmno));
    }

    $result = [
        'base_hp' => 1,
        'equipment_hp' => 0,
        'total_hp' => 1,
        'equipment_atk' => 0,
        'equipment_def' => 0,
        'equipment_spatk' => 0,
        'equipment_spdef' => 0,
        'equipment_speed' => 0,
        'base_atk' => 0,
        'base_def' => 0,
        'base_spatk' => 0,
        'base_spdef' => 0,
        'base_speed' => 0,
        'total_atk' => 0,
        'total_def' => 0,
        'total_spatk' => 0,
        'total_spdef' => 0,
        'total_speed' => 0,
    ];

    if (!$pm_data) {
        return $result;
    }

    // 状态修正系数已在 utils.php 中定义（全局变量）

    $iv = [
        'hp' => (int) $pm['hpg'],
        'atk' => (int) $pm['atkg'],
        'def' => (int) $pm['defg'],
        'spatk' => (int) $pm['spatkg'],
        'spdef' => (int) $pm['spdefg'],
        'speed' => (int) $pm['sdg'],
    ];

    $ev = [
        'hp' => (int) $pm['hpn'],
        'atk' => (int) $pm['atkn'],
        'def' => (int) $pm['defn'],
        'spatk' => (int) $pm['spatkn'],
        'spdef' => (int) $pm['spdefn'],
        'speed' => (int) $pm['sdn'],
    ];

    $state = (int) $pm['state'];
    $sg = (int) $pm['is_shiny'];
    $flash_boost = ($sg == 1) ? 2 : 1;

    $stats_config = [
        'hp',
        'atk',
        'def',
        'spatk',
        'spdef',
        'speed'
    ];

    foreach ($stats_config as $stat_name) {
        $base = (int) $pm_data[$stat_name];
        $stat_iv = $iv[$stat_name];
        $stat_ev = $ev[$stat_name];

        $is_hp = ($stat_name === 'hp');
        $boost = $is_hp ? 10 : 5;
        $state_multiplier = 1.0;

        if ($is_hp) {
            $state_multiplier = isset($statehp[$state]) ? (float)$statehp[$state] : 1.0;
        } else {
            $state_var = 'state' . $stat_name;
            $state_multiplier = isset($GLOBALS[$state_var][$state]) ? (float)$GLOBALS[$state_var][$state] : 1.0;
        }

        $final_boost = $boost * $flash_boost;

        if ($is_hp) {
            $final_boost += $level;
        }

        $base_value = floor(((2 * $base + $stat_iv + $stat_ev / 4) * $level / 100 + $final_boost) * $state_multiplier);

        if ($is_hp) {
            $result['base_hp'] = max(1, (int) $base_value);
        } else {
            $result['base_' . $stat_name] = (int) $base_value;
        }
    }

    $equipment_bonuses = [
        'hp' => 0,
        'atk' => 0,
        'def' => 0,
        'spatk' => 0,
        'spdef' => 0,
        'speed' => 0,
    ];

    for ($i = 1; $i <= 4; $i++) {
        $equip_id = (int) $pm["equipmentid$i"];

        if ($equip_id > 0) {
            $equip_item = DB::fetch_first(pm_sql(
                "SELECT i.equipment
FROM " . pm_table('pm_myitem') . " m LEFT JOIN " . pm_table('pm_itemdata') . " i ON m.itemid=i.id WHERE m.id=%d",
                $equip_id
            ));

            if ($equip_item) {
                $equipment = json_decode($equip_item['equipment'], true) ?: [];
                $equipment_bonuses['hp'] += (int) (isset($equipment['hp']) ? $equipment['hp'] : 0);
                $equipment_bonuses['atk'] += (int) (isset($equipment['atk']) ? $equipment['atk'] : 0);
                $equipment_bonuses['def'] += (int) (isset($equipment['def']) ? $equipment['def'] : 0);
                $equipment_bonuses['spatk'] += (int) (isset($equipment['spatk']) ? $equipment['spatk'] : 0);
                $equipment_bonuses['spdef'] += (int) (isset($equipment['spdef']) ? $equipment['spdef'] : 0);
                $equipment_bonuses['speed'] += (int) (isset($equipment['spd']) ? $equipment['spd'] : 0);
            }
        }
    }

    $result['equipment_hp'] = $equipment_bonuses['hp'];
    $result['equipment_atk'] = $equipment_bonuses['atk'];
    $result['equipment_def'] = $equipment_bonuses['def'];
    $result['equipment_spatk'] = $equipment_bonuses['spatk'];
    $result['equipment_spdef'] = $equipment_bonuses['spdef'];
    $result['equipment_speed'] = $equipment_bonuses['speed'];

    $result['total_hp'] = max(1, $result['base_hp'] + $result['equipment_hp']);
    $result['total_atk'] = $result['base_atk'] + $result['equipment_atk'];
    $result['total_def'] = $result['base_def'] + $result['equipment_def'];
    $result['total_spatk'] = $result['base_spatk'] + $result['equipment_spatk'];
    $result['total_spdef'] = $result['base_spdef'] + $result['equipment_spdef'];
    $result['total_speed'] = $result['base_speed'] + $result['equipment_speed'];

    return $result;
}

/**
 * 获取用户宠物列表
 */
function api_get_pokemon_list()
{
    require_login();

    global $_G, $statehp;
    $uid = validate_uid($_G['uid']);

    // 查询用户宠物，按 site 排序（首位在前），然后按 id 排序
    $pm_rows = DB::fetch_all(pm_sql("SELECT * FROM " . pm_table('pm_mypm') . " WHERE uid = %d ORDER BY site ASC, id ASC", $uid));

    $pokemons = [];

    foreach ($pm_rows as $pm) {
        $pmno = (int) $pm['species_id'];
        $petid = (int) $pm['id'];

        // 检查并更新濒危状态
        $state_result = check_and_update_critical_state($pm);
        if ($state_result['state_changed']) {
            $pm['state'] = $state_result['new_state'];
        }

        // 查询宠物基础信息
        $info = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d", $pmno));

        // 查询宠物技能（连接 pm_skill 获取详情）
        $skill_rows = DB::fetch_all(pm_sql(
            "SELECT ms.skillid, ms.skillnum, s.name as skill_name, s.element, s.category, s.level_required, s.power, s.max_uses as max_pp "
                . "FROM " . pm_table('pm_myskill') . " ms "
                . "LEFT JOIN " . pm_table('pm_skill') . " s ON ms.skillid = s.id "
                . "WHERE ms.petid = %d LIMIT 4",
            $petid
        ));
        $skills = [];

        foreach ($skill_rows as $sk) {
            $skills[] = [
                'type_id' => (int) $sk['skillid'],
                'pp' => (int) $sk['skillnum'],
                'name' => $sk['skill_name'] ? $sk['skill_name'] : '',
                'skill_type' => $sk['element'] ? $sk['element'] : '',
                'category' => $sk['category'] ? $sk['category'] : '',
                'level' => (int) $sk['level_required'],
                'power' => (int) $sk['power'],
                'max_pp' => (int) $sk['max_pp'],
            ];
        }

        // 填充到4个技能槽
        while (count($skills) < 4) {
            $skills[] = [
                'type_id' => 0,
                'pp' => 0,
                'name' => '',
                'skill_type' => '',
                'category' => '',
                'level' => 0,
                'power' => 0,
                'max_pp' => 0
            ];
        }

        // 计算最大 HP（使用统一计算函数）
        $level = (int) $pm['level'];
        $max_hp = api_calculate_pokemon_max_hp($pm, $info);

        // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
        $hp_validation = api_validate_and_correct_hp($pm, null, $max_hp, $info);
        $pm['hp'] = strval($hp_validation['hp']);

        // 计算经验值字段
        $exp_table = get_pokemon_exp_table_local($pmno);

        $current_exp = isset($pm['exp']) ? (int) $pm['exp'] : 0;

        // exp_for_current_level 是上一级的阈值（当前等级的起始经验）
        // exp_for_next_level 是当前等级的阈值（升到下一级需要的经验）
        $prev_level_key = $level - 1;
        $current_level_threshold = isset($exp_table[$prev_level_key]) ? $exp_table[$prev_level_key] : (isset($exp_table[(string)$prev_level_key]) ? $exp_table[(string)$prev_level_key] : 0);
        $next_level_exp = isset($exp_table[$level]) ? $exp_table[$level] : (isset($exp_table[(string)$level]) ? $exp_table[(string)$level] : 1250000);
        $exp_needed_for_next = max(0, $next_level_exp - $current_exp);

        $pm_state = (int) $pm['state'];
        $pm_affection = (int) $pm['good'];

        $pokemons[] = [
            'id' => $petid,
            'name' => $info ? $info['name'] : '???',
            'nickname' => $pm['nickname'] ? $pm['nickname'] : null,
            'type_id' => $pmno,
            'level' => $level,
            'exp' => (int) $pm['exp'],
            'exp_to_next_level' => $exp_needed_for_next,
            'exp_for_current_level' => $current_level_threshold,
            'exp_for_next_level' => $next_level_exp,
            'hp' => (int) $pm['hp'],
            'max_hp' => $max_hp,
            'gender' => (int) $pm['sex'],
            'is_shiny' => ((int) $pm['is_shiny']) === 1,
            'site' => (int) $pm['site'],
            'state' => $pm_state,
            'state_text' => get_pokemon_state_text($pm_state),
            'state_class' => get_pokemon_state_class($pm_state),
            'affection' => $pm_affection,
            'skills' => $skills,
            'base_info' => [
                'id' => $pmno,
                'name' => $info ? $info['name'] : '???',
                'type_1' => $info ? $info['xs'] : '',
                'type_2' => ($info && $info['xs2']) ? $info['xs2'] : null,
                'image' => '',
                'description' => $info ? (isset($info['description']) ? $info['description'] : '') : '',
            ],
        ];
    }

    api_success([
        'pokemons' => $pokemons,
        'total' => count($pokemons),
    ]);
}

/**
 * 获取宠物详情
 */
function api_get_pokemon_detail()
{
    require_login();

    $pet_id_param = get_param('pokemon_id', 0);

    if (!$pet_id_param) {
        $pet_id_param = get_param('id', 0);
    }

    if (!$pet_id_param) {
        api_error('Missing parameter: pokemon_id', 400);
    }

    $pet_id = validate_id($pet_id_param, 'pet_id');

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 查询宠物
    $pm = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    if (!$pm) {
        api_error('Pokemon not found', 404);
    }

    $pmno = (int) $pm['species_id'];

    // 查询基础信息
    $info = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d", $pmno));

    // 计算能力值
    $stats = calculate_stats($pm, $info);

    // 查询技能（连接 pm_skill 获取详情）
    $skill_rows = DB::fetch_all(pm_sql(
        "SELECT ms.skillid, ms.skillnum, s.name as skill_name, s.element, s.category, s.level_required, s.power, s.max_uses as max_pp "
            . "FROM " . pm_table('pm_myskill') . " ms "
            . "LEFT JOIN " . pm_table('pm_skill') . " s ON ms.skillid = s.id "
            . "WHERE ms.petid = %d LIMIT 4",
        $pet_id
    ));
    $skills = [];

    foreach ($skill_rows as $sk) {
        $max_pp = (int) $sk['max_pp'];
        $current_pp = (int) $sk['skillnum'];
        $skills[] = [
            'type_id' => (int) $sk['skillid'],
            'pp' => $current_pp,
            'name' => $sk['skill_name'] ? $sk['skill_name'] : '',
            'skill_type' => $sk['element'] ? $sk['element'] : '',
            'category' => $sk['category'] ? $sk['category'] : '',
            'level' => (int) $sk['level_required'],
            'power' => (int) $sk['power'],
            'max_pp' => $max_pp,
            // 遗忘接口要求PP为满才能遗忘；max_uses=0 视为不限PP恒可遗忘
            'can_forget' => ($max_pp === 0 || $current_pp >= $max_pp),
        ];
    }

    while (count($skills) < 4) {
        $skills[] = [
            'type_id' => 0,
            'pp' => 0,
            'name' => '',
            'skill_type' => '',
            'category' => '',
            'level' => 0,
            'power' => 0,
            'max_pp' => 0,
            'can_forget' => false
        ];
    }

    // 计算最大 HP（使用统一计算函数）
    $level = (int) $pm['level'];
    $max_hp = api_calculate_pokemon_max_hp($pm, $info);

    // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
    $hp_validation = api_validate_and_correct_hp($pm, null, $max_hp, $info);
    $pm['hp'] = strval($hp_validation['hp']);

    // 计算升级到下一级需要的经验值
    $exp_table = get_pokemon_exp_table_local($pmno);
    $current_exp = (int) $pm['exp'];

    // exp_for_current_level 是上一级的阈值（当前等级的起始经验）
    // exp_for_next_level 是当前等级的阈值（升到下一级需要的经验）
    $prev_level_key = $level - 1;
    $current_level_threshold = isset($exp_table[$prev_level_key]) ? $exp_table[$prev_level_key] : (isset($exp_table[(string)$prev_level_key]) ? $exp_table[(string)$prev_level_key] : 0);
    $next_level_exp = isset($exp_table[$level]) ? $exp_table[$level] : (isset($exp_table[(string)$level]) ? $exp_table[(string)$level] : 1250000);
    $exp_needed_for_next = max(0, $next_level_exp - $current_exp);

    $pm_state = (int) $pm['state'];
    $pm_affection = (int) $pm['good'];

    api_success([
        'id' => (int) $pm['id'],
        'site' => (int) $pm['site'],
        'name' => $info ? $info['name'] : '???',
        'nickname' => $pm['nickname'] ? $pm['nickname'] : null,
        'type_id' => $pmno,
        'level' => $level,
        'exp' => (int) $pm['exp'],
        'exp_to_next_level' => $exp_needed_for_next,
        'exp_for_current_level' => $current_level_threshold,
        'exp_for_next_level' => $next_level_exp,
        'hp' => (int) $pm['hp'],
        'max_hp' => $max_hp,
        'gender' => (int) $pm['sex'],
        'is_shiny' => ((int) $pm['is_shiny']) === 1,
        'state' => $pm_state,
        'state_text' => get_pokemon_state_text($pm_state),
        'state_class' => get_pokemon_state_class($pm_state),
        'affection' => $pm_affection,
        'stats' => $stats,
        'skills' => $skills,
        // 遗忘规则声明：遗忘技能要求该技能PP为满（can_forget 已按技能给出），
        // 客户端据此提前禁用/提示，而不是等玩家点了才收到报错
        'forget_requires_full_pp' => true,
        'base_info' => [
            'id' => $pmno,
            'name' => $info ? $info['name'] : '???',
            'type_1' => $info ? $info['xs'] : '',
            'type_2' => ($info && $info['xs2']) ? $info['xs2'] : null,
            'description' => $info ? (isset($info['description']) ? $info['description'] : '') : '',
        ],
    ]);
}

/**
 * 重命名宠物
 */
function api_rename_pokemon()
{
    require_login();

    $input = get_json_input();

    if (!isset($input['id']) || !$input['id']) {
        api_error('Missing parameter: id', 400);
    }

    $pet_id = validate_id($input['id'], 'pet_id');

    if (!isset($input['name']) || !$input['name']) {
        api_error('Missing parameter: name', 400);
    }

    $new_name = validate_string($input['name'], 'name', 20);

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 更新名称
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET nickname = %s WHERE id = %d AND uid = %d",
        addslashes($new_name),
        $pet_id,
        $uid
    ));

    api_success(['message' => 'Pokemon renamed successfully']);
}

/**
 * 放生宠物
 */
function api_release_pokemon()
{
    require_login();

    $input = get_json_input();

    if (!isset($input['id']) || !$input['id']) {
        api_error('Missing parameter: id', 400);
    }

    $pet_id = validate_id($input['id'], 'pet_id');

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 验证宠物归属
    $pm = DB::fetch_first(pm_sql(
        "SELECT id, site, state, hp FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    if (!$pm) {
        api_error('Pokemon not found', 404);
    }

    // 检查是否为濒危状态
    if ((int)$pm['state'] === 0) {
        api_error('濒危状态的宠物无法放生', 400);
    }

    // 检查血量是否为0
    if ((int)$pm['hp'] <= 0) {
        api_error('血量为0的宠物无法放生', 400);
    }

    // 检查用户是否在战斗中且这是首位宠物
    $user_data = api_my_usersdata($uid);
    if (!empty($user_data['npcid']) && $user_data['npcid'] > 0 && (int)$pm['site'] === 1) {
        api_error('战斗中的首位宠物无法放生', 400);
    }

    // 检查是否为最后一个宠物
    $total_count = DB::result_first(pm_sql("SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = %d", $uid));
    if ($total_count <= 1) {
        api_error('无法放生最后一只宠物，您至少需要保留一只宠物', 400);
    }

    // 如果放生的是首位宠物，需要自动指定新的首位
    if ((int)$pm['site'] === 1) {
        // 找到下一个可用的宠物作为首位
        $next_first = DB::fetch_first(pm_sql(
            "SELECT id FROM " . pm_table('pm_mypm') . " WHERE uid = %d AND id != %d ORDER BY site ASC, id ASC LIMIT 1",
            $uid,
            $pet_id
        ));
        if ($next_first) {
            DB::query(pm_sql("UPDATE " . pm_table('pm_mypm') . " SET site = 1 WHERE id = %d", intval($next_first['id'])));
        }
    }

    // 删除宠物及技能
    DB::query(pm_sql("DELETE FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d", $pet_id, $uid));
    DB::query(pm_sql("DELETE FROM " . pm_table('pm_myskill') . " WHERE petid = %d AND uid = %d", $pet_id, $uid));

    api_success(['message' => 'Pokemon released successfully']);
}

/**
 * 计算能力值
 */
function calculate_stats($pm, $info)
{
    global $statehp, $stateatk, $statedef, $statespatk, $statespdef, $statespeed;

    if (!$info) {
        return [
            'hp' => 0,
            'attack' => 0,
            'defense' => 0,
            'sp_attack' => 0,
            'sp_defense' => 0,
            'speed' => 0
        ];
    }

    $level = (int) $pm['level'];
    $state = (int) $pm['state'];
    $sg = (int) $pm['is_shiny'];
    $flash_boost = ($sg == 1) ? 2 : 1;

    $stat_map = [
        'hp'     => ['base' => 'hp',     'iv' => 'hpg',   'ev' => 'hpn',   'state' => $statehp],
        'attack' => ['base' => 'atk',    'iv' => 'atkg',  'ev' => 'atkn',  'state' => $stateatk],
        'defense'=> ['base' => 'def',    'iv' => 'defg',  'ev' => 'defn',  'state' => $statedef],
        'sp_attack' => ['base' => 'spatk','iv' => 'spatkg','ev' => 'spatkn','state' => $statespatk],
        'sp_defense'=>['base' => 'spdef', 'iv' => 'spdefg','ev' => 'spdefn','state' => $statespdef],
        'speed'  => ['base' => 'speed',     'iv' => 'sdg',   'ev' => 'sdn',   'state' => $statespeed],
    ];

    $result = [];
    foreach ($stat_map as $key => $cfg) {
        $base = (int) $info[$cfg['base']];
        $iv = (int) $pm[$cfg['iv']];
        $ev = (int) $pm[$cfg['ev']];
        $is_hp = ($key === 'hp');
        $boost = $is_hp ? (10 + $level) : 5;
        $boost *= $flash_boost;
        $state_multiplier = isset($cfg['state'][$state]) ? (float)$cfg['state'][$state] : 1.0;
        $result[$key] = (int) floor(((2 * $base + $iv + $ev / 4) * $level / 100 + $boost) * $state_multiplier);
    }

    $equipment_bonuses = api_parse_pet_wear_items($pm, false, $result['hp']);
    if (!empty($equipment_bonuses)) {
        $result['attack']     += $equipment_bonuses[1];
        $result['defense']    += $equipment_bonuses[2];
        $result['sp_attack']  += $equipment_bonuses[3];
        $result['sp_defense'] += $equipment_bonuses[4];
        $result['speed']      += $equipment_bonuses[5];
    }

    return $result;
}

/**
 * 获取宠物可学习的技能列表
 *
 * 根据宠物ID和等级，返回该宠物可以学习的技能列表
 */
function api_get_learnable_skills()
{
    require_login();

    $pokemon_id = get_param('pokemon_id', 0);

    if (!$pokemon_id) {
        $pokemon_id = get_param('id', 0);
    }

    if (!$pokemon_id) {
        api_error('Missing parameter: pokemon_id', 400);
    }

    $pet_id = validate_id($pokemon_id, 'pokemon_id');

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 获取宠物信息
    $pm = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    if (!$pm) {
        api_error('Pokemon not found', 404);
    }

    $pmno = (int) $pm['species_id'];
    $pet_level = (int) $pm['level'];

    // 获取宠物基础信息（包含属性）
    $info = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d", $pmno));

    if (!$info) {
        api_error('Pokemon data not found', 404);
    }

    // 查询该宠物可以学习的技能
    // 数据库格式是 "k,27,28,31,k"，旧版的 like '%,$pmno,%' 匹配不到
    // 所以我们查所有技能，然后在PHP中过滤

    $all_skills = DB::fetch_all(
            "SELECT * FROM " . pm_table('pm_skill') . " ORDER BY level_required ASC"
    );

    // 获取宠物已学习的技能ID列表（需要加uid过滤）
    $learned_rows = DB::fetch_all(pm_sql(
        "SELECT skillid FROM " . pm_table('pm_myskill') . " WHERE petid = %d AND uid = %d",
        $pet_id,
        $uid
    ));
    $learned_skills = [];

    foreach ($learned_rows as $learned) {
        $learned_skills[] = (int) $learned['skillid'];
    }

    // 分类技能
    $available_skills = []; // 可学习的技能
    $unlocked_skills = []; // 未达到等级的技能

    // 检查技能是否可以被该宠物学习
    // pmid = '99999' 是全局技能
    // 数据库格式是 "k,27,28,31,k" 或者可能被损坏成包含SQL语句
    foreach ($all_skills as $skill) {
        $pmid = isset($skill['available_pokemons']) ? $skill['available_pokemons'] : '';

        // 检查是否是全局技能
        $is_global = ($pmid === '99999');

        // 处理 pmid 格式
        $is_race_skill = false;

        if (!$is_global && !empty($pmid)) {
            // 去掉首尾的 k 字符
            $pmid_clean = trim($pmid, 'k');

            // 如果包含 "where" 说明数据损坏了，需要特殊处理
            if (strpos($pmid_clean, 'where') !== false) {
                // 只取 where 前面的部分
                $pmid_clean = trim(explode('where', $pmid_clean)[0]);
            }

            // 按逗号分隔
            $pmid_parts = array_filter(explode(',', $pmid_clean));
            $is_race_skill = in_array((string)$pmno, $pmid_parts);
        }

        if (!$is_global && !$is_race_skill) {
            continue;
        }

        // 跳过已学习的技能
        if (in_array((int)$skill['id'], $learned_skills)) {
            continue;
        }

        $skill_id = (int) $skill['id'];
        $required_level = (int) $skill['level_required'];

        $skill_info = [
            'id' => $skill_id,
            'name' => $skill['name'],
            'description' => $skill['description'] ?: '',
            'type' => $skill['element'] ?: '',
            'category' => $skill['category'] ?: '',
            'power' => (int) $skill['power'],
            'max_pp' => (int) $skill['max_uses'],
            'required_level' => $required_level,
            'is_available' => $pet_level >= $required_level,
        ];

        if ($pet_level >= $required_level) {
            $available_skills[] = $skill_info;
        } else {
            $unlocked_skills[] = $skill_info;
        }
    }

    api_success([
        'pokemon_id' => $pet_id,
        'pokemon_level' => $pet_level,
        'available_skills' => $available_skills,
        'unlocked_skills' => $unlocked_skills,
    ]);
}

/**
 * 遗忘宠物技能
 */
function api_forget_skill()
{
    require_login();

    $input = get_json_input();

    if (!isset($input['pokemon_id']) || !$input['pokemon_id']) {
        api_error('Missing parameter: pokemon_id', 400);
    }

    $pet_id = validate_id($input['pokemon_id'], 'pokemon_id');

    if (!isset($input['skill_id']) || !$input['skill_id']) {
        api_error('Missing parameter: skill_id', 400);
    }

    $skill_id = validate_id($input['skill_id'], 'skill_id');

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 检查战斗状态
    $user_data = DB::fetch_first(pm_sql("SELECT npcid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d", $uid));

    if ($user_data && !empty($user_data['npcid']) && $user_data['npcid'] > 0) {
        api_error('战斗中不能遗忘技能', 400, null, 'skill_battle_restricted');
    }

    // 验证宠物归属
    $pm = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    if (!$pm) {
        api_error('Pokemon not found', 404);
    }

    // 检查宠物是否有这个技能
    $myskill = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_myskill') . "
WHERE petid=%d AND skillid=%d",
        $pet_id,
        $skill_id
    ));

    if (!$myskill) {
        api_error('该宠物没有学会这个技能', 404, null, 'skill_not_learned');
    }

    // 检查 PP 值是否已满（必须满 PP 才能遗忘）
    $current_pp = (int) $myskill['skillnum'];
    $skill_info = DB::fetch_first(pm_sql("SELECT max_uses FROM " . pm_table('pm_skill') . " WHERE id = %d", $skill_id));
    $max_pp = $skill_info ? (int) $skill_info['max_uses'] : 0;

    if ($current_pp < $max_pp) {
        api_error(
            "技能PP未满，无法遗忘（当前PP：{$current_pp}/{$max_pp}，需先用PP恢复道具补满后才能遗忘）",
            400,
            null,
            'skill_pp_not_full'
        );
    }

    // 删除技能
    DB::query(pm_sql(
        "DELETE FROM " . pm_table('pm_myskill') . "
WHERE petid=%d AND skillid=%d",
        $pet_id,
        $skill_id
    ));

    api_success(['message' => '技能遗忘成功']);
}

/**
 * 学习新技能
 */
function api_learn_skill()
{
    require_login();

    $input = get_json_input();

    if (!isset($input['pokemon_id']) || !$input['pokemon_id']) {
        api_error('Missing parameter: pokemon_id', 400);
    }

    $pet_id = validate_id($input['pokemon_id'], 'pokemon_id');

    if (!isset($input['skill_id']) || !$input['skill_id']) {
        api_error('Missing parameter: skill_id', 400);
    }

    $skill_id = validate_id($input['skill_id'], 'skill_id');

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 检查战斗状态
    $user_data = DB::fetch_first(pm_sql("SELECT npcid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d", $uid));

    if ($user_data && !empty($user_data['npcid']) && $user_data['npcid'] > 0) {
        api_error('战斗中不能学习技能', 400, null, 'skill_battle_restricted');
    }

    // 验证宠物归属
    $pm = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    if (!$pm) {
        api_error('Pokemon not found', 404);
    }

    $pet_level = (int) $pm['level'];
    $pmno = (int) $pm['species_id'];

    // 检查技能是否存在
    $skill = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_skill') . " WHERE id = %d",
        $skill_id
    ));

    if (!$skill) {
        api_error('技能不存在', 404, null, 'skill_not_found');
    }

    // 检查等级是否满足
    $required_level = (int) $skill['level_required'];

    if ($pet_level < $required_level) {
        api_error("等级不足，需要达到 Lv {$required_level} 才能学习这个技能", 400, null, 'skill_level_not_met');
    }

    // 检查宠物是否可以学习这个技能（种族或全局）
    $can_learn = false;
    $pmid = $skill['available_pokemons'];

    if (strpos($pmid, (string)$pmno) !== false || $pmid === '99999') {
        $can_learn = true;
    }

    if (!$can_learn) {
        api_error('该宠物无法学习这个技能', 400, null, 'skill_not_learnable');
    }

    // 检查是否已学习该技能
    $existing = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_myskill') . "
WHERE petid=%d AND skillid=%d",
        $pet_id,
        $skill_id
    ));

    if ($existing) {
        api_error('已经学会了这个技能', 400, null, 'skill_already_learned');
    }

    // 检查技能槽是否已满（pm_myskill 无槽位列，技能按插入顺序生效；
    // 如需替换技能，客户端需先调用遗忘接口，遗忘要求技能PP为满）
    $current_skills_count = DB::result_first(pm_sql(
        "SELECT COUNT(*) FROM " . pm_table('pm_myskill') . " WHERE petid = %d",
        $pet_id
    ));

    if ($current_skills_count >= 4) {
        api_error('技能槽已满，请先遗忘一个技能', 400, null, 'skill_slots_full');
    }

    // 学习技能
    $max_pp = (int) $skill['max_uses'];
    DB::query(
        "INSERT INTO " . pm_table('pm_myskill') . "
(uid, petid, skillid, skillnum) VALUES ($uid, $pet_id, $skill_id, $max_pp)"
    );

    api_success([
        'message' => '技能学习成功',
        'pokemon_id' => $pet_id,
        'skill' => [
            'type_id' => $skill_id,
            'pp' => $max_pp,
            'name' => $skill['name'],
            'skill_type' => $skill['element'] ?: '',
            'category' => $skill['category'] ?: '',
            'level' => (int) $skill['level_required'],
            'max_pp' => $max_pp,
        ],
    ]);
}

/**
 * 获取宝可梦装备信息
 * 返回：当前装备的4个槽位信息 + 用户拥有的所有装备道具列表
 */
function api_get_equipment()
{
    require_login();

    $pokemon_id_param = get_param('pokemon_id', 0);

    if (!$pokemon_id_param) {
        $pokemon_id_param = get_param('id', 0);
    }

    if (!$pokemon_id_param) {
        api_error('Missing parameter: pokemon_id', 400);
    }

    $pet_id = validate_id($pokemon_id_param, 'pokemon_id');

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 验证宠物归属
    $pm = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    if (!$pm) {
        api_error('Pokemon not found', 404);
    }

    // 获取当前装备的4个槽位
    $equipment_slots = [];

    for ($i = 1; $i <= 4; $i++) {
        $equipment_id = (int) $pm["equipmentid$i"];
        $slot_info = [
            'slot_index' => $i - 1,
            'equipment_id' => $equipment_id,
            'item' => null,
        ];

        if ($equipment_id > 0) {
            // 获取装备详情
            $myitem = DB::fetch_first(pm_sql(
                "SELECT m.id as myitem_id, m.itemid, i.* FROM " . pm_table('pm_myitem') . " m
LEFT JOIN " . pm_table('pm_itemdata') . " i ON m.itemid=i.id WHERE m.id=%d",
                $equipment_id
            ));

            if ($myitem) {
                $equipment = json_decode($myitem['equipment'], true) ?: [];
                $slot_info['item'] = [
                    'myitem_id' => (int) $myitem['myitem_id'],
                    'type_id' => (int) $myitem['id'],
                    'name' => $myitem['name'],
                    'description' => $myitem['description'] ?: '',
                    'image' => $myitem['tpname'] ?: '',
                    'zbtype' => (int) $myitem['zbtype'],
                    'equipment_hp' => (int) (isset($equipment['hp']) ? $equipment['hp'] : 0),
                    'equipment_atk' => (int) (isset($equipment['atk']) ? $equipment['atk'] : 0),
                    'equipment_def' => (int) (isset($equipment['def']) ? $equipment['def'] : 0),
                    'equipment_spatk' => (int) (isset($equipment['spatk']) ? $equipment['spatk'] : 0),
                    'equipment_spdef' => (int) (isset($equipment['spdef']) ? $equipment['spdef'] : 0),
                    'equipment_sd' => (int) (isset($equipment['spd']) ? $equipment['spd'] : 0),
                ];
            }
        }

        $equipment_slots[] = $slot_info;
    }

    // 获取用户拥有的所有装备道具 (type=5)
    $owned_items = [];
    $owned_rows = DB::fetch_all(pm_sql(
        "SELECT m.id as myitem_id, m.itemid, m.nums, i.* FROM " . pm_table('pm_myitem') . " m
LEFT JOIN " . pm_table('pm_itemdata') . " i ON m.itemid=i.id WHERE m.uid=%d AND i.type=5 AND m.nums > 0",
        $uid
    ));

    foreach ($owned_rows as $row) {

        $equipment = json_decode($row['equipment'], true) ?: [];

        // 检查该装备被所有宝可梦使用的总数
        $myitem_id = $row['myitem_id'];
        $equipped_count_query = DB::fetch_first(pm_sql(
            "SELECT COUNT(*) as cnt FROM " . pm_table('pm_mypm') . "
            WHERE (equipmentid1 = %d OR equipmentid2 = %d OR equipmentid3 = %d OR equipmentid4 = %d)",
            $myitem_id,
            $myitem_id,
            $myitem_id,
            $myitem_id
        ));
        $equipped_count = $equipped_count_query ? (int) $equipped_count_query['cnt'] : 0;

        // 检查该装备是否被当前宝可梦使用
        $equipped_by_current = DB::fetch_first(pm_sql(
            "SELECT id FROM " . pm_table('pm_mypm') . "
            WHERE (equipmentid1 = %d OR equipmentid2 = %d OR equipmentid3 = %d OR equipmentid4 = %d) AND id = %d",
            $myitem_id,
            $myitem_id,
            $myitem_id,
            $myitem_id,
            $pet_id
        ));
        $is_equipped_by_current = !empty($equipped_by_current);

        $owned_items[] = [
            'myitem_id' => (int) $row['myitem_id'],
            'type_id' => (int) $row['itemid'],
            'name' => $row['name'],
            'description' => $row['description'] ?: '',
            'image' => $row['tpname'] ?: '',
            'quantity' => (int) $row['nums'],
            'equipped_count' => $equipped_count,
            'available_count' => max(0, (int) $row['nums'] - $equipped_count),
            'is_equipped' => $is_equipped_by_current,
            'zbtype' => (int) $row['zbtype'],
            'equipment_hp' => (int) (isset($equipment['hp']) ? $equipment['hp'] : 0),
            'equipment_atk' => (int) (isset($equipment['atk']) ? $equipment['atk'] : 0),
            'equipment_def' => (int) (isset($equipment['def']) ? $equipment['def'] : 0),
            'equipment_spatk' => (int) (isset($equipment['spatk']) ? $equipment['spatk'] : 0),
            'equipment_spdef' => (int) (isset($equipment['spdef']) ? $equipment['spdef'] : 0),
            'equipment_sd' => (int) (isset($equipment['spd']) ? $equipment['spd'] : 0),
        ];
    }

    // 获取商店中可购买的装备道具
    $shop_items = [];
    $shop_rows = DB::fetch_all(
        "SELECT * FROM " . pm_table('pm_itemdata') . " 
WHERE type=5 AND shop=1 ORDER BY money ASC"
    );

    // 获取用户金钱
    $user_data = DB::fetch_first(pm_sql(
        "SELECT money FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $uid
    ));
    $user_money = $user_data ? (int) $user_data['money'] : 0;

    foreach ($shop_rows as $row) {
        $equipment = json_decode($row['equipment'], true) ?: [];
        // 检查用户是否已拥有该装备
        $owned_type_ids = array_column($owned_items, 'type_id');
        $is_owned = in_array((int) $row['id'], $owned_type_ids);

        $shop_items[] = [
            'type_id' => (int) $row['id'],
            'name' => $row['name'],
            'description' => $row['description'] ?: '',
            'image' => $row['tpname'] ?: '',
            'price' => (int) $row['money'],
            'zbtype' => (int) $row['zbtype'],
            'equipment_hp' => (int) (isset($equipment['hp']) ? $equipment['hp'] : 0),
            'equipment_atk' => (int) (isset($equipment['atk']) ? $equipment['atk'] : 0),
            'equipment_def' => (int) (isset($equipment['def']) ? $equipment['def'] : 0),
            'equipment_spatk' => (int) (isset($equipment['spatk']) ? $equipment['spatk'] : 0),
            'equipment_spdef' => (int) (isset($equipment['spdef']) ? $equipment['spdef'] : 0),
            'equipment_sd' => (int) (isset($equipment['spd']) ? $equipment['spd'] : 0),
            'is_owned' => $is_owned,
            'can_buy' => $user_money >= (int) $row['money'],
        ];
    }

    api_success([
        'pokemon_id' => $pet_id,
        'equipment_slots' => $equipment_slots,
        'owned_items' => $owned_items,
        'shop_items' => $shop_items,
        'user_money' => $user_money,
    ]);
}

/**
 * 装备物品
 */
function api_equip_item()
{
    require_login();

    $input = get_json_input();

    if (!isset($input['pokemon_id']) || !$input['pokemon_id']) {
        api_error('Missing parameter: pokemon_id', 400);
    }

    $pet_id = validate_id($input['pokemon_id'], 'pokemon_id');

    if (!isset($input['myitem_id']) || !$input['myitem_id']) {
        api_error('Missing parameter: myitem_id', 400);
    }

    $myitem_id = validate_id($input['myitem_id'], 'myitem_id');

    // 槽位索引 (0-3)，可选，默认自动选择第一个空槽
    $slot_index = isset($input['slot_index']) ? (int) $input['slot_index'] : -1;

    if ($slot_index !== -1 && ($slot_index < 0 || $slot_index > 3)) {
        api_error('Invalid slot_index, must be 0-3 or -1 for auto', 400);
    }

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 验证宠物归属
    $pm = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    if (!$pm) {
        api_error('Pokemon not found', 404);
    }

    // 验证物品归属
    $myitem = DB::fetch_first(pm_sql(
        "SELECT m.*, i.type, i.name, i.equipment FROM " . pm_table('pm_myitem') . " m LEFT JOIN " . pm_table('pm_itemdata') . " i ON m.itemid=i.id WHERE m.id=%d AND m.uid=%d",
        $myitem_id, $uid
    ));

    if (!$myitem) {
        api_error('Item not found or not owned by user', 404);
    }

    if ((int) $myitem['type'] !== 5) {
        api_error('This item is not an equipment', 400);
    }

    if ((int) $myitem['nums'] <= 0) {
        api_error('No items available', 400);
    }

    // 同一背包装备记录只能占用一个槽位，也要检查当前宠物。
    $equipped_on = DB::fetch_first(pm_sql(
        "SELECT id, nickname FROM " . pm_table('pm_mypm') . "
WHERE (equipmentid1=%d OR equipmentid2=%d OR equipmentid3=%d OR equipmentid4=%d) AND uid=%d",
        $myitem_id, $myitem_id, $myitem_id, $myitem_id, $uid
    ));

    if ($equipped_on) {
        api_error("该装备已被 {$equipped_on['nickname']} 使用", 400);
    }

    // 确定槽位
    if ($slot_index === -1) {

        // 自动选择第一个空槽
        for ($i = 1; $i <= 4; $i++) {
            if ((int) $pm["equipmentid$i"] === 0) {
                $slot_index = $i - 1;
                break;
            }
        }

        if ($slot_index === -1) {
            api_error('All equipment slots are full', 400);
        }
    }

    $slot_field = "equipmentid" . ($slot_index + 1);

    // 检查目标槽位是否已有装备
    if ((int) $pm[$slot_field] !== 0) {
        api_error('Target slot already has equipment. Please unequip first.', 400);
    }

    // 装备前的当前血量百分比
    $old_hp = (int) $pm['hp'];
    $old_maxhp = api_calculate_pokemon_max_hp($pm);

    if ($old_maxhp <= 0) {
        $old_maxhp = $old_hp > 0 ? $old_hp : 1;
    }

    $hp_percent = $old_hp / $old_maxhp;

    // 原子占用：只有目标槽位仍为空、且该背包记录未被同账号任何宠物（含当前宠物）占用时才写入。
    // 上方的占用预检只负责给出友好的错误提示，并发窗口由这里的条件 UPDATE 关闭：
    // 抢占失败的请求不会写入任何数据。
    // 注意不能用括号子查询（NOT EXISTS (SELECT ...)）：Discuz querysafe 拦截 "(select"
    // （见 user.php 背包列表的同类规避），这里用自连接反连接实现同一守卫。
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " pet
LEFT JOIN " . pm_table('pm_mypm') . " occupier
    ON occupier.uid = pet.uid
   AND (occupier.equipmentid1=%d OR occupier.equipmentid2=%d OR occupier.equipmentid3=%d OR occupier.equipmentid4=%d)
SET pet.$slot_field=%d
WHERE pet.id=%d AND pet.uid=%d AND pet.$slot_field=0 AND occupier.id IS NULL",
        $myitem_id, $myitem_id, $myitem_id, $myitem_id, $myitem_id, $pet_id, $uid
    ));

    if (!DB::affected_rows()) {
        api_error('Equipment conflict, please retry', 409);
    }

    // 重新获取宠物数据并计算完整属性
    $pm = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    $pmno = (int) $pm['species_id'];
    $pm_data = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d", $pmno));
    $full_stats = calculate_pokemon_full_stats($pm, $pm_data);

    // 计算新的 HP（保持血量百分比）
    $new_maxhp = $full_stats['total_hp'];
    $new_hp = (int) round($new_maxhp * $hp_percent);
    $new_hp = max(1, min($new_hp, $new_maxhp));

    // 更新数据库中的 hp
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
        $new_hp,
        $pet_id
    ));

    api_success([
        'message' => 'Equipment equipped successfully',
        'pokemon_id' => $pet_id,
        'slot_index' => $slot_index,
        'item_name' => $myitem['name'],
        'base_hp' => $full_stats['base_hp'],
        'equipment_hp' => $full_stats['equipment_hp'],
        'total_hp' => $full_stats['total_hp'],
        'equipment_atk' => $full_stats['equipment_atk'],
        'equipment_def' => $full_stats['equipment_def'],
        'equipment_spatk' => $full_stats['equipment_spatk'],
        'equipment_spdef' => $full_stats['equipment_spdef'],
        'equipment_speed' => $full_stats['equipment_speed'],
        'base_atk' => $full_stats['base_atk'],
        'base_def' => $full_stats['base_def'],
        'base_spatk' => $full_stats['base_spatk'],
        'base_spdef' => $full_stats['base_spdef'],
        'base_speed' => $full_stats['base_speed'],
        'total_atk' => $full_stats['total_atk'],
        'total_def' => $full_stats['total_def'],
        'total_spatk' => $full_stats['total_spatk'],
        'total_spdef' => $full_stats['total_spdef'],
        'total_speed' => $full_stats['total_speed'],
        'new_hp' => $new_hp,
        'new_maxhp' => $new_maxhp,
    ]);
}

/**
 * 卸下装备
 */
function api_unequip_item()
{
    require_login();

    $input = get_json_input();

    if (!isset($input['pokemon_id']) || !$input['pokemon_id']) {
        api_error('Missing parameter: pokemon_id', 400);
    }

    $pet_id = validate_id($input['pokemon_id'], 'pokemon_id');

    if (!isset($input['slot_index']) || $input['slot_index'] === '') {
        api_error('Missing parameter: slot_index', 400);
    }

    $slot_index = (int) $input['slot_index'];

    if ($slot_index < 0 || $slot_index > 3) {
        api_error('Invalid slot_index, must be 0-3', 400);
    }

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 验证宠物归属
    $pm = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    if (!$pm) {
        api_error('Pokemon not found', 404);
    }

    $slot_field = "equipmentid" . ($slot_index + 1);
    $equipment_id = (int) $pm[$slot_field];

    if ($equipment_id === 0) {
        api_error('No equipment in this slot', 400);
    }

    // 获取装备信息
    $myitem = DB::fetch_first(pm_sql(
        "SELECT m.*, i.name FROM " . pm_table('pm_myitem') . " m
LEFT JOIN " . pm_table('pm_itemdata') . " i ON m.itemid=i.id WHERE m.id=%d",
        $equipment_id
    ));

    $item_name = $myitem ? $myitem['name'] : 'Unknown';

    // 卸下装备前的当前血量百分比
    $old_hp = (int) $pm['hp'];
    $old_maxhp = api_calculate_pokemon_max_hp($pm);

    if ($old_maxhp <= 0) {
        $old_maxhp = $old_hp > 0 ? $old_hp : 1;
    }

    $hp_percent = $old_hp / $old_maxhp;

    // 卸下装备
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . "
SET $slot_field=0 WHERE id=%d AND uid=%d",
        $pet_id, $uid
    ));

    // 重新获取宠物数据并计算完整属性
    $pm = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    $pmno = (int) $pm['species_id'];
    $pm_data = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d", $pmno));
    $full_stats = calculate_pokemon_full_stats($pm, $pm_data);

    // 计算新的 HP（保持血量百分比）
    $new_maxhp = $full_stats['total_hp'];
    $new_hp = (int) round($new_maxhp * $hp_percent);
    $new_hp = max(1, min($new_hp, $new_maxhp));

    // 更新数据库中的 hp
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
        $new_hp,
        $pet_id
    ));

    api_success([
        'message' => 'Equipment unequipped successfully',
        'pokemon_id' => $pet_id,
        'slot_index' => $slot_index,
        'item_name' => $item_name,
        'base_hp' => $full_stats['base_hp'],
        'equipment_hp' => $full_stats['equipment_hp'],
        'total_hp' => $full_stats['total_hp'],
        'equipment_atk' => $full_stats['equipment_atk'],
        'equipment_def' => $full_stats['equipment_def'],
        'equipment_spatk' => $full_stats['equipment_spatk'],
        'equipment_spdef' => $full_stats['equipment_spdef'],
        'equipment_speed' => $full_stats['equipment_speed'],
        'base_atk' => $full_stats['base_atk'],
        'base_def' => $full_stats['base_def'],
        'base_spatk' => $full_stats['base_spatk'],
        'base_spdef' => $full_stats['base_spdef'],
        'base_speed' => $full_stats['base_speed'],
        'total_atk' => $full_stats['total_atk'],
        'total_def' => $full_stats['total_def'],
        'total_spatk' => $full_stats['total_spatk'],
        'total_spdef' => $full_stats['total_spdef'],
        'total_speed' => $full_stats['total_speed'],
        'new_hp' => $new_hp,
        'new_maxhp' => $new_maxhp,
    ]);
}

/**
 * 更新宠物状态（随机触发）
 * 
 * 基于旧版 state.inc.php 的状态机逻辑
 * 状态会影响宠物的属性（HP、攻击、防御等）
 * 状态变化间隔：建议前端每隔 5-10 分钟调用一次
 */
function api_update_pokemon_state()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);
    $timestamp = $_G['timestamp'];

    // 获取首只宠物（site=1）
    $pm = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE uid = %d AND site = 1",
        $uid
    ));

    if (!$pm) {
        api_error('No active pokemon found', 404);
    }

    $pet_id = (int) $pm['id'];
    $current_state = (int) $pm['state'];
    $pmno = (int) $pm['species_id'];

    // 可随机触发的基础状态列表
    $base_states = [
        1,
        2,
        5,
        7,
        8,
        9,
        11,
        12,
        15,
        16,
        18
    ];

    $new_state = $current_state;
    $state_changed = false;
    $exp_change = 0;

    // 状态机逻辑（修复版：状态从阶段一开始，逐步恶化，每个阶段都有恢复机会）
    switch ($current_state) {
        case 2: // 生病阶段一
            $roll = rand(1, 100);
            if ($roll <= 50) {
                $new_state = 2; // 50% 保持生病阶段一
            } elseif ($roll <= 70) {
                $new_state = 3; // 20% 恶化到阶段二
            } else {
                $new_state = 1; // 30% 恢复正常
            }
            $state_changed = true;
            break;

        case 3: // 生病阶段二
            $roll = rand(1, 100);
            if ($roll <= 40) {
                $new_state = 3; // 40% 保持生病阶段二
            } elseif ($roll <= 60) {
                $new_state = 4; // 20% 恶化到阶段三
                $exp_change = $pm['exp'] >= 50 ? -50 : 0;
            } else {
                $new_state = 2; // 40% 好转到阶段一
            }
            $state_changed = true;
            break;

        case 4: // 生病阶段三（濒危）
            $roll = rand(1, 100);
            if ($roll <= 30) {
                $new_state = 4; // 30% 保持生病阶段三
            } elseif ($roll <= 50) {
                $new_state = 0; // 20% 晕倒（濒危状态，需要治疗）
            } else {
                $new_state = 3; // 50% 好转到阶段二
            }
            $state_changed = true;
            break;

        case 5: // 饥饿阶段一
            $roll = rand(1, 100);
            if ($roll <= 50) {
                $new_state = 5; // 50% 保持饥饿阶段一
            } elseif ($roll <= 70) {
                $new_state = 6; // 20% 恶化到阶段二
            } else {
                $new_state = 1; // 30% 恢复正常
            }
            $state_changed = true;
            break;

        case 6: // 饥饿阶段二
            $roll = rand(1, 100);
            if ($roll <= 40) {
                $new_state = 6; // 40% 保持饥饿阶段二
            } elseif ($roll <= 50) {
                $new_state = 2; // 10% 变成生病阶段一（饥饿导致生病）
            } else {
                $new_state = 5; // 50% 好转到阶段一
            }
            $state_changed = true;
            break;

        case 7: // 疲惫
            $roll = rand(1, 100);
            if ($roll <= 40) {
                $new_state = 7; // 40% 保持疲惫
            } elseif ($roll <= 50) {
                $new_state = 2; // 10% 变成生病阶段一（过度疲劳）
            } else {
                $new_state = 1; // 50% 恢复正常
            }
            $state_changed = true;
            break;

        case 8: // 兴奋阶段一
            $roll = rand(1, 100);
            if ($roll <= 50) {
                $new_state = 8; // 50% 保持兴奋阶段一
            } elseif ($roll <= 70) {
                $new_state = 9; // 20% 升级到阶段二
            } else {
                $new_state = 1; // 30% 恢复正常
            }
            $state_changed = true;
            break;

        case 9: // 兴奋阶段二
            $roll = rand(1, 100);
            if ($roll <= 40) {
                $new_state = 9; // 40% 保持兴奋阶段二
            } elseif ($roll <= 50) {
                $new_state = 10; // 10% 升级到阶段三
            } elseif ($roll <= 70) {
                $new_state = 7; // 20% 变成疲惫（过度兴奋后疲劳）
            } else {
                $new_state = 8; // 30% 降级到阶段一
            }
            $state_changed = true;
            break;

        case 10: // 兴奋阶段三
            $roll = rand(1, 100);
            if ($roll <= 30) {
                $new_state = 10; // 30% 保持兴奋阶段三
            } elseif ($roll <= 50) {
                $new_state = 7; // 20% 变成疲惫
            } else {
                $new_state = 9; // 50% 降级到阶段二
            }
            $state_changed = true;
            break;

        case 11: // 受伤
            $roll = rand(1, 100);
            if ($roll <= 50) {
                $new_state = 11; // 50% 保持受伤
            } else {
                $new_state = 1; // 50% 恢复正常
            }
            $state_changed = true;
            break;

        case 12: // 快乐阶段一
            $roll = rand(1, 100);
            if ($roll <= 50) {
                $new_state = 12; // 50% 保持快乐阶段一
            } elseif ($roll <= 70) {
                $new_state = 13; // 20% 升级到阶段二
            } else {
                $new_state = 1; // 30% 恢复正常
            }
            $state_changed = true;
            break;

        case 13: // 快乐阶段二
            $roll = rand(1, 100);
            if ($roll <= 40) {
                $new_state = 13; // 40% 保持快乐阶段二
            } elseif ($roll <= 50) {
                $new_state = 14; // 10% 升级到阶段三
            } else {
                $new_state = 12; // 50% 降级到阶段一
            }
            $state_changed = true;
            break;

        case 14: // 快乐阶段三
            $roll = rand(1, 100);
            if ($roll <= 30) {
                $new_state = 14; // 30% 保持快乐阶段三
            } else {
                $new_state = 13; // 70% 降级到阶段二
            }
            $state_changed = true;
            break;

        case 15: // 惊慌
            $roll = rand(1, 100);
            if ($roll <= 40) {
                $new_state = 15; // 40% 保持惊慌
            } else {
                $new_state = 1; // 60% 恢复正常
            }
            $state_changed = true;
            break;

        case 16: // 自恋阶段一
            $roll = rand(1, 100);
            if ($roll <= 50) {
                $new_state = 16; // 50% 保持自恋阶段一
            } elseif ($roll <= 70) {
                $new_state = 17; // 20% 升级到阶段二
            } else {
                $new_state = 1; // 30% 恢复正常
            }
            $state_changed = true;
            break;

        case 17: // 自恋阶段二
            $roll = rand(1, 100);
            if ($roll <= 30) {
                $new_state = 17; // 30% 保持自恋阶段二
            } else {
                $new_state = 16; // 70% 降级到阶段一
            }
            $state_changed = true;
            break;

        case 18: // 愤怒阶段一
            $roll = rand(1, 100);
            if ($roll <= 50) {
                $new_state = 18; // 50% 保持愤怒阶段一
            } elseif ($roll <= 70) {
                $new_state = 19; // 20% 升级到阶段二
            } else {
                $new_state = 1; // 30% 恢复正常
            }
            $state_changed = true;
            break;

        case 19: // 愤怒阶段二
            $roll = rand(1, 100);
            if ($roll <= 30) {
                $new_state = 19; // 30% 保持愤怒阶段二
            } elseif ($roll <= 50) {
                $new_state = 7; // 20% 变成疲惫（愤怒后疲劳）
            } else {
                $new_state = 18; // 50% 降级到阶段一
            }
            $state_changed = true;
            break;

        case 0: // 濒危/晕倒状态，不自动变化，需要去宠物中心治疗
            break;

        default: // 正常或其他状态，随机触发新状态
            if ($pmno == 0) {
                // 蛋不触发状态变化
                $new_state = 1;
            } else {
                // 随机触发状态（20% 概率，降低触发频率）
                if (rand(1, 100) <= 20) {
                    // 只触发阶段一的状态，不会直接给严重状态
                    $first_stage_states = [1, 2, 5, 7, 8, 11, 12, 15, 16, 18];
                    $new_state = $first_stage_states[array_rand($first_stage_states)];
                    $state_changed = true;
                }
            }
            break;
    }

    // 更新数据库
    if ($state_changed) {
        $update_parts = ["state = %d", "statetime = %d"];
        $update_params = [$new_state, $timestamp];

        if ($exp_change != 0) {
            $update_parts[] = "exp = exp + %d";
            $update_params[] = $exp_change;
        }

        // 如果变成死亡状态，HP 设为 1
        if ($new_state == 0) {
            $update_parts[] = "hp = 1";
        }

        $update_parts[] = "WHERE id = %d AND uid = %d";
        $update_params[] = $pet_id;
        $update_params[] = $uid;

        $update_sql = "UPDATE " . pm_table('pm_mypm') . " SET " . implode(', ', $update_parts);
        DB::query(pm_sql_v($update_sql, $update_params));

        // 重新获取宠物数据
        $pm = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
            $pet_id,
            $uid
        ));
    }

    // 计算状态对属性的影响
    $state_multipliers = get_state_multipliers($new_state);

    api_success([
        'pokemon_id' => $pet_id,
        'old_state' => $current_state,
        'new_state' => $new_state,
        'state_changed' => $state_changed,
        'state_text' => get_pokemon_state_text($new_state),
        'state_class' => get_pokemon_state_class($new_state),
        'state_multipliers' => $state_multipliers,
        'exp_change' => $exp_change,
    ]);
}

/**
 * 获取状态对属性的修正系数
 */
function get_state_multipliers($state)
{
    // 状态属性修正系数（基于 abilityrevise.inc.php）
    $multipliers = [
        1 => [
            'hp' => 1.0,
            'atk' => 1.0,
            'def' => 1.0,
            'spatk' => 1.0,
            'spdef' => 1.0,
            'speed' => 1.0
        ],
        // 正常
        2 => [
            'hp' => 0.9,
            'atk' => 0.9,
            'def' => 0.9,
            'spatk' => 0.9,
            'spdef' => 0.9,
            'speed' => 0.9
        ],
        // 生病一
        3 => [
            'hp' => 0.8,
            'atk' => 0.8,
            'def' => 0.8,
            'spatk' => 0.8,
            'spdef' => 0.8,
            'speed' => 0.8
        ],
        // 生病二
        4 => [
            'hp' => 0.5,
            'atk' => 0.5,
            'def' => 0.5,
            'spatk' => 0.5,
            'spdef' => 0.5,
            'speed' => 0.5
        ],
        // 生病三
        5 => [
            'hp' => 0.9,
            'atk' => 0.9,
            'def' => 0.9,
            'spatk' => 0.9,
            'spdef' => 0.9,
            'speed' => 0.9
        ],
        // 饥饿一
        6 => [
            'hp' => 0.8,
            'atk' => 0.8,
            'def' => 0.8,
            'spatk' => 0.8,
            'spdef' => 0.8,
            'speed' => 0.8
        ],
        // 饥饿二
        7 => [
            'hp' => 0.5,
            'atk' => 0.5,
            'def' => 0.5,
            'spatk' => 0.5,
            'spdef' => 0.5,
            'speed' => 0.5
        ],
        // 疲惫
        8 => [
            'hp' => 1.1,
            'atk' => 1.2,
            'def' => 1.0,
            'spatk' => 1.2,
            'spdef' => 1.0,
            'speed' => 1.0
        ],
        // 兴奋一
        9 => [
            'hp' => 1.2,
            'atk' => 1.5,
            'def' => 0.5,
            'spatk' => 1.5,
            'spdef' => 0.5,
            'speed' => 1.2
        ],
        // 兴奋二
        10 => [
            'hp' => 2.0,
            'atk' => 2.0,
            'def' => 2.0,
            'spatk' => 2.0,
            'spdef' => 2.0,
            'speed' => 2.0
        ],
        // 兴奋三
        11 => [
            'hp' => 0.8,
            'atk' => 1.0,
            'def' => 1.0,
            'spatk' => 1.0,
            'spdef' => 1.0,
            'speed' => 1.0
        ],
        // 受伤
        12 => [
            'hp' => 1.3,
            'atk' => 1.0,
            'def' => 1.0,
            'spatk' => 1.0,
            'spdef' => 1.0,
            'speed' => 1.0
        ],
        // 快乐一
        13 => [
            'hp' => 1.1,
            'atk' => 1.0,
            'def' => 1.0,
            'spatk' => 1.0,
            'spdef' => 1.0,
            'speed' => 1.2
        ],
        // 快乐二
        14 => [
            'hp' => 1.3,
            'atk' => 1.0,
            'def' => 1.2,
            'spatk' => 1.0,
            'spdef' => 1.2,
            'speed' => 1.6
        ],
        // 快乐三
        15 => [
            'hp' => 0.7,
            'atk' => 0.7,
            'def' => 0.7,
            'spatk' => 0.7,
            'spdef' => 0.7,
            'speed' => 0.7
        ],
        // 惊慌
        16 => [
            'hp' => 1.0,
            'atk' => 0.8,
            'def' => 1.2,
            'spatk' => 0.8,
            'spdef' => 1.2,
            'speed' => 1.0
        ],
        // 自恋一
        17 => [
            'hp' => 1.5,
            'atk' => 0.6,
            'def' => 1.5,
            'spatk' => 0.6,
            'spdef' => 1.5,
            'speed' => 1.0
        ],
        // 自恋二
        18 => [
            'hp' => 1.0,
            'atk' => 1.5,
            'def' => 0.8,
            'spatk' => 1.5,
            'spdef' => 0.8,
            'speed' => 1.0
        ],
        // 愤怒一
        19 => [
            'hp' => 1.0,
            'atk' => 2.5,
            'def' => 0.3,
            'spatk' => 2.5,
            'spdef' => 0.3,
            'speed' => 1.0
        ],
        // 愤怒二
        0 => [
            'hp' => 0.0,
            'atk' => 0.0,
            'def' => 0.0,
            'spatk' => 0.0,
            'spdef' => 0.0,
            'speed' => 0.0
        ],
        // 死亡
    ];

    return isset($multipliers[$state]) ? $multipliers[$state] : $multipliers[1];
}

/**
 * 移动宝可梦到指定位置
 *
 * site: 1=首位, 2=背包, 3=仓库
 */
function api_move_pokemon()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    $pokemon_id = validate_id(get_param('pokemon_id', 0), 'pokemon_id');
    $target_site = (int) get_param('site', 2);

    if (!in_array($target_site, [1, 2, 3])) {
        api_error('Invalid site value', 400);
    }

    // 验证宠物所有权（快速路径）
    $pokemon = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pokemon_id,
        $uid
    ));

    if (!$pokemon) {
        api_error('宝可梦不存在或不属于您', 403);
    }

    if ((int) $pokemon['site'] === $target_site) {
        api_success(['message' => '宝可梦已在目标位置']);
        return;
    }

    // 队伍 site 变更（移动/交换/设首位）与战斗换宠共用 pm_usersdata 行锁：
    // 同账号的 site 变更在锁内串行化，多步移动（让位、补位、容量检查）
    // 作为一个整体提交或回滚，不会与其他请求交错出一半。
    DB::query("START TRANSACTION");
    // 事务体内的任何异常（含数据库错误）都要显式回滚：常驻 worker 的连接
    // 不随请求关闭，未提交事务和行锁不能泄漏到下一个请求。
    try {
        $owner = DB::fetch_first(pm_sql(
            "SELECT uid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $uid
        ));
        if (!$owner) {
            pm_abort_battle_transaction('用户状态不存在，请刷新后重试', 500);
        }

        // 锁内重读最新位置
        $pokemon = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
            $pokemon_id,
            $uid
        ));
        if (!$pokemon) {
            pm_abort_battle_transaction('宝可梦不存在或不属于您', 403);
        }

        $current_site = (int) $pokemon['site'];

        // 如果已经在目标位置，无需移动
        if ($current_site === $target_site) {
            DB::query("COMMIT");
            api_success(['message' => '宝可梦已在目标位置']);
        }

        // 检查用户是否在战斗中且这是首位宠物
        $user_data = api_my_usersdata($uid);
        if (!empty($user_data['npcid']) && $user_data['npcid'] > 0 && $current_site === 1) {
            pm_abort_battle_transaction('战斗中的首位宠物无法移动');
        }

        // 检查目标位置是否已满
        if ($target_site === 1) {
            // 首位只能有一只宠物，需要先交换
            $existing_first = DB::fetch_first(pm_sql(
                "SELECT * FROM " . pm_table('pm_mypm') . " WHERE uid = %d AND site = 1",
                $uid
            ));
            if ($existing_first) {
                // 交换位置
                DB::query(pm_sql(
                    "UPDATE " . pm_table('pm_mypm') . " SET site = %d WHERE id = %d AND uid = %d",
                    $current_site,
                    intval($existing_first['id']),
                    $uid
                ));
            }
        }

        // 如果移动到仓库，需要确保背包至少有一个宠物（首位）
        if ($target_site === 3 && ($current_site === 1 || $current_site === 2)) {
            // 检查移动后背包是否还有宠物
            $bag_count = DB::result_first(
                "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = $uid AND site IN (1, 2)"
            );
            if ($bag_count <= 1) {
                pm_abort_battle_transaction('背包至少需要保留一只宠物');
            }

            // 如果移动的是首位宠物，需要自动指定新的首位
            if ($current_site === 1) {
                $next_first = DB::fetch_first(pm_sql(
                    "SELECT id FROM " . pm_table('pm_mypm') . " WHERE uid = %d AND site = 2 LIMIT 1",
                    $uid
                ));
                if ($next_first) {
                    DB::query(pm_sql(
                        "UPDATE " . pm_table('pm_mypm') . " SET site = 1 WHERE id = %d AND uid = %d AND site = 2",
                        intval($next_first['id']),
                        $uid
                    ));
                }
            }
        }

        // 如果从仓库移到背包，需要检查背包是否已满（最多6只）
        // 旧版数据用 site 3..N 表示多个箱子，仓库判定不能只认 site === 3
        if ($current_site >= 3 && ($target_site === 1 || $target_site === 2)) {
            $bag_count = DB::result_first(
                "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = $uid AND site IN (1, 2)"
            );
            if ($bag_count >= 6) {
                pm_abort_battle_transaction('背包已满（最多6只），请先将背包宠物放入仓库');
            }
        }

        // 移动宠物到目标位置；带上锁内读到的原位置作守卫（防御性，
        // 正常并发路径已被行锁串行化），位置意外变化时整体回滚
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET site = %d WHERE id = %d AND uid = %d AND site = %d",
            $target_site,
            $pokemon_id,
            $uid,
            $current_site
        ));
        if (!DB::affected_rows()) {
            pm_abort_battle_transaction('宝可梦位置已变化，请刷新后重试', 409);
        }

        $old_site = $current_site;
        DB::query("COMMIT");
    } catch (Throwable $txn_error) {
        DB::query("ROLLBACK");
        throw $txn_error;
    }

    api_success([
        'pokemon_id' => $pokemon_id,
        'old_site' => $old_site,
        'new_site' => $target_site,
        'message' => '宝可梦移动成功'
    ]);
}

/**
 * 交换两个宝可梦的位置
 */
function api_swap_pokemon()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    $pokemon_id_1 = validate_id(get_param('pokemon_id_1', 0), 'pokemon_id_1');
    $pokemon_id_2 = validate_id(get_param('pokemon_id_2', 0), 'pokemon_id_2');

    if ($pokemon_id_1 === $pokemon_id_2) {
        api_error('不能交换同一个宝可梦', 400);
    }

    // 验证两只宠物都属于当前用户（快速路径）
    $pokemon_1 = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pokemon_id_1,
        $uid
    ));
    $pokemon_2 = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pokemon_id_2,
        $uid
    ));

    if (!$pokemon_1) {
        api_error('第一个宝可梦不存在或不属于您', 403);
    }
    if (!$pokemon_2) {
        api_error('第二个宝可梦不存在或不属于您', 403);
    }

    // 交换是两笔互逆写入：与移动/设首位/战斗换宠共用 pm_usersdata 行锁，
    // 两个并发的交叉交换（A↔B 与 B↔C）不会丢失任何一次交换
    DB::query("START TRANSACTION");
    try {
        $owner = DB::fetch_first(pm_sql(
            "SELECT uid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $uid
        ));
        if (!$owner) {
            pm_abort_battle_transaction('用户状态不存在，请刷新后重试', 500);
        }

        // 锁内重读最新位置
        $pokemon_1 = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
            $pokemon_id_1,
            $uid
        ));
        $pokemon_2 = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
            $pokemon_id_2,
            $uid
        ));
        if (!$pokemon_1) {
            pm_abort_battle_transaction('第一个宝可梦不存在或不属于您', 403);
        }
        if (!$pokemon_2) {
            pm_abort_battle_transaction('第二个宝可梦不存在或不属于您', 403);
        }

        $site_1 = (int) $pokemon_1['site'];
        $site_2 = (int) $pokemon_2['site'];

        // 交换位置：写入带 uid 与锁内读到的原位置作守卫
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET site = %d WHERE id = %d AND uid = %d AND site = %d",
            $site_2,
            $pokemon_id_1,
            $uid,
            $site_1
        ));
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET site = %d WHERE id = %d AND uid = %d AND site = %d",
            $site_1,
            $pokemon_id_2,
            $uid,
            $site_2
        ));

        DB::query("COMMIT");
    } catch (Throwable $txn_error) {
        DB::query("ROLLBACK");
        throw $txn_error;
    }

    api_success([
        'pokemon_id_1' => $pokemon_id_1,
        'pokemon_id_2' => $pokemon_id_2,
        'old_site_1' => $site_1,
        'old_site_2' => $site_2,
        'new_site_1' => $site_2,
        'new_site_2' => $site_1,
        'message' => '宝可梦位置交换成功'
    ]);
}

/**
 * 设置首只宝可梦（插队式）
 *
 * 将指定的宝可梦设为首位（site=1）
 * 原首位宝可梦降为普通背包宠物（site=2）
 *
 * 例如：原顺序 1(首位),2,3,4,5,6，设置5为首位后变成 5(首位),1,2,3,4,6
 */
function api_set_first_pokemon()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    $pokemon_id = validate_id(get_param('pokemon_id', 0), 'pokemon_id');

    // 验证宠物所有权（快速路径）
    $pokemon = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
        $pokemon_id,
        $uid
    ));

    if (!$pokemon) {
        api_error('宝可梦不存在或不属于您', 403);
    }

    $current_site = (int) $pokemon['site'];

    // 如果已经是首位，无需操作
    if ($current_site === 1) {
        api_success(['message' => '该宝可梦已是首位']);
        return;
    }

    // 与其他 site 变更共用 pm_usersdata 行锁：两个并发的「设首位」
    // 串行执行，不会同时降位/升位出两个首位
    DB::query("START TRANSACTION");
    try {
        $owner = DB::fetch_first(pm_sql(
            "SELECT uid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $uid
        ));
        if (!$owner) {
            pm_abort_battle_transaction('用户状态不存在，请刷新后重试', 500);
        }

        // 锁内重读最新位置
        $pokemon = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d",
            $pokemon_id,
            $uid
        ));
        if (!$pokemon) {
            pm_abort_battle_transaction('宝可梦不存在或不属于您', 403);
        }

        $current_site = (int) $pokemon['site'];

        if ($current_site === 1) {
            DB::query("COMMIT");
            api_success(['message' => '该宝可梦已是首位']);
        }

        // 如果宠物在仓库（site >= 3，旧数据 3..N 表示多个箱子），不允许直接设为首位
        if ($current_site >= 3) {
            pm_abort_battle_transaction('请先将宝可梦移出仓库再设为首位');
        }

        // 将当前首位宝可梦降为普通背包宠物（site=2）
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET site = 2 WHERE uid = %d AND site = 1",
            $uid
        ));

        // 将指定宝可梦设为首位；带上锁内读到的原位置作守卫，位置意外
        // 变化（正常并发路径已被行锁串行化）时整体回滚
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET site = 1 WHERE id = %d AND uid = %d AND site = 2",
            $pokemon_id,
            $uid
        ));
        if (!DB::affected_rows()) {
            pm_abort_battle_transaction('宝可梦位置已变化，请刷新后重试', 409);
        }

        $old_site = $current_site;
        DB::query("COMMIT");
    } catch (Throwable $txn_error) {
        DB::query("ROLLBACK");
        throw $txn_error;
    }

    api_success([
        'pokemon_id' => $pokemon_id,
        'old_site' => $old_site,
        'new_site' => 1,
        'message' => '已设为首位宝可梦'
    ]);
}
