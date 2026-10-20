<?php
/**
 * PVE pet-coin drop controls: global on/off, global random range, level-gap
 * cut-off and the per-day cap. Runs the real calculate_rewards/apply_rewards
 * and pm_config reader against an in-memory DB stub (no live database).
 * Run: php scripts/test/money_drop_controls.php
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/utils.php';

$wanted = ['calculate_rewards', 'apply_rewards', 'battle_money_config_flag', 'battle_money_config_int',
    'battle_money_drop_settings', 'battle_money_drop_range', 'battle_money_today'];
$tokens = token_get_all(file_get_contents(__DIR__ . '/../../plugin/api/battle.php'));
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
            $depth++;
            $opened = true;
        } elseif ($token === '}' && --$depth === 0 && $opened) {
            break;
        }
    }
    if (in_array($name, $wanted, true)) eval($body);
    $i = $j;
}
foreach ($wanted as $name) {
    if (!function_exists($name)) throw new RuntimeException("Function not loaded: $name");
}

function pm_table($name) { return $name; }
function pm_sql($sql, ...$args) { return vsprintf($sql, $args); }

class DB
{
    public static $config = [];
    public static $config_reads = 0;
    public static $user = [];
    public static $updates = [];

    public static function fetch_all($sql)
    {
        if ($sql !== 'SELECT * FROM pm_config') throw new RuntimeException('Unexpected list query: ' . $sql);
        self::$config_reads++;
        return self::$config;
    }

    public static function fetch_first($sql)
    {
        if (str_contains($sql, 'FROM pm_data')) return ['hp' => 100];
        if (str_contains($sql, 'pm_myitem')) return false;
        throw new RuntimeException('Unexpected row query: ' . $sql);
    }

    /** Emulates the one-statement reward UPDATE, including MySQL's left-to-right SET order. */
    public static function query($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (str_starts_with($sql, 'UPDATE pm_mypm ')) return;
        self::$updates[] = $sql;
        if (!preg_match('/^UPDATE pm_usersdata SET money = money \+ (\d+), '
            . 'money_today = CASE WHEN money_day = (\d+) THEN money_today \+ (\d+) ELSE (\d+) END, '
            . 'money_day = (\d+), dataall = dataall \+ 1, datawin = datawin \+ 1 WHERE uid = 7$/', $sql, $m)) {
            throw new RuntimeException('Unexpected reward SQL: ' . $sql);
        }
        self::$user['money'] += (int)$m[1];
        self::$user['money_today'] = ((int)self::$user['money_day'] === (int)$m[2])
            ? self::$user['money_today'] + (int)$m[3] : (int)$m[4];
        self::$user['money_day'] = (int)$m[5];
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

/** Replaces pm_config with the given rows; values are [value, data_type] or a plain string. */
function use_config(array $values)
{
    DB::$config = [];
    foreach ($values as $key => $value) {
        [$stored, $type] = is_array($value) ? $value : [$value, 'string'];
        DB::$config[] = ['key' => $key, 'value' => $stored, 'data_type' => $type];
    }
    api_get_runtime_config(null, null, true);
}

function rewards_for(array $user, $species_range, $pet_level = 10, $npc_level = 10)
{
    return calculate_rewards(['level' => $pet_level], array_merge(['npcid' => 1, 'strength' => 1], $user),
        ['drop_money' => $species_range], $npc_level, []);
}

$petbasisexp = [];
$settings = [];
$today = battle_money_today();
$yesterday = (int)date('Ymd', strtotime('-1 day', strtotime((string)$today)));

// ---- pm_config reader ----
use_config(['drop_money_level_gap' => ['10', 'integer'], 'drop_money_on_pve' => ['1', 'boolean'],
    'drop_money_config_by_global' => '', 'ann_title' => 'x']);
$config = api_get_runtime_config();
check($config['drop_money_level_gap'] === 10 && $config['drop_money_on_pve'] === true
    && $config['drop_money_config_by_global'] === '' && $config['ann_title'] === 'x',
    'pm_config rows are typed by data_type like the global config API');
$reads = DB::$config_reads;
api_get_runtime_config('ann_title');
api_get_runtime_config('missing', 7);
check(DB::$config_reads === $reads, 'pm_config is read once per request');
check(api_get_runtime_config('missing', 7) === 7, 'A missing pm_config key returns the caller default');

// ---- settings normalisation ----
$drop = battle_money_drop_settings([]);
check($drop === ['enabled' => true, 'by_global' => false, 'global_min' => 0, 'global_max' => 0,
    'level_gap' => 10, 'daily_limit' => 0], 'Missing keys keep the legacy drop behavior and the default level gap of 10');
$drop = battle_money_drop_settings(['drop_money_on_pve' => '', 'drop_money_config_by_global' => '1',
    'drop_money_percent_min_on_pve' => '-5', 'drop_money_percent_max_on_pve' => '20',
    'drop_money_level_gap' => '', 'drop_money_daily_limit' => 'abc']);
check(!$drop['enabled'] && $drop['by_global'] && $drop['global_min'] === 0 && $drop['global_max'] === 20
    && $drop['level_gap'] === 10 && $drop['daily_limit'] === 0, 'String-stored flags and numbers are normalised safely');
check(battle_money_drop_settings(['drop_money_on_pve' => false])['enabled'] === false
    && battle_money_drop_settings(['drop_money_on_pve' => 'true'])['enabled'] === true,
    'Boolean and textual switches are both honoured');

// ---- drop switch ----
use_config(['drop_money_on_pve' => '']);
srand(1);
$reward = rewards_for([], '[100,100]');
check($reward['money'] === 0 && $reward['exp'] > 0 && $reward['money_notes'] === [],
    'Disabling PVE money drops pays no pet coins but still pays experience');

// ---- global random range ----
use_config(['drop_money_config_by_global' => ['1', 'boolean'],
    'drop_money_percent_min_on_pve' => ['30', 'integer'], 'drop_money_percent_max_on_pve' => ['30', 'integer']]);
$reward = rewards_for(['strength' => 2], '[1,1]');
check($reward['money'] === 60, 'Equal global bounds give a fixed amount multiplied by strength');
use_config(['drop_money_config_by_global' => '1',
    'drop_money_percent_min_on_pve' => '5', 'drop_money_percent_max_on_pve' => '9']);
$inside = true;
$seen = [];
foreach (range(0, 40) as $seed) {
    srand($seed);
    $money = rewards_for(['strength' => 3], '[1000,1000]')['money'];
    $inside = $inside && $money % 3 === 0 && $money >= 15 && $money <= 27;
    $seen[$money] = true;
}
check($inside, 'Global range rolls stay within [min, max] times strength and ignore the species range');
check(count($seen) > 1, 'Global range drops are random rather than fixed');
use_config(['drop_money_config_by_global' => '1',
    'drop_money_percent_min_on_pve' => '0', 'drop_money_percent_max_on_pve' => '0']);
check(rewards_for([], '[77,77]')['money'] === 77, 'An unset (0,0) global range falls back to the species range');
use_config(['drop_money_config_by_global' => '1',
    'drop_money_percent_min_on_pve' => '25', 'drop_money_percent_max_on_pve' => '0']);
check(rewards_for([], '[77,77]')['money'] === 77, 'A global range without an upper bound falls back to the species range');
srand(5);
$money = rewards_for([], '[0,0]')['money'];
check($money >= 10 && $money <= 50, 'An unset global range and unset species range fall back to 10-50');
use_config(['drop_money_config_by_global' => '', 'drop_money_percent_min_on_pve' => '1',
    'drop_money_percent_max_on_pve' => '1']);
check(rewards_for([], '[44,44]')['money'] === 44, 'Species ranges apply when the global source is off');

// ---- level gap ----
use_config([]);
$reward = rewards_for([], '[40,40]', 21, 10);
check($reward['money'] === 0 && $reward['exp'] > 0, 'A pet 11 levels above the wild Pokemon earns no pet coins but keeps experience');
check($reward['money_notes'] === ['⚠️ 等级差距过大（高出 11 级），本场没有获得宠物币'], 'The level-gap notice names the gap');
$reward = rewards_for([], '[40,40]', 20, 10);
check($reward['money'] === 40 && $reward['money_notes'] === [], 'A pet exactly 10 levels above still earns pet coins');
check(rewards_for([], '[40,40]', 5, 30)['money'] === 40, 'Fighting a stronger wild Pokemon is never cut off');
use_config(['drop_money_level_gap' => ['0', 'integer']]);
check(rewards_for([], '[40,40]', 100, 1)['money'] === 40, 'A level gap of 0 disables the cut-off');
use_config(['drop_money_level_gap' => ['3', 'integer']]);
check(rewards_for([], '[40,40]', 14, 10)['money'] === 0 && rewards_for([], '[40,40]', 13, 10)['money'] === 40,
    'A custom level gap is honoured at its boundary');

// ---- daily cap ----
use_config(['drop_money_daily_limit' => ['100', 'integer']]);
$reward = rewards_for(['money_day' => $today, 'money_today' => 90], '[50,50]');
check($reward['money'] === 10 && $reward['money_today'] === 100 && $reward['money_daily_limit'] === 100,
    'Only the remaining daily allowance is paid');
check($reward['money_notes'] === ['今日宠物币已达上限，本场只获得 10'], 'A partial payout explains the cap');
$reward = rewards_for(['money_day' => $today, 'money_today' => 100], '[50,50]');
check($reward['money'] === 0 && $reward['money_today'] === 100 && $reward['money_notes'] === ['今日宠物币已达上限（上限 100）'],
    'Nothing is paid once the daily cap is reached');
$reward = rewards_for(['money_day' => $yesterday, 'money_today' => 100], '[50,50]');
check($reward['money'] === 50 && $reward['money_today'] === 50 && $reward['money_day'] === $today && $reward['money_notes'] === [],
    'Yesterday\'s total resets on a new day');
$reward = rewards_for(['money_day' => $today, 'money_today' => 20], '[50,50]', 30, 10);
check($reward['money'] === 0 && count($reward['money_notes']) === 1 && str_contains($reward['money_notes'][0], '等级差距'),
    'The level-gap cut-off reports only the level notice');
use_config(['drop_money_daily_limit' => ['0', 'integer']]);
$reward = rewards_for(['money_day' => $today, 'money_today' => 5000], '[50,50]');
check($reward['money'] === 50 && $reward['money_today'] === 5050 && $reward['money_daily_limit'] === 0,
    'Without a cap the daily total keeps counting');
$reward = rewards_for([], '[50,50]');
check($reward['money'] === 50 && $reward['money_today'] === 50, 'Rows without daily columns start a fresh day');

// ---- apply_rewards persists the daily total atomically ----
function settle($user, $rewards)
{
    DB::$user = $user;
    DB::$updates = [];
    apply_rewards(7, ['id' => 501, 'uid' => 7, 'species_id' => 1, 'level' => 10, 'exp' => 1000, 'hp' => 40,
        'hpg' => 0, 'hpn' => 0, 'state' => 1, 'is_shiny' => 0,
        'equipmentid1' => 0, 'equipmentid2' => 0, 'equipmentid3' => 0, 'equipmentid4' => 0], $rewards);
    return DB::$user;
}
$user = settle(['money' => 100, 'money_day' => $today, 'money_today' => 30],
    ['exp' => 1, 'money' => 20, 'money_day' => $today]);
check($user === ['money' => 120, 'money_day' => $today, 'money_today' => 50], 'The same-day total accumulates with the payout');
check(count(DB::$updates) === 1 && !preg_match('/\bIF\s*\(/i', DB::$updates[0]),
    'Money and the daily total share one UPDATE without the querysafe-banned IF()');
$user = settle(['money' => 100, 'money_day' => $yesterday, 'money_today' => 999],
    ['exp' => 1, 'money' => 20, 'money_day' => $today]);
check($user === ['money' => 120, 'money_day' => $today, 'money_today' => 20], 'A new day restarts the stored total');
$user = settle(['money' => 100, 'money_day' => $today, 'money_today' => 5], ['exp' => 1, 'money' => 0]);
check($user === ['money' => 100, 'money_day' => $today, 'money_today' => 5], 'Legacy reward arrays default to today and pay nothing extra');

// ---- day boundary uses the site time zone ----
$GLOBALS['_G'] = ['setting' => ['timeoffset' => '9'], 'member' => ['timeoffset' => '-12']];
eval('function dgmdate($timestamp, $format = "dt", $timeoffset = 9999) { $GLOBALS["dgmdate_args"] = [$format, $timeoffset]; return gmdate($format, $timestamp + intval($timeoffset) * 3600); }');
$day = battle_money_today();
check($GLOBALS['dgmdate_args'] === ['Ymd', '9'] && $day === (int)gmdate('Ymd', time() + 9 * 3600),
    'The day boundary follows the site time zone, not the member setting');

echo "Money drop controls: $checks checks, $failures failures.\n";
exit($failures ? 1 : 0);
