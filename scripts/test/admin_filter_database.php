<?php
/** Exercise all six production admin filters with real SQL and populated rows. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) { throw new ErrorException($message, 0, $severity, $file, $line); });
define('IN_DISCUZ', true);
class DB
{
    public static $connection;
    public static function table($name) { return $name; }
    public static function query($sql) { return self::$connection->query($sql); }
    public static function fetch_first($sql) { return self::query($sql)->fetch_assoc(); }
    public static function fetch_all($sql) { return self::query($sql)->fetch_all(MYSQLI_ASSOC); }
    public static function result_first($sql) { $row = self::query($sql)->fetch_row(); return $row ? $row[0] : null; }
}
require __DIR__ . '/../../plugin/admin/routes.php';
mysqli_report(MYSQLI_REPORT_ERROR | MYSQLI_REPORT_STRICT);
$db = DB::$connection = new mysqli(getenv('TSDM_DB_HOST') ?: '127.0.0.1', getenv('TSDM_DB_USER') ?: 'root',
    getenv('TSDM_DB_PASSWORD') ?: '', '', (int)(getenv('TSDM_DB_PORT') ?: 3306));
$db->set_charset('utf8mb4');
$database = 'tsdm_test_filters_' . bin2hex(random_bytes(8));
$db->query("CREATE DATABASE `$database` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci");
$checks = $failures = 0;
function filter_case($function, $conditions, $expected, $label)
{
    global $checks, $failures;
    $checks++;
    try {
        $rows = $function($conditions);
        $actual = array_map(function ($row) { return (int)$row['id']; }, $rows);
        sort($actual); sort($expected);
        if ($actual !== $expected) throw new RuntimeException('Got ' . json_encode($actual) . ', expected ' . json_encode($expected));
        echo "PASS $label\n";
    } catch (Throwable $error) {
        $failures++;
        echo "FAIL $label: ", $error->getMessage(), "\n";
    }
}
function condition($tag, $value, $operator = 'equal') { return ['tag' => $tag, 'value' => $value, 'operator' => $operator]; }
try {
    $db->select_db($database);
    $db->multi_query(file_get_contents(__DIR__ . '/../../docker/init.d/02-pokemon-schema.sql'));
    do { if ($result = $db->store_result()) $result->free(); } while ($db->more_results() && $db->next_result());
    $db->query("SET SESSION sql_mode = ''");
    $db->query('CREATE TABLE common_member (uid INT PRIMARY KEY, username VARCHAR(60) NOT NULL)');
    $db->query("INSERT INTO common_member VALUES (7, 'Demo Alice'), (8, 'Demo Bob'), (9, 'O\'Brien')");
    $db->query('INSERT INTO pm_usersdata (uid, money) VALUES (7,100),(8,200),(9,50)');
    $db->query("INSERT INTO pm_data (id,name,description,xs,effort_values,drop_money,mapid) VALUES
        (1,'Alpha','Keep','火','{}','[1,1]','1'),(2,'Beta','Keep','水','{}','[1,1]','1'),(3,'90099','Other','水','{}','[1,1]','2')");
    $db->query("INSERT INTO pm_itemdata (id,name,description,type,money,shop,effects,equipment) VALUES
        (1,'Alpha','Keep',1,100,1,'{}','{}'),(2,'Beta','Keep',1,200,1,'{}','{}'),(3,'90099','Other',1,50,0,'{}','{}')");
    $db->query("INSERT INTO pm_map (id,name,min_level,max_level,site,boss_config) VALUES
        (1,'Alpha',1,10,'1','{}'),(2,'Beta',2,10,'1','{}'),(3,'90099',3,10,'2','{}')");
    $db->query("INSERT INTO pm_skill (id,name,description,available_pokemons,element,category) VALUES
        (1,'Alpha','Keep','k,1,k','火','物理'),(2,'Beta','Keep','k,2,k','水','物理'),(3,'90099','Other','k,3,k','水','物理')");
    $db->query("INSERT INTO pm_evolution (id,from_id,to_id,method,condition_value) VALUES
        (1,1,2,'level','16'),(2,1,3,'level','20'),(3,2,3,'level','30')");

    $cases = [
        ['filter_user_info', [condition('UID','7'), condition('金钱','100','greater_or_equal')], [7]],
        ['filter_item_type', [condition('ID','1'), condition('价格','100','greater_or_equal')], [1]],
        ['filter_map_info', [condition('ID','1'), condition('野怪最低等级','1','greater_or_equal')], [1]],
        ['filter_pokemon_type', [condition('ID','1'), condition('描述','Keep')], [1]],
        ['filter_skill_type', [condition('ID','1'), condition('描述','Keep')], [1]],
        ['filter_evolution_info', [condition('ID','1'), condition('进化目标','3')], []],
    ];
    foreach ($cases as [$function,$conditions,$expected]) {
        filter_case($function,$conditions,$expected,"$function combines every condition");
        filter_case($function,array_reverse($conditions),$expected,"$function is independent of condition order");
        filter_case($function,[],[],"$function handles an empty filter without a warning");
        filter_case($function,[condition('unknown','x'),$conditions[0]],[],"$function rejects an unsupported condition safely");
    }
    $shortcuts = [
        ['filter_user_info','昵称','7',condition('金钱','150','greater_or_equal')],
        ['filter_item_type','名称','1',condition('价格','150','greater_or_equal')],
        ['filter_map_info','名称','1',condition('野怪最低等级','2','greater_or_equal')],
        ['filter_pokemon_type','名称','1',condition('宠物类型','水')],
        ['filter_skill_type','名称','1',condition('描述','Other')],
        ['filter_evolution_info','进化来源','1',condition('进化目标','1')],
    ];
    foreach ($shortcuts as [$function,$tag,$value,$other]) {
        filter_case($function,[condition($tag,$value),$other],[],"$function ID shortcut still applies later conditions");
        filter_case($function,[$other,condition($tag,$value)],[],"$function ID shortcut still applies earlier conditions");
    }
    filter_case('filter_user_info',[condition('昵称','Demo')],[7,8],'Nickname matches form a union of matching users');
    filter_case('filter_user_info',[condition('昵称','Demo','not_equal')],[9],'Nickname exclusion excludes every matching user');
    filter_case('filter_user_info',[condition('昵称','missing'),condition('金钱','0','greater_or_equal')],[],'Missing nickname cannot drop the name restriction');
    filter_case('filter_user_info',[condition('昵称','missing','not_equal')],[7,8,9],'An unmatched nickname exclusion preserves all users');
    filter_case('filter_user_info',[condition('昵称',"O'Brien")],[9],'Quoted nicknames are escaped exactly once');
    filter_case('filter_user_info',[condition('昵称','7','not_equal')],[8,9],'Numeric nickname exclusion respects its operator');
    filter_case('filter_evolution_info',[condition('进化来源','Alpha')],[1,2],'Evolution source names resolve species IDs');
    filter_case('filter_evolution_info',[condition('进化目标','Beta')],[1],'Evolution target names resolve species IDs');
    filter_case('filter_evolution_info',[condition('进化来源','Alpha','not_equal')],[3],'Evolution source exclusion respects its operator');
    foreach (['filter_item_type','filter_map_info','filter_pokemon_type','filter_skill_type'] as $function) {
        filter_case($function,[condition('名称','1','not_equal')],[2,3],"$function numeric name exclusion respects its operator");
        filter_case($function,[condition('名称','90099')],[3],"$function falls back to numeric names when no ID exists");
        filter_case($function,[condition('名称','Alpha','greater'),condition('ID','1')],[],"$function rejects an invalid text operator safely");
        filter_case($function,[condition('名称','Al','contains')],[1],"$function supports the UI contains operator");
    }
    filter_case('filter_user_info',[condition('昵称','Demo','contains')],[7,8],'Nickname contains searches matching members');
    filter_case('filter_evolution_info',[condition('进化目标','Bet','contains')],[1],'Evolution target contains searches species names');
    filter_case('filter_pokemon_type',[condition('宠物类型','水','contains')],[2,3],'Pokemon type supports the UI contains operator');
    filter_case('filter_skill_type',[condition('可用此的种族','1','not_equal')],[2,3],'Skill species exclusion supports the UI operator');
    foreach (['否','0','false'] as $value) filter_case('filter_item_type',[condition('是否出售',$value)],[3],"Not-for-sale input $value selects only disabled shop items");
    foreach (['是','1','true'] as $value) filter_case('filter_item_type',[condition('是否出售',$value)],[1,2],"For-sale input $value selects enabled shop items");
    filter_case('filter_item_type',[condition('是否出售','invalid')],[],'Unknown boolean values do not silently mean true');
    echo "Admin filter database: $checks checks, $failures failures.\n";
} finally {
    // Only the name generated and exclusively created by this process can be dropped.
    if (!preg_match('/^tsdm_test_filters_[a-f0-9]{16}$/D', $database)) throw new RuntimeException('Unsafe database name');
    $db->query("DROP DATABASE `$database`");
    $db->close();
}
exit($failures ? 1 : 0);
