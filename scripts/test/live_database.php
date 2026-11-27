<?php
/** Real MariaDB + independent PHP CGI requests. No running Discuz site needed. */
mysqli_report(MYSQLI_REPORT_ERROR | MYSQLI_REPORT_STRICT);
$db = new mysqli(getenv('TSDM_DB_HOST') ?: '127.0.0.1', getenv('TSDM_DB_USER') ?: 'root',
    getenv('TSDM_DB_PASSWORD') ?: '', '', (int)(getenv('TSDM_DB_PORT') ?: 3306));
$database = 'tsdm_test_' . bin2hex(random_bytes(8));
$db->query('CREATE DATABASE `' . $database . '` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci');
$db->select_db($database);
putenv('TSDM_TEST_DB=' . $database);
$passed = 0;
$contracts = [];
$workers = [];

function check($condition, $message)
{
    if (!$condition) throw new RuntimeException($message);
    $GLOBALS['passed']++;
    echo 'PASS ', $message, PHP_EOL;
}
function scalar($sql) { return $GLOBALS['db']->query($sql)->fetch_row()[0]; }
function begin_request($endpoint, $action, $input = null, $extra = [])
{
    $root = realpath(__DIR__ . '/support/live_api.php');
    $base = tempnam(sys_get_temp_dir(), 'tsdm-api-');
    $body = $input === null ? '' : (is_string($input) ? $input : json_encode($input, JSON_THROW_ON_ERROR));
    $env = array_merge(getenv(), [
        'REDIRECT_STATUS' => '200', 'GATEWAY_INTERFACE' => 'CGI/1.1',
        'REQUEST_METHOD' => $input === null ? 'GET' : 'POST',
        'SCRIPT_FILENAME' => $root, 'SCRIPT_NAME' => '/live_api.php',
        'QUERY_STRING' => 'action=' . rawurlencode($action),
        'CONTENT_TYPE' => 'application/json', 'CONTENT_LENGTH' => (string)strlen($body),
        'HTTP_X_PM_FORMHASH' => 'test-formhash', 'TSDM_TEST_ENDPOINT' => $endpoint,
        'TSDM_TEST_READY' => $base . '.ready', 'TSDM_TEST_UID' => '7',
        'TSDM_TEST_FAIL_SQL' => '',
        'TSDM_TEST_FAIL_TYPE' => '',
    ], $extra);
    $cgi = getenv('TSDM_PHP_CGI') ?: 'php-cgi';
    $command = [$cgi];
    if ($ini = php_ini_loaded_file()) array_push($command, '-c', $ini);
    // CI images may preload coverage extensions that conflict with PHP's JIT.
    // These request tests do not need JIT; keep genuine PHP warnings fatal below.
    array_push($command, '-d', 'display_errors=0', '-d', 'log_errors=1',
        '-d', 'opcache.jit=0', '-d', 'opcache.jit_buffer_size=0');
    $process = proc_open($command, [0 => ['pipe', 'r'], 1 => ['file', $base, 'w'],
        2 => ['file', $base . '.err', 'w']], $pipes, null, $env, ['bypass_shell' => true]);
    if (!is_resource($process)) throw new RuntimeException('Cannot start PHP CGI');
    fwrite($pipes[0], $body);
    fclose($pipes[0]);
    $worker = ['process' => $process, 'base' => $base, 'start' => microtime(true)];
    $GLOBALS['workers'][] = $worker;
    return $worker;
}
function finish_request($worker)
{
    while (proc_get_status($worker['process'])['running']) {
        if (microtime(true) - $worker['start'] > 25) {
            proc_terminate($worker['process']);
            throw new RuntimeException('PHP CGI request timed out');
        }
        usleep(10000);
    }
    proc_close($worker['process']);
    $raw = file_get_contents($worker['base']);
    $error = file_get_contents($worker['base'] . '.err');
    $parts = preg_split('/\r?\n\r?\n/', $raw, 2);
    $response = json_decode($parts[1] ?? '', true);
    if (!is_array($response)) throw new RuntimeException('Invalid CGI response: ' . $raw . $error);
    // Warnings must never disappear behind an otherwise valid success envelope.
    if ($error !== '' && !str_contains($error, 'Injected database failure')) throw new RuntimeException($error);
    return $response;
}
function request($endpoint, $action, $input = null, $extra = [])
{
    return finish_request(begin_request($endpoint, $action, $input, $extra));
}
function race($endpoint, $action, $inputs, $extra = [])
{
    $db = $GLOBALS['db'];
    $uid = (int)($extra['TSDM_TEST_UID'] ?? 7);
    $table = $endpoint === 'user' && $action === 'initialize' ? 'common_member' : 'pm_usersdata';
    $db->begin_transaction();
    $db->query("SELECT uid FROM $table WHERE uid = $uid FOR UPDATE");
    $requests = [];
    try {
        foreach ($inputs as $input) $requests[] = begin_request($endpoint, $action, $input, $extra);
        $deadline = microtime(true) + 10;
        foreach ($requests as $worker) {
            while (!is_file($worker['base'] . '.ready')) {
                if (microtime(true) > $deadline || !proc_get_status($worker['process'])['running']) {
                    throw new RuntimeException('Worker did not reach account lock: ' . file_get_contents($worker['base']) . file_get_contents($worker['base'] . '.err'));
                }
                clearstatcache();
                usleep(10000);
            }
        }
    } finally {
        $db->commit();
    }
    return array_map('finish_request', $requests);
}
function action_input($scene, $key, $fields = [])
{
    return array_merge(['request_id' => $key, 'engine_battle_id' => $scene['engine_battle_id'],
        'expected_revision' => $scene['revision']], $fields);
}

function pause_pet_list($uid)
{
    $worker = begin_request('pokemon', 'list', null, ['TSDM_TEST_UID' => (string)$uid,
        'TSDM_TEST_PAUSE_PET_LIST' => '1']);
    $deadline = microtime(true) + 10;
    while (!is_file($worker['base'] . '.ready')) {
        if (microtime(true) > $deadline || !proc_get_status($worker['process'])['running']) {
            throw new RuntimeException('Pet list did not reach its snapshot barrier');
        }
        clearstatcache();
        usleep(10000);
    }
    return $worker;
}

function resume_pet_list($worker)
{
    file_put_contents($worker['base'] . '.ready.resume', 'continue');
    return finish_request($worker);
}

try {
    $schema = file_get_contents(__DIR__ . '/../../docker/init.d/02-pokemon-schema.sql');
    $db->multi_query($schema);
    do { if ($result = $db->store_result()) $result->free(); } while ($db->more_results() && $db->next_result());
    $db->query('CREATE TABLE common_member (uid INT PRIMARY KEY, username VARCHAR(60) NOT NULL, groupid INT NOT NULL DEFAULT 10) ENGINE=InnoDB');
    $db->query("INSERT INTO common_member (uid, username) VALUES (7, 'fixture-player-7'), (8, 'fixture-player-8')");
    $db->query('INSERT INTO pm_usersdata (uid, money) VALUES (7, 100), (8, 100)');
    $db->query("INSERT INTO pm_data (id, name, hp, atk, def, spatk, spdef, speed, mapid, effort_values, drop_money, strength, met, capture) VALUES
        (1, 'Fixture ally', 100, 50, 100, 50, 100, 100, '', '{}', '1,1', 1, 100, 100),
        (2, 'Fixture enemy', 100, 1, 100, 1, 100, 1, '1', '{}', '1,1', 1, 100, 100)");
    $db->query("INSERT INTO pm_map (id, name, min_level, max_level, site, boss_config) VALUES (1, 'Fixture map', 50, 50, '', '')");
    $db->query("UPDATE pm_data SET xs = '普通'");
    $db->query("INSERT INTO pm_mypm (id, uid, pmname, species_id, level, hp, state, site) VALUES
        (501, 7, 'Fixture ally', 1, 50, 50, 1, 1), (502, 8, 'Fixture ally', 1, 50, 50, 1, 1)");
    $db->query("INSERT INTO pm_skill (id, available_pokemons, name, description, power, max_uses) VALUES (12, '1,2', 'Fixture move', '', 1, 20)");
    $db->query('INSERT INTO pm_myskill (id, uid, petid, skillid, skillnum) VALUES (701, 7, 501, 12, 2), (702, 8, 502, 12, 2)');
    $db->query("INSERT INTO pm_itemdata (id, name, type, module, effects, equipment, shop, money) VALUES
        (17, 'Fixture potion', 1, 'hp20', '{\"hp\":20}', '{}', 0, 0),
        (18, 'Fixture ether', 1, 'pp5', '{}', '{}', 0, 0),
        (24, 'Fixture purchase', 1, 'hp20', '{\"hp\":20}', '{}', 1, 60)");
    $db->query("INSERT INTO pm_myitem (uid, itemid, nums) VALUES (7, '17', 10), (7, '18', 10), (8, '17', 10)");

    $purchases = race('shop', 'buy', [['item_id' => 24, 'quantity' => 1], ['item_id' => 24, 'quantity' => 1]]);
    check(count(array_filter($purchases, fn($r) => $r['success'])) === 1, 'Concurrent purchases only spend available money');
    check((int)scalar('SELECT money FROM pm_usersdata WHERE uid = 7') === 40 && (int)scalar("SELECT SUM(nums) FROM pm_myitem WHERE uid = 7 AND itemid = '24'") === 1, 'Money and purchased inventory commit together');

    $db->query('DROP TABLE pm_battle_action');
    $db->query('ALTER TABLE pm_battle DROP COLUMN revision');
    $empty = request('battle', 'recover');
    check(!$empty['success'] && $empty['code'] === 404, 'Recover reports an empty battle after upgrading an old schema');
    check((bool)$db->query("SHOW COLUMNS FROM pm_battle LIKE 'revision'")->fetch_assoc()
        && (int)scalar('SELECT COUNT(*) FROM pm_battle_action') === 0, 'Old installations create revision and receipt schema before transactions');

    $start = ['request_id' => 'fixture-start-0001', 'engine_battle_id' => 0, 'expected_revision' => 0, 'map_id' => 1];
    $starts = race('battle', 'start', [$start, $start]);
    check($starts[0]['success'] && $starts[0] === $starts[1], 'Concurrent duplicate starts replay the exact committed response');
    check((int)scalar('SELECT COUNT(*) FROM pm_battle WHERE uid = 7') === 1, 'Duplicate start creates exactly one battle');
    $scene = $contracts['start'] = $starts[0]['data'];
    check($scene['engine_battle_id'] > 0 && $scene['revision'] === 1, 'Start returns authoritative battle ID and revision');
    $contracts['items'] = request('battle', 'get_battle_items')['data'];
    $contracts['inventory'] = request('user', 'inventory')['data'];
    $preview = action_input($scene, 'fixture-preview-01', ['item_id' => 18]);
    $selection = request('battle', 'use_item', $preview);
    check($selection['success'] && $selection['data']['requires_skill_selection'], 'PP item preview offers actual owned skill records');
    $contracts['pp_selection'] = $selection['data'];
    check((int)scalar('SELECT revision FROM pm_battle WHERE uid = 7') === $scene['revision'], 'Opening the PP picker does not change battle revision');

    $pp = action_input($scene, 'fixture-pp-use-001', ['item_id' => 18, 'skill_record_id' => 701]);
    $pp_responses = race('battle', 'use_item_on_skill', [$pp, $pp]);
    check($pp_responses[0]['success'] && $pp_responses[0] === $pp_responses[1], 'Concurrent duplicate PP items return one outcome');
    check((int)scalar("SELECT nums FROM pm_myitem WHERE uid = 7 AND itemid = '18'") === 9 && (int)scalar('SELECT skillnum FROM pm_myskill WHERE id = 701') === 7, 'PP and item quantity change exactly once');
    $scene = $contracts['pp_used'] = $pp_responses[0]['data'];

    $heal = action_input($scene, 'fixture-heal-00001', ['item_id' => 17]);
    $other_heal = array_merge($heal, ['request_id' => 'fixture-heal-00002']);
    $heals = race('battle', 'use_item', [$heal, $other_heal]);
    $successes = array_values(array_filter($heals, fn($r) => $r['success']));
    $failures = array_values(array_filter($heals, fn($r) => !$r['success']));
    check(count($successes) === 1 && count($failures) === 1 && $failures[0]['error_code'] === 'battle_state_conflict', 'Different actions based on one revision reject the stale competitor');
    check((int)scalar("SELECT nums FROM pm_myitem WHERE uid = 7 AND itemid = '17'") === 9, 'Competing heal consumes one potion');
    $scene = $contracts['healed'] = $successes[0]['data'];
    check((int)scalar('SELECT hp FROM pm_mypm WHERE id = 501') === $scene['my_pokemon']['hp'], 'Persisted HP matches the committed response');

    $rollback = action_input($scene, 'fixture-rollback-01', ['item_id' => 17]);
    $old_hp = scalar('SELECT hp FROM pm_mypm WHERE id = 501');
    $old_events = scalar('SELECT COUNT(*) FROM pm_battle_event');
    $failed = request('battle', 'use_item', $rollback, ['TSDM_TEST_FAIL_SQL' => 'INSERT INTO pm_battle_action']);
    check(!$failed['success'] && $failed['code'] === 500, 'Failure after gameplay writes returns an error');
    check((int)scalar("SELECT nums FROM pm_myitem WHERE uid = 7 AND itemid = '17'") === 9 && scalar('SELECT hp FROM pm_mypm WHERE id = 501') === $old_hp && scalar('SELECT COUNT(*) FROM pm_battle_event') === $old_events, 'Receipt failure rolls back items, HP and battle events');
    $retry = request('battle', 'use_item', $rollback);
    check($retry['success'], 'Rolled-back request can retry successfully with the same key');
    $scene = $contracts['retried'] = $retry['data'];
    check(request('battle', 'use_item', $rollback) === $retry, 'Lost response retry returns the original complete envelope');
    $conflict = request('battle', 'use_item', array_merge($rollback, ['item_id' => 18]));
    check(!$conflict['success'] && $conflict['error_code'] === 'request_id_conflict', 'Reusing a key with another payload is rejected');

    $turn = action_input($scene, 'fixture-turn-00001', ['skill_id' => 12]);
    $turns = race('battle', 'turn', [$turn, $turn]);
    check($turns[0]['success'] && $turns[0] === $turns[1], 'Duplicate skill turns resolve exactly once');
    $scene = $contracts['turn'] = $turns[0]['data'];
    check($scene['revision'] === $turn['expected_revision'] + 1, 'One committed turn advances revision once');
    $recover = request('battle', 'recover');
    check($recover['success'] && $recover['data']['engine_battle_id'] === $scene['engine_battle_id'] && $recover['data']['revision'] === $scene['revision'], 'Reconnect returns current battle and revision');
    $contracts['recover'] = $recover['data'];
    $contracts['log'] = request('battle', 'battle_log', null, ['QUERY_STRING' => 'action=battle_log&battle_id=' . $scene['engine_battle_id']])['data'];
    check(count($contracts['log']['turns']) > 0, 'Battle log contains grouped persisted turns');

    $second = request('battle', 'start', $start, ['TSDM_TEST_UID' => '8']);
    check($second['success'] && $second['data']['engine_battle_id'] !== $scene['engine_battle_id'], 'Request keys are isolated by account');
    $foreign = request('battle', 'battle_log', null, ['TSDM_TEST_UID' => '8', 'QUERY_STRING' => 'action=battle_log&battle_id=' . $scene['engine_battle_id']]);
    check(!$foreign['success'], 'Another account cannot read a private battle log');

    $db->query("INSERT INTO pm_itemdata (id, name, type, effects, equipment, captmax, ballid) VALUES (19, 'Fixture ball', 2, '{}', '{}', 255, 1)");
    $db->query("INSERT INTO pm_myitem (uid, itemid, nums) VALUES (7, '19', 2)");
    $capture = action_input($scene, 'fixture-capture-01', ['ball_id' => 19]);
    $captures = race('battle', 'capture', [$capture, $capture]);
    check($captures[0]['success'] && $captures[0] === $captures[1] && $captures[0]['data']['status'] === 'captured', 'Concurrent captures return the same terminal result');
    check((int)scalar("SELECT nums FROM pm_myitem WHERE uid = 7 AND itemid = '19'") === 1
        && (int)scalar('SELECT COUNT(*) FROM pm_mypm WHERE uid = 7') === 2, 'Capture consumes one ball and grants one Pokemon');
    $contracts['captured'] = $captures[0]['data'];
    check(request('battle', 'capture', $capture) === $captures[0], 'Lost terminal capture response remains replayable');
    check(request('battle', 'turn', $turn) === $turns[0], 'Committed receipt remains replayable after battle ends');
    $db->query("UPDATE pm_battle_action SET created_at = 1 WHERE uid = 7 AND request_id = 'fixture-start-0001'");
    $expired = request('battle', 'start', $start);
    check(!$expired['success'] && $expired['error_code'] === 'request_expired' && (int)scalar('SELECT COUNT(*) FROM pm_battle WHERE uid = 7') === 1, 'Expired start key cannot create a new battle');

    // A failed inventory write must not grant a free out-of-battle heal.
    $db->query("UPDATE pm_myitem SET nums = 1 WHERE uid = 7 AND itemid = '17'");
    $db->query('UPDATE pm_mypm SET hp = 10 WHERE id = 501');
    $failed_item = request('user', 'use_item', ['item_id' => 17, 'pokemon_id' => 501],
        ['TSDM_TEST_FAIL_SQL' => 'DELETE FROM pm_myitem']);
    check(!$failed_item['success'] && $failed_item['code'] === 500, 'Inventory write failure is reported to the player');
    check((int)scalar('SELECT hp FROM pm_mypm WHERE id = 501') === 10
        && (int)scalar("SELECT nums FROM pm_myitem WHERE uid = 7 AND itemid = '17'") === 1,
        'Failed item consumption rolls back the out-of-battle heal');

    $inventory_uses = race('user', 'use_item', array_fill(0, 2, ['item_id' => 17, 'pokemon_id' => 501]));
    check(count(array_filter($inventory_uses, fn($r) => $r['success'])) === 1,
        'Concurrent inventory uses cannot spend the last potion twice');
    check((int)scalar('SELECT hp FROM pm_mypm WHERE id = 501') === 30
        && (int)scalar("SELECT COUNT(*) FROM pm_myitem WHERE uid = 7 AND itemid = '17'") === 0,
        'One potion produces exactly one out-of-battle heal');

    $before_hp = scalar('SELECT hp FROM pm_mypm WHERE id = 502');
    $bypass = request('user', 'use_item', ['item_id' => 17, 'pokemon_id' => 502], ['TSDM_TEST_UID' => '8']);
    check(!$bypass['success'] && scalar('SELECT hp FROM pm_mypm WHERE id = 502') === $before_hp
        && (int)scalar("SELECT nums FROM pm_myitem WHERE uid = 8 AND itemid = '17'") === 10,
        'Inventory endpoint cannot bypass a battle turn to heal the active Pokemon');
    $flee_args = ['TSDM_TEST_UID' => '8', 'TSDM_TEST_STRICT' => '1',
        'QUERY_STRING' => 'action=heal_and_flee&pokemon_id=502'];
    $failed_flee = request('user', 'heal_and_flee', [], array_merge($flee_args,
        ['TSDM_TEST_FAIL_SQL' => 'UPDATE pm_myskill']));
    check(!$failed_flee['success'] && scalar('SELECT hp FROM pm_mypm WHERE id = 502') === $before_hp
        && (int)scalar('SELECT npcid FROM pm_usersdata WHERE uid = 8') > 0
        && scalar('SELECT phase FROM pm_battle WHERE uid = 8') === 'active',
        'Failed center treatment rolls back healing, battle termination and the legacy mirror');
    $healed_flee = request('user', 'heal_and_flee', [], $flee_args);
    check($healed_flee['success'] && (int)scalar('SELECT npcid FROM pm_usersdata WHERE uid = 8') === 0
        && scalar('SELECT phase FROM pm_battle WHERE uid = 8') === 'ended'
        && (int)scalar('SELECT revision FROM pm_battle WHERE uid = 8') === $second['data']['revision'] + 1,
        'Center treatment ends the actual battle and advances its revision in strict SQL mode');
    $recovered_flee = request('battle', 'recover', null, ['TSDM_TEST_UID' => '8']);
    check(!$recovered_flee['success'] && $recovered_flee['code'] === 404,
        'A battle abandoned at the center cannot reappear on reconnect');

    $db->query("INSERT INTO common_member (uid, username) VALUES (9, 'fixture-new-player')");
    $starters = race('user', 'initialize', [[], []], ['TSDM_TEST_UID' => '9']);
    check(count(array_filter($starters, fn($r) => $r['success'])) === 1
        && (int)scalar('SELECT COUNT(*) FROM pm_mypm WHERE uid = 9') === 1,
        'Concurrent initialization grants one starter to a new account');
    check((int)scalar('SELECT COUNT(*) FROM pm_usersdata WHERE uid = 9') === 1
        && (int)scalar('SELECT COUNT(*) FROM pm_mypm WHERE uid = 9 AND site = 1') === 1,
        'Starter and account are initialized together with one active Pokemon');
    $db->query("INSERT INTO common_member (uid, username) VALUES (10, 'fixture-failed-starter')");
    $failed_starter = request('user', 'initialize', [], ['TSDM_TEST_UID' => '10',
        'TSDM_TEST_FAIL_SQL' => 'INSERT INTO pm_mypm']);
    check(!$failed_starter['success'] && (int)scalar('SELECT COUNT(*) FROM pm_usersdata WHERE uid = 10') === 0
        && (int)scalar('SELECT COUNT(*) FROM pm_mypm WHERE uid = 10') === 0,
        'Failed starter creation leaves no incomplete account');

    foreach (['true', 'null', '42', '"text"', '{invalid'] as $body) {
        $invalid_json = request('shop', 'buy', $body);
        check(!$invalid_json['success'] && $invalid_json['code'] === 400,
            'Malformed or scalar JSON is rejected as bad input: ' . $body);
    }
    $type_error = request('user', 'inventory', null, ['TSDM_TEST_FAIL_SQL' => 'SELECT SQL_CALC_FOUND_ROWS',
        'TSDM_TEST_FAIL_TYPE' => 'TypeError']);
    check(!$type_error['success'] && $type_error['code'] === 500 && $type_error['error'] === 'Server Error',
        'Unexpected engine errors do not expose internal messages or server paths');

    $released_ids = $db->query('SELECT id FROM pm_mypm WHERE uid = 7 ORDER BY id')->fetch_all(MYSQLI_ASSOC);
    $release_responses = race('pokemon', 'release', [['id' => (int)$released_ids[0]['id']], ['id' => (int)$released_ids[1]['id']]]);
    check(count(array_filter($release_responses, fn($r) => $r['success'])) === 1
        && (int)scalar('SELECT COUNT(*) FROM pm_mypm WHERE uid = 7') === 1,
        'Concurrent releases cannot delete the last Pokemon');
    check((int)scalar('SELECT COUNT(*) FROM pm_mypm WHERE uid = 7 AND site = 1') === 1,
        'Concurrent releases preserve exactly one active Pokemon');

    $rename = "O'Brien \\ path";
    $remaining_id = (int)scalar('SELECT id FROM pm_mypm WHERE uid = 7');
    $renamed = request('pokemon', 'rename', ['id' => $remaining_id, 'name' => $rename]);
    check($renamed['success'] && scalar('SELECT nickname FROM pm_mypm WHERE uid = 7') === $rename,
        'Pokemon names preserve apostrophes and backslashes exactly');
    $unicode_name = '皮卡丘的冒險夥伴';
    $unicode_renamed = request('pokemon', 'rename', ['id' => $remaining_id, 'name' => $unicode_name]);
    check($unicode_renamed['success'] && scalar('SELECT nickname FROM pm_mypm WHERE uid = 7') === $unicode_name,
        'Chinese nicknames are limited by characters rather than UTF-8 byte length');
    foreach ([str_repeat('中', 21), '   ', '<b></b>', '<b> </b>'] as $invalid_name) {
        $invalid_rename = request('pokemon', 'rename', ['id' => $remaining_id, 'name' => $invalid_name]);
        check(!$invalid_rename['success'] && $invalid_rename['code'] === 400
            && scalar('SELECT nickname FROM pm_mypm WHERE uid = 7') === $unicode_name,
            'Rejected empty or overlong nickname preserves the previous name');
    }

    $db->query("INSERT INTO common_member (uid, username) VALUES (11, 'fixture-legacy-player')");
    $db->query("INSERT INTO pm_mypm (uid, species_id, pmname, site) VALUES (11, 1, 'Legacy companion', 1)");
    $legacy_profiles = race('user', 'profile', [null, null], ['TSDM_TEST_UID' => '11', 'TSDM_TEST_LEGACY_PROFILE' => '1']);
    check($legacy_profiles[0]['success'] && $legacy_profiles[1]['success']
        && (int)scalar('SELECT COUNT(*) FROM pm_usersdata WHERE uid = 11') === 1,
        'Concurrent legacy profile loads repair one account without duplicate-key errors');

    $db->query("INSERT INTO pm_itemdata (id, name, type, equipment, effects) VALUES (40, 'Fixture bracelet', 5, '{\"hp\":100}', '{}')");
    $db->query("INSERT INTO pm_myitem (uid, itemid, nums) VALUES (7, '40', 1)");
    $equipment_id = (int)$db->insert_id;
    $equip_input = ['pokemon_id' => $remaining_id, 'myitem_id' => $equipment_id, 'slot_index' => 0];
    $before_equipment_hp = scalar('SELECT hp FROM pm_mypm WHERE uid = 7');
    $failed_equip = request('pokemon', 'equip_item', $equip_input, ['TSDM_TEST_FAIL_SQL' => 'UPDATE pm_mypm SET hp =']);
    check(!$failed_equip['success'] && (int)scalar('SELECT equipmentid1 FROM pm_mypm WHERE uid = 7') === 0
        && scalar('SELECT hp FROM pm_mypm WHERE uid = 7') === $before_equipment_hp,
        'Failed equipment HP update rolls back the new slot assignment');
    $equipped = request('pokemon', 'equip_item', $equip_input);
    check($equipped['success'] && (int)scalar('SELECT equipmentid1 FROM pm_mypm WHERE uid = 7') === $equipment_id
        && (int)scalar('SELECT hp FROM pm_mypm WHERE uid = 7') === $equipped['data']['new_hp'],
        'Equipment and adjusted HP commit together');
    $unequip_input = ['pokemon_id' => $remaining_id, 'slot_index' => 0];
    $failed_unequip = request('pokemon', 'unequip_item', $unequip_input, ['TSDM_TEST_FAIL_SQL' => 'UPDATE pm_mypm SET hp =']);
    check(!$failed_unequip['success'] && (int)scalar('SELECT equipmentid1 FROM pm_mypm WHERE uid = 7') === $equipment_id
        && (int)scalar('SELECT hp FROM pm_mypm WHERE uid = 7') === $equipped['data']['new_hp'],
        'Failed unequip HP update restores the original equipment and HP');
    $unequipped = request('pokemon', 'unequip_item', $unequip_input);
    check($unequipped['success'] && (int)scalar('SELECT equipmentid1 FROM pm_mypm WHERE uid = 7') === 0,
        'Unequip succeeds after a rolled-back attempt');
    $db->query("INSERT INTO pm_mypm (uid, species_id, pmname, site, level, hp, state) VALUES (7, 1, 'Equipment competitor', 3, 50, 50, 1)");
    $equipment_competitor = (int)$db->insert_id;
    $equip_race = race('pokemon', 'equip_item', [$equip_input,
        array_merge($equip_input, ['pokemon_id' => $equipment_competitor])]);
    check(count(array_filter($equip_race, fn($r) => $r['success'])) === 1
        && (int)scalar('SELECT COUNT(*) FROM pm_mypm WHERE uid = 7 AND equipmentid1 = ' . $equipment_id) === 1,
        'Concurrent equipment requests cannot assign one item to two Pokemon');

    // Skill changes share the same account lock as battle actions and pet mutations.
    $db->query("INSERT INTO common_member (uid, username) VALUES (12, 'fixture-skill-player')");
    $db->query('INSERT INTO pm_usersdata (uid) VALUES (12)');
    $db->query("INSERT INTO pm_mypm (id, uid, species_id, pmname, site, level, hp, state) VALUES (601, 12, 1, 'Skill learner', 1, 50, 50, 1)");
    for ($skill_id = 301; $skill_id <= 305; $skill_id++) {
        $db->query("INSERT INTO pm_skill (id, available_pokemons, name, description, max_uses, level_required) VALUES ($skill_id, '1', 'Skill $skill_id', '', 20, 1)");
    }
    $db->query('INSERT INTO pm_myskill (uid, petid, skillid, skillnum) VALUES (12, 601, 301, 20), (12, 601, 302, 20), (12, 601, 303, 20)');
    $skill_owner = ['TSDM_TEST_UID' => '12'];
    $learn_race = race('pokemon', 'learn_skill', [
        ['pokemon_id' => 601, 'skill_id' => 304], ['pokemon_id' => 601, 'skill_id' => 305],
    ], $skill_owner);
    $learn_errors = array_values(array_filter($learn_race, fn($r) => !$r['success']));
    check(count($learn_errors) === 1 && $learn_errors[0]['error_code'] === 'skill_slots_full'
        && (int)scalar('SELECT COUNT(*) FROM pm_myskill WHERE uid = 12 AND petid = 601') === 4,
        'Concurrent different learns cannot exceed four skill slots');
    $db->query('DELETE FROM pm_myskill WHERE uid = 12');
    $learn_input = ['pokemon_id' => 601, 'skill_id' => 301];
    $duplicate_learn = race('pokemon', 'learn_skill', [$learn_input, $learn_input], $skill_owner);
    $duplicate_errors = array_values(array_filter($duplicate_learn, fn($r) => !$r['success']));
    check(count($duplicate_errors) === 1 && $duplicate_errors[0]['error_code'] === 'skill_already_learned'
        && (int)scalar('SELECT COUNT(*) FROM pm_myskill WHERE uid = 12 AND petid = 601 AND skillid = 301') === 1,
        'Concurrent duplicate learns store one learned skill');

    foreach (['learn_skill' => 302, 'forget_skill' => 301] as $action => $skill_id) {
        $db->begin_transaction();
        $db->query('SELECT uid FROM pm_usersdata WHERE uid = 12 FOR UPDATE');
        $worker = begin_request('pokemon', $action, ['pokemon_id' => 601, 'skill_id' => $skill_id], $skill_owner);
        try {
            $deadline = microtime(true) + 10;
            while (!is_file($worker['base'] . '.ready')) {
                if (microtime(true) > $deadline || !proc_get_status($worker['process'])['running']) {
                    throw new RuntimeException('Skill request did not wait for the account lock');
                }
                clearstatcache(); usleep(10000);
            }
            // Represent a battle-start transaction committing while this request
            // waits; its authoritative legacy projection now marks an encounter.
            $db->query('UPDATE pm_usersdata SET npcid = 2 WHERE uid = 12');
            $db->commit();
        } catch (Throwable $error) {
            $db->rollback(); throw $error;
        }
        $blocked = finish_request($worker);
        check(!$blocked['success'] && $blocked['error_code'] === 'skill_battle_restricted'
            && (int)scalar('SELECT COUNT(*) FROM pm_myskill WHERE uid = 12 AND skillid = 301') === 1
            && (int)scalar('SELECT COUNT(*) FROM pm_myskill WHERE uid = 12 AND skillid = 302') === 0,
            $action . ' observes a battle that committed before its account lock');
        $db->query('UPDATE pm_usersdata SET npcid = 0 WHERE uid = 12');
    }
    $skill_write_failure = request('pokemon', 'learn_skill', ['pokemon_id' => 601, 'skill_id' => 302],
        array_merge($skill_owner, ['TSDM_TEST_FAIL_SQL' => 'INSERT INTO pm_myskill']));
    check(!$skill_write_failure['success'] && $skill_write_failure['code'] === 500
        && (int)scalar('SELECT COUNT(*) FROM pm_myskill WHERE uid = 12') === 1,
        'Failed skill learning leaves the original learned set intact');
    $forget_failure = request('pokemon', 'forget_skill', $learn_input,
        array_merge($skill_owner, ['TSDM_TEST_FAIL_SQL' => 'DELETE FROM pm_myskill']));
    check(!$forget_failure['success'] && $forget_failure['code'] === 500
        && (int)scalar('SELECT COUNT(*) FROM pm_myskill WHERE uid = 12') === 1,
        'Failed skill forgetting preserves the learned skill');
    $forgotten = request('pokemon', 'forget_skill', $learn_input, $skill_owner);
    check($forgotten['success'] && (int)scalar('SELECT COUNT(*) FROM pm_myskill WHERE uid = 12') === 0,
        'A valid forget commits after a rolled-back attempt');
    $learned_again = request('pokemon', 'learn_skill', $learn_input, $skill_owner);
    check($learned_again['success'] && (int)scalar('SELECT skillnum FROM pm_myskill WHERE uid = 12 AND skillid = 301') === 20,
        'A valid learn commits full PP after competing and failed requests');

    // Read-time repairs must not overwrite a mutation that committed after SELECT.
    $db->query("INSERT INTO common_member (uid, username) VALUES (13, 'fixture-player-13')");
    $db->query('INSERT INTO pm_usersdata (uid, money) VALUES (13, 100)');
    $db->query("INSERT INTO pm_mypm (id, uid, species_id, pmname, site, level, hp, state) VALUES (701, 13, 1, 'Read repair', 1, 50, 0, 1)");
    $reader = ['TSDM_TEST_UID' => '13'];
    $stale_list = pause_pet_list(13);
    $healed = request('user', 'heal', [], array_merge($reader, ['QUERY_STRING' => 'action=heal&pokemon_id=701']));
    $list_result = resume_pet_list($stale_list);
    check($healed['success'] && $list_result['success']
        && (int)scalar('SELECT hp FROM pm_mypm WHERE id = 701') === 160
        && (int)scalar('SELECT state FROM pm_mypm WHERE id = 701') === 1,
        'Stale fainted-pet list cannot undo a committed heal');

    $db->query('UPDATE pm_data SET speed = 200, atk = 100 WHERE id = 2');
    $db->query("INSERT INTO pm_myskill (uid, petid, skillid, skillnum) VALUES (13, 701, 12, 20)");
    $repair_start = request('battle', 'start', ['map_id' => 1], $reader);
    check($repair_start['success'], 'Read repair fixture starts a real battle');
    $db->query('UPDATE pm_mypm SET hp = 1000 WHERE id = 701');
    $stale_list = pause_pet_list(13);
    // First turn repairs legacy over-healing; the next turn applies fresh damage.
    $repair_turn = request('battle', 'turn', ['skill_id' => 12], $reader);
    $repair_turn = request('battle', 'turn', ['skill_id' => 12], $reader);
    $damaged_hp = (int)scalar('SELECT hp FROM pm_mypm WHERE id = 701');
    $list_result = resume_pet_list($stale_list);
    check($repair_turn['success'] && $damaged_hp < 160 && $list_result['success']
        && (int)scalar('SELECT hp FROM pm_mypm WHERE id = 701') === $damaged_hp,
        'Stale over-healed list cannot restore HP consumed by a real battle turn');

    foreach (['level = 100', 'equipmentid1 = ' . $equipment_id] as $changed_maximum) {
        $db->query('UPDATE pm_mypm SET hp = 200, level = 50, state = 1, equipmentid1 = 0 WHERE id = 701');
        $stale_list = pause_pet_list(13);
        $db->query('UPDATE pm_mypm SET ' . $changed_maximum . ' WHERE id = 701');
        $list_result = resume_pet_list($stale_list);
        check($list_result['success'] && (int)scalar('SELECT hp FROM pm_mypm WHERE id = 701') === 200,
            'Read repair ignores an obsolete HP maximum after ' . $changed_maximum);
    }
    $db->query('UPDATE pm_mypm SET hp = 200, level = 50, state = 1, equipmentid1 = 0 WHERE id = 701');
    $repaired_list = request('pokemon', 'list', null, $reader);
    check($repaired_list['success'] && (int)scalar('SELECT hp FROM pm_mypm WHERE id = 701') === 160,
        'Uncontested list still repairs legacy HP above its current maximum');
    $db->query('UPDATE pm_mypm SET hp = 0, state = 1 WHERE id = 701');
    $repaired_list = request('pokemon', 'list', null, $reader);
    check($repaired_list['success'] && (int)scalar('SELECT state FROM pm_mypm WHERE id = 701') === 0,
        'Uncontested list still marks a zero-HP Pokemon fainted');

    // Pass through the real plugin router, including both independent switches.
    foreach ([0, 1] as $legacy_open) foreach ([0, 1] as $global_open) {
        $db->query("REPLACE INTO pm_config (`key`, value, data_type) VALUES ('is_open', '$global_open', 'boolean')");
        $route = ['TSDM_TEST_ROUTE_PLUGIN' => '1', 'TSDM_TEST_LEGACY_OPEN' => (string)$legacy_open];
        $maps = request('battle', 'maps', null, $route);
        $open = $legacy_open && $global_open;
        check($open ? $maps['success'] : (!$maps['success'] && $maps['error_code'] === 'game_closed'),
            "Router honors both maintenance switches: legacy=$legacy_open global=$global_open");
        if (!$open) {
            $battle_count = scalar('SELECT COUNT(*) FROM pm_battle');
            $closed_start = request('battle', 'start', ['map_id' => 1], $route);
            check(!$closed_start['success'] && $closed_start['error_code'] === 'game_closed'
                && scalar('SELECT COUNT(*) FROM pm_battle') === $battle_count,
                'A valid formhash cannot start battles through a closed plugin');
        }
    }
    $db->query("UPDATE pm_config SET value = '0' WHERE `key` = 'is_open'");
    $closed_route = ['TSDM_TEST_ROUTE_PLUGIN' => '1', 'TSDM_TEST_LEGACY_OPEN' => '0'];
    $closed_config = request('config', 'global_config', null, $closed_route);
    check($closed_config['success'] && $closed_config['data']['is_open'] === false
        && isset($closed_config['data']['news_announcements']), 'Closed router still exposes announcement configuration');
    $staff_maps = request('battle', 'maps', null, $closed_route + ['TSDM_TEST_STAFF' => '1']);
    check($staff_maps['success'], 'Named game staff retains routed API access with both switches closed');

    // Readers must share legacy migration and never erase an administrator's newer list.
    $old_news = [['title' => 'Trainer\'s "news" C:\\news\\today', 'url' => 'https://example.com/?q="news"']];
    $db->query("DELETE FROM pm_config WHERE `key` IN ('news_announcements', 'ann_title', 'ann_url')");
    $legacy_title = $db->real_escape_string($old_news[0]['title']);
    $legacy_url = $db->real_escape_string($old_news[0]['url']);
    $db->query("INSERT INTO pm_config VALUES ('ann_title', '$legacy_title', 'string'), ('ann_url', '$legacy_url', 'string')");
    $config_news = request('config', 'global_config', null, $closed_route);
    check($config_news['success'] && $config_news['data']['news_announcements'] === $old_news,
        'First config read migrates legacy announcements without changing escaped text');
    $topics_news = request('topics', 'list');
    check($topics_news['success'] && $topics_news['data']['news_announcements'] === $old_news,
        'Topics retains announcements after config performed the migration');
    $db->query("UPDATE pm_config SET value = '[]' WHERE `key` = 'news_announcements'");
    $cleared_news = request('config', 'global_config', null, $closed_route);
    check($cleared_news['success'] && $cleared_news['data']['news_announcements'] === [],
        'Explicitly cleared announcements do not restore legacy notices');

    foreach ([false, true] as $admin_saved) {
        $db->query("DELETE FROM pm_config WHERE `key` = 'news_announcements'");
        $news_workers = [];
        foreach ([1, 2] as $_) {
            $news_workers[] = begin_request('config', 'global_config', null,
                $closed_route + ['TSDM_TEST_PAUSE_NEWS_INSERT' => '1']);
        }
        $deadline = microtime(true) + 10;
        foreach ($news_workers as $worker) {
            while (!is_file($worker['base'] . '.ready')) {
                if (microtime(true) > $deadline || !proc_get_status($worker['process'])['running']) {
                    throw new RuntimeException('Announcement reader failed to reach insert barrier');
                }
                usleep(10000);
                clearstatcache();
            }
        }
        $expected_news = $old_news;
        if ($admin_saved) {
            $expected_news = [['title' => 'Fresh administrator notice', 'url' => 'https://example.com/new']];
            $saved_news = $db->real_escape_string(json_encode($expected_news));
            $db->query("INSERT INTO pm_config VALUES ('news_announcements', '$saved_news', 'string')");
        } else {
            // Hold an uncommitted duplicate so both first-read inserts wait on
            // the same record. Rolling it back exposes INSERT IGNORE's shared-
            // lock upgrade deadlock deterministically, rather than by chance.
            $db->begin_transaction();
            $db->query("INSERT INTO pm_config VALUES ('news_announcements', '[]', 'string')");
        }
        foreach ($news_workers as $worker) file_put_contents($worker['base'] . '.ready.resume', 'resume');
        if (!$admin_saved) {
            try {
                $reader_ids = array_map(function ($worker) { return (int)file_get_contents($worker['base'] . '.ready'); }, $news_workers);
                $deadline = microtime(true) + 10;
                while ((int)scalar("SELECT COUNT(*) FROM information_schema.PROCESSLIST
                    WHERE COMMAND = 'Query' AND INFO LIKE 'INSERT%INTO pm_config%'
                    AND ID IN (" . implode(',', $reader_ids) . ')') !== 2) {
                    // A deadlock victim can finish before the other reader is
                    // visible in PROCESSLIST; report its actual response.
                    if (!proc_get_status($news_workers[0]['process'])['running']
                        || !proc_get_status($news_workers[1]['process'])['running']) break;
                    if (microtime(true) > $deadline) throw new RuntimeException('Both announcement readers must reach the duplicate-record lock');
                    usleep(10000);
                }
            } finally {
                $db->rollback();
            }
        }
        foreach ($news_workers as $worker) {
            $result = finish_request($worker);
            check($result['success'] && $result['data']['news_announcements'] === $expected_news,
                $admin_saved ? 'Stale reader returns the newer administrator announcement'
                    : 'Concurrent first reads migrate announcements without duplicate-key failure');
        }
        check(json_decode(scalar("SELECT value FROM pm_config WHERE `key` = 'news_announcements'"), true) === $expected_news,
            $admin_saved ? 'Read migration cannot overwrite a committed administrator list'
                : 'Concurrent migration persists the original legacy announcement');
    }

    // Signed SMALLINT inventory capacity must be enforced before either SQL mode truncates/errors.
    $db->query("INSERT INTO common_member (uid, username) VALUES (14, 'fixture-player-14')");
    $db->query('INSERT INTO pm_usersdata (uid, money) VALUES (14, 10000)');
    foreach (['0', '1'] as $strict) {
        $shopper = ['TSDM_TEST_UID' => '14', 'TSDM_TEST_STRICT' => $strict];
        $db->query('UPDATE pm_usersdata SET money = 10000 WHERE uid = 14');
        $db->query('DELETE FROM pm_myitem WHERE uid = 14');
        $bulk = request('shop', 'buy', ['item_id' => 24, 'quantity' => 99], $shopper);
        check($bulk['success'] && $bulk['data']['items_purchased'] === 99
            && $bulk['data']['total_cost'] === 5940 && $bulk['data']['remaining_money'] === 4060,
            "A 99-item order preserves the purchase response in SQL strict=$strict");
        check((int)scalar("SELECT nums FROM pm_myitem WHERE uid = 14 AND itemid = '24'") === 99
            && (int)scalar('SELECT money FROM pm_usersdata WHERE uid = 14') === 4060,
            "A 99-item order charges and delivers the full quantity in SQL strict=$strict");

        $db->query('UPDATE pm_usersdata SET money = 10000 WHERE uid = 14');
        $db->query('UPDATE pm_myitem SET nums = 32668 WHERE uid = 14');
        $exact = request('shop', 'buy', ['item_id' => 24, 'quantity' => 99], $shopper);
        check($exact['success'] && (int)scalar('SELECT nums FROM pm_myitem WHERE uid = 14') === 32767
            && (int)scalar('SELECT money FROM pm_usersdata WHERE uid = 14') === 4060,
            "Purchasing up to exactly 32767 succeeds in SQL strict=$strict");

        foreach ([[32767, 1], [32766, 2], [-1, 1]] as [$stock, $quantity]) {
            $db->query('UPDATE pm_usersdata SET money = 10000 WHERE uid = 14');
            $db->query("UPDATE pm_myitem SET nums = $stock WHERE uid = 14");
            $rejected = request('shop', 'buy', ['item_id' => 24, 'quantity' => $quantity], $shopper);
            check(!$rejected['success'] && $rejected['code'] === 400
                && (int)scalar('SELECT nums FROM pm_myitem WHERE uid = 14') === $stock
                && (int)scalar('SELECT money FROM pm_usersdata WHERE uid = 14') === 10000,
                "Invalid stock/capacity rolls back the complete order: $stock + $quantity, SQL strict=$strict");
        }

        $db->query('UPDATE pm_myitem SET nums = 0 WHERE uid = 14');
        $stack_id = scalar('SELECT id FROM pm_myitem WHERE uid = 14');
        $refilled = request('shop', 'buy', ['item_id' => 24, 'quantity' => 99], $shopper);
        check($refilled['success'] && (int)scalar('SELECT COUNT(*) FROM pm_myitem WHERE uid = 14') === 1
            && scalar('SELECT id FROM pm_myitem WHERE uid = 14') === $stack_id
            && (int)scalar('SELECT nums FROM pm_myitem WHERE uid = 14') === 99,
            "Purchases reuse an empty existing stack in SQL strict=$strict");

        $db->query('UPDATE pm_usersdata SET money = 10000 WHERE uid = 14');
        $db->query('UPDATE pm_myitem SET nums = 32766 WHERE uid = 14');
        $contenders = race('shop', 'buy', [['item_id' => 24, 'quantity' => 1], ['item_id' => 24, 'quantity' => 1]], $shopper);
        $accepted = array_values(array_filter($contenders, fn($r) => $r['success']));
        $denied = array_values(array_filter($contenders, fn($r) => !$r['success']));
        check(count($accepted) === 1 && count($denied) === 1 && $denied[0]['code'] === 400,
            "Only one simultaneous purchase can fill the final inventory slot in SQL strict=$strict");
        check((int)scalar('SELECT nums FROM pm_myitem WHERE uid = 14') === 32767
            && (int)scalar('SELECT money FROM pm_usersdata WHERE uid = 14') === 9940,
            "Inventory capacity race charges only for the delivered item in SQL strict=$strict");

        $db->query('UPDATE pm_usersdata SET money = 10000 WHERE uid = 14');
        $db->query('UPDATE pm_myitem SET nums = 10 WHERE uid = 14');
        $failed_bulk = request('shop', 'buy', ['item_id' => 24, 'quantity' => 99],
            $shopper + ['TSDM_TEST_FAIL_SQL' => 'UPDATE pm_myitem']);
        check(!$failed_bulk['success'] && $failed_bulk['code'] === 500
            && (int)scalar('SELECT nums FROM pm_myitem WHERE uid = 14') === 10
            && (int)scalar('SELECT money FROM pm_usersdata WHERE uid = 14') === 10000,
            "Failed bulk delivery rolls back the entire debit in SQL strict=$strict");
    }

    // Equipment slots reference a whole inventory row even when its quantity exceeds one.
    $db->query("INSERT INTO pm_myitem (uid, itemid, nums) VALUES (14, '40', 2)");
    $occupied_stack = $db->insert_id;
    $db->query("INSERT INTO pm_mypm (id, uid, species_id, pmname, site, level, hp, state, equipmentid1) VALUES
        (801, 14, 1, 'Equipment owner', 1, 50, 160, 1, $occupied_stack),
        (802, 14, 1, 'Equipment observer', 2, 50, 160, 1, 0)");
    foreach ([801 => true, 802 => false] as $pet_id => $equipped_here) {
        $equipment_view = request('pokemon', 'equipment', null,
            ['TSDM_TEST_UID' => '14', 'QUERY_STRING' => 'action=equipment&pokemon_id=' . $pet_id]);
        $stack = $equipment_view['data']['owned_items'][0] ?? [];
        check($equipment_view['success'] && ($stack['myitem_id'] ?? 0) === $occupied_stack
            && $stack['quantity'] === 2 && $stack['equipped_count'] === 1 && $stack['available_count'] === 0
            && $stack['is_equipped'] === $equipped_here,
            "Occupied inventory row cannot be reused despite quantity 2: Pokemon $pet_id");
    }
    $unequipped_stack = request('pokemon', 'unequip_item', ['pokemon_id' => 801, 'slot_index' => 0], ['TSDM_TEST_UID' => '14']);
    $equipment_view = request('pokemon', 'equipment', null,
        ['TSDM_TEST_UID' => '14', 'QUERY_STRING' => 'action=equipment&pokemon_id=802']);
    $stack = $equipment_view['data']['owned_items'][0] ?? [];
    check($unequipped_stack['success'] && $equipment_view['success'] && $stack['quantity'] === 2
        && $stack['equipped_count'] === 0 && $stack['available_count'] === 2 && $stack['is_equipped'] === false,
        'Unequipping the row restores its full available quantity');

    // Boss configuration must survive the actual dispatcher, persistence and replay.
    $db->query("UPDATE pm_battle SET phase = 'ended', result = 'abandoned' WHERE uid = 7");
    $db->query('UPDATE pm_usersdata SET npcid = 0, strength = 1 WHERE uid = 7');
    $db->query('UPDATE pm_mypm SET site = 2 WHERE uid = 7');
    $db->query("UPDATE pm_mypm SET site = 1, hp = 160, state = 1, level = 50, species_id = 1, pmname = 'Fixture ally',
        equipmentid1 = 0, equipmentid2 = 0, equipmentid3 = 0, equipmentid4 = 0 WHERE id = $remaining_id");
    $db->query('UPDATE pm_data SET atk = 1, speed = 1 WHERE id = 2');
    $db->query('DELETE FROM pm_myskill WHERE uid = 7');
    $db->query("INSERT INTO pm_myskill (id,uid,petid,skillid,skillnum) VALUES (1701,7,$remaining_id,12,20)");
    $db->query("DELETE FROM pm_myitem WHERE uid = 7 AND itemid = '17'");
    $db->query("INSERT INTO pm_myitem (uid,itemid,nums) VALUES (7,'17',10)");
    $iv_keys = ['hit_points','attack','defense','special_attack','special_defense','speed'];
    $bosses = [
        ['pokemon_type_id' => 2, 'pokemon_name' => 'Fixture Boss', 'level' => 80, 'boss_multiplier' => 2, 'attributes' => array_fill_keys($iv_keys, 0)],
        ['pokemon_type_id' => 2, 'pokemon_name' => 'Fixture Boss', 'level' => 50, 'boss_multiplier' => 3, 'attributes' => array_fill_keys($iv_keys, 255)],
    ];
    $boss_json = $db->real_escape_string(json_encode(['bosses' => $bosses]));
    $db->query("UPDATE pm_map SET site = 'g', is_enabled = 1, experience = -1, boss_config = '$boss_json' WHERE id = 1");
    $map_result = request('battle', 'maps');
    $contracts['boss_maps'] = $map_result['data'];
    $boss_list = $map_result['data']['maps'][0]['bosses'];
    check($boss_list[0]['boss_index'] === 1 && $boss_list[0]['level'] === 50 && $boss_list[1]['boss_index'] === 0,
        'Game map list preserves original Boss indices after sorting by level');
    $boss_start = ['request_id' => 'fixture-boss-start-01', 'engine_battle_id' => 0, 'expected_revision' => 0,
        'map_id' => 1, 'boss_pokemon_type_id' => 2, 'boss_index' => 1];
    foreach ([['boss_index' => -1], ['boss_index' => '1'], ['boss_index' => null], ['boss_index' => 3], ['boss_pokemon_type_id' => 1]] as $invalid) {
        $bad_boss = request('battle', 'start', array_merge($boss_start, $invalid));
        check(!$bad_boss['success'] && $bad_boss['code'] === 400, 'Invalid Boss variant is rejected before creating a battle: ' . json_encode($invalid));
    }
    $boss_result = request('battle', 'start', $boss_start);
    $boss_scene = $boss_result['data'] ?? [];
    $contracts['boss_start'] = $boss_scene;
    check($boss_result['success'] && $boss_scene['wild_pokemon']['level'] === 50 && $boss_scene['wild_pokemon']['boss_multiplier'] === 3,
        'Explicit second same-species Boss uses its own configured level and multiplier');
    $boss_id = (int)$boss_scene['engine_battle_id'];
    check((int)scalar("SELECT hp FROM pm_battle_unit WHERE battle_id = $boss_id AND side = 'enemy'") >= 861,
        'Configured Boss IV 255 reaches the persisted combat stats');
    check(request('battle', 'start', $boss_start) === $boss_result, 'Lost Boss start response replays exactly');
    $different_boss = request('battle', 'start', array_merge($boss_start, ['boss_index' => 0]));
    check(!$different_boss['success'] && $different_boss['code'] === 409 && $different_boss['error_code'] === 'request_id_conflict',
        'One request key cannot be changed to another same-species Boss');
    $boss_turn = request('battle', 'turn', action_input($boss_scene, 'fixture-boss-turn-01', ['skill_id' => 12]));
    $contracts['boss_turn'] = $boss_turn['data'] ?? [];
    check($boss_turn['success'] && $boss_turn['data']['wild_pokemon']['is_boss'] && $boss_turn['data']['wild_pokemon']['boss_multiplier'] === 3
        && str_starts_with($boss_turn['data']['wild_pokemon']['name'], '[BOSS] '), 'Boss identity remains visible after an actual turn');
    $boss_recover = request('battle', 'recover');
    check($boss_recover['success'] && $boss_recover['data']['wild_pokemon'] === $boss_turn['data']['wild_pokemon'],
        'Recovered Boss metadata agrees with the latest action response');
    $db->query("UPDATE pm_mypm SET hp = 100 WHERE id = $remaining_id");
    $boss_item = request('battle', 'use_item', action_input($boss_turn['data'], 'fixture-boss-item-01', ['item_id' => 17]));
    check($boss_item['success'] && $boss_item['data']['wild_pokemon']['is_boss'] && $boss_item['data']['wild_pokemon']['boss_multiplier'] === 3,
        'Boss metadata also survives a healing item action');
    $db->query("UPDATE pm_battle_unit SET hp = 1 WHERE battle_id = $boss_id AND side = 'enemy'");
    $db->query('UPDATE pm_usersdata SET hp = 1 WHERE uid = 7');
    // This assertion tests victory metadata, not the faster Boss's 20% evasion.
    // Seed 2 draws enemy damage/crit, then an ally hit roll of 46 (>20).
    $db->query("UPDATE pm_battle SET rng_seed = 2, rng_counter = 0 WHERE id = $boss_id");
    $boss_victory = request('battle', 'turn', action_input($boss_item['data'], 'fixture-boss-victory', ['skill_id' => 12]));
    $contracts['boss_victory'] = $boss_victory['data'] ?? [];
    check($boss_victory['success'] && $boss_victory['data']['status'] === 'victory' && $boss_victory['data']['wild_pokemon']['is_boss']
        && $boss_victory['data']['wild_pokemon']['boss_multiplier'] === 3, 'Final Boss result retains its identity after the mirror is cleared');

    $db->query('UPDATE pm_map SET experience = 30 WHERE id = 1');
    $hybrid_maps = request('battle', 'maps');
    $contracts['hybrid_maps'] = $hybrid_maps['data'];
    check($hybrid_maps['data']['maps'][0]['mode'] === 'hybrid', 'Existing mixed maps advertise both wild and Boss choices');
    $hybrid_result = request('battle', 'start', ['map_id' => 1]);
    check($hybrid_result['success'] && !$hybrid_result['data']['wild_pokemon']['is_boss'], 'Ordinary start in a mixed map remains a wild encounter');
    $db->query("UPDATE pm_battle SET phase = 'ended', result = 'abandoned' WHERE uid = 7");
    $db->query('UPDATE pm_usersdata SET npcid = 0 WHERE uid = 7');
    $legacy_boss = request('battle', 'start', ['map_id' => 1, 'boss_pokemon_type_id' => 2]);
    check($legacy_boss['success'] && $legacy_boss['data']['wild_pokemon']['level'] === 80,
        'Legacy species-only Boss requests retain first-match selection');
    $db->query("UPDATE pm_battle SET phase = 'ended', result = 'abandoned' WHERE uid = 7");
    $db->query('UPDATE pm_usersdata SET npcid = 0 WHERE uid = 7');
    $db->query("UPDATE pm_map SET experience = -1, boss_config = '{\"bosses\":[{\"pokemon_type_id\":2}]}' WHERE id = 1");
    $default_boss = request('battle', 'start', ['map_id' => 1, 'boss_pokemon_type_id' => 2, 'boss_index' => 0]);
    check($default_boss['success'] && $default_boss['data']['wild_pokemon']['level'] === 50 && $default_boss['data']['wild_pokemon']['boss_multiplier'] === 1.5,
        'Legacy Boss configuration uses the management level and multiplier defaults');

    if ($output = getenv('TSDM_API_CONTRACT_FIXTURES')) {
        file_put_contents($output, json_encode($contracts, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE | JSON_THROW_ON_ERROR));
    }
    echo "Real database checks passed: $passed", PHP_EOL;
} finally {
    foreach ($workers as $worker) {
        if (is_resource($worker['process'])) { proc_terminate($worker['process']); proc_close($worker['process']); }
        foreach (['', '.err', '.ready', '.ready.resume'] as $suffix) if (is_file($worker['base'] . $suffix)) unlink($worker['base'] . $suffix);
    }
    // This name is generated here, never accepted from a caller or site config.
    if (!preg_match('/^tsdm_test_[a-f0-9]{16}$/D', $database)) throw new RuntimeException('Unsafe database name');
    $db->query('DROP DATABASE `' . $database . '`');
    $db->close();
}
