<?php
/**
 * Exercise the real item-use endpoint for trainer-level items (capacity box):
 * they must work without selecting a Pokemon, and must not be blocked by the
 * "Pokemon is in battle" guard that applies to per-Pokemon items.
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/utils.php';
require __DIR__ . '/../../plugin/api/constants.php';
require __DIR__ . '/../../plugin/api/item_modules.php';

function load_trainer_functions($path, $wanted)
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
load_trainer_functions(__DIR__ . '/../../plugin/api/user.php', ['api_use_item']);
load_trainer_functions(__DIR__ . '/../../plugin/api/index.php', ['pm_sql', 'pm_sql_v']);

class TrainerResponse extends RuntimeException
{
    public $data;
    public function __construct($code, $data)
    {
        parent::__construct(is_string($data) ? $data : 'success', $code);
        $this->data = $data;
    }
}
function api_success($data) { throw new TrainerResponse(200, $data); }
function api_error($message, $status = 400) { throw new TrainerResponse($status, $message); }
function require_login() {}
function pm_table($name) { return $name; }
function validate_uid($value) { return (int) $value; }
function validate_id($value, $name) { return (int) $value; }
function get_json_input() { return $GLOBALS['input']; }
function get_param($name, $default = null) { return $GLOBALS['input'][$name] ?? $default; }

class DB
{
    public static $user, $item, $stock, $writes;
    public static function fetch_first($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if ($sql === 'SELECT * FROM pm_usersdata WHERE uid = 7 FOR UPDATE') return self::$user;
        if ($sql === 'SELECT * FROM pm_itemdata WHERE id = 42') return self::$item;
        if ($sql === "SELECT * FROM pm_myitem WHERE uid = 7 AND itemid = '42'") return self::$stock;
        if ($sql === 'SELECT boxnum FROM pm_usersdata WHERE uid = 7') return ['boxnum' => self::$user['boxnum']];
        throw new RuntimeException('Unexpected read: ' . $sql);
    }
    public static function query($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (in_array($sql, ['START TRANSACTION', 'COMMIT', 'ROLLBACK'], true)) return;
        self::$writes[] = $sql;
        if (preg_match('/^UPDATE pm_usersdata SET boxnum = (\d+) WHERE uid = 7$/', $sql, $m)) {
            self::$user['boxnum'] = (int) $m[1];
        } elseif ($sql === 'DELETE FROM pm_myitem WHERE id = 2') {
            self::$stock = false;
        } elseif (preg_match('/^UPDATE pm_myitem SET nums = (\d+) WHERE id = 2$/', $sql, $m)) {
            self::$stock['nums'] = (int) $m[1];
        } else {
            throw new RuntimeException('Unexpected write: ' . $sql);
        }
    }
}

$GLOBALS['_G'] = ['uid' => 7];

/**
 * The capacity box is seeded as type=4 with sitemname=box9 (module column empty),
 * so module resolution must fall back to sitemname.
 */
function fixture($input, $npcid = 0)
{
    $GLOBALS['input'] = $input;
    DB::$user = ['uid' => 7, 'npcid' => $npcid, 'boxnum' => 9];
    DB::$item = ['id' => 42, 'name' => '容量箱子', 'type' => 4, 'module' => '', 'sitemname' => 'box9', 'tpname' => 'box', 'effects' => '{"hp":0}'];
    DB::$stock = ['id' => 2, 'uid' => 7, 'itemid' => 42, 'nums' => 2];
    DB::$writes = [];
}

function invoke()
{
    try { api_use_item(); } catch (TrainerResponse $response) { return $response; }
    throw new RuntimeException('No response');
}

$passed = 0;
function check($condition, $message)
{
    if (!$condition) throw new RuntimeException($message);
    $GLOBALS['passed']++;
}

// 1. No Pokemon selected: still succeeds and grows the capacity.
fixture(['item_id' => 42]);
$response = invoke();
check($response->getCode() === 200 && $response->data['success'] === true, 'Capacity box works without a Pokemon');
check(DB::$user['boxnum'] === 18, 'Capacity box adds its capacity without a Pokemon');
check(DB::$stock['nums'] === 1, 'Capacity box consumes one item');

// 2. A Pokemon that is currently in battle must not block a trainer-level item.
fixture(['item_id' => 42, 'pokemon_id' => 1], 5);
$response = invoke();
check($response->getCode() === 200 && DB::$user['boxnum'] === 18, 'Trainer item ignores the in-battle Pokemon guard');

// 3. A stray Pokemon parameter (even one that does not exist) is ignored, not validated.
fixture(['item_id' => 42, 'pokemon_id' => 999]);
$response = invoke();
check($response->getCode() === 200 && DB::$user['boxnum'] === 18, 'Trainer item ignores a stray Pokemon id');

// 4. use_target tells the client not to open the Pokemon picker.
check(api_item_use_target('box9', 4) === 'global', 'Capacity box targets the trainer, not a Pokemon');
check(api_item_use_target('hunger', 1) === 'pokemon', 'Ordinary healing items still target a Pokemon');
check(api_item_use_target('jlq', 2) === 'battle', 'Balls still target the battle');

echo "$passed trainer item assertions passed\n";
