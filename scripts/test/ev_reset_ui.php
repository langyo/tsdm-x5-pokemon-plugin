<?php
/** Exercise the effort-value-reset (怠惰药片) target picker and its use flow. */
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
        if (preg_match("/^SELECT \* FROM pm_myitem WHERE uid = (\d+) AND itemid = '(\d+)'$/", $sql, $m)) {
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
            // 强制按真实查询的投影返回，漏选列（如 site/ev 列）会直接失败。
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
        if (preg_match('/^UPDATE pm_mypm SET hpn = 0, atkn = 0, defn = 0, spatkn = 0, spdefn = 0, sdn = 0 WHERE id = (\d+)$/', $sql, $m)) {
            foreach (['hpn', 'atkn', 'defn', 'spatkn', 'spdefn', 'sdn'] as $column) {
                self::$pets[(int) $m[1]][$column] = 0;
            }
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

function fixture($item_id = 212, $stock = 2)
{
    $GLOBALS['input'] = ['item_id' => $item_id, 'pokemon_id' => 1];
    DB::$items = [
        212 => ['id' => 212, 'name' => '怠惰药片', 'type' => 4, 'module' => '', 'sitemname' => 'clean_equipment',
            'tpname' => 'clean_equipment', 'effects' => '{"hp": 0, "exp": 0, "level": 0, "intimacy": 0}'],
    ];
    $spec = ['id' => 25, 'name' => 'Pikachu', 'hp' => 50, 'atk' => 50, 'def' => 50,
        'spatk' => 50, 'spdef' => 50, 'speed' => 50];
    DB::$species = [25 => $spec];
    DB::$pets = [];
    // seed(id, site, [hpn, atkn, defn, spatkn, spdefn, sdn], hp)
    $seed = [
        [1, 1, [100, 50, 0, 0, 0, 0], 100],    // ev 150
        [2, 2, [0, 0, 0, 0, 0, 0], 90],        // ev 0 -> 已清零，不可选
        [3, 2, [10, 10, 10, 10, 10, 10], 80],  // ev 60
        [4, 1, [252, 0, 0, 0, 0, 0], 120],     // ev 252
        [5, 2, [30, 0, 0, 0, 0, 0], 70],       // ev 30（被 6 只上限截掉）
        [6, 1, [90, 0, 0, 0, 0, 0], 60],       // ev 90
        [7, 2, [120, 0, 0, 0, 0, 0], 50],      // ev 120（被 6 只上限截掉）
        [8, 1, [510, 0, 0, 0, 0, 0], 40],      // ev 510
        [9, 3, [100, 0, 0, 0, 0, 0], 30],      // storage
        [10, 3, [60, 0, 0, 0, 0, 0], 20],      // storage
    ];
    foreach ($seed as [$id, $site, $evs, $hp]) {
        DB::$pets[$id] = [
            'id' => $id, 'uid' => 7, 'site' => $site, 'species_id' => 25,
            'nickname' => 'Pet' . $id, 'level' => 50, 'hp' => $hp,
            'hpg' => 20, 'atkg' => 20, 'defg' => 20,
            'spatkg' => 20, 'spdefg' => 20, 'sdg' => 20,
            'hpn' => $evs[0], 'atkn' => $evs[1], 'defn' => $evs[2],
            'spatkn' => $evs[3], 'spdefn' => $evs[4], 'sdn' => $evs[5],
            'state' => 1, 'is_shiny' => 0,
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
// 1. 怠惰药片目标选择器：仅携带中的宠物（site<3），按 site,id 排序且最多 6 只；
//    努力值已清零的携带宠物不再列为可选目标。
// ---------------------------------------------------------------------------
fixture();
$response = invoke('api_get_usable_pokemon');
check($response->getCode() === 200, 'EV-reset picker succeeds for the tablet item');
$data = $response->data;
check($data['item_module'] === 'clean_equipment', 'Picker reports the clean_equipment item module');
check($data['item_type'] === 4, 'Picker keeps reporting the item type');
check(array_column($data['usable_pokemon'], 'id') === [1, 4, 6, 8, 3],
    'EV-reset picker lists carried pets with effort values only, sorted by site and id');
check($data['unusable_count'] === 1, 'Zero-effort carried pet is counted unusable, not listed');
$by_id = [];
foreach ($data['usable_pokemon'] as $pokemon) {
    $by_id[$pokemon['id']] = $pokemon;
    check($pokemon['ev_total'] === pm_ev_total(DB::$pets[$pokemon['id']]),
        'Picker pet carries its effort-value total');
}
check(!isset($by_id[2]) && !isset($by_id[5]) && !isset($by_id[7]),
    'Zero-effort and capped-out carried pets are excluded from the listed targets');
check(!isset($by_id[9]) && !isset($by_id[10]), 'Stored pets are excluded from the EV-reset picker');
check($by_id[1]['ev_total'] === 150 && $by_id[4]['ev_total'] === 252 && $by_id[8]['ev_total'] === 510,
    'Effort totals survive the picker payload');

// 没有携带宠物时返回空列表而不是仓库宠物
fixture();
foreach (DB::$pets as $id => $pet) DB::$pets[$id]['site'] = 3;
$response = invoke('api_get_usable_pokemon');
check($response->getCode() === 200 && $response->data['usable_pokemon'] === [],
    'EV-reset picker returns no targets when nothing is carried');

// 携带宠物的努力值全部为 0 时同样返回空列表（无可重置目标）
fixture();
foreach (DB::$pets as $id => $pet) {
    if ((int) $pet['site'] < 3) {
        foreach (['hpn', 'atkn', 'defn', 'spatkn', 'spdefn', 'sdn'] as $column) DB::$pets[$id][$column] = 0;
    }
}
$response = invoke('api_get_usable_pokemon');
check($response->getCode() === 200 && $response->data['usable_pokemon'] === [],
    'EV-reset picker returns no targets when every carried pet is already reset');

// ---------------------------------------------------------------------------
// 2. 使用怠惰药片：清空努力值并报告前后变化
// ---------------------------------------------------------------------------
fixture();
$response = invoke('api_use_item');
check($response->getCode() === 200 && $response->data['success'] === true, 'EV reset succeeds');
$message = $response->data['message'];
check(strpos($message, 'Pet1') !== false && strpos($message, '怠惰药片') !== false,
    'EV-reset message names the pet and the item');
check(strpos($message, '努力值 150 → 0') !== false, 'EV-reset message reports the before/after totals');
check(pm_ev_total(DB::$pets[1]) === 0, 'Effort values are zeroed in storage');
check((int) DB::$pets[1]['hp'] === 100, 'EV reset leaves current HP untouched');
check($response->data['item_remaining'] === 1 && DB::$stock['nums'] === 1, 'Successful reset consumes one item');

// 使用后选择器不再列出该宠物
$response = invoke('api_get_usable_pokemon');
$ids = array_column($response->data['usable_pokemon'], 'id');
check(!in_array(1, $ids, true) && in_array(4, $ids, true),
    'Picker drops the freshly reset pet while keeping other targets');

// 努力值已为 0 的宠物直接调用使用接口：400 且不扣道具
fixture();
$GLOBALS['input']['pokemon_id'] = 2;
$response = invoke('api_use_item');
check($response->getCode() === 400 && DB::$writes === [] && DB::$stock['nums'] === 2,
    'Reset on an already-zero pet is rejected without writes or consumption');
check(strpos($response->getMessage(), '努力值') !== false, 'Zero-effort rejection explains the reason');

// 不能对他人宠物使用怠惰药片
fixture();
DB::$pets[1]['uid'] = 8;
$response = invoke('api_use_item');
check($response->getCode() === 403 && DB::$writes === [] && DB::$stock['nums'] === 2,
    'EV reset rejects another user pet without writes');

// 消耗最后一片时库存被清除，item_remaining 归零
fixture(212, 1);
$response = invoke('api_use_item');
check($response->getCode() === 200 && $response->data['item_remaining'] === 0 && DB::$stock === false,
    'Last tablet is consumed and reported as zero remaining');

echo "$passed EV reset UI assertions passed\n";
