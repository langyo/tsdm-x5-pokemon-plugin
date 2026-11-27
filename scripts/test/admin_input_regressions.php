<?php
/** Exercise admin input round trips through production dispatch and isolated SQL storage. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

class DB
{
    public static $tables = ['pm_data' => [1 => ['id' => 1]], 'pm_evolution' => [], 'pm_config' => []];
    public static $fail_news_insert = false;
    public static function errno() { return 1142; }
    public static $next_ids = [];
    public static $last_insert_id = 0;

    public static function value($raw)
    {
        $raw = trim($raw);
        if (!preg_match("/^(?:'(?:\\\\.|[^'\\\\])*'|-?\d+)$/s", $raw)) {
            throw new RuntimeException('Malformed SQL value: ' . $raw);
        }
        if ($raw[0] !== "'") return (int) $raw;
        // Model the SQL string escaping used by Discuz's MySQL connection.
        return preg_replace_callback('/\\\\(.)/s', function ($match) {
            $escapes = ['0' => "\0", 'b' => "\x08", 'n' => "\n", 'r' => "\r", 't' => "\t", 'Z' => "\x1a"];
            return $escapes[$match[1]] ?? $match[1];
        }, substr($raw, 1, -1));
    }

    public static function fetch_all($sql)
    {
        if (strpos($sql, 'FROM forum_thread') !== false) return [];
        if (!preg_match('/^SELECT (?:\*|id) from (\w+)(.*)$/i', trim($sql), $match)) {
            throw new RuntimeException('Unexpected read: ' . $sql);
        }
        if (!isset(self::$tables[$match[1]])) throw new RuntimeException('Unexpected table: ' . $sql);
        $rows = array_values(self::$tables[$match[1]]);
        $tail = trim($match[2]);
        if ($tail === '') return $rows;
        if (preg_match('/^where `?(id|from_id|key)`?\s*=\s*(.+)$/is', $tail, $where)) {
            $value = self::value($where[2]);
            return array_values(array_filter($rows, function ($row) use ($where, $value) {
                return $row[$where[1]] == $value;
            }));
        }
        if (strtolower($tail) === 'order by id desc limit 1') {
            usort($rows, function ($a, $b) { return $b['id'] <=> $a['id']; });
            return array_slice($rows, 0, 1);
        }
        throw new RuntimeException('Unexpected read condition: ' . $sql);
    }

    public static function fetch_first($sql)
    {
        if ($sql === "SHOW COLUMNS FROM pm_config LIKE 'value'") return ['Type' => 'longtext'];
        $rows = self::fetch_all($sql);
        return $rows[0] ?? false;
    }

    public static function table($name) { return $name; }

    public static function query($sql)
    {
        if (self::$fail_news_insert && str_starts_with($sql, 'INSERT') && str_contains($sql, 'news_announcements')) return false;
        if (preg_match('/^UPDATE (\w+) set `?(\w+)`?\s*=\s*(.*?) where `?(id|key)`?\s*=\s*(.+)$/is', trim($sql), $match)) {
            $id = self::value($match[5]);
            if (!isset(self::$tables[$match[1]][$id])) throw new RuntimeException('Unknown update row');
            self::$tables[$match[1]][$id][$match[2]] = self::value($match[3]);
            return true;
        }
        if (preg_match('/^INSERT(?: IGNORE)? INTO (\w+)\s*\((.*?)\)\s*VALUES\s*\((.*?)\)( ON DUPLICATE KEY UPDATE (?:`key` = `key`|`value` = VALUES\(`value`\), `data_type` = VALUES\(`data_type`\)))?$/is', trim($sql), $match)) {
            $columns = array_map(function ($value) { return trim($value, " \t\r\n`"); }, explode(',', $match[2]));
            $literal = "(?:'(?:\\\\.|[^'\\\\])*'|-?\d+)";
            if (!preg_match('/^' . $literal . '(?:\s*,\s*' . $literal . ')*$/s', trim($match[3]))) {
                throw new RuntimeException('Malformed INSERT values');
            }
            preg_match_all("/'(?:\\\\.|[^'\\\\])*'|-?\d+/s", $match[3], $values);
            if (count($columns) !== count($values[0])) throw new RuntimeException('Malformed INSERT columns');
            $row = array_combine($columns, array_map([self::class, 'value'], $values[0]));
            if (($match[4] ?? '') === ' ON DUPLICATE KEY UPDATE `key` = `key`' && isset(self::$tables[$match[1]][$row['key']])) return true;
            if ($match[1] !== 'pm_config') {
                $table = $match[1];
                if (!isset(self::$next_ids[$table])) {
                    self::$next_ids[$table] = max(array_merge([0], array_keys(self::$tables[$table]))) + 1;
                }
                if (!isset($row['id'])) $row['id'] = self::$next_ids[$table];
                self::$next_ids[$table] = max(self::$next_ids[$table], $row['id'] + 1);
                self::$last_insert_id = $row['id'];
            }
            self::$tables[$match[1]][$row['id'] ?? $row['key']] = $row;
            return true;
        }
        throw new RuntimeException('Unexpected write: ' . $sql);
    }

    public static function insert_id() { return self::$last_insert_id; }
}

function api_success($data) { exit(json_encode(['success' => true, 'data' => $data], JSON_UNESCAPED_UNICODE)); }
function api_error($message, $code = 400) { throw new RuntimeException($message, $code); }
function get_param($name, $default = null) { return $_GET[$name] ?? $_POST[$name] ?? $default; }

if (isset($argv[1])) {
    $request = json_decode(base64_decode($argv[1]), true, 512, JSON_THROW_ON_ERROR);
    DB::$tables = $request['tables'] ?? DB::$tables;
    DB::$fail_news_insert = !empty($request['fail_news_insert']);
    register_shutdown_function(function () { fwrite(STDERR, json_encode(DB::$tables, JSON_UNESCAPED_UNICODE)); });
    define('IN_DISCUZ', true);
    $_POST = $request['params'] ?? [];
    $_GET = [];
    $_G = [];
    $plugin = __DIR__ . '/../../plugin';
    try {
        if (isset($request['endpoint'])) {
            require $plugin . '/admin/routes.php';
            $_GET['action'] = $request['endpoint'] === 'config' ? 'global_config' : 'list';
            require $plugin . '/api/' . $request['endpoint'] . '.php';
        } else {
            require $plugin . '/admin/dispatch.php';
        }
    } catch (Throwable $error) {
        echo json_encode(['success' => false, 'reason' => $error->getMessage()], JSON_UNESCAPED_UNICODE);
        exit(1);
    }
    throw new RuntimeException('Endpoint should terminate');
}

$passed = 0;
$failed = 0;
function check_input($name, $test)
{
    global $passed, $failed;
    try {
        $test();
        $passed++;
    } catch (Throwable $error) {
        $failed++;
        echo "FAIL: $name: {$error->getMessage()}\n";
    }
}

function expect_input($actual, $expected)
{
    if ($actual !== $expected) {
        throw new RuntimeException(json_encode(['expected' => $expected, 'actual' => $actual], JSON_UNESCAPED_UNICODE));
    }
}

function input_request($request)
{
    $pipes = [];
    $process = proc_open([PHP_BINARY, __FILE__, base64_encode(json_encode($request, JSON_UNESCAPED_UNICODE))], [
        0 => ['pipe', 'r'], 1 => ['pipe', 'w'], 2 => ['pipe', 'w'],
    ], $pipes, null, null, ['bypass_shell' => true]);
    if (!is_resource($process)) throw new RuntimeException('Cannot start input test subprocess');
    fclose($pipes[0]);
    $stdout = stream_get_contents($pipes[1]);
    $stderr = stream_get_contents($pipes[2]);
    fclose($pipes[1]);
    fclose($pipes[2]);
    $status = proc_close($process);
    $response = json_decode($stdout, true, 512, JSON_THROW_ON_ERROR);
    if ($status !== 0 || ($response['success'] ?? false) !== true) {
        throw new RuntimeException($response['reason'] ?? $stdout);
    }
    return [$response['data'], json_decode($stderr, true, 512, JSON_THROW_ON_ERROR)];
}

function admin_input_request($action, $data, $tables = null)
{
    $request = ['params' => ['action' => $action, 'data' => json_encode($data, JSON_UNESCAPED_UNICODE)]];
    if ($tables !== null) $request['tables'] = $tables;
    return input_request($request);
}

function pokemon_input()
{
    return [
        'name' => 'test species', 'description' => 'test description', 'cost' => 123, 'is_selling' => true,
        'sex_weight' => 0.5, 'initial_statistic' => array_fill_keys([
            'hit_points', 'attack', 'defense', 'special_attack', 'special_defense', 'speed',
        ], 50), 'initial_base_points' => array_fill_keys([
            'hit_points', 'attack', 'defense', 'special_attack', 'special_defense', 'speed',
        ], 1), 'kind' => ['normal', null], 'is_legendary' => false, 'map_ids' => [],
        'capture_weight' => 45, 'meet_weight' => 100, 'birth_order' => 1,
        'strength_weight' => 80, 'drop_money_range' => [10, 20],
    ];
}

foreach ([[45, 100], [0, 0], [1, 1], [255, 10000]] as $weights) {
    check_input('Species numeric weights ' . implode('/', $weights), function () use ($weights) {
        $payload = pokemon_input();
        [$payload['capture_weight'], $payload['meet_weight']] = $weights;
        [$saved, $tables] = admin_input_request('insert::pokemon_type', $payload);
        expect_input([$saved[0]['capture_weight'], $saved[0]['meet_weight']], $weights);
        expect_input([$tables['pm_data'][2]['capture'], $tables['pm_data'][2]['met']], $weights);
        [$savedAgain] = admin_input_request('set::pokemon_type', $saved[0], $tables);
        expect_input([$savedAgain[0]['capture_weight'], $savedAgain[0]['meet_weight']], $weights);
    });
}

foreach (["Farfetch'd", 'C:\\new\\bird', '中文 "描述", with punctuation', ''] as $text) {
    foreach (['name', 'description'] as $field) {
        check_input("Species $field text " . json_encode($text), function () use ($field, $text) {
            $payload = pokemon_input();
            $payload[$field] = $text;
            [$saved, $tables] = admin_input_request('insert::pokemon_type', $payload);
            expect_input($saved[0][$field], $text);
            expect_input($tables['pm_data'][2][$field], $text);
            $saved[0][$field] = $text . " changed's \\path";
            [$updated, $tables] = admin_input_request('set::pokemon_type', $saved[0], $tables);
            expect_input($updated[0][$field], $saved[0][$field]);
            expect_input($tables['pm_data'][2][$field], $saved[0][$field]);
        });
    }
}

$announcements = [
    [['title' => 'Plain title', 'url' => 'https://example.com/news']],
    [['title' => 'The "new" update', 'url' => 'https://example.com/news?q="a"']],
    [['title' => 'Folder C:\\news\\today', 'url' => 'https://example.com/news']],
    [['title' => "Trainer's update", 'url' => 'https://example.com/?q=trainer%27s']],
    [['title' => '繁體中文公告', 'url' => 'https://example.com/news']],
    [],
];
foreach ($announcements as $index => $news) {
    foreach (['insert', 'update'] as $operation) {
        check_input("Announcements $operation case $index", function () use ($operation, $news) {
            $tables = DB::$tables;
            if ($operation === 'update') {
                $tables['pm_config']['news_announcements'] = [
                    'key' => 'news_announcements', 'data_type' => 'string', 'value' => '[]',
                ];
            }
            [$saved, $tables] = admin_input_request('set::global_config', ['news_announcements' => $news], $tables);
            expect_input($saved[0]['news_announcements'], $news);
            expect_input(json_decode($tables['pm_config']['news_announcements']['value'], true), $news);
            [$savedAgain] = admin_input_request('get::global_config', [], $tables);
            expect_input($savedAgain[0]['news_announcements'], $news);
        });
    }
    foreach (['config', 'topics'] as $endpoint) {
        check_input("Announcements public $endpoint case $index", function () use ($endpoint, $news) {
            [, $tables] = admin_input_request('set::global_config', ['news_announcements' => $news]);
            [$data] = input_request(['endpoint' => $endpoint, 'tables' => $tables]);
            expect_input($data['news_announcements'], $news);
        });
    }
}

check_input('Public config preserves ordinary string escapes', function () {
    $title = 'Folder C:\\news\\today';
    [, $tables] = admin_input_request('set::global_config', ['ann_title' => $title]);
    [$data] = input_request(['endpoint' => 'config', 'tables' => $tables]);
    expect_input($data['ann_title'], $title);
});

check_input('Topics legacy announcement migration preserves text', function () {
    $title = 'Trainer\'s "news" C:\\news\\today';
    $url = 'https://example.com/?q="news"';
    $tables = DB::$tables;
    foreach (['ann_title' => $title, 'ann_url' => $url] as $key => $value) {
        $tables['pm_config'][$key] = ['key' => $key, 'value' => $value, 'data_type' => 'string'];
    }
    [$data, $tables] = input_request(['endpoint' => 'topics', 'tables' => $tables]);
    $expected = [['title' => $title, 'url' => $url]];
    expect_input($data['news_announcements'], $expected);
    expect_input(json_decode($tables['pm_config']['news_announcements']['value'], true), $expected);
    [$readAgain] = input_request(['endpoint' => 'topics', 'tables' => $tables]);
    expect_input($readAgain['news_announcements'], $expected);
});

check_input('Topics retains its six announcement limit', function () {
    $news = array_fill(0, 8, ['title' => 'Title', 'url' => 'https://example.com']);
    [, $tables] = admin_input_request('set::global_config', ['news_announcements' => $news]);
    [$data] = input_request(['endpoint' => 'topics', 'tables' => $tables]);
    expect_input($data['news_announcements'], array_slice($news, 0, 6));
});

foreach (['config', 'admin'] as $firstReader) {
    check_input("Legacy announcements survive $firstReader before topics", function () use ($firstReader) {
        $title = 'Trainer\'s "news" C:\\news\\today';
        $url = 'https://example.com/?q="news"';
        $tables = DB::$tables;
        foreach (['ann_title' => $title, 'ann_url' => $url] as $key => $value) {
            $tables['pm_config'][$key] = ['key' => $key, 'value' => $value, 'data_type' => 'string'];
        }
        $expected = [['title' => $title, 'url' => $url]];
        if ($firstReader === 'admin') {
            [$first, $tables] = admin_input_request('get::global_config', [], $tables);
            $first = $first[0];
        } else {
            [$first, $tables] = input_request(['endpoint' => 'config', 'tables' => $tables]);
        }
        expect_input($first['news_announcements'], $expected);
        expect_input(json_decode($tables['pm_config']['news_announcements']['value'], true), $expected);
        [$topics] = input_request(['endpoint' => 'topics', 'tables' => $tables]);
        expect_input($topics['news_announcements'], $expected);
    });
}

foreach (['config', 'admin', 'topics'] as $reader) {
    check_input("Explicitly cleared announcements stay empty through $reader", function () use ($reader) {
        $tables = DB::$tables;
        foreach (['ann_title' => 'Retired notice', 'ann_url' => 'https://example.com/old', 'news_announcements' => '[]'] as $key => $value) {
            $tables['pm_config'][$key] = ['key' => $key, 'value' => $value, 'data_type' => 'string'];
        }
        if ($reader === 'admin') {
            [$data] = admin_input_request('get::global_config', [], $tables);
            $data = $data[0];
        } else {
            [$data] = input_request(['endpoint' => $reader, 'tables' => $tables]);
        }
        expect_input($data['news_announcements'], []);
    });
}

check_input('Failed announcement inserts cannot return an empty successful list', function () {
    try {
        input_request(['endpoint' => 'config', 'fail_news_insert' => true]);
    } catch (RuntimeException $error) {
        expect_input($error->getMessage(), 'Configuration write failed');
        return;
    }
    throw new RuntimeException('A failed insert must reject the read migration');
});

echo "Admin input regressions: $passed passed, $failed failed\n";
exit($failed === 0 ? 0 : 1);
