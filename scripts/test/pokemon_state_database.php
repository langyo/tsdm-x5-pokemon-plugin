<?php
/** Execute the real random-state handler and SQL against a disposable MariaDB database. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) { throw new ErrorException($message, 0, $severity, $file, $line); });
define('IN_DISCUZ', true);

function load_state_functions($path, $wanted)
{
    $tokens = token_get_all(file_get_contents($path));
    for ($i = 0; $i < count($tokens); $i++) {
        if (!is_array($tokens[$i]) || $tokens[$i][0] !== T_FUNCTION) continue;
        $start = $i;
        while (++$i < count($tokens) && (!is_array($tokens[$i]) || $tokens[$i][0] !== T_STRING)) {}
        $name = $tokens[$i][1];
        $depth = 0; $body = ''; $opened = false;
        for ($j = $start; $j < count($tokens); $j++) {
            $token = $tokens[$j];
            $body .= is_array($token) ? $token[1] : $token;
            if ($token === '{' || (is_array($token) && in_array($token[0], [T_CURLY_OPEN, T_DOLLAR_OPEN_CURLY_BRACES], true))) {
                $depth++; $opened = true;
            } elseif ($token === '}' && --$depth === 0 && $opened) break;
        }
        if (in_array($name, $wanted, true)) eval($body);
        $i = $j;
    }
    foreach ($wanted as $name) if (!function_exists($name)) throw new RuntimeException('Missing function: ' . $name);
}
load_state_functions(__DIR__ . '/../../plugin/api/index.php', ['pm_sql', 'pm_sql_v', 'pm_table', 'require_login', 'validate_uid']);
load_state_functions(__DIR__ . '/../../plugin/api/utils.php', ['pm_abort_battle_transaction']);
load_state_functions($argv[1] ?? __DIR__ . '/../../plugin/api/pokemon.php',
    ['api_update_pokemon_state', 'get_state_multipliers', 'get_pokemon_state_text', 'get_pokemon_state_class']);

class StateResponse extends RuntimeException
{
    public $data;
    public function __construct($status, $data) { parent::__construct('response', $status); $this->data = $data; }
}
function api_success($data) { throw new StateResponse(200, $data); }
function api_error($message, $status = 400) { throw new StateResponse($status, $message); }
class DB
{
    public static $connection;
    public static $fail_after_write = false;
    public static $queries = [];
    public static function query($sql)
    {
        self::$queries[] = $sql;
        $result = self::$connection->query($sql);
        if (self::$fail_after_write && strpos($sql, 'UPDATE pm_mypm SET state =') === 0) {
            throw new RuntimeException('Injected failure after state write');
        }
        return $result;
    }
    public static function fetch_first($sql) { return self::query($sql)->fetch_assoc(); }
}
function state_request($seed)
{
    srand($seed);
    try { api_update_pokemon_state(); }
    catch (StateResponse $response) { return $response; }
    throw new RuntimeException('Handler did not respond');
}
$passed = 0;
function check($condition, $label)
{
    if (!$condition) throw new RuntimeException($label);
    $GLOBALS['passed']++;
    echo 'PASS ', $label, PHP_EOL;
}

mysqli_report(MYSQLI_REPORT_ERROR | MYSQLI_REPORT_STRICT);
$db = DB::$connection = new mysqli(getenv('TSDM_DB_HOST') ?: '127.0.0.1', getenv('TSDM_DB_USER') ?: 'root',
    getenv('TSDM_DB_PASSWORD') ?: '', '', (int)(getenv('TSDM_DB_PORT') ?: 3306));
$database = 'tsdm_test_state_' . bin2hex(random_bytes(8));
$db->query("CREATE DATABASE `$database` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci");
try {
    $db->select_db($database);
    $db->multi_query(file_get_contents(__DIR__ . '/../../docker/init.d/02-pokemon-schema.sql'));
    do { if ($result = $db->store_result()) $result->free(); } while ($db->more_results() && $db->next_result());
    $db->query("SET SESSION sql_mode = 'STRICT_TRANS_TABLES'");
    $db->query('INSERT INTO pm_usersdata (uid, npcid) VALUES (7, 0)');
    $db->query("INSERT INTO pm_mypm (id, uid, species_id, state, hp, exp, site) VALUES (501,7,1,2,80,100,1),(502,8,1,2,70,100,1)");
    $_G = ['uid' => 7, 'timestamp' => 1800000000];
    // Pin both the PRNG seed and its branch: tests must not silently miss the UPDATE.
    srand(3); check(rand(1, 100) === 87, 'Seed 3 selects recovery to normal');
    srand(0); check(rand(1, 100) === 45, 'Seed 0 selects worsening and fainting branches');
    foreach ([[2,100,80,3,1,100,80], [3,100,80,0,4,50,80], [3,20,80,0,4,20,80], [4,100,80,0,0,100,1]] as $case) {
        [$state,$exp,$hp,$seed,$next,$expected_exp,$expected_hp] = $case;
        $db->query("UPDATE pm_mypm SET state=$state, exp=$exp, hp=$hp, statetime=1 WHERE id=501");
        $response = state_request($seed);
        $pet = DB::fetch_first('SELECT state, exp, hp, statetime FROM pm_mypm WHERE id=501');
        check($response->getCode() === 200 && $response->data['state_changed'] && $response->data['new_state'] === $next
            && (int)$pet['state'] === $next && (int)$pet['exp'] === $expected_exp && (int)$pet['hp'] === $expected_hp
            && (int)$pet['statetime'] === $_G['timestamp'], "State $state to $next stores the expected state, EXP, HP and timestamp (EXP $exp)");
    }
    check(DB::fetch_first('SELECT state, exp, hp, statetime FROM pm_mypm WHERE id=502') ===
        ['state'=>'2','exp'=>'100','hp'=>'70','statetime'=>'0'], 'State updates leave another owner untouched');
    $db->query('UPDATE pm_mypm SET state=3, exp=100, hp=80, statetime=123 WHERE id=501');
    DB::$fail_after_write = true;
    try { state_request(0); throw new RuntimeException('Injected failure was not triggered'); }
    catch (RuntimeException $error) { check($error->getMessage() === 'Injected failure after state write', 'Write failure propagates'); }
    finally { DB::$fail_after_write = false; }
    check(DB::fetch_first('SELECT state, exp, hp, statetime FROM pm_mypm WHERE id=501') ===
        ['state'=>'3','exp'=>'100','hp'=>'80','statetime'=>'123'], 'Failed state update rolls back state, EXP, HP and timestamp');
    $db->query('UPDATE pm_usersdata SET npcid=2 WHERE uid=7');
    $db->query("INSERT INTO pm_battle (id,uid,phase,field_json) VALUES (99,7,'active','{}')");
    $db->query("INSERT INTO pm_battle_unit (battle_id,side,instance_id,species_id,stats_json,types_json,hp,stages_json,status_json,volatile_json,buffs_json,effects_json)
        VALUES (99,'ally',501,1,'{}','[]',80,'{}','{}','{}','[]','[]')");
    $db->query('UPDATE pm_mypm SET state=4 WHERE id=501');
    DB::$queries = [];
    $response = state_request(0);
    check($response->getCode() === 400 && (int)DB::fetch_first('SELECT state FROM pm_mypm WHERE id=501')['state'] === 4
        && (int)DB::fetch_first('SELECT hp FROM pm_mypm WHERE id=501')['hp'] === 80
        && DB::fetch_first('SELECT phase FROM pm_battle WHERE id=99')['phase'] === 'active'
        && (int)DB::fetch_first("SELECT hp FROM pm_battle_unit WHERE battle_id=99 AND side='ally'")['hp'] === 80,
        'Active battle rejects random state changes and preserves both pet and engine snapshot');
    check(strpos(DB::$queries[1], 'FROM pm_usersdata WHERE uid = 7 FOR UPDATE') !== false
        && DB::$queries[2] === 'ROLLBACK', 'Battle rejection happens while the account lock is held');
    $db->query('UPDATE pm_usersdata SET npcid=0 WHERE uid=7');
    DB::$queries = [];
    state_request(0);
    check(strpos(DB::$queries[1], 'FROM pm_usersdata WHERE uid = 7 FOR UPDATE') !== false
        && strpos(DB::$queries[2], 'FROM pm_mypm WHERE uid = 7 AND site = 1 FOR UPDATE') !== false
        && end(DB::$queries) === 'COMMIT', 'Outside battle locks account then active Pokemon and commits the state update');
    $db->query('UPDATE pm_mypm SET state=0, hp=0, statetime=123 WHERE id=501');
    $response = state_request(0);
    check(!$response->data['state_changed'] && (int)DB::fetch_first('SELECT statetime FROM pm_mypm WHERE id=501')['statetime'] === 123,
        'Fainted Pokemon do not receive a random update');
    $db->query('UPDATE pm_mypm SET species_id=0, state=1 WHERE id=501');
    $response = state_request(0);
    check(!$response->data['state_changed'], 'Eggs do not receive a random update');
    $db->query('UPDATE pm_mypm SET site=3 WHERE id=501');
    check(state_request(0)->getCode() === 404, 'Missing active Pokemon is rejected');
    echo "Pokemon state database checks passed: $passed", PHP_EOL;
} finally {
    // The name is generated in this process and never comes from caller or site configuration.
    if (!preg_match('/^tsdm_test_state_[a-f0-9]{16}$/D', $database)) throw new RuntimeException('Unsafe database name');
    $db->query("DROP DATABASE `$database`");
    $db->close();
}
