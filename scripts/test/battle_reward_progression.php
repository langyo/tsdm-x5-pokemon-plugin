<?php
/** Real experience helper and victory reward handler at progression boundaries. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/utils.php';

$tokens = token_get_all(file_get_contents(__DIR__ . '/../../plugin/api/battle.php'));
for ($i = 0; $i < count($tokens); $i++) {
    if (!is_array($tokens[$i]) || $tokens[$i][0] !== T_FUNCTION) continue;
    $start = $i;
    while (++$i < count($tokens) && (!is_array($tokens[$i]) || $tokens[$i][0] !== T_STRING)) {}
    if (!in_array($tokens[$i][1], ['apply_rewards', 'calculate_rewards'], true)) continue;
    $body = '';
    $depth = 0;
    $opened = false;
    for ($j = $start; $j < count($tokens); $j++) {
        $token = $tokens[$j];
        $body .= is_array($token) ? $token[1] : $token;
        if ($token === '{') { $opened = true; $depth++; }
        elseif ($token === '}' && --$depth === 0 && $opened) break;
    }
    eval($body);
    $i = $j;
}
function pm_table($name) { return $name; }
function pm_sql($sql, ...$args) { return vsprintf($sql, $args); }

class DB
{
    public static $pet;
    public static $user;
    public static function query($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (preg_match('/^UPDATE pm_mypm SET exp = (\d+), level = (\d+), hp = (\d+) WHERE id = 501$/', $sql, $m)) {
            self::$pet['exp'] = (int)$m[1];
            self::$pet['level'] = (int)$m[2];
            self::$pet['hp'] = (int)$m[3];
            return;
        }
        if (preg_match('/^UPDATE pm_mypm SET exp = (\d+) WHERE id = 501$/', $sql, $m)) {
            self::$pet['exp'] = (int)$m[1];
            return;
        }
        if (preg_match('/^UPDATE pm_usersdata SET money = money \+ (\d+), dataall = dataall \+ 1, datawin = datawin \+ 1 WHERE uid = 7$/', $sql, $m)) {
            self::$user['money'] += (int)$m[1];
            self::$user['dataall']++;
            self::$user['datawin']++;
            return;
        }
        throw new RuntimeException('Unexpected reward SQL: ' . $sql);
    }
    public static function fetch_first($sql)
    {
        if (str_contains($sql, 'FROM pm_data')) return ['hp' => 100];
        if (str_contains($sql, 'pm_myitem')) return false;
        throw new RuntimeException('Unexpected HP query: ' . $sql);
    }
}

$checks = 0;
$failures = 0;
function check($condition, $message)
{
    global $checks, $failures;
    $checks++;
    if (!$condition) $failures++;
    echo ($condition ? 'PASS ' : 'FAIL ') . $message . "\n";
}
function reward($level, $experience, $gain)
{
    DB::$pet = ['id' => 501, 'uid' => 7, 'species_id' => 1, 'level' => $level,
        'exp' => $experience, 'hp' => 40, 'hpg' => 0, 'hpn' => 0, 'state' => 1, 'is_shiny' => 0,
        'equipmentid1' => 0, 'equipmentid2' => 0, 'equipmentid3' => 0, 'equipmentid4' => 0];
    DB::$user = ['money' => 100, 'dataall' => 2, 'datawin' => 1];
    return apply_rewards(7, DB::$pet, ['exp' => $gain, 'money' => 25]);
}

$table = api_get_pet_exp_max_data(1);
$cap = $table[100];
check(api_get_pet_exp_level(1, $cap) === 100, 'Exact final experience boundary stays at level 100');
check(api_get_pet_exp_level(1, $cap + 1) === 100, 'Experience above the final boundary stays at level 100');
check(api_get_pet_exp_level(1, 2147483647) === 100, 'Very large stored experience remains capped');
check(api_get_pet_exp_level([], 10) === 0, 'An empty custom experience table retains its fallback');
check(api_get_pet_exp_level(1, 0) === 0 && api_get_pet_exp_level(1, -1) === 0, 'Nonpositive experience retains its existing behavior');
check(api_get_pet_exp_level(1, $table[30] - 1) === 30
    && api_get_pet_exp_level(1, $table[30]) === 31, 'Ordinary threshold behavior is preserved');

$result = reward(99, $table[98], $cap);
check($result['level_up'] && $result['new_level'] === 100 && DB::$pet['level'] === 100,
    'A large victory reward can cross the final threshold and reach level 100');
check($result['new_exp'] === $table[98] + $cap && DB::$pet['exp'] === $result['new_exp'],
    'The whole cumulative experience reward is stored');
check(DB::$pet['hp'] > 40, 'The final level-up restores HP');
check(DB::$user === ['money' => 125, 'dataall' => 3, 'datawin' => 2], 'Victory money and counters are awarded once');

$result = reward(100, $cap, 100);
check(!$result['level_up'] && $result['new_level'] === 100 && DB::$pet['level'] === 100,
    'A max-level victory reports the level actually stored');
check(DB::$pet['hp'] === 40, 'A capped victory does not grant another level-up heal');

$result = reward(50, 1, 20);
check(!$result['level_up'] && $result['new_level'] === 50 && DB::$pet['level'] === 50,
    'Legacy pets with low experience report their preserved level');
check(DB::$pet['exp'] === 21 && DB::$pet['hp'] === 40, 'Legacy experience mismatch still awards experience without healing');

$petbasisexp = [];
$settings = [];
foreach ([0, 1, 3, 42] as $seed) {
    srand($seed);
    $reward = calculate_rewards([], ['npcid' => 1, 'strength' => 1], ['drop_money' => '[0,0]'], 10, []);
    check($reward['money'] >= 10 && $reward['money'] <= 50, "An unset (0,0) range falls back to the default with seed $seed");
    srand($seed);
    $reward = calculate_rewards([], ['npcid' => 1, 'strength' => 1], ['drop_money' => '[0,5]'], 10, []);
    check($reward['money'] >= 0 && $reward['money'] <= 5, "A zero minimum honors the configured reward range with seed $seed");
}
srand(3);
$reward = calculate_rewards([], ['npcid' => 1, 'strength' => 1], ['drop_money' => '[]'], 10, []);
check($reward['money'] >= 10 && $reward['money'] <= 50, 'Absent reward bounds keep the default range');
srand(3);
$reward = calculate_rewards([], ['npcid' => 1, 'strength' => 1], ['drop_money' => 'not-json'], 10, []);
check($reward['money'] >= 10 && $reward['money'] <= 50, 'Malformed reward bounds fall back to the default range');
srand(3);
$reward = calculate_rewards([], ['npcid' => 1, 'strength' => 1], ['drop_money' => '[50,10]'], 10, []);
check($reward['money'] >= 10 && $reward['money'] <= 50, 'Inverted reward bounds are normalized');
srand(3);
$reward = calculate_rewards([], ['npcid' => 1, 'strength' => 0], ['drop_money' => '[0,0]'], 10, []);
check($reward['money'] >= 10, 'A zero strength still pays at least the base reward');

echo "Battle reward progression: $checks checks, $failures failures.\n";
exit($failures ? 1 : 0);
