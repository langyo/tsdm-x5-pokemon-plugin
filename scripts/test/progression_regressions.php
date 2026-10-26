<?php
/** Execute evolution and skill-learning endpoints with the real HP/SQL helpers. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/utils.php';

function load_functions($path, $wanted)
{
    $tokens = token_get_all(file_get_contents($path));
    for ($i = 0; $i < count($tokens); $i++) {
        if (!is_array($tokens[$i]) || $tokens[$i][0] !== T_FUNCTION) continue;
        $start = $i;
        while (++$i < count($tokens) && (!is_array($tokens[$i]) || $tokens[$i][0] !== T_STRING)) {}
        $name = $tokens[$i][1];
        $body = '';
        $depth = 0;
        $opened = false;
        for ($j = $start; $j < count($tokens); $j++) {
            $token = $tokens[$j];
            $body .= is_array($token) ? $token[1] : $token;
            if ($token === '{' || (is_array($token) && in_array($token[0], [T_CURLY_OPEN, T_DOLLAR_OPEN_CURLY_BRACES], true))) {
                $opened = true;
                $depth++;
            } elseif ($token === '}' && --$depth === 0 && $opened) {
                break;
            }
        }
        if (in_array($name, $wanted, true)) eval($body);
        $i = $j;
    }
}
$api_root = __DIR__ . '/../../plugin/api/';
load_functions($api_root . 'index.php', ['pm_sql', 'pm_sql_v', 'validate_id', 'validate_uid', 'get_param']);
load_functions($api_root . 'pokemon.php', ['pokemon_can_learn_skill', 'api_learn_skill', 'api_get_learnable_skills']);
load_functions($api_root . 'evolution.php', ['api_check_evolution', 'api_evolve_pokemon', 'api_get_available_evolutions', 'evaluate_evolution_routes', '_evaluate_evolution_rule_group', '_build_available_evolution_entries', 'check_evolution_conditions', 'check_user_has_item', '_format_evolution_condition', 'calculate_max_hp']);

class ProgressionResponse extends RuntimeException
{
    public $data;
    public function __construct($message, $code, $data = null)
    {
        parent::__construct($message, $code);
        $this->data = $data;
    }
}
function api_success($data) { throw new ProgressionResponse('success', 200, $data); }
function api_error($message, $code = 400, $debug = null, $error_code = null) { throw new ProgressionResponse($message, $code, $error_code); }
function require_login() {}
function get_json_input() { return $GLOBALS['input']; }
function pm_table($name) { return $name; }

class DB
{
    public static $pet;
    public static $base;
    public static $evolution;
    public static $evolution_rules;
    public static $item;
    public static $skill;
    public static $learned;
    public static $user;
    public static $writes;
    public static $snapshot;
    public static $affected;
    public static $fail_update;
    public static $lose_item;

    public static function fetch_first($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (preg_match('/^SELECT \* FROM pm_mypm WHERE id = (\d+) AND uid = (\d+)(?: FOR UPDATE)?$/', $sql, $m)) {
            return self::$pet['id'] === (int) $m[1] && self::$pet['uid'] === (int) $m[2] ? self::$pet : false;
        }
        if (str_contains($sql, 'FROM pm_usersdata')) return self::$user;
        if (str_contains($sql, 'FROM pm_data')) return self::$base;
        if (str_contains($sql, 'SELECT i.equipment')) return ['equipment' => '{"hp":25}'];
        if (str_contains($sql, 'FROM pm_myitem')) {
            if (str_contains($sql, 'FOR UPDATE') && self::$lose_item) self::$item['nums'] = 0;
            if (!self::$item || !preg_match("/uid = (\d+) AND itemid = '(\d+)' AND nums > 0/", $sql, $m)) return false;
            return self::$item['uid'] === (int) $m[1] && self::$item['itemid'] === (int) $m[2] && self::$item['nums'] > 0 ? self::$item : false;
        }
        if (str_contains($sql, 'FROM pm_itemdata')) return ['name' => 'Evolution stone'];
        if (str_contains($sql, 'FROM pm_skill')) return self::$skill;
        if (str_contains($sql, 'FROM pm_myskill')) return self::$learned ? self::$learned[0] : false;
        throw new RuntimeException('Unexpected read: ' . $sql);
    }
    public static function fetch_all($sql)
    {
        if (str_contains($sql, 'FROM pm_myskill')) return self::$learned;
        if (str_contains($sql, 'FROM pm_skill')) return [self::$skill];
        if (str_contains($sql, 'FROM pm_evolution')) {
            $rules = self::$evolution_rules !== null ? self::$evolution_rules : (self::$evolution ? [self::$evolution] : []);
            preg_match('/from_id = (\d+)/', $sql, $m);
            $from_id = isset($m[1]) ? (int) $m[1] : null;
            $found = [];
            foreach ($rules as $rule) {
                if ($from_id === null || (int) $rule['from_id'] === $from_id) $found[] = $rule;
            }
            return $found;
        }
        throw new RuntimeException('Unexpected list: ' . $sql);
    }
    public static function result_first($sql)
    {
        if (str_contains($sql, 'COUNT(*) FROM pm_myskill')) return count(self::$learned);
        throw new RuntimeException('Unexpected scalar: ' . $sql);
    }
    public static function query($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        self::$affected = 0;
        if ($sql === 'START TRANSACTION') {
            self::$snapshot = [self::$pet, self::$item];
            return;
        }
        if ($sql === 'COMMIT') { self::$snapshot = null; return; }
        if ($sql === 'ROLLBACK') {
            if (self::$snapshot) [self::$pet, self::$item] = self::$snapshot;
            self::$snapshot = null;
            return;
        }
        self::$writes[] = $sql;
        if (preg_match('/^UPDATE pm_myitem SET nums = nums - 1 WHERE id = (\d+) AND uid = (\d+) AND nums > 0$/', $sql, $m)) {
            if (self::$item['id'] === (int) $m[1] && self::$item['uid'] === (int) $m[2] && self::$item['nums'] > 0) {
                self::$item['nums']--;
                self::$affected = 1;
            }
        } elseif (preg_match('/^UPDATE pm_mypm SET species_id = (\d+), hp = (\d+), pmname = (.+), nickname = (.+) WHERE id = (\d+)(?: AND uid = (\d+))?$/', $sql, $m)) {
            if (self::$fail_update) throw new RuntimeException('Simulated pet write failure');
            check(self::$pet['id'] === (int) $m[5], 'Wrong pet updated');
            if (isset($m[6])) check(self::$pet['uid'] === (int) $m[6], 'Wrong owner updated');
            self::$pet['species_id'] = (int) $m[1];
            self::$pet['hp'] = (int) $m[2];
            self::$pet['pmname'] = stripslashes(trim($m[3], "'"));
            self::$pet['nickname'] = stripslashes(trim($m[4], "'"));
            self::$affected = 1;
        } elseif (preg_match('/^INSERT INTO pm_myskill \(uid, petid, skillid, skillnum\) VALUES \((\d+), (\d+), (\d+), (\d+)\)$/', $sql, $m)) {
            self::$learned[] = ['uid' => (int) $m[1], 'petid' => (int) $m[2], 'skillid' => (int) $m[3], 'skillnum' => (int) $m[4]];
            self::$affected = 1;
        } else throw new RuntimeException('Unexpected write: ' . $sql);
    }
    public static function affected_rows() { return self::$affected; }
}
function fixture()
{
    $GLOBALS['_G'] = ['uid' => 7];
    $_GET = ['pet_id' => 3, 'petid' => 3, 'pokemon_id' => 3];
    $_POST = [];
    $GLOBALS['input'] = ['pet_id' => 3, 'pokemon_id' => 3, 'skill_id' => 9];
    DB::$pet = ['id' => 3, 'uid' => 7, 'species_id' => 1, 'level' => 50, 'good' => 100, 'hp' => 100, 'site' => 1,
        'hpg' => 0, 'hpn' => 0, 'state' => 1, 'is_shiny' => 0, 'equipmentid1' => 0];
    DB::$base = ['id' => 2, 'name' => 'Evolved', 'hp' => 80];
    DB::$evolution = ['id' => 4, 'from_id' => 1, 'to_id' => 2, 'method' => 'item', 'condition_value' => '12'];
    DB::$evolution_rules = null;
    DB::$item = ['id' => 8, 'uid' => 7, 'itemid' => 12, 'nums' => 2];
    DB::$skill = ['id' => 9, 'name' => 'Test skill', 'available_pokemons' => 'k,1,10,k', 'level_required' => 5,
        'max_uses' => 20, 'element' => 'normal', 'category' => 'physical', 'power' => 20, 'description' => 'Test'];
    DB::$learned = [];
    DB::$user = ['uid' => 7, 'npcid' => 0];
    DB::$writes = [];
    DB::$snapshot = null;
    DB::$affected = 0;
    DB::$fail_update = false;
    DB::$lose_item = false;
}
function check($condition, $message) { if (!$condition) throw new RuntimeException($message); }
function endpoint($name, $expected_code = 200)
{
    try { $name(); } catch (ProgressionResponse $response) {
        check($response->getCode() === $expected_code, 'Unexpected response: ' . $response->getMessage());
        return $response->data;
    }
    throw new RuntimeException('Endpoint did not respond');
}
$passed = 0;
$failed = 0;
function run_case($name, $body)
{
    global $passed, $failed;
    fixture();
    try { $body(); $passed++; echo "PASS $name\n"; }
    catch (Throwable $e) { $failed++; echo "FAIL $name: {$e->getMessage()}\n"; }
}

foreach (['k,10,11,21,k', '11', '101', 'k,21,k where species=1'] as $ids) {
    run_case("Reject substring-only species match in $ids", function () use ($ids) {
        DB::$skill['available_pokemons'] = $ids;
        $list = endpoint('api_get_learnable_skills');
        check($list['available_skills'] === [], 'List offered a foreign species skill');
        endpoint('api_learn_skill', 400);
        check(DB::$writes === [] && DB::$learned === [], 'Rejected skill changed the database');
    });
}
foreach (['k,1,10,k', '1', '99999', '1,2', 'k, 1, 10,k'] as $ids) {
    run_case("Learn exact/global species match in $ids", function () use ($ids) {
        DB::$skill['available_pokemons'] = $ids;
        $list = endpoint('api_get_learnable_skills');
        check(count($list['available_skills']) === 1, 'List did not offer a learnable skill');
        endpoint('api_learn_skill');
        check(count(DB::$learned) === 1 && DB::$learned[0]['skillnum'] === 20, 'Skill not learned with full PP');
    });
}
run_case('Learning retains level and battle restrictions', function () {
    DB::$pet['level'] = 1;
    endpoint('api_learn_skill', 400);
    DB::$pet['level'] = 50;
    DB::$user['npcid'] = 25;
    endpoint('api_learn_skill', 400);
    check(DB::$writes === [], 'Restricted skill request wrote data');
});
foreach (['pet_id', 'petid'] as $parameter) {
    run_case("Evolution check accepts $parameter and returns the typed response", function () use ($parameter) {
        $_GET = [$parameter => 3];
        $result = endpoint('api_check_evolution');
        foreach (['can_evolve', 'pokemon_id', 'current_form', 'target_form', 'conditions', 'evolution_method', 'reason'] as $key) {
            check(array_key_exists($key, $result), "Missing response field $key");
        }
        check($result['can_evolve'] && $result['target_form'] === 2, 'Wrong evolution check');
        $available = endpoint('api_get_available_evolutions');
        check(count($available['available_evolutions']) === 1, 'Available endpoint rejected parameter');
    });
}
run_case('Terminal species returns the typed negative response', function () {
    DB::$evolution = false;
    $result = endpoint('api_check_evolution');
    foreach (['can_evolve', 'pokemon_id', 'current_form', 'target_form', 'conditions', 'evolution_method', 'reason'] as $key) {
        check(array_key_exists($key, $result), "Missing negative response field $key");
    }
    check(!$result['can_evolve'] && !$result['conditions']['can_evolve'], 'Terminal species can evolve');
});
foreach (['absent', 'empty', 'foreign', 'wrong_type'] as $kind) {
    run_case("Reject $kind evolution item", function () use ($kind) {
        if ($kind === 'absent') DB::$item = false;
        if ($kind === 'empty') DB::$item['nums'] = 0;
        if ($kind === 'foreign') DB::$item['uid'] = 8;
        if ($kind === 'wrong_type') DB::$item['itemid'] = 99;
        check(!endpoint('api_check_evolution')['can_evolve'], 'Invalid item satisfied the check');
        endpoint('api_evolve_pokemon', 400);
        check(DB::$pet['species_id'] === 1 && DB::$writes === [], 'Rejected evolution changed data');
        check(DB::$snapshot === null, 'Error left a transaction open');
    });
}
run_case('Evolution consumes exactly one item and duplicate request cannot spend another', function () {
    endpoint('api_evolve_pokemon');
    check(DB::$pet['species_id'] === 2 && DB::$item['nums'] === 1, 'Evolution did not spend one stone');
    endpoint('api_evolve_pokemon', 400);
    check(DB::$item['nums'] === 1, 'Duplicate request consumed another stone');
});
run_case('Failed pet write rolls back item consumption', function () {
    DB::$fail_update = true;
    try { endpoint('api_evolve_pokemon'); }
    catch (RuntimeException $e) { check($e->getMessage() === 'Simulated pet write failure', 'Unexpected failure'); }
    check(DB::$pet['species_id'] === 1 && DB::$item['nums'] === 2 && DB::$snapshot === null, 'Failed evolution was not rolled back');
});
run_case('Missing target species rejects without spending the item', function () {
    DB::$base = false;
    endpoint('api_evolve_pokemon', 500);
    check(DB::$pet['species_id'] === 1 && DB::$item['nums'] === 2, 'Missing target changed pet or inventory');
    check(DB::$writes === [] && DB::$snapshot === null, 'Missing target left writes or transaction');
});
run_case('Item lost between check and reservation prevents evolution', function () {
    DB::$lose_item = true;
    endpoint('api_evolve_pokemon', 409);
    check(DB::$pet['species_id'] === 1, 'Evolved without reserving an item');
    check(DB::$snapshot === null, 'Race left a transaction open');
});
foreach ([[0, 0, 1, 0, 0], [31, 80, 8, 1, 1], [7, 40, 4, 0, 1]] as $stats) {
    run_case('Evolution uses persisted stats ' . implode('/', $stats), function () use ($stats) {
        [DB::$pet['hpg'], DB::$pet['hpn'], DB::$pet['state'], DB::$pet['is_shiny'], DB::$pet['equipmentid1']] = $stats;
        DB::$evolution['method'] = 'level';
        DB::$evolution['condition_value'] = '16';
        $expected = api_calculate_pokemon_max_hp(DB::$pet, DB::$base);
        $result = endpoint('api_evolve_pokemon');
        check(DB::$pet['hp'] === $expected && $result['stats']['hp'] === $expected, 'Evolution HP ignores actual pet stats');
        check(DB::$item['nums'] === 2, 'Level evolution consumed an item');
    });
}
run_case('Riolu-style maxed intimacy evolution', function () {
    DB::$evolution = ['id' => 4, 'from_id' => 1, 'to_id' => 2, 'method' => 'good', 'condition_value' => '220'];
    DB::$pet['good'] = 255;
    $result = endpoint('api_check_evolution');
    check($result['can_evolve'] && $result['target_form'] === 2, 'Maxed intimacy did not qualify the check');
    endpoint('api_evolve_pokemon');
    check(DB::$pet['species_id'] === 2 && DB::$item['nums'] === 2, 'Intimacy evolution consumed an item');
});
run_case('Check falls through to a satisfiable later route', function () {
    DB::$evolution_rules = [
        ['id' => 4, 'from_id' => 1, 'to_id' => 2, 'method' => 'item', 'condition_value' => '12'],
        ['id' => 5, 'from_id' => 1, 'to_id' => 3, 'method' => 'good', 'condition_value' => '80'],
    ];
    DB::$item = false;
    $result = endpoint('api_check_evolution');
    check($result['can_evolve'] && $result['target_form'] === 3 && $result['evolution_method'] === 'good',
        'Check did not fall through to the satisfiable later route');
    endpoint('api_evolve_pokemon');
    check(DB::$pet['species_id'] === 3 && DB::$item === false, 'Evolution touched the unavailable first route');
    DB::$pet['good'] = 10;
    DB::$pet['species_id'] = 1;
    DB::$writes = [];
    $negative = endpoint('api_check_evolution');
    check(!$negative['can_evolve'] && $negative['reason'] !== '', 'Unsatisfiable routes still reported a check pass');
});
run_case('Compound route requires every rule', function () {
    DB::$evolution_rules = [
        ['id' => 4, 'from_id' => 1, 'to_id' => 2, 'method' => 'level', 'condition_value' => '20'],
        ['id' => 5, 'from_id' => 1, 'to_id' => 2, 'method' => 'good', 'condition_value' => '80'],
    ];
    $result = endpoint('api_check_evolution');
    check($result['can_evolve'] && $result['target_form'] === 2, 'Compound route ignored a satisfied rule');
    endpoint('api_evolve_pokemon');
    check(DB::$pet['species_id'] === 2 && DB::$item['nums'] === 2, 'Compound route evolution failed');
    DB::$pet['good'] = 10;
    DB::$pet['species_id'] = 1;
    DB::$writes = [];
    check(!endpoint('api_check_evolution')['can_evolve'], 'Compound route ignored the intimacy rule');
    endpoint('api_evolve_pokemon', 400);
    check(DB::$writes === [] && DB::$pet['species_id'] === 1, 'Unsatisfied compound route changed data');
});
run_case('Unsupported rule inside a compound route blocks it', function () {
    DB::$evolution_rules = [
        ['id' => 4, 'from_id' => 1, 'to_id' => 2, 'method' => 'level', 'condition_value' => '20'],
        ['id' => 5, 'from_id' => 1, 'to_id' => 2, 'method' => 'sex', 'condition_value' => 'male'],
    ];
    $result = endpoint('api_check_evolution');
    check(!$result['can_evolve'] && str_contains($result['reason'], 'Unsupported'), 'Compound route hid the unsupported rule');
    endpoint('api_evolve_pokemon', 400);
    check(DB::$writes === [] && DB::$pet['species_id'] === 1, 'Unsupported compound rule evolved the pet');
});
run_case('Explicit target selection', function () {
    $routes = [
        ['id' => 4, 'from_id' => 1, 'to_id' => 2, 'method' => 'level', 'condition_value' => '10'],
        ['id' => 5, 'from_id' => 1, 'to_id' => 3, 'method' => 'level', 'condition_value' => '10'],
    ];
    DB::$evolution_rules = $routes;
    $GLOBALS['input']['to_id'] = 3;
    endpoint('api_evolve_pokemon');
    check(DB::$pet['species_id'] === 3, 'Explicit to_id was not honored');
    fixture();
    DB::$evolution_rules = $routes;
    $GLOBALS['input']['to_id'] = 99;
    endpoint('api_evolve_pokemon', 400);
    check(DB::$writes === [] && DB::$pet['species_id'] === 1, 'Unknown target still evolved or wrote data');
    unset($GLOBALS['input']['to_id']);
    $targets = array_column(endpoint('api_check_evolution')['available_evolutions'], 'to_id');
    check(in_array(2, $targets, true) && in_array(3, $targets, true), 'Check hid an available route');
});
run_case('Single intimacy condition still allows qualifying evolution', function () {
    DB::$evolution['method'] = 'good';
    DB::$evolution['condition_value'] = '80';
    endpoint('api_evolve_pokemon');
    check(DB::$pet['species_id'] === 2 && DB::$item['nums'] === 2, 'Single intimacy evolution regressed');
});
foreach (['comp_atk_def', 'sex', 'random', 'is_have_chairs', 'is_exchange', 'unknown'] as $method) {
    run_case("Unsupported $method rule cannot bypass evolution conditions", function () use ($method) {
        DB::$evolution['method'] = $method;
        check(!endpoint('api_check_evolution')['can_evolve'], 'Check accepted an unevaluated rule');
        endpoint('api_evolve_pokemon', 400);
        check(DB::$writes === [] && DB::$pet['species_id'] === 1, 'Unsupported rule evolved the pet');
    });
}
run_case('Evolution validates owner and minimum level', function () {
    DB::$pet['uid'] = 8;
    endpoint('api_evolve_pokemon', 404);
    DB::$pet['uid'] = 7;
    DB::$evolution['method'] = 'level';
    DB::$evolution['condition_value'] = '60';
    endpoint('api_evolve_pokemon', 400);
    check(DB::$writes === [] && DB::$snapshot === null, 'Rejected evolution changed data or left transaction open');
});
echo "Progression regressions: $passed passed, $failed failed.\n";
exit($failed === 0 ? 0 : 1);
