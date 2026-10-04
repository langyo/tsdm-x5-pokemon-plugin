<?php
/**
 * Execute the capture endpoint against saved encounters and in-memory inventory.
 * Captures are guaranteed so the checks cover persisted shiny/gender attributes.
 * Run: php scripts/test/captured_pokemon_attributes.php
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

// Load actual endpoint functions without running the Discuz dispatcher. Preserve
// __DIR__ so the endpoint still loads its real pokemon_utils.php dependency.
$source = __DIR__ . '/../../plugin/api/battle.php';
define('IN_DISCUZ', 1);
require __DIR__ . '/../../plugin/api/battle_core.php';
require __DIR__ . '/../../plugin/api/pokemon_utils.php';

$wanted = ['api_capture_pokemon'];
$tokens = token_get_all(file_get_contents($source));
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
        $body .= is_array($token)
            ? ($token[0] === T_DIR ? var_export(dirname($source), true) : $token[1])
            : $token;
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

class CaptureResponse extends RuntimeException
{
    public $data;
    public function __construct($data) { $this->data = $data; }
}
function api_success($data) { throw new CaptureResponse($data); }
function api_error($message, ...$args) { throw new RuntimeException($message); }
function pm_abort_battle_transaction($message, $status = 400)
{
    DB::query('ROLLBACK');
    throw new RuntimeException($message);
}
function battle_ensure_tables() {}
function battle_load_active($uid, $myusersdata, $mypokemon)
{
    // build the engine state straight from the legacy fixture columns
    $allure = intval($myusersdata['allure']);
    return battle_core_initial_state([
        'uid' => intval($uid),
        'kind' => 'wild',
        'map_id' => 0,
        'rng_seed' => 1,
        'allies' => [[
            'instance_id' => 10, 'species_id' => 1, 'name' => 'Active', 'species_name' => 'Active',
            'level' => 20, 'hp' => 100,
            'stats' => ['max_hp' => 100, 'atk' => 20, 'def' => 20, 'spatk' => 20, 'spdef' => 20, 'speed' => 20],
            'types' => ['electric'],
        ]],
        'enemies' => [[
            'species_id' => intval($myusersdata['npcid']),
            'name' => 'Wild', 'species_name' => 'Wild',
            'level' => intval($myusersdata['level']),
            'hp' => intval($myusersdata['hp']),
            'stats' => [
                'max_hp' => intval($myusersdata['hpg']),
                'atk' => intval($myusersdata['atkg']), 'def' => intval($myusersdata['defg']),
                'spatk' => intval($myusersdata['spatkg']), 'spdef' => intval($myusersdata['spdefg']),
                'speed' => intval($myusersdata['sdg']),
            ],
            'types' => ['electric'],
            'gender' => ($allure >> 1) & 1,
            'is_shiny' => ($allure & 1) === 1,
            'capture_rate' => intval($myusersdata['capture']),
        ]],
    ]);
}
function battle_persist_state($state, $new_events = []) { return $state; }
function require_login() {}
function get_json_input() { return ['ball_id' => 1]; }
function pm_table($name) { return $name; }
function pm_sql($sql, ...$args)
{
    $index = 0;
    return preg_replace_callback('/%([sd])/', function ($match) use ($args, &$index) {
        $value = $args[$index++];
        return $match[1] === 'd' ? strval(intval($value)) : "'" . addslashes($value) . "'";
    }, $sql);
}
function api_my_usersdata($uid) { return $GLOBALS['user']; }
function api_my_pokemon($name) { return ['id' => 10, 'uid' => 7, 'level' => 20, 'hp' => 100, 'species_id' => 1]; }
function pm_data($id) { return $GLOBALS['species']; }
function clear_battle_state($uid) { $GLOBALS['user']['npcid'] = 0; }
function build_battle_response($user, $pet) { return []; }

class DB
{
    public static $captured;
    public static $balls;
    public static function fetch_first($sql)
    {
        if (strpos($sql, 'FOR UPDATE') !== false) {
            return ['uid' => 7];
        }
        if (strpos($sql, 'FROM pm_myitem') !== false) {
            return ['myitem_id' => 1, 'itemid' => 1, 'nums' => self::$balls, 'uid' => 7, 'captmax' => 255, 'ballid' => 1];
        }
        throw new RuntimeException('Unexpected read: ' . $sql);
    }
    public static function result_first($sql)
    {
        if (strpos($sql, 'SELECT COUNT(*) FROM pm_mypm WHERE uid = 7') === 0) return 1;
        throw new RuntimeException('Unexpected count: ' . $sql);
    }
    public static function query($sql)
    {
        if (preg_match('/^(START TRANSACTION|COMMIT|ROLLBACK)$/', $sql) || strpos($sql, 'CREATE TABLE') === 0) {
            return;
        }
        if (preg_match('/^(INSERT INTO pm_battle|UPDATE pm_battle|DELETE FROM pm_battle_unit|INSERT INTO pm_battle_event)/', $sql)) {
            return;
        }
        if ($sql === 'DELETE FROM pm_myitem WHERE id = 1') {
            self::$balls = 0;
        } elseif (preg_match('/^INSERT INTO pm_mypm\s*\(([^)]+)\)\s*VALUES\s*\(([^)]+)\)\s*$/s', $sql, $match)) {
            $columns = array_map('trim', explode(',', $match[1]));
            $values = array_map('trim', str_getcsv($match[2], ',', "'", '\\'));
            if (count($columns) !== count($values)) throw new RuntimeException('Capture INSERT columns and values disagree');
            // Match pm_mypm's default when the INSERT omits is_shiny.
            self::$captured = array_merge(['is_shiny' => 0], array_combine($columns, $values));
        } else {
            throw new RuntimeException('Unexpected write: ' . $sql);
        }
    }
}

function check($condition, $message) { if (!$condition) throw new RuntimeException($message); }
$passed = 0;
$failed = 0;
foreach ([0, 1] as $shiny) {
    foreach (['male' => 0, 'female' => 1, 'genderless' => 0] as $gender => $gender_bit) {
        $_G = ['uid' => 7, 'username' => 'player'];
        $GLOBALS['species'] = ['id' => 25, 'name' => 'Wild', 'sex' => $gender === 'genderless' ? -1 : 500, 'strength' => 1, 'xs' => 'electric'];
        $GLOBALS['user'] = [
            'uid' => 7, 'npcid' => 25, 'level' => 10, 'hp' => 100, 'hpg' => 100,
            'atkg' => 20, 'defg' => 20, 'spatkg' => 20, 'spdefg' => 20, 'sdg' => 20,
            'strength' => 1, 'boxnum' => 100, 'capture' => 255, 'allure' => strval(($gender_bit << 1) | $shiny),
        ];
        DB::$captured = null;
        DB::$balls = 1;
        srand(7);
        try {
            $response = null;
            try {
                api_capture_pokemon();
            } catch (CaptureResponse $result) {
                $response = $result->data;
            }
            check($response !== null && $response['status'] === 'captured', 'Guaranteed capture failed');
            check((int) DB::$captured['is_shiny'] === $shiny, 'Capture changed the encounter shiny flag');
            $expected_sex = $gender === 'genderless' ? 0 : $gender_bit + 1;
            check((int) DB::$captured['sex'] === $expected_sex, 'Capture changed the encounter gender');
            check((int) DB::$captured['uid'] === 7 && (int) DB::$captured['species_id'] === 25, 'Capture stored the wrong owner or species');
            check(DB::$balls === 0 && $GLOBALS['user']['npcid'] === 0 && $response['battle_over'] === true, 'Successful capture did not consume the ball and end battle');
            $passed++;
            echo "PASS Capture preserves $gender with shiny=$shiny\n";
        } catch (Throwable $error) {
            $failed++;
            echo "FAIL Capture preserves $gender with shiny=$shiny: {$error->getMessage()}\n";
        }
    }
}
echo "Captured Pokemon attribute tests: $passed passed, $failed failed.\n";
exit($failed === 0 ? 0 : 1);
