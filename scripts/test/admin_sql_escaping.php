<?php
/** Hostile admin payloads must reach SQL only as quoted literals or integers. */
error_reporting(E_ALL & ~E_WARNING & ~E_NOTICE & ~E_DEPRECATED);

class DB
{
    public static $log = [];
    public static $row = [
        'id' => 1, 'name' => 'old', 'site' => 'l', 'is_enabled' => 0, 'min_level' => 1, 'max_level' => 2,
        'experience' => 1, 'boss_config' => '', 'description' => 'old', 'money' => 0, 'shop' => 0, 'sex' => 0,
        'hp' => 0, 'atk' => 0, 'def' => 0, 'spatk' => 0, 'spdef' => 0, 'speed' => 0, 'xs' => '', 'xs2' => '',
        'is_legendary' => 0, 'mapid' => '', 'capture' => 0, 'met' => 0, 'birth' => 0, 'strength' => 0,
        'drop_money' => '[0,0]', 'effort_values' => '{}', 'tpname' => 'old', 'type' => 1, 'sitemname' => 'old',
        'level_required' => 0, 'max_uses' => 0, 'category' => '', 'element' => '', 'power' => 0, 'effect_id' => 0,
        'available_pokemons' => 'k,k', 'from_id' => 0, 'to_id' => 0, 'method' => 'level', 'condition_value' => '1',
        'priority' => 0, 'code' => 'x', 'kind' => 'x',
    ];

    public static function table($name) { return $name; }
    public static function fetch_first($sql)
    {
        self::$log[] = $sql;
        if (stripos($sql, 'SHOW COLUMNS') === 0) return ['Type' => 'text'];
        if (stripos($sql, 'SELECT') === 0) return self::$row;
        return false;
    }
    public static function fetch_all($sql) { self::$log[] = $sql; return []; }
    public static function result_first($sql) { self::$log[] = $sql; return 0; }
    public static function query($sql) { self::$log[] = $sql; return true; }
    public static function insert_id() { return 7; }
    public static function errno() { return 0; }
}

function api_success($data) { exit(json_encode(['success' => true, 'data' => $data], JSON_UNESCAPED_UNICODE)); }
function api_error($message, $code = 400) { throw new RuntimeException($message, $code); }
function get_param($name, $default = null) { return $_GET[$name] ?? $_POST[$name] ?? $default; }

const INJECT = "1' OR '1'='1";
const NUM_INJECT = '1 OR 1=1';

function cases()
{
    $stat = ['hit_points' => 1, 'attack' => 1, 'defense' => 1, 'special_attack' => 1, 'special_defense' => 1, 'speed' => 1];
    return [
        'set_map_info' => ['id' => NUM_INJECT, 'name' => 'new', 'area_type' => 'g', 'is_enabled' => true,
            'min_level' => '5, name=0x41', 'max_level' => '9 OR 1=1'],
        'insert_map_info' => ['name' => 'new', 'area_type' => 'g', 'is_enabled' => true, 'min_level' => 1, 'max_level' => 9],
        'set_pokemon_type' => ['id' => NUM_INJECT, 'name' => 'new', 'description' => 'new', 'cost' => 5,
            'is_selling' => true, 'sex_weight' => 0.5, 'initial_statistic' => $stat, 'effort_values' => $stat,
            'kind' => ['fire', 'water'], 'is_legendary' => true, 'map_ids' => [1, 2], 'capture_weight' => 1,
            'meet_weight' => 1, 'birth_order' => 1, 'strength_weight' => 1, 'drop_money' => [1, 2]],
        'set_skill_type' => ['id' => 1, 'name' => 'new', 'description' => 'new', 'min_level_limit' => INJECT,
            'use_times_limit' => INJECT, 'available_pokemons' => [INJECT, 2], 'effect' => ['physical_damage' => ['normal', 40]]],
        'insert_skill_type' => ['name' => 'new', 'description' => 'new', 'min_level_limit' => 1, 'use_times_limit' => 1,
            'available_pokemons' => [INJECT, 2], 'effect' => ['physical_damage' => ['normal', 40]]],
        'set_item_type' => ['id' => 1, 'name' => 'new', 'img_name' => INJECT, 'description' => 'new', 'is_selling' => true,
            'price' => 1, 'tag' => ['special' => INJECT], 'limits' => ['min_level' => 1, 'kind_require' => 'fire'],
            'effects' => []],
        'insert_item_type' => ['name' => 'new', 'img_name' => INJECT, 'description' => 'new', 'is_selling' => true,
            'price' => 1, 'tag' => ['special' => INJECT], 'limits' => ['min_level' => 1, 'kind_require' => 'fire'],
            'effects' => []],
        'set_evolution_info' => ['id' => 1, 'source_id' => 2, 'target_id' => 3, 'priority' => 1,
            'condition' => ['min_level' => INJECT]],
        'insert_evolution_info' => ['source_id' => 2, 'target_id' => 3, 'priority' => 1, 'condition' => ['use_item' => INJECT]],
    ];
}

if (isset($argv[1])) {
    define('IN_DISCUZ', true);
    $_G = [];
    $_GET = $_POST = [];
    register_shutdown_function(function () { fwrite(STDERR, json_encode(DB::$log, JSON_UNESCAPED_UNICODE)); });
    require __DIR__ . '/../../plugin/admin/routes.php';
    $fn = $argv[1];
    $fn(cases()[$fn]);
    exit(0);
}

$passed = 0;
$failed = 0;
foreach (array_keys(cases()) as $fn) {
    $proc = proc_open([PHP_BINARY, __FILE__, $fn], [1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes);
    $out = stream_get_contents($pipes[1]);
    $err = stream_get_contents($pipes[2]);
    proc_close($proc);
    $log = json_decode(substr($err, strrpos($err, '[') === false ? 0 : strpos($err, '[')), true) ?: [];
    $writes = array_values(array_filter($log, function ($sql) { return preg_match('/^\s*(UPDATE|INSERT|DELETE)/i', $sql); }));
    $bad = [];
    foreach ($log as $sql) {
        // 去掉正确转义的字串字面量后，剩下的不能再有注入的 OR、注释或语句分隔
        $bare = preg_replace("/'(?:\\\\.|[^'\\\\])*'/s", "''", $sql);
        if (preg_match('/\bOR\b|--|#|;|0x41/i', $bare) || preg_match('/where\s+`?id`?\s*=\s*(?!\d+\s*$)(?!\'\d+\'\s*$)/i', $bare)) {
            $bad[] = $sql;
        }
    }
    if ($writes && !$bad) {
        $passed++;
        echo "PASS {$fn}: " . count($writes) . " writes, payloads stayed inside literals\n";
    } else {
        $failed++;
        echo "FAIL {$fn}: " . ($writes ? 'injected SQL ' . json_encode($bad, JSON_UNESCAPED_UNICODE) : 'no writes; output ' . trim($out)) . "\n";
    }
}
echo "Admin SQL escaping: {$passed} passed, {$failed} failed\n";
exit($failed ? 1 : 0);
