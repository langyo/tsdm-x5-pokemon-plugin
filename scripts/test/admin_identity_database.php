<?php
/** Exercise catalog identity allocation with real routes, schema and concurrent connections. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) { throw new ErrorException($message, 0, $severity, $file, $line); });
define('IN_DISCUZ', true);
class DB {
    public static $connection;
    public static $barrier;
    public static $queries = [];
    public static function table($name) { return $name; }
    public static function query($sql) {
        self::$queries[] = $sql;
        if (self::$barrier && preg_match('/^INSERT INTO\s+' . self::$barrier['table'] . '\b/i', trim($sql))) {
            $barrier = self::$barrier; self::$barrier = null;
            // Both old MAX+1 reads have completed when the two INSERTs reach this gate.
            file_put_contents($barrier['path'] . '.ready', 'ready');
            $deadline = microtime(true) + 10;
            while (!is_file($barrier['path'] . '.resume')) {
                if (microtime(true) > $deadline) throw new RuntimeException('Insert barrier timed out');
                usleep(10000); clearstatcache();
            }
        }
        return self::$connection->query($sql);
    }
    public static function fetch_first($sql) { return self::query($sql)->fetch_assoc(); }
    public static function fetch_all($sql) { return self::query($sql)->fetch_all(MYSQLI_ASSOC); }
    public static function result_first($sql) { $row = self::query($sql)->fetch_row(); return $row ? $row[0] : null; }
    public static function insert_id() { return self::$connection->insert_id; }
}
require __DIR__ . '/../../plugin/admin/routes.php';
mysqli_report(MYSQLI_REPORT_ERROR | MYSQLI_REPORT_STRICT);
$db = DB::$connection = new mysqli(getenv('TSDM_DB_HOST') ?: '127.0.0.1', getenv('TSDM_DB_USER') ?: 'root',
    getenv('TSDM_DB_PASSWORD') ?: '', '', (int)(getenv('TSDM_DB_PORT') ?: 3306));
$db->set_charset('utf8mb4');
$catalogs = ['effect_data'=>'pm_effect', 'evolution_info'=>'pm_evolution', 'map_info'=>'pm_map',
    'item_type'=>'pm_itemdata', 'pokemon_type'=>'pm_data', 'skill_type'=>'pm_skill'];

function catalog_input($entity, $name) {
    $attributes = array_fill_keys(['hit_points','attack','defense','special_attack','special_defense','speed'], 40);
    switch ($entity) {
        case 'effect_data': return ['code'=>$name, 'kind'=>'move', 'hooks'=>['on_hit'],
            'params'=>['code'=>'stages_boost','stat'=>'atk','stages'=>1,'target'=>'self'], 'description'=>$name, 'version'=>1];
        case 'evolution_info': return ['source_id'=>10, 'target_id'=>10, 'condition'=>['min_level'=>5], 'priority'=>$name === 'Concurrent1' ? 2 : 1];
        case 'map_info': return ['name'=>$name, 'area_type'=>'g', 'is_enabled'=>true, 'min_level'=>1, 'max_level'=>10,
            'mode'=>'wild', 'experience'=>1, 'experience_increase_times'=>1];
        case 'item_type': return ['name'=>$name, 'img_name'=>'hp20', 'description'=>$name, 'is_selling'=>false,
            'price'=>10, 'tag'=>['drug'=>null], 'limits'=>['min_level'=>1,'kind_require'=>null],
            'effects'=>array_fill_keys(['add_hit_points','add_experience','add_level','add_intimacy',
                'attribute_add_hit_points','attribute_add_attack','attribute_add_defense','attribute_add_special_attack',
                'attribute_add_special_defense','attribute_add_speed','capture'], 0)];
        case 'pokemon_type': return ['name'=>$name, 'description'=>$name, 'cost'=>10, 'is_selling'=>false,
            'sex_weight'=>0.5, 'initial_statistic'=>$attributes, 'initial_base_points'=>array_fill_keys(array_keys($attributes), 0),
            'kind'=>['normal',null], 'is_legendary'=>false, 'map_ids'=>[], 'capture_weight'=>100, 'meet_weight'=>100,
            'birth_order'=>1, 'strength_weight'=>1, 'drop_money_range'=>[1,2]];
        case 'skill_type': return ['name'=>$name, 'description'=>$name, 'available_pokemons'=>[10],
            'min_level_limit'=>1, 'use_times_limit'=>35, 'effect'=>['physical_damage'=>['normal',40]], 'effect_id'=>0];
    }
    throw new RuntimeException('Unknown fixture entity');
}

if (($argv[1] ?? '') === '--worker') {
    [$script, $flag, $database, $entity, $name, $mode, $barrier] = $argv;
    if (!preg_match('/\Atsdm_test_identity_[a-f0-9]{16}\z/D', $database) || !isset($catalogs[$entity])) {
        throw new RuntimeException('Invalid isolated worker target');
    }
    $db->select_db($database);
    $db->query("SET SESSION sql_mode = '" . ($mode === 'strict' ? 'STRICT_ALL_TABLES' : '') . "'");
    DB::$barrier = ['table'=>$catalogs[$entity], 'path'=>$barrier];
    try {
        $id = call_user_func('insert_' . $entity, catalog_input($entity, $name));
        echo json_encode(['success'=>true, 'id'=>$id, 'row'=>call_user_func('get_' . $entity, $id)[0]]);
    } catch (Throwable $error) { echo json_encode(['success'=>false, 'error'=>$error->getMessage()]); }
    $db->close(); exit;
}

$checks = $failures = 0;
function identity_check($condition, $label) {
    $GLOBALS['checks']++; if (!$condition) $GLOBALS['failures']++;
    echo ($condition ? 'PASS ' : 'FAIL '), $label, PHP_EOL;
}
function identity_case($label, $callback) {
    try { $callback(); } catch (Throwable $error) { identity_check(false, $label . ': ' . $error->getMessage()); }
}
function race_inserts($database, $entity, $mode) {
    $prefix = sys_get_temp_dir() . '/tsdm_identity_' . bin2hex(random_bytes(8));
    $workers = [];
    try {
        for ($index=0; $index<2; $index++) {
            $base = $prefix . '_' . $index;
            $command = [PHP_BINARY];
            if ($ini = php_ini_loaded_file()) array_push($command, '-c', $ini);
            array_push($command, '-d', 'opcache.jit=0', '-d', 'opcache.jit_buffer_size=0', __FILE__, '--worker',
                $database, $entity, 'Concurrent' . $index, $mode, $base);
            $process = proc_open($command, [0=>['pipe','r'],1=>['pipe','w'],2=>['pipe','w']], $pipes, null, null, ['bypass_shell'=>true]);
            if (!is_resource($process)) throw new RuntimeException('Could not start insert worker');
            fclose($pipes[0]);
            $workers[] = ['process'=>$process,'out'=>$pipes[1],'err'=>$pipes[2],'base'=>$base];
        }
        $deadline = microtime(true) + 10;
        foreach ($workers as $worker) {
            while (!is_file($worker['base'] . '.ready')) {
                if (microtime(true)>$deadline || !proc_get_status($worker['process'])['running']) {
                    throw new RuntimeException('Insert worker did not reach barrier');
                }
                usleep(10000); clearstatcache();
            }
        }
        foreach ($workers as $worker) file_put_contents($worker['base'] . '.resume', 'resume');
        $results = [];
        foreach ($workers as $worker) {
            $output = stream_get_contents($worker['out']); $error = stream_get_contents($worker['err']);
            if ($error !== '') throw new RuntimeException($error);
            $results[] = json_decode($output, true, 512, JSON_THROW_ON_ERROR);
        }
        return $results;
    } finally {
        foreach ($workers as $worker) {
            if (proc_get_status($worker['process'])['running']) proc_terminate($worker['process']);
            fclose($worker['out']); fclose($worker['err']); proc_close($worker['process']);
            foreach (['.ready','.resume'] as $suffix) if (is_file($worker['base'] . $suffix)) unlink($worker['base'] . $suffix);
        }
    }
}

$database = 'tsdm_test_identity_' . bin2hex(random_bytes(8));
$db->query("CREATE DATABASE `$database` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci");
try {
    $db->select_db($database);
    $db->multi_query(file_get_contents(__DIR__ . '/../../docker/init.d/02-pokemon-schema.sql'));
    do { if ($result = $db->store_result()) $result->free(); } while ($db->more_results() && $db->next_result());
    $seeds = [
        'effect_data'=>"INSERT INTO pm_effect (id,code,hooks_json,params_json) VALUES (10,'seed','[]','{}')",
        'evolution_info'=>"INSERT INTO pm_evolution (id,from_id,to_id) VALUES (10,10,10)",
        'map_info'=>"INSERT INTO pm_map (id,name,site,boss_config) VALUES (10,'Seed','g','')",
        'item_type'=>"INSERT INTO pm_itemdata (id,name,effects,equipment) VALUES (10,'Seed','{}','{}')",
        'pokemon_type'=>"INSERT INTO pm_data (id,name,mapid,effort_values,drop_money) VALUES (10,'Seed','','{}','[0,0]')",
        'skill_type'=>"INSERT INTO pm_skill (id,name,description,available_pokemons,element,category) VALUES (10,'Seed','Seed','k,10,k','普通','物攻')",
    ];
    foreach (['non-strict','strict'] as $mode) {
        $db->query("SET SESSION sql_mode = '" . ($mode === 'strict' ? 'STRICT_ALL_TABLES' : '') . "'");
        $db->query(str_replace('INSERT INTO', 'INSERT IGNORE INTO', $seeds['pokemon_type']));
        foreach ($catalogs as $entity=>$table) {
            $db->query("TRUNCATE TABLE $table"); $db->query($seeds[$entity]);
            identity_check(strpos(DB::fetch_first("SHOW COLUMNS FROM $table LIKE 'id'")['Extra'], 'auto_increment') !== false,
                "$mode $entity uses the installed AUTO_INCREMENT schema");
            identity_case("$mode $entity deleted identity", function () use ($entity, $table, $mode, $db) {
                $input = catalog_input($entity, 'Original'); $input['id'] = 999;
                $id = call_user_func('insert_' . $entity, $input);
                $row = call_user_func('get_' . $entity, $id)[0];
                identity_check(is_int($id) && $id !== 999 && $row['id'] === $id && $row['_TYPE'] === $entity,
                    "$mode $entity preserves the generated-ID/get-row response contract");
                if ($entity === 'map_info') $db->query("UPDATE pm_data SET mapid='$id' WHERE id=10");
                if ($entity === 'item_type') $db->query("INSERT INTO pm_myitem (uid,itemid,nums) VALUES (7,'$id',1)");
                if ($entity === 'pokemon_type') $db->query("INSERT INTO pm_mypm (uid,species_id) VALUES (7,$id)");
                call_user_func('delete_' . $entity, $id);
                $replacement = call_user_func('insert_' . $entity, catalog_input($entity, 'Unrelated'));
                identity_check($replacement > $id, "$mode $entity does not reuse a deleted highest ID");
                $join = [
                    'map_info'=>'SELECT COUNT(*) FROM pm_data p JOIN pm_map m ON FIND_IN_SET(m.id,p.mapid)>0 WHERE p.id=10',
                    'item_type'=>'SELECT COUNT(*) FROM pm_myitem i JOIN pm_itemdata d ON d.id=i.itemid WHERE i.uid=7',
                    'pokemon_type'=>'SELECT COUNT(*) FROM pm_mypm p JOIN pm_data d ON d.id=p.species_id WHERE p.uid=7',
                ];
                if (isset($join[$entity])) identity_check((int)DB::result_first($join[$entity]) === 0,
                    "$mode $entity leaves stale references detached from the new catalog entry");
            });
            identity_case("$mode $entity concurrent insert", function () use ($database, $entity, $mode, $table) {
                $before = (int)DB::result_first("SELECT COUNT(*) FROM $table");
                $results = race_inserts($database, $entity, $mode);
                $ok = count(array_filter($results, function ($result) { return $result['success']; })) === 2;
                identity_check($ok, "$mode $entity concurrent inserts both succeed" . ($ok ? '' : ': ' . json_encode($results)));
                $own_rows = $ok;
                if ($ok) foreach ($results as $index=>$result) {
                    $own_rows = $own_rows && ($entity === 'evolution_info'
                        ? $result['row']['priority'] === $index + 1
                        : $result['row'][$entity === 'effect_data' ? 'code' : 'name'] === 'Concurrent' . $index);
                }
                identity_check($own_rows && $results[0]['id'] !== $results[1]['id']
                    && $results[0]['row']['id'] === $results[0]['id'] && $results[1]['row']['id'] === $results[1]['id']
                    && (int)DB::result_first("SELECT COUNT(*) FROM $table") === $before + 2,
                    "$mode $entity allocates distinct IDs and returns each connection's own row");
            });
            identity_case("$mode $entity empty table", function () use ($db, $entity, $table, $mode) {
                $last = (int)DB::result_first("SELECT MAX(id) FROM $table");
                $db->query("DELETE FROM $table");
                $id = call_user_func('insert_' . $entity, catalog_input($entity, 'AfterEmpty'));
                identity_check($id > $last, "$mode $entity works after all rows are deleted without recycling IDs");
            });
            $db->query('DELETE FROM pm_myitem'); $db->query('DELETE FROM pm_mypm');
        }
    }
    echo "Admin identity database checks: $checks, failures: $failures", PHP_EOL;
} finally {
    // Only the name generated and exclusively created by this process can be dropped.
    if (!preg_match('/^tsdm_test_identity_[a-f0-9]{16}$/D', $database)) throw new RuntimeException('Unsafe database name');
    $db->query("DROP DATABASE `$database`"); $db->close();
}
exit($failures ? 1 : 0);
