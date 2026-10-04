<?php
/**
 * Execute the real active-switch endpoint against in-memory pet rows.
 * No live forum is needed; the counterattack roll is made deterministic via srand.
 * Run: php scripts/test/active_pokemon_switch.php
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

// Load actual endpoint functions without running the Discuz dispatcher.
// pm_abort_battle_transaction lives in utils.php and is mocked below.
$wanted = ['api_switch_pokemon'];
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

class SwitchResponse extends RuntimeException
{
    public $data;
    public function __construct($message, $code, $data = null)
    {
        parent::__construct($message, $code);
        $this->data = $data;
    }
}

function api_error($message, $code) { throw new SwitchResponse($message, $code); }
function battle_ensure_tables()
{
    // engine table lazy-DDL: not under test here (site-swap semantics only)
}
function api_success($data) { throw new SwitchResponse('success', 200, $data); }
// 与 utils.php 中的生产版本一致：回滚未提交事务后以错误终止
function pm_abort_battle_transaction($message, $status = 400)
{
    DB::query("ROLLBACK");
    api_error($message, $status);
}
function require_login() {}
function get_json_input() { return $GLOBALS['input']; }
function pm_table($name) { return $name; }
function pm_sql($sql, ...$args) { return vsprintf($sql, $args); }
function api_my_usersdata($uid) { return $GLOBALS['user']; }
function api_my_pokemon($username)
{
    foreach (DB::rows() as $pet) {
        if ((int) $pet['uid'] === (int) $GLOBALS['_G']['uid'] && (int) $pet['site'] === 1) return $pet;
    }
    return false;
}
function pm_data($id)
{
    $GLOBALS['calls']['pm_data']++;
    return ['name' => 'Wild', 'strength' => 1, 'xs' => 'normal'];
}
function battle_calc_my_stats($data, $pokemon)
{
    return [150, 100, 25, 40, 35, 50];
}
function battle_calc_npc_stats($data, $user, $strength)
{
    return [100, 30, 20, 20, 20, 20];
}
function calculate_counter_damage_legacy(...$args)
{
    $GLOBALS['calls']['counter']++;
    $GLOBALS['counter_args'] = $args;
    return $GLOBALS['counter_damage'];
}
function api_calculate_pokemon_max_hp($pokemon) { return 100; }
function api_validate_and_correct_hp($pokemon, $hp, $max_hp) { return ['hp' => $hp]; }
function build_battle_response($user, $pokemon) { return ['my_pokemon' => $pokemon, 'wild_pokemon' => ['hp' => $user['hp']]]; }
function clear_battle_state($uid)
{
    $GLOBALS['calls']['clear']++;
    $GLOBALS['user'] = array_merge($GLOBALS['user'], [
        'npcid' => 0, 'level' => 0, 'hp' => 0, 'hpg' => 0,
        'atkg' => 0, 'defg' => 0, 'spatkg' => 0, 'spdefg' => 0, 'sdg' => 0,
        'allure' => 0, 'capture' => 0,
    ]);
}

class DB
{
    public static $pets;
    public static $writes;
    public static $reads;
    public static $txn;
    public static $affected = 0;
    // 模拟并发竞争的钩子：$on_lock 在行锁 SELECT 时、$on_promote 在升位 UPDATE 求值前触发
    public static $on_lock;
    public static $on_promote;
    private static $in_txn = false;
    private static $pending = null;
    private static $pending_writes = [];

    // 与被动替换套件相同的 REPEATABLE READ 建模：锁后首次读建立快照，
    // 写入缓冲到 COMMIT，UPDATE 按最新已提交版本求值 WHERE。
    public static function rows()
    {
        if (self::$in_txn && self::$pending === null) {
            self::$pending = self::$pets;
        }
        return self::$pending !== null ? self::$pending : self::$pets;
    }

    private static function eligible($pet, $uid)
    {
        return (int) $pet['uid'] === (int) $uid && (int) $pet['site'] < 3
            && (int) $pet['hp'] > 0 && (int) $pet['state'] !== 0;
    }
    public static function fetch_first($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (preg_match('/^SELECT uid FROM pm_usersdata WHERE uid = (\d+) FOR UPDATE$/', $sql, $match)) {
            if (self::$on_lock) {
                $interpose = self::$on_lock;
                self::$on_lock = null;
                $interpose();
            }
            return !empty($GLOBALS['user']['npcid']) ? ['uid' => (int) $match[1]] : false;
        }
        self::$reads++;
        if (!preg_match('/^SELECT \* FROM pm_mypm WHERE uid = (\d+) AND id = (\d+) AND site < 3 AND hp > 0 AND state != 0$/', $sql, $match)) {
            throw new RuntimeException('Unexpected read: ' . $sql);
        }
        foreach (self::rows() as $pet) {
            if ((int) $pet['id'] === (int) $match[2] && self::eligible($pet, $match[1])) return $pet;
        }
        return false;
    }
    public static function fetch_all($sql)
    {
        self::$reads++;
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (!preg_match('/^SELECT \* FROM pm_mypm WHERE uid = (\d+) AND site < 3 AND hp > 0 AND state != 0 ORDER BY site ASC, id ASC$/', $sql, $match)) {
            throw new RuntimeException('Unexpected list: ' . $sql);
        }
        $pets = array_values(array_filter(self::rows(), function ($pet) use ($match) {
            return self::eligible($pet, $match[1]);
        }));
        usort($pets, function ($a, $b) {
            return ((int) $a['site'] <=> (int) $b['site']) ?: ((int) $a['id'] <=> (int) $b['id']);
        });
        return $pets;
    }
    public static function result_first($sql)
    {
        self::$reads++;
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (!preg_match('/^SELECT COUNT\(\*\) FROM pm_mypm WHERE uid = (\d+) AND site < 3 AND hp > 0 AND state != 0 AND id != (\d+)$/', $sql, $match)) {
            throw new RuntimeException('Unexpected scalar: ' . $sql);
        }
        $count = 0;
        foreach (self::rows() as $pet) {
            if ((int) $pet['id'] !== (int) $match[2] && self::eligible($pet, $match[1])) $count++;
        }
        return $count;
    }
    public static function query($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        self::$affected = 0;
        if ($sql === 'START TRANSACTION') {
            if (self::$in_txn) {
                throw new RuntimeException('Nested transaction');
            }
            self::$in_txn = true;
            self::$pending = null;
            self::$pending_writes = [];
            self::$txn[] = $sql;
        } elseif ($sql === 'COMMIT') {
            if (!self::$in_txn) {
                throw new RuntimeException('Commit outside a transaction');
            }
            if (self::$pending !== null) {
                self::$pets = self::$pending;
                self::$writes = array_merge(self::$writes, self::$pending_writes);
            }
            self::$in_txn = false;
            self::$pending = null;
            self::$pending_writes = [];
            self::$txn[] = $sql;
        } elseif ($sql === 'ROLLBACK') {
            // 真实 MySQL 里无事务时 ROLLBACK 是无害告警；幂等处理且不留痕
            if (self::$in_txn) {
                self::$in_txn = false;
                self::$pending = null;
                self::$pending_writes = [];
                self::$txn[] = $sql;
            }
        } elseif (preg_match('/^UPDATE pm_mypm SET site = 2 WHERE id = (\d+) AND uid = (\d+) AND site = 1$/', $sql, $match)) {
            foreach (self::pending_rows() as &$pet) {
                if ((int) $pet['id'] === (int) $match[1] && (int) $pet['uid'] === (int) $match[2] && (int) $pet['site'] === 1) {
                    $pet['site'] = 2;
                    self::$affected = 1;
                }
            }
            unset($pet);
            if (self::$affected) self::$pending_writes[] = $sql;
        } elseif (preg_match('/^UPDATE pm_mypm SET site = 1 WHERE id = (\d+) AND uid = (\d+) AND site < 3 AND hp > 0 AND state != 0$/', $sql, $match)) {
            if (self::$on_promote) {
                $interpose = self::$on_promote;
                self::$on_promote = null;
                $interpose();
            }
            $rows = &self::pending_rows();
            foreach (self::$pets as $index => $latest) {
                if ((int) $latest['id'] === (int) $match[1] && self::eligible($latest, $match[2])) {
                    if ((int) $rows[$index]['id'] !== (int) $match[1]) {
                        $rows[$index] = $latest;
                    }
                    $rows[$index]['site'] = 1;
                    self::$affected = 1;
                }
            }
            unset($rows);
            if (self::$affected) self::$pending_writes[] = $sql;
        } elseif (preg_match('/^UPDATE pm_mypm SET hp = (\d+) WHERE id = (\d+)$/', $sql, $match)) {
            foreach (self::pending_rows() as &$pet) {
                if ((int) $pet['id'] === (int) $match[2]) {
                    $pet['hp'] = (int) $match[1];
                }
            }
            unset($pet);
            self::$affected = 1;
            self::$pending_writes[] = $sql;
        } else {
            throw new RuntimeException('Unexpected write: ' . $sql);
        }
    }

    public static function affected_rows()
    {
        return self::$affected;
    }

    public static function reset_txn()
    {
        self::$in_txn = false;
        self::$pending = null;
        self::$pending_writes = [];
    }

    private static function &pending_rows()
    {
        if (!self::$in_txn) {
            throw new RuntimeException('Write outside a transaction');
        }
        if (self::$pending === null) {
            self::$pending = self::$pets;
        }
        return self::$pending;
    }
}

function reset_battle()
{
    $GLOBALS['_G'] = ['uid' => 7, 'username' => 'test-player'];
    $GLOBALS['input'] = ['pokemon_id' => 11];
    $GLOBALS['user'] = ['npcid' => 25, 'level' => 10, 'hp' => 90, 'hpg' => 100, 'strength' => 1];
    $GLOBALS['counter_damage'] = 10;
    $GLOBALS['counter_args'] = [];
    $GLOBALS['calls'] = array_fill_keys(['pm_data', 'counter', 'clear'], 0);
    srand(3); // rand(1, 100) = 87 > 30，默认不触发反击
    DB::$pets = [
        ['id' => 10, 'uid' => 7, 'hp' => 100, 'state' => 1, 'site' => 1, 'species_id' => 1, 'nickname' => 'Active'],
        ['id' => 11, 'uid' => 7, 'hp' => 80, 'state' => 1, 'site' => 2, 'species_id' => 1, 'nickname' => 'Reserve'],
    ];
    DB::$writes = [];
    DB::$reads = 0;
    DB::$txn = [];
    DB::$affected = 0;
    DB::$on_lock = null;
    DB::$on_promote = null;
    DB::reset_txn();
}
function check($condition, $message) { if (!$condition) throw new RuntimeException($message); }
function response($code, $message = null)
{
    try {
        api_switch_pokemon();
    } catch (SwitchResponse $response) {
        check($response->getCode() === $code, 'Unexpected response: ' . $response->getMessage());
        if ($message !== null) check($response->getMessage() === $message, 'Unexpected response message');
        return $response->data;
    }
    throw new RuntimeException('Endpoint did not respond');
}
function rejected($message)
{
    $snapshot = [DB::$pets, $GLOBALS['user']];
    response(400, $message);
    check(DB::$writes === [] && [DB::$pets, $GLOBALS['user']] === $snapshot, 'Rejected switch changed state');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Rejected switch did not roll its transaction back');
    check($GLOBALS['calls']['counter'] === 0, 'Rejected switch resolved a counterattack');
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
// 断言完成一次换位：恰好一个 site=1，且是 $expected_id
function switched_to($expected_id, $hp_writes)
{
    $data = response(200);
    check((int) $data['my_pokemon']['id'] === $expected_id, 'Wrong active pet after switch');
    check($data['status'] === 'active' && $data['battle_over'] === false && $data['turn'] === 0, 'Switch response changed');
    $actives = 0;
    foreach (DB::$pets as $pet) {
        if ((int) $pet['site'] === 1) $actives++;
    }
    check($actives === 1, 'Switch left more than one active pet');
    check(DB::$txn === ['START TRANSACTION', 'COMMIT'], 'Switch did not commit its transaction');
    check(count(DB::$writes) === 2 + $hp_writes, 'Unexpected number of writes for the switch');
}

run_case('Specified switch without counterattack', function () {
    switched_to(11, 0);
    check(strpos($GLOBALS['calls']['counter'] === 0 ? '' : 'x', 'x') === false, 'Counter resolved without a roll');
    check((int) DB::$pets[0]['site'] === 2 && (int) DB::$pets[1]['site'] === 1, 'Sites not swapped');
});
run_case('Automatic switch picks the first reserve', function () {
    $GLOBALS['input']['pokemon_id'] = 0;
    switched_to(11, 0);
});
run_case('Switch with surviving counterattack damages the new pet', function () {
    srand(7); // rand(1, 100) = 16 <= 30，触发反击
    switched_to(11, 1);
    check($GLOBALS['calls']['counter'] === 1 && (int) DB::$pets[1]['hp'] === 70, 'Counter damage did not subtract from the new pet current HP');
    check($GLOBALS['counter_args'][2] === 25 && $GLOBALS['counter_args'][4] === 35, 'Counterattack received the wrong defense stats');
    check((int) DB::$pets[0]['hp'] === 100, 'Counterattack damaged the previous pet');
});
run_case('Counterattack defeats the new pet with reserves left', function () {
    srand(7);
    $GLOBALS['counter_damage'] = 200;
    $data = response(200);
    check($data['status'] === 'active' && $data['battle_over'] === false && $data['can_continue_switch'] === true, 'Defeat-with-reserves response changed');
    check((int) DB::$pets[1]['hp'] === 0, 'Defeated new pet kept HP');
    check($GLOBALS['calls']['clear'] === 0, 'Battle state cleared although reserves remain');
    check(DB::$txn === ['START TRANSACTION', 'COMMIT'], 'Defeat-with-reserves did not commit');
});
run_case('Counterattack defeats the last pet and ends the battle', function () {
    srand(7);
    $GLOBALS['counter_damage'] = 200;
    // 旧上场宠物已倒下：新宠物被反击打倒后再无可用宠物
    DB::$pets[0]['hp'] = 0;
    $data = response(200);
    check($data['status'] === 'defeat' && $data['battle_over'] === true && $data['can_continue_switch'] === false, 'Defeat response changed');
    check((int) DB::$pets[1]['hp'] === 0, 'Defeated new pet kept HP');
    check($GLOBALS['calls']['clear'] === 1 && $GLOBALS['user']['npcid'] === 0, 'Battle state not cleared on final defeat');
    check(DB::$txn === ['START TRANSACTION', 'COMMIT'], 'Final defeat did not commit');
});
run_case('Switching to the current pet is rejected', function () {
    $GLOBALS['input']['pokemon_id'] = 10;
    rejected('不能切换到当前上场的宠物');
});
run_case('Switching to a missing reserve is rejected', function () {
    $GLOBALS['input']['pokemon_id'] = 99;
    rejected('指定的宠物不可用');
});
run_case('Switching without any reserve is rejected', function () {
    $GLOBALS['input']['pokemon_id'] = 0;
    DB::$pets = [DB::$pets[0]];
    rejected('没有可用的替补宠物');
});
run_case('Missing active pet is rejected under the lock', function () {
    DB::$pets[0]['site'] = 2;
    rejected('没有上场宠物');
});
run_case('No active battle is rejected before the lock', function () {
    $GLOBALS['user']['npcid'] = 0;
    $snapshot = [DB::$pets, $GLOBALS['user']];
    response(400, '没有进行中的战斗');
    check(DB::$writes === [] && [DB::$pets, $GLOBALS['user']] === $snapshot, 'Rejected switch changed state');
    check(DB::$txn === [], 'Rejection before the battle check still opened a transaction');
});

// 并发竞争：另一请求已在本请求取得锁前完成了一整套切换。
run_case('Switch after a concurrent switch stays consistent', function () {
    $GLOBALS['input']['pokemon_id'] = 0;
    DB::$on_lock = function () {
        // 第一个切换请求已提交：10 退到替补，11 上场且健康
        DB::$pets[0]['site'] = 2;
        DB::$pets[1]['site'] = 1;
    };
    // 本请求锁内重读后：上场是 11，自动选择替补 10 并换回
    $data = response(200);
    check((int) $data['my_pokemon']['id'] === 10, 'Wrong active pet after the second switch');
    $actives = 0;
    foreach (DB::$pets as $pet) {
        if ((int) $pet['site'] === 1) $actives++;
    }
    check($actives === 1, 'Concurrent switches left more than one active pet');
    check((int) DB::$pets[0]['site'] === 1 && (int) DB::$pets[1]['site'] === 2, 'Sites not swapped for the second switch');
    check(DB::$txn === ['START TRANSACTION', 'COMMIT'], 'Second switch did not commit');
});
run_case('Reserve boxed before promotion rolls the switch back', function () {
    $GLOBALS['input']['pokemon_id'] = 11;
    DB::$on_promote = function () {
        DB::$pets[1]['site'] = 3;
    };
    $expected = DB::$pets;
    $expected[1]['site'] = 3;
    response(400, '指定的宠物不可用');
    check(DB::$pets === $expected && DB::$writes === [], 'Failed promotion still wrote site changes');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Failed promotion did not roll back');
});
run_case('Mid-transaction database error rolls back and rethrows', function () {
    $GLOBALS['input']['pokemon_id'] = 11;
    DB::$on_promote = function () {
        throw new RuntimeException('simulated driver failure');
    };
    $snapshot = DB::$pets;
    $thrown = null;
    try {
        api_switch_pokemon();
    } catch (RuntimeException $error) {
        $thrown = $error;
    }
    check($thrown !== null && $thrown->getMessage() === 'simulated driver failure', 'Driver failure was swallowed');
    check(DB::$pets === $snapshot && DB::$writes === [], 'Failed transaction left changes behind');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Failed transaction did not roll back');
});

echo "Active switch tests: $passed passed, $failed failed.\n";
exit($failed === 0 ? 0 : 1);
