<?php
/**
 * Execute the real passive replacement endpoint against in-memory pet rows.
 * No live forum, combat formulas or concurrent requests are exercised.
 * Run: php scripts/test/passive_pokemon_replacement.php
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

// Load actual endpoint functions without running the Discuz dispatcher.
// pm_abort_battle_transaction lives in utils.php and is mocked below.
$wanted = ['api_replace_pokemon'];
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

class ReplacementResponse extends RuntimeException
{
    public $data;
    public function __construct($message, $code, $data = null)
    {
        parent::__construct($message, $code);
        $this->data = $data;
    }
}
function api_error($message, $code) { throw new ReplacementResponse($message, $code); }

// ---- battle-engine adapter: no engine rows; the switch-in path is skipped ----
function battle_load_active($uid, $myusersdata, $mypokemon)
{
    return null;
}

function battle_ensure_tables()
{
    // engine table lazy-DDL: not under test here
}
function api_success($data) { throw new ReplacementResponse('success', 200, $data); }
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
function build_battle_response($user, $pokemon) { return ['my_pokemon' => $pokemon]; }
function calculate_counter_damage_legacy(...$args) { $GLOBALS['counter_calls']++; return 10; }

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

    // REPEATABLE READ 语义：事务的读视图在行锁之后第一次一致性读时建立（惰性快照），
    // 行锁前提交的并发改动可见；读视图建立后再变更 $pets 视为不可见的并发提交，
    // 只会体现在 UPDATE 对最新已提交版本的求值上。
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
            // pm_usersdata 行存在与否跟随战斗状态：无战斗视为行缺失
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
            // 真实 MySQL 里无事务时 ROLLBACK 是无害告警；幂等处理且不留痕，
            // 兼容「abort 回滚后异常继续冒泡再被兜底回滚一次」的路径
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
            // InnoDB 的 UPDATE 按最新已提交行版本求值 WHERE（本事务读视图未覆盖的
            // 并发提交仍生效），命中后写入落在事务副本上
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

    // 事务内的写入落到待提交副本；无事务时直接落表（本套件不应出现）
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
    $GLOBALS['user'] = ['npcid' => 25, 'hp' => 90];
    $GLOBALS['counter_calls'] = 0;
    DB::$pets = [
        ['id' => 10, 'uid' => 7, 'hp' => 0, 'state' => 1, 'site' => 1, 'nickname' => 'Active'],
        ['id' => 11, 'uid' => 7, 'hp' => 80, 'state' => 1, 'site' => 2, 'nickname' => 'Reserve'],
    ];
    DB::$writes = [];
    DB::$reads = 0;
    DB::$txn = [];
    DB::$affected = 0;
    DB::$on_lock = null;
    DB::$on_promote = null;
    DB::reset_txn();
}function check($condition, $message) { if (!$condition) throw new RuntimeException($message); }
function response($code, $message = null)
{
    try {
        api_replace_pokemon();
    } catch (ReplacementResponse $response) {
        check($response->getCode() === $code, 'Unexpected response: ' . $response->getMessage());
        if ($message !== null) check($response->getMessage() === $message, 'Unexpected response message');
        return $response->data;
    }
    throw new RuntimeException('Endpoint did not respond');
}
// $expected_reads：拒绝前允许的 pm_mypm 读取次数（null 为不检查）；
// 事务内的拒绝必须回滚且不留任何已提交写入。
function rejected($message, $expected_reads = null)
{
    $snapshot = [DB::$pets, $GLOBALS['user']];
    response(400, $message);
    check(DB::$writes === [] && [DB::$pets, $GLOBALS['user']] === $snapshot, 'Rejected replacement changed state');
    check($GLOBALS['counter_calls'] === 0, 'Rejected replacement caused a counterattack');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Rejected replacement did not roll its transaction back');
    if ($expected_reads !== null) check(DB::$reads === $expected_reads, 'Rejected replacement queried reserve pets');
}
function replaced($expected_id)
{
    $snapshot = DB::$pets;
    $user = $GLOBALS['user'];
    $data = response(200);
    check((int) $data['my_pokemon']['id'] === $expected_id, 'Wrong replacement pet');
    check($data['status'] === 'active' && $data['turn'] === 0 && $data['battle_over'] === false
        && $data['can_continue_switch'] === false, 'Replacement response changed');
    foreach ($snapshot as &$pet) {
        if ((int) $pet['id'] === 10) $pet['site'] = 2;
        if ((int) $pet['id'] === $expected_id) $pet['site'] = 1;
    }
    check(DB::$pets === $snapshot && count(DB::$writes) === 2, 'Replacement changed more than the two site fields');
    check(DB::$txn === ['START TRANSACTION', 'COMMIT'], 'Successful replacement did not commit its transaction');
    check($GLOBALS['user'] === $user && $GLOBALS['counter_calls'] === 0, 'Replacement changed wild state or caused a counterattack');
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

foreach ([11, 0] as $target) {
    foreach ([1, 100, '1', '100'] as $hp) {
        run_case('Healthy pet HP ' . var_export($hp, true) . " rejects target $target", function () use ($target, $hp) {
            $GLOBALS['input']['pokemon_id'] = $target;
            DB::$pets[0]['hp'] = $hp;
            rejected('当前宠物尚未倒下，请使用主动切换', 0);
        });
    }
    run_case("Current pet state zero with positive HP rejects target $target", function () use ($target) {
        $GLOBALS['input']['pokemon_id'] = $target;
        DB::$pets[0]['hp'] = 100;
        DB::$pets[0]['state'] = 0;
        rejected('当前宠物尚未倒下，请使用主动切换', 0);
    });
    foreach ([false, true] as $strings) {
        run_case(($strings ? 'Numeric string' : 'Integer') . " fainted pet replaces with target $target", function () use ($target, $strings) {
            $GLOBALS['input']['pokemon_id'] = $target;
            if ($strings) {
                foreach (DB::$pets as &$pet) {
                    foreach (['id', 'uid', 'hp', 'state', 'site'] as $field) $pet[$field] = (string) $pet[$field];
                }
            }
            replaced(11);
        });
    }
    run_case("Missing active pet rejects target $target", function () use ($target) {
        $GLOBALS['input']['pokemon_id'] = $target;
        DB::$pets[0]['site'] = 2;
        rejected('没有上场宠物', 0);
    });
    run_case("No active battle error precedes pet eligibility for target $target", function () use ($target) {
        $GLOBALS['input']['pokemon_id'] = $target;
        $GLOBALS['user']['npcid'] = 0;
        if ($target === 0) DB::$pets[0]['site'] = 2;
        else DB::$pets[0]['hp'] = 100;
        $snapshot = [DB::$pets, $GLOBALS['user']];
        response(400, '没有进行中的战斗');
        check(DB::$writes === [] && [DB::$pets, $GLOBALS['user']] === $snapshot, 'Rejected replacement changed state');
        check(DB::$txn === [], 'Rejection before the battle check still opened a transaction');
        check(DB::$reads === 0, 'Rejected replacement queried reserve pets');
    });
    $invalid = ['another user' => ['uid', 8], 'box' => ['site', 3], 'fainted' => ['hp', 0], 'state zero' => ['state', 0]];
    foreach ($invalid as $name => $change) {
        run_case("Reject $name reserve with target $target", function () use ($target, $change) {
            $GLOBALS['input']['pokemon_id'] = $target;
            DB::$pets[1][$change[0]] = $change[1];
            rejected($target === 0 ? '没有可用的替补宠物' : '指定的宠物不可用', 1);
        });
    }
}
run_case('Automatic replacement rejects no reserves', function () {
    $GLOBALS['input']['pokemon_id'] = 0;
    DB::$pets = [DB::$pets[0]];
    rejected('没有可用的替补宠物', 1);
});
run_case('Specified missing reserve is rejected', function () {
    $GLOBALS['input']['pokemon_id'] = 99;
    rejected('指定的宠物不可用', 1);
});
run_case('Specified fainted current pet is not an eligible replacement', function () {
    $GLOBALS['input']['pokemon_id'] = 10;
    rejected('指定的宠物不可用', 1);
});
run_case('Automatic replacement filters invalid pets and orders numeric string IDs', function () {
    $GLOBALS['input']['pokemon_id'] = 0;
    $reserve = DB::$pets[1];
    $reserve['id'] = '9';
    DB::$pets[] = $reserve;
    foreach (['uid' => 8, 'site' => 3, 'hp' => 0, 'state' => 0] as $field => $value) {
        $invalid = $reserve;
        $invalid['id'] = count(DB::$pets) - 2;
        $invalid[$field] = $value;
        DB::$pets[] = $invalid;
    }
    replaced(9);
});
run_case('Automatic replacement orders site before ID', function () {
    $GLOBALS['input']['pokemon_id'] = 0;
    // Existing query permits another site=1 row; preserve its ordering rule.
    $reserve = DB::$pets[1];
    $reserve['id'] = '90';
    $reserve['site'] = '1';
    DB::$pets[] = $reserve;
    replaced(90);
});

// 并发竞争：模拟另一请求在行锁释放后、本请求取得锁前完成了一整套替换。
run_case('Second concurrent replacement is rejected under the battle lock', function () {
    $GLOBALS['input']['pokemon_id'] = 11;
    $expected = DB::$pets;
    $expected[0]['site'] = 2;
    $expected[1]['site'] = 1;
    DB::$on_lock = function () {
        // 第一个替换请求已提交：原上场宠物退到替补，新宠物已上场且健康
        DB::$pets[0]['site'] = 2;
        DB::$pets[1]['site'] = 1;
    };
    response(400, '当前宠物尚未倒下，请使用主动切换');
    // 本请求零写入；并发替换的提交结果原样保留
    check(DB::$pets === $expected && DB::$writes === [], 'Rejected replacement changed state');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Rejected replacement did not roll its transaction back');
    check(DB::$reads === 0, 'Rejected replacement queried reserve pets');
    check($GLOBALS['counter_calls'] === 0, 'Rejected replacement caused a counterattack');
});

// 并发竞争：替补在读取之后、升位写入之前被并发操作装箱（site=3）。
run_case('Reserve boxed before promotion rolls the replacement back', function () {
    $GLOBALS['input']['pokemon_id'] = 11;
    DB::$on_promote = function () {
        DB::$pets[1]['site'] = 3;
    };
    $expected = DB::$pets;
    $expected[1]['site'] = 3;
    response(400, '指定的宠物不可用');
    // 回滚生效：并发装箱保留，本请求的降位/升位都不落地
    check(DB::$pets === $expected && DB::$writes === [], 'Failed promotion still wrote site changes');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Failed promotion did not roll back');
    check($GLOBALS['counter_calls'] === 0, 'Failed promotion caused a counterattack');
});

// 事务体内数据库异常：兜底 catch 必须回滚并把异常原样抛出（常驻 worker 防锁泄漏）。
run_case('Mid-transaction database error rolls back and rethrows', function () {
    $GLOBALS['input']['pokemon_id'] = 11;
    DB::$on_promote = function () {
        throw new RuntimeException('simulated driver failure');
    };
    $snapshot = DB::$pets;
    $thrown = null;
    try {
        api_replace_pokemon();
    } catch (RuntimeException $error) {
        $thrown = $error;
    }
    check($thrown !== null && $thrown->getMessage() === 'simulated driver failure', 'Driver failure was swallowed');
    check(DB::$pets === $snapshot && DB::$writes === [], 'Failed transaction left changes behind');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Failed transaction did not roll back');
});

echo "Passive replacement tests: $passed passed, $failed failed.\n";
exit($failed === 0 ? 0 : 1);
