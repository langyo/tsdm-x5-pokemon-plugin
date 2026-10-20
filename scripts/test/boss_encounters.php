<?php
/** Configured Boss variants and IVs, using the real encounter/map/stat functions. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) { throw new ErrorException($message, 0, $severity, $file, $line); });
define('IN_DISCUZ', 1);
require __DIR__ . '/../../plugin/api/pokemon_utils.php';
foreach (['boss.php' => ['get_map_boss_config_from_map', 'try_spawn_boss_from_config'],
    'battle.php' => ['generate_wild_pokemon_legacy', 'battle_calc_new_npc_stats', 'api_get_maps', 'translate_map_alpha_to_full_name']] as $file => $wanted) {
    $tokens = token_get_all(file_get_contents(__DIR__ . '/../../plugin/api/' . $file));
    for ($i = 0; $i < count($tokens); $i++) {
        if (!is_array($tokens[$i]) || $tokens[$i][0] !== T_FUNCTION) continue;
        $start = $i;
        while (++$i < count($tokens) && (!is_array($tokens[$i]) || $tokens[$i][0] !== T_STRING)) {}
        $name = $tokens[$i][1]; $depth = 0; $opened = false; $body = '';
        for ($j = $start; $j < count($tokens); $j++) {
            $token = $tokens[$j]; $body .= is_array($token) ? $token[1] : $token;
            if ($token === '{' || (is_array($token) && in_array($token[0], [T_CURLY_OPEN, T_DOLLAR_OPEN_CURLY_BRACES], true))) { $depth++; $opened = true; }
            elseif ($token === '}' && --$depth === 0 && $opened) break;
        }
        if (in_array($name, $wanted, true)) eval($body);
        $i = $j;
    }
}
class BossResponse extends RuntimeException { public $data; public function __construct($data) { $this->data = $data; } }
function api_success($data) { throw new BossResponse($data); }
function api_error($message, $code = 400) { throw new RuntimeException($message, $code); }
function require_login() {}
function get_param($key, $default = null) { return $default; }
function pm_table($name) { return $name; }
function pm_sql($sql, ...$args) { return vsprintf($sql, $args); }
function pm_sql_v($sql, $args) { return vsprintf($sql, $args); }
class DB {
    public static $map;
    public static $species = ['id' => 150, 'name' => 'Fixture Boss', 'capture' => 30, 'strength' => 1, 'met' => 101,
        'hp' => 100, 'atk' => 100, 'def' => 100, 'spatk' => 100, 'spdef' => 100, 'speed' => 100, 'mapid' => '999'];
    public static function fetch_first($sql) { return strpos($sql, 'SHOW COLUMNS') === 0 ? ['Field' => 'region'] : self::$species; }
    public static function fetch_all($sql) { return strpos($sql, 'FROM pm_map') !== false ? [self::$map] : [self::$species]; }
}
$settings = [];
$passed = $failed = 0;
function check($condition, $label) {
    if ($condition) { $GLOBALS['passed']++; echo "PASS $label\n"; }
    else { $GLOBALS['failed']++; echo "FAIL $label\n"; }
}
$keys = ['hit_points', 'attack', 'defense', 'special_attack', 'special_defense', 'speed'];
$boss = ['pokemon_type_id' => 150, 'pokemon_name' => 'Fixture Boss', 'level' => 80, 'boss_multiplier' => 2,
    'attributes' => array_fill_keys($keys, 0)];
$second = $boss; $second['level'] = 20; $second['boss_multiplier'] = 3;
$map = ['id' => 3, 'name' => 'Boss fixture', 'site' => 'g', 'region' => 'Test', 'pos_x' => 50, 'pos_y' => 50,
    'min_level' => 5, 'max_level' => 5, 'is_enabled' => 1, 'experience' => -1, 'boss_config' => json_encode(['bosses' => [$boss, $second]])];
DB::$map = $map;
try { api_get_maps(); } catch (BossResponse $response) { $listed = $response->data['maps'][0]['bosses']; }
check($listed[0]['level'] === 20 && ($listed[0]['boss_index'] ?? null) === 1 && ($listed[1]['boss_index'] ?? null) === 0,
    'Sorted Boss list retains each original configuration index');
$selected = generate_wild_pokemon_legacy($map, 1, 150, 1);
check($selected['level'] === 20 && $selected['boss_multiplier'] === 3, 'Selecting second same-species variant honors its level and multiplier');
check(generate_wild_pokemon_legacy($map, 1, 150)['level'] === 80, 'Legacy species-only requests retain first-match behavior');
check(generate_wild_pokemon_legacy($map, 1, 150, 0)['level'] === 80, 'Boss index zero is a valid explicit variant');
foreach ([[2,150],[-1,150],['1',150],[1,151],[0,0]] as [$index,$species]) {
    try { generate_wild_pokemon_legacy($map, 1, $species, $index); $code = 200; }
    catch (RuntimeException $error) { $code = $error->getCode(); }
    check($code === 400, 'Invalid or mismatched Boss index is rejected: ' . json_encode([$index,$species]));
}
check(($selected['attributes'] ?? null) === $second['attributes'], 'Configured zero IVs survive encounter selection');
$hybrid = $map; $hybrid['experience'] = 30; DB::$map = $hybrid;
try { api_get_maps(); } catch (BossResponse $response) { $mode = $response->data['maps'][0]['mode']; }
check($mode === 'hybrid', 'Existing mixed map configuration retains both encounter choices');
check(!generate_wild_pokemon_legacy($hybrid, 1)['is_boss'], 'Ordinary start in a mixed map encounters a wild Pokemon');
check(generate_wild_pokemon_legacy($hybrid, 1, 150, 1)['level'] === 20, 'Explicit Boss choice remains available in a mixed map');
$defaults = $map; $defaults['boss_config'] = '{"bosses":[{"pokemon_type_id":150}]}'; DB::$map = $defaults;
try { api_get_maps(); } catch (BossResponse $response) { $listed_default = $response->data['maps'][0]['bosses'][0]; }
check($listed_default['level'] === 50 && $listed_default['boss_multiplier'] === 1.5, 'Missing Boss level and multiplier use management defaults in the list');
try { $default_wild = generate_wild_pokemon_legacy($defaults, 1, 150, 0); }
catch (Throwable $error) { $default_wild = []; }
check(($default_wild['level'] ?? null) === 50 && ($default_wild['boss_multiplier'] ?? null) === 1.5,
    'Missing Boss level and multiplier use the same defaults in encounters');
check(array_key_exists('attributes', $default_wild) && $default_wild['attributes'] === null, 'Missing legacy Boss IVs preserve random generation');

// Keep random EV/flash draws identical while proving each configured IV maps to one stat.
$zero = array_fill_keys($keys, 0);
srand(42); $baseline = battle_calc_new_npc_stats(DB::$species, 100, 1, $zero);
foreach ($keys as $slot => $key) {
    $attributes = $zero; $attributes[$key] = 255;
    srand(42); $stats = battle_calc_new_npc_stats(DB::$species, 100, 1, $attributes);
    $expected = $baseline; $expected[$slot] += 255;
    check($stats === $expected, 'Boss IV ' . $key . ' accepts the editor range 0..255 and changes only its stat');
}
srand(42); $legacy = battle_calc_new_npc_stats(DB::$species, 50, 2);
srand(42); $missing = battle_calc_new_npc_stats(DB::$species, 50, 2, null);
check($legacy == [348,232,226,238,238,236] && $missing === $legacy, 'Unconfigured Boss and ordinary wild stats preserve previous random behavior');
foreach ([-1, 256, 1.5, 'invalid'] as $invalid_iv) {
    srand(42); $invalid_stats = battle_calc_new_npc_stats(DB::$species, 50, 2, ['hit_points' => $invalid_iv]);
    check($invalid_stats === $legacy, 'Invalid configured IV preserves the previous random fallback: ' . json_encode($invalid_iv));
}
echo "Boss encounter checks: $passed passed, $failed failed\n";
exit($failed ? 1 : 0);
