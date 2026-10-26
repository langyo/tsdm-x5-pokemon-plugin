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
require_once __DIR__ . '/utils.php';

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

    $pet_id_param = get_param('pet_id', get_param('petid', 0));
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

    // 评估该物种的全部候选进化路线
    $routes = evaluate_evolution_routes($pet, $pet['species_id']);

    if (!$routes) {
        api_success([
            'can_evolve' => false,
            'reason' => 'This Pokemon cannot evolve',
            'pokemon_id' => $pet_id,
            'current_form' => (int) $pet['species_id'],
            'target_form' => (int) $pet['species_id'],
            'evolution_method' => '',
            'conditions' => [
                'can_evolve' => false,
                'reason' => 'This Pokemon cannot evolve',
                'details' => [
                    'level_requirement' => null,
                    'item_requirement' => null,
                    'intimacy_requirement' => null,
                ],
            ],
        ]);
    }

    // 默认报告第一条可满足的路线；全部不满足时报告首条路线的缺口
    $selected = null;
    foreach ($routes as $route) {
        if ($route['can_evolve']) {
            $selected = $route;
            break;
        }
    }
    if (!$selected) {
        $selected = $routes[0];
    }

    $conditions = [
        'can_evolve' => $selected['can_evolve'],
        'reason' => $selected['can_evolve'] ? 'All conditions met' : implode(', ', $selected['reasons']),
        'details' => $selected['details'],
    ];

    api_success([
        'can_evolve' => $conditions['can_evolve'],
        'pokemon_id' => $pet_id,
        'current_form' => (int) $pet['species_id'],
        'target_form' => (int) $selected['to_id'],
        'conditions' => $conditions,
        'reason' => $conditions['reason'],
        'evolution_method' => $selected['method'],
        'available_evolutions' => _build_available_evolution_entries($pet, $routes),
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

    // 与队伍和商店变更使用相同锁顺序；进化与道具消耗必须一同提交。
    DB::query("START TRANSACTION");
    try {
        $owner = DB::fetch_first(pm_sql(
            "SELECT uid, npcid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE", $uid
        ));
        if (!$owner) {
            api_my_usersdata($uid);
            $owner = DB::fetch_first(pm_sql(
                "SELECT uid, npcid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE", $uid
            ));
            if (!$owner) {
                pm_abort_battle_transaction('User state not found', 500);
            }
        }
        $pet = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d AND uid = %d FOR UPDATE",
            $pet_id, $uid
        ));
        if (!$pet) {
            pm_abort_battle_transaction('Pokemon not found', 404);
        }
        if ((int) $pet['site'] === 1 && (int) $owner['npcid'] > 0) {
            pm_abort_battle_transaction('战斗中的首位宠物无法进化', 400);
        }

        // 评估全部候选路线；同一目标的复合规则在评估器内按 AND 判定
        $routes = evaluate_evolution_routes($pet, $pet['species_id']);
        if (!$routes) {
            pm_abort_battle_transaction('This Pokemon cannot evolve', 400);
        }

        // 请求体可指定目标路线；未指定时按序取第一条满足的路线
        $selected = null;
        if (isset($input['to_id'])) {
            // 非数值目标按未知目标拒绝，避免数组等脏输入落入默认路线
            if (!is_numeric($input['to_id'])) {
                pm_abort_battle_transaction('Evolution target not available for this Pokemon', 400);
            }
            $requested_to_id = (int) $input['to_id'];
            foreach ($routes as $route) {
                if ((int) $route['to_id'] === $requested_to_id) {
                    $selected = $route;
                    break;
                }
            }
            if (!$selected) {
                pm_abort_battle_transaction('Evolution target not available for this Pokemon', 400);
            }
        } else {
            foreach ($routes as $route) {
                if ($route['can_evolve']) {
                    $selected = $route;
                    break;
                }
            }
        }
        if (!$selected || !$selected['can_evolve']) {
            $blocked = $selected ? $selected : $routes[0];
            pm_abort_battle_transaction('Evolution conditions not met: ' . implode(', ', $blocked['reasons']), 400);
        }

        $new_type_id = (int) $selected['to_id'];
        $new_base_info = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d",
            $new_type_id
        ));
        if (!$new_base_info) {
            pm_abort_battle_transaction("Evolution target not found: pm_data id=$new_type_id", 500);
        }

        // 复合路线可能包含多条道具规则；逐条锁定并各扣一个
        foreach ($selected['rules'] as $rule) {
            if ($rule['method'] !== 'item') {
                continue;
            }
            $item = DB::fetch_first(pm_sql(
                "SELECT id FROM " . pm_table('pm_myitem') . " WHERE uid = %d AND itemid = %s AND nums > 0 ORDER BY id LIMIT 1 FOR UPDATE",
                $uid, strval((int) $rule['condition_value'])
            ));
            if (!$item) {
                pm_abort_battle_transaction('Required evolution item is no longer available', 409);
            }
            DB::query(pm_sql(
                "UPDATE " . pm_table('pm_myitem') . " SET nums = nums - 1 WHERE id = %d AND uid = %d AND nums > 0",
                $item['id'], $uid
            ));
            if (!DB::affected_rows()) {
                pm_abort_battle_transaction('Required evolution item is no longer available', 409);
            }
        }

        $evolved_pet = $pet;
        $evolved_pet['species_id'] = $new_type_id;
        $new_max_hp = api_calculate_pokemon_max_hp($evolved_pet, $new_base_info);
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET species_id = %d, hp = %d, pmname = %s, nickname = %s WHERE id = %d AND uid = %d",
            $new_type_id, $new_max_hp, $new_base_info['name'], $new_base_info['name'], $pet_id, $uid
        ));
        DB::query("COMMIT");
    } catch (Throwable $error) {
        DB::query("ROLLBACK");
        throw $error;
    }

    api_success([
        'message' => 'Evolution successful!',
        'previous_form' => (int) $pet['species_id'],
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

    $pet_id = (int) get_param('pet_id', get_param('petid', 0));
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

    // 评估全部候选进化路线
    $routes = evaluate_evolution_routes($pet, $pet['species_id']);

    api_success([
        'pokemon_id' => $pet_id,
        'current_form' => (int) $pet['species_id'],
        'available_evolutions' => _build_available_evolution_entries($pet, $routes),
    ]);
}

/**
 * 评估某个物种的全部候选进化路线
 *
 * 同一 to_id 的多行规则是复合 AND 条件（组内全部满足才算满足）；
 * 不同 to_id 是候选路线，按 priority, id 的先后顺序返回。
 */
function evaluate_evolution_routes($pet, $from_id)
{
    $rows = DB::fetch_all(pm_sql(
        "SELECT * FROM " . pm_table('pm_evolution') . " WHERE from_id = %d ORDER BY priority, id",
        $from_id
    ));

    // 按 to_id 分组，保持首次出现的顺序
    $groups = [];
    foreach ($rows as $row) {
        $groups[(int) $row['to_id']][] = $row;
    }

    $routes = [];
    foreach ($groups as $rules) {
        $route = _evaluate_evolution_rule_group($pet, $rules);

        // 展示文案：组内每条规则各生成一段，用「且」连接
        $displays = [];
        foreach ($rules as $rule) {
            $displays[] = _format_evolution_condition($rule['method'], $rule['condition_value']);
        }
        $route['condition_display'] = implode(' 且 ', $displays);

        $routes[] = $route;
    }

    return $routes;
}

/**
 * 评估单条路线（一组 AND 规则）；自身不发 pm_evolution 查询，供评估器与单行包装复用
 *（item 规则未满足时会查询道具名称用于生成原因文案）
 */
function _evaluate_evolution_rule_group($pet, array $rules)
{
    $first = $rules[0];
    $can_evolve = true;
    $reasons = [];
    $details = [
        'level_requirement' => null,
        'item_requirement' => null,
        'intimacy_requirement' => null,
    ];

    foreach ($rules as $rule) {
        $cond_type = isset($rule['method']) ? $rule['method'] : '';
        $cond_value = isset($rule['condition_value']) ? $rule['condition_value'] : '';

        if ($cond_type === 'level') {
            // 检查等级条件
            $required_level = (int) $cond_value;
            $details['level_requirement'] = $required_level;
            if ((int) $pet['level'] < $required_level) {
                $can_evolve = false;
                $reasons[] = "Level must be at least $required_level (current: {$pet['level']})";
            }
        } elseif ($cond_type === 'item') {
            // 检查道具条件
            $required_item_id = (int) $cond_value;
            $details['item_requirement'] = $required_item_id;
            if (!check_user_has_item($pet['uid'], $required_item_id)) {
                $can_evolve = false;
                $item_info = DB::fetch_first(
                    "SELECT name FROM " . pm_table('pm_itemdata') . " WHERE id = $required_item_id"
                );
                $reasons[] = "Requires item: " . ($item_info ? $item_info['name'] : "ID $required_item_id");
            }
        } elseif ($cond_type === 'good') {
            // 检查亲密度条件
            $required_intimacy = (int) $cond_value;
            $details['intimacy_requirement'] = $required_intimacy;
            if ((int) $pet['good'] < $required_intimacy) {
                $can_evolve = false;
                $reasons[] = "Intimacy must be at least $required_intimacy (current: {$pet['good']})";
            }
        } else {
            // 尚未实现的规则不能被当作无条件进化。
            $can_evolve = false;
            $reasons[] = 'Unsupported evolution method: ' . $cond_type;
        }
    }

    return [
        'to_id' => (int) $first['to_id'],
        'rules' => $rules,
        'can_evolve' => $can_evolve,
        'reasons' => $reasons,
        'method' => isset($first['method']) ? $first['method'] : '',
        'details' => $details,
    ];
}

/**
 * 把评估出的路线列表组装成 available_evolutions 响应条目
 */
function _build_available_evolution_entries($pet, $routes)
{
    $evolutions = [];
    foreach ($routes as $route) {
        $target_info = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d",
            $route['to_id']
        ));

        $evolutions[] = [
            'from_id' => (int) $pet['species_id'],
            'to_id' => (int) $route['to_id'],
            'to_name' => $target_info ? $target_info['name'] : 'Unknown',
            'evolution_type' => $route['method'],
            'can_evolve' => $route['can_evolve'],
            'conditions' => [
                'can_evolve' => $route['can_evolve'],
                'reason' => $route['can_evolve'] ? 'All conditions met' : implode(', ', $route['reasons']),
                'details' => $route['details'],
            ],
            'condition_display' => $route['condition_display'],
        ];
    }

    return $evolutions;
}

/**
 * 检查进化条件（单行规则的薄包装，保持旧签名与旧返回形态）
 */
function check_evolution_conditions($pet, $evolution_info)
{
    // 复合条件改由评估器的分组机制处理；这里只评估传入的单行规则，不发 pm_evolution 查询（item 未满足时仍会查询道具名称）
    $route = _evaluate_evolution_rule_group($pet, [$evolution_info]);

    return [
        'can_evolve' => $route['can_evolve'],
        'reason' => $route['can_evolve'] ? 'All conditions met' : implode(', ', $route['reasons']),
        'details' => $route['details'],
    ];
}

/**
 * 检查用户是否拥有某个道具
 */
function check_user_has_item($uid, $item_id)
{
    $item = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_myitem') . "
        WHERE uid = %d AND itemid = %s AND nums > 0",
        $uid, strval($item_id)
    ));

    return !empty($item);
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
        FROM " . pm_table('pm_evolution') . " u
        JOIN " . pm_table('pm_data') . " d ON u.to_id = d.id
        WHERE u.from_id = %d
        ORDER BY u.priority",
        $pmid
    ));

    foreach ($forward_rows as $row) {
        $cond_type = $row['method'];
        $cond_value = $row['condition_value'];
        $cond_display = _format_evolution_condition($cond_type, $cond_value);

        $forward[] = [
            'id' => (int) $row['to_id'],
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
        FROM " . pm_table('pm_evolution') . " u
        JOIN " . pm_table('pm_data') . " d ON u.from_id = d.id
        WHERE u.to_id = %d
        ORDER BY u.priority",
        $pmid
    ));

    foreach ($backward_rows as $row) {
        $cond_type = $row['method'];
        $cond_value = $row['condition_value'];
        $cond_display = _format_evolution_condition($cond_type, $cond_value);

        $backward[] = [
            'id' => (int) $row['from_id'],
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
