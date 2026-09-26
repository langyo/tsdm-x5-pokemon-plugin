<?php

/**
 * 后台测试辅助 API
 *
 * 仅限测试环境使用，生产环境应禁用
 * 所有接口需要管理员权限
 *
 * 端点:
 * - POST ?action=test_reset_user        重置用户数据
 * - POST ?action=test_create_pokemon    创建测试宠物
 * - POST ?action=test_add_item          添加测试物品
 * - POST ?action=test_set_money         设置用户金钱
 * - GET  ?action=test_get_state         获取用户完整状态
 * - POST ?action=test_set_battle_state  设置战斗状态
 */

if (defined('API_ROUTED')) {
    require_once __DIR__ . '/index.php';
} elseif (!defined('IN_DISCUZ')) {
    require_once __DIR__ . '/bootstrap.php';
    require_once __DIR__ . '/index.php';
}

require_once __DIR__ . '/constants.php';

// 加载 API 工具函数：test_get_state 等需要 api_calculate_pokemon_max_hp，
// 此前仅经路由访问时未加载 utils.php，调用即 500
require_once __DIR__ . '/utils.php';

global $_G;

$action = get_param('action', '');

switch ($action) {
    case 'test_reset_user':
        api_test_reset_user();
        break;

    case 'test_create_pokemon':
        api_test_create_pokemon();
        break;

    case 'test_add_item':
        api_test_add_item();
        break;

    case 'test_set_money':
        api_test_set_money();
        break;

    case 'test_get_state':
        api_test_get_state();
        break;

    case 'test_set_battle_state':
        api_test_set_battle_state();
        break;

    case 'test_create_test_user':
        api_test_create_test_user();
        break;

    case 'test_cleanup':
        api_test_cleanup();
        break;

    default:
        api_error('Invalid admin action', 400);
}

function api_require_admin()
{
    global $_G;

    require_login();

    $uid = (int) $_G['uid'];

    $admin_group = (int) DB::result_first(pm_sql(
        "SELECT groupid FROM " . DB::table('common_member') . " WHERE uid = %d",
        $uid
    ));

    // 测试/管理接口可直接改写资金与玩家数据，仅允许管理员用户组（groupid=1）
    // 与插件配置中指名的宠物管理员（poke_smgly）；
    // 版主组（2=超级版主、3=版主）不再放行，避免权限放大。
    if ($admin_group == 1) {
        return $uid;
    }

    $settings = isset($_G['cache']['plugin']['pokemon']) ? $_G['cache']['plugin']['pokemon'] : array();
    $admins = isset($settings['poke_smgly']) ? explode(',', $settings['poke_smgly']) : array();
    $username = $_G['member']['username'];

    if (in_array($username, $admins)) {
        return $uid;
    }

    api_error('Admin permission required', 403);
}

function api_test_reset_user()
{
    $uid = api_require_admin();

    $input = get_json_input();
    $target_uid = isset($input['uid']) ? validate_id($input['uid'], 'uid') : $uid;
    $reset_all = isset($input['reset_all']) ? validate_bool($input['reset_all'], 'reset_all') : true;

    DB::query(pm_sql("DELETE FROM " . pm_table('pm_mypm') . " WHERE uid = %d", $target_uid));
    DB::query(pm_sql("DELETE FROM " . pm_table('pm_myitem') . " WHERE uid = %d", $target_uid));
    DB::query(pm_sql("DELETE FROM " . pm_table('pm_myskill') . " WHERE uid = %d", $target_uid));

    if ($reset_all) {
        DB::query(pm_sql("DELETE FROM " . pm_table('pm_usersdata') . " WHERE uid = %d", $target_uid));
    } else {
        DB::query(pm_sql("UPDATE " . pm_table('pm_usersdata') . " SET
            money = 0,
            npcid = 0,
            level = 0,
            hp = 0,
            hpg = 0,
            atkg = 0,
            spatkg = 0,
            defg = 0,
            spdefg = 0,
            sdg = 0,
            dataall = 0,
            datawin = 0,
            datalost = 0,
            fullexp = 0
            WHERE uid = %d", $target_uid));
    }

    $pokemon_count = (int) DB::result_first(pm_sql(
        "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = %d",
        $target_uid
    ));
    $item_count = (int) DB::result_first(pm_sql(
        "SELECT COUNT(*) FROM " . pm_table('pm_myitem') . " WHERE uid = %d",
        $target_uid
    ));

    api_success([
        'message' => 'User data reset successfully',
        'user' => [
            'uid' => $target_uid,
            'money' => 0,
            'total_pokemons' => $pokemon_count,
            'total_items' => $item_count
        ]
    ]);
}

function api_test_create_pokemon()
{
    $uid = api_require_admin();

    $input = get_json_input();

    $pmno = isset($input['species_id']) ? validate_id($input['species_id'], 'pmno') : null;
    if (!$pmno) {
        $pmno = rand(1, 151);
    }

    $level = isset($input['level']) ? validate_int_range($input['level'], 'level', 1, 100) : 5;
    $as_lead = isset($input['is_zd']) ? validate_int_range($input['is_zd'], 'is_zd', 0, 1) : 0;
    $hp_percent = isset($input['hp_percent']) ? validate_int_range($input['hp_percent'], 'hp_percent', 1, 100) : 100;
    $skills = isset($input['skills']) ? $input['skills'] : null;

    $pokemon_data = DB::fetch_first(
        "SELECT * FROM " . pm_table('pm_data') . " WHERE id = $pmno"
    );

    if (!$pokemon_data) {
        api_error("Pokemon not found: pmno=$pmno", 404);
    }

    $base_hp = (int) $pokemon_data['hp'];
    $base_atk = (int) $pokemon_data['atk'];
    $base_def = (int) $pokemon_data['def'];
    $base_spatk = (int) $pokemon_data['spatk'];
    $base_spdef = (int) $pokemon_data['spdef'];
    $base_sd = (int) $pokemon_data['speed'];

    $hp = (int) (($base_hp * 2 * $level / 100) + $level + 10);
    $max_hp = $hp;
    $current_hp = (int) ($hp * $hp_percent / 100);

    $atk = (int) (($base_atk * 2 * $level / 100) + 5);
    $def = (int) (($base_def * 2 * $level / 100) + 5);
    $spatk = (int) (($base_spatk * 2 * $level / 100) + 5);
    $spdef = (int) (($base_spdef * 2 * $level / 100) + 5);
    $sd = (int) (($base_sd * 2 * $level / 100) + 5);

    $exp_for_level = calculate_exp_for_level($level);
    $exp_to_next = calculate_exp_for_level($level + 1) - $exp_for_level;

    $sex = rand(0, 1);
    $is_shiny = (rand(1, 8192) === 1) ? 1 : 0;

    $site = 0;
    if ($as_lead) {
        DB::query(pm_sql("UPDATE " . pm_table('pm_mypm') . " SET site = 0 WHERE uid = %d AND site = 1", $uid));
        $site = 1;
    }

    $pmname = $pokemon_data['name'];
    $xs = $pokemon_data['xs'];
    $current_time = time();

    DB::query(pm_sql("INSERT INTO " . pm_table('pm_mypm') . "
        (uid, species_id, pmname, nickname, level, exp, sex, sx, hp, hpg, atkg, defg, spatkg, spdefg, sdg, hpn, atkn, defn, spatkn, spdefn, sdn, site, created_at, good, ballid, state, statetime, gduptime, initialuid, swap, is_shiny)
        VALUES
        (%d, %d, %s, %s, %d, %d, %d,
         %s, %d, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, %d,
         %d, 0, 0, 1, 0, %d, %d, 1, %d)",
        $uid, $pmno, $pmname, $pmname, $level, $exp_for_level, $sex,
         $xs, $current_hp, $site,
         $current_time, $current_time, $uid, $is_shiny));

    $pet_id = DB::insert_id();

    if ($skills && is_array($skills)) {
        foreach ($skills as $skill_id) {
            $skill_id = (int) $skill_id;
            $skill_data = DB::fetch_first(pm_sql(
                "SELECT * FROM " . pm_table('pm_skill') . " WHERE id = %d",
                $skill_id
            ));
            if ($skill_data) {
                $max_pp = (int) $skill_data['max_uses'];
                DB::query(pm_sql("INSERT INTO " . pm_table('pm_myskill') . "
                    (uid, petid, skillid, skillnum)
                    VALUES (%d, %d, %d, %d)",
                    $uid, $pet_id, $skill_id, $max_pp));
            }
        }
    } else {
        $auto_skills = DB::fetch_all(pm_sql("SELECT id, max_uses FROM " . pm_table('pm_skill') . "
            WHERE FIND_IN_SET(%d, REPLACE(available_pokemons, '|', ',')) > 0
            AND level_required <= %d
            ORDER BY level_required DESC LIMIT 4",
            $pmno, $level));

        foreach ($auto_skills as $skill) {
            $skill_id = (int) $skill['id'];
            $max_pp = (int) $skill['max_uses'];
            DB::query(pm_sql("INSERT INTO " . pm_table('pm_myskill') . "
                (uid, petid, skillid, skillnum)
                VALUES (%d, %d, %d, %d)",
                $uid, $pet_id, $skill_id, $max_pp));
        }
    }

    $skills_response = [];
    $skill_rows = DB::fetch_all(pm_sql("SELECT s.id, s.name, s.max_uses as max_pp, ms.skillnum as pp, s.type, s.category, s.power as power
        FROM " . pm_table('pm_myskill') . " ms
        JOIN " . pm_table('pm_skill') . " s ON ms.skillid = s.id
        WHERE ms.petid = %d", $pet_id));

    foreach ($skill_rows as $skill) {
        $skills_response[] = [
            'id' => (int) $skill['id'],
            'name' => $skill['name'],
            'pp' => (int) $skill['pp'],
            'max_pp' => (int) $skill['max_pp'],
            'type' => $skill['type'],
            'category' => $skill['category'],
            'power' => (int) $skill['power']
        ];
    }

    api_success([
        'id' => (int) $pet_id,
        'pmno' => $pmno,
        'name' => $pokemon_data['name'],
        'level' => $level,
        'hp' => $current_hp,
        'max_hp' => $max_hp,
        'is_zd' => $site,
        'is_shiny' => $is_shiny,
        'exp' => $exp_for_level,
        'exp_to_next_level' => $exp_to_next,
        'skills' => $skills_response
    ]);
}

function api_test_add_item()
{
    $uid = api_require_admin();

    $input = get_json_input();

    $item_id = validate_required_param($input, 'item_id', 'id');
    $quantity = isset($input['quantity']) ? validate_int_range($input['quantity'], 'quantity', 1, 999) : 1;

    $item_data = DB::fetch_first(
        "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE id = $item_id"
    );

    if (!$item_data) {
        api_error("Item not found: item_id=$item_id", 404);
    }

    $existing = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_myitem') . " WHERE uid = %d AND itemid = %s",
        $uid,
        strval($item_id)
    ));

    if ($existing) {
        $new_quantity = (int) $existing['nums'] + $quantity;
        DB::query(pm_sql("UPDATE " . pm_table('pm_myitem') . "
            SET nums = %d
            WHERE id = %d",
            $new_quantity, (int) $existing['id']));
        $inventory_id = (int) $existing['id'];
        $final_quantity = $new_quantity;
    } else {
        DB::query(pm_sql("INSERT INTO " . pm_table('pm_myitem') . "
            (uid, itemid, nums)
            VALUES (%d, %s, %d)",
            $uid, strval($item_id), $quantity));
        $inventory_id = DB::insert_id();
        $final_quantity = $quantity;
    }

    api_success([
        'inventory_id' => $inventory_id,
        'item_id' => $item_id,
        'name' => $item_data['name'],
        'quantity' => $final_quantity,
        'added' => $quantity,
        'message' => "Added {$quantity}x {$item_data['name']} to inventory"
    ]);
}

function api_test_set_money()
{
    $uid = api_require_admin();

    $input = get_json_input();

    $amount = validate_required_param($input, 'amount', 'int', ['min' => 0, 'max' => 999999999]);

    $user_data = DB::fetch_first(pm_sql(
        "SELECT money FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $uid
    ));

    $old_money = $user_data ? (int) $user_data['money'] : 0;

    if (!$user_data) {
        DB::query(pm_sql("INSERT INTO " . pm_table('pm_usersdata') . "
            (uid, money, datawin, datalost, dataall, strength)
            VALUES (%d, %d, 0, 0, 0, 1)",
            $uid, $amount));
    } else {
        DB::query(pm_sql("UPDATE " . pm_table('pm_usersdata') . "
            SET money = %d
            WHERE uid = %d",
            $amount, $uid));
    }

    api_success([
        'old_money' => $old_money,
        'new_money' => $amount
    ]);
}

function api_test_get_state()
{
    $uid = api_require_admin();

    $input = get_json_input();
    $target_uid = isset($input['uid']) ? validate_id($input['uid'], 'uid') : $uid;

    $user_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $target_uid
    ));

    $member = DB::fetch_first(pm_sql(
        "SELECT uid, username, groupid FROM " . DB::table('common_member') . " WHERE uid = %d",
        $target_uid
    ));

    $pokemons_response = [];
    $pokemon_rows = DB::fetch_all(pm_sql("SELECT m.*, d.name as pmname, d.xs as type1, d.xs2 as type2, d.capture
        FROM " . pm_table('pm_mypm') . " m
        LEFT JOIN " . pm_table('pm_data') . " d ON m.species_id = d.id
        WHERE m.uid = %d
        ORDER BY m.site DESC, m.id ASC",
        $target_uid));

    foreach ($pokemon_rows as $p) {
        $skills_response = [];
        $skill_rows = DB::fetch_all(pm_sql("SELECT s.id, s.name, ms.skillnum as pp, s.max_uses as max_pp
            FROM " . pm_table('pm_myskill') . " ms
            JOIN " . pm_table('pm_skill') . " s ON ms.skillid = s.id
            WHERE ms.petid = %d",
            (int) $p['id']));

        foreach ($skill_rows as $s) {
            $skills_response[] = [
                'id' => (int) $s['id'],
                'name' => $s['name'],
                'pp' => (int) $s['pp'],
                'max_pp' => (int) $s['max_pp']
            ];
        }

        $pokemons_response[] = [
            'id' => (int) $p['id'],
            'pmno' => (int) $p['species_id'],
            'name' => $p['pmname'] ?: $p['nickname'],
            'nickname' => $p['nickname'],
            'level' => (int) $p['level'],
            'exp' => (int) $p['exp'],
            'hp' => (int) $p['hp'],
            'max_hp' => api_calculate_pokemon_max_hp($p),
            'is_zd' => (int)($p['site'] == 1),
            'is_shiny' => (int) $p['is_shiny'],
            'type1' => $p['type1'],
            'type2' => $p['type2'],
            'skills' => $skills_response
        ];
    }

    $items_response = [];
    $item_rows = DB::fetch_all(pm_sql("SELECT m.id, m.itemid, m.nums, d.name, d.type as item_type, d.id as itemdata_id
        FROM " . pm_table('pm_myitem') . " m
        LEFT JOIN " . pm_table('pm_itemdata') . " d ON m.itemid = d.id
        WHERE m.uid = %d
        ORDER BY m.id ASC",
        $target_uid));

    foreach ($item_rows as $i) {
        $items_response[] = [
            'id' => (int) $i['id'],
            'type_id' => (int) $i['itemdata_id'],
            'name' => $i['name'] ?: '未知物品',
            'quantity' => (int) $i['nums'],
            'item_type' => (int) $i['item_type']
        ];
    }

    $battle = null;
    if ($user_data && (int) $user_data['npcid'] > 0) {
        $wild_pmno = (int) $user_data['npcid'];
        $wild_level = (int) $user_data['level'];
        $wild_hp = (int) $user_data['hp'];
        $wild_max_hp = (int) $user_data['hpg'];

        $wild_data = DB::fetch_first(pm_sql(
            "SELECT name FROM " . pm_table('pm_data') . " WHERE id = %d",
            $wild_pmno
        ));

        $battle = [
            'wild_pokemon' => [
                'pmno' => $wild_pmno,
                'name' => $wild_data ? $wild_data['name'] : 'Unknown',
                'level' => $wild_level,
                'hp' => $wild_hp,
                'max_hp' => $wild_max_hp
            ]
        ];
    }

    api_success([
        'user' => [
            'uid' => (int) $target_uid,
            'username' => $member ? $member['username'] : 'Unknown',
            'group_id' => $member ? (int) $member['groupid'] : 0,
            'money' => $user_data ? (int) $user_data['money'] : 0,
            'wins' => $user_data ? (int) $user_data['datawin'] : 0,
            'losses' => $user_data ? (int) $user_data['datalost'] : 0,
            'total_battles' => $user_data ? (int) $user_data['dataall'] : 0,
            'total_pokemons' => count($pokemons_response),
            'total_items' => count($items_response)
        ],
        'pokemons' => $pokemons_response,
        'items' => $items_response,
        'battle' => $battle
    ]);
}

function api_test_set_battle_state()
{
    $uid = api_require_admin();

    $input = get_json_input();

    $map_id = validate_required_param($input, 'map_id', 'id');
    $wild_pmno = isset($input['wild_pmno']) ? validate_id($input['wild_pmno'], 'wild_pmno') : null;
    $wild_level = isset($input['wild_level']) ? validate_int_range($input['wild_level'], 'wild_level', 1, 100) : 5;
    $wild_hp_percent = isset($input['wild_hp_percent']) ? validate_int_range($input['wild_hp_percent'], 'wild_hp_percent', 1, 100) : 100;

    $map_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_map') . " WHERE id = %d",
        $map_id
    ));

    if (!$map_data) {
        api_error("Map not found: map_id=$map_id", 404);
    }

    if (!$wild_pmno) {
        $site = $map_data['site'];
        $possible_pmno = array_filter(explode(',', $site));
        if (empty($possible_pmno)) {
            $wild_pmno = rand(1, 151);
        } else {
            $wild_pmno = (int) $possible_pmno[array_rand($possible_pmno)];
        }
    }

    $wild_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d",
        $wild_pmno
    ));

    if (!$wild_data) {
        api_error("Pokemon not found: pmno=$wild_pmno", 404);
    }

    $base_hp = (int) $wild_data['hp'];
    $max_hp = (int) (($base_hp * 2 * $wild_level / 100) + $wild_level + 10);
    $current_hp = (int) ($max_hp * $wild_hp_percent / 100);

    $user_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_usersdata') . " WHERE uid = %d",
        $uid
    ));

    if (!$user_data) {
        DB::query(pm_sql("INSERT INTO " . pm_table('pm_usersdata') . "
            (uid, money, datawin, datalost, dataall, strength)
            VALUES (%d, 0, 0, 0, 0, 1)",
            $uid));
    }

    $base_atk = (int) $wild_data['atk'];
    $base_def = (int) $wild_data['def'];
    $base_spatk = (int) $wild_data['spatk'];
    $base_spdef = (int) $wild_data['spdef'];
    $base_sd = (int) $wild_data['speed'];

    $atk = (int) (($base_atk * 2 * $wild_level / 100) + 5);
    $def = (int) (($base_def * 2 * $wild_level / 100) + 5);
    $spatk = (int) (($base_spatk * 2 * $wild_level / 100) + 5);
    $spdef = (int) (($base_spdef * 2 * $wild_level / 100) + 5);
    $sd = (int) (($base_sd * 2 * $wild_level / 100) + 5);

    DB::query(pm_sql("UPDATE " . pm_table('pm_usersdata') . " SET
        npcid = %d,
        level = %d,
        hp = %d,
        hpg = %d,
        atkg = %d,
        spatkg = %d,
        defg = %d,
        spdefg = %d,
        sdg = %d
        WHERE uid = %d",
        $wild_pmno, $wild_level, $current_hp, $max_hp, $atk, $spatk, $def, $spdef, $sd, $uid));

    $battle_id = 'battle_' . microtime(true);

    api_success([
        'battle_id' => $battle_id,
        'map' => [
            'id' => $map_id,
            'name' => $map_data['name']
        ],
        'wild_pokemon' => [
            'pmno' => $wild_pmno,
            'name' => $wild_data['name'],
            'level' => $wild_level,
            'hp' => $current_hp,
            'max_hp' => $max_hp,
            'hp_percent' => $wild_hp_percent
        ]
    ]);
}

function api_test_create_test_user()
{
    $uid = api_require_admin();

    $input = get_json_input();

    $username = isset($input['username']) ? validate_string($input['username'], 'username', 50) : 'test_user_' . time();
    $password = isset($input['password']) ? validate_string($input['password'], 'password', 50) : 'test123456';

    $existing = DB::fetch_first(pm_sql(
        "SELECT uid FROM " . DB::table('common_member') . " WHERE username = %s",
        $username
    ));

    if ($existing) {
        api_success([
            'message' => 'Test user already exists',
            'uid' => (int) $existing['uid'],
            'username' => $username,
            'password' => $password
        ]);
    }

    $salt = substr(uniqid(rand()), -6);
    $password_hash = md5(md5($password) . $salt);
    $current_time = time();
    $email = $username . '@test.local';

    DB::query(pm_sql("INSERT INTO " . DB::table('common_member') . "
        (username, password, salt, groupid, regdate, email)
        VALUES (%s, %s, %s, 10, %d, %s)",
        $username, $password_hash, $salt, $current_time, $email));

    $new_uid = DB::insert_id();

    DB::query(pm_sql("INSERT INTO " . DB::table('common_member_status') . "
        (uid, regip, lastip, lastvisit, lastactivity)
        VALUES (%d, '127.0.0.1', '127.0.0.1', %d, %d)",
        $new_uid, $current_time, $current_time));

    api_success([
        'message' => 'Test user created',
        'uid' => (int) $new_uid,
        'username' => $username,
        'password' => $password
    ]);
}

function api_test_cleanup()
{
    $uid = api_require_admin();

    $input = get_json_input();

    $cleanup_users = isset($input['cleanup_users']) ? validate_bool($input['cleanup_users'], 'cleanup_users') : false;

    if ($cleanup_users) {
        $deleted_count = 0;
        $users = DB::fetch_all("SELECT uid FROM " . DB::table('common_member') . " WHERE username LIKE 'test_user_%'");

        foreach ($users as $user) {
            $test_uid = (int) $user['uid'];
            DB::query(pm_sql("DELETE FROM " . DB::table('common_member') . " WHERE uid = %d", $test_uid));
            DB::query(pm_sql("DELETE FROM " . DB::table('common_member_status') . " WHERE uid = %d", $test_uid));
            DB::query(pm_sql("DELETE FROM " . pm_table('pm_mypm') . " WHERE uid = %d", $test_uid));
            DB::query(pm_sql("DELETE FROM " . pm_table('pm_myitem') . " WHERE uid = %d", $test_uid));
            DB::query(pm_sql("DELETE FROM " . pm_table('pm_myskill') . " WHERE uid = %d", $test_uid));
            DB::query(pm_sql("DELETE FROM " . pm_table('pm_usersdata') . " WHERE uid = %d", $test_uid));
            $deleted_count++;
        }

        api_success([
            'message' => 'Cleanup completed',
            'deleted_users' => $deleted_count
        ]);
    }

    api_success([
        'message' => 'No cleanup action specified'
    ]);
}

function calculate_exp_for_level($level)
{
    if ($level <= 1) {
        return 0;
    }

    return (int) (4 * pow($level, 3) / 5);
}
