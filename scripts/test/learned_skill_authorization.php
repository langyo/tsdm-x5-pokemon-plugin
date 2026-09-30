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

// Load actual endpoint functions without running the Discuz dispatcher.
$wanted = ['api_use_skill', 'api_normalize_skill_category', 'pm_refund_reserved_skill_pp'];
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
    return ['name' => 'Wild', 'strength' => 1, 'xs' => 'normal'];
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
function handle_my_pokemon_fainted($uid, $id) { $GLOBALS['calls']['fainted']++; return [false, true]; }

class DB
{
    public static $pet;
    public static $skills;
    public static $learned;
    public static $writes;
    public static $affected = 0;
    // 模拟并发竞争：在 PP 预扣语句求值前变更数据，模拟另一请求抢先扣减
    public static $on_claim;

    public static function fetch_first($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
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
        } elseif (preg_match('/^UPDATE pm_myskill SET skillnum = skillnum \+ 1 WHERE skillid = (\d+) AND uid = (\d+) AND petid = (\d+)$/', $sql, $match)) {
            foreach (self::$learned as &$row) {
                if ((int) $row['skillid'] === (int) $match[1] && (int) $row['uid'] === (int) $match[2] && (int) $row['petid'] === (int) $match[3]) {
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
}

function reset_battle()
{
    $GLOBALS['_G'] = ['uid' => 7, 'username' => 'test-player'];
    $GLOBALS['input'] = ['skill_id' => 4];
    $GLOBALS['user'] = ['npcid' => 25, 'level' => 10, 'hp' => 100, 'hpg' => 100, 'strength' => 1];
    $GLOBALS['speed'] = 20; // 20/19 test both attack orders without random evasion.
    $GLOBALS['counter_damage'] = 1;
    $GLOBALS['calls'] = array_fill_keys(['data', 'stats', 'damage', 'counter', 'rewards', 'apply', 'clear', 'fainted'], 0);
    DB::$pet = ['id' => 10, 'uid' => 7, 'species_id' => 1, 'hp' => 100, 'level' => 10, 'nickname' => 'Active'];
    DB::$skills = [4 => ['id' => 4, 'name' => 'Learned move', 'power' => 40, 'max_uses' => 10, 'element' => 'normal', 'category' => '物攻']];
    DB::$learned = [['id' => 20, 'skillid' => 4, 'uid' => 7, 'petid' => 10, 'skillnum' => 2]];
    DB::$writes = [];
    DB::$on_claim = null;
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
function completed_turn($remaining_pp)
{
    $data = response(200);
    check($data['status'] === 'active' && $data['turn'] === 1 && $data['battle_over'] === false, 'Valid turn response changed');
    check($GLOBALS['calls']['damage'] === 1 && $GLOBALS['calls']['counter'] === 1, 'Valid turn missed an attack');
    check($GLOBALS['calls']['apply'] === 0 && $GLOBALS['calls']['clear'] === 0, 'Ongoing turn granted rewards or ended battle');
    check($GLOBALS['user']['hp'] === 90 && DB::$pet['hp'] === 99, 'Unexpected combat HP');
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
            check(count(DB::$writes) === 3, 'Expected PP and both HP updates');
        });
    }
    run_case("Learned unlimited skill works with zero PP at speed $speed", function () use ($speed) {
        $GLOBALS['speed'] = $speed;
        DB::$skills[4]['max_uses'] = 0;
        DB::$learned[0]['skillnum'] = 0;
        completed_turn(0);
        check(count(DB::$writes) === 2, 'Unlimited skill unexpectedly updated PP');
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
                check(count(DB::$writes) === ($state === 'unlimited' ? 2 : 3), 'Numeric string row changed PP update behavior');
            }
        });
    }
    run_case("Basic attack needs no learned skill at speed $speed", function () use ($speed) {
        $GLOBALS['speed'] = $speed;
        $GLOBALS['input']['skill_id'] = 0;
        DB::$learned = [];
        DB::$skills = [];
        $data = completed_turn(null);
        check(strpos($data['message'], '普通攻击') !== false && count(DB::$writes) === 2, 'Basic attack behavior changed');
    });
    run_case("Learned skill victory grants rewards once at speed $speed", function () use ($speed) {
        $GLOBALS['speed'] = $speed;
        $GLOBALS['user']['hp'] = 1;
        $data = response(200);
        check($data['status'] === 'victory' && $data['battle_over'] === true && $data['turn'] === 0, 'Victory response changed');
        check(DB::$learned[0]['skillnum'] === 1 && $GLOBALS['calls']['rewards'] === 1 && $GLOBALS['calls']['apply'] === 1, 'Victory PP or rewards incorrect');
        check($GLOBALS['calls']['damage'] === 1 && $GLOBALS['calls']['counter'] === ($speed === 20 ? 0 : 1), 'Victory attack order changed');
        check($GLOBALS['calls']['clear'] === 1 && $GLOBALS['user']['npcid'] === 0, 'Victory failed to clear battle');
    });
}
run_case('Pet defeated before its attack keeps learned PP', function () {
    $GLOBALS['speed'] = 19;
    $GLOBALS['counter_damage'] = 100;
    $data = response(200);
    check($data['status'] === 'defeat' && $data['can_continue_switch'] === true && $data['battle_over'] === false, 'Defeat response changed');
    check(DB::$learned[0]['skillnum'] === 2 && $GLOBALS['calls']['damage'] === 0 && $GLOBALS['calls']['counter'] === 1, 'Defeated pet attacked or consumed PP');
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
    srand(42); // 固定种子使 rand(1, 20) = 3 <= 4，必闪避
    $data = response(200);
    check(strpos($data['message'], '避开') !== false, 'Expected an evaded attack');
    check(DB::$learned[0]['skillnum'] === 2, 'Evaded attack did not refund the reserved PP');
    check($GLOBALS['user']['hp'] === 100 && $data['wild_pokemon']['hp'] === 100, 'Evaded attack still dealt damage');
    check($GLOBALS['calls']['counter'] === 1 && DB::$pet['hp'] === 99, 'Counterattack behavior changed');
    check(count(DB::$writes) === 4, 'Expected claim, HP and refund writes');
});
run_case('No active battle retains existing error', function () {
    $GLOBALS['user']['npcid'] = 0;
    rejected_turn(400, 'No active battle found');
});

echo "Learned skill authorization tests: $passed passed, $failed failed.\n";
exit($failed === 0 ? 0 : 1);
