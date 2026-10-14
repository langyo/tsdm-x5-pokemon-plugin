<?php

/**
 * 商店系统API
 *
 * 端点:
 * - GET  ?action=items&type=xxx&page=1&keyword=xx  获取商品列表（keyword 为名称字面子串筛选）
 * - POST ?action=buy                                购买商品
 * - GET  ?action=categories                         获取商品分类
 * - GET  ?action=pets&page=1&keyword=xx             获取可购买宠物列表
 * - POST ?action=buy_pet                            购买宠物
 */

// 加载Discuz环境
if (!defined('IN_DISCUZ')) {
    require_once __DIR__ . '/bootstrap.php';
}

// 加载API辅助函数
require_once __DIR__ . '/index.php';
require_once __DIR__ . '/utils.php';

// 加载常量定义
require_once __DIR__ . '/constants.php';

global $_G;

// IDE 类型提示
if (false) {
    function validate_required_param($input, $key, $type = 'string', $options = []) {}
    function validate_optional_param($input, $key, $default = null, $type = 'string', $options = []) {}
    function validate_id($id, $name = 'ID') {}
}

$action = get_param('action', '');

switch ($action) {
    case 'list':
    case 'items':
        api_get_shop_items();
        break;

    case 'buy':
        api_buy_item();
        break;

    case 'categories':
        api_get_categories();
        break;

    case 'pets':
        api_get_shop_pets();
        break;

    case 'buy_pet':
        api_buy_pet();
        break;

    default:
        api_error('Invalid action', 400);
}

/**
 * 获取商店商品列表
 * pm_itemdata 表: shop=1 表示在商店出售
 * 列: id, name, tpname, shop, money, txt, type, lvask, xsask,
 *     addhp, addexp, addlv, addgood, ballid, upitem, captmax, captmin,
 *     sitemname, hot, zbtype, equipment_hp/atk/def/spatk/spdef/sd
 */
function api_get_shop_items()
{
    require_login();

    // 验证输入参数
    $input = [
        'type' => get_param('type', null),
        'page' => get_param('page', 1),
    ];

    $item_type = validate_optional_param($input, 'type', null, 'int', ['min' => 1, 'max' => 5]);
    $page = validate_optional_param($input, 'page', 1, 'int', ['min' => 1, 'max' => 9999]);
    $keyword = pm_get_search_keyword();

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 构建查询条件 - 使用 pm_itemdata 表，shop=1 表示在售
    $where_params = ["shop = 1"];
    $where_values = [];
    if ($item_type !== null) {
        $where_params[] = "type = %d";
        $where_values[] = $item_type;
    }
    if ($keyword !== '') {
        // 与背包搜索一致：LOCATE 字面子串匹配，% 与 _ 不当通配符。
        $where_params[] = "LOCATE(%s, name) > 0";
        $where_values[] = $keyword;
    }

    $where_clause = implode(' AND ', $where_params);

    // 获取总数
    $count_sql = "SELECT COUNT(*) FROM " . pm_table('pm_itemdata') . " WHERE $where_clause";
    $total_result = DB::query(pm_sql_v($count_sql, $where_values));
    $total = (int) DB::result($total_result, 0);

    // 分页
    $per_page = SHOP_ITEMS_PER_PAGE;
    $offset = ($page - 1) * $per_page;

    // 查询商品
    $sql = "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE $where_clause ORDER BY money ASC LIMIT %d, %d";
    $rows = DB::fetch_all(pm_sql_v($sql, array_merge($where_values, array($offset, $per_page))));

    // 获取用户金钱
    $user = DB::fetch_first(pm_sql(
        "SELECT money FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $uid
    ));
    $user_money = $user ? (int) $user['money'] : 0;

    $items = array();
    foreach ($rows as $row) {
        $price = (int) $row['money'];
        $effects = json_decode($row['effects'], true);
        $items[] = array(
            'id' => (int) $row['id'],
            'name' => $row['name'],
            'description' => $row['description'],
            'type_id' => (int) $row['type'],
            'type_name' => get_item_type_name($row['type']),
            'image' => $row['tpname'],
            'price' => $price,
            'stock' => -1,
            'effect' => array(
                'description' => $row['description'],
                'type' => get_item_type_name($row['type']),
                'addhp' => (int) ($effects['hp'] ?? 0),
                'addexp' => (int) ($effects['exp'] ?? 0),
                'addlv' => (int) ($effects['level'] ?? 0),
            ),
            'can_buy' => $user_money >= $price,
        );
    }

    api_success(array(
        'items' => $items,
        'total' => $total,
        'page' => $page,
        'per_page' => $per_page,
        'total_pages' => (int) ceil($total / max($per_page, 1)),
    ));
}

/**
 * 购买商品
 */
function api_buy_item()
{
    require_login();

    $input = get_json_input();

    // 严格验证输入参数
    $item_id = validate_required_param($input, 'item_id', 'id');
    $quantity = validate_optional_param($input, 'quantity', 1, 'int', ['min' => 1, 'max' => 99]);

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 获取商品信息 (from pm_itemdata, shop=1)
    $item = DB::fetch_first(
        "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE id = $item_id AND shop = 1"
    );

    if (!$item) {
        api_error('Item not found or unavailable', 404);
    }

    // 验证商品数据完整性
    if (!isset($item['money'])) {
        api_error('Invalid item data: missing price', 500);
    }

    // 计算总价格
    $price = validate_int_range($item['money'], 'item.money', 0, 999999999);
    $total_price = $price * $quantity;

    // 与宠物购买、队伍调整共用用户行锁，余额检查和发货一起提交。
    DB::query("START TRANSACTION");
    try {
        $user = DB::fetch_first(pm_sql(
            "SELECT money FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $uid
        ));

        if (!$user) {
            pm_abort_battle_transaction('User not found', 404);
        }
        if (!isset($user['money'])) {
            pm_abort_battle_transaction('Invalid user data: missing money field', 500);
        }
        if (!is_numeric($user['money']) || (int) $user['money'] < 0 || (int) $user['money'] > 999999999) {
            pm_abort_battle_transaction('Invalid user.money', 400);
        }
        $user_money = (int) $user['money'];
        if ($user_money < $total_price) {
            pm_abort_battle_transaction('Insufficient funds', 400);
        }

        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_usersdata') . " SET money = money - %d WHERE uid = %d",
            $total_price,
            $uid
        ));

        add_item_to_inventory($uid, $item_id, $quantity);
        DB::query("COMMIT");
    } catch (Throwable $e) {
        DB::query("ROLLBACK");
        throw $e;
    }

    api_success(array(
        'message' => 'Purchase successful',
        'total_cost' => $total_price,
        'remaining_money' => $user_money - $total_price,
        'items_purchased' => $quantity,
    ));
}

/**
 * 获取商品分类
 */
function api_get_categories()
{
    $categories = array(
        array('id' => 1, 'name' => '回复药', 'description' => '恢复HP和PP的道具'),
        array('id' => 2, 'name' => '精灵球', 'description' => '捕捉野生宝可梦的道具'),
        array('id' => 3, 'name' => '进化石', 'description' => '使宝可梦进化的石头'),
        array('id' => 4, 'name' => '强化道具', 'description' => '提升能力值的道具'),
        array('id' => 5, 'name' => '装备道具', 'description' => '可装备在宝可梦身上的道具'),
    );

    api_success(array('categories' => $categories));
}

/**
 * 判断用户是否可以购买商品
 */
function can_buy_item($uid, $price)
{
    $user = DB::fetch_first(
        "SELECT money FROM " . pm_table('pm_usersdata') . " WHERE uid = $uid"
    );

    if (!$user) {
        return false;
    }

    return (int) $user['money'] >= $price;
}

/**
 * 添加物品到用户背包
 */
function add_item_to_inventory($uid, $item_type_id, $quantity = 1)
{
    // 调用方已持有用户行锁；再锁住原有物品，按整笔购买检查有符号 SMALLINT 容量。
    $existing = DB::fetch_first(
        "SELECT * FROM " . pm_table('pm_myitem') . "
        WHERE uid = $uid AND itemid = '$item_type_id' FOR UPDATE"
    );

    if ($existing) {
        $stock = (int) $existing['nums'];
        if ($stock < 0 || $stock > 32767 - $quantity) {
            pm_abort_battle_transaction('Inventory quantity limit exceeded', 400);
        }
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_myitem') . " SET nums = nums + %d WHERE id = %d",
            $quantity,
            intval($existing['id'])
        ));
    } else {
        // 添加新物品
        DB::query(pm_sql(
            "INSERT INTO " . pm_table('pm_myitem') . " (uid, itemid, nums) VALUES (%d, %d, %d)",
            $uid,
            $item_type_id,
            $quantity
        ));
    }
}

/**
 * 获取物品类型名称
 */
function get_item_type_name($type_id)
{
    static $types = array(
        1 => '回复药',
        2 => '精灵球',
        3 => '进化石',
        4 => '强化道具',
        5 => '装备道具',
    );

    return isset($types[$type_id]) ? $types[$type_id] : '未知类型';
}

/**
 * 获取商店可购买的宠物列表
 * pm_data 表: shop=1 表示在商店出售
 */
function api_get_shop_pets()
{
    require_login();

    $page = (int) get_param('page', 1);
    $per_page = SHOP_ITEMS_PER_PAGE;
    $keyword = pm_get_search_keyword();

    global $_G;
    $uid = validate_uid($_G['uid']);

    $where_params = ["shop = 1"];
    $where_values = [];
    if ($keyword !== '') {
        // 与商品列表一致：LOCATE 字面子串匹配。
        $where_params[] = "LOCATE(%s, name) > 0";
        $where_values[] = $keyword;
    }
    $where_clause = implode(' AND ', $where_params);

    $count_result = DB::query(
        pm_sql_v(
            "SELECT COUNT(*) FROM " . pm_table('pm_data') . " WHERE $where_clause",
            $where_values
        )
    );
    $total = (int) DB::result($count_result, 0);

    $offset = ($page - 1) * $per_page;

    $rows = DB::fetch_all(pm_sql_v(
        "SELECT * FROM " . pm_table('pm_data') . " WHERE $where_clause ORDER BY id ASC LIMIT %d, %d",
        array_merge($where_values, array($offset, $per_page))
    ));

    $user = DB::fetch_first(pm_sql(
        "SELECT money FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $uid
    ));
    $user_money = $user ? (int) $user['money'] : 0;

    $pets = array();
    foreach ($rows as $row) {
        $price = (int) $row['money'];
        $pets[] = array(
            'id' => (int) $row['id'],
            'name' => $row['name'],
            'type_1' => $row['xs'],
            'type_2' => $row['xs2'] ? $row['xs2'] : null,
            'hp' => (int) $row['hp'],
            'atk' => (int) $row['atk'],
            'def' => (int) $row['def'],
            'spatk' => (int) $row['spatk'],
            'spdef' => (int) $row['spdef'],
            'speed' => (int) $row['speed'],
            'price' => $price,
            'can_buy' => $user_money >= $price,
        );
    }

    api_success(array(
        'pets' => $pets,
        'total' => $total,
        'page' => $page,
        'per_page' => $per_page,
        'total_pages' => $total > 0 ? (int) ceil($total / max($per_page, 1)) : 0,
    ));
}

/**
 * 购买宠物
 */
function api_buy_pet()
{
    require_login();

    $input = get_json_input();

    if (!isset($input['pokemon_type_id']) || !$input['pokemon_type_id']) {
        api_error('Missing parameter: pokemon_type_id', 400);
    }

    $pokemon_type_id = validate_id($input['pokemon_type_id'], 'pokemon_type_id');

    global $_G;
    $uid = validate_uid($_G['uid']);

    $pokemon_data = DB::fetch_first(
        "SELECT * FROM " . pm_table('pm_data') . " WHERE id = $pokemon_type_id AND shop = 1"
    );

    if (!$pokemon_data) {
        api_error('Pet not found or unavailable for purchase', 404);
    }

    $price = (int) $pokemon_data['money'];

    require_once __DIR__ . '/pokemon_utils.php';
    $initial_level = 5;

    DB::query("START TRANSACTION");
    try {
        $user = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $uid
        ));

        if (!$user) {
            pm_abort_battle_transaction('User data not found', 404);
        }
        $user_money = (int) $user['money'];
        if ($user_money < $price) {
            pm_abort_battle_transaction('Insufficient funds', 400);
        }

        // 容量与首位检查必须在锁内，避免同时购买占用同一个空位。
        $pokemon_count = DB::result_first(pm_sql(
            "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = %d",
            $uid
        ));
        if ($pokemon_count >= (int) $user['boxnum']) {
            pm_abort_battle_transaction('箱子容量不足，请扩展！', 400);
        }

        $has_first = DB::result_first(pm_sql(
            "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = %d AND site = 1",
            $uid
        ));
        if ($has_first == 0) {
            $site = 1;
        } else {
            $bag_count = DB::result_first(pm_sql(
                "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = %d AND site < 3",
                $uid
            ));
            $site = $bag_count >= 6 ? 3 : 2;
        }

        $new_pokemon = create_new_pokemon_data($pokemon_data, $initial_level, $uid);
        DB::query(build_pokemon_insert_sql($new_pokemon, $site));
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_usersdata') . " SET money = money - %d WHERE uid = %d",
            $price,
            $uid
        ));
        DB::query("COMMIT");
    } catch (Throwable $e) {
        DB::query("ROLLBACK");
        throw $e;
    }

    api_success(array(
        'message' => 'Purchase successful',
        'total_cost' => $price,
        'remaining_money' => $user_money - $price,
        'pokemon_name' => $pokemon_data['name'],
        'pokemon_type_id' => (int) $pokemon_type_id,
        'site' => $site,
    ));
}
