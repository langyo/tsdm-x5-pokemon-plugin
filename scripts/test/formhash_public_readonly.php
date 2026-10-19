<?php
/** formhash 豁免回归：侧栏宠物卡片（公开只读 GET）不被 403，其余请求仍被拦。 */
error_reporting(E_ALL);
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/security.php';

$failures = 0;
function assert_same($expected, $actual, $label)
{
    global $failures;
    if ($expected !== $actual) {
        $failures++;
        echo "FAIL: $label\n  expected: " . var_export($expected, true) . "\n  actual:   " . var_export($actual, true) . "\n";
    }
}

// --- 子进程门禁用例：真实 include api/index.php，验证放行/拦截 ---
// 用法：php formhash_public_readonly.php gate-case <action> <method>
if (isset($argv[1]) && $argv[1] === 'gate-case') {
    $_SERVER['REQUEST_METHOD'] = isset($argv[3]) ? $argv[3] : 'GET';
    $_GET = ['endpoint' => 'pokemon', 'action' => $argv[2]];
    // 跳过文件尾部的“未路由 404”，只考察 formhash 门禁本身
    define('API_ROUTED', true);
    require __DIR__ . '/../../plugin/api/index.php';
    exit(0);
}

function run_gate_case($action, $method = 'GET')
{
    $proc = proc_open(
        [PHP_BINARY, __FILE__, 'gate-case', $action, $method],
        [1 => ['pipe', 'w'], 2 => ['pipe', 'w']],
        $pipes
    );
    if (!is_resource($proc)) {
        return [false, ''];
    }
    $stdout = stream_get_contents($pipes[1]);
    $stderr = stream_get_contents($pipes[2]);
    fclose($pipes[1]);
    fclose($pipes[2]);
    return [proc_close($proc), $stdout . $stderr];
}

// --- pm_public_readonly_request 白名单 ---
$_SERVER['REQUEST_METHOD'] = 'GET';
$_GET = ['endpoint' => 'pokemon', 'action' => 'card'];
assert_same(true, pm_public_readonly_request(), 'GET pokemon/card is exempted');

$_SERVER['REQUEST_METHOD'] = 'POST';
assert_same(false, pm_public_readonly_request(), 'POST pokemon/card is NOT exempted');

$_SERVER['REQUEST_METHOD'] = 'GET';
$_GET = ['endpoint' => 'pokemon', 'action' => 'list'];
assert_same(false, pm_public_readonly_request(), 'GET pokemon/list is NOT exempted');

$_GET = ['endpoint' => 'shop', 'action' => 'card'];
assert_same(false, pm_public_readonly_request(), 'endpoint other than pokemon is NOT exempted');

$_GET = ['endpoint' => 'pokemon'];
assert_same(false, pm_public_readonly_request(), 'missing action is NOT exempted');

$_GET = ['endpoint' => 'pokemon', 'action' => ['card']];
assert_same(false, pm_public_readonly_request(), 'array action is NOT exempted');

// --- api/index.php 门禁集成（无任何 formhash 供给的访客视角） ---
list($code, $out) = run_gate_case('card');
assert_same(0, $code, 'card gate-case exits cleanly');
assert_same(false, strpos($out, 'formhash') !== false, 'card request passes the api gate without formhash');

list($code, $out) = run_gate_case('list');
assert_same(true, strpos($out, 'formhash 校验失败') !== false,
    'list request must be blocked by the api gate');

list($code, $out) = run_gate_case('card', 'POST');
assert_same(true, strpos($out, 'formhash 校验失败') !== false,
    'POST card request must be blocked by the api gate');

if ($failures) {
    echo "formhash public readonly regression: $failures failure(s)\n";
    exit(1);
}
echo "formhash public readonly regression: all assertions passed\n";
