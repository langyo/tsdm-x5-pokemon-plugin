<?php
/** Exercise the real spacecp refresh route with Discuz DB method signatures. */
if (($argv[1] ?? '') === '--worker') {
    $case = json_decode(base64_decode($argv[2]), true, 512, JSON_THROW_ON_ERROR);
    define('IN_DISCUZ', true);
    set_error_handler(function ($severity, $message) { throw new RuntimeException($message); });
    $_G = ['uid' => $case['uid'] ?? 7];
    $_GET = $case['query'] ?? [];
    function showmessage($message, $url = '', $values = [], $options = [])
    {
        echo json_encode(['message' => $message, 'options' => $options,
            'members' => DB::$members, 'reads' => DB::$reads, 'writes' => DB::$writes], JSON_THROW_ON_ERROR);
        exit;
    }
    class DB
    {
        public static $members = [7 => 'existing-badge', 8 => 'other-member-badge'];
        public static $reads = 0;
        public static $writes = 0;
        public static function table($table) { return ($GLOBALS['case']['prefix'] ?? '') . $table; }
        public static function query($sql, $arg = [], $silent = false, $unbuffered = false)
        {
            $sql = preg_replace_callback('/%[td]/', function ($match) use (&$arg) {
                $value = array_shift($arg);
                return $match[0] === '%t' ? self::table($value) : (string)(int)$value;
            }, $sql);
            if (!preg_match('/\bFROM (\w+) WHERE uid=(\d+) AND site < 3$/', $sql, $matches)) {
                throw new RuntimeException('Unexpected pet query: ' . $sql);
            }
            self::$reads++;
            if ($matches[1] !== 'pm_mypm' || !empty($GLOBALS['case']['read_failure'])) {
                if ($silent) return false;
                throw new RuntimeException('Database query failed');
            }
            $rows = [];
            if (empty($GLOBALS['case']['empty'])) {
                // Include another member and a stored pet so the SQL filters matter.
                foreach ([[11, 7, 1], [12, 7, 2], [13, 7, 0], [14, 7, 3], [15, 8, 1]] as [$id, $uid, $site]) {
                    if ($uid != $matches[2] || $site >= 3) continue;
                    $rows[] = ['id' => $id, 'species_id' => 25, 'nickname' => "宠物 ' $id",
                        'level' => 10, 'site' => $site, 'is_shiny' => $id === 11 ? 1 : 0];
                }
            }
            return (object)['rows' => $rows];
        }
        public static function fetch($query) { return array_shift($query->rows); }
        public static function free_result($query) {}
        public static function fetch_all($sql, $arg = [], $keyfield = '', $silent = false)
        {
            $query = self::query($sql, $arg, $silent);
            return $query ? $query->rows : [];
        }
        // Discuz's fourth parameter is an unbuffered flag, not SQL arguments.
        public static function update($table, $data, $condition = '', $unbuffered = false, $low_priority = false)
        {
            if ($table !== 'common_member_field_forum' || !is_bool($unbuffered)
                || !is_array($condition) || array_keys($condition) !== ['uid']) {
                throw new RuntimeException('Invalid Discuz member update');
            }
            self::table($table);
            self::$members[$condition['uid']] = $data['pokemon'];
            self::$writes++;
            return true;
        }
    }
    require __DIR__ . '/../../plugin/refresh.inc.php';
    throw new RuntimeException('Refresh route did not show a result');
}

$passed = 0;
$failed = 0;
function check_refresh($case, $expect, $label)
{
    $process = proc_open([PHP_BINARY, '-d', 'display_errors=stderr', '-d', 'log_errors=0',
        __FILE__, '--worker', base64_encode(json_encode($case, JSON_THROW_ON_ERROR))],
        [0 => ['pipe', 'r'], 1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes);
    if (!is_resource($process)) throw new RuntimeException('Cannot run refresh worker');
    fclose($pipes[0]);
    $output = stream_get_contents($pipes[1]); fclose($pipes[1]);
    $error = stream_get_contents($pipes[2]); fclose($pipes[2]);
    $status = proc_close($process);
    $result = json_decode($output, true);
    if ($status === 0 && $error === '' && is_array($result) && $expect($result)) {
        $GLOBALS['passed']++;
        echo 'PASS ', $label, PHP_EOL;
    } else {
        $GLOBALS['failed']++;
        echo 'FAIL ', $label, PHP_EOL, $error;
    }
}

$preserved = fn($r) => $r['members'] === [7 => 'existing-badge', 8 => 'other-member-badge'] && $r['writes'] === 0;
$refreshed = function ($r) {
    $badge = unserialize($r['members'][7], ['allowed_classes' => false]);
    return $r['message'] === '已刷新状态栏' && ($r['options']['alert'] ?? '') === 'right'
        && $r['reads'] === 1 && $r['writes'] === 1 && $r['members'][8] === 'other-member-badge'
        && $badge['first'] === ['id' => 11, 'species_id' => 25, 'nickname' => "宠物 ' 11",
            'level' => 10, 'site' => 1, 'is_shiny' => 1]
        && array_column($badge['creeps'], 'id') === [12, 13];
};
foreach (['', 'forum_'] as $prefix) {
    check_refresh(['prefix' => $prefix, 'query' => ['hide' => '0', 'uid' => 8]], $refreshed,
        "Refresh uses fixed pet table and authenticated member with prefix '$prefix'");
    check_refresh(['prefix' => $prefix], $refreshed, "Missing hide defaults to refresh with prefix '$prefix'");
    check_refresh(['prefix' => $prefix, 'query' => ['hide' => '1'], 'read_failure' => true],
        fn($r) => $r['message'] === '已隐藏状态栏' && $r['members'] === [7 => '', 8 => 'other-member-badge']
            && $r['reads'] === 0 && $r['writes'] === 1,
        "Hide works without reading the pet table with prefix '$prefix'");
    check_refresh(['prefix' => $prefix, 'query' => ['hide' => '0'], 'read_failure' => true],
        fn($r) => $preserved($r) && $r['reads'] === 1 && str_contains($r['message'], '原状态栏已保留'),
        "Failed read preserves the existing badge with prefix '$prefix'");
    check_refresh(['prefix' => $prefix, 'empty' => true],
        fn($r) => $r['message'] === '已刷新状态栏' && $r['writes'] === 1 && $r['members'][8] === 'other-member-badge'
            && unserialize($r['members'][7], ['allowed_classes' => false]) === ['creeps' => []],
        "Empty successful result clears stale pets with prefix '$prefix'");
}
foreach ([[], ['hide' => '1']] as $query) {
    check_refresh(['uid' => 0, 'query' => $query],
        fn($r) => $r['message'] === '错误: 没有登录' && $preserved($r) && $r['reads'] === 0,
        'Guests cannot read or update a badge');
}
echo "Forum status refresh regressions: $passed passed, $failed failed", PHP_EOL;
exit($failed ? 1 : 0);
