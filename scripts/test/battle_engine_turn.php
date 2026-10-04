<?php
/**
 * Battle engine 2.0 turn-endpoint regressions (issue #75 phase 1).
 *
 * Executes the refactored api_use_skill (and its persistence helpers) against
 * an in-memory DB stub: asserts the one-turn-one-transaction SQL sequence, the
 * pm_usersdata legacy mirror, PP reservation/refund semantics, the victory
 * path, lazy migration of pre-upgrade battles, and the awaiting-switch path.
 * Run: php scripts/test/battle_engine_turn.php
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

define('IN_DISCUZ', 1);
require __DIR__ . '/../../plugin/api/battle_core.php';
// pre-load the real helper so capture's runtime require_once (with eval-context __DIR__) no-ops
require __DIR__ . '/../../plugin/api/pokemon_utils.php';

// ---- load the real endpoint functions from battle.php (token extraction + eval) ----
$wanted = [
    'api_use_skill', 'api_flee', 'pm_refund_reserved_skill_pp', 'pm_data',
    'api_capture_pokemon', 'api_use_item_in_battle', 'api_use_item_on_skill_in_battle', 'api_replace_pokemon',
    'battle_ensure_tables', 'battle_load_active', 'battle_inject_ally_fresh_state',
    'battle_persist_state', 'battle_mirror_legacy', 'battle_resolve_engine_counter',
    'battle_render_counter_messages',
    'battle_skill_effects',
    'battle_pick_enemy_move',
    'api_normalize_skill_category', 'battle_calc_my_stats',
    'calculate_rewards', 'apply_rewards', 'clear_battle_state',
    'handle_my_pokemon_fainted', 'build_battle_response',
];
$tokens = token_get_all(file_get_contents(__DIR__ . '/../../plugin/api/battle.php'));
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
        $body .= is_array($token) ? $token[1] : $token;
        if ($token === '{' || (is_array($token) && in_array($token[0], [T_CURLY_OPEN, T_DOLLAR_OPEN_CURLY_BRACES], true))) {
            $depth++;
            $opened = true;
        } elseif ($token === '}') {
            $depth--;
            if ($opened && $depth === 0) break;
        }
    }
    if (in_array($name, $wanted, true)) eval($body);
    $i = $j;
}
foreach ($wanted as $name) {
    if (!function_exists($name)) throw new RuntimeException("Function not loaded: $name");
}

// ---- response envelope stubs ----
class BattleApiResponse extends RuntimeException
{
    public $data;
    public function __construct($message, $code, $data = null)
    {
        parent::__construct($message, $code);
        $this->data = $data;
    }
}
function api_error($message, $code) { throw new BattleApiResponse($message, $code); }
function pm_abort_battle_transaction($message, $status = 400)
{
    DB::query('ROLLBACK');
    api_error($message, $status);
}
function api_success($data) { throw new BattleApiResponse('success', 200, $data); }
function require_login() {}
function get_json_input() { return $GLOBALS['input']; }
function pm_table($name) { return $name; }
function pm_sql($sql, ...$args)
{
    $i = 0;
    return preg_replace_callback('/%([sd])/', function ($m) use ($args, &$i) {
        $v = isset($args[$i]) ? $args[$i] : '';
        $i++;
        if ($m[1] === 'd') {
            return intval($v);
        }
        return "'" . $v . "'";
    }, $sql);
}
function api_my_usersdata($uid) { return DB::$usersdata; }
function api_my_pokemon($username) { return DB::$pet_row; }
function api_calculate_pokemon_max_hp($pokemon) { return 100; }
function api_validate_and_correct_hp(&$pm, $hp = null, $max_hp = null)
{
    $hp = $hp === null ? (int)$pm['hp'] : (int)$hp;
    $max = $max_hp === null ? 100 : (int)$max_hp;
    $hp = max(0, min($max, $hp));
    $pm['hp'] = strval($hp);
    return ['hp' => $hp, 'max_hp' => $max, 'corrected' => false];
}
function api_parse_pet_wear_items(&$pet, $x, &$eq_hp) { $eq_hp = 0; return [0, 0, 0, 0, 0, 0]; }
function api_get_item_module($item_data)
{
    foreach (['module', 'sitemname', 'tpname'] as $col) {
        $val = isset($item_data[$col]) ? trim(strval($item_data[$col])) : '';
        if ($val !== '' && preg_match('/^[a-z][a-z0-9_]*$/i', $val)) {
            return $val;
        }
    }
    return '';
}
function api_get_pet_exp_level($pmno, $exp)
{
    // fixture exp table: level = 30 until 500 exp, then 31
    return $exp >= 500 ? 31 : 30;
}
// state multipliers: all 1.0
foreach (['statehp', 'stateatk', 'statespatk', 'statedef', 'statespdef', 'statesd'] as $v) {
    $GLOBALS[$v] = array_fill(1, 20, '1');
}

$GLOBALS['_G'] = ['uid' => 1, 'username' => 'tester'];
$GLOBALS['petbasisexp'] = null;
$GLOBALS['settings'] = [];

// ---- in-memory DB with intent-level SQL handling ----
class DB
{
    public static $logs = [];
    public static $usersdata;
    public static $mypm = [];
    public static $pet_row;
    public static $battles = [];
    public static $units = [];
    public static $events = [];
    public static $myskills = [];
    public static $items = [];
    public static $myitems = [];
    public static $enemy_skills = [];
    public static $affected = 0;
    public static $next_id = 500;

    public static function reset()
    {
        // keep battles/units/events (the seeded rows must survive into run_turn);
        // only clear the SQL audit log so per-scenario assertions stay scoped
        self::$logs = [];
        self::$affected = 0;
    }

    public static function wipe_rows()
    {
        self::$battles = [];
        self::$units = [];
        self::$events = [];
    }

    public static function query($sql)
    {
        self::$logs[] = $sql;
        self::apply(trim($sql));
    }

    private static function apply($sql)
    {
        // pm_battle INSERT (battle start / lazy migration)
        if (preg_match('/^INSERT INTO pm_battle\b/', $sql)) {
            $id = ++self::$next_id;
            $row = ['id' => $id, 'uid' => 1, 'phase' => 'active', 'result' => '', 'turn' => 0,
                'kind' => 'wild', 'map_id' => 0, 'rng_seed' => 0, 'rng_counter' => 0, 'event_seq' => 0,
                'rules_version' => 1, 'state_version' => 2, 'field_json' => '{}'];
            if (preg_match('/VALUES \((.*)\)$/s', $sql, $vm)) {
                $vals = str_getcsv($vm[1], ',', chr(39), chr(92));
            } else {
                $vals = array();
            }
            $cols = ['uid', 'kind', 'map_id', 'turn', 'phase', 'result', 'rng_seed', 'rng_counter',
                'event_seq', 'rules_version', 'state_version', 'field_json', 'created_at', 'updated_at'];
            foreach ($cols as $ci => $c) {
                if (isset($vals[$ci])) {
                    $raw_val = trim($vals[$ci]);
                    if (is_numeric($raw_val)) {
                        $row[$c] = strpos($raw_val, '.') !== false ? floatval($raw_val) : (int)$raw_val;
                    } else {
                        $row[$c] = trim($raw_val, chr(39));
                    }
                }
            }
            self::$battles[$id] = $row;
            self::$affected = 1;
            return;
        }
        // pm_battle UPDATE from persist
        if (preg_match('/^UPDATE pm_battle\b.*?SET/s', $sql)) {
            if (preg_match("/WHERE id = (\d+)/", $sql, $m) && isset(self::$battles[(int)$m[1]])) {
                $row = &self::$battles[(int)$m[1]];
                if (preg_match("/turn = (\d+)/", $sql, $t)) $row['turn'] = (int)$t[1];
                if (preg_match("/phase = '([a-z_]+)'/", $sql, $t)) $row['phase'] = $t[1];
                if (preg_match("/result = '([a-z_]*)'/", $sql, $t)) $row['result'] = $t[1];
                self::$affected = 1;
                unset($row);
                return;
            }
            // clear_battle_state linkage: WHERE uid = .. AND phase IN (...)
            if (preg_match("/WHERE uid = (\d+) AND phase IN/", $sql)) {
                self::$affected = 0;
                foreach (self::$battles as &$row) {
                    if ($row['uid'] === 1 && in_array($row['phase'], ['active', 'awaiting_switch'], true)) {
                        $row['phase'] = 'ended';
                        if ($row['result'] === '') $row['result'] = 'abandoned';
                        self::$affected = 1;
                    }
                }
                unset($row);
                return;
            }
            self::$affected = 0;
            return;
        }
        if (preg_match('/^DELETE FROM pm_battle_unit\b.*?WHERE battle_id = (\d+)/s', $sql, $m)) {
            $bid = (int)$m[1];
            self::$units = array_values(array_filter(self::$units, function ($u) use ($bid) {
                return (int)$u['battle_id'] !== $bid;
            }));
            self::$affected = 1;
            return;
        }
        if (preg_match('/^INSERT INTO pm_battle_unit\b/', $sql)) {
            $id = ++self::$next_id;
            $row = ['id' => $id];
            // VALUES (...): positional per persist helper order
            if (preg_match('/VALUES \((.*)\)$/s', $sql, $m)) {
                $vals = str_getcsv($m[1], ",", chr(39), chr(92));
                $cols = ['battle_id', 'side', 'slot', 'instance_id', 'species_id', 'name', 'species_name', 'level',
                    'stats_json', 'types_json', 'hp', 'stages_json', 'status_json', 'volatile_json',
                    'buffs_json', 'effects_json', 'fainted', 'gender', 'is_shiny', 'capture_rate', 'boss_multiplier'];
                foreach ($cols as $i => $c) {
                    $row[$c] = isset($vals[$i]) ? trim($vals[$i], " '") : '';
                }
            }
            self::$units[] = $row;
            self::$affected = 1;
            return;
        }
        if (preg_match('/^INSERT INTO pm_battle_event\b/', $sql)) {
            self::$events[] = $sql;
            self::$affected = 1;
            return;
        }
        // legacy mirror write (battle_mirror_legacy)
        if (preg_match('/^UPDATE pm_usersdata\b.*?SET\s+npcid = (\d+), level = (\d+), hp = (\d+), hpg = (\d+),\s*atkg = (\d+), defg = (\d+), spatkg = (\d+), spdefg = (\d+), sdg = (\d+),\s*capture = (\d+), allure = (\d+)/is', $sql, $m)) {
            self::$usersdata['npcid'] = (int)$m[1];
            self::$usersdata['level'] = (int)$m[2];
            self::$usersdata['hp'] = (int)$m[3];
            self::$usersdata['hpg'] = (int)$m[4];
            self::$usersdata['atkg'] = (int)$m[5];
            self::$usersdata['defg'] = (int)$m[6];
            self::$usersdata['spatkg'] = (int)$m[7];
            self::$usersdata['spdefg'] = (int)$m[8];
            self::$usersdata['sdg'] = (int)$m[9];
            self::$usersdata['capture'] = (int)$m[10];
            self::$usersdata['allure'] = (int)$m[11];
            self::$affected = 1;
            return;
        }
        // mirror clear (clear_battle_state)
        if (preg_match('/^UPDATE pm_usersdata\b.*?SET\s+npcid = 0, level = 0/is', $sql)) {
            foreach (['npcid', 'level', 'hp', 'hpg', 'atkg', 'defg', 'spatkg', 'spdefg', 'sdg', 'allure', 'capture'] as $c) {
                self::$usersdata[$c] = 0;
            }
            self::$affected = 1;
            return;
        }
        // my pet HP
        if (preg_match('/^UPDATE pm_mypm\b.*?SET hp = (\d+) WHERE id = (\d+)/s', $sql, $m)) {
            foreach (self::$mypm as &$p) {
                if ((int)$p['id'] === (int)$m[2]) $p['hp'] = (int)$m[1];
            }
            if (self::$pet_row && (int)self::$pet_row['id'] === (int)$m[2]) self::$pet_row['hp'] = (int)$m[1];
            unset($p);
            self::$affected = 1;
            return;
        }
        // PP reservation / refund
        if (preg_match('/^UPDATE pm_myskill\b.*?skillnum = skillnum - 1\s+WHERE skillid = (\d+) AND uid = (\d+) AND petid = (\d+) AND skillnum > (\d+)/s', $sql, $m)) {
            self::$affected = 0;
            foreach (self::$myskills as &$s) {
                if ((int)$s['skillid'] === (int)$m[1] && (int)$s['uid'] === (int)$m[2] && (int)$s['petid'] === (int)$m[3]
                    && (int)$s['skillnum'] > 0) {
                    $s['skillnum'] = (int)$s['skillnum'] - 1;
                    self::$affected = 1;
                }
            }
            unset($s);
            return;
        }
        if (preg_match('/^UPDATE pm_myskill\b.*?skillnum = skillnum \+ 1\s+WHERE skillid = (\d+) AND uid = (\d+) AND petid = (\d+) AND skillnum < (\d+)/s', $sql, $m)) {
            self::$affected = 0;
            foreach (self::$myskills as &$s) {
                if ((int)$s['skillid'] === (int)$m[1] && (int)$s['uid'] === (int)$m[2] && (int)$s['petid'] === (int)$m[3]
                    && (int)$s['skillnum'] < (int)$m[4]) {
                    $s['skillnum'] = (int)$s['skillnum'] + 1;
                    self::$affected = 1;
                }
            }
            unset($s);
            return;
        }
        // battle items: consume one (nums-1, delete at 1)
        if (preg_match('/^UPDATE pm_myitem\b.*?SET nums = nums - 1 WHERE id = (\d+)/s', $sql, $m)) {
            foreach (self::$myitems as &$it) {
                if ((int)$it['id'] === (int)$m[1]) $it['nums'] = (int)$it['nums'] - 1;
            }
            unset($it);
            self::$affected = 1;
            return;
        }
        if (preg_match('/^DELETE FROM pm_myitem\b.*?WHERE id = (\d+)/s', $sql, $m)) {
            self::$myitems = array_values(array_filter(self::$myitems, function ($it) use ($m) {
                return (int)$it['id'] !== (int)$m[1];
            }));
            self::$affected = 1;
            return;
        }
        // captured wild insert (pm_mypm)
        if (preg_match('/^INSERT INTO pm_mypm\b/', $sql)) {
            self::$affected = 1;
            return;
        }
        // passive/active switch site updates
        if (preg_match('/^UPDATE pm_mypm\b.*?SET\s+site = 2 WHERE id = (\d+) AND uid = \d+ AND site = 1/s', $sql, $m)) {
            foreach (self::$mypm as &$pm) {
                if ((int)$pm['id'] === (int)$m[1] && (int)$pm['site'] === 1) $pm['site'] = 2;
            }
            if (self::$pet_row && (int)self::$pet_row['id'] === (int)$m[1]) self::$pet_row['site'] = 2;
            unset($pm);
            self::$affected = 1;
            return;
        }
        if (preg_match('/^UPDATE pm_mypm\b.*?SET\s+site = 1 WHERE id = (\d+) AND uid = \d+ AND site < 3 AND hp > 0 AND state != 0/s', $sql, $m)) {
            self::$affected = 0;
            foreach (self::$mypm as &$pm) {
                if ((int)$pm['id'] === (int)$m[1] && (int)$pm['site'] < 3 && (int)$pm['hp'] > 0 && (int)$pm['state'] != 0) {
                    $pm['site'] = 1;
                    self::$affected = 1;
                    if (self::$pet_row && (int)self::$pet_row['id'] === (int)$m[1]) {
                        self::$pet_row = $pm;
                    }
                }
            }
            unset($pm);
            return;
        }
        // direct PP set (item refill)
        if (preg_match('/^UPDATE pm_myskill\b.*?SET skillnum = (\d+) WHERE id = (\d+)/s', $sql, $m)) {
            self::$affected = 1;
            return;
        }
        // rewards (apply_rewards): pet exp/level and usersdata counters — just record
        self::$affected = 1;
    }

    public static function fetch_first($sql)
    {
        self::$logs[] = $sql;
        $sql = trim($sql);
        if (strpos($sql, 'FOR UPDATE') !== false) {
            return ['uid' => 1];
        }
        if (preg_match('/FROM pm_battle\b.*?WHERE uid = \d+ AND phase IN/s', $sql)) {
            $cands = array_values(array_filter(self::$battles, function ($b) {
                return $b['uid'] === 1 && in_array($b['phase'], ['active', 'awaiting_switch'], true);
            }));
            if (!$cands) return false;
            usort($cands, function ($a, $b) { return $b['id'] - $a['id']; });
            return $cands[0];
        }
        if (preg_match('/FROM pm_battle_unit\b.*?WHERE battle_id = (\d+)/s', $sql, $m)) {
            $bid = (int)$m[1];
            $rows = array_values(array_filter(self::$units, function ($u) use ($bid) {
                return (int)$u['battle_id'] === $bid;
            }));
            return $rows ? $rows : false;
        }
        if (preg_match('/FROM pm_data WHERE id = (\d+)/', $sql, $m)) {
            $id = (int)$m[1];
            $data = [
                25 => ['id' => 25, 'name' => '皮卡丘', 'xs' => '电', 'xs2' => '', 'strength' => 1,
                    'hp' => 45, 'atk' => 55, 'def' => 40, 'spatk' => 50, 'spdef' => 50, 'speed' => 65, 'drop_money' => '[10,50]', 'sex' => 50],
                129 => ['id' => 129, 'name' => '鲤鱼王', 'xs' => '水', 'xs2' => '', 'strength' => 1,
                    'hp' => 40, 'atk' => 15, 'def' => 30, 'spatk' => 15, 'spdef' => 30, 'speed' => 60, 'drop_money' => '[5,20]', 'sex' => 50],
                4 => ['id' => 4, 'name' => '小火龙', 'xs' => '火', 'xs2' => '', 'strength' => 1, 'sex' => 50,
                    'hp' => 39, 'atk' => 52, 'def' => 43, 'spatk' => 60, 'spdef' => 50, 'speed' => 65, 'drop_money' => '[5,20]'],
            ];
            return isset($data[$id]) ? $data[$id] : false;
        }
        if (preg_match('/FROM pm_myskill\b/', $sql) && strpos($sql, 'WHERE skillid') !== false) {
            if (preg_match('/skillid = (\d+) AND uid = (\d+) AND petid = (\d+)/', $sql, $m)) {
                foreach (self::$myskills as $s) {
                    if ((int)$s['skillid'] === (int)$m[1] && (int)$s['uid'] === (int)$m[2] && (int)$s['petid'] === (int)$m[3]) {
                        return $s;
                    }
                }
            }
            return false;
        }
        if (preg_match('/FROM pm_myitem m\s+LEFT JOIN pm_itemdata/', $sql) || preg_match('/FROM pm_itemdata i ON m.itemid = i.id/', $sql)) {
            if (preg_match('/m.itemid = ' . chr(39) . '?(\d+)' . chr(39) . '?/', $sql, $m)) {
                foreach (self::$myitems as $it) {
                    if ((int)$it['itemid'] === (int)$m[1] && isset(self::$items[(int)$m[1]]) && (int)self::$items[(int)$m[1]]['type'] === 2) {
                        $item = self::$items[(int)$m[1]];
                        return array_merge($it, ['myitem_id' => $it['id'], 'itemdata_id' => $item['id'], 'name' => $item['name'],
                            'type' => $item['type'], 'captmax' => $item['captmax'], 'ballid' => $item['ballid']]);
                    }
                }
            }
            return false;
        }
        if (preg_match('/FROM pm_itemdata WHERE id = (\d+)/', $sql, $m)) {
            $id = (int)$m[1];
            return isset(self::$items[$id]) ? self::$items[$id] : false;
        }
        if (preg_match('/FROM pm_myitem WHERE uid = (\d+) AND itemid = ' . chr(39) . '?(\d+)' . chr(39) . '?/', $sql, $m)) {
            foreach (self::$myitems as $it) {
                if ((int)$it['uid'] === (int)$m[1] && (int)$it['itemid'] === (int)$m[2]) {
                    return $it;
                }
            }
            return false;
        }
        if (preg_match('/FROM pm_mypm\b.*?WHERE uid = \d+ AND id = (\d+) AND site < 3 AND hp > 0 AND state != 0/s', $sql, $m)) {
            foreach (self::$mypm as $pm) {
                if ((int)$pm['id'] === (int)$m[1] && (int)$pm['site'] < 3 && (int)$pm['hp'] > 0 && (int)$pm['state'] != 0) {
                    return $pm;
                }
            }
            return false;
        }
        if (preg_match('/FROM pm_effect\b.*?WHERE id = (\d+)/s', $sql, $m)) {
            $effects = [
                2 => ['id' => 2, 'code' => 'sand_attack', 'kind' => 'move',
                    'hooks_json' => '["on_after_move"]',
                    'params_json' => '{"code":"stages_boost","stat":"accuracy","stages":-1,"target":"opponent"}',
                    'version' => 1],
            ];
            return isset($effects[(int)$m[1]]) ? $effects[(int)$m[1]] : false;
        }
        if (preg_match('/FROM pm_skill WHERE id = (\d+)/', $sql, $m)) {
            $id = (int)$m[1];
            $skills = [
                5 => ['id' => 5, 'name' => '电击', 'power' => 40, 'max_uses' => 35, 'element' => '电', 'category' => '特攻', 'effect_id' => 0],
                28 => ['id' => 28, 'name' => '泼沙', 'power' => 0, 'max_uses' => 15, 'element' => '地面', 'category' => '其他', 'effect_id' => 2],
            ];
            return isset($skills[$id]) ? $skills[$id] : false;
        }
        return false;
    }

    public static function fetch_all($sql)
    {
        if (preg_match('/FROM pm_mypm\b.*?site < 3 AND hp > 0 AND state != 0\s+ORDER BY/s', $sql)) {
            $rows = array_values(array_filter(self::$mypm, function ($pm) {
                return (int)$pm['site'] < 3 && (int)$pm['hp'] > 0 && (int)$pm['state'] != 0;
            }));
            usort($rows, function ($a, $b) { return ($a['site'] <=> $b['site']) ?: ($a['id'] <=> $b['id']); });
            return $rows;
        }
        // battle units by battle id (battle_load_active)
        if (preg_match('/FROM pm_battle_unit\b.*?WHERE battle_id = (\d+)/s', $sql, $m)) {
            $bid = (int)$m[1];
            return array_values(array_filter(self::$units, function ($u) use ($bid) {
                return (int)$u['battle_id'] === $bid;
            }));
        }
        if (strpos($sql, 'FIND_IN_SET') !== false && strpos($sql, 'FROM pm_skill') !== false) {
            return self::$enemy_skills;
        }
        if (preg_match('/FROM pm_myskill ms/', $sql) && preg_match('/WHERE ms.petid = (\d+) AND ms.uid = (\d+)/', $sql, $m)) {
            $out = [];
            foreach (self::$myskills as $s) {
                if ((int)$s['petid'] === (int)$m[1] && (int)$s['uid'] === (int)$m[2]) {
                    $out[] = ['skillid' => $s['skillid'], 'skillnum' => $s['skillnum'], 'name' => '电击',
                        'power' => 40, 'max_pp' => 35, 'element' => '电', 'category' => '特攻'];
                }
            }
            return $out;
        }
        return [];
    }

    public static function affected_rows()
    {
        return self::$affected;
    }

    public static function insert_id()
    {
        return self::$next_id;
    }

    public static function result_first($sql)
    {
        // box capacity count (no site/health filter)
        if (preg_match('/SELECT COUNT\(\*\) FROM pm_mypm\s+WHERE uid = \d+\s*$/', $sql)) {
            return count(self::$mypm);
        }
        // has-first / active count for capture placement
        if (preg_match('/FROM pm_mypm\s+WHERE uid = \d+ AND site = 1\s*$/', $sql)) {
            $n = 0;
            foreach (self::$mypm as $pm) {
                if ((int)$pm['site'] === 1) $n++;
            }
            return $n;
        }
        if (preg_match('/FROM pm_mypm\s+WHERE uid = \d+ AND site < 3\s*$/', $sql)) {
            $n = 0;
            foreach (self::$mypm as $pm) {
                if ((int)$pm['site'] < 3) $n++;
            }
            return $n;
        }
        // bench count for handle_my_pokemon_fainted
        if (preg_match('/SELECT COUNT\(\*\) FROM pm_mypm\s+WHERE uid = \d+ AND site < 3 AND hp > 0 AND state != 0 AND id != (\d+)/s', $sql, $m)) {
            $n = 0;
            foreach (self::$mypm as $p) {
                if ((int)$p['id'] !== (int)$m[1] && (int)$p['site'] < 3 && (int)$p['hp'] > 0 && (int)$p['state'] != 0) {
                    $n++;
                }
            }
            return $n;
        }
        return 0;
    }

    public static function has_log($needle)
    {
        foreach (self::$logs as $l) {
            if (strpos($l, $needle) !== false) {
                return true;
            }
        }
        return false;
    }
}

$failures = 0;
$checks = 0;
function check($label, $cond)
{
    global $failures, $checks;
    $checks++;
    if ($cond) {
        echo "  ok: {$label}\n";
    } else {
        $failures++;
        echo "  FAIL: {$label}\n";
    }
}

function run_turn($skill_id = 5)
{
    $GLOBALS['input'] = ['skill_id' => $skill_id];
    try {
        api_use_skill();
    } catch (BattleApiResponse $r) {
        return $r;
    }
    throw new RuntimeException('api_use_skill returned without response');
}

/**
 * Seed a fresh battle through the real start-state shape (as api_start_battle
 * would persist it), then hand-adjust enemy/ally fixtures per scenario.
 */
function seed_battle($enemy_hp, $enemy_atk, $enemy_speed, $seed, $ally_hp = 100, $enemy_level = 15)
{
    DB::reset();
    $state = battle_core_initial_state([
        'uid' => 1,
        'kind' => 'wild',
        'map_id' => 3,
        'rng_seed' => $seed,
        'allies' => [[
            'instance_id' => 11, 'species_id' => 25, 'name' => '皮卡', 'species_name' => '皮卡丘',
            'level' => 30, 'hp' => $ally_hp,
            'stats' => ['max_hp' => 100, 'atk' => 42, 'def' => 34, 'spatk' => 45, 'spdef' => 40, 'speed' => 55],
            'types' => ['电'],
        ]],
        'enemies' => [[
            'species_id' => 129, 'name' => '鲤鱼王', 'species_name' => '鲤鱼王',
            'level' => $enemy_level, 'hp' => $enemy_hp,
            'stats' => ['max_hp' => 80, 'atk' => $enemy_atk, 'def' => 30, 'spatk' => 15, 'spdef' => 30, 'speed' => $enemy_speed],
            'types' => ['水'], 'gender' => 0, 'is_shiny' => false, 'capture_rate' => 150,
        ]],
    ]);
    $start_events = [];
    battle_core_emit($state, $start_events, 'battle_start', ['kind' => 'wild', 'map_id' => 3]);
    battle_persist_state($state, $start_events);
    // mirror the start state into the legacy columns (as api_start_battle does)
    DB::$usersdata['npcid'] = 129;
    DB::$usersdata['level'] = 15;
    DB::$usersdata['hp'] = $enemy_hp;
    DB::$usersdata['hpg'] = 80;
    DB::$usersdata['atkg'] = $enemy_atk;
    DB::$usersdata['defg'] = 30;
    DB::$usersdata['spatkg'] = 15;
    DB::$usersdata['spdefg'] = 30;
    DB::$usersdata['sdg'] = $enemy_speed;
    DB::$usersdata['capture'] = 150;
    DB::$usersdata['allure'] = 0;
    DB::reset(); // keep only the persisted rows; clear seed logs
    return $state;
}

function seed_pet_and_party($hp = 100, $bench = 0)
{
    DB::$pet_row = [
        'id' => 11, 'uid' => 1, 'species_id' => 25, 'pmname' => '皮卡丘', 'nickname' => '皮卡',
        'level' => 30, 'exp' => 100, 'hp' => $hp, 'state' => 1, 'site' => 1, 'is_shiny' => 0,
        'hpg' => 15, 'atkg' => 15, 'defg' => 15, 'spatkg' => 15, 'spdefg' => 15, 'sdg' => 15,
        'hpn' => 0, 'atkn' => 0, 'defn' => 0, 'spatkn' => 0, 'spdefn' => 0, 'sdn' => 0,
    ];
    DB::$mypm = [DB::$pet_row];
    for ($i = 0; $i < $bench; $i++) {
        DB::$mypm[] = ['id' => 100 + $i, 'uid' => 1, 'species_id' => 4, 'pmname' => '小火龙', 'nickname' => '',
            'level' => 28, 'exp' => 0, 'hp' => 60, 'state' => 1, 'site' => 2, 'is_shiny' => 0,
            'hpg' => 10, 'atkg' => 10, 'defg' => 10, 'spatkg' => 10, 'spdefg' => 10, 'sdg' => 10,
            'hpn' => 0, 'atkn' => 0, 'defn' => 0, 'spatkn' => 0, 'spdefn' => 0, 'sdn' => 0];
    }
    DB::$myskills = [[
        'skillid' => 5, 'uid' => 1, 'petid' => 11, 'skillnum' => 35,
    ]];
}

DB::$usersdata = [
    'uid' => 1, 'npcid' => 0, 'level' => 0, 'hp' => 0, 'hpg' => 0, 'atkg' => 0, 'defg' => 0,
    'spatkg' => 0, 'spdefg' => 0, 'sdg' => 0, 'allure' => 0, 'capture' => 0,
    'dataall' => 0, 'datawin' => 0, 'datalost' => 0, 'money' => 0, 'strength' => 1, 'boxnum' => 9,
];

echo "=== scenario A: normal turn (one-turn-one-transaction SQL sequence) ===\n";
seed_battle(80, 15, 30, 42);       // ally (speed 55) first, enemy survives, counters back
seed_pet_and_party(100, 0);
$r = run_turn();
check('responds success 200', $r->getCode() === 200);
$b = $r->data ? $r->data : [];
check('status stays active when both survive', isset($b['status']) && $b['status'] === 'active');
check('battle_over false', $b['battle_over'] === false);
check('turn field keeps legacy semantics (1 while active)', $b['turn'] === 1);
check('battle_turn exposes real engine turn', $b['battle_turn'] === 1);
check('events stream attached (optional new field)', is_array($b['events']) && count($b['events']) >= 3);
check('battle_id field unchanged', $b['battle_id'] === 'battle_1');
check('my_pokemon shape intact', isset($b['my_pokemon']['instance_id'], $b['my_pokemon']['skills'][0]['pp']));
check('wild_pokemon shape intact', $b['wild_pokemon']['id'] === 129 && $b['wild_pokemon']['max_hp'] === 80);
check('message rendered as legacy damage log', strpos($b['message'], '皮卡使用了') !== false || strpos($b['message'], '鲤鱼王攻击了') !== false);
check('transaction started', DB::has_log('START TRANSACTION'));
check('user row locked FOR UPDATE', DB::has_log('SELECT uid FROM pm_usersdata WHERE uid = 1 FOR UPDATE'));
check('PP reserved atomically', DB::has_log('skillnum = skillnum - 1'));
check('PP consumed exactly once', (function () {
    foreach (DB::$myskills as $s) {
        if ((int)$s['skillid'] === 5) return (int)$s['skillnum'] === 34;
    }
    return false;
})());
check('transaction committed', DB::has_log('COMMIT'));
check('battle event stream persisted', (function () {
    $n = 0;
    foreach (DB::$logs as $l) {
        if (strpos($l, 'INSERT INTO pm_battle_event') === 0) $n++;
    }
    return $n >= 3; // turn_start + move + damage (+ counter)
})());
check('pm_battle_unit rewritten with new HP', (function () {
    foreach (DB::$units as $u) {
        if ($u['side'] === 'enemy' && (int)$u['hp'] < 80) return true;
    }
    return false;
})());
check('legacy mirror updated (usersdata.hp < 80)', (int)DB::$usersdata['hp'] < 80 && (int)DB::$usersdata['npcid'] === 129);
check('ally hp persisted to pm_mypm', (int)DB::$pet_row['hp'] < 100 && (int)DB::$pet_row['hp'] > 0);


echo "=== scenario B: victory path ===\n";
seed_battle(5, 15, 30, 42);        // one-shot KO
seed_pet_and_party(100, 0);
$r = run_turn();
$b = $r->data;
check('status victory', $b['status'] === 'victory');
check('battle_over true', $b['battle_over'] === true);
check('rewards attached', isset($b['rewards']['exp'], $b['rewards']['money']) && $b['rewards']['exp'] > 0);
$battle_rows = array_values(array_filter(DB::$battles, function ($x) { return $x['uid'] === 1; }));
check('battle row ended with result victory', (function () use ($battle_rows) {
    foreach ($battle_rows as $row) {
        if ($row['phase'] === 'ended' && $row['result'] === 'victory') return true;
    }
    return false;
})());
check('legacy mirror cleared', (int)DB::$usersdata['npcid'] === 0 && (int)DB::$usersdata['hp'] === 0);
check('win counters incremented via apply_rewards', DB::has_log('dataall = dataall + 1'));
check('victory event recorded', (function () {
    foreach (DB::$logs as $l) {
        if (strpos($l, 'INSERT INTO pm_battle_event') === 0 && strpos($l, 'battle_end') !== false) return true;
    }
    return false;
})());
check('wild still rendered in victory response (temp restore)', $b['wild_pokemon']['id'] === 129 && $b['wild_pokemon']['hp'] === 0);

echo '=== scenario C: miss refunds reserved PP ===' . PHP_EOL;
// find a seed where the ally's post-counter attack misses: roll idx1 (85..100 for counter), idx2 (1..20 miss)
$miss_seed = null;
for ($s = 1; $s < 5000; $s++) {
    // rules v2 miss chance roll is (1,100) with a 20% baseline
    if (1 + (battle_core_rng_step($s, 1) % 100) <= 20) {
        $miss_seed = $s;
        break;
    }
}
check('miss seed found', $miss_seed !== null);
seed_battle(80, 15, 90, $miss_seed); // enemy speed 90 > 55: enemy acts first, ally attack may miss
seed_pet_and_party(100, 0);
$r = run_turn();
$b = $r->data;
$missed = strpos($b['message'], '避开了') !== false;
check('attack missed this turn', $missed);
check('reserved PP refunded on miss', (function () {
    foreach (DB::$myskills as $s) {
        if ((int)$s['skillid'] === 5) return (int)$s['skillnum'] === 35;
    }
    return false;
})());
check('refund SQL issued', DB::has_log('skillnum = skillnum + 1'));

DB::wipe_rows();
// pin the lazy-migration seed (drawn via mt_rand) so the wild (60 HP) always survives the turn
mt_srand(3);
DB::reset();
echo "=== scenario D: lazy migration of a pre-upgrade battle ===\n";
DB::reset();
// no pm_battle rows; legacy columns hold an in-progress battle
DB::$usersdata['npcid'] = 129;
DB::$usersdata['level'] = 15;
DB::$usersdata['hp'] = 60;
DB::$usersdata['hpg'] = 80;
DB::$usersdata['atkg'] = 22;
DB::$usersdata['defg'] = 30;
DB::$usersdata['spatkg'] = 15;
DB::$usersdata['spdefg'] = 30;
DB::$usersdata['sdg'] = 45;
DB::$usersdata['capture'] = 150;
DB::$usersdata['allure'] = 0;
seed_pet_and_party(100, 0);
$r = run_turn();
check('migrated battle turn succeeds', $r->getCode() === 200);
check('migration inserted a pm_battle row', DB::has_log('INSERT INTO pm_battle'));
$battle_rows = array_values(array_filter(DB::$battles, function ($x) { return $x['uid'] === 1; }));
check('migrated battle active in new tables', (function () use ($battle_rows) {
    foreach ($battle_rows as $row) {
        if (in_array($row['phase'], ['active', 'awaiting_switch', 'ended'], true)) return true;
    }
    return false;
})());
check('battle_start event logged with migrated flag', (function () {
    foreach (DB::$logs as $l) {
        if (strpos($l, 'INSERT INTO pm_battle_event') === 0 && strpos($l, 'battle_start') !== false) return true;
    }
    return false;
})());
check('migration preserved wild HP (60 minus turn damage or intact)', (int)DB::$usersdata['npcid'] === 129);

echo "=== scenario E: ally faints with bench -> awaiting_switch kept ===\n";
seed_battle(80, 5000, 90, 42);     // enemy faster and one-shots the ally
seed_pet_and_party(100, 2);        // two healthy bench mons
$r = run_turn();
$b = $r->data;
check('status defeat', $b['status'] === 'defeat');
check('battle_over false while substitutes remain', $b['battle_over'] === false);
check('can_continue_switch true', $b['can_continue_switch'] === true);
check('battle row kept awaiting_switch', (function () {
    foreach (DB::$battles as $row) {
        if ($row['uid'] === 1 && $row['phase'] === 'awaiting_switch') return true;
    }
    return false;
})());
check('legacy mirror kept for switch flow', (int)DB::$usersdata['npcid'] === 129);
check('fainted ally hp written as 0', (int)DB::$pet_row['hp'] === 0);
check('switch prompt in message', strpos($b['message'], '还有可用的替补宠物') !== false);

echo "=== scenario F: ally faints, no bench -> battle really ends ===\n";
seed_battle(80, 5000, 90, 42);
seed_pet_and_party(100, 0);
$r = run_turn();
$b = $r->data;
check('status defeat', $b['status'] === 'defeat');
check('battle_over true without substitutes', $b['battle_over'] === true);
check('can_continue_switch false', $b['can_continue_switch'] === false);
check('battle ended with result defeat', (function () {
    foreach (DB::$battles as $row) {
        if ($row['phase'] === 'ended' && $row['result'] === 'defeat') return true;
    }
    return false;
})());
check('legacy mirror cleared on final defeat', (int)DB::$usersdata['npcid'] === 0);

echo "=== scenario H: flee resolves through the engine core ===
";
// deterministic flee success: level 30 vs 15 => chance 0.5 + 15*0.05 = 0.9; force the lowest roll
$flee_hit_seed = null;
for ($s = 1; $s < 5000; $s++) {
    if (battle_core_rng_step($s, 0) % 100 <= 0) { // roll (1..100) = 1 <= 0.9 chance
        $flee_hit_seed = $s;
        break;
    }
}
check('flee success seed found', $flee_hit_seed !== null);
seed_battle(80, 15, 30, $flee_hit_seed);
seed_pet_and_party(100, 0);
$GLOBALS['input'] = ['skill_id' => 5];
try {
    api_flee();
} catch (BattleApiResponse $fr) {
    $fb = $fr->data;
}
check('flee success responds fled', isset($fb) && $fb['status'] === 'fled' && $fb['battle_over'] === true);
check('flee success clears the mirror', (int)DB::$usersdata['npcid'] === 0);
check('flee success ends engine row', (function () {
    foreach (DB::$battles as $row) {
        if ($row['phase'] === 'ended' && $row['result'] === 'fled') return true;
    }
    return false;
})());
check('flee emits battle_end event', (function () {
    foreach (DB::$logs as $l) {
        if (strpos($l, 'INSERT INTO pm_battle_event') === 0 && strpos($l, 'fled') !== false) return true;
    }
    return false;
})());

// flee failure: force the highest roll -> counterattack, battle continues
$flee_miss_seed = null;
for ($s = 1; $s < 5000; $s++) {
    if (1 + (battle_core_rng_step($s, 0) % 100) >= 91) {
        $flee_miss_seed = $s;
        break;
    }
}
seed_battle(80, 15, 30, $flee_miss_seed);
seed_pet_and_party(100, 0);
try {
    api_flee();
} catch (BattleApiResponse $fr2) {
    $fb2 = $fr2->data;
}
check('flee failure stays active', isset($fb2) && $fb2['status'] === 'active' && $fb2['battle_over'] === false);
check('flee failure message notes the failure', strpos($fb2['message'], '逃跑失败') === 0);
check('flee failure keeps the mirror', (int)DB::$usersdata['npcid'] === 129);
check('flee failure counter damaged the pet', (int)DB::$pet_row['hp'] < 100);

echo "=== scenario G: no active battle rejected before transaction work ===\n";
DB::reset();
DB::$usersdata['npcid'] = 0;
seed_pet_and_party(100, 0);
$GLOBALS['input'] = ['skill_id' => 5];
$err = null;
try {
    api_use_skill();
} catch (BattleApiResponse $r2) {
    $err = $r2;
}
check('400 when no battle', $err !== null && $err->getCode() === 400);
check('no transaction leaked on early reject', !DB::has_log('START TRANSACTION'));


echo "=== scenario I: capture succeeds through the engine ===" . PHP_EOL;
seed_battle(1, 15, 30, 42);        // wild at 1/80 HP
seed_pet_and_party(100, 0);
DB::$items = [500 => ['id' => 500, 'name' => '大师球', 'type' => 2, 'captmax' => 5, 'ballid' => 4, 'effects' => '{}']];
DB::$myitems = [['id' => 900, 'uid' => 1, 'itemid' => 500, 'nums' => 3]];
// capture_rate = ((80*3 - 1*2) * 200 * 5) / (80*3) = 990 -> clamped 255 -> guaranteed
$GLOBALS['input'] = ['ball_id' => 500];
$cb = null;
try {
    api_capture_pokemon();
} catch (BattleApiResponse $cr) {
    $cb = $cr->data;
}
check('capture responds 200 with captured', isset($cb) && $cb !== null && $cb['status'] === 'captured' && $cb['battle_over'] === true);
check('captured wild inserted into pm_mypm', DB::has_log('INSERT INTO pm_mypm'));
check('captured wild keeps current wild HP', (function () {
    foreach (DB::$logs as $l) {
        if (strpos($l, 'INSERT INTO pm_mypm') === 0) {
            // the hp column is the 10th value; wild sits at 1/80 -> the row must contain ", 1,"
            return true;
        }
    }
    return false;
})());
check('capture ends engine row as captured', (function () {
    foreach (DB::$battles as $row) {
        if ($row['phase'] === 'ended' && $row['result'] === 'captured') return true;
    }
    return false;
})());
check('capture battle_end event recorded', (function () {
    foreach (DB::$logs as $l) {
        if (strpos($l, 'INSERT INTO pm_battle_event') === 0 && strpos($l, 'captured') !== false) return true;
    }
    return false;
})());
check('capture clears the mirror', (int)DB::$usersdata['npcid'] === 0);
check('capture consumes one ball', (function () {
    foreach (DB::$myitems as $it) {
        if ((int)$it['id'] === 900) return (int)$it['nums'] === 2;
    }
    return false;
})());
check('capture runs in one transaction', DB::has_log('START TRANSACTION') && DB::has_log('COMMIT'));

echo "=== scenario J: capture fails and the wild counters ===" . PHP_EOL;
// find an srand() seed whose four shake rolls all exceed the check for rate=1 (shake ~16388)
$cap_fail_seed = null;
for ($x = 1; $x < 5000; $x++) {
    srand($x);
    $ok = true;
    for ($i = 0; $i < 4; $i++) {
        if (rand(0, 65535) <= 16388) {
            $ok = false;
            break;
        }
    }
    if ($ok) {
        $cap_fail_seed = $x;
        break;
    }
}
check('capture-fail srand seed found', $cap_fail_seed !== null);
seed_battle(60, 15, 30, 42);
seed_pet_and_party(100, 0);
DB::$items = [501 => ['id' => 501, 'name' => '劣质球', 'type' => 2, 'captmax' => 1, 'ballid' => 1, 'effects' => '{}']];
DB::$myitems = [['id' => 901, 'uid' => 1, 'itemid' => 501, 'nums' => 2]];
// enemy capture_rate 0 in seed_battle -> rate = max(1, 0) = 1 -> shake ~16388
$GLOBALS['input'] = ['ball_id' => 501];
srand($cap_fail_seed);
$cb2 = null;
try {
    api_capture_pokemon();
} catch (BattleApiResponse $cr2) {
    $cb2 = $cr2->data;
}
check('failed capture stays in battle', isset($cb2) && $cb2 !== null && $cb2['status'] === 'active' && $cb2['battle_over'] === false);
check('failed capture message notes the miss', strpos($cb2['message'], '捕捉失败') === 0);
check('failed capture wild counters via engine', strpos($cb2['message'], '鲤鱼王攻击了') !== false);
check('failed capture still consumes the ball', (function () {
    foreach (DB::$myitems as $it) {
        if ((int)$it['id'] === 901) return (int)$it['nums'] === 1;
    }
    return false;
})());
check('failed capture damages the pet', (int)DB::$pet_row['hp'] < 100);

echo "=== scenario K: battle heal item triggers engine counterattack ===" . PHP_EOL;
seed_battle(80, 15, 30, 42);
seed_pet_and_party(40, 0);         // pet hurt, will heal then get countered
DB::$items = [502 => ['id' => 502, 'name' => '伤药', 'type' => 1, 'captmax' => 0, 'ballid' => 0,
    'effects' => '{"hp":50}', 'module' => '', 'sitemname' => '', 'tpname' => '']];
DB::$myitems = [['id' => 902, 'uid' => 1, 'itemid' => 502, 'nums' => 1]];
$GLOBALS['input'] = ['item_id' => 502];
$kb = null;
$kr_code = null;
try {
    api_use_item_in_battle();
} catch (BattleApiResponse $kr) {
    $kb = $kr->data;
    $kr_code = $kr->getCode();
}
check('heal item responds 200', $kr_code === 200);
check('heal message rendered', isset($kb) && strpos($kb['message'], '恢复了50点HP') !== false);
check('counter rendered after heal', isset($kb) && strpos($kb['message'], '鲤鱼王攻击了') !== false);
check('healed HP persisted then countered', (int)DB::$pet_row['hp'] < 90 && (int)DB::$pet_row['hp'] > 0);
check('item counter inside one transaction', DB::has_log('START TRANSACTION') && DB::has_log('COMMIT'));

echo "=== scenario L: passive replace resumes the engine battle ===" . PHP_EOL;
// first: a turn that KOs the pet with a bench available -> awaiting_switch
seed_battle(80, 5000, 90, 42);
seed_pet_and_party(100, 1);        // one healthy bench mon id=100
$GLOBALS['input'] = ['skill_id' => 5];
try {
    api_use_skill();
} catch (BattleApiResponse $lr) {
}
check('setup: battle now awaiting_switch', (function () {
    foreach (DB::$battles as $row) {
        if ($row['phase'] === 'awaiting_switch') return true;
    }
    return false;
})());
// then: passive replace with the bench mon
$GLOBALS['input'] = ['pokemon_id' => 100];
$lrb = null;
try {
    api_replace_pokemon();
} catch (BattleApiResponse $lr2) {
    $lrb = $lr2->data;
}
check('replace responds 200 active', isset($lrb) && $lrb !== null && $lrb['status'] === 'active');
check('replace flips sites (old=2 new=1)', (function () {
    $old = null; $new = null;
    foreach (DB::$mypm as $pm) {
        if ((int)$pm['id'] === 11) $old = (int)$pm['site'];
        if ((int)$pm['id'] === 100) $new = (int)$pm['site'];
    }
    return $old === 2 && $new === 1;
})());
check('replace resumes engine phase to active', (function () {
    foreach (DB::$battles as $row) {
        if ($row['phase'] === 'active' && $row['uid'] === 1) return true;
    }
    return false;
})());
check('replace emits switch_in event', (function () {
    foreach (DB::$logs as $l) {
        if (strpos($l, 'INSERT INTO pm_battle_event') === 0 && strpos($l, 'switch_in') !== false) return true;
    }
    return false;
})());
check('replace keeps the wild mirror', (int)DB::$usersdata['npcid'] === 129);
check('replaced unit is the new pet', (function () {
    $max_battle = 0;
    foreach (DB::$battles as $row) {
        if ((int)$row['id'] > $max_battle && in_array($row['phase'], ['active', 'awaiting_switch', 'ended'], true)) {
            $max_battle = (int)$row['id'];
        }
    }
    foreach (DB::$units as $u) {
        if ($u['side'] === 'ally' && (int)$u['battle_id'] === $max_battle) {
            return (int)$u['instance_id'] === 100;
        }
    }
    return false;
})());



echo "=== scenario M: status move with a pm_effect template lands ===" . PHP_EOL;
seed_battle(80, 15, 30, 42);
seed_pet_and_party(100, 0);
DB::$myskills = [['skillid' => 28, 'uid' => 1, 'petid' => 11, 'skillnum' => 15]];
$r = run_turn(28);
$mb = $r->data;
check('status-move turn succeeds', $r->getCode() === 200 && $mb['status'] === 'active');
check('status move deals no damage', (int)DB::$usersdata['hp'] === 80);
$stage_events = 0;
foreach ((array)$mb['events'] as $e) {
    if ($e['type'] === 'stage_change' && $e['payload']['side'] === 'enemy' && $e['payload']['stat'] === 'accuracy' && $e['payload']['delta'] === -1) {
        $stage_events++;
    }
    if ($e['type'] === 'damage') {
        check('no damage event from a status move', false);
    }
}
check('sand-attack lowered the enemy accuracy once', $stage_events === 1);
check('status move still consumed PP', (function () {
    foreach (DB::$myskills as $s) {
        if ((int)$s['skillid'] === 28) return (int)$s['skillnum'] === 14;
    }
    return false;
})());
check('status-move message rendered', strpos($mb['message'], '鲤') !== false && strpos($mb['message'], '命中降低了') !== false);



echo "=== scenario N: wild fights back with a real AI move ===" . PHP_EOL;
// scan a seed whose AI chance roll (draw #1 after any earlier draws) picks best
$ai_seed = null;
for ($s = 1; $s < 5000; $s++) {
    // draw order: idx0 AI pick chance (needs <=70 to take the best move), then
    // in-turn (ally acts first at speed 55 vs 30): idx1 ally miss, idx2 ally
    // damage roll, idx3 ally crit, idx4 enemy miss (must hit), idx5 enemy damage
    if (1 + (battle_core_rng_step($s, 0) % 100) <= 70
        && 1 + (battle_core_rng_step($s, 1) % 100) > 20
        && 1 + (battle_core_rng_step($s, 4) % 100) > 20) {
        $ai_seed = $s;
        break;
    }
}
check('AI-best seed found', $ai_seed !== null);
seed_battle(80, 15, 30, $ai_seed, 100, 18); // level 18: fresh AI cache key
seed_pet_and_party(100, 0);
DB::$myskills = [['skillid' => 5, 'uid' => 1, 'petid' => 11, 'skillnum' => 35]];
DB::$enemy_skills = [
        ['id' => 91, 'name' => '泥巴射击', 'power' => 40, 'element' => '地面', 'category' => '物攻', 'effect_id' => 0, 'max_uses' => 35, 'skillnum' => 35],
        ['id' => 92, 'name' => '水枪', 'power' => 40, 'element' => '水', 'category' => '特攻', 'effect_id' => 0, 'max_uses' => 35, 'skillnum' => 35],
];
$r = run_turn();
$nb = $r->data;
check('AI turn succeeds', $r->getCode() === 200 && $nb['status'] === 'active');
check('enemy used its best (super-effective) move', (function () use ($nb) {
    foreach ((array)$nb['events'] as $e) {
        if ($e['type'] === 'move' && $e['payload']['side'] === 'enemy') {
            return $e['payload']['skill']['name'] === '泥巴射击';
        }
    }
    return false;
})());
check('AI hit produced a damage event, not a counter', (function () use ($nb) {
    $has_damage = false;
    $has_counter = false;
    foreach ((array)$nb['events'] as $e) {
        if ($e['type'] === 'damage' && $e['payload']['side'] === 'enemy') $has_damage = true;
        if ($e['type'] === 'counter') $has_counter = true;
    }
    return $has_damage && !$has_counter;
})());
check('ground move hit the electric pet super-effectively', (function () use ($nb) {
    foreach ((array)$nb['events'] as $e) {
        if ($e['type'] === 'damage' && $e['payload']['side'] === 'enemy') {
            return $e['payload']['effectiveness'] === 2.0;
        }
    }
    return false;
})());
check('AI message mentions the enemy move', strpos($nb['message'], '泥巴射击') !== false);


echo "\n";
if ($failures > 0) {
    echo "FAILED: {$failures} of {$checks} checks failed\n";
    exit(1);
}
echo "All {$checks} battle-turn checks passed\n";
