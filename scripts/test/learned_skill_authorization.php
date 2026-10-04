<?php
/**
 * Defensive regressions executing api_use_skill against an in-memory DB.
 * Combat formulas are deterministic fixtures; no live forum or concurrency test.
 * Run: php scripts/test/learned_skill_authorization.php
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

// Load the pure battle-core (real engine math) before extracting endpoint fns.
define('IN_DISCUZ', 1);
require __DIR__ . '/../../plugin/api/battle_core.php';

// Load actual endpoint functions without running the Discuz dispatcher.
$wanted = ['api_use_skill', 'api_normalize_skill_category', 'pm_refund_reserved_skill_pp',
    'battle_ensure_tables', 'battle_load_active', 'battle_inject_ally_fresh_state',
    'battle_persist_state', 'battle_mirror_legacy'];
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

class SkillResponse extends RuntimeException
{
    public $data;
    public function __construct($message, $code, $data = null)
    {
        parent::__construct($message, $code);
        $this->data = $data;
    }
}

function api_error($message, $code) { throw new SkillResponse($message, $code); }
function pm_abort_battle_transaction($message, $status = 400)
{
    DB::query('ROLLBACK');
    api_error($message, $status);
}
function api_success($data) { throw new SkillResponse('success', 200, $data); }
function require_login() {}
function get_json_input() { return $GLOBALS['input']; }
function pm_table($name) { return $name; }
function pm_sql($sql, ...$args) { return vsprintf($sql, $args); }
function api_my_usersdata($uid) { return $GLOBALS['user']; }
function api_my_pokemon($username) { return DB::$pet; }
function pm_data($id)
{
    $GLOBALS['calls']['data']++;
    return ['name' => 'Wild', 'strength' => 1, 'xs' => 'normal', 'xs2' => '', 'id' => 25];
}
function battle_calc_my_stats($data, $pokemon)
{
    $GLOBALS['calls']['stats']++;
    return [100, 20, 20, 20, 20, $GLOBALS['speed']];
}
function battle_calc_npc_stats($data, $user, $strength)
{
    $GLOBALS['calls']['stats']++;
    return [100, 20, 20, 20, 20, 20];
}
function calculate_damage_legacy(...$args) { $GLOBALS['calls']['damage']++; return 10; }
function calculate_counter_damage_legacy(...$args) { $GLOBALS['calls']['counter']++; return $GLOBALS['counter_damage']; }
function api_calculate_pokemon_max_hp($pokemon) { return 100; }
function api_validate_and_correct_hp($pokemon, $hp, $max_hp) { return ['hp' => $hp]; }
function build_battle_response($user, $pokemon)
{
    return ['my_pokemon' => $pokemon, 'wild_pokemon' => ['hp' => $user['hp']]];
}
function calculate_rewards(...$args) { $GLOBALS['calls']['rewards']++; return ['experience' => 1]; }
function clear_battle_state($uid) { $GLOBALS['calls']['clear']++; $GLOBALS['user']['npcid'] = 0; }
function apply_rewards($uid, $pokemon, $rewards) { $GLOBALS['calls']['apply']++; return null; }
function handle_my_pokemon_fainted($uid, $id)
{
    $GLOBALS['calls']['fainted']++;
    if (!$GLOBALS['has_replacements']) {
        clear_battle_state($uid);
        return [true, false];
    }
    return [false, true];
}

class DB
{
    public static $pet;
    public static $skills;
    public static $learned;
    public static $writes;
    public static $affected = 0;
    // 模拟并发竞争：$on_claim 在 PP 预扣求值前、$on_refund 在退还求值前触发
    public static $on_claim;
    public static $on_refund;

    public static function fetch_first($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (strpos($sql, 'FOR UPDATE') !== false) {
            return ['uid' => $GLOBALS['_G']['uid']];
        }
        if (preg_match('/FROM pm_battle\b/', $sql)) {
            // no engine battle rows: battle_load_active falls into the lazy-migration path
            return false;
        }
        if (preg_match('/^SELECT \* FROM pm_skill WHERE id = (\d+)$/', $sql, $match)) {
            return self::$skills[(int) $match[1]] ?? false;
        }
        if (preg_match('/^SELECT \* FROM pm_myskill WHERE skillid = (\d+) AND uid = (\d+) AND petid = (\d+)$/', $sql, $match)) {
            foreach (self::$learned as $row) {
                if ((int) $row['skillid'] === (int) $match[1] && (int) $row['uid'] === (int) $match[2] && (int) $row['petid'] === (int) $match[3]) {
                    return $row;
                }
            }
            return false;
        }
        throw new RuntimeException('Unexpected read: ' . $sql);
    }

    public static function query($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        self::$affected = 0;
        if (preg_match('/^(START TRANSACTION|COMMIT|ROLLBACK)$/', $sql) || strpos($sql, 'CREATE TABLE') === 0) {
            // engine plumbing (txn control / lazy DDL): not gameplay state
            return;
        }
        if (preg_match('/^(INSERT INTO pm_battle(_unit|_event)?|UPDATE pm_battle\b|DELETE FROM pm_battle_unit)\b/', $sql)) {
            self::$affected = 1;
            return;
        }
        if (preg_match('/^UPDATE pm_usersdata SET npcid = (\d+), level = (\d+), hp = (\d+), hpg = (\d+)/', $sql, $match)) {
            // legacy mirror write from the engine
            $GLOBALS['user']['npcid'] = (int) $match[1];
            $GLOBALS['user']['hp'] = (int) $match[3];
            self::$affected = 1;
            return;
        }
        if (preg_match('/^UPDATE pm_myskill SET skillnum = skillnum - 1 WHERE skillid = (\d+) AND uid = (\d+) AND petid = (\d+) AND skillnum > 0$/', $sql, $match)) {
            if (self::$on_claim) {
                $interpose = self::$on_claim;
                self::$on_claim = null;
                $interpose();
            }
            foreach (self::$learned as &$row) {
                if ((int) $row['skillid'] === (int) $match[1] && (int) $row['uid'] === (int) $match[2] && (int) $row['petid'] === (int) $match[3] && (int) $row['skillnum'] > 0) {
                    $row['skillnum'] = (int) $row['skillnum'] - 1;
                    self::$affected = 1;
                }
            }
            unset($row);
            if (self::$affected) {
                self::$writes[] = $sql;
            }
        } elseif (preg_match('/^UPDATE pm_myskill SET skillnum = skillnum \+ 1 WHERE skillid = (\d+) AND uid = (\d+) AND petid = (\d+) AND skillnum < (\d+)$/', $sql, $match)) {
            if (self::$on_refund) {
                $interpose = self::$on_refund;
                self::$on_refund = null;
                $interpose();
            }
            foreach (self::$learned as &$row) {
                if ((int) $row['skillid'] === (int) $match[1] && (int) $row['uid'] === (int) $match[2] && (int) $row['petid'] === (int) $match[3] && (int) $row['skillnum'] < (int) $match[4]) {
                    $row['skillnum'] = (int) $row['skillnum'] + 1;
                    self::$affected = 1;
                }
            }
            unset($row);
            if (self::$affected) {
                self::$writes[] = $sql;
            }
        } elseif (preg_match('/^UPDATE pm_mypm SET hp = (\d+) WHERE id = (\d+)$/', $sql, $match)) {
            if (self::$pet['id'] === (int) $match[2]) {
                self::$pet['hp'] = (int) $match[1];
            }
            self::$affected = 1;
            self::$writes[] = $sql;
        } elseif (preg_match('/^UPDATE pm_usersdata SET hp = (\d+) WHERE uid = (\d+)$/', $sql, $match)) {
            if ($GLOBALS['_G']['uid'] === (int) $match[2]) {
                $GLOBALS['user']['hp'] = (int) $match[1];
            }
            self::$affected = 1;
            self::$writes[] = $sql;
        } else {
            throw new RuntimeException('Unexpected write: ' . $sql);
        }
    }

    public static function affected_rows()
    {
        return self::$affected;
    }

    public static function insert_id()
    {
        return 1;
    }
}

function reset_battle()
{
    $GLOBALS['_G'] = ['uid' => 7, 'username' => 'test-player'];
    $GLOBALS['input'] = ['skill_id' => 4];
    $GLOBALS['user'] = ['npcid' => 25, 'level' => 10, 'hp' => 100, 'hpg' => 100, 'strength' => 1, 'atkg' => 15, 'defg' => 20, 'spatkg' => 15, 'spdefg' => 20, 'sdg' => 20, 'capture' => 100, 'allure' => 0, 'uid' => 7];
    $GLOBALS['speed'] = 20; // 20/19 test both attack orders without random evasion.
    $GLOBALS['counter_damage'] = 1;
    $GLOBALS['has_replacements'] = true;
    $GLOBALS['calls'] = array_fill_keys(['data', 'stats', 'damage', 'counter', 'rewards', 'apply', 'clear', 'fainted'], 0);
    DB::$pet = ['id' => 10, 'uid' => 7, 'species_id' => 1, 'hp' => 100, 'state' => 1, 'level' => 10, 'nickname' => 'Active', 'pmname' => 'Species'];
    DB::$skills = [4 => ['id' => 4, 'name' => 'Learned move', 'power' => 40, 'max_uses' => 10, 'element' => 'normal', 'category' => '物攻']];
    DB::$learned = [['id' => 20, 'skillid' => 4, 'uid' => 7, 'petid' => 10, 'skillnum' => 2]];
    DB::$writes = [];
    DB::$on_claim = null;
    DB::$on_refund = null;
}
function check($condition, $message)
{
    if (!$condition) throw new RuntimeException($message);
}
function response($code, $message = null)
{
    try {
        api_use_skill();
    } catch (SkillResponse $response) {
        check($response->getCode() === $code, 'Unexpected response: ' . $response->getMessage());
        if ($message !== null) check($response->getMessage() === $message, 'Unexpected response message');
        return $response->data;
    }
    throw new RuntimeException('Endpoint did not respond');
}
function rejected_turn($code, $message)
{
    $snapshot = [DB::$pet, DB::$learned, $GLOBALS['user']];
    response($code, $message);
    check(DB::$writes === [] && [DB::$pet, DB::$learned, $GLOBALS['user']] === $snapshot, 'Rejected skill changed stored state');
    check(array_sum($GLOBALS['calls']) === 0, 'Rejected skill loaded battle data or produced combat effects');
}
function find_evading_mt_seed()
{
    // lazy migration draws its rng seed via mt_rand(1, 2147483647); with the
    // enemy acting first, rolls run [counter 85-100, miss 1-20] => pick a seed
    // whose draw #2 lands the miss (<= 4)
    for ($x = 1; $x < 5000; $x++) {
        mt_srand($x);
        $seed = mt_rand(1, 2147483647);
        if (1 + (battle_core_rng_step($seed, 1) % 20) <= 4) {
            return $x;
        }
    }
    throw new RuntimeException('no evading mt seed found');
}

function completed_turn($remaining_pp)
{
    $data = response(200);
    check($data['status'] === 'active' && $data['turn'] === 1 && $data['battle_over'] === false, 'Valid turn response changed');
    $event_types = [];
    foreach ($data['events'] as $event) {
        $event_types[] = $event['type'];
    }
    check(in_array('damage', $event_types, true) && in_array('counter', $event_types, true), 'Valid turn missed an attack');
    check($GLOBALS['calls']['apply'] === 0 && $GLOBALS['calls']['clear'] === 0, 'Ongoing turn granted rewards or ended battle');
    check($GLOBALS['user']['hp'] < 100 && $GLOBALS['user']['hp'] > 0 && DB::$pet['hp'] < 100 && DB::$pet['hp'] > 0, 'Unexpected combat HP');
    if ($remaining_pp !== null) check(DB::$learned[0]['skillnum'] === $remaining_pp, 'Unexpected remaining PP');
    return $data;
}
$passed = 0;
$failed = 0;
function run_case($name, $test)
{
    global $passed, $failed;
    reset_battle();
    try {
        $test();
        $passed++;
        echo "PASS $name\n";
    } catch (Throwable $error) {
        $failed++;
        echo "FAIL $name: {$error->getMessage()}\n";
    }
}

foreach ([20, 19] as $speed) {
    $rejected = [
        'Unknown skill' => function () { $GLOBALS['input']['skill_id'] = 99; },
        'Unlearned skill' => function () { DB::$learned = []; },
        'Another pet skill' => function () { DB::$learned[0]['petid'] = 11; },
        'Another user skill' => function () { DB::$learned[0]['uid'] = 8; },
        'Unlearned unlimited skill' => function () { DB::$skills[4]['max_uses'] = 0; DB::$learned = []; },
        'Depleted skill' => function () { DB::$learned[0]['skillnum'] = 0; },
    ];
    foreach ($rejected as $name => $prepare) {
        run_case("$name is rejected before combat at speed $speed", function () use ($name, $prepare, $speed) {
            $GLOBALS['speed'] = $speed;
            $prepare();
            $code = $name === 'Unknown skill' ? 404 : 400;
            $message = $name === 'Unknown skill' ? '技能不存在' : ($name === 'Depleted skill' ? 'Skill PP is depleted' : '当前宠物尚未学会该技能');
            rejected_turn($code, $message);
        });
    }
    foreach ([1, 2] as $pp) {
        run_case("Learned skill consumes one of $pp PP at speed $speed", function () use ($speed, $pp) {
            $GLOBALS['speed'] = $speed;
            DB::$learned[0]['skillnum'] = $pp;
            DB::$learned[] = ['id' => 21, 'skillid' => 4, 'uid' => 7, 'petid' => 11, 'skillnum' => 3];
            DB::$learned[] = ['id' => 22, 'skillid' => 4, 'uid' => 8, 'petid' => 10, 'skillnum' => 3];
            DB::$learned[] = ['id' => 23, 'skillid' => 5, 'uid' => 7, 'petid' => 10, 'skillnum' => 3];
            completed_turn($pp - 1);
            check(array_column(DB::$learned, 'skillnum') === [$pp - 1, 3, 3, 3], 'PP update affected an unrelated learned skill');
            check(count(DB::$writes) === 2, 'Expected PP and both HP updates');
        });
    }
    run_case("Learned unlimited skill works with zero PP at speed $speed", function () use ($speed) {
        $GLOBALS['speed'] = $speed;
        DB::$skills[4]['max_uses'] = 0;
        DB::$learned[0]['skillnum'] = 0;
        completed_turn(0);
        check(count(DB::$writes) === 1, 'Unlimited skill unexpectedly updated PP');
    });
    foreach (['unlimited', 'depleted', 'one PP'] as $state) {
        run_case("Numeric string learned row ($state) at speed $speed", function () use ($speed, $state) {
            $GLOBALS['speed'] = $speed;
            DB::$learned[0] = array_map('strval', DB::$learned[0]);
            DB::$skills[4]['id'] = '4';
            DB::$skills[4]['max_uses'] = $state === 'unlimited' ? '0' : '10';
            DB::$learned[0]['skillnum'] = $state === 'one PP' ? '1' : '0';
            if ($state === 'depleted') {
                rejected_turn(400, 'Skill PP is depleted');
            } else {
                completed_turn($state === 'unlimited' ? '0' : 0);
                check(count(DB::$writes) === ($state === 'unlimited' ? 1 : 2), 'Numeric string row changed PP update behavior');
            }
        });
    }
    run_case("Basic attack needs no learned skill at speed $speed", function () use ($speed) {
        $GLOBALS['speed'] = $speed;
        $GLOBALS['input']['skill_id'] = 0;
        DB::$learned = [];
        DB::$skills = [];
        $data = completed_turn(null);
        check(strpos($data['message'], '普通攻击') !== false && count(DB::$writes) === 1, 'Basic attack behavior changed');
    });
    run_case("Learned skill victory grants rewards once at speed $speed", function () use ($speed) {
        $GLOBALS['speed'] = $speed;
        $GLOBALS['user']['hp'] = 1;
        $data = response(200);
        check($data['status'] === 'victory' && $data['battle_over'] === true && $data['turn'] === 0, 'Victory response changed');
        check(DB::$learned[0]['skillnum'] === 1 && $GLOBALS['calls']['rewards'] === 1 && $GLOBALS['calls']['apply'] === 1, 'Victory PP or rewards incorrect');
        $victory_types = [];
    foreach ($data['events'] as $event) {
        $victory_types[] = $event['type'];
    }
    check(in_array('damage', $victory_types, true) && in_array('counter', $victory_types, true) === ($speed === 19), 'Victory attack order changed');
        check($GLOBALS['calls']['clear'] === 1 && $GLOBALS['user']['npcid'] === 0, 'Victory failed to clear battle');
    });
}
run_case('Pet defeated before its attack keeps learned PP', function () {
    $GLOBALS['speed'] = 19;
    // engine counterattack uses the wild snapshot: crank its attack so the counter one-shots the pet
    $GLOBALS['user']['atkg'] = 5000;
    $data = response(200);
    check($data['status'] === 'defeat' && $data['can_continue_switch'] === true && $data['battle_over'] === false, 'Defeat response changed');
    $defeat_types = [];
    foreach ($data['events'] as $event) {
        $defeat_types[] = $event['type'];
    }
    check(DB::$learned[0]['skillnum'] === 2 && !in_array('damage', $defeat_types, true) && in_array('counter', $defeat_types, true), 'Defeated pet attacked or consumed PP');
    check(DB::$pet['hp'] === 0 && $GLOBALS['calls']['fainted'] === 1 && $GLOBALS['calls']['apply'] === 0, 'Defeat effects incorrect');
});
run_case('Concurrent PP exhaustion is rejected before combat', function () {
    $GLOBALS['speed'] = 20;
    DB::$learned[0]['skillnum'] = 1;
    // 预检读到 1 点 PP；预扣求值前，并发回合抢先把它扣到 0
    DB::$on_claim = function () {
        DB::$learned[0]['skillnum'] = 0;
    };
    $data = response(400, 'Skill PP is depleted');
    check($data === null, 'Rejected turn returned battle data');
    check(array_sum($GLOBALS['calls']) === 0, 'Losing PP race still entered combat');
    check(DB::$writes === [], 'Losing PP race still wrote');
    check(DB::$pet['hp'] === 100 && $GLOBALS['user']['hp'] === 100, 'Losing PP race changed combat state');
});
run_case("Evaded second attack refunds the reserved PP", function () {
    // 闪避只可能出现在野怪更快时（npcsd - msd >= 10 且 rand <= 4），此时我方必为后手
    $GLOBALS['speed'] = 5;
    mt_srand(find_evading_mt_seed()); // 迁移种子确定 => 反击后我方 miss roll 必中
    $data = response(200);
    check(strpos($data['message'], '避开') !== false, 'Expected an evaded attack');
    check(DB::$learned[0]['skillnum'] === 2, 'Evaded attack did not refund the reserved PP');
    check($GLOBALS['user']['hp'] === 100 && $data['wild_pokemon']['hp'] === 100, 'Evaded attack still dealt damage');
    $evaded_types = [];
    foreach ($data['events'] as $event) {
        $evaded_types[] = $event['type'];
    }
    check(in_array('counter', $evaded_types, true) && DB::$pet['hp'] < 100 && DB::$pet['hp'] > 0, 'Counterattack behavior changed');
    check(count(DB::$writes) === 3, 'Expected claim, HP and refund writes');
});
run_case('Refund never exceeds the skill PP cap', function () {
    // 预扣（2→1）之后、退还之前，并发 PP 道具把 skillnum 回满到 max_uses
    $GLOBALS['speed'] = 5;
    mt_srand(find_evading_mt_seed());
    DB::$skills[4]['max_uses'] = 2;
    DB::$on_refund = function () {
        DB::$learned[0]['skillnum'] = 2;
    };
    $data = response(200);
    check(strpos($data['message'], '避开') !== false, 'Expected an evaded attack');
    check(DB::$learned[0]['skillnum'] === 2, 'Refund pushed PP past the cap');
});
run_case('No active battle retains existing error', function () {
    $GLOBALS['user']['npcid'] = 0;
    rejected_turn(400, 'No active battle found');
});

run_case('Fast pet fainting preserves damage while a reserve can continue', function () {
    DB::$pet['hp'] = 1;
    $data = response(200);
    check($data['status'] === 'defeat' && $data['can_continue_switch'] === true && $data['battle_over'] === false, 'Expected a pending replacement');
    check($GLOBALS['user']['hp'] < 100 && $GLOBALS['user']['hp'] > 0 && $data['wild_pokemon']['hp'] === (int) $GLOBALS['user']['hp'], 'Damage dealt before fainting was lost');
    check(DB::$pet['hp'] === 0 && DB::$learned[0]['skillnum'] === 1, 'Completed attack did not consume PP or faint the pet');
    check($GLOBALS['calls']['clear'] === 0 && $GLOBALS['calls']['apply'] === 0, 'Pending replacement ended the battle');
});
run_case('Last pet fainting does not restore a cleared battle', function () {
    DB::$pet['hp'] = 1;
    $GLOBALS['has_replacements'] = false;
    $data = response(200);
    check($data['battle_over'] === true && $data['can_continue_switch'] === false, 'Last pet defeat did not end battle');
    check($GLOBALS['user']['npcid'] === 0 && $GLOBALS['calls']['clear'] >= 1, 'Defeat did not clear battle state');
    foreach (DB::$writes as $sql) {
        check(strpos($sql, 'UPDATE pm_usersdata SET hp') === false, 'Defeat wrote HP back into a cleared battle');
    }
});
foreach ([0, 4] as $skill_id) {
    run_case("Fainted pet cannot win with skill $skill_id while awaiting replacement", function () use ($skill_id) {
        DB::$pet['hp'] = 0;
        $GLOBALS['user']['hp'] = 1;
        $GLOBALS['input']['skill_id'] = $skill_id;
        rejected_turn(400, '当前宠物已倒下，请先更换宠物');
    });
}
run_case('Critical state cannot attack with positive stored HP', function () {
    DB::$pet['state'] = '0';
    rejected_turn(400, '当前宠物已倒下，请先更换宠物');
});
run_case('Missing active pet is rejected before combat', function () {
    DB::$pet = false;
    rejected_turn(400, '没有上场宠物');
});

echo "Learned skill authorization tests: $passed passed, $failed failed.\n";
exit($failed === 0 ? 0 : 1);
