<?php
/**
 * Battle engine 2.0 pure-core regressions (issue #75 phase 1 / E0 contract).
 *
 * battle_core.php is dependency-free: load it directly and assert on the
 * pure functions — no DB stubs needed. Run: php scripts/test/battle_core_engine.php
 */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

define('IN_DISCUZ', 1);
require __DIR__ . '/../../plugin/api/battle_core.php';

$failures = 0;
$checks = 0;

function check($label, $cond)
{
    global $failures, $checks;
    $checks++;
    if ($cond) {
        echo "  ok: {$label}\n";
    } else {
        $failures++;
        echo "  FAIL: {$label}\n";
    }
}

function make_test_state($seed = 42, $ally_speed = 50, $enemy_speed = 30, $ally_hp = 100, $enemy_hp = 80, $ally_atk = 40, $enemy_atk = 25)
{
    return battle_core_initial_state([
        'kind' => 'wild',
        'map_id' => 7,
        'rng_seed' => $seed,
        'allies' => [[
            'instance_id' => 11,
            'species_id' => 25,
            'name' => '皮卡',
            'species_name' => '皮卡丘',
            'level' => 20,
            'hp' => $ally_hp,
            'stats' => ['max_hp' => $ally_hp, 'atk' => $ally_atk, 'def' => 30, 'spatk' => 35, 'spdef' => 28, 'speed' => $ally_speed],
            'types' => ['电'],
        ]],
        'enemies' => [[
            'species_id' => 129,
            'name' => '鲤鱼王',
            'species_name' => '鲤鱼王',
            'level' => 15,
            'hp' => $enemy_hp,
            'stats' => ['max_hp' => $enemy_hp, 'atk' => $enemy_atk, 'def' => 20, 'spatk' => 15, 'spdef' => 18, 'speed' => $enemy_speed],
            'types' => ['水'],
            'gender' => 1,
            'is_shiny' => true,
            'capture_rate' => 200,
        ]],
    ]);
}

echo "=== E0 state shape ===\n";
$state = make_test_state();
$unit = $state['sides']['ally'][0];
check('state version is 2', $state['version'] === BATTLE_STATE_VERSION);
check('rules version is 1', $state['rules_version'] === BATTLE_RULES_VERSION);
check('event schema version is 1', BATTLE_EVENT_SCHEMA_VERSION === 1);
check('multi-unit sides exist', isset($state['sides']['ally'][0], $state['sides']['enemy'][0]));
check('stage table present (-6..+6 keys init 0)', isset($unit['stages']['atk'], $unit['stages']['accuracy'], $unit['stages']['evasion']) && $unit['stages']['atk'] === 0);
check('status slot present (nullable)', array_key_exists('status', $unit) && $unit['status'] === null);
check('volatile slot present', array_key_exists('volatile', $unit) && $unit['volatile'] === []);
check('buffs slot present', array_key_exists('buffs', $unit) && $unit['buffs'] === []);
check('effects slot present', array_key_exists('effects', $unit) && $unit['effects'] === []);
check('field weather/terrain refs present', array_key_exists('weather', $state['field']) && array_key_exists('terrain', $state['field']) && $state['field']['weather'] === null);
check('rng seed persisted in state', $state['rng_seed'] === 42 && $state['rng_counter'] === 0);
check('enemy shiny/gender/capture snapshot', $state['sides']['enemy'][0]['is_shiny'] === true && $state['sides']['enemy'][0]['gender'] === 1 && $state['sides']['enemy'][0]['capture_rate'] === 200);

echo "=== type chart (ported from legacy) ===\n";
check('electric -> water 2x', battle_core_type_effectiveness('电', ['水']) === 2.0);
check('fire -> fire 0.5x', battle_core_type_effectiveness('火', ['火']) === 0.5);
check('ground -> flying immune 0x', battle_core_type_effectiveness('地面', ['飞行']) === 0.0);
check('dual type multiplies (water/flying vs electric)', battle_core_type_effectiveness('电', ['水', '飞行']) === 4.0);
check('unknown attack type 1x', battle_core_type_effectiveness('??', ['水']) === 1.0);
check('empty defender type skipped', battle_core_type_effectiveness('电', ['', null]) === 1.0);

echo "=== damage formula parity with legacy ===\n";
// legacy formula replica for fixed rolls (rand factor 1.0, no crit)
function legacy_damage($level, $atk, $def, $spatk, $spdef, $power, $skill_type, $category, $attacker_types, $defender_types)
{
    if ($category != 1) {
        $damage = (($level * 0.4 + 2) * $power * $atk / $def / 50 + 2);
    } else {
        $damage = (($level * 0.4 + 2) * $power * $spatk / $spdef / 50 + 2);
    }
    $boost = battle_core_type_effectiveness($skill_type, $defender_types);
    $damage *= $boost;
    if (in_array($skill_type, $attacker_types)) {
        $damage *= 1.5;
    }
    $damage *= 1.0; // rand(85,100) forced to 100
    return intval(max(1, floor($damage)));
}

$rng_no_crit_full = function ($min, $max) {
    // every roll deterministic: damage factor 100, crit roll != 1
    if ($max === 100 && $min === 85) {
        return 100;
    }
    if ($min === 1 && $max === 20) {
        return 2;
    }
    return $min;
};

$st = make_test_state();
$move = ['id' => 5, 'name' => '电击', 'power' => 40, 'type' => '电', 'category' => 0];
$dmg = battle_core_calc_damage($st, $st['sides']['ally'][0], $st['sides']['enemy'][0], $move, $rng_no_crit_full);
$expected = legacy_damage(20, 40, 20, 35, 18, 40, '电', 0, ['电'], ['水']);
check("physical damage parity (got {$dmg['amount']} want {$expected})", $dmg['amount'] === $expected);
check('effectiveness reported 2x', $dmg['effectiveness'] === 2.0);
check('crit false with forced roll', $dmg['crit'] === false);
check('stat reported atk', $dmg['stat'] === 'atk');

$move_sp = ['id' => 5, 'name' => '电击', 'power' => 40, 'type' => '电', 'category' => 1];
$dmg_sp = battle_core_calc_damage($st, $st['sides']['ally'][0], $st['sides']['enemy'][0], $move_sp, $rng_no_crit_full);
$expected_sp = legacy_damage(20, 40, 20, 35, 18, 40, '电', 1, ['电'], ['水']);
check("special damage parity (got {$dmg_sp['amount']} want {$expected_sp})", $dmg_sp['amount'] === $expected_sp);
check('special stat reported spatk', $dmg_sp['stat'] === 'spatk');

$crit_rng = function ($min, $max) {
    if ($min === 1 && $max === 20) {
        return 1; // force crit
    }
    if ($min === 85) {
        return 100;
    }
    return $min;
};
$dmg_crit = battle_core_calc_damage($st, $st['sides']['ally'][0], $st['sides']['enemy'][0], $move, $crit_rng);
check('crit doubles damage', $dmg_crit['amount'] === $expected * 2);
check('crit flagged', $dmg_crit['crit'] === true);

echo "=== counter damage parity (fixed power 40, physical, no type, no crit) ===\n";
$counter = battle_core_calc_counter_damage($st, $st['sides']['enemy'][0], $st['sides']['ally'][0], $rng_no_crit_full);
$expected_counter = intval(max(1, floor(((15 * 0.4 + 2) * 40 * 25 / 30 / 50 + 2) * 1.0)));
check("counter parity (got {$counter} want {$expected_counter})", $counter === $expected_counter);

echo "=== turn order & dodge ===\n";
// equal speed => ally first (legacy >=)
$st_equal = make_test_state(1, 30, 30);
$r = battle_core_apply_action($st_equal, ['type' => 'struggle']);
$types = [];
foreach ($r['events'] as $e) {
    $types[] = $e['type'];
}
check('equal speed ally moves first', $types === ['turn_start', 'move', 'damage', 'counter']);
check('turn incremented', $r['state']['turn'] === 1);

// slow ally vs fast enemy => enemy counters first
$st_slow = make_test_state(1, 20, 40);
$r2 = battle_core_apply_action($st_slow, ['type' => 'struggle']);
$types2 = [];
foreach ($r2['events'] as $e) {
    $types2[] = $e['type'];
}
check('slow ally acts after counter', $types2[0] === 'turn_start' && $types2[1] === 'counter' && $types2[2] === 'move');

// dodge: speed gap >= 10 and forced roll <= 4
$dodge_rng = function ($min, $max) {
    if ($min === 1 && $max === 20) {
        return 3; // rules v1 miss roll
    }
    if ($min === 1 && $max === 100) {
        return 3; // rules v2 miss-chance roll -> always misses
    }
    return 100;
};
$st_dodge = make_test_state(9, 20, 40);
$ally_hp_before = $st_dodge['sides']['ally'][0]['hp'];
$r3 = battle_core_apply_action($st_dodge, ['type' => 'struggle'], $dodge_rng);
$has_miss = false;
foreach ($r3['events'] as $e) {
    if ($e['type'] === 'miss') {
        $has_miss = true;
    }
}
check('miss event emitted when dodge roll hits', $has_miss);
check('miss refunds PP (flag)', $r3['pp_refund'] === true);

// no dodge possible when gap < 10
$st_nododge = make_test_state(9, 35, 40);
$r4 = battle_core_apply_action($st_nododge, ['type' => 'struggle'], $dodge_rng);
$has_miss2 = false;
foreach ($r4['events'] as $e) {
    if ($e['type'] === 'miss') {
        $has_miss2 = true;
    }
}
check('no dodge when speed gap < 10', !$has_miss2);

echo "=== victory path (first-strike KO: no counter) ===\n";
$st_v = make_test_state(5, 60, 30, 100, 5);
$r5 = battle_core_apply_action($st_v, ['type' => 'struggle']);
$types5 = [];
foreach ($r5['events'] as $e) {
    $types5[] = $e['type'];
}
check('first-strike KO ends without counter', $types5 === ['turn_start', 'move', 'damage', 'faint', 'battle_end']);
check('result victory', $r5['state']['phase'] === 'ended' && $r5['state']['result'] === 'victory');
check('enemy hp clamped to 0', $r5['state']['sides']['enemy'][0]['hp'] === 0 && $r5['state']['sides']['enemy'][0]['fainted'] === true);
check('ally untouched on first-strike KO', $r5['state']['sides']['ally'][0]['hp'] === 100);

echo "=== defeat path with substitutes (awaiting_switch) ===\n";
$st_d = make_test_state(5, 20, 50, 3, 80, 40, 250); // enemy faster and one-shots ally
$r6 = battle_core_apply_action($st_d, ['type' => 'struggle']);
check('awaiting_switch phase when ally faints with bench left to confirm', $r6['state']['phase'] === 'awaiting_switch');
$has_switch_req = false;
foreach ($r6['events'] as $e) {
    if ($e['type'] === 'switch_required') {
        $has_switch_req = true;
    }
    if ($e['type'] === 'counter') {
        check('counter killed the ally in one hit', $e['payload']['amount'] >= 3);
    }
}
check('switch_required event emitted', $has_switch_req);
check('pp refunded when ally never acted (second, KOd)', $r6['pp_refund'] === true);
check('ally fainted flag set', $r6['state']['sides']['ally'][0]['fainted'] === true);

// next turn: a replacement unit injected -> battle resumes active
$r6['state']['sides']['ally'][0] = battle_core_make_unit([
    'slot' => 0,
    'instance_id' => 12,
    'species_id' => 4,
    'name' => '小火龙',
    'species_name' => '小火龙',
    'level' => 20,
    'hp' => 90,
    'stats' => ['max_hp' => 90, 'atk' => 35, 'def' => 25, 'spatk' => 40, 'spdef' => 25, 'speed' => 55],
    'types' => ['火'],
]);
$r7 = battle_core_apply_action($r6['state'], ['type' => 'struggle']);
check('battle resumes active after switch-in', $r7['state']['phase'] === 'active');

echo "=== deterministic replay (same seed => same everything) ===\n";
function play_three_turns($seed)
{
    $st = battle_core_initial_state([
        'kind' => 'wild', 'map_id' => 3, 'rng_seed' => $seed,
        'allies' => [[
            'instance_id' => 1, 'species_id' => 6, 'name' => '蜥蜴', 'species_name' => '火恐龙', 'level' => 30,
            'hp' => 120, 'stats' => ['max_hp' => 120, 'atk' => 55, 'def' => 40, 'spatk' => 60, 'spdef' => 40, 'speed' => 70],
            'types' => ['火'],
        ]],
        'enemies' => [[
            'species_id' => 9, 'name' => '水箭龟', 'species_name' => '水箭龟', 'level' => 30,
            'hp' => 130, 'stats' => ['max_hp' => 130, 'atk' => 50, 'def' => 50, 'spatk' => 50, 'spdef' => 50, 'speed' => 65],
            'types' => ['水'],
        ]],
    ]);
    $all_events = [];
    for ($i = 0; $i < 3; $i++) {
        $r = battle_core_apply_action($st, ['type' => 'move', 'skill' => ['id' => 2, 'name' => '喷射火焰', 'power' => 90, 'type' => '火', 'category' => 1]]);
        $st = $r['state'];
        foreach ($r['events'] as $e) {
            $all_events[] = $e;
        }
    }
    return ['state' => $st, 'events' => $all_events];
}
$run_a = play_three_turns(777);
$run_b = play_three_turns(777);
$run_c = play_three_turns(778);
check('same seed replays identical event stream', json_encode($run_a['events']) === json_encode($run_b['events']));
check('same seed replays identical final HP', $run_a['state']['sides']['ally'][0]['hp'] === $run_b['state']['sides']['ally'][0]['hp'] && $run_a['state']['sides']['enemy'][0]['hp'] === $run_b['state']['sides']['enemy'][0]['hp']);
check('different seed diverges', json_encode($run_a['events']) !== json_encode($run_c['events']));
check('event seq strictly increasing', (function ($ev) {
    $last = 0;
    foreach ($ev as $e) {
        if ($e['seq'] <= $last) {
            return false;
        }
        $last = $e['seq'];
    }
    return true;
})($run_a['events']));

echo "=== injectable RNG does not consume internal counter ===\n";
$st_inj = make_test_state(5);
$counter_before = $st_inj['rng_counter'];
$r8 = battle_core_apply_action($st_inj, ['type' => 'struggle'], $rng_no_crit_full);
check('internal counter untouched when rng injected', $r8['state']['rng_counter'] === $counter_before);

echo "=== status move pipeline (power 0 + stages_boost effect, the one built-in effect) ===\n";
$st_eff = make_test_state(11);
$action = ['type' => 'move', 'skill' => [
    'id' => 99, 'name' => '剑舞', 'power' => 0, 'type' => '普通', 'category' => 0,
    'effects' => [[
        'code' => 'stages_boost', 'kind' => 'move', 'hooks' => ['on_after_move'],
        'params' => ['stat' => 'atk', 'stages' => 2], 'version' => 1,
    ]],
]];
$r9 = battle_core_apply_action($st_eff, $action);
$has_stage = false;
foreach ($r9['events'] as $e) {
    if ($e['type'] === 'stage_change') {
        $has_stage = true;
        check('stage_change payload correct', $e['payload']['stat'] === 'atk' && $e['payload']['delta'] === 2 && $e['payload']['stages'] === 2);
    }
    if ($e['type'] === 'damage') {
        check('status move deals no damage', false);
    }
}
check('status move triggers stage_change event', $has_stage);
check('stage recorded in state', $r9['state']['sides']['ally'][0]['stages']['atk'] === 2);
check('stage clamp works', (function () {
    $st = make_test_state(3);
    $st['sides']['ally'][0]['stages']['atk'] = 6;
    $st['sides']['ally'][0]['effects'] = [[
        'code' => 'stages_boost', 'kind' => 'ability', 'hooks' => ['on_after_move'],
        'params' => ['stat' => 'atk', 'stages' => 1], 'version' => 1,
    ]];
    $r = battle_core_apply_action($st, ['type' => 'move', 'skill' => ['id' => 1, 'name' => 'X', 'power' => 0, 'type' => '普通', 'category' => 0]]);
    return $r['state']['sides']['ally'][0]['stages']['atk'] === 6;
})());

echo "=== effect schema validation ===\n";
check('valid effect passes', battle_core_validate_effect(['code' => 'stages_boost', 'kind' => 'move', 'hooks' => ['on_after_move'], 'params' => ['stat' => 'atk', 'stages' => 1], 'version' => 1]) === true);
check('unknown code rejected', battle_core_validate_effect(['code' => 'explode_all', 'kind' => 'move', 'hooks' => ['on_after_move'], 'params' => [], 'version' => 1]) !== true);
check('unknown hook rejected', battle_core_validate_effect(['code' => 'stages_boost', 'kind' => 'move', 'hooks' => ['on_spaghetti'], 'params' => ['stat' => 'atk', 'stages' => 1], 'version' => 1]) !== true);
check('bad stages range rejected', battle_core_validate_effect(['code' => 'stages_boost', 'kind' => 'move', 'hooks' => ['on_after_move'], 'params' => ['stat' => 'atk', 'stages' => 9], 'version' => 1]) !== true);
check('bad stat rejected', battle_core_validate_effect(['code' => 'stages_boost', 'kind' => 'move', 'hooks' => ['on_after_move'], 'params' => ['stat' => 'hp', 'stages' => 1], 'version' => 1]) !== true);
check('bad kind rejected', battle_core_validate_effect(['code' => 'stages_boost', 'kind' => 'cheat', 'hooks' => ['on_after_move'], 'params' => ['stat' => 'atk', 'stages' => 1], 'version' => 1]) !== true);
check('missing version rejected', battle_core_validate_effect(['code' => 'stages_boost', 'kind' => 'move', 'hooks' => ['on_after_move'], 'params' => ['stat' => 'atk', 'stages' => 1]]) !== true);
check('make_unit filters invalid effects', (function () {
    $u = battle_core_make_unit(['effects' => [
        ['code' => 'nope', 'kind' => 'move', 'hooks' => [], 'params' => [], 'version' => 1],
        ['code' => 'stages_boost', 'kind' => 'ability', 'hooks' => ['on_switch_in'], 'params' => ['stat' => 'def', 'stages' => -1], 'version' => 1],
    ]]);
    return count($u['effects']) === 1 && $u['effects'][0]['code'] === 'stages_boost';
})());

echo "=== power resolution semantics ===\n";
$st_res = make_test_state(1);
$m1 = battle_core_resolve_move($st_res, ['type' => 'move', 'skill' => ['id' => 1, 'name' => 'A', 'power' => null, 'type' => '火', 'category' => 0]]);
check('missing power falls back to 40', $m1['power'] === 40);
$m2 = battle_core_resolve_move($st_res, ['type' => 'move', 'skill' => ['id' => 1, 'name' => 'A', 'power' => 0, 'type' => '火', 'category' => 0]]);
check('explicit power 0 stays a status move', $m2['power'] === 0);
$m3 = battle_core_resolve_move($st_res, ['type' => 'move', 'skill' => ['id' => 1, 'name' => 'A', 'power' => 55, 'type' => '', 'category' => 0]]);
check('empty type falls back to attacker type', $m3['type'] === '电');
$m4 = battle_core_resolve_move($st_res, ['type' => 'struggle']);
check('struggle is power 30 of own type', $m4['power'] === 30 && $m4['type'] === '电' && $m4['name'] === null);

echo "=== flee formula (legacy parity) ===\n";
$flee_hit = function ($min, $max) {
    return 1; // always lowest roll => succeeds when chance >= 0.01
};
$flee_miss = function ($min, $max) {
    return 100; // always fails unless chance >= 1.0 (capped 0.9) => always fails
};
$st_f = make_test_state(3);
$f1 = battle_core_try_flee($st_f, $flee_hit);
check('flee success on low roll', $f1['fled'] === true && $f1['state']['result'] === 'fled' && $f1['state']['phase'] === 'ended');
$st_f2 = make_test_state(3);
$f2 = battle_core_try_flee($st_f2, $flee_miss);
check('flee failure on high roll', $f2['fled'] === false);
$has_flee_msg = false;
foreach ($f2['events'] as $e) {
    if ($e['type'] === 'message' && $e['payload']['text'] === 'flee_failed') {
        $has_flee_msg = true;
    }
}
check('flee failure emits message event', $has_flee_msg);

echo "=== persistence round-trip (to_rows/from_rows) ===\n";
$st_rt = make_test_state(123);
$st_rt['sides']['ally'][0]['stages']['atk'] = 2;
$st_rt['sides']['ally'][0]['status'] = ['code' => 'poison', 'turns_left' => 3];
$st_rt['sides']['ally'][0]['volatile'] = ['confusion'];
$st_rt['sides']['ally'][0]['effects'] = [['code' => 'stages_boost', 'kind' => 'ability', 'hooks' => ['on_switch_in'], 'params' => ['stat' => 'def', 'stages' => -1], 'version' => 1]];
$st_rt['field']['weather'] = 'rain';
$st_after = $st_rt; // no turn applied: rules v2 would resolve the status and mutate it

$rows = battle_state_to_rows($st_after);
$rows['battle']['id'] = 55;
$rows['battle']['uid'] = 9;
foreach ($rows['units'] as $i => $u) {
    $rows['units'][$i]['battle_id'] = 55;
}
$rebuilt = battle_state_from_rows($rows['battle'], $rows['units']);
check('round-trip keeps versions', $rebuilt['version'] === $st_after['version'] && $rebuilt['rules_version'] === $st_after['rules_version']);
check('round-trip keeps rng seed/counter', $rebuilt['rng_seed'] === $st_after['rng_seed'] && $rebuilt['rng_counter'] === $st_after['rng_counter']);
check('round-trip keeps hp', $rebuilt['sides']['ally'][0]['hp'] === $st_after['sides']['ally'][0]['hp'] && $rebuilt['sides']['enemy'][0]['hp'] === $st_after['sides']['enemy'][0]['hp']);
check('round-trip keeps stages', $rebuilt['sides']['ally'][0]['stages']['atk'] === 2);
check('round-trip keeps status', $rebuilt['sides']['ally'][0]['status'] === ['code' => 'poison', 'turns_left' => 3]);
check('round-trip keeps volatile', $rebuilt['sides']['ally'][0]['volatile'] === ['confusion']);
check('round-trip keeps effects', count($rebuilt['sides']['ally'][0]['effects']) === 1);
check('round-trip keeps field weather', $rebuilt['field']['weather'] === 'rain');
check('round-trip keeps shiny/gender', $rebuilt['sides']['enemy'][0]['is_shiny'] === true && $rebuilt['sides']['enemy'][0]['gender'] === 1);
check('battle_id backfilled', $rebuilt['battle_id'] === 55);

echo "=== legacy state migration (battle_core_state_from_legacy) ===\n";
$legacy = [
    'npcid' => 10, 'level' => 12, 'hp' => 33, 'hpg' => 40,
    'atkg' => 21, 'defg' => 15, 'spatkg' => 18, 'spdefg' => 16, 'sdg' => 25,
    'capture' => 190, 'allure' => (1 << 1) | 1, // female + shiny
];
$migrated = battle_core_state_from_legacy($legacy, [
    'instance_id' => 77, 'species_id' => 25, 'name' => '皮卡', 'species_name' => '皮卡丘',
    'level' => 18, 'hp' => 55,
    'stats' => ['max_hp' => 55, 'atk' => 30, 'def' => 20, 'spatk' => 35, 'spdef' => 25, 'speed' => 45],
    'types' => ['电'],
], ['species_name' => '波波', 'types' => ['普通', '飞行'], 'map_id' => 0, 'rng_seed' => 999]);
$enemy = $migrated['sides']['enemy'][0];
check('legacy enemy species mapped', $enemy['species_id'] === 10 && $enemy['species_name'] === '波波');
check('legacy enemy hp/maxhp mapped', $enemy['hp'] === 33 && $enemy['stats']['max_hp'] === 40);
check('legacy enemy stats snapshot mapped', $enemy['stats']['atk'] === 21 && $enemy['stats']['speed'] === 25);
check('legacy allure decoded (female shiny)', $enemy['gender'] === 1 && $enemy['is_shiny'] === true);
check('legacy capture rate mapped', $enemy['capture_rate'] === 190);
check('migration keeps ally snapshot', $migrated['sides']['ally'][0]['hp'] === 55 && $migrated['sides']['ally'][0]['stats']['speed'] === 45);
check('migration seeds rng', $migrated['rng_seed'] === 999 && $migrated['phase'] === 'active');

echo "=== message rendering matches legacy strings ===\n";
$st_msg = make_test_state(21, 60, 30, 100, 5);
$r11 = battle_core_apply_action($st_msg, ['type' => 'move', 'skill' => ['id' => 3, 'name' => '电击', 'power' => 40, 'type' => '电', 'category' => 0]]);
$msgs = battle_core_render_messages($r11['events'], ['ally' => '皮卡', 'enemy' => '鲤鱼王']);
check('first line is attack string', preg_match('/^皮卡使用了电击，对鲤鱼王造成了\d+点伤害！$/u', $msgs[0]) === 1);
check('second line is faint string', $msgs[1] === '鲤鱼王倒下了！');
$st_msg2 = make_test_state(22, 3, 90, 10, 200, 5, 99);
$r12 = battle_core_apply_action($st_msg2, ['type' => 'struggle']);
$msgs2 = battle_core_render_messages($r12['events'], ['ally' => '皮卡', 'enemy' => '鲤鱼王']);
check('counter line rendered', strpos($msgs2[0], '鲤鱼王攻击了皮卡，造成了') === 0);
check('ally faint line rendered', in_array('皮卡倒下了...', $msgs2, true));
check('switch-required line rendered', in_array('还有可用的替补宠物，请更换宠物继续战斗！', $msgs2, true));


echo "=== rules_version 2: stage multipliers ===" . PHP_EOL;
check('stage +1 is 1.5x', battle_core_stage_multiplier(1) === 1.5);
check('stage +6 is 4x', battle_core_stage_multiplier(6) === 4);
check('stage -6 is 0.25x', battle_core_stage_multiplier(-6) === 0.25);
check('stage -1 is 2/3', abs(battle_core_stage_multiplier(-1) - 2 / 3) < 1e-9);
check('stage 0 neutral', battle_core_stage_multiplier(0) === 1);
check('accuracy mult neutral', battle_core_accuracy_multiplier(0, 0) === 1.0);
check('accuracy -6 raises miss', battle_core_accuracy_multiplier(-6, 0) < 0.3);
check('evasion +6 lowers hit', battle_core_accuracy_multiplier(0, 6) < 0.3);

echo "=== rules v2: damage scales with stages ===" . PHP_EOL;
$full_roll = function ($min, $max) {
    if ($min === 85) return 100;
    if ($min === 1) return 2; // no crit, no miss
    return $min;
};
$st_v2 = make_test_state(5, 50, 30, 100, 200); // rules_version defaults to 2 now
check('v2 is the default rules version', (int)$st_v2['rules_version'] === 2);
$move_flat = ['id' => 9, 'name' => '撞击', 'power' => 40, 'type' => '电', 'category' => 0];
$dmg_base = battle_core_calc_damage($st_v2, $st_v2['sides']['ally'][0], $st_v2['sides']['enemy'][0], $move_flat, $full_roll);
$st_up = make_test_state(5, 50, 30, 100, 200);
$st_up['sides']['ally'][0]['stages']['atk'] = 1;
$dmg_up = battle_core_calc_damage($st_up, $st_up['sides']['ally'][0], $st_up['sides']['enemy'][0], $move_flat, $full_roll);
$ratio_up = $dmg_base['amount'] > 0 ? $dmg_up['amount'] / $dmg_base['amount'] : 0;
check('attacker +1 atk scales damage toward 1.5x', $ratio_up > 1.4 && $ratio_up < 1.6);
$st_dn = make_test_state(5, 50, 30, 100, 200);
$st_dn['sides']['enemy'][0]['stages']['def'] = -1;
$dmg_dn = battle_core_calc_damage($st_dn, $st_dn['sides']['ally'][0], $st_dn['sides']['enemy'][0], $move_flat, $full_roll);
$ratio_dn = $dmg_base['amount'] > 0 ? $dmg_dn['amount'] / $dmg_base['amount'] : 0;
check('defender -1 def scales damage toward 1.5x', $ratio_dn > 1.4 && $ratio_dn < 1.6);

echo "=== rules v1: stages recorded but inert ===" . PHP_EOL;
$st_v1 = make_test_state(5, 50, 30, 100, 200);
$st_v1['rules_version'] = 1;
$st_v1['sides']['ally'][0]['stages']['atk'] = 2;
$dmg_v1 = battle_core_calc_damage($st_v1, $st_v1['sides']['ally'][0], $st_v1['sides']['enemy'][0], $move_flat, $full_roll);
$dmg_v1_plain = null;
$st_v1p = make_test_state(5, 50, 30, 100, 200);
$st_v1p['rules_version'] = 1;
$dmg_v1_plain = battle_core_calc_damage($st_v1p, $st_v1p['sides']['ally'][0], $st_v1p['sides']['enemy'][0], $move_flat, $full_roll);
check('v1 ignores stage multipliers', $dmg_v1['amount'] === $dmg_v1_plain['amount']);

echo "=== rules v2: status_inflict on hit ===" . PHP_EOL;
$st_inf = make_test_state(7, 60, 30, 100, 300);
$action_inf = ['type' => 'move', 'skill' => [
    'id' => 30, 'name' => '毒针', 'power' => 25, 'type' => '毒', 'category' => 0,
    'effects' => [[
        'code' => 'status_inflict', 'kind' => 'move', 'hooks' => ['on_hit'],
        'params' => ['status' => 'poison', 'chance' => 100], 'version' => 1,
    ]],
]];
$r_inf = battle_core_apply_action($st_inf, $action_inf);
$has_inflict = false;
foreach ($r_inf['events'] as $e) {
    if ($e['type'] === 'status_inflict') {
        $has_inflict = true;
        check('status lands on the defender', $e['payload']['side'] === 'enemy' && $e['payload']['status'] === 'poison');
    }
    if ($e['type'] === 'status_damage') {
        check('poison ticks at end of the same turn', $e['payload']['side'] === 'enemy' && $e['payload']['amount'] >= 1);
    }
}
check('status_inflict event emitted', $has_inflict);
check('enemy carries the status in state', $r_inf['state']['sides']['enemy'][0]['status'] !== null && $r_inf['state']['sides']['enemy'][0]['status']['code'] === 'poison');
check('status_inflict rejected for unknown status', battle_core_validate_effect([
    'code' => 'status_inflict', 'kind' => 'move', 'hooks' => ['on_hit'],
    'params' => ['status' => 'dna'], 'version' => 1,
]) !== true);
check('status_inflict rejects chance > 100', battle_core_validate_effect([
    'code' => 'status_inflict', 'kind' => 'move', 'hooks' => ['on_hit'],
    'params' => ['status' => 'poison', 'chance' => 120], 'version' => 1,
]) !== true);

echo "=== rules v2: sleep prevents acting, may wake ===" . PHP_EOL;
$st_slp = make_test_state(11, 60, 30, 100, 300);
$st_slp['sides']['ally'][0]['status'] = ['code' => 'sleep', 'turns_left' => 1];
// injected rng: pre-move wake roll hits, so cure + prevent; counter roll max
$wake_rng = function ($min, $max) {
    if ($min === 1 && $max === 100) return 1; // wake chance 33 -> 1 <= 33 wakes
    if ($min === 85) return 100;
    return $min;
};
$r_slp = battle_core_apply_action($st_slp, ['type' => 'struggle'], $wake_rng);
$types_slp = [];
foreach ($r_slp['events'] as $e) {
    $types_slp[] = $e['type'];
}
check('sleep cures naturally', in_array('status_cure', $types_slp, true));
check('sleep still prevents the move this turn', in_array('status_prevent', $types_slp, true));
check('no attack happened while asleep', !in_array('damage', $types_slp, true));
check('status cleared in state', $r_slp['state']['sides']['ally'][0]['status'] === null);

echo "=== rules v2: freeze blocks with chance to thaw ===" . PHP_EOL;
$st_frz = make_test_state(12, 60, 30, 100, 300);
$st_frz['sides']['ally'][0]['status'] = ['code' => 'freeze', 'turns_left' => 0];
$frozen_rng = function ($min, $max) {
    if ($min === 1 && $max === 100) return 100; // wake roll 100 > 20: stays frozen
    if ($min === 85) return 100;
    return $min;
};
$r_frz = battle_core_apply_action($st_frz, ['type' => 'struggle'], $frozen_rng);
$types_frz = [];
foreach ($r_frz['events'] as $e) {
    $types_frz[] = $e['type'];
}
check('frozen unit cannot act', in_array('status_prevent', $types_frz, true) && !in_array('damage', $types_frz, true));
check('frozen status persists', $r_frz['state']['sides']['ally'][0]['status'] !== null);

echo "=== rules v2: paralysis halves effective speed ===" . PHP_EOL;
$st_par = make_test_state(13, 60, 30, 100, 300);
$ally_par = $st_par['sides']['ally'][0];
check('plain speed compares raw', battle_core_effective_speed($st_par, $ally_par) === 60);
$ally_par['status'] = ['code' => 'paralysis', 'turns_left' => 0];
check('paralysis halves speed', battle_core_effective_speed($st_par, $ally_par) === 30);
$st_par_v1 = make_test_state(13, 60, 30, 100, 300);
$st_par_v1['rules_version'] = 1;
$ally_par_v1 = $st_par_v1['sides']['ally'][0];
$ally_par_v1['status'] = ['code' => 'paralysis', 'turns_left' => 0];
check('v1 ignores paralysis speed cut', battle_core_effective_speed($st_par_v1, $ally_par_v1) === 60);

echo "=== rules v2: opponent-targeted stage moves (sand-attack) ===" . PHP_EOL;
$st_snd = make_test_state(21, 60, 30, 100, 300);
$action_snd = ['type' => 'move', 'skill' => [
    'id' => 28, 'name' => '泼沙', 'power' => 0, 'type' => '地面', 'category' => 0,
    'effects' => [[
        'code' => 'stages_boost', 'kind' => 'move', 'hooks' => ['on_after_move'],
        'params' => ['stat' => 'accuracy', 'stages' => -1, 'target' => 'opponent'], 'version' => 1,
    ]],
]];
$r_snd = battle_core_apply_action($st_snd, $action_snd);
$has_enemy_drop = false;
foreach ($r_snd['events'] as $e) {
    if ($e['type'] === 'stage_change' && $e['payload']['side'] === 'enemy' && $e['payload']['stat'] === 'accuracy') {
        $has_enemy_drop = true;
    }
    if ($e['type'] === 'damage') {
        check('status move deals no damage in v2', false);
    }
}
check('sand-attack lowers enemy accuracy', $has_enemy_drop);
check('enemy accuracy stage stored', $r_snd['state']['sides']['enemy'][0]['stages']['accuracy'] === -1);
check('target param validated', battle_core_validate_effect([
    'code' => 'stages_boost', 'kind' => 'move', 'hooks' => ['on_after_move'],
    'params' => ['stat' => 'atk', 'stages' => 1, 'target' => 'everyone'], 'version' => 1,
]) !== true);

echo "=== rules v2: status damage can KO and decide the battle ===" . PHP_EOL;
$st_ko = make_test_state(31, 20, 60, 100, 300); // enemy faster & fat: survives our hit
$st_ko['sides']['enemy'][0]['status'] = ['code' => 'poison', 'turns_left' => 0];
$no_touch_rng = function ($min, $max) {
    if ($min === 1) return 100; // no crit, no miss
    if ($min === 85) return 100;
    return $min;
};
$r_ko = battle_core_apply_action($st_ko, ['type' => 'struggle'], $no_touch_rng);
$poison_tick = null;
foreach ($r_ko['events'] as $e) {
    if ($e['type'] === 'status_damage' && $e['payload']['side'] === 'enemy') {
        $poison_tick = $e['payload']['amount'];
    }
}
check('poison ticks for 1/8 max HP (37 of 300)', $poison_tick === 37);
$my_hit = 0;
foreach ($r_ko['events'] as $e) {
    if ($e['type'] === 'damage' && $e['payload']['side'] === 'ally') {
        $my_hit = $e['payload']['amount'];
    }
}
$enemy_after = (int) $r_ko['state']['sides']['enemy'][0]['hp'];
check('enemy HP reflects hit plus poison tick', $enemy_after === 300 - $my_hit - 37);
check('battle continues after the tick', $r_ko['state']['phase'] === 'active');



echo "=== rules v2: AI move scoring and picking ===" . PHP_EOL;
$st_ai = make_test_state(41, 50, 30, 100, 300);
$enemy_ai = $st_ai['sides']['enemy'][0];
$ally_ai = $st_ai['sides']['ally'][0];
$mud_shot = ['id' => 91, 'name' => '泥巴射击', 'power' => 40, 'type' => '地面', 'category' => 0];
$water_gun = ['id' => 92, 'name' => '水枪', 'power' => 40, 'type' => '水', 'category' => 0];
$sand = ['id' => 28, 'name' => '泼沙', 'power' => 0, 'type' => '地面', 'category' => 0];
check('super-effective move scores higher', battle_core_ai_score_move($enemy_ai, $ally_ai, $mud_shot) > battle_core_ai_score_move($enemy_ai, $ally_ai, $water_gun));
check('status move scores low', battle_core_ai_score_move($enemy_ai, $ally_ai, $sand) < battle_core_ai_score_move($enemy_ai, $ally_ai, $water_gun));
$best_rng = function ($min, $max) { return $min; }; // chance roll 1 <= 70 => best
$picked = battle_core_ai_pick_move($st_ai, [$water_gun, $mud_shot, $sand], $best_rng, 70);
check('AI picks the best move when the roll hits', $picked['id'] === 91);
$lucky_rng = function ($min, $max) { return $max; }; // 100 > 70 => random slot
$rand_pick = battle_core_ai_pick_move($st_ai, [$water_gun, $mud_shot], $lucky_rng, 70);
check('random fallback still returns a candidate move', $rand_pick['id'] === 92 || $rand_pick['id'] === 91);
check('empty candidates return null', battle_core_ai_pick_move($st_ai, [], null) === null);
check('boss difficulty is deterministic-best', (function () {
    $st = make_test_state(42, 50, 30, 100, 300);
    $m = ['id' => 91, 'name' => 'X', 'power' => 40, 'type' => '地面', 'category' => 0];
    $w = ['id' => 92, 'name' => 'Y', 'power' => 40, 'type' => '水', 'category' => 0];
    for ($i = 0; $i < 5; $i++) {
        $p = battle_core_ai_pick_move($st, [$w, $m], function ($min, $max) { return $max; }, 100);
        if ($p['id'] !== 91) return false; // 100% best regardless of roll
    }
    return true;
})());

echo "=== rules v2: enemy_move runs a real move instead of the fixed counter ===" . PHP_EOL;
$st_em = make_test_state(43, 50, 30, 100, 300);
$flat_rng = function ($min, $max) {
    if ($min === 85) return 100;
    if ($min === 1) return 100; // no miss, no crit
    return $min;
};
$r_em = battle_core_apply_action($st_em, ['type' => 'struggle', 'enemy_move' => $mud_shot], $flat_rng);
$em_types = [];
foreach ($r_em['events'] as $e) {
    $em_types[] = $e['type'];
}
check('enemy move emits a move event for the enemy side', (function () use ($r_em) {
    foreach ($r_em['events'] as $e) {
        if ($e['type'] === 'move' && $e['payload']['side'] === 'enemy' && $e['payload']['skill']['name'] === '泥巴射击') {
            return true;
        }
    }
    return false;
})());
check('enemy move emits damage instead of a counter event', in_array('damage', $em_types, true) && !in_array('counter', $em_types, true));
check('enemy move applies type effectiveness (ground vs electric = 2x)', (function () use ($r_em) {
    foreach ($r_em['events'] as $e) {
        if ($e['type'] === 'damage' && $e['payload']['side'] === 'enemy') {
            return $e['payload']['effectiveness'] === 2.0;
        }
    }
    return false;
})());
$st_v1c = make_test_state(44, 50, 30, 100, 300);
$st_v1c['rules_version'] = 1;
$r_v1c = battle_core_apply_action($st_v1c, ['type' => 'struggle', 'enemy_move' => $mud_shot], $flat_rng);
$v1c_types = [];
foreach ($r_v1c['events'] as $e) {
    $v1c_types[] = $e['type'];
}
$v1_enemy_damage = false;
foreach ($r_v1c['events'] as $e) {
    if ($e['type'] === 'damage' && $e['payload']['side'] === 'enemy') {
        $v1_enemy_damage = true;
    }
}
check('v1 ignores enemy_move and keeps the fixed counter', in_array('counter', $v1c_types, true) && !$v1_enemy_damage);


echo "\n";
if ($failures > 0) {
    echo "FAILED: {$failures} of {$checks} checks failed\n";
    exit(1);
}
echo "All {$checks} battle-core checks passed\n";
