<?php
/** Exercise the real item-use endpoint, hunger modules and HP calculation. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/utils.php';
require __DIR__ . '/../../plugin/api/constants.php';
require $argv[2] ?? __DIR__ . '/../../plugin/api/item_modules.php';

function load_item_functions($path, $wanted)
{
    $tokens = token_get_all(file_get_contents($path));
    for ($i = 0; $i < count($tokens); $i++) {
        if (!is_array($tokens[$i]) || $tokens[$i][0] !== T_FUNCTION) continue;
        $start = $i;
        while (++$i < count($tokens) && (!is_array($tokens[$i]) || $tokens[$i][0] !== T_STRING)) {}
        $name = $tokens[$i][1];
        $depth = 0;
        $body = '';
        $opened = false;
        for ($j = $start; $j < count($tokens); $j++) {
            $token = $tokens[$j];
            $body .= is_array($token) && $token[0] === T_DIR ? var_export(realpath(__DIR__ . '/../../plugin/api'), true) : (is_array($token) ? $token[1] : $token);
            if ($token === '{' || (is_array($token) && in_array($token[0], [T_CURLY_OPEN, T_DOLLAR_OPEN_CURLY_BRACES], true))) {
                $depth++;
                $opened = true;
            } elseif ($token === '}' && --$depth === 0 && $opened) break;
        }
        if (in_array($name, $wanted, true)) eval($body);
        $i = $j;
    }
}
load_item_functions($argv[1] ?? __DIR__ . '/../../plugin/api/user.php', ['api_use_item', 'api_get_usable_pokemon', 'downgrade_weak_state']);
load_item_functions(__DIR__ . '/../../plugin/api/index.php', ['pm_sql', 'pm_sql_v']);

class ItemResponse extends RuntimeException
{
    public $data;
    public function __construct($code, $data)
    {
        parent::__construct(is_string($data) ? $data : 'success', $code);
        $this->data = $data;
    }
}
function api_success($data) { throw new ItemResponse(200, $data); }
function api_error($message, $status = 400) { throw new ItemResponse($status, $message); }
function require_login() {}
function pm_table($name) { return $name; }
function validate_uid($value) { return (int) $value; }
function validate_id($value, $name) { return (int) $value; }
function get_json_input() { return $GLOBALS['input']; }
function get_param($name, $default = null) { return $GLOBALS['input'][$name] ?? $default; }

class DB
{
    public static $pet, $item, $stock, $writes;
    public static function fetch_first($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if ($sql === 'SELECT * FROM pm_usersdata WHERE uid = 7 FOR UPDATE') return ['uid' => 7, 'npcid' => 0];
        if ($sql === 'SELECT * FROM pm_itemdata WHERE id = 42') return self::$item;
        if ($sql === "SELECT * FROM pm_myitem WHERE uid = 7 AND itemid = '42'") return self::$stock;
        if ($sql === 'SELECT * FROM pm_mypm WHERE id = 1') return self::$pet;
        if ($sql === 'SELECT * FROM pm_mypm WHERE id = 1 AND uid = 7') return self::$pet['uid'] === 7 ? self::$pet : false;
        if ($sql === 'SELECT * FROM pm_data WHERE id = 1') return ['id' => 1, 'name' => 'Test', 'hp' => 50, 'atk' => 50, 'def' => 50, 'spatk' => 50, 'spdef' => 50, 'speed' => 50];
        if ($sql === 'SELECT hp FROM pm_data WHERE id = 1') return ['hp' => 50];
        if ($sql === 'SELECT name, hp FROM pm_data WHERE id = 1') return ['name' => 'Test', 'hp' => 50];
        if ($sql === 'SELECT i.equipment FROM pm_myitem m LEFT JOIN pm_itemdata i ON m.itemid=i.id WHERE m.id=9') return ['equipment' => '{"hp":20}'];
        if ($sql === "SELECT * FROM pm_itemdata WHERE module = 'yypg' LIMIT 1") return false;
        throw new RuntimeException('Unexpected read: ' . $sql);
    }
    public static function fetch_all($sql)
    {
        if (!preg_match('/^SELECT (.+) FROM pm_mypm WHERE uid = 7$/', trim($sql), $m)) throw new RuntimeException('Unexpected list: ' . $sql);
        if (self::$pet['uid'] !== 7) return [];
        // Enforce the actual projection, so omitted shiny/equipment columns fail.
        return [array_intersect_key(self::$pet, array_flip(explode(', ', $m[1])))];
    }
    public static function query($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (in_array($sql, ['START TRANSACTION', 'COMMIT', 'ROLLBACK'], true)) return;
        self::$writes[] = $sql;
        if (preg_match('/^UPDATE pm_mypm SET hp = (\d+)(?:, state = (\d+), statetime = (\d+))? WHERE id = 1$/', $sql, $m)) {
            self::$pet['hp'] = (int) $m[1];
            if (isset($m[2])) self::$pet['state'] = (int) $m[2];
        } elseif (preg_match('/^UPDATE pm_mypm SET hpg = (\d+), atkg = (\d+), defg = (\d+), spatkg = (\d+), spdefg = (\d+), sdg = (\d+), hp = (\d+) WHERE id = 1$/', $sql, $m)) {
            foreach (['hpg' => 1, 'atkg' => 2, 'defg' => 3, 'spatkg' => 4, 'spdefg' => 5, 'sdg' => 6] as $column => $group) {
                self::$pet[$column] = (int) $m[$group];
            }
            self::$pet['hp'] = (int) $m[7];
        } elseif ($sql === 'DELETE FROM pm_myitem WHERE id = 2') {
            self::$stock = false;
        } elseif (preg_match('/^UPDATE pm_myitem SET nums = (\d+) WHERE id = 2$/', $sql, $m)) {
            self::$stock['nums'] = (int) $m[1];
        } else {
            throw new RuntimeException('Unexpected write: ' . $sql);
        }
    }
}

$GLOBALS['_G'] = ['uid' => 7];
function fixture($state, $hp = 10, $type = 1, $module = 'hunger')
{
    $GLOBALS['input'] = ['item_id' => 42, 'pokemon_id' => 1];
    DB::$pet = ['id' => 1, 'uid' => 7, 'site' => 1, 'species_id' => 1, 'nickname' => 'Test', 'hp' => $hp, 'level' => 50, 'hpg' => 20, 'hpn' => 0, 'state' => $state, 'is_shiny' => 0, 'equipmentid1' => 9, 'equipmentid2' => 0, 'equipmentid3' => 0, 'equipmentid4' => 0];
    // The seeded milk selects hunger through sitemname, not module.
    DB::$item = ['id' => 42, 'name' => 'Milk', 'type' => $type, 'module' => '', 'sitemname' => $module, 'tpname' => 'yypg', 'effects' => '{"hp":50}'];
    DB::$stock = ['id' => 2, 'uid' => 7, 'itemid' => 42, 'nums' => 2];
    DB::$writes = [];
}
function invoke($endpoint = 'api_use_item')
{
    try { $endpoint(); } catch (ItemResponse $response) { return $response; }
    throw new RuntimeException('No response');
}
$passed = 0;
function check($condition, $message)
{
    if (!$condition) throw new RuntimeException($message);
    $GLOBALS['passed']++;
}

foreach ([1, 4] as $type) {
    foreach ([5, 6, 7] as $state) {
        foreach ([false, true] as $full) {
            fixture($state, 10, $type);
            if ($full) DB::$pet['hp'] = api_calculate_pokemon_max_hp(DB::$pet);
            $expected_hp = min(140, DB::$pet['hp'] + 50);
            $response = invoke();
            check($response->getCode() === 200 && DB::$pet['state'] === 1, "Type $type cures hunger stage $state even when full=$full");
            check(DB::$pet['hp'] === $expected_hp, "Type $type uses the cured HP maximum for stage $state");
            check(DB::$stock['nums'] === 1, 'Successful treatment consumes one item');
            if ($type === 1) check($response->data['pokemon_updated']['max_hp'] === 140 && $response->data['pokemon_updated']['state_changed'], 'Response contains the healed state and maximum');
        }
    }
}

fixture(1, 140);
check(invoke()->getCode() === 400 && !DB::$writes, 'Healthy full-HP pets do not consume milk');

fixture(5, 10, 1, 'hp1');
check(invoke()->getCode() === 200 && DB::$pet['state'] === 5, 'Ordinary potions keep hunger status');

fixture(8, 10);
check(invoke()->getCode() === 200 && DB::$pet['state'] === 8, 'Milk preserves beneficial status');

fixture(5);
DB::$pet['uid'] = 8;
check(invoke()->getCode() === 403 && !DB::$writes, 'Items cannot treat another user pet');

fixture(5);
DB::$stock = false;
check(invoke()->getCode() === 400 && !DB::$writes, 'Missing inventory cannot produce healing');

fixture(0, 0);
check(invoke()->getCode() === 200 && DB::$pet['state'] === 20 && DB::$pet['hp'] === 50, 'Milk preserves existing critical-to-weak revival behavior');

fixture(7, 30);
DB::$item['module'] = 'yypg';
check(invoke()->getCode() === 200 && DB::$pet['state'] === 1 && DB::$pet['hp'] === 80, 'Explicit milk modules also cure starvation');

foreach ([0, 5, 6, 7, 20, 21, 22] as $state) {
    fixture($state);
    DB::$pet['hp'] = api_calculate_pokemon_max_hp(DB::$pet);
    $response = invoke('api_get_usable_pokemon');
    check(count($response->data['usable_pokemon']) === 1 && !DB::$writes, "Full-HP state $state remains selectable for treatment");
}
fixture(1, 130);
$response = invoke('api_get_usable_pokemon');
check(count($response->data['usable_pokemon']) === 1 && $response->data['usable_pokemon'][0]['max_hp'] === 140, 'Equipment HP is included when choosing injured pets');
fixture(1, 150);
DB::$pet['is_shiny'] = 1;
$response = invoke('api_get_usable_pokemon');
check(count($response->data['usable_pokemon']) === 1 && $response->data['usable_pokemon'][0]['max_hp'] === 200, 'Shiny HP is included when choosing injured pets');
fixture(5, 128, 1, 'hp1');
check(invoke('api_get_usable_pokemon')->data['usable_pokemon'] === [], 'Ordinary potions omit full-HP hungry pets');
fixture(1, 140);
check(invoke('api_get_usable_pokemon')->data['usable_pokemon'] === [], 'Healthy full-HP pets remain absent from the picker');
foreach ([1, 2] as $quantity) {
    fixture(1, 100, 4, 'quality');
    DB::$stock['nums'] = $quantity;
    $response = invoke();
    $reroll = null;
    foreach (DB::$writes as $write) {
        if (strpos($write, 'SET hpg = ') !== false) $reroll = $write;
    }
    check($response->getCode() === 200 && $reroll !== null, 'Quality rerolls report success and persist new individual values');
    $stock_left = $quantity - 1;
    $stock_ok = $stock_left === 0 ? DB::$stock === false : (is_array(DB::$stock) && DB::$stock['nums'] === $stock_left);
    check($stock_ok, 'Quality rerolls consume one item');
    $ivs = [DB::$pet['hpg'], DB::$pet['atkg'], DB::$pet['defg'], DB::$pet['spatkg'], DB::$pet['spdefg'], DB::$pet['sdg']];
    check(min($ivs) >= 0 && max($ivs) <= 31, 'Rerolled individual values stay within 0-31');
    $rolled_pet = DB::$pet;
    $new_max = api_calculate_pokemon_max_hp($rolled_pet);
    check(DB::$pet['hp'] >= 0 && DB::$pet['hp'] <= $new_max, 'Current HP is clamped to the recalculated maximum');
}

echo "$passed hunger item assertions passed\n";
