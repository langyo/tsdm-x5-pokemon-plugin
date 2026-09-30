<?php
/**
 * Execute the real party-management endpoints (move / swap / set_first)
 * against in-memory pet rows. No live forum is needed.
 * Run: php scripts/test/party_pokemon_moves.php
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

// Load actual endpoint functions without running the Discuz dispatcher.
// pm_abort_battle_transaction lives in utils.php and is mocked below.
$wanted = ['api_move_pokemon', 'api_swap_pokemon', 'api_set_first_pokemon'];
$tokens = token_get_all(file_get_contents(__DIR__ . '/../../plugin/api/pokemon.php'));
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

class PartyResponse extends RuntimeException
{
    public $data;
    public function __construct($message, $code, $data = null)
    {
        parent::__construct($message, $code);
        $this->data = $data;
    }
}

function api_error($message, $code) { throw new PartyResponse($message, $code); }
function api_success($data) { throw new PartyResponse('success', 200, $data); }
// 与 utils.php 中的生产版本一致：回滚未提交事务后以错误终止
function pm_abort_battle_transaction($message, $status = 400)
{
    DB::query("ROLLBACK");
    api_error($message, $status);
}
function require_login() {}
function pm_table($name) { return $name; }
function pm_sql($sql, ...$args) { return vsprintf($sql, $args); }
function validate_id($value, $name = 'ID') { return (int) $value; }
function validate_uid($value) { return (int) $value; }
function get_param($key, $default = null) { return isset($GLOBALS['params'][$key]) ? $GLOBALS['params'][$key] : $default; }
function api_my_usersdata($uid) { return $GLOBALS['user']; }

class DB
{
    public static $pets;
    public static $writes;
    public static $txn;
    public static $affected = 0;
    // 模拟并发竞争的钩子：$on_lock 在行锁 SELECT 时、$on_final 在最终写入求值前触发
    public static $on_lock;
    public static $on_final;
    private static $in_txn = false;
    private static $pending = null;
    private static $pending_writes = [];

    // 与战斗套件相同的 REPEATABLE READ 建模：锁后首次读建立快照，
    // 写入缓冲到 COMMIT，UPDATE 按最新已提交版本求值 WHERE。
    public static function rows()
    {
        if (self::$in_txn && self::$pending === null) {
            self::$pending = self::$pets;
        }
        return self::$pending !== null ? self::$pending : self::$pets;
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
            return !empty($GLOBALS['user']) ? ['uid' => (int) $match[1]] : false;
        }
        if (preg_match('/^SELECT \* FROM pm_mypm WHERE id = (\d+) AND uid = (\d+)$/', $sql, $match)) {
            foreach (self::rows() as $pet) {
                if ((int) $pet['id'] === (int) $match[1] && (int) $pet['uid'] === (int) $match[2]) return $pet;
            }
            return false;
        }
        if (preg_match('/^SELECT \* FROM pm_mypm WHERE uid = (\d+) AND site = 1$/', $sql, $match)) {
            foreach (self::rows() as $pet) {
                if ((int) $pet['uid'] === (int) $match[1] && (int) $pet['site'] === 1) return $pet;
            }
            return false;
        }
        if (preg_match('/^SELECT id FROM pm_mypm WHERE uid = (\d+) AND site = 2 LIMIT 1$/', $sql, $match)) {
            foreach (self::rows() as $pet) {
                if ((int) $pet['uid'] === (int) $match[1] && (int) $pet['site'] === 2) return ['id' => $pet['id']];
            }
            return false;
        }
        throw new RuntimeException('Unexpected read: ' . $sql);
    }
    public static function result_first($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (!preg_match('/^SELECT COUNT\(\*\) FROM pm_mypm WHERE uid = (\d+) AND site IN \(1, 2\)$/', $sql, $match)) {
            throw new RuntimeException('Unexpected scalar: ' . $sql);
        }
        $count = 0;
        foreach (self::rows() as $pet) {
            if ((int) $pet['uid'] === (int) $match[1] && (in_array((int) $pet['site'], [1, 2], true))) $count++;
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
        } elseif (preg_match('/^UPDATE pm_mypm SET site = (\d+) WHERE id = (\d+) AND uid = (\d+)(?: AND site = (\d+))?$/', $sql, $match)) {
            $guarded = isset($match[4]);
            if ($guarded && self::$on_final) {
                $interpose = self::$on_final;
                self::$on_final = null;
                $interpose();
            }
            // InnoDB 语义：UPDATE 按最新已提交行版本求值 WHERE（含绕过行锁的
            // 并发写入，如后台直改），命中后写入落在事务副本上
            $rows = &self::pending_rows();
            foreach (self::$pets as $index => $latest) {
                if ((int) $latest['id'] === (int) $match[2] && (int) $latest['uid'] === (int) $match[3]
                    && (!$guarded || (int) $latest['site'] === (int) $match[4])
                    && (int) $rows[$index]['site'] !== (int) $match[1]) {
                    $rows[$index]['site'] = (int) $match[1];
                    self::$affected = 1;
                    self::$pending_writes[] = $sql;
                }
            }
            unset($rows);
        } elseif (preg_match('/^UPDATE pm_mypm SET site = (\d+) WHERE uid = (\d+) AND site = (\d+)$/', $sql, $match)) {
            $rows = &self::pending_rows();
            foreach (self::$pets as $index => $latest) {
                if ((int) $latest['uid'] === (int) $match[2] && (int) $latest['site'] === (int) $match[3]
                    && (int) $rows[$index]['site'] !== (int) $match[1]) {
                    $rows[$index]['site'] = (int) $match[1];
                    self::$affected = 1;
                    self::$pending_writes[] = $sql;
                }
            }
            unset($rows);
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

function reset_party()
{
    $GLOBALS['_G'] = ['uid' => 7, 'username' => 'test-player'];
    $GLOBALS['user'] = ['npcid' => 0, 'hp' => 0];
    $GLOBALS['params'] = ['pokemon_id' => 11, 'site' => 3];
    DB::$pets = [
        ['id' => 10, 'uid' => 7, 'hp' => 100, 'state' => 1, 'site' => 1, 'nickname' => 'First'],
        ['id' => 11, 'uid' => 7, 'hp' => 80, 'state' => 1, 'site' => 2, 'nickname' => 'Second'],
        ['id' => 12, 'uid' => 7, 'hp' => 60, 'state' => 1, 'site' => 2, 'nickname' => 'Third'],
        ['id' => 13, 'uid' => 7, 'hp' => 40, 'state' => 1, 'site' => 3, 'nickname' => 'Boxed'],
    ];
    DB::$writes = [];
    DB::$txn = [];
    DB::$affected = 0;
    DB::$on_lock = null;
    DB::$on_final = null;
    DB::reset_txn();
}
function check($condition, $message) { if (!$condition) throw new RuntimeException($message); }
function call_endpoint($fn, $code, $message = null)
{
    try {
        $fn();
    } catch (PartyResponse $response) {
        check($response->getCode() === $code, 'Unexpected response: ' . $response->getMessage());
        if ($message !== null) check($response->getMessage() === $message, 'Unexpected response message');
        return $response->data;
    }
    throw new RuntimeException('Endpoint did not respond');
}
function rejected($fn, $message, $code = 400)
{
    $snapshot = DB::$pets;
    call_endpoint($fn, $code, $message);
    check(DB::$writes === [] && DB::$pets === $snapshot, 'Rejected operation changed state');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Rejected operation did not roll its transaction back');
}
$passed = 0;
$failed = 0;
function run_case($name, $test)
{
    global $passed, $failed;
    reset_party();
    try {
        $test();
        $passed++;
        echo "PASS $name\n";
    } catch (Throwable $error) {
        $failed++;
        echo "FAIL $name: {$error->getMessage()}\n";
    }
}
function site_of($id)
{
    foreach (DB::$pets as $pet) {
        if ((int) $pet['id'] === (int) $id) return (int) $pet['site'];
    }
    throw new RuntimeException('No such pet: ' . $id);
}
function actives()
{
    $count = 0;
    foreach (DB::$pets as $pet) {
        if ((int) $pet['site'] === 1) $count++;
    }
    return $count;
}

run_case('Move a bag pet to the box keeps a bag pet', function () {
    $data = call_endpoint('api_move_pokemon', 200);
    check($data['message'] === '宝可梦移动成功' && $data['old_site'] === 2 && $data['new_site'] === 3, 'Move response changed');
    check(site_of(11) === 3, 'Pet was not boxed');
    check(DB::$txn === ['START TRANSACTION', 'COMMIT'], 'Move did not commit');
});
run_case('Move to first swaps with the current first', function () {
    $GLOBALS['params'] = ['pokemon_id' => 12, 'site' => 1];
    call_endpoint('api_move_pokemon', 200);
    check(site_of(12) === 1 && site_of(10) === 2, 'First-place swap broken');
    check(actives() === 1, 'Move left more than one active pet');
});
run_case('Boxing the first pet promotes the next bag pet', function () {
    $GLOBALS['params'] = ['pokemon_id' => 10, 'site' => 3];
    call_endpoint('api_move_pokemon', 200);
    check(site_of(10) === 3 && site_of(11) === 1, 'First-pet boxing broken');
    check(actives() === 1, 'No single active pet after boxing the first');
});
run_case('Boxing the last bag pet is rejected', function () {
    DB::$pets = [DB::$pets[0], DB::$pets[3]]; // 仅一只在包内
    $GLOBALS['params'] = ['pokemon_id' => 10, 'site' => 3];
    rejected('api_move_pokemon', '背包至少需要保留一只宠物');
});
run_case('Moving the first pet during battle is rejected', function () {
    $GLOBALS['user']['npcid'] = 25;
    $GLOBALS['params'] = ['pokemon_id' => 10, 'site' => 2];
    rejected('api_move_pokemon', '战斗中的首位宠物无法移动');
});
run_case('Bag pets can still be boxed during battle', function () {
    $GLOBALS['user']['npcid'] = 25;
    $GLOBALS['params'] = ['pokemon_id' => 11, 'site' => 3];
    $data = call_endpoint('api_move_pokemon', 200);
    check(site_of(11) === 3, 'Pet was not boxed');
});
run_case('Moving to the current site is a no-op without a transaction', function () {
    $GLOBALS['params'] = ['pokemon_id' => 11, 'site' => 2];
    $data = call_endpoint('api_move_pokemon', 200);
    check($data['message'] === '宝可梦已在目标位置', 'No-op response changed');
    check(DB::$txn === [], 'No-op move still opened a transaction');
});
run_case('Invalid target site is rejected', function () {
    $GLOBALS['params'] = ['pokemon_id' => 11, 'site' => 9];
    call_endpoint('api_move_pokemon', 400, 'Invalid site value');
    check(DB::$txn === [], 'Invalid site still opened a transaction');
});
run_case('Moving a foreign pet is rejected', function () {
    DB::$pets[3]['uid'] = 8;
    $GLOBALS['params'] = ['pokemon_id' => 13, 'site' => 2];
    call_endpoint('api_move_pokemon', 403, '宝可梦不存在或不属于您');
    check(DB::$txn === [], 'Foreign-pet move still opened a transaction');
});
run_case('Unboxing into a full bag is rejected', function () {
    // 背包已有 6 只
    for ($i = 20; $i <= 24; $i++) {
        DB::$pets[] = ['id' => $i, 'uid' => 7, 'hp' => 10, 'state' => 1, 'site' => 2, 'nickname' => "Filler$i"];
    }
    $GLOBALS['params'] = ['pokemon_id' => 13, 'site' => 2];
    rejected('api_move_pokemon', '背包已满（最多6只），请先将背包宠物放入仓库');
});
run_case('Concurrent site change before the final write rolls back', function () {
    $GLOBALS['params'] = ['pokemon_id' => 11, 'site' => 3];
    $expected = DB::$pets;
    $expected[1]['site'] = 3;
    // 锁内重读后、最终写入前，绕过行锁的写入（如后台直改）改变了该宠物的位置
    DB::$on_final = function () {
        foreach (DB::$pets as &$pet) {
            if ((int) $pet['id'] === 11) $pet['site'] = 3;
        }
    };
    call_endpoint('api_move_pokemon', 409, '宝可梦位置已变化，请刷新后重试');
    check(DB::$pets === $expected && DB::$writes === [], 'Failed guard still wrote site changes');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Failed guard did not roll back');
});
run_case('Move after a concurrent move sees the locked state', function () {
    $GLOBALS['params'] = ['pokemon_id' => 10, 'site' => 3];
    DB::$on_lock = function () {
        // 另一请求已提交：10 已被装箱且 11 补为首位
        DB::$pets[0]['site'] = 3;
        DB::$pets[1]['site'] = 1;
    };
    $data = call_endpoint('api_move_pokemon', 200);
    check($data['message'] === '宝可梦已在目标位置', 'Locked re-read did not steer the move');
    check(site_of(10) === 3 && site_of(11) === 1, 'Concurrent move state was disturbed');
    check(actives() === 1, 'Move after concurrent move broke the single-active rule');
    check(DB::$txn === ['START TRANSACTION', 'COMMIT'], 'No-op under lock did not commit');
});

run_case('Swap exchanges the two sites', function () {
    $GLOBALS['params'] = ['pokemon_id_1' => 10, 'pokemon_id_2' => 11];
    $data = call_endpoint('api_swap_pokemon', 200);
    check(site_of(10) === 2 && site_of(11) === 1, 'Sites not exchanged');
    check($data['old_site_1'] === 1 && $data['new_site_1'] === 2 && $data['new_site_2'] === 1, 'Swap response changed');
    check(actives() === 1, 'Swap left more than one active pet');
    check(DB::$txn === ['START TRANSACTION', 'COMMIT'], 'Swap did not commit');
});
run_case('Swapping two bag pets is a harmless no-op write', function () {
    $GLOBALS['params'] = ['pokemon_id_1' => 11, 'pokemon_id_2' => 12];
    call_endpoint('api_swap_pokemon', 200);
    check(site_of(11) === 2 && site_of(12) === 2 && actives() === 1, 'Bag swap changed positions unexpectedly');
});
run_case('Swapping a pet with itself is rejected', function () {
    $GLOBALS['params'] = ['pokemon_id_1' => 11, 'pokemon_id_2' => 11];
    call_endpoint('api_swap_pokemon', 400, '不能交换同一个宝可梦');
    check(DB::$txn === [], 'Self-swap still opened a transaction');
});
run_case('Swapping with a foreign pet is rejected before the lock', function () {
    DB::$pets[3]['uid'] = 8;
    $GLOBALS['params'] = ['pokemon_id_1' => 11, 'pokemon_id_2' => 13];
    call_endpoint('api_swap_pokemon', 403, '第二个宝可梦不存在或不属于您');
    check(DB::$txn === [], 'Foreign-pet swap still opened a transaction');
});

run_case('Set-first demotes the old first', function () {
    $GLOBALS['params'] = ['pokemon_id' => 11];
    $data = call_endpoint('api_set_first_pokemon', 200);
    check(site_of(11) === 1 && site_of(10) === 2, 'Set-first did not swap');
    check($data['old_site'] === 2 && $data['new_site'] === 1, 'Set-first response changed');
    check(actives() === 1, 'Set-first left more than one active pet');
    check(DB::$txn === ['START TRANSACTION', 'COMMIT'], 'Set-first did not commit');
});
run_case('Setting the current first is a no-op without a transaction', function () {
    $GLOBALS['params'] = ['pokemon_id' => 10];
    $data = call_endpoint('api_set_first_pokemon', 200);
    check($data['message'] === '该宝可梦已是首位', 'No-op response changed');
    check(DB::$txn === [], 'No-op set-first still opened a transaction');
});
run_case('Setting a boxed pet first is rejected', function () {
    $GLOBALS['params'] = ['pokemon_id' => 13];
    rejected('api_set_first_pokemon', '请先将宝可梦移出仓库再设为首位');
});
run_case('Concurrent demotion before promotion rolls back', function () {
    $GLOBALS['params'] = ['pokemon_id' => 11];
    $expected = DB::$pets;
    $expected[1]['site'] = 3;
    // 降位已执行、升位写入前，绕过行锁的写入把目标宠物装箱
    DB::$on_final = function () {
        foreach (DB::$pets as &$pet) {
            if ((int) $pet['id'] === 11) $pet['site'] = 3;
        }
    };
    call_endpoint('api_set_first_pokemon', 409, '宝可梦位置已变化，请刷新后重试');
    check(DB::$pets === $expected && DB::$writes === [], 'Failed guard still wrote site changes');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Failed guard did not roll back');
});
run_case('Second set-first under the lock stays consistent', function () {
    $GLOBALS['params'] = ['pokemon_id' => 11];
    DB::$on_lock = function () {
        // 另一请求已提交：11 已是首位
        DB::$pets[0]['site'] = 2;
        DB::$pets[1]['site'] = 1;
    };
    $data = call_endpoint('api_set_first_pokemon', 200);
    check($data['message'] === '该宝可梦已是首位', 'Locked re-read did not steer set-first');
    check(actives() === 1, 'Concurrent set-first broke the single-active rule');
});
run_case('Mid-transaction database error rolls back and rethrows', function () {
    $GLOBALS['params'] = ['pokemon_id' => 11, 'site' => 3];
    DB::$on_final = function () {
        throw new RuntimeException('simulated driver failure');
    };
    $snapshot = DB::$pets;
    $thrown = null;
    try {
        api_move_pokemon();
    } catch (RuntimeException $error) {
        $thrown = $error;
    }
    check($thrown !== null && $thrown->getMessage() === 'simulated driver failure', 'Driver failure was swallowed');
    check(DB::$pets === $snapshot && DB::$writes === [], 'Failed transaction left changes behind');
    check(DB::$txn === ['START TRANSACTION', 'ROLLBACK'], 'Failed transaction did not roll back');
});

echo "Party moves tests: $passed passed, $failed failed.\n";
exit($failed === 0 ? 0 : 1);
