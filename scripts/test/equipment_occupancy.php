<?php

/**
 * Execute the production equip endpoint against an in-memory inventory.
 * No Discuz installation or database is needed. This does not test concurrency.
 * Run: php scripts/test/equipment_occupancy.php
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

class EquipmentResponse extends RuntimeException
{
    public $success;
    public $data;
    public $status;

    public function __construct($success, $data, $status = 200)
    {
        parent::__construct($success ? 'success' : $data);
        $this->success = $success;
        $this->data = $data;
        $this->status = $status;
    }
}

class DB
{
    public static $pets = [];
    public static $items = [];
    public static $writes = [];
    public static $affected = 0;
    // 模拟并发竞争：在原子占位语句求值前变更数据，模拟另一请求抢先写入
    public static $on_claim;

    public static function fetch_first($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        if (preg_match('/^SELECT \* FROM pm_mypm WHERE id = (\d+) AND uid = (\d+)$/', $sql, $match)) {
            foreach (self::$pets as $pet) {
                if ($pet['id'] === (int) $match[1] && $pet['uid'] === (int) $match[2]) {
                    return $pet;
                }
            }
            return false;
        }
        if (preg_match('/^SELECT m\.\*, i\.type, i\.name, i\.equipment FROM pm_myitem m LEFT JOIN pm_itemdata i ON m\.itemid=i\.id WHERE m\.id=(\d+) AND m\.uid=(\d+)$/', $sql, $match)) {
            foreach (self::$items as $item) {
                if ($item['id'] === (int) $match[1] && $item['uid'] === (int) $match[2]) {
                    return $item;
                }
            }
            return false;
        }
        if (preg_match('/^SELECT id, nickname FROM pm_mypm WHERE \(equipmentid1=(\d+) OR equipmentid2=(\d+) OR equipmentid3=(\d+) OR equipmentid4=(\d+)\)(?: AND id!=(\d+))? AND uid=(\d+)$/', $sql, $match)) {
            // Evaluate every predicate against the fixture, including the old
            // exclusion of the current pet. Never preselect a desired answer.
            foreach (self::$pets as $pet) {
                if ($pet['uid'] !== (int) $match[6] || ($match[5] !== '' && $pet['id'] === (int) $match[5])) {
                    continue;
                }
                for ($slot = 1; $slot <= 4; $slot++) {
                    if ($pet['equipmentid' . $slot] === (int) $match[$slot]) {
                        return ['id' => $pet['id'], 'nickname' => $pet['nickname']];
                    }
                }
            }
            return false;
        }
        if ($sql === 'SELECT * FROM pm_data WHERE id = 1') {
            return ['id' => 1];
        }
        throw new RuntimeException('Unexpected read: ' . $sql);
    }

    public static function query($sql)
    {
        $sql = preg_replace('/\s+/', ' ', trim($sql));
        self::$affected = 0;
        if (preg_match('/^UPDATE pm_mypm SET (equipmentid[1-4])=(\d+) WHERE id=(\d+) AND uid=(\d+) AND (equipmentid[1-4])=0 AND NOT EXISTS \(SELECT 1 FROM \(SELECT id FROM pm_mypm WHERE uid=(\d+) AND \(equipmentid1=(\d+) OR equipmentid2=(\d+) OR equipmentid3=(\d+) OR equipmentid4=(\d+)\)\) AS occupied\)$/', $sql, $match)) {
            if ($match[1] !== $match[5]) {
                throw new RuntimeException('Claim targets a different slot in SET and WHERE: ' . $sql);
            }
            if (self::$on_claim) {
                $interpose = self::$on_claim;
                self::$on_claim = null;
                $interpose();
            }
            $item_id = (int) $match[2];
            $occupied = false;
            foreach (self::$pets as $other) {
                if ($other['uid'] !== (int) $match[6]) {
                    continue;
                }
                for ($slot = 1; $slot <= 4; $slot++) {
                    if ((int) $other['equipmentid' . $slot] === $item_id) {
                        $occupied = true;
                    }
                }
            }
            foreach (self::$pets as &$pet) {
                if (!$occupied && $pet['id'] === (int) $match[3] && $pet['uid'] === (int) $match[4] && (int) $pet[$match[1]] === 0) {
                    $pet[$match[1]] = $item_id;
                    self::$affected = 1;
                    self::$writes[] = $sql;
                }
            }
            unset($pet);
        } elseif (preg_match('/^UPDATE pm_mypm SET hp = (\d+) WHERE id = (\d+)$/', $sql, $match)) {
            foreach (self::$pets as &$pet) {
                if ($pet['id'] === (int) $match[2]) {
                    $pet['hp'] = (int) $match[1];
                }
            }
            self::$affected = 1;
            self::$writes[] = $sql;
        } else {
            throw new RuntimeException('Unexpected write: ' . $sql);
        }
    }

    public static function affected_rows()
    {
        return self::$affected;
    }
}

function require_login() {}
function get_json_input() { return $GLOBALS['input']; }
function validate_id($value, $name) { return (int) $value; }
function validate_uid($value) { return (int) $value; }
function pm_table($name) { return $name; }
function pm_sql($sql, ...$args) { return vsprintf($sql, $args); }
function api_error($message, $status = 400) { throw new EquipmentResponse(false, $message, $status); }
function api_success($data) { throw new EquipmentResponse(true, $data); }

// Stats are deterministic fixtures: base HP 100, each equipped item adds 25.
// Stat formulas are outside the scope of these endpoint tests.
function api_calculate_pokemon_max_hp($pet)
{
    $hp = 100;
    for ($slot = 1; $slot <= 4; $slot++) {
        if ($pet['equipmentid' . $slot] !== 0) {
            $hp += 25;
        }
    }
    return $hp;
}
function calculate_pokemon_full_stats($pet, $base)
{
    $stats = [];
    foreach (['hp', 'atk', 'def', 'spatk', 'spdef', 'speed'] as $stat) {
        $stats['base_' . $stat] = 100;
        $stats['equipment_' . $stat] = $stat === 'hp' ? api_calculate_pokemon_max_hp($pet) - 100 : 0;
        $stats['total_' . $stat] = $stats['base_' . $stat] + $stats['equipment_' . $stat];
    }
    return $stats;
}

// Load only the real endpoint function without running the request dispatcher.
function load_equip_endpoint($path)
{
    $tokens = token_get_all(file_get_contents($path));
    $count = count($tokens);
    for ($i = 0; $i < $count; $i++) {
        if (!is_array($tokens[$i]) || $tokens[$i][0] !== T_FUNCTION) {
            continue;
        }
        $name_index = $i + 1;
        while (is_array($tokens[$name_index]) && $tokens[$name_index][0] === T_WHITESPACE) {
            $name_index++;
        }
        if (!is_array($tokens[$name_index]) || $tokens[$name_index][1] !== 'api_equip_item') {
            continue;
        }
        $source = '';
        $depth = 0;
        $started = false;
        for (; $i < $count; $i++) {
            $token = $tokens[$i];
            if (is_array($token)) {
                $source .= $token[1];
                if ($token[0] === T_CURLY_OPEN || $token[0] === T_DOLLAR_OPEN_CURLY_BRACES) {
                    $depth++;
                }
            } else {
                $source .= $token;
                if ($token === '{') {
                    $depth++;
                    $started = true;
                } elseif ($token === '}' && --$depth === 0 && $started) {
                    eval($source);
                    return;
                }
            }
        }
    }
    throw new RuntimeException('api_equip_item was not found');
}

load_equip_endpoint(dirname(__DIR__, 2) . '/plugin/api/pokemon.php');
$_G = ['uid' => 1];
$passed = 0;
$failed = 0;

function verify($condition, $message)
{
    if (!$condition) {
        throw new RuntimeException($message);
    }
}

function run_case($name, $input, $pets, $items, $expected_error = null, $expected_slot = null, $expected_status = null)
{
    global $passed, $failed;
    $GLOBALS['input'] = $input;
    DB::$pets = $pets;
    DB::$items = $items;
    DB::$writes = [];
    DB::$on_claim = null;
    try {
        try {
            api_equip_item();
            throw new RuntimeException('Endpoint did not return a response');
        } catch (EquipmentResponse $response) {
            verify($response->success === ($expected_error === null), 'Unexpected response: ' . $response->getMessage());
            if ($expected_error !== null) {
                verify($response->data === $expected_error, 'Unexpected error: ' . $response->data);
                if ($expected_status === null) {
                    $expected_status = strpos($expected_error, 'not found') !== false ? 404 : 400;
                }
                verify($response->status === $expected_status, 'Unexpected HTTP status');
                verify(DB::$writes === [], 'Rejected action wrote to the database');
                verify(DB::$pets === $pets && DB::$items === $items, 'Rejected action changed inventory or pets');
            } else {
                verify($response->data['slot_index'] === $expected_slot, 'Wrong equipment slot');
                verify(count(DB::$writes) === 2, 'Expected equipment and HP updates');
                $expected = $pets;
                foreach ($expected as &$pet) {
                    if ($pet['id'] === $input['pokemon_id']) {
                        $old_maxhp = api_calculate_pokemon_max_hp($pet);
                        $pet['equipmentid' . ($expected_slot + 1)] = $input['myitem_id'];
                        $pet['hp'] = (int) round(api_calculate_pokemon_max_hp($pet) * $pet['hp'] / $old_maxhp);
                    }
                }
                verify(DB::$pets === $expected, 'Unexpected pet state or HP after equipping');
                verify(DB::$items === $items, 'Equipping changed inventory quantities');
            }
        }
        $passed++;
        echo "PASS $name\n";
    } catch (Throwable $error) {
        $failed++;
        echo "FAIL $name: {$error->getMessage()}\n";
    }
}

$pet = ['id' => 7, 'uid' => 1, 'nickname' => 'Current pet', 'species_id' => 1, 'hp' => 50,
    'equipmentid1' => 0, 'equipmentid2' => 0, 'equipmentid3' => 0, 'equipmentid4' => 0];
$item = ['id' => 9, 'uid' => 1, 'itemid' => 3, 'type' => 5, 'nums' => 1, 'name' => 'Test equipment', 'equipment' => '{}'];
$request = ['pokemon_id' => 7, 'myitem_id' => 9];

for ($slot = 1; $slot <= 4; $slot++) {
    $equipped = $pet;
    $equipped['equipmentid' . $slot] = 9;
    $empty_slot = $slot === 1 ? 1 : 0;
    run_case("Reject current pet slot $slot with explicit empty target", $request + ['slot_index' => $empty_slot],
        [$equipped], [$item], '该装备已被 Current pet 使用');
    run_case("Reject current pet slot $slot with automatic target", $request,
        [$equipped], [$item], '该装备已被 Current pet 使用');
    $other = $equipped;
    $other['id'] = 8;
    $other['nickname'] = 'Other pet';
    run_case("Reject another pet slot $slot", $request, [$pet, $other], [$item], '该装备已被 Other pet 使用');
}

run_case('Equip available item in explicit fourth slot', $request + ['slot_index' => 3], [$pet], [$item], null, 3);
run_case('Equip available item in automatic first slot', $request, [$pet], [$item], null, 0);
run_case('Equip item again after the previous slot was cleared', $request + ['slot_index' => 2], [$pet], [$item], null, 2);
run_case('Explicit -1 also selects an automatic slot', $request + ['slot_index' => -1], [$pet], [$item], null, 0);
$other_item = $item;
$other_item['id'] = 10;
$occupied = $pet;
$occupied['equipmentid1'] = 9;
$stacked_item = $item;
$stacked_item['nums'] = 2;
run_case('Reject repeated inventory row even with multiple units', $request, [$occupied], [$stacked_item],
    '该装备已被 Current pet 使用');
run_case('Equip another inventory instance of the same item type', ['pokemon_id' => 7, 'myitem_id' => 10],
    [$occupied], [$item, $other_item], null, 1);
run_case('Reject an occupied target slot', ['pokemon_id' => 7, 'myitem_id' => 10, 'slot_index' => 0],
    [$occupied], [$item, $other_item], 'Target slot already has equipment. Please unequip first.');
$full = $pet;
for ($slot = 1; $slot <= 4; $slot++) {
    $full['equipmentid' . $slot] = 20 + $slot;
}
run_case('Reject automatic equip with all slots full', $request, [$full], [$item], 'All equipment slots are full');
foreach ([-2, 4] as $invalid_slot) {
    run_case("Reject invalid slot $invalid_slot", $request + ['slot_index' => $invalid_slot], [$pet], [$item],
        'Invalid slot_index, must be 0-3 or -1 for auto');
}
run_case('Reject missing inventory item', $request, [$pet], [], 'Item not found or not owned by user');
$foreign_item = $item;
$foreign_item['uid'] = 2;
run_case('Reject another user inventory item', $request, [$pet], [$foreign_item], 'Item not found or not owned by user');
$empty_item = $item;
$empty_item['nums'] = 0;
run_case('Reject zero quantity', $request, [$pet], [$empty_item], 'No items available');
$ordinary_item = $item;
$ordinary_item['type'] = 1;
run_case('Reject a non-equipment item', $request, [$pet], [$ordinary_item], 'This item is not an equipment');
$foreign_pet = $pet;
$foreign_pet['uid'] = 2;
run_case('Reject another user pet', $request, [$foreign_pet], [$item], 'Pokemon not found');

// 并发竞争：占用预检已经通过，另一请求在原子占位求值前抢先写入。
// 预检面对的是竞态前的数据，只有条件 UPDATE 能挡住这类请求。
function claim_race_case($name, $input, $pets, $items, $interpose, $expected_error, $expected_status, $assert_state)
{
    global $passed, $failed;
    $GLOBALS['input'] = $input;
    DB::$pets = $pets;
    DB::$items = $items;
    DB::$writes = [];
    DB::$on_claim = $interpose;
    try {
        try {
            api_equip_item();
            throw new RuntimeException('Endpoint did not return a response');
        } catch (EquipmentResponse $response) {
            verify($response->success === false, 'Unexpected success in race case');
            verify($response->data === $expected_error, 'Unexpected error: ' . $response->data);
            verify($response->status === $expected_status, 'Unexpected HTTP status');
            verify(DB::$writes === [], 'Lost race still wrote the claim');
            $assert_state();
        }
        $passed++;
        echo "PASS $name\n";
    } catch (Throwable $error) {
        $failed++;
        echo "FAIL $name: {$error->getMessage()}\n";
    }
}

claim_race_case('Concurrent equip onto another pet loses the race', $request, [$pet], [$item],
    function () use ($pet) {
        $rival = $pet;
        $rival['id'] = 8;
        $rival['nickname'] = 'Rival pet';
        $rival['equipmentid2'] = 9;
        DB::$pets[] = $rival;
    },
    'Equipment conflict, please retry', 409,
    function () {
        verify(DB::$pets[0]['equipmentid1'] === 0, 'Losing request still occupied the slot');
        verify(count(DB::$pets) === 2 && DB::$pets[1]['equipmentid2'] === 9, 'Rival request state was disturbed');
    });

claim_race_case('Concurrent equip of the same item onto the same pet loses the race', $request, [$pet], [$item],
    function () {
        DB::$pets[0]['equipmentid4'] = 9;
    },
    'Equipment conflict, please retry', 409,
    function () {
        verify(DB::$pets[0]['equipmentid1'] === 0 && DB::$pets[0]['equipmentid4'] === 9, 'Losing request still occupied a slot');
    });

claim_race_case('Concurrent takeover of the explicit target slot loses the race',
    $request + ['slot_index' => 1], [$pet], [$item],
    function () {
        DB::$pets[0]['equipmentid2'] = 50;
    },
    'Equipment conflict, please retry', 409,
    function () {
        verify(DB::$pets[0]['equipmentid2'] === 50, 'Losing request overwrote the rival equipment');
    });

echo "Equipment occupancy tests: $passed passed, $failed failed.\n";
exit($failed === 0 ? 0 : 1);
