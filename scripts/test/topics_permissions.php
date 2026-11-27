<?php
/** Real topics handler with Discuz table boundaries; optionally use native helpers and MariaDB. */
if (($argv[1] ?? '') === '--worker') {
    error_reporting(E_ALL);
    set_error_handler(function ($severity, $message, $file, $line) {
        throw new ErrorException($message, 0, $severity, $file, $line);
    });
    $case = json_decode(base64_decode($argv[2]), true, 512, JSON_THROW_ON_ERROR);
    define('IN_DISCUZ', true);
    $_GET = ['action' => 'list'];
    $_G = array_replace_recursive([
        'uid' => 7, 'adminid' => 0, 'groupid' => 10, 'username' => 'player',
        'member' => ['accessmasks' => 1, 'extgroupids' => ''], 'group' => ['readaccess' => 10],
        'setting' => ['forumstatus' => 1, 'verify' => ['enabled' => 0], 'plugins' => ['perm' => []]],
        'cookie' => [], 'cache' => ['plugin' => ['pokemon' => ['fid' => 2]]],
        'fid' => 99, 'forum' => ['fid' => 99, 'name' => 'Original context'],
    ], $case['global'] ?? []);
    $original_forum = $_G['forum'];
    $_G['forum'] = &$original_forum;
    $original_fid = $_G['fid'];
    $_G['fid'] = &$original_fid;
    $forum_reference = ReflectionReference::fromArrayElement($_G, 'forum')->getId();
    $fid_reference = ReflectionReference::fromArrayElement($_G, 'fid')->getId();
    $before = $_G;
    class DB {
        public static $connection;
        public static $thread_reads = 0;
        public static function table($name) { return $name; }
        public static function fetch_first($sql) {
            if (self::$connection) return self::$connection->query($sql)->fetch_assoc();
            if (strpos($sql, 'pm_config') !== false) return ['value' => '[{"title":"Public news","url":"https://example.com/news"}]'];
            throw new RuntimeException('Unexpected SQL: ' . $sql);
        }
        public static function fetch_all($sql) {
            if (strpos($sql, 'forum_thread') === false) throw new RuntimeException('Unexpected SQL: ' . $sql);
            self::$thread_reads++;
            if (self::$connection) return self::$connection->query($sql)->fetch_all(MYSQLI_ASSOC);
            return [['tid' => 91, 'subject' => 'Forum-only subject', 'author' => 'Author', 'authorid' => 1,
                'dateline' => 1700000000, 'displayorder' => 0, 'views' => 10, 'replies' => 2, 'readperm' => 255]];
        }
    }
    class C {
        public static function t($name) { return new TopicTable($name); }
    }
    if (($case['table_api'] ?? '') === 'x3') {
        // X3 can autoload these classes, but only C::t() is the table factory.
        class table_forum_forum {}
    } elseif (($case['table_api'] ?? '') === 'x5') {
        class table_forum_forum { public static function t() { return new TopicTable('forum_forum'); } }
    }
    class TopicTable {
        private $name;
        public function __construct($name) { $this->name = $name; }
        public function fetch_info_by_fid($fid) {
            if (DB::$connection) return DB::fetch_first('SELECT ff.*, f.* FROM forum_forum f LEFT JOIN forum_forumfield ff ON ff.fid=f.fid WHERE f.fid=' . (int)$fid);
            return $GLOBALS['case']['forum'];
        }
        public function fetch_all_by_fid_uid($fid, $uid) { return [['allowview' => $GLOBALS['case']['allowview'] ?? 0]]; }
        public function fetch_uid_by_fid_uid($fid, $uid) { return $GLOBALS['case']['moderator'] ?? false; }
        public function get_credits($uid, $fid) { return $GLOBALS['case']['paid'] ?? 0; }
        public function fetch_userinfo($uid, $fid) { return $GLOBALS['case']['groupuser'] ?? false; }
    }
    function memory($command, $key) { return [[]]; }
    function dunserialize($value) { return unserialize($value, ['allowed_classes' => false]); }
    // Native helper mode is opt-in and uses unmodified official source outside this repository.
    if ($path = getenv('TSDM_DISCUZ_FORUMPERM')) require $path;
    if ($path = getenv('TSDM_DISCUZ_GROUP')) require $path;
    if (empty($case['missing_permission_helper'])) {
        function forumperm($permission, $groupid = 0) {
            if (!empty($GLOBALS['case']['permission_error'])) throw new RuntimeException('Native permission failure');
            if (class_exists('helper_forumperm')) return (new helper_forumperm($permission))->check($groupid);
            return in_array((string)($groupid ?: $GLOBALS['_G']['groupid']), explode("\t", $permission), true);
        }
    }
    if (!function_exists('groupperm')) {
        function groupperm(&$forum, $uid, $action = '', $member = '') { return $GLOBALS['case']['group_result'] ?? ''; }
    }
    if (!empty($case['database'])) {
        if (!preg_match('/^tsdm_test_topics_[a-f0-9]{16}$/D', $case['database'])) exit('Invalid isolated schema');
        mysqli_report(MYSQLI_REPORT_ERROR | MYSQLI_REPORT_STRICT);
        DB::$connection = new mysqli(getenv('TSDM_DB_HOST') ?: '127.0.0.1', getenv('TSDM_DB_USER') ?: 'root',
            getenv('TSDM_DB_PASSWORD') ?: '', $case['database'], (int)(getenv('TSDM_DB_PORT') ?: 3306));
        DB::$connection->set_charset('utf8mb4');
    }
    class TopicResponse extends Exception { public $data; public function __construct($data) { $this->data = $data; } }
    function api_success($data) { throw new TopicResponse($data); }
    function api_error($message, $status = 400) { throw new RuntimeException($message, $status); }
    function get_param($key, $default = null) { return $_GET[$key] ?? $default; }
    require __DIR__ . '/../../plugin/admin/routes.php';
    try { require __DIR__ . '/../../plugin/api/topics.php'; }
    catch (TopicResponse $response) {
        $restored = $_G == $before && $forum_reference === ReflectionReference::fromArrayElement($_G, 'forum')->getId()
            && $fid_reference === ReflectionReference::fromArrayElement($_G, 'fid')->getId();
        echo json_encode(['data' => $response->data, 'restored' => $restored, 'thread_reads' => DB::$thread_reads], JSON_THROW_ON_ERROR);
    }
    exit;
}

$forum = ['fid' => 2, 'fup' => 1, 'type' => 'forum', 'status' => 1, 'viewperm' => '', 'password' => '',
    'formulaperm' => '', 'price' => 0, 'redirect' => '', 'level' => 1, 'moderators' => '',
    'founderuid' => 0, 'jointype' => 0, 'gviewperm' => 1];
$cases = [];
function add_topic_case($label, $allowed, $changes = []) {
    $GLOBALS['cases'][] = [$label, $allowed, array_replace_recursive(['forum' => $GLOBALS['forum']], $changes)];
}
add_topic_case('Public forum retains thread titles with a high individual readperm', true);
add_topic_case('Guest sees public forum titles', true, ['global' => ['uid' => 0, 'groupid' => 7]]);
add_topic_case('Empty viewperm follows forumdisplay even with zero group readaccess', true, ['global' => ['group' => ['readaccess' => 0]]]);
add_topic_case('Different forum group cannot read restricted metadata', false, ['forum' => ['viewperm' => '1']]);
add_topic_case('Guest cannot read member-only forum metadata', false, ['forum' => ['viewperm' => '10'], 'global' => ['uid' => 0, 'groupid' => 7]]);
add_topic_case('Explicitly allowed guest group can read forum metadata without warnings', true, ['forum' => ['viewperm' => "1\t7"], 'global' => ['uid' => 0, 'groupid' => 7]]);
add_topic_case('Allowed group sees restricted forum titles', true, ['forum' => ['viewperm' => "1\t10"]]);
add_topic_case('Explicit individual grant overrides group restriction', true, ['forum' => ['viewperm' => '1'], 'allowview' => 1]);
add_topic_case('Explicit individual denial hides public titles', false, ['allowview' => -1]);
add_topic_case('Plugin administrator name alone does not grant forum access', false, ['forum' => ['viewperm' => '1'], 'global' => ['cache' => ['plugin' => ['pokemon' => ['poke_smgly' => 'player']]]]]);
add_topic_case('Native administrator still needs viewperm', false, ['forum' => ['viewperm' => '10'], 'global' => ['groupid' => 1, 'adminid' => 1]]);
add_topic_case('Password forum with no cookie hides metadata', false, ['forum' => ['password' => 'forum-secret']]);
add_topic_case('Password forum rejects a wrong cookie', false, ['forum' => ['password' => 'forum-secret'], 'global' => ['cookie' => ['fidpw2' => 'wrong']]]);
add_topic_case('Password forum accepts the native verified cookie', true, ['forum' => ['password' => 'forum-secret'], 'global' => ['cookie' => ['fidpw2' => 'forum-secret']]]);
add_topic_case('Moderator cannot bypass forum password', false, ['forum' => ['password' => 'forum-secret'], 'global' => ['adminid' => 2]]);
add_topic_case('Unpaid forum metadata stays hidden', false, ['forum' => ['price' => 10]]);
add_topic_case('Partial forum payment does not grant preview', false, ['forum' => ['price' => 10], 'paid' => 9]);
add_topic_case('Fully paid forum allows preview', true, ['forum' => ['price' => 10], 'paid' => 10]);
add_topic_case('Native moderator bypasses forum payment', true, ['forum' => ['price' => 10], 'global' => ['adminid' => 3], 'moderator' => true]);
add_topic_case('Moderator of another forum still pays', false, ['forum' => ['price' => 10], 'global' => ['adminid' => 3]]);
add_topic_case('Legacy formula is not evaluated by the preview endpoint', false, ['forum' => ['formulaperm' => 'legacy serialized formula']]);
add_topic_case('Native moderator retains formula exemption', true, ['forum' => ['formulaperm' => 'legacy serialized formula'], 'global' => ['adminid' => 2]]);
add_topic_case('Missing forum returns only public news', false, ['forum' => []]);
$cases[count($cases)-1][2]['forum'] = [];
add_topic_case('Category cannot expose attached topic metadata', false, ['forum' => ['type' => 'group']]);
add_topic_case('Redirect forum cannot expose attached topic metadata', false, ['forum' => ['redirect' => 'https://example.com/']]);
add_topic_case('No permission helper leaves public forums readable', true, ['missing_permission_helper' => true]);
add_topic_case('No permission helper does not bypass a restricted forum', false, ['missing_permission_helper' => true, 'forum' => ['viewperm' => '1']]);
add_topic_case('Closed native forum service returns only news', false, ['global' => ['setting' => ['forumstatus' => 0]]]);
add_topic_case('Group pending verification is hidden', false, ['forum' => ['status' => 3, 'type' => 'sub', 'level' => -1]]);
add_topic_case('Private social group rejects outsiders', false, ['forum' => ['status' => 3, 'type' => 'sub', 'gviewperm' => 0], 'group_result' => 2]);
add_topic_case('Social group member may preview', true, ['forum' => ['status' => 3, 'type' => 'sub', 'gviewperm' => 0], 'group_result' => 'isgroupuser', 'groupuser' => ['uid' => 7, 'level' => 4]]);
add_topic_case('Public social group permits an outsider without PHP warnings', true, ['forum' => ['status' => 3, 'type' => 'sub']]);
add_topic_case('Public social group permits a guest without PHP warnings', true, ['forum' => ['status' => 3, 'type' => 'sub'], 'global' => ['uid' => 0, 'groupid' => 7]]);
add_topic_case('Group founder retains native access to a private group', true, ['forum' => ['status' => 3, 'type' => 'sub', 'gviewperm' => 0, 'founderuid' => 7], 'group_result' => 'isgroupuser']);
add_topic_case('Closed group rejects outsiders despite public view flag', false, ['forum' => ['status' => 3, 'type' => 'sub', 'jointype' => -1], 'group_result' => 1]);
add_topic_case('Unapproved group member remains denied', false, ['forum' => ['status' => 3, 'type' => 'sub', 'gviewperm' => 0, 'jointype' => 2], 'group_result' => 3, 'groupuser' => ['uid' => 7, 'level' => 0]]);
add_topic_case('X3 loaded table class without static factory stays compatible', true, ['table_api' => 'x3']);
add_topic_case('X3 table path still blocks restricted forums', false, ['table_api' => 'x3', 'forum' => ['viewperm' => '1']]);
add_topic_case('X5 native table factory supports public topics', true, ['table_api' => 'x5']);
add_topic_case('X5 table path still blocks restricted forums', false, ['table_api' => 'x5', 'forum' => ['viewperm' => '1']]);
add_topic_case('Permission failure restores context and keeps public news', false, ['permission_error' => true, 'forum' => ['viewperm' => '1']]);

$passed = $failed = 0;
$connection = null;
$owned = false;
try {
    if (getenv('TSDM_TOPICS_REAL_DB') === '1') {
        mysqli_report(MYSQLI_REPORT_ERROR | MYSQLI_REPORT_STRICT);
        $connection = new mysqli(getenv('TSDM_DB_HOST') ?: '127.0.0.1', getenv('TSDM_DB_USER') ?: 'root',
            getenv('TSDM_DB_PASSWORD') ?: '', '', (int)(getenv('TSDM_DB_PORT') ?: 3306));
        $database = 'tsdm_test_topics_' . bin2hex(random_bytes(8));
        $connection->query('CREATE DATABASE `' . $database . '` CHARACTER SET utf8mb4');
        $owned = true;
        $connection->select_db($database);
        $connection->query('CREATE TABLE forum_forum (fid INT PRIMARY KEY, fup INT, type VARCHAR(10), status INT)');
        $connection->query('CREATE TABLE forum_forumfield (fid INT PRIMARY KEY, viewperm TEXT, password TEXT, formulaperm TEXT, price INT, redirect TEXT, level INT, moderators TEXT, founderuid INT, jointype INT, gviewperm INT)');
        $connection->query('CREATE TABLE forum_thread (tid INT, fid INT, subject TEXT, author TEXT, authorid INT, dateline INT, displayorder INT, views INT, replies INT, lastpost INT, readperm INT)');
        $connection->query("INSERT INTO forum_thread VALUES (91,2,'Forum-only subject','Author',1,1700000000,0,10,2,1700000000,255)");
        $connection->query('CREATE TABLE pm_config (`key` VARCHAR(100) PRIMARY KEY, value TEXT)');
        $connection->query("INSERT INTO pm_config VALUES ('news_announcements','[{\"title\":\"Public news\",\"url\":\"https://example.com/news\"}]')");
    }
    // 夹具行按列名绑定插入。列名是本文件里的常量，值全部走参数——不再把值
    // 转义成 SQL 字面量拼进语句。
    $insert_row = static function (mysqli $connection, string $table, array $columns, array $row): void {
        $statement = $connection->prepare(
            'INSERT INTO `' . $table . '` (`' . implode('`,`', $columns) . '`) VALUES ('
            . implode(',', array_fill(0, count($columns), '?')) . ')'
        );
        $values = array_map('strval', array_map(fn($column) => $row[$column], $columns));
        $statement->bind_param(str_repeat('s', count($columns)), ...$values);
        $statement->execute();
    };
    foreach ($cases as [$label, $allowed, $case]) {
        if ($connection) {
            $connection->query('DELETE FROM forum_forum');
            $connection->query('DELETE FROM forum_forumfield');
            if ($case['forum']) {
                $insert_row($connection, 'forum_forum', ['fid', 'fup', 'type', 'status'], $case['forum']);
                $insert_row($connection, 'forum_forumfield',
                    ['fid', 'viewperm', 'password', 'formulaperm', 'price', 'redirect', 'level', 'moderators', 'founderuid', 'jointype', 'gviewperm'],
                    $case['forum']);
            }
            $case['database'] = $database;
        }
        $command = [PHP_BINARY];
        if ($ini = php_ini_loaded_file()) array_push($command, '-c', $ini);
        array_push($command, '-d', 'display_errors=stderr', '-d', 'log_errors=0', __FILE__, '--worker', base64_encode(json_encode($case)));
        $process = proc_open($command, [0 => ['pipe','r'], 1 => ['pipe','w'], 2 => ['pipe','w']], $pipes);
        fclose($pipes[0]);
        $output = stream_get_contents($pipes[1]); fclose($pipes[1]);
        $error = stream_get_contents($pipes[2]); fclose($pipes[2]);
        $status = proc_close($process);
        $result = json_decode($output, true);
        $ok = $status === 0 && $error === '' && ($result['data']['total'] ?? -1) === ($allowed ? 1 : 0)
            && ($result['thread_reads'] ?? -1) === ($allowed ? 1 : 0) && !empty($result['restored'])
            && ($result['data']['news_announcements'][0]['title'] ?? '') === 'Public news';
        if ($ok) { $passed++; echo 'PASS ', $label, PHP_EOL; }
        else { $failed++; echo 'FAIL ', $label, ': ', $output, $error, PHP_EOL; }
    }
} finally {
    try {
        if ($owned) {
            // Only the name generated and exclusively created by this process can be dropped.
            if (!preg_match('/^tsdm_test_topics_[a-f0-9]{16}$/D', $database)) throw new RuntimeException('Unsafe database name');
            $connection->query('DROP DATABASE `' . $database . '`');
        }
    } finally {
        if ($connection) $connection->close();
    }
}
echo "Topics permission checks: $passed passed, $failed failed", PHP_EOL;
exit($failed ? 1 : 0);
