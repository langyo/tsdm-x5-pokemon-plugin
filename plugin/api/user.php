<?php

/**
 * 用户数据API
 *
 * 端点:
 * - GET  ?action=profile          获取用户资料
 * - GET  ?action=inventory        获取背包物品
 * - GET  ?action=stats            获取游戏统计
 * - GET  ?action=online_players   获取在线玩家
 * - POST ?action=use_item         使用物品
 */

// 如果通过路由访问，加载API辅助函数
if (defined('API_ROUTED')) {
    require_once __DIR__ . '/index.php';
} elseif (!defined('IN_DISCUZ')) {
    require_once __DIR__ . '/bootstrap.php';
    require_once __DIR__ . '/index.php';
}

// 加载常量定义
require_once __DIR__ . '/constants.php';

// 加载API工具函数（包含经验计算、状态修正等）
require_once __DIR__ . '/utils.php';

// 加载物品模块函数
require_once __DIR__ . '/item_modules.php';

global $_G;

$action = get_param('action', '');

switch ($action) {
    case 'profile':
        api_get_user_profile();
        break;

    case 'inventory':
        api_get_inventory();
        break;

    case 'stats':
        api_get_user_stats();
        break;

    case 'inventory_stats':
        api_get_inventory_stats();
        break;

    case 'heal':
        api_heal_pokemon();
        break;

    case 'heal_and_flee':
        api_heal_and_flee();
        break;

    case 'use_item':
        api_use_item();
        break;

    case 'get_usable_pokemon':
        api_get_usable_pokemon();
        break;

    case 'online_players':
        api_get_online_players();
        break;

    case 'initialize':
        api_initialize_new_player();
        break;

    case 'refresh_badge':
        api_refresh_forum_badge();
        break;

    default:
        api_error('Invalid action', 400);
}

/**
 * 获取用户资料
 */
function api_get_user_profile()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 获取基础用户信息
    $user_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $uid
    ));

    $is_new_player = false;

    if (!$user_data) {
        // 标记为新玩家，但不自动创建数据
        $is_new_player = true;
    }

    // 获取论坛用户信息
    $member = DB::fetch_first(pm_sql(
        "SELECT uid, username, groupid FROM " . DB::table('common_member') . " WHERE uid = %d",
        $uid
    ));

    // 无论是否有 usersdata，都查询实际的宠物数量和物品数量
    // 因为用户可能通过旧版系统领取了宠物，但没有 usersdata 记录
    $actual_pokemon_count = (int) DB::result_first(pm_sql(
        "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = %d",
        $uid
    ));
    $actual_item_count = (int) DB::result_first(pm_sql(
        "SELECT COUNT(*) FROM " . pm_table('pm_myitem') . " WHERE uid = %d",
        $uid
    ));

    // 如果用户有宠物但没有 usersdata，说明是旧版用户，需要创建 usersdata
    if (!$user_data && $actual_pokemon_count > 0) {
        // 自动为旧版用户创建 usersdata 记录
        DB::query(pm_sql(
            "INSERT INTO " . pm_table('pm_usersdata') . "
            (uid, money, datawin, datalost, dataall, strength)
            VALUES (%d, 0, 0, 0, 0, 1)",
            $uid
        ));
        $user_data = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
            $uid
        ));
        $is_new_player = false;
    }

    $is_admin = false;
    $group_id = (int) $member['groupid'];
    if ($group_id == 1 || $group_id == 2 || $group_id == 3) {
        $is_admin = true;
    }
    if (!$is_admin) {
        $settings = isset($_G['cache']['plugin']['pokemon']) ? $_G['cache']['plugin']['pokemon'] : array();
        $admins = isset($settings['poke_smgly']) ? explode(',', $settings['poke_smgly']) : array();
        $admins = array_map('trim', $admins);
        if (in_array($member['username'], $admins)) {
            $is_admin = true;
        }
    }

    api_success([
        'uid' => (int) $uid,
        'username' => $member['username'],
        'group_id' => (int) $member['groupid'],
        'is_admin' => $is_admin,
        'is_new_player' => $is_new_player,
        'money' => $user_data ? (int) $user_data['money'] : null,
        'adventure_strength' => $user_data ? (int) $user_data['str'] : null,
        'strength_level' => $user_data ? (int) $user_data['strength'] : null,
        'wins' => $user_data ? (int) $user_data['datawin'] : null,
        'losses' => $user_data ? (int) $user_data['datalost'] : null,
        'total_battles' => $user_data ? (int) $user_data['dataall'] : null,
        'total_pokemons' => $actual_pokemon_count,
        'total_items' => $actual_item_count,
        'npcid' => $user_data ? (int) $user_data['npcid'] : 0,
        '_debug' => [
            'has_usersdata' => $user_data ? true : false,
        ],
    ]);
}

/**
 * 获取背包物品
 */
function api_get_inventory()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    $page = (int) get_param('page', 1);
    $per_page = USER_ITEMS_PER_PAGE;

    // type=0 或未传参 表示查看全部，不限制类型
    // 使用 isset 来检测参数是否传递
    $has_type_param = isset($_GET['type']) || isset($_POST['type']);
    $item_type = 0;
    if ($has_type_param) {
        $item_type = (int) get_param('type', 0);
    }

    // 使用 LEFT JOIN 来关联物品表，根据物品的实际 type 字段过滤
    $where_params = ["m.uid = %d"];
    $where_values = [$uid];

    if ($has_type_param && $item_type > 0) {
        $where_params[] = "i.type = %d";
        $where_values[] = $item_type;
    }

    $where_sql = "WHERE " . implode(' AND ', $where_params);

    // 获取总数 - 使用 JOIN
    $count_sql = "SELECT COUNT(*) as total FROM " . pm_table('pm_myitem') . " m
        LEFT JOIN " . pm_table('pm_itemdata') . " i ON m.itemid = i.id
        " . $where_sql;
    $total = (int) DB::result_first(pm_sql_v($count_sql, $where_values));

    // 分页查询
    $offset = ($page - 1) * $per_page;
    $limit_sql = "SELECT m.*, i.type as item_type, i.name as item_name, i.description as item_desc, i.tpname as item_image, i.id as itemdata_id
        FROM " . pm_table('pm_myitem') . " m
        LEFT JOIN " . pm_table('pm_itemdata') . " i ON m.itemid = i.id
        " . $where_sql . " ORDER BY m.id DESC LIMIT %d, %d";

    $rows = DB::fetch_all(pm_sql_v($limit_sql, array_merge($where_values, array($offset, $per_page))));

    $items = [];

    foreach ($rows as $row) {
        // 使用 JOIN 获取的 item_type
        $item_type_from_item = isset($row['item_type']) && $row['item_type'] !== null
            ? (int) $row['item_type']
            : 0;
        
        $typeid = isset($row['itemdata_id']) ? (int)$row['itemdata_id'] : 0;

        $items[] = [
            'id' => (int) $row['id'],
            'type_id' => $typeid,
            'item_type' => $item_type_from_item,
            'name' => isset($row['item_name']) && $row['item_name'] ? $row['item_name'] : '未知',
            'description' => isset($row['item_desc']) ? $row['item_desc'] : '',
            'image' => isset($row['item_image']) ? $row['item_image'] : '',
            'quantity' => (int) $row['nums'],
            'type_name' => get_item_type_name($item_type_from_item),
            'can_use' => $row['nums'] > 0,
        ];
    }

    api_success([
        'items' => $items,
        'total' => $total,
        'page' => $page,
        'per_page' => $per_page,
        'total_pages' => $total > 0 ? (int) ceil($total / $per_page) : 0,
    ]);
}

/**
 * 获取游戏统计
 */
function api_get_user_stats()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 获取宠物统计
    $pokemon_stats = DB::fetch_first(
        "SELECT
 COUNT(*) as total,
            COUNT(CASE WHEN site=1 THEN 1 END) as active,
            SUM(level) as total_levels,
            MAX(level) as max_level FROM " . pm_table('pm_mypm') . "
            WHERE uid=%d",
        $uid
    );

    // 获取物品统计
    $item_stats = DB::fetch_first(
        "SELECT
 COUNT(*) as total_items,
            SUM(nums) as total_quantity FROM " . pm_table('pm_myitem') . "
            WHERE uid=%d",
        $uid
    );

    // 获取战斗统计（简化版）
    $battle_stats = [
        'total_battles' => 0,
        'victories' => 0,
        'defeats' => 0,
    ];

    // PHP 5.6兼容：使用三元运算符替代??
    $total_items = isset($item_stats['total_items']) ? $item_stats['total_items'] : 0;
    $total = isset($pokemon_stats['total']) ? $pokemon_stats['total'] : 0;
    $active = isset($pokemon_stats['active']) ? $pokemon_stats['active'] : 0;
    $total_levels = isset($pokemon_stats['total_levels']) ? $pokemon_stats['total_levels'] : 0;
    $max_level = isset($pokemon_stats['max_level']) ? $pokemon_stats['max_level'] : 0;
    $total_quantity = isset($item_stats['total_quantity']) ? $item_stats['total_quantity'] : 0;

    // 获取成就进度
    $achievements = [
        'first_pokemon' => $total > 0,
        'pokemon_master' => $total >= 151,
        // 收集所有宝可梦
        'high_level' => $max_level >= HIGH_LEVEL_THRESHOLD,
        'collector' => $total_items >= COLLECTOR_THRESHOLD,
    ];

    api_success([
        'pokemon_stats' => [
            'total_owned' => (int) $total,
            'active_pokemon' => (int) $active,
            'total_levels' => (int) $total_levels,
            'highest_level' => (int) $max_level,
        ],
        'inventory_stats' => [
            'total_items' => (int) $total_items,
            'total_quantity' => (int) $total_quantity,
        ],
        'battle_stats' => $battle_stats,
        'achievements' => $achievements,
    ]);
}

/**
 * 获取物品类型名称（辅助函数）
 */
function get_item_type_name($type_id)
{
    static $types = [
        1 => '回复药',
        2 => '精灵球',
        3 => '进化石',
        4 => '强化道具',
        5 => '装备道具',
    ];

    return isset($types[$type_id]) ? $types[$type_id] : '未知类型';
}

/**
 * 获取背包物品类型统计
 */
function api_get_inventory_stats()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    $result = [
        1 => 0,  // 回复药
        2 => 0,  // 精灵球
        3 => 0,  // 进化石
        4 => 0,  // 强化道具
        5 => 0,  // 特殊道具
    ];

    // 查询用户的物品，并关联 pm_itemdata 获取正确的 type 字段
    $rows = DB::fetch_all(pm_sql(
        "SELECT m.itemid, m.nums, i.type as item_type FROM " . pm_table('pm_myitem') . " m
        LEFT JOIN " . pm_table('pm_itemdata') . " i ON m.itemid = i.id
        WHERE m.uid = %d AND m.nums > 0",
        (int)$uid
    ));

    foreach ($rows as $row) {
        // 使用 pm_itemdata 的 type 字段
        $type_id = isset($row['item_type']) && $row['item_type'] !== null
            ? (int) $row['item_type']
            : 0;

        if ($type_id > 0 && isset($result[$type_id])) {
            $result[$type_id] += (int) $row['nums'];
        }
    }

    api_success([
        'categories' => [
            ['type_id' => 1, 'count' => $result[1]],
            ['type_id' => 2, 'count' => $result[2]],
            ['type_id' => 3, 'count' => $result[3]],
            ['type_id' => 4, 'count' => $result[4]],
            ['type_id' => 5, 'count' => $result[5]],
        ],
    ]);
}

/**
 * 治疗宝可梦
 */
function api_heal_pokemon()
{
    require_login();

    global $_G, $statehp;

    $uid = validate_uid($_G['uid']);
    $pokemon_id = validate_id(get_param('pokemon_id', 0), 'pokemon_id');

    // 加载必要的函数
    // 加载状态修正变量（$statehp, $stateatk 等）

    // 验证宠物所有权 - 使用 pm_table 确保表名一致
    $pokemon_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d",
        $pokemon_id
    ));

    if (!$pokemon_data || $pokemon_data['uid'] != $uid) {
        api_error('宝可梦不存在或不属于您', 403);
    }

    // 获取用户数据
    $user_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $uid
    ));

    if (!$user_data) {
        api_error('用户数据不存在', 500);
    }

    // 检查宠物是否在战斗中
    $is_in_battle = !empty($user_data['npcid']) && $user_data['npcid'] > 0;
    $is_battle_pokemon = (int)$pokemon_data['site'] === 1;

    if ($is_in_battle && $is_battle_pokemon) {
        api_error('该宠物正在战斗中，请使用脱战并治疗功能', 400);
    }

    // 治疗免费
    $cost = 0;

    // 使用统一计算函数获取最大 HP
    $petmaxhp = api_calculate_pokemon_max_hp($pokemon_data);

    // 计算装备加成
    api_parse_pet_wear_items($pokemon_data, false, $petmaxhp);

    // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
    $pokemon_data['hp'] = strval($petmaxhp);
    $hp_validation = api_validate_and_correct_hp($pokemon_data, $petmaxhp, $petmaxhp);
    $petmaxhp = $hp_validation['hp'];

    // 处理宠物状态
    $timestamp = time();
    $state_sql = '';
    $needs_healing = false;

    // 负面状态列表（需要治疗的异常状态）
    // 状态码说明：
    // 0 = 濒危/晕倒（负面，需要治疗）
    // 1 = 正常（不需要治疗）
    // 2-4 = 生病（负面，需要治疗）
    // 5-6 = 饥饿（负面，需要治疗）
    // 7 = 疲惫（负面，需要治疗）
    // 8-10 = 兴奋（正面，不治疗）
    // 11 = 受伤（负面，需要治疗）
    // 12-14 = 快乐（正面，不治疗）
    // 15 = 惊慌（负面，需要治疗）
    // 16-17 = 自恋（中性，不治疗）
    // 18-19 = 愤怒（中性，不治疗）
    $negative_states = [0, 2, 3, 4, 5, 6, 7, 11, 15];

    // 检查是否需要治疗状态
    if ($pokemon_data['hp'] <= 0) {
        // 宠物晕倒，需要恢复状态
        $needs_healing = true;
    } elseif (in_array((int)$pokemon_data['state'], $negative_states)) {
        // 负面状态需要治疗
        $needs_healing = true;
    }

    if ($needs_healing) {
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET hp=%d, state='1', statetime=%d WHERE id=%d AND uid=%d",
            $petmaxhp,
            $timestamp,
            $pokemon_id,
            $uid
        ));
    } else {
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET hp=%d WHERE id=%d AND uid=%d",
            $petmaxhp,
            $pokemon_id,
            $uid
        ));
    }

    // 恢复技能PP值
    $all_skills = DB::fetch_all(pm_sql(
        "SELECT * FROM " . pm_table('pm_myskill') . " WHERE uid=%d AND petid=%d",
        $uid,
        $pokemon_id
    ));
    foreach ($all_skills as $my_skill) {
        $skill_id = $my_skill['skillid'];
        $skill = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_skill') . " WHERE id=%d",
            $skill_id
        ));
        if ($skill && isset($skill['max_uses'])) {
            DB::query(pm_sql(
                "UPDATE " . pm_table('pm_myskill') . " SET skillnum=%d WHERE skillid=%d AND uid=%d AND petid=%d",
                $skill['max_uses'],
                $skill_id,
                $uid,
                $pokemon_id
            ));
        }
    }

    api_success([
        'cost' => $cost,
        'message' => "{$pokemon_data['nickname']}已治疗，花费 {$cost} 金币",
        'pokemon_id' => $pokemon_id,
        'current_hp' => (int)$petmaxhp,
        'max_hp' => (int)$petmaxhp,
    ]);
}

/**
 * 脱战并治疗宝可梦（宠物中心绿色通道）
 * 一次性完成脱战和治疗，避免战斗中治疗的漏洞
 */
function api_heal_and_flee()
{
    require_login();

    global $_G, $statehp;

    $uid = validate_uid($_G['uid']);
    $pokemon_id = validate_id(get_param('pokemon_id', 0), 'pokemon_id');


    $pokemon_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_mypm') . " WHERE id = %d",
        $pokemon_id
    ));

    if (!$pokemon_data || $pokemon_data['uid'] != $uid) {
        api_error('宝可梦不存在或不属于您', 403);
    }

    $user_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $uid
    ));

    if (!$user_data) {
        api_error('用户数据不存在', 500);
    }

    $is_in_battle = !empty($user_data['npcid']) && $user_data['npcid'] > 0;
    $is_battle_pokemon = (int)$pokemon_data['site'] === 1;

    if (!$is_in_battle || !$is_battle_pokemon) {
        api_error('该宠物不在战斗中，请使用普通治疗', 400);
    }

    // 清除战斗状态
    DB::query(pm_sql("UPDATE " . pm_table('pm_usersdata') . "
        SET npcid = '', level = '', hp = '', hpg = '', atkg = '', defg = '',
            spatkg = '', spdefg = '', sdg = '', allure = '', capture = ''
        WHERE uid = %d", $uid));

    $cost = 0;

    // 使用统一计算函数获取最大 HP
    $petmaxhp = api_calculate_pokemon_max_hp($pokemon_data);

    api_parse_pet_wear_items($pokemon_data, false, $petmaxhp);

    // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
    $pokemon_data['hp'] = strval($petmaxhp);
    $hp_validation = api_validate_and_correct_hp($pokemon_data, $petmaxhp, $petmaxhp);
    $petmaxhp = $hp_validation['hp'];

    $timestamp = time();
    $state_sql = '';
    $needs_healing = false;

    // 负面状态列表（需要治疗的异常状态）
    $negative_states = [0, 2, 3, 4, 5, 6, 7, 11, 15];

    // 检查是否需要治疗状态
    if ($pokemon_data['hp'] <= 0) {
        // 宠物晕倒，需要恢复状态
        $needs_healing = true;
    } elseif (in_array((int)$pokemon_data['state'], $negative_states)) {
        // 负面状态需要治疗
        $needs_healing = true;
    }

    if ($needs_healing) {
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET hp=%d, state='1', statetime=%d WHERE id=%d AND uid=%d",
            $petmaxhp,
            $timestamp,
            $pokemon_id,
            $uid
        ));
    } else {
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET hp=%d WHERE id=%d AND uid=%d",
            $petmaxhp,
            $pokemon_id,
            $uid
        ));
    }

    $all_skills = DB::fetch_all(pm_sql(
        "SELECT * FROM " . pm_table('pm_myskill') . " WHERE uid=%d AND petid=%d",
        $uid,
        $pokemon_id
    ));
    foreach ($all_skills as $my_skill) {
        $skill_id = $my_skill['skillid'];
        $skill = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_skill') . " WHERE id=%d",
            $skill_id
        ));
        if ($skill && isset($skill['max_uses'])) {
            DB::query(pm_sql(
                "UPDATE " . pm_table('pm_myskill') . " SET skillnum=%d WHERE skillid=%d AND uid=%d AND petid=%d",
                $skill['max_uses'],
                $skill_id,
                $uid,
                $pokemon_id
            ));
        }
    }

    api_success([
        'cost' => $cost,
        'message' => "{$pokemon_data['nickname']}已脱战并治疗",
        'pokemon_id' => $pokemon_id,
        'current_hp' => (int)$petmaxhp,
        'max_hp' => (int)$petmaxhp,
        'fled' => true,
    ]);
}

/**
 * 获取在线玩家
 */
function api_get_online_players()
{
    global $_G;

    // 更新当前用户的session（如果已登录）
    if ($_G['uid']) {
        DB::query(pm_sql(
            "UPDATE " . DB::table('common_session') . " SET action='221' WHERE uid=%d",
            intval($_G['uid'])
        ));
    }

    // 获取所有在线玩家（action=221表示在宠物系统）
    // 注意：头像信息由前端根据 uid 构造 URL，无需从数据库获取
    $rows = DB::fetch_all(
        "SELECT uid, username, lastactivity 
         FROM " . DB::table('common_session') . "
         WHERE action='221' AND uid >= 1
         ORDER BY lastactivity DESC"
    );

    $players = [];
    $count = 0;

    foreach ($rows as $row) {
        $count++;

        // 返回所有玩家信息（用于滚动显示）
        $players[] = [
            'uid' => (int) $row['uid'],
            'username' => $row['username'],
            'avatar' => '', // 前端会根据 uid 构造头像 URL
            'last_activity' => (int) $row['lastactivity'],
        ];
    }

    api_success([
        'total' => $count,
        'players' => $players,
        'max_display' => $count,
    ]);
}

/**
 * 初始化新玩家
 * 自动创建用户数据并赠送第一个随机宝可梦
 */
function api_initialize_new_player()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 检查是否已有宝可梦
    $count_sql = "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = $uid";

    $existing_pokemon_count = (int) DB::result_first($count_sql);

    if ($existing_pokemon_count > 0) {
        api_error('您已有宝可梦，无需初始化', 400);
        return;
    }

    // 检查是否有 usersdata 记录
    $user_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $uid
    ));

    // 如果没有 usersdata，创建一个
    if (!$user_data) {
        DB::query(pm_sql(
            "INSERT INTO " . pm_table('pm_usersdata') . "
            (uid, money, datawin, datalost, dataall, strength)
            VALUES (%d, 0, 0, 0, 0, 1)",
            $uid
        ));
    }

    // 加载宠物数值计算工具函数
    require_once __DIR__ . '/pokemon_utils.php';

    // 随机选择一个初始宝可梦（ID 1-151，第一代）
    $random_pokemon_id = rand(1, 151);

    // 获取宝可梦数据
    $pokemon_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d",
        $random_pokemon_id
    ));

    if (!$pokemon_data) {
        // 如果获取失败，使用默认的妙蛙种子（ID=1）
        $random_pokemon_id = 1;
        $pokemon_data = DB::fetch_first(
            "SELECT * FROM " . pm_table('pm_data') . " WHERE id = 1"
        );
    }

    // 初始等级设为5
    $initial_level = 5;

    // 使用工具函数创建宠物完整数据
    $new_pokemon = create_new_pokemon_data($pokemon_data, $initial_level, $uid);

    // 生成并执行 INSERT 语句
    $sql = build_pokemon_insert_sql($new_pokemon, 1);

    DB::query($sql);

    api_success([
        'success' => true,
        'message' => '欢迎来到宝可梦世界！已为您准备初始伙伴',
        'pokemon_id' => (int) $pokemon_data['id'],
        'pokemon_name' => $pokemon_data['name'],
        'level' => $initial_level,
        'money' => 0,
        'egg_received' => false,
    ]);
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
 * 使用物品
 */
function api_use_item()
{
    require_login();

    global $_G;

    $uid = validate_uid($_G['uid']);
    $input = get_json_input();

    // 验证输入参数
    if (!isset($input['item_id']) || !$input['item_id']) {
        api_error('缺少物品ID', 400);
    }

    $item_id = validate_id($input['item_id'], 'item_id');

    $pokemon_id = null;

    if (isset($input['pokemon_id']) && $input['pokemon_id']) {
        $pokemon_id = validate_id($input['pokemon_id'], 'pokemon_id');
    }

    // 加载必要的函数

    // 获取物品数据
    $item_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE id = %d",
        $item_id
    ));

    if (!$item_data) {
        api_error('物品不存在', 404);
    }

    // 检查用户是否拥有该物品
    $my_item = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_myitem') . " WHERE uid = %d AND itemid = %s",
        $uid,
        strval($item_id)
    ));

    if (!$my_item || $my_item['nums'] <= 0) {
        api_error('您没有该物品', 400);
    }

    // 根据物品类型处理
    $item_type = $item_data['type'];
    // 旧数据 module 列为空，模块名在 sitemname/tpname 中，需回退读取
    $item_module = api_get_item_module($item_data);
    $success = false;
    $message = '';
    $pokemon_update = null;

    // PP 恢复道具（迁移数据中 type=1）需要指定技能，战斗外不支持直接使用
    if (in_array($item_module, ['pp5', 'pp10', 'pp15', 'pp99'])) {
        api_error('PP恢复道具请在战斗中对技能使用', 400);
    }

    switch ($item_type) {
        case '1': // 回复药

            if (!$pokemon_id) {
                api_error('请选择要使用的宝可梦', 400);
            }

            // 验证宝可梦所有权
            $pokemon = api_my_pokemon_data($pokemon_id);

            if (!$pokemon || $pokemon['uid'] != $uid) {
                api_error('宝可梦不存在或不属于您', 403);
            }

            $effects = json_decode(isset($item_data['effects']) ? $item_data['effects'] : '{}', true) ?: [];
            $addhp = (int) (isset($effects['hp']) ? $effects['hp'] : 0);
            $current_hp = (int) $pokemon['hp'];
            $current_state = (int) $pokemon['state'];

            // 计算最大 HP（使用统一计算函数）
            $max_hp = api_calculate_pokemon_max_hp($pokemon);

            // 如果是濒危状态（state=0），使用血瓶会转为虚弱状态
            // 如果已经是虚弱状态（state=20,21,22），使用血瓶会降级
            $new_state = $current_state;
            $state_changed = false;
            $timestamp = time();

            if ($current_state === 0) {
                // 濒危状态 -> 虚弱状态（最高等级）
                $new_state = 20;
                $state_changed = true;
            } elseif (in_array($current_state, [20, 21, 22])) {
                // 虚弱状态降级
                $new_state = downgrade_weak_state($current_state);
                $state_changed = ($new_state !== $current_state);
            }

            if ($current_hp >= $max_hp && $addhp > 0 && !$state_changed) {
                api_error("{$pokemon['nickname']}不需要增加 HP 了", 400);
            }

            $heal = $addhp;
            $new_hp = api_get_item_healed_hp($pokemon, $heal, $max_hp);

            // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
            $pokemon['hp'] = strval($new_hp);
            $hp_validation = api_validate_and_correct_hp($pokemon, $new_hp, $max_hp);
            $new_hp = $hp_validation['hp'];

            // 更新HP和状态
            if ($state_changed) {
                DB::query(pm_sql(
                    "UPDATE " . pm_table('pm_mypm') . " SET hp = %d, state = %d, statetime = %d WHERE id = %d",
                    $new_hp,
                    $new_state,
                    $timestamp,
                    $pokemon_id
                ));
            } else {
                DB::query(pm_sql(
                    "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
                    $new_hp,
                    $pokemon_id
                ));
            }

            // 物品扣除在 switch 结束后统一处理

            $success = true;

            // 构建消息
            $message_parts = [];
            $message_parts[] = "成功对 {$pokemon['nickname']} 使用了 {$item_data['name']}";
            if ($heal > 0) {
                $message_parts[] = "恢复了 {$heal} 点 HP";
            }
            if ($state_changed) {
                if ($current_state === 0) {
                    $message_parts[] = "脱离濒危状态，进入虚弱状态";
                } elseif ($new_state === 1) {
                    $message_parts[] = "虚弱状态已完全恢复";
                } else {
                    $message_parts[] = "虚弱状态有所好转";
                }
            }
            $message = implode('，', $message_parts);

            $pokemon_update = [
                'id' => $pokemon_id,
                'hp' => $new_hp,
                'max_hp' => $max_hp,
                'old_state' => $current_state,
                'new_state' => $new_state,
                'state_changed' => $state_changed,
            ];

            break;

        case '2': // 精灵球
            api_error('精灵球请在战斗中使用', 400);
            break;

        case '3': // 进化石

            if (!$pokemon_id) {
                api_error('请选择要使用的宝可梦', 400);
            }

            // 验证宝可梦所有权
            $pokemon = api_my_pokemon_data($pokemon_id);

            if (!$pokemon || $pokemon['uid'] != $uid) {
                api_error('宝可梦不存在或不属于您', 403);
            }

            // 调用物品函数（传递物品 ID 而不是名称）
            if (function_exists($item_module)) {
                $result = call_user_func($item_module, $pokemon_id, $item_data['name'], $item_id);

                if (isset($result) && $result === 1) {
                    api_error('该物品无法对当前宝可梦使用', 400);
                }

                $success = true;
                $message = "成功对 {$pokemon['nickname']} 使用了 {$item_data['name']}";
            } else {
                api_error('物品功能未实现', 500);
            }

            break;

        case '4': // 强化道具

            if (!$pokemon_id) {
                api_error('请选择要使用的宝可梦', 400);
            }

            // 验证宝可梦所有权
            $pokemon = api_my_pokemon_data($pokemon_id);

            if (!$pokemon || $pokemon['uid'] != $uid) {
                api_error('宝可梦不存在或不属于您', 403);
            }

            // 调用物品函数
            if (function_exists($item_module)) {
                // 解析物品参数
                $params = [
                    'iatk' => isset($item_data['atk']) ? $item_data['atk'] : 0,
                    'idef' => isset($item_data['def']) ? $item_data['def'] : 0,
                    'ispatk' => isset($item_data['spatk']) ? $item_data['spatk'] : 0,
                    'ispdef' => isset($item_data['spdef']) ? $item_data['spdef'] : 0,
                    'isd' => isset($item_data['speed']) ? $item_data['speed'] : 0,
                    'ihp' => isset($item_data['hp']) ? $item_data['hp'] : 0,
                ];

                $result = call_user_func_array($item_module, array_merge([$pokemon_id, $item_data['name']], array_values($params)));

                if (isset($result) && $result === 1) {
                    api_error('该物品无法对当前宝可梦使用', 400);
                }

                $success = true;
                $message = "成功对 {$pokemon['nickname']} 使用了 {$item_data['name']}";
            } else {
                api_error('物品功能未实现', 500);
            }

            break;

        case '5': // 装备道具（在装备页面使用）
            api_error('装备道具请在装备页面使用', 400);
            break;

        default:
            api_error('未知物品类型', 400);
    }

    if ($success) {
        // 扣除物品
        $new_num = $my_item['nums'] - 1;

        if ($new_num <= 0) {
            DB::query(pm_sql(
                "DELETE FROM " . pm_table('pm_myitem') . " WHERE id = %d",
                intval($my_item['id'])
            ));
            $item_remaining = 0;
        } else {
            DB::query(pm_sql(
                "UPDATE " . pm_table('pm_myitem') . " SET nums = %d WHERE id = %d",
                $new_num,
                intval($my_item['id'])
            ));
            $item_remaining = $new_num;
        }

        api_success([
            'success' => true,
            'message' => $message,
            'item_remaining' => (int) $item_remaining,
            'pokemon_updated' => $pokemon_update,
        ]);
    } else {
        api_error('物品使用失败', 500);
    }
}

/**
 * 获取可以使用指定物品的宠物列表
 */
function api_get_usable_pokemon()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    $item_id = validate_id(get_param('item_id', 0), 'item_id');

    // 获取物品数据
    $item_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE id = %d",
        $item_id
    ));

    if (!$item_data) {
        api_error('物品不存在', 404);
    }

    // 检查用户是否拥有该物品
    $my_item = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_myitem') . " WHERE uid = %d AND itemid = %s",
        $uid,
        strval($item_id)
    ));

    if (!$my_item || $my_item['nums'] <= 0) {
        api_error('您没有该物品', 400);
    }

    $item_type = $item_data['type'];

    // 获取用户所有宠物
    $all_pokemon = DB::fetch_all(pm_sql(
        "SELECT id, species_id, nickname, level, hp, hpg, atkg, defg, spatkg, spdefg, sdg, hpn, atkn, defn, spatkn, spdefn, sdn, state FROM " . pm_table('pm_mypm') . " WHERE uid = %d",
        $uid
    ));

    $usable_pokemon = [];
    $unusable_count = 0;

    // 加载状态修正系数
    require_once __DIR__ . '/utils.php';
    global $statehp;

    foreach ($all_pokemon as $pokemon) {
        $can_use = false;
        $level = (int) $pokemon['level'];
        $state = (int) $pokemon['state'];

        // 获取宠物基础信息（包括名字）
        $pm_data = DB::fetch_first(pm_sql(
            "SELECT name, hp FROM " . pm_table('pm_data') . " WHERE id = %d",
            $pokemon['species_id']
        ));

        $pokemon_name = $pm_data ? $pm_data['name'] : '???';

        // 计算最大 HP（使用统一计算函数）
        $max_hp = api_calculate_pokemon_max_hp($pokemon, $pm_data);
        $current_hp = (int) $pokemon['hp'];

        switch ($item_type) {
            case '1': // 回复药 - 只有 HP < maxhp 的宠物可以使用
                $can_use = $current_hp < $max_hp;
                break;
            case '3': // 进化石 - 所有宠物都可以尝试使用，后端会验证进化条件
                $can_use = true;
                break;
            case '4': // 强化道具 - 所有宠物都可以使用
                $can_use = true;
                break;
            case '5': // 装备道具 - 所有宠物都可以尝试装备
                $can_use = true;
                break;
            default:
                $can_use = false;
        }

        $pokemon_info = [
            'id' => (int) $pokemon['id'],
            'type_id' => (int) $pokemon['species_id'],
            'name' => $pokemon_name,
            'nickname' => $pokemon['nickname'] ?: $pokemon_name,
            'level' => $level,
            'hp' => $current_hp,
            'max_hp' => $max_hp,
            'state' => $state,
        ];

        if ($can_use) {
            $usable_pokemon[] = $pokemon_info;
        } else {
            $unusable_count++;
        }
    }

    api_success([
        'item_id' => (int) $item_id,
        'item_name' => $item_data['name'],
        'item_type' => (int) $item_type,
        'usable_pokemon' => $usable_pokemon,
        'unusable_count' => $unusable_count,
    ]);
}

/**
 * 刷新论坛帖子宠物徽章
 * 将用户当前宠物数据同步到 common_member_field_forum.pokemon 字段
 */
function api_refresh_forum_badge()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    $pm_data = _get_badge_pokemon_data($uid);

    $forum_table = DB::table('common_member_field_forum');
    $serialized = serialize($pm_data);

    $col = DB::fetch_first("SHOW COLUMNS FROM $forum_table LIKE 'pokemon'");
    if (!$col) {
        DB::query("ALTER TABLE $forum_table ADD COLUMN pokemon text NOT NULL");
    }

    DB::query(pm_sql(
        "UPDATE $forum_table SET pokemon = %s WHERE uid = %d",
        $serialized, $uid
    ));

    $first_name = isset($pm_data['first']) ? $pm_data['first']['nickname'] : '';
    $creep_count = isset($pm_data['creeps']) ? count($pm_data['creeps']) : 0;

    api_success([
        'message' => '徽章已刷新',
        'first_pokemon' => $first_name,
        'creep_count' => $creep_count,
    ]);
}

/**
 * 获取用于论坛徽章的宠物数据
 */
function _get_badge_pokemon_data($uid)
{
    $rows = DB::fetch_all(pm_sql(
        "SELECT id, species_id, nickname, level, site, is_shiny FROM " . pm_table('pm_mypm') . "
        WHERE uid = %d AND site < 3",
        $uid
    ));

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
