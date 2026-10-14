<?php
/**
 * Exercise the out-of-battle PP restore flow (feedback tid=847433 page 31):
 * PP potions used to be rejected outside battle ("PP恢复道具请在战斗中对技能使用")
 * while the picker hid full-HP PP-depleted pets, and forgetting a skill requires
 * full PP — a three-way deadlock. The endpoint must now restore PP out of battle
 * and the Pokemon picker must list pets with depleted skill PP.
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/utils.php';
require __DIR__ . '/../../plugin/api/constants.php';
require __DIR__ . '/../../plugin/api/item_modules.php';

function load_pp_functions($path, $wanted)
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
load_pp_functions(__DIR__ . '/../../plugin/api/user.php', ['api_use_item', 'api_get_usable_pokemon']);
load_pp_functions(__DIR__ . '/../../plugin/api/index.php', ['pm_sql', 'pm_sql_v']);

class PpResponse extends RuntimeException
{
    public $data;
    public function __construct($code, $data)
    {
        parent::__construct(is_string($data) ? $data : 'success', $code);
        $this->data = $data;
    }
}
function api_success($data) { throw new PpResponse(200, $data); }
function api_error($message, $status = 400) { throw new PpResponse($status, $message); }
function require_login() {}
function pm_table($name) { return $name; }
function validate_uid($value) { return (int) $value; }
function validate_id($value, $name) { return (int) $value; }
function get_json_input() { return $GLOBALS['input']; }
function get_param($name, $default = null) { return $GLOBALS['input'][$name] ?? $default; }

class DB
{
    public static $npcid = 0, $item, $stock, $writes;
    /** @var array|null pm_mypm row */
    public static $pet;
    /** @var array pm_myskill JOIN pm_skill rows for pet 1 (skillnum < max_uses) */
    public static $depleted_skills;
    public static function fetch_first($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if ($sql === 'SELECT * FROM pm_usersdata WHERE uid = 7 FOR UPDATE') return ['uid' => 7, 'npcid' => self::$npcid];
        if ($sql === 'SELECT * FROM pm_itemdata WHERE id = 42') return self::$item;
        if ($sql === "SELECT * FROM pm_myitem WHERE uid = 7 AND itemid = '42'") return self::$stock;
        if ($sql === 'SELECT * FROM pm_mypm WHERE id = 1') return self::$pet;
        if ($sql === 'SELECT id, nickname FROM pm_mypm WHERE id = 1 AND uid = 7') return self::$pet && self::$pet['uid'] === 7 ? ['id' => 1, 'nickname' => self::$pet['nickname']] : false;
        if ($sql === 'SELECT name, hp FROM pm_data WHERE id = 1') return ['name' => 'Squirtle', 'hp' => 50];
        if ($sql === 'SELECT i.equipment FROM pm_myitem m LEFT JOIN pm_itemdata i ON m.itemid=i.id WHERE m.id=9') return ['equipment' => '{"hp":20}'];
        if ($sql === 'SELECT ms.id FROM pm_myskill ms LEFT JOIN pm_skill s ON ms.skillid = s.id WHERE ms.uid = 7 AND ms.petid = 1 AND ms.skillnum < s.max_uses LIMIT 1') {
            return empty(self::$depleted_skills) ? false : ['id' => self::$depleted_skills[0]['record_id']];
        }
        throw new RuntimeException('Unexpected read: ' . $sql);
    }
    public static function fetch_all($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (strpos($sql, 'SELECT ms.id AS record_id, ms.skillid, ms.skillnum, s.max_uses AS max_pp, s.name AS skill_name FROM pm_myskill ms') === 0) {
            return self::$depleted_skills;
        }
        if (preg_match('/^SELECT (.+) FROM pm_mypm WHERE uid = 7$/', $sql, $m)) {
            if (!self::$pet || self::$pet['uid'] !== 7) return [];
            return [array_intersect_key(self::$pet, array_flip(explode(', ', $m[1])))];
        }
        throw new RuntimeException('Unexpected list: ' . $sql);
    }
    public static function query($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (in_array($sql, ['START TRANSACTION', 'COMMIT', 'ROLLBACK'], true)) return;
        self::$writes[] = $sql;
        if (preg_match('/^UPDATE pm_myskill SET skillnum = (\d+) WHERE id = (\d+) AND uid = 7 AND petid = 1$/', $sql, $m)) {
            foreach (self::$depleted_skills as &$skill) {
                if ((int) $skill['record_id'] === (int) $m[2]) $skill['skillnum'] = (int) $m[1];
            }
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

function fixture($module, $skills, $withPokemon = true)
{
    $GLOBALS['input'] = $withPokemon ? ['item_id' => 42, 'pokemon_id' => 1] : ['item_id' => 42];
    DB::$npcid = 0;
    DB::$pet = ['id' => 1, 'uid' => 7, 'site' => 1, 'species_id' => 1, 'nickname' => 'Kami', 'hp' => 140, 'level' => 50, 'hpg' => 20, 'hpn' => 0, 'state' => 1, 'is_shiny' => 0, 'equipmentid1' => 9, 'equipmentid2' => 0, 'equipmentid3' => 0, 'equipmentid4' => 0];
    DB::$item = ['id' => 42, 'name' => 'PP单项小饮料', 'type' => 1, 'module' => $module, 'sitemname' => '', 'tpname' => '', 'effects' => '{"hp":0}'];
    DB::$stock = ['id' => 2, 'uid' => 7, 'itemid' => 42, 'nums' => 2];
    DB::$writes = [];
    DB::$depleted_skills = $skills;
}

function invoke($endpoint = 'api_use_item')
{
    try { $endpoint(); } catch (PpResponse $response) { return $response; }
    throw new RuntimeException('No response');
}

$passed = 0;
function check($condition, $message)
{
    if (!$condition) throw new RuntimeException($message);
    $GLOBALS['passed']++;
}

// Edbljl's live case: Bubble at 28/30, everything else full.
$depleted = [['record_id' => 11, 'skillid' => 145, 'skillnum' => 28, 'max_pp' => 30, 'skill_name' => '泡沫']];

// 1. Out-of-battle PP potion restores the depleted skill and consumes one item.
fixture('pp10', $depleted);
$response = invoke();
check($response->getCode() === 200, 'Out-of-battle PP potion succeeds');
check(DB::$depleted_skills[0]['skillnum'] === 30, 'pp10 tops the 28/30 skill up to 30');
check(DB::$stock['nums'] === 1, 'PP potion consumes one item');
check(strpos($response->data['message'], '泡沫') !== false && strpos($response->data['message'], '2 点PP') !== false, 'Message names the restored skill and amount');

// 2. Partial potions cap at the maximum instead of overflowing.
fixture('pp15', [['record_id' => 12, 'skillid' => 229, 'skillnum' => 36, 'max_pp' => 40, 'skill_name' => '高速旋转']]);
invoke();
check(DB::$depleted_skills[0]['skillnum'] === 40, 'Restore amount is capped at max PP');

// 3. The full-restore potion fills every depleted skill at once.
fixture('pp99', [
    ['record_id' => 11, 'skillid' => 145, 'skillnum' => 28, 'max_pp' => 30, 'skill_name' => '泡沫'],
    ['record_id' => 12, 'skillid' => 229, 'skillnum' => 36, 'max_pp' => 40, 'skill_name' => '高速旋转'],
]);
$response = invoke();
check($response->getCode() === 200, 'pp99 succeeds');
check(DB::$depleted_skills[0]['skillnum'] === 30 && DB::$depleted_skills[1]['skillnum'] === 40, 'pp99 fills all depleted skills');
check(DB::$stock['nums'] === 1, 'pp99 still consumes a single item');

// 4. Everything full: explicit failure without consuming the potion.
fixture('pp10', []);
$before = DB::$stock;
$response = invoke();
check($response->getCode() === 400 && strpos($response->getMessage(), 'PP都已满') !== false, 'Full-PP pet reports a clear error');
check(DB::$stock === $before && !DB::$writes, 'Full-PP pet consumes nothing');

// 5. PP potions still require choosing a Pokemon.
fixture('pp10', $depleted, false);
$response = invoke();
check($response->getCode() === 400 && strpos($response->getMessage(), '宝可梦') !== false, 'PP potion without a target is rejected');
check(!DB::$writes, 'No PP write happens without a target');

// 6. The battle guard still applies: battle-active pets must use the battle flow.
fixture('pp10', $depleted);
DB::$npcid = 5;
$response = invoke();
check($response->getCode() === 400 && strpos($response->getMessage(), '战斗') !== false, 'In-battle pet refuses out-of-battle potions');

// 7. Ownership is enforced before any PP write.
fixture('pp10', $depleted);
DB::$pet['uid'] = 8;
$response = invoke();
check($response->getCode() === 403, "Another trainer's pet is rejected");
check(!DB::$writes, "Another trainer's pet triggers no writes");

// 8. Picker: a full-HP pet with depleted skill PP becomes selectable for PP potions.
fixture('pp10', $depleted);
$response = invoke('api_get_usable_pokemon');
check(count($response->data['usable_pokemon']) === 1, 'Full-HP PP-depleted pet is selectable for PP potions');

// 9. Picker: everything-full pets stay hidden for PP potions.
fixture('pp10', []);
$response = invoke('api_get_usable_pokemon');
check($response->data['usable_pokemon'] === [], 'Full-HP full-PP pet stays hidden for PP potions');

// 10. Picker: ordinary potions keep ignoring healthy pets (no behavior change).
fixture('hp1', $depleted);
$response = invoke('api_get_usable_pokemon');
check($response->data['usable_pokemon'] === [], 'Ordinary potions still omit healthy full-HP pets');

echo "$passed PP restore assertions passed\n";
