<?php
/** Real evolution handler: battle state must be checked after the account lock. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) { throw new ErrorException($message, 0, $severity, $file, $line); });
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/utils.php';
function load_evolution_functions($path, $wanted)
{
    $tokens = token_get_all(file_get_contents($path));
    for ($i = 0; $i < count($tokens); $i++) {
        if (!is_array($tokens[$i]) || $tokens[$i][0] !== T_FUNCTION) continue;
        $start = $i;
        while (++$i < count($tokens) && (!is_array($tokens[$i]) || $tokens[$i][0] !== T_STRING)) {}
        $name = $tokens[$i][1]; $source = ''; $depth = 0; $opened = false;
        for ($j = $start; $j < count($tokens); $j++) {
            $token = $tokens[$j]; $source .= is_array($token) ? $token[1] : $token;
            if ($token === '{' || (is_array($token) && in_array($token[0], [T_CURLY_OPEN, T_DOLLAR_OPEN_CURLY_BRACES], true))) {
                $depth++; $opened = true;
            } elseif ($token === '}' && --$depth === 0 && $opened) break;
        }
        if (in_array($name, $wanted, true)) eval($source);
        $i = $j;
    }
}
load_evolution_functions(__DIR__ . '/../../plugin/api/index.php', ['pm_sql', 'pm_sql_v', 'validate_id', 'validate_uid']);
load_evolution_functions($argv[1] ?? __DIR__ . '/../../plugin/api/evolution.php',
    ['api_evolve_pokemon', 'evaluate_evolution_routes', '_evaluate_evolution_rule_group', 'check_evolution_conditions', 'check_user_has_item', '_format_evolution_condition']);
class EvolutionResponse extends RuntimeException {}
function require_login() {}
function get_json_input() { return ['pet_id' => 1]; }
function api_success($data) { throw new EvolutionResponse('success', 200); }
function api_error($message, $status = 400) { throw new EvolutionResponse($message, $status); }
function pm_table($name) { return $name; }
class DB
{
    public static $user, $pet, $item, $rule, $snapshot, $race, $transaction;
    public static function reset($site = 1, $npcid = 0, $method = 'level')
    {
        $GLOBALS['_G'] = ['uid' => 7];
        self::$user = ['uid' => 7, 'npcid' => $npcid];
        self::$pet = ['id'=>1,'uid'=>7,'species_id'=>1,'site'=>$site,'state'=>1,'level'=>20,'good'=>100,
            'hp'=>10,'hpg'=>0,'hpn'=>0,'is_shiny'=>0,'equipmentid1'=>0,'equipmentid2'=>0,'equipmentid3'=>0,'equipmentid4'=>0];
        self::$item = ['id'=>3,'uid'=>7,'itemid'=>12,'nums'=>2];
        self::$rule = ['id'=>1,'from_id'=>1,'to_id'=>2,'method'=>$method,'condition_value'=>$method === 'item' ? '12' : '16'];
        self::$snapshot = self::$race = null; self::$transaction = false;
    }
    public static function fetch_first($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (preg_match('/^SELECT uid(?:, npcid)? FROM pm_usersdata WHERE uid = 7 FOR UPDATE$/', $sql)) {
            if (!self::$transaction) throw new RuntimeException('Account lock must be inside the transaction');
            if (self::$race) { $race = self::$race; self::$race = null; $race(); }
            self::$snapshot = [self::$pet, self::$item];
            return strpos($sql, 'uid, npcid') !== false ? self::$user : ['uid' => self::$user['uid']];
        }
        if ($sql === 'SELECT * FROM pm_mypm WHERE id = 1 AND uid = 7 FOR UPDATE') return self::$pet['uid'] === 7 ? self::$pet : false;
        if ($sql === 'SELECT * FROM pm_data WHERE id = 2') return ['id'=>2,'hp'=>80,'name'=>'Evolved'];
        if (preg_match('/^SELECT name FROM pm_itemdata WHERE id = \d+$/', $sql)) return ['name' => 'Evolution stone'];
        if (str_contains($sql, 'FROM pm_myitem') && str_contains($sql, "uid = 7 AND itemid = '12' AND nums > 0")) return self::$item;
        throw new RuntimeException('Unexpected SQL read: ' . $sql);
    }
    public static function fetch_all($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if ($sql === 'SELECT * FROM pm_evolution WHERE from_id = 1 ORDER BY priority, id') return [self::$rule];
        throw new RuntimeException('Unexpected SQL list: ' . $sql);
    }
    public static function query($sql)
    {
        if ($sql === 'START TRANSACTION') { self::$transaction = true; self::$snapshot = [self::$pet,self::$item]; return; }
        if ($sql === 'ROLLBACK' || $sql === 'COMMIT') {
            if ($sql === 'ROLLBACK' && self::$transaction) [self::$pet,self::$item] = self::$snapshot;
            self::$transaction = false; return;
        }
        if ($sql === 'UPDATE pm_myitem SET nums = nums - 1 WHERE id = 3 AND uid = 7 AND nums > 0') { self::$item['nums']--; return; }
        if (preg_match("/^UPDATE pm_mypm SET species_id = 2, hp = (\d+), pmname = 'Evolved', nickname = 'Evolved' WHERE id = 1 AND uid = 7$/", $sql, $m)) {
            self::$pet['species_id'] = 2; self::$pet['hp'] = (int)$m[1]; return;
        }
        throw new RuntimeException('Unexpected write: ' . $sql);
    }
    public static function affected_rows() { return 1; }
}
function evolve()
{
    try { api_evolve_pokemon(); } catch (EvolutionResponse $response) { return $response->getCode(); }
    throw new RuntimeException('Missing response');
}
$passed = $failed = 0;
function check($ok, $label)
{
    if ($ok) { $GLOBALS['passed']++; echo "PASS $label\n"; }
    else { $GLOBALS['failed']++; echo "FAIL $label\n"; }
}
foreach (['level','item','good'] as $method) {
    DB::reset(1,9,$method); $before = [DB::$pet,DB::$item];
    check(evolve() === 400 && [DB::$pet,DB::$item] === $before && !DB::$transaction,
        "Battle-active $method evolution preserves species, HP and inventory");
}
foreach ([2,3] as $site) {
    DB::reset($site,9,'item');
    check(evolve() === 200 && DB::$pet['species_id'] === 2 && DB::$item['nums'] === 1 && !DB::$transaction,
        "A non-active Pokemon at site $site can still evolve during a battle");
}
DB::reset();
check(evolve() === 200 && DB::$pet['species_id'] === 2 && DB::$pet['hp'] === 62 && !DB::$transaction,
    'The active Pokemon can evolve and receive normal HP outside battle');
DB::reset(); DB::$race = function () { DB::$user['npcid'] = 9; };
check(evolve() === 400 && DB::$pet['species_id'] === 1 && DB::$pet['hp'] === 10 && !DB::$transaction,
    'A battle started before acquiring the account lock prevents evolution');
DB::reset(2); DB::$race = function () { DB::$user['npcid'] = 9; DB::$pet['site'] = 1; };
check(evolve() === 400 && DB::$pet['site'] === 1 && DB::$pet['species_id'] === 1 && !DB::$transaction,
    'A concurrent switch to the battle-active slot is rechecked after locking');
DB::reset(); DB::$pet['uid'] = 8; $before = DB::$pet;
check(evolve() === 404 && DB::$pet === $before && !DB::$transaction, 'Another user Pokemon remains protected');
echo "Evolution battle guard: $passed passed, $failed failed.\n";
exit($failed ? 1 : 0);
