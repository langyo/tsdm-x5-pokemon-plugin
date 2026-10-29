<?php
/** Exercise the quality-reroll target picker and the item-use grade-change payload. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/utils.php';
require __DIR__ . '/../../plugin/api/constants.php';
require __DIR__ . '/../../plugin/api/pokemon_utils.php';
require __DIR__ . '/../../plugin/api/item_modules.php';

function load_item_functions($path, $wanted)
{
    $tokens = token_get_all(file_get_contents($path));
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
            $body .= is_array($token) && $token[0] === T_DIR ? var_export(realpath(__DIR__ . '/../../plugin/api'), true) : (is_array($token) ? $token[1] : $token);
            if ($token === '{' || (is_array($token) && in_array($token[0], [T_CURLY_OPEN, T_DOLLAR_OPEN_CURLY_BRACES], true))) {
                $depth++;
                $opened = true;
            } elseif ($token === '}' && --$depth === 0 && $opened) break;
        }
        if (in_array($name, $wanted, true)) eval($body);
        $i = $j;
    }
}
load_item_functions(__DIR__ . '/../../plugin/api/user.php', ['api_use_item', 'api_get_usable_pokemon', 'downgrade_weak_state']);
load_item_functions(__DIR__ . '/../../plugin/api/index.php', ['pm_sql', 'pm_sql_v']);

class ItemResponse extends RuntimeException
{
    public $data;
    public function __construct($code, $data)
    {
        parent::__construct(is_string($data) ? $data : 'success', $code);
        $this->data = $data;
    }
}
function api_success($data) { throw new ItemResponse(200, $data); }
function api_error($message, $status = 400) { throw new ItemResponse($status, $message); }
function require_login() {}
function pm_table($name) { return $name; }
function validate_uid($value) { return (int) $value; }
function validate_id($value, $name) { return (int) $value; }
function get_json_input() { return $GLOBALS['input']; }
function get_param($name, $default = null) { return $GLOBALS['input'][$name] ?? $default; }

class DB
{
    public static $pets = [], $items = [], $species = [], $stock = null, $writes = [];

    private static function normalize($sql)
    {
        return preg_replace('/\s+/', ' ', trim($sql));
    }

    public static function fetch_first($sql)
    {
        $sql = self::normalize($sql);
        if (preg_match('/^SELECT \* FROM pm_usersdata WHERE uid = (\d+) FOR UPDATE$/', $sql, $m)) {
            return ['uid' => (int) $m[1], 'npcid' => 0];
        }
        if (preg_match('/^SELECT \* FROM pm_itemdata WHERE id = (\d+)$/', $sql, $m)) {
            return self::$items[(int) $m[1]] ?? false;
        }
        if (preg_match("/^SELECT \\* FROM pm_myitem WHERE uid = (\\d+) AND itemid = '(\\d+)'$/", $sql, $m)) {
            return self::$stock;
        }
        if (preg_match('/^SELECT \* FROM pm_mypm WHERE id = (\d+)$/', $sql, $m)) {
            return self::$pets[(int) $m[1]] ?? false;
        }
        if (preg_match('/^SELECT \* FROM pm_mypm WHERE id = (\d+) AND uid = (\d+)$/', $sql, $m)) {
            $pet = self::$pets[(int) $m[1]] ?? false;
            return ($pet && (int) $pet['uid'] === (int) $m[2]) ? $pet : false;
        }
        if (preg_match('/^SELECT \* FROM pm_data WHERE id = (\d+)$/', $sql, $m)) {
            return self::$species[(int) $m[1]] ?? false;
        }
        if (preg_match('/^SELECT name, hp FROM pm_data WHERE id = (\d+)$/', $sql, $m)) {
            $species = self::$species[(int) $m[1]] ?? false;
            return $species ? ['name' => $species['name'], 'hp' => $species['hp']] : false;
        }
        if (preg_match('/^SELECT hp FROM pm_data WHERE id = (\d+)$/', $sql, $m)) {
            $species = self::$species[(int) $m[1]] ?? false;
            return $species ? ['hp' => $species['hp']] : false;
        }
        throw new RuntimeException('Unexpected read: ' . $sql);
    }

    public static function fetch_all($sql)
    {
        $sql = self::normalize($sql);
        if (!preg_match('/^SELECT (.+?) FROM pm_mypm WHERE uid = (\d+)( AND site < 3 ORDER BY site ASC, id ASC LIMIT 6)?$/', $sql, $m)) {
            throw new RuntimeException('Unexpected list: ' . $sql);
        }
        $columns = array_map('trim', explode(',', $m[1]));
        $carried_only = !empty($m[3]);
        $pets = array_values(array_filter(self::$pets, function ($pet) use ($m, $carried_only) {
            return (int) $pet['uid'] === (int) $m[2] && (!$carried_only || (int) $pet['site'] < 3);
        }));
        if ($carried_only) {
            usort($pets, function ($a, $b) {
                return [(int) $a['site'], (int) $a['id']] <=> [(int) $b['site'], (int) $b['id']];
            });
            $pets = array_slice($pets, 0, 6);
        } else {
            usort($pets, fn($a, $b) => (int) $a['id'] <=> (int) $b['id']);
        }

        $rows = [];
        foreach ($pets as $pet) {
            // 强制按真实查询的投影返回，漏选列（如 site/is_shiny）会直接失败。
            $row = [];
            foreach ($columns as $column) {
                if (!array_key_exists($column, $pet)) {
                    throw new RuntimeException('List projection column missing: ' . $column);
                }
                $row[$column] = $pet[$column];
            }
            $rows[] = $row;
        }
        return $rows;
    }

    public static function query($sql)
    {
        $sql = self::normalize($sql);
        if (in_array($sql, ['START TRANSACTION', 'COMMIT', 'ROLLBACK'], true)) return;
        self::$writes[] = $sql;
        if (preg_match('/^UPDATE pm_mypm SET hpg = (\d+), atkg = (\d+), defg = (\d+), spatkg = (\d+), spdefg = (\d+), sdg = (\d+), hp = (\d+) WHERE id = (\d+)$/', $sql, $m)) {
            $pet =& self::$pets[(int) $m[8]];
            foreach (['hpg' => 1, 'atkg' => 2, 'defg' => 3, 'spatkg' => 4, 'spdefg' => 5, 'sdg' => 6] as $column => $group) {
                $pet[$column] = (int) $m[$group];
            }
            $pet['hp'] = (int) $m[7];
            return;
        }
        if (preg_match('/^UPDATE pm_myitem SET nums = (\d+) WHERE id = (\d+)$/', $sql, $m)) {
            self::$stock['nums'] = (int) $m[1];
            return;
        }
        if (preg_match('/^DELETE FROM pm_myitem WHERE id = (\d+)$/', $sql, $m)) {
            self::$stock = false;
            return;
        }
        throw new RuntimeException('Unexpected write: ' . $sql);
    }
}

$GLOBALS['_G'] = ['uid' => 7];
$passed = 0;
function check($condition, $message)
{
    if (!$condition) throw new RuntimeException($message);
    $GLOBALS['passed']++;
}

function fixture($item_id = 46, $stock = 2)
{
    $GLOBALS['input'] = ['item_id' => $item_id, 'pokemon_id' => 1];
    DB::$items = [
        46 => ['id' => 46, 'name' => '品质重洗药', 'type' => 4, 'module' => '', 'sitemname' => 'quality',
            'tpname' => 'yypg', 'effects' => '{"hp": 0, "exp": 0, "level": 0, "intimacy": 0}'],
        53 => ['id' => 53, 'name' => '安详铃', 'type' => 4, 'module' => '', 'sitemname' => '',
            'tpname' => 'ocl', 'effects' => '{"hp": 0, "exp": 0, "level": 0, "intimacy": 15}'],
    ];
    $spec = ['id' => 25, 'name' => 'Pikachu', 'hp' => 50, 'atk' => 50, 'def' => 50,
        'spatk' => 50, 'spdef' => 50, 'speed' => 50];
    DB::$species = [25 => $spec];
    DB::$pets = [];
    // seed(id, site, [hp, atk, def, spatk, spdef, speed], hp, shiny)
    $seed = [
        [1, 1, [31, 31, 31, 31, 31, 31], 100, 1],   // 186 = SS+
        [2, 2, [31, 31, 31, 31, 31, 31], 90, 0],
        [3, 2, [31, 31, 31, 31, 31, 31], 80, 0],
        [4, 1, [31, 31, 31, 31, 31, 15], 120, 0],  // 170 = SS
        [5, 2, [31, 31, 31, 31, 31, 31], 70, 0],
        [6, 1, [20, 20, 20, 20, 20, 15], 60, 0],   // 115 = B
        [7, 2, [31, 31, 31, 31, 31, 31], 50, 0],
        [8, 3, [10, 10, 10, 10, 10, 10], 40, 0],   // storage
        [9, 3, [5, 5, 5, 5, 5, 5], 30, 0],         // storage
    ];
    foreach ($seed as [$id, $site, $ivs, $hp, $shiny]) {
        DB::$pets[$id] = [
            'id' => $id, 'uid' => 7, 'site' => $site, 'species_id' => 25,
            'nickname' => 'Pet' . $id, 'level' => 50, 'hp' => $hp,
            'hpg' => $ivs[0], 'atkg' => $ivs[1], 'defg' => $ivs[2],
            'spatkg' => $ivs[3], 'spdefg' => $ivs[4], 'sdg' => $ivs[5],
            'hpn' => 0, 'atkn' => 0, 'defn' => 0, 'spatkn' => 0, 'spdefn' => 0, 'sdn' => 0,
            'state' => 1, 'is_shiny' => $shiny,
            'equipmentid1' => 0, 'equipmentid2' => 0, 'equipmentid3' => 0, 'equipmentid4' => 0,
        ];
    }
    DB::$stock = ['id' => 2, 'uid' => 7, 'itemid' => $item_id, 'nums' => $stock];
    DB::$writes = [];
}

function invoke($endpoint)
{
    try { $endpoint(); } catch (ItemResponse $response) { return $response; }
    throw new RuntimeException('No response');
}

// ---------------------------------------------------------------------------
// 1. 品质重洗药目标选择器：仅携带中的宠物（site<3），按 site,id 排序且最多 6 只
// ---------------------------------------------------------------------------
fixture(46);
$response = invoke('api_get_usable_pokemon');
check($response->getCode() === 200, 'Quality picker succeeds for the quality item');
$data = $response->data;
check($data['item_module'] === 'quality', 'Picker reports the quality item module');
check($data['item_type'] === 4, 'Picker keeps reporting the item type');
check(array_column($data['usable_pokemon'], 'id') === [1, 4, 6, 2, 3, 5],
    'Quality picker lists only the six carried pets sorted by site and id');
check($data['unusable_count'] === 0, 'Quality picker has no unusable remainder for filtered pets');
$by_id = [];
foreach ($data['usable_pokemon'] as $index => $pokemon) {
    $by_id[$pokemon['id']] = $pokemon;
    $expected = pm_quality_of_pokemon(DB::$pets[$pokemon['id']]);
    check($pokemon['site'] === (int) DB::$pets[$pokemon['id']]['site'], 'Picker pet carries its site');
    check($pokemon['is_shiny'] === (DB::$pets[$pokemon['id']]['is_shiny'] === 1), 'Picker pet carries the shiny flag');
    check($pokemon['iv_total'] === $expected['iv_total'], 'Picker pet carries the IV total');
    check($pokemon['quality_rank'] === $expected['rank'], 'Picker pet carries the quality rank');
    check($pokemon['quality_grade'] === $expected['grade'], 'Picker pet carries the quality grade');
}
check(!isset($by_id[8]) && !isset($by_id[9]), 'Stored pets are excluded from the quality picker');
check($by_id[1]['quality_grade'] === 'SS+' && $by_id[1]['iv_total'] === 186 && $by_id[1]['quality_rank'] === 8,
    'A capped 186-total pet shows SS+ in the picker');
check($by_id[1]['is_shiny'] === true && $by_id[4]['is_shiny'] === false, 'Shiny flags survive the picker payload');
check($by_id[4]['quality_grade'] === 'SS' && $by_id[4]['iv_total'] === 170, '170-total pet shows SS');
check($by_id[6]['quality_grade'] === 'B' && $by_id[6]['iv_total'] === 115, '115-total pet shows B');

// 品质重洗药在“没有携带宠物”时返回空列表而不是仓库宠物
fixture(46);
foreach (DB::$pets as $id => $pet) DB::$pets[$id]['site'] = 3;
$response = invoke('api_get_usable_pokemon');
check($response->getCode() === 200 && $response->data['usable_pokemon'] === [],
    'Quality picker returns no targets when nothing is carried');

// ---------------------------------------------------------------------------
// 2. 非品质道具维持原有行为：仍然列出账号下全部宠物
// ---------------------------------------------------------------------------
fixture(53, 1);
$response = invoke('api_get_usable_pokemon');
check($response->getCode() === 200 && $response->data['item_module'] === 'ocl',
    'Non-quality items report their own module');
check(count($response->data['usable_pokemon']) === 9,
    'Non-quality items keep listing all pets including stored ones');
check(array_column($response->data['usable_pokemon'], 'id') === [1, 2, 3, 4, 5, 6, 7, 8, 9],
    'Non-quality items keep the unfiltered pet order');
check($response->data['unusable_count'] === 0, 'Type-4 non-quality items remain usable on every pet');

// ---------------------------------------------------------------------------
// 3. 品质重洗：返回前后品质、总个体值、HP 上限并由服务端完成重洗
// ---------------------------------------------------------------------------
fixture(46, 2);
DB::$pets[1]['hp'] = 500; // 超过任何等级 50 的上限，验证会被钳制
$before = pm_quality_of_pokemon(DB::$pets[1]);
$response = invoke('api_use_item');
check($response->getCode() === 200 && $response->data['success'] === true, 'Quality reroll succeeds');
$update = $response->data['pokemon_updated'];
check(is_array($update) && $update['id'] === 1, 'Reroll response carries the pokemon update');
$after = pm_quality_of_pokemon(DB::$pets[1]);
check($before['iv_total'] === 186 && $before['grade'] === 'SS+', 'Fixture starts as a capped SS+ pet');
check($update['old_quality_grade'] === 'SS+' && $update['old_iv_total'] === 186,
    'Reroll response carries the pre-use quality');
check($update['new_quality_grade'] === $after['grade'] && $update['new_iv_total'] === $after['iv_total'],
    'Reroll response carries the post-use quality');
check($update['new_iv_total'] === array_sum([DB::$pets[1]['hpg'], DB::$pets[1]['atkg'], DB::$pets[1]['defg'],
    DB::$pets[1]['spatkg'], DB::$pets[1]['spdefg'], DB::$pets[1]['sdg']]),
    'Post-use IV total matches the persisted individual values');
$new_ivs = [DB::$pets[1]['hpg'], DB::$pets[1]['atkg'], DB::$pets[1]['defg'],
    DB::$pets[1]['spatkg'], DB::$pets[1]['spdefg'], DB::$pets[1]['sdg']];
check(min($new_ivs) >= 0 && max($new_ivs) <= 31, 'Rerolled individual values stay within 0-31');
$new_max_hp = api_calculate_pokemon_max_hp(DB::$pets[1]);
check($update['max_hp'] === $new_max_hp, 'Reroll response carries the recalculated maximum HP');
check(DB::$pets[1]['hp'] === $new_max_hp && $update['hp'] === $new_max_hp,
    'Current HP is clamped to the recalculated maximum');
$message = $response->data['message'];
check(strpos($message, '品质') !== false && strpos($message, '→') !== false
    && strpos($message, '总个体值') !== false, 'Reroll message describes the grade change');
check(strpos($message, $update['old_quality_grade']) !== false
    && strpos($message, $update['new_quality_grade']) !== false, 'Reroll message shows both grades');
check($response->data['item_remaining'] === 1 && DB::$stock['nums'] === 1, 'Successful reroll consumes one item');

// 选择器回读已重洗后的品质
$response = invoke('api_get_usable_pokemon');
$target = null;
foreach ($response->data['usable_pokemon'] as $pokemon) {
    if ($pokemon['id'] === 1) $target = $pokemon;
}
check($target !== null && $target['quality_grade'] === $after['grade'] && $target['iv_total'] === $after['iv_total'],
    'Picker reflects the rerolled quality afterwards');

// 消耗最后一件时库存被清除，item_remaining 归零
fixture(46, 1);
$response = invoke('api_use_item');
check($response->getCode() === 200 && $response->data['item_remaining'] === 0 && DB::$stock === false,
    'Last quality item is consumed and reported as zero remaining');

// 不能对他人宠物使用品质重洗药
fixture(46);
DB::$pets[1]['uid'] = 8;
$response = invoke('api_use_item');
check($response->getCode() === 403 && DB::$writes === [] && DB::$stock['nums'] === 2,
    'Quality reroll rejects another user pet without writes');

echo "$passed quality reroll UI assertions passed\n";
