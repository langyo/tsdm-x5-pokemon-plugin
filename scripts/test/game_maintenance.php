<?php
/** Exercise the actual plugin router and API/admin dispatch while the game is closed. */
if (($argv[1] ?? '') === '--worker') {
    $case = json_decode(base64_decode($argv[2]), true, 512, JSON_THROW_ON_ERROR);
    define('IN_DISCUZ', true);
    $settings = ['poke_smgly' => 'fixture-staff'];
    if (array_key_exists('legacy', $case)) $settings['is_open'] = $case['legacy'];
    $_G = ['uid' => $case['uid'] ?? 7, 'username' => $case['staff'] ?? false ? 'fixture-staff' : 'fixture-player',
        'adminid' => !empty($case['admin']) ? 1 : 0, 'groupid' => !empty($case['admin']) ? 1 : 10,
        'cache' => ['plugin' => ['pokemon' => $settings]]];
    $_G['member'] = ['username' => $_G['username']];
    $_GET = ['action' => $case['action'] ?? 'maps'];
    if (!empty($case['endpoint'])) $_GET['endpoint'] = $case['endpoint'];
    if (!empty($case['index'])) $_GET['index'] = $case['index'];
    if (!empty($case['ui'])) $_GET['ui'] = $case['ui'];
    $_POST = !empty($case['admin_write']) ? ['action' => 'set::global_config', 'data' => '{"is_open":true}'] : [];
    $_SERVER['REQUEST_METHOD'] = $_POST ? 'POST' : ($case['method'] ?? 'GET');
    $_SERVER['HTTP_X_PM_FORMHASH'] = $case['token'] ?? 'test-formhash';
    function loadcache($key) {}
    function formhash() { return 'test-formhash'; }
    function lang($kind, $key) { return $key; }
    function showmessage($message) { echo json_encode(['success' => false, 'message' => $message]); exit; }
    function template($name) { throw new RuntimeException('Game template reached'); }
    class DB
    {
        public static $config;
        public static function table($table) { return $table; }
        public static function fetch_first($sql)
        {
            if (str_contains($sql, 'pm_config') && str_contains($sql, 'is_open')) return self::$config;
            if (str_contains($sql, 'pm_config')) return ['value' => '1', 'data_type' => 'string'];
            if (str_contains($sql, 'SHOW COLUMNS')) return false;
            throw new RuntimeException('Unexpected router read: ' . $sql);
        }
        public static function fetch_all($sql)
        {
            if (str_contains($sql, 'pm_config')) return array_values(array_filter([self::$config,
                ['key' => 'news_announcements', 'value' => '[]', 'data_type' => 'string']]));
            if (str_contains($sql, 'pm_map') || str_contains($sql, 'pm_data') || str_contains($sql, 'forum_forum')) return [];
            throw new RuntimeException('Unexpected router list: ' . $sql);
        }
        public static function result_first($sql) { return $GLOBALS['_G']['groupid']; }
        public static function query($sql)
        {
            if (preg_match("/UPDATE pm_config SET `value`='([^']*)' WHERE `key`='is_open'/", $sql, $m)) {
                self::$config['value'] = $m[1];
                file_put_contents($GLOBALS['case']['result'], json_encode(self::$config));
                return;
            }
            throw new RuntimeException('Unexpected router write: ' . $sql);
        }
    }
    DB::$config = array_key_exists('global', $case)
        ? ['key' => 'is_open', 'value' => $case['global'], 'data_type' => $case['data_type'] ?? 'boolean'] : false;
    try {
        if (($case['entry'] ?? '') === 'game') require __DIR__ . '/../../plugin/game.inc.php';
        else require __DIR__ . '/../../plugin/pokemon.inc.php';
    } catch (RuntimeException $error) {
        if ($error->getMessage() !== 'Game template reached') throw $error;
        echo json_encode(['success' => true, 'page' => 'game']);
    }
    exit;
}
$passed = 0;
function route_case($case)
{
    $result = tempnam(sys_get_temp_dir(), 'tsdm-route-');
    $case['result'] = $result;
    $process = proc_open([PHP_BINARY, __FILE__, '--worker', base64_encode(json_encode($case))],
        [0 => ['pipe', 'r'], 1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes);
    fclose($pipes[0]);
    $out = stream_get_contents($pipes[1]); fclose($pipes[1]);
    $err = stream_get_contents($pipes[2]); fclose($pipes[2]);
    $status = proc_close($process);
    $saved = file_get_contents($result); unlink($result);
    if ($status !== 0 || $err !== '') throw new RuntimeException($out . $err);
    return [json_decode($out, true, 512, JSON_THROW_ON_ERROR), $saved ? json_decode($saved, true) : null];
}
function check_route($ok, $label)
{
    if (!$ok) throw new RuntimeException($label);
    $GLOBALS['passed']++; echo 'PASS ', $label, PHP_EOL;
}
foreach ([0, 1] as $legacy) foreach ([0, 1] as $global) {
    [$r] = route_case(['legacy' => $legacy, 'global' => (string)$global, 'endpoint' => 'battle']);
    $open = $legacy && $global;
    check_route($open ? $r['success'] : (!$r['success'] && ($r['error_code'] ?? '') === 'game_closed'),
        "Both switches govern game APIs: legacy=$legacy global=$global");
}
foreach ([['legacy' => 0, 'global' => '1'], ['legacy' => 1, 'global' => '0']] as $flags) {
    [$r] = route_case($flags + ['endpoint' => 'battle', 'action' => 'start', 'method' => 'POST']);
    check_route(!$r['success'] && ($r['error_code'] ?? '') === 'game_closed', 'Closed game rejects mutations before gameplay');
    [$r] = route_case($flags + ['endpoint' => 'battle', 'index' => 'admin']);
    check_route(!$r['success'] && ($r['error_code'] ?? '') === 'game_closed', 'Admin page query cannot bypass game API closure');
    [$r] = route_case($flags + ['endpoint' => 'battle', 'staff' => true]);
    check_route($r['success'], 'Named Pokemon staff retains game access during maintenance');
    [$r] = route_case($flags + ['endpoint' => 'config', 'action' => 'global_config']);
    check_route($r['success'] && isset($r['data']['news_announcements']), 'Closure keeps announcement configuration readable');
}
foreach ([['legacy' => 1], ['legacy' => 1, 'global' => 'false'], ['legacy' => 1, 'global' => 'false', 'data_type' => 'string']] as $flags) {
    [$r] = route_case($flags + ['endpoint' => 'battle']);
    check_route($r['success'], 'Absent global row and truthy strings keep existing config semantics');
}
foreach ([['global' => '1'], ['legacy' => 1, 'global' => ''], ['legacy' => 1, 'global' => 'false', 'data_type' => 'integer']] as $flags) {
    [$r] = route_case($flags + ['endpoint' => 'battle']);
    check_route(!$r['success'] && ($r['error_code'] ?? '') === 'game_closed', 'Missing legacy switch and false parsed config remain closed');
}
[$r, $saved] = route_case(['legacy' => 0, 'global' => '0', 'index' => 'admin', 'admin' => true, 'admin_write' => true]);
check_route($r['success'] && $saved['value'] === '1', 'Administrator can reopen global configuration during maintenance');
[$r, $saved] = route_case(['legacy' => 0, 'global' => '0', 'index' => 'admin', 'admin_write' => true]);
check_route(!$r['success'] && $saved === null, 'Ordinary players cannot write through the admin exemption');
[$r] = route_case(['legacy' => 0, 'global' => '0', 'endpoint' => 'admin', 'action' => 'test_cleanup']);
check_route(!$r['success'] && ($r['code'] ?? 0) === 403, 'Admin API exemption preserves administrator authorization');
[$r] = route_case(['legacy' => 0, 'global' => '0', 'endpoint' => 'battle', 'token' => 'bad-token']);
check_route(!$r['success'] && ($r['code'] ?? 0) === 403, 'Closed API still requires a valid formhash');
[$r] = route_case(['legacy' => 0, 'global' => '0', 'endpoint' => 'unknown']);
check_route(!$r['success'] && ($r['code'] ?? 0) === 400, 'Unknown endpoint retains the original routing error');
foreach ([0, 1] as $legacy) foreach ([0, 1] as $global) {
    [$r] = route_case(['entry' => 'game', 'legacy' => $legacy, 'global' => (string)$global]);
    check_route($legacy && $global ? $r['success'] : (!$r['success'] && $r['message'] === 'system_closed'),
        "Direct game page honors both switches: legacy=$legacy global=$global");
}
[$r] = route_case(['entry' => 'game', 'legacy' => 0, 'global' => '0', 'staff' => true]);
check_route($r['success'] && $r['page'] === 'game', 'Named staff can open the direct game page during maintenance');
[$r] = route_case(['entry' => 'game', 'legacy' => 1, 'global' => '0', 'uid' => 0]);
check_route(!$r['success'] && $r['message'] === 'system_closed', 'Guests also see closure at the direct game page');
// 默认 admin 路由已是 tairitsu 壳（admin_v2.inc.php）：管理员维护期可渲染，
// 非管理员被 admin_only 拦下。
[$r] = route_case(['entry' => 'game', 'legacy' => 0, 'global' => '0', 'index' => 'admin', 'admin' => true]);
check_route($r['success'] && $r['page'] === 'game', 'Default admin route renders the tairitsu shell for admins during maintenance');
[$r] = route_case(['entry' => 'game', 'legacy' => 0, 'global' => '0', 'index' => 'admin', 'uid' => 0]);
check_route(!$r['success'] && $r['message'] === 'admin_only', 'Default admin route still rejects non-admins');
// Dioxus admincp 的维护期写配置路径经 &ui=v1 逃生参数进入。
[$r, $saved] = route_case(['entry' => 'game', 'legacy' => 0, 'global' => '0', 'index' => 'admin', 'admin' => true, 'admin_write' => true, 'ui' => 'v1']);
check_route($r['success'] && $saved['value'] === '1', 'Direct game admin route can reopen global configuration');
[$r, $saved] = route_case(['entry' => 'game', 'legacy' => 0, 'global' => '0', 'index' => 'admin', 'admin_write' => true, 'ui' => 'v1']);
check_route(!$r['success'] && $saved === null, 'Direct game admin route still rejects ordinary players');
[$r] = route_case(['legacy' => 1, 'global' => '1']);
check_route($r['success'] && $r['page'] === 'game', 'Router can include the game page with shared access helpers');
echo "Maintenance route checks passed: $passed", PHP_EOL;
