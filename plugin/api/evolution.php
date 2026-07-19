<?php

/**
 * 进化系统API
 *
 * 端点:
 * - GET  ?action=check&petid=xxx      检查进化条件
 * - POST ?action=evolve&petid=xxx    触发进化
 * - GET  ?action=available&petid=xxx 获取可进化形态
 */

// 加载Discuz环境
if (!defined('IN_DISCUZ')) {
    require_once __DIR__ . '/bootstrap.php';
}

// 加载API辅助函数
require_once __DIR__ . '/index.php';

global $_G;

$action = get_param('action', '');

switch ($action) {
    case 'check':
        api_check_evolution();
        break;

    case 'evolve':
        api_evolve_pokemon();
        break;

    case 'available':
        api_get_available_evolutions();
        break;

    case 'evolution_path':
        api_get_evolution_path();
        break;

    default:
        api_error('Invalid action', 400);
}

/**
 * 检查进化条件
 */
function api_check_evolution()
{
    require_login();

    $pet_id_param = get_param('petid', 0);
    if (!$pet_id_param) {
        api_error('Missing parameter: petid', 400);
    }
    $pet_id = validate_id($pet_id_param, 'pet_id');

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 获取宠物信息
    $pet = DB::fetch_first(
        "SELECT * FROM " . pm_table('pm_mypm') . "
        WHERE id = $pet_id AND uid = $uid"
    );

    if (!$pet) {
        api_error('Pokemon not found', 404);
    }

    // 获取宠物的进化信息
    $evolution_info = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_up') . "
        WHERE pmid = %d",
        $pet['pmno']
    ));

    if (!$evolution_info) {
        api_success([
            'can_evolve' => false,
            'reason' => 'This Pokemon cannot evolve',
            'current_form' => $pet['pmno'],
        ]);
    }

    // 检查进化条件
    $conditions = check_evolution_conditions($pet, $evolution_info);

    api_success([
        'can_evolve' => $conditions['can_evolve'],
        'pokemon_id' => $pet_id,
        'current_form' => (int) $pet['pmno'],
        'target_form' => (int) $evolution_info['targetpmid'],
        'conditions' => $conditions,
        'evolution_method' => $evolution_info['type'],
    ]);
}

/**
 * 触发进化
 */
function api_evolve_pokemon()
{
    require_login();

    $input = get_json_input();

    if (!isset($input['pet_id']) || !$input['pet_id']) {
        api_error('Missing parameter: pet_id', 400);
    }
    $pet_id = validate_id($input['pet_id'], 'pet_id');

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 获取宠物信息
    $pet = DB::fetch_first(
        "SELECT * FROM " . pm_table('pm_mypm') . "
        WHERE id = $pet_id AND uid = $uid"
    );

    if (!$pet) {
        api_error('Pokemon not found', 404);
    }

    // 获取进化信息
    $evolution_info = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_up') . "
        WHERE pmid = %d",
        $pet['pmno']
    ));

    if (!$evolution_info) {
        api_error('This Pokemon cannot evolve', 400);
    }

    // 检查进化条件
    $conditions = check_evolution_conditions($pet, $evolution_info);
    if (!$conditions['can_evolve']) {
        api_error('Evolution conditions not met: ' . $conditions['reason'], 400);
    }

    // 执行进化
    $new_type_id = $evolution_info['targetpmid'];

    // 获取新形态的基础信息
    $new_base_info = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d",
        $new_type_id
    ));

    if (!$new_base_info) {
        api_error("Evolution target not found: pm_data id=$new_type_id", 500);
    }

    $new_max_hp = calculate_max_hp($new_base_info, $pet['level']);

    // 更新宠物形态
    DB::query("UPDATE " . pm_table('pm_mypm') . " SET
        pmno = $new_type_id,
        hp = $new_max_hp,
        pmname = '" . addslashes($new_base_info['name']) . "',
        nowname = '" . addslashes($new_base_info['name']) . "'
        WHERE id = $pet_id");

    api_success([
        'message' => 'Evolution successful!',
        'previous_form' => (int) $pet['pmno'],
        'new_form' => (int) $new_type_id,
        'new_name' => $new_base_info['name'],
        'stats' => [
            'hp' => (int) $new_max_hp,
            'level' => (int) $pet['level'],
        ],
    ]);
}

/**
 * 获取可进化形态列表
 */
function api_get_available_evolutions()
{
    require_login();

    $pet_id = (int) get_param('petid', 0);
    if (!$pet_id) {
        api_error('Missing parameter: petid', 400);
    }

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 获取宠物信息
    $pet = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . "
        WHERE id = %d AND uid = %d",
        $pet_id,
        $uid
    ));

    if (!$pet) {
        api_error('Pokemon not found', 404);
    }

    // 获取所有可能的进化路线
    $rows = DB::fetch_all(pm_sql(
        "SELECT * FROM " . pm_table('pm_up') . "
        WHERE pmid = %d",
        $pet['pmno']
    ));

    $evolutions = [];
    foreach ($rows as $row) {
        $target_info = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d",
            $row['targetpmid']
        ));

        $conditions = check_evolution_conditions($pet, $row);

        $evolutions[] = [
            'from_id' => (int) $pet['pmno'],
            'to_id' => (int) $row['targetpmid'],
            'to_name' => $target_info ? $target_info['name'] : 'Unknown',
            'evolution_type' => $row['cond'],
            'can_evolve' => $conditions['can_evolve'],
            'conditions' => $conditions,
        ];
    }

    api_success([
        'pokemon_id' => $pet_id,
        'current_form' => (int) $pet['pmno'],
        'available_evolutions' => $evolutions,
    ]);
}

/**
 * 检查进化条件
 */
function check_evolution_conditions($pet, $evolution_info)
{
    $can_evolve = true;
    $unmet_reasons = [];

    $cond_type = isset($evolution_info['cond']) ? $evolution_info['cond'] : '';
    $cond_value = isset($evolution_info['val']) ? $evolution_info['val'] : '';

    // 检查等级条件
    if ($cond_type === 'level') {
        $required_level = (int) $cond_value;
        if ((int) $pet['level'] < $required_level) {
            $can_evolve = false;
            $unmet_reasons[] = "Level must be at least $required_level (current: {$pet['level']})";
        }
    }

    // 检查道具条件
    if ($cond_type === 'item') {
        $required_item_id = (int) $cond_value;
        $has_item = check_user_has_item($pet['uid'], $required_item_id);

        if (!$has_item) {
            $can_evolve = false;
            $item_info = DB::fetch_first(
                "SELECT name FROM " . pm_table('pm_itemdata') . " WHERE id = $required_item_id"
            );
            $unmet_reasons[] = "Requires item: " . ($item_info ? $item_info['name'] : "ID $required_item_id");
        }
    }

    // 检查亲密度条件
    if ($cond_type === 'good') {
        $required_intimacy = (int) $cond_value;
        if ((int) $pet['good'] < $required_intimacy) {
            $can_evolve = false;
            $unmet_reasons[] = "Intimacy must be at least $required_intimacy (current: {$pet['good']})";
        }
    }

    return [
        'can_evolve' => $can_evolve,
        'reason' => $can_evolve ? 'All conditions met' : implode(', ', $unmet_reasons),
        'details' => [
            'level_requirement' => $cond_type === 'level' ? (int) $cond_value : null,
            'item_requirement' => $cond_type === 'item' ? (int) $cond_value : null,
            'intimacy_requirement' => $cond_type === 'good' ? (int) $cond_value : null,
        ],
    ];
}

/**
 * 检查用户是否拥有某个道具
 */
function check_user_has_item($uid, $item_id)
{
    $item = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_myitem') . "
        WHERE uid = %d AND itemid = %s AND num > 0",
        $uid, strval($item_id)
    ));

    return $item !== null;
}

/**
 * 计算最大HP（简化版）
 */
function calculate_max_hp($base_info, $level)
{
    $base_hp = (int) $base_info['hp'];
    return floor(($level / 100.0) * (2 * $base_hp + 31) + $level + 10);
}

/**
 * 获取进化路径（正向 + 反向）
 * 正向：由 pmid 可以进化到哪些目标
 * 反向：由哪些源头可以进化到 pmid
 */
function api_get_evolution_path()
{
    $pmid = (int) get_param('pmid', 0);
    if (!$pmid) {
        api_error('Missing parameter: pmid', 400);
    }

    $pokemon_info = DB::fetch_first(pm_sql(
        "SELECT name, xs, xs2 FROM " . pm_table('pm_data') . " WHERE id = %d",
        $pmid
    ));

    if (!$pokemon_info) {
        api_error('Pokemon not found', 404);
    }

    $forward = [];
    $forward_rows = DB::fetch_all(pm_sql(
        "SELECT u.*, d.name as target_name, d.xs as target_type1, d.xs2 as target_type2
        FROM " . pm_table('pm_up') . " u
        JOIN " . pm_table('pm_data') . " d ON u.targetpmid = d.id
        WHERE u.pmid = %d
        ORDER BY u.priority",
        $pmid
    ));

    foreach ($forward_rows as $row) {
        $cond_type = $row['cond'];
        $cond_value = $row['val'];
        $cond_display = _format_evolution_condition($cond_type, $cond_value);

        $forward[] = [
            'id' => (int) $row['targetpmid'],
            'name' => $row['target_name'],
            'type_1' => $row['target_type1'],
            'type_2' => $row['target_type2'] ? $row['target_type2'] : null,
            'condition_type' => $cond_type,
            'condition_value' => $cond_value,
            'condition_display' => $cond_display,
        ];
    }

    $backward = [];
    $backward_rows = DB::fetch_all(pm_sql(
        "SELECT u.*, d.name as source_name, d.xs as source_type1, d.xs2 as source_type2
        FROM " . pm_table('pm_up') . " u
        JOIN " . pm_table('pm_data') . " d ON u.pmid = d.id
        WHERE u.targetpmid = %d
        ORDER BY u.priority",
        $pmid
    ));

    foreach ($backward_rows as $row) {
        $cond_type = $row['cond'];
        $cond_value = $row['val'];
        $cond_display = _format_evolution_condition($cond_type, $cond_value);

        $backward[] = [
            'id' => (int) $row['pmid'],
            'name' => $row['source_name'],
            'type_1' => $row['source_type1'],
            'type_2' => $row['source_type2'] ? $row['source_type2'] : null,
            'condition_type' => $cond_type,
            'condition_value' => $cond_value,
            'condition_display' => $cond_display,
        ];
    }

    api_success([
        'pokemon_id' => $pmid,
        'pokemon_name' => $pokemon_info['name'],
        'pokemon_type1' => $pokemon_info['xs'],
        'pokemon_type2' => $pokemon_info['xs2'] ? $pokemon_info['xs2'] : null,
        'forward' => $forward,
        'backward' => $backward,
    ]);
}

function _format_evolution_condition($cond_type, $cond_value)
{
    switch ($cond_type) {
        case 'level':
            return "等级达到 Lv.{$cond_value}";
        case 'item':
            $item_info = DB::fetch_first(
                "SELECT name FROM " . pm_table('pm_itemdata') . " WHERE id = " . intval($cond_value)
            );
            $item_name = $item_info ? $item_info['name'] : "未知道具(ID:{$cond_value})";
            return "使用 {$item_name}";
        case 'good':
            return "亲密度达到 {$cond_value}";
        default:
            return $cond_type . ': ' . $cond_value;
    }
}
