<?php
/** Exercise production inventory filtering before pagination with isolated rows. */
define('IN_DISCUZ', true);
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

function load_inventory_functions($file, $names)
{
    $tokens = token_get_all(file_get_contents($file));
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
            $body .= is_array($token) ? $token[1] : $token;
            if ($token === '{' || (is_array($token) && in_array($token[0], [T_CURLY_OPEN, T_DOLLAR_OPEN_CURLY_BRACES], true))) {
                $depth++;
                $opened = true;
            } elseif ($token === '}' && --$depth === 0 && $opened) {
                break;
            }
        }
        if (in_array($name, $names, true)) eval($body);
        $i = $j;
    }
}

load_inventory_functions(__DIR__ . '/../../plugin/api/index.php', ['get_param', 'pm_sql_v', 'pm_sql', 'pm_table', 'validate_uid']);
load_inventory_functions(__DIR__ . '/../../plugin/api/utils.php', ['api_get_item_module', 'api_is_trainer_item', 'api_trainer_item_modules', 'api_item_use_target']);
load_inventory_functions(__DIR__ . '/../../plugin/api/user.php', ['api_get_inventory', 'get_item_type_name']);
require __DIR__ . '/../../plugin/api/constants.php';

class InventoryResponse extends RuntimeException
{
    public $data;
    public function __construct($data, $status = 200) { parent::__construct('response', $status); $this->data = $data; }
}
function require_login() {}
function api_success($data) { throw new InventoryResponse($data); }
function api_error($message, $status = 400) { throw new InventoryResponse($message, $status); }

class DB
{
    public static $rows = [];
    public static $total = 0;
    public static function fetch_all($sql)
    {
        if (!preg_match('/WHERE m.uid = (\d+)(.*?)GROUP BY m.id/is', $sql, $where)) throw new RuntimeException('Missing owner filter');
        if (!preg_match('/LIMIT (\d+), (\d+)$/', $sql, $limit)) throw new RuntimeException('Invalid pagination');
        $type = preg_match('/i.type = (\d+)/', $where[2], $match) ? (int)$match[1] : null;
        $search = null;
        if (strpos($where[2], 'LOCATE(') !== false) {
            if (!preg_match("/LOCATE\(('(?:\\\\.|[^'\\\\])*'), i.name\) > 0/", $where[2], $match)) throw new RuntimeException('Malformed search literal');
            $search = stripslashes(substr($match[1], 1, -1));
        }
        if (strpos($sql, 'HAVING i.type <> 5 OR m.nums > equipped_cnt') === false) throw new RuntimeException('Missing equipped stack exclusion');
        $rows = array_filter(self::$rows, function ($row) use ($where, $type, $search) {
            return $row['uid'] === (int)$where[1]
                && ($type === null || $row['item_type'] === $type)
                && ($search === null || strpos(strtolower($row['item_name']), strtolower($search)) !== false)
                && ($row['item_type'] !== 5 || $row['nums'] > $row['equipped_cnt']);
        });
        usort($rows, function ($a, $b) { return $b['id'] <=> $a['id']; });
        self::$total = count($rows);
        return array_slice($rows, (int)$limit[1], (int)$limit[2]);
    }
    public static function result_first($sql)
    {
        if ($sql !== 'SELECT FOUND_ROWS()') throw new RuntimeException('Unexpected count query');
        return self::$total;
    }
}

function inventory_row($id, $name, $type = 1, $uid = 1, $quantity = 1, $equipped = 0)
{
    return ['id' => $id, 'uid' => $uid, 'item_type' => $type, 'item_name' => $name,
        'item_desc' => 'description', 'item_image' => '', 'itemdata_id' => $id + 1000,
        'nums' => $quantity, 'equipped_cnt' => $equipped];
}

for ($i = 1; $i <= 110; $i++) DB::$rows[] = inventory_row($i, $i <= 60 ? '伤药 ' . $i : '木牌 ' . $i);
DB::$rows[] = inventory_row(200, '伤药 other user', 1, 2);
DB::$rows[] = inventory_row(201, '伤药护符', 5, 1, 2, 1);
DB::$rows[] = inventory_row(202, '伤药满装备', 5, 1, 1, 1);
DB::$rows[] = inventory_row(203, '伤药球', 2);
foreach (["Trainer's item", '100%宝石', 'under_score', 'C:\\bag\\item', 'Lemon Stone'] as $i => $name) {
    DB::$rows[] = inventory_row(300 + $i, $name);
}
$GLOBALS['_G'] = ['uid' => 1];

function inventory_request($params)
{
    $_GET = $params;
    $_POST = [];
    try { api_get_inventory(); } catch (InventoryResponse $response) { return $response; }
    throw new RuntimeException('Missing inventory response');
}

$checks = 0;
function inventory_expect($condition, $message)
{
    $GLOBALS['checks']++;
    if (!$condition) throw new RuntimeException($message);
}

$data = inventory_request(['search' => ' 伤药 ', 'type' => 1])->data;
inventory_expect($data['total'] === 60 && $data['total_pages'] === 2, 'Search count must include all matching pages');
inventory_expect(count($data['items']) === 50 && $data['items'][0]['id'] === 60, 'Name search must run before LIMIT');
$next = inventory_request(['search' => '伤药', 'type' => 1, 'page' => 2])->data;
inventory_expect(count($next['items']) === 10 && $next['items'][0]['id'] === 10, 'Search must be retained on later pages');
inventory_expect(array_intersect(array_column($data['items'], 'id'), array_column($next['items'], 'id')) === [], 'Pages may not overlap');

$all = inventory_request(['search' => '伤药'])->data;
inventory_expect($all['total'] === 62, 'All types includes ball and free equipment, excluding other owners and fully equipped stacks');
$equipment = inventory_request(['search' => '伤药', 'type' => 5])->data;
inventory_expect(count($equipment['items']) === 1 && $equipment['items'][0]['quantity'] === 1, 'Search preserves equipment availability');
$ball = inventory_request(['search' => '伤药', 'type' => 2])->data;
inventory_expect($ball['total'] === 1 && $ball['items'][0]['id'] === 203, 'Name and category filters combine');

foreach (["Trainer's" => 300, '%' => 301, '_' => 302, 'C:\\bag\\' => 303, 'leMON' => 304] as $term => $id) {
    $data = inventory_request(['search' => $term])->data;
    inventory_expect($data['total'] === 1 && $data['items'][0]['id'] === $id, 'Literal search failed: ' . $term);
}
$empty = inventory_request(['search' => 'not found'])->data;
inventory_expect($empty['items'] === [] && $empty['total'] === 0 && $empty['total_pages'] === 0, 'Empty search result keeps pagination valid');
$legacy = inventory_request([])->data;
inventory_expect(inventory_request(['search' => '   '])->data === $legacy, 'Blank search preserves the old endpoint contract');
inventory_expect(inventory_request(['page' => 0])->data['page'] === 1, 'Page zero is clamped to one');
inventory_expect(inventory_request(['search' => ['invalid']])->getCode() === 400, 'Malformed search is rejected before SQL');
echo "Inventory search: $checks checks passed\n";
