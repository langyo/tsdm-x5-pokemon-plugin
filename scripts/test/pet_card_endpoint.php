<?php
/** Pet sidebar card endpoint regression (api_get_pokemon_card). */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/utils.php';
require __DIR__ . '/../../plugin/api/pokemon_utils.php';

// 从 pokemon.php 提取被测函数（与 healing.php 相同的 token 提取方式，
// 避免加载整个端点文件触发其路由 switch）。
$wanted = ['api_get_pokemon_card', 'get_pokemon_state_text', 'calculate_stats',
    'get_pokemon_exp_progress', 'get_pokemon_exp_table_local', 'get_pokemon_exp_type_local'];
$tokens = token_get_all(file_get_contents(__DIR__ . '/../../plugin/api/pokemon.php'));
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
        } elseif ($token === '}') {
            $depth--;
            if ($opened && $depth === 0) break;
        }
    }
    if (in_array($name, $wanted, true)) eval($body);
    $i = $j;
}

class CardResponse extends RuntimeException
{
    public $data;
    public function __construct($message, $code, $data = null)
    {
        parent::__construct($message, $code);
        $this->data = $data;
    }
}
function api_success($data) { throw new CardResponse('success', 200, $data); }
function api_error($message, $code) { throw new CardResponse($message, $code); }
function validate_id($id, $name = 'ID') { return (int) $id; }
function get_param($key, $default = null) { return $GLOBALS['card_case']['params'][$key] ?? $default; }
function pm_table($name) { return $name; }
function pm_sql($sql)
{
    $args = func_get_args();
    array_shift($args);
    return vsprintf($sql, array_map(function ($v) { return is_int($v) ? $v : "'" . addslashes($v) . "'"; },
        array_pad($args, substr_count($sql, '%') - 1, '')));
}
function pm_sql_v($sql, $args) { return $sql; }

$GLOBALS['card_case'] = [
    'pet' => null,
    'params' => ['petid' => 99],
];

class DB
{
    private static function expand($sql, $arg)
    {
        return preg_replace_callback('/%[tdsn]/', function ($match) use (&$arg) {
            $value = array_shift($arg);
            switch ($match[0]) {
                case '%t': return 'pre_' . $value;
                case '%d': return (string) (int) $value;
                case '%n': return implode(',', array_map('intval', (array) $value));
                default: return "'" . addslashes($value) . "'";
            }
        }, $sql);
    }

    public static function fetch_first($sql, $arg = [])
    {
        $sql = self::expand($sql, $arg);
        if (strpos($sql, 'FROM pm_mypm WHERE id =') !== false) {
            $pet = $GLOBALS['card_case']['pet'];
            // 真实数据库按 WHERE ... AND site < 3 过滤，桩遵循同一条件
            if ($pet && (int) $pet['site'] >= 3) {
                return false;
            }
            return $pet;
        }
        if (strpos($sql, 'FROM pm_data WHERE id =') !== false) {
            return ['id' => 25, 'name' => '皮卡丘', 'xs' => '电', 'xs2' => '',
                'hp' => 35, 'atk' => 55, 'def' => 40, 'spatk' => 50, 'spdef' => 50, 'speed' => 90,
                'is_legendary' => !empty($GLOBALS['card_case']['species_legendary']) ? 1 : 0];
        }
        if (strpos($sql, 'FROM pre_common_member WHERE') !== false) {
            return ['username' => '训练家甲'];
        }
        throw new RuntimeException('Unexpected fetch_first: ' . $sql);
    }

    public static function fetch_all($sql, $arg = [])
    {
        $sql = self::expand($sql, $arg);
        if (strpos($sql, 'FROM pm_myskill') !== false) {
            return [
                ['skillnum' => 15, 'skill_name' => '十万伏特', 'element' => '电', 'power' => 90, 'max_pp' => 20],
                ['skillnum' => 0, 'skill_name' => '电光一闪', 'element' => '一般', 'power' => 40, 'max_pp' => 0],
            ];
        }
        throw new RuntimeException('Unexpected fetch_all: ' . $sql);
    }
}

$failures = 0;
function assert_same($expected, $actual, $label)
{
    global $failures;
    if ($expected !== $actual) {
        $failures++;
        echo "FAIL: $label\n  expected: " . var_export($expected, true) . "\n  actual:   " . var_export($actual, true) . "\n";
    }
}

function base_pet()
{
    return ['id' => 99, 'uid' => 7, 'species_id' => 25, 'nickname' => '皮卡', 'level' => 50,
        'exp' => 95000,
        'hp' => 9999, 'sex' => 1, 'is_shiny' => 1, 'good' => 88, 'state' => 1,
        'hpg' => 31, 'hpn' => 0, 'atkg' => 0, 'atkn' => 0, 'defg' => 0, 'defn' => 0,
        'spatkg' => 0, 'spatkn' => 0, 'spdefg' => 0, 'spdefn' => 0, 'sdg' => 0, 'sdn' => 0,
        'equipmentid1' => 0, 'equipmentid2' => 0, 'equipmentid3' => 0, 'equipmentid4' => 0,
        'site' => 2];
}

// --- 携带中的闪光宠物：完整卡片 ---
$GLOBALS['card_case']['pet'] = base_pet();
try {
    api_get_pokemon_card();
    $failures++;
    echo "FAIL: card must respond via api_success\n";
} catch (CardResponse $r) {
    $card = $r->data;
    assert_same(99, $card['id'], 'card id');
    assert_same('训练家甲', $card['owner'], 'owner username resolved');
    assert_same('皮卡丘', $card['name'], 'species name from pm_data');
    assert_same('皮卡', $card['nickname'], 'nickname');
    assert_same(50, $card['level'], 'level');
    assert_same(1, $card['gender'], 'gender');
    assert_same(true, $card['is_shiny'], 'shiny flag');
    assert_same(false, $card['is_legendary'], 'non-legendary card flag');
    assert_same('正常', $card['state_text'], 'state text');
    assert_same('电', $card['type_1'], 'type_1');
    assert_same('', $card['type_2'], 'type_2 empty for mono type');
    assert_same(88, $card['affection'], 'affection');
    // 经验值：默认经验表 Lv.50 起 91689、Lv.51 需 95995（见 api/utils.php）
    assert_same(95000, $card['exp'], 'current exp echoed back');
    assert_same(91689, $card['exp_for_current_level'], 'exp threshold of current level');
    assert_same(95995, $card['exp_for_next_level'], 'exp threshold of next level');
    assert_same('电', $card['skills'][0]['element'], 'skill element');
    assert_same(90, $card['skills'][0]['power'], 'skill power');
    assert_same(15, $card['skills'][0]['pp'], 'skill current pp');
    assert_same(20, $card['skills'][0]['max_pp'], 'skill max pp');
    // 闪光大图链走 pmsg
    assert_same('https://img.tsdm39.com/Pokemon/pmsg/25.gif', $card['image_urls'][0],
        'shiny card image chain starts at CDN pmsg');
    assert_same('source/plugin/pokemon/images/pmsg/25.png', $card['image_urls'][1],
        'shiny card image chain falls back to local pmsg png');
    // hp=9999 必须被钳制到 max_hp（只读展示，不回写）
    if ($card['hp'] !== $card['max_hp'] || $card['hp'] <= 0) {
        $failures++;
        echo "FAIL: hp must clamp to max_hp (hp={$card['hp']}, max={$card['max_hp']})\n";
    }
    foreach (['hp', 'attack', 'defense', 'sp_attack', 'sp_defense', 'speed'] as $stat) {
        if (!isset($card['stats'][$stat])) {
            $failures++;
            echo "FAIL: stats missing $stat\n";
        }
    }
}

// --- 非闪光：大图链走 pm ---
$GLOBALS['card_case']['pet'] = array_merge(base_pet(), ['is_shiny' => 0, 'site' => 1]);
try {
    api_get_pokemon_card();
} catch (CardResponse $r) {
    assert_same('https://img.tsdm39.com/Pokemon/pm/25.gif', $r->data['image_urls'][0],
        'normal card image chain starts at CDN pm');
    assert_same(false, $r->data['is_legendary'], 'non-legendary card keeps flag false');
}

// --- 神兽 / 活动定制宠：卡片顶层 is_legendary=true ---
$GLOBALS['card_case']['species_legendary'] = true;
$GLOBALS['card_case']['pet'] = array_merge(base_pet(), ['site' => 1]);
try {
    api_get_pokemon_card();
} catch (CardResponse $r) {
    assert_same(true, $r->data['is_legendary'], 'legendary card flag');
}
$GLOBALS['card_case']['species_legendary'] = false;

// --- 未携带（site>=3）或不存在：404 ---
foreach ([['pet' => array_merge(base_pet(), ['site' => 5])], ['pet' => false]] as $case) {
    $GLOBALS['card_case']['pet'] = $case['pet'];
    try {
        api_get_pokemon_card();
        $failures++;
        echo "FAIL: uncarried/missing pet must error\n";
    } catch (CardResponse $r) {
        assert_same(404, $r->getCode(), 'missing/uncarried pet returns 404');
    }
}

if ($failures) {
    echo "pet card endpoint regression: $failures failure(s)\n";
    exit(1);
}
echo "pet card endpoint regression: all assertions passed\n";
