<?php
/** Deterministic regressions for status, faint and PP handling in one battle turn. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/battle_core.php';

$checks = 0;
$failures = 0;
function check_turn($condition, $label)
{
    $GLOBALS['checks']++;
    if (!$condition) {
        $GLOBALS['failures']++;
        echo "FAIL: $label\n";
    }
}
function turn_state($ally_speed = 60, $enemy_speed = 30)
{
    $unit = [
        'level' => 20, 'hp' => 500, 'types' => ['普通'],
        'stats' => ['max_hp' => 500, 'atk' => 40, 'def' => 40, 'spatk' => 40, 'spdef' => 40, 'speed' => 30],
    ];
    $ally = $enemy = $unit;
    $ally['stats']['speed'] = $ally_speed;
    $enemy['stats']['speed'] = $enemy_speed;
    return battle_core_initial_state(['kind' => 'wild', 'rng_seed' => 42, 'allies' => [$ally], 'enemies' => [$enemy]]);
}
function turn_move($power = 40, $effects = [])
{
    return ['id' => 1, 'name' => 'Test move', 'power' => $power, 'type' => '普通', 'category' => 0, 'effects' => $effects];
}
function turn_events($result, $type, $side = null)
{
    return array_values(array_filter($result['events'], function ($event) use ($type, $side) {
        return $event['type'] === $type && ($side === null || ($event['payload']['side'] ?? null) === $side);
    }));
}
$max_rng = function ($min, $max) { return $max; };
$untyped = turn_state();
$untyped['sides']['ally'][0]['types'] = [];
$untyped['sides']['enemy'][0]['types'] = [];
check_turn(battle_core_resolve_move($untyped, ['type' => 'struggle'])['type'] === '普通', 'Missing species types use normal basic attack');
check_turn(battle_core_resolve_move_for_side($untyped, 'enemy', ['power' => 40])['type'] === '普通', 'Missing species types use normal enemy skill fallback');
$untyped_result = battle_core_apply_action($untyped, ['type' => 'struggle', 'enemy_move' => ['power' => 40]], $max_rng);
check_turn($untyped_result['state']['sides']['ally'][0]['hp'] < 500 && $untyped_result['state']['sides']['enemy'][0]['hp'] < 500, 'Untyped species complete both actions without warnings');
$boost = ['code' => 'stages_boost', 'kind' => 'move', 'hooks' => ['on_after_move'], 'params' => ['stat' => 'atk', 'stages' => 1], 'version' => 1];
$passive = ['code' => 'stages_boost', 'kind' => 'ability', 'hooks' => ['on_switch_in'], 'params' => ['stat' => 'def', 'stages' => 1], 'version' => 1];

// Each enemy action, including the legacy counter fallback, gets one status check.
foreach (['sleep', 'freeze', 'paralysis'] as $status) {
    foreach ([true, false] as $ally_first) {
        foreach ([true, false] as $enemy_skill) {
            $state = turn_state($ally_first ? 100 : 10, 60);
            $state['sides']['enemy'][0]['status'] = ['code' => $status, 'turns_left' => 3];
            $action = ['type' => 'move', 'skill' => turn_move()];
            if ($enemy_skill) $action['enemy_move'] = turn_move();
            $rng = function ($min, $max) use ($status) {
                return $status === 'paralysis' && $min === 1 && $max === 100 ? 1 : $max;
            };
            $result = battle_core_apply_action($state, $action, $rng);
            $label = "$status ally_first=" . (int)$ally_first . ' enemy_skill=' . (int)$enemy_skill;
            check_turn(count(turn_events($result, 'status_prevent', 'enemy')) === 1, "$label prevents one enemy action");
            check_turn(!turn_events($result, 'move', 'enemy') && !turn_events($result, 'counter'), "$label emits no enemy attack");
            check_turn($result['state']['sides']['ally'][0]['hp'] === 500, "$label causes no enemy damage");
            check_turn($result['state']['rng_counter'] === 0, "$label uses only injected RNG");
        }
    }
}

// Item, capture and failed-flee turns enter the counter helper directly.
foreach (['sleep', 'freeze', 'paralysis'] as $status) {
    $state = turn_state();
    $state['sides']['enemy'][0]['status'] = ['code' => $status, 'turns_left' => 3];
    $events = [];
    $rng = function ($min, $max) use ($status) {
        return $status === 'paralysis' && $min === 1 && $max === 100 ? 1 : $max;
    };
    battle_core_counter_attack($state, $events, $rng);
    check_turn($state['sides']['ally'][0]['hp'] === 500 && count($events) === 1 && $events[0]['type'] === 'status_prevent', "$status also prevents a standalone counter");
}
foreach ([true, false] as $enemy_skill) {
    $state = turn_state();
    $state['sides']['enemy'][0]['status'] = ['code' => 'paralysis', 'turns_left' => 0];
    $action = ['type' => 'move', 'skill' => turn_move(0)];
    if ($enemy_skill) $action['enemy_move'] = turn_move(0);
    $status_rolls = 0;
    $rng = function ($min, $max) use (&$status_rolls) {
        if ($min === 1 && $max === 100) return ++$status_rolls === 1 ? 100 : 1;
        return $max;
    };
    $result = battle_core_apply_action($state, $action, $rng);
    check_turn($status_rolls === 1 && !turn_events($result, 'status_prevent'), 'Enemy action has one successful paralysis roll, enemy_skill=' . (int)$enemy_skill);
    check_turn(count(turn_events($result, $enemy_skill ? 'move' : 'counter', 'enemy')) === 1, 'Successful enemy paralysis roll allows its action, enemy_skill=' . (int)$enemy_skill);
}

// A successful first paralysis roll must not be followed by a second roll.
$state = turn_state(100, 30);
$state['sides']['ally'][0]['status'] = ['code' => 'paralysis', 'turns_left' => 0];
$status_rolls = 0;
$rng = function ($min, $max) use (&$status_rolls) {
    if ($min === 1 && $max === 100) return ++$status_rolls === 1 ? 100 : 1;
    return $max;
};
$result = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move()], $rng);
check_turn($status_rolls === 1, 'Fast paralyzed ally gets one prevention roll');
check_turn(count(turn_events($result, 'move', 'ally')) === 1, 'Successful paralysis roll lets ally attack');
check_turn(count(turn_events($result, 'counter')) === 1, 'Paralysis roll cannot silently remove enemy action');
check_turn(!turn_events($result, 'status_prevent'), 'No prevention event after a successful paralysis roll');

// Both units can be prevented in the same turn; their temporary skill effects expire.
$state = turn_state();
foreach (['ally', 'enemy'] as $side) {
    $state['sides'][$side][0]['status'] = ['code' => 'freeze', 'turns_left' => 3];
    $state['sides'][$side][0]['effects'] = [$passive];
}
$result = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move(0, [$boost]), 'enemy_move' => turn_move(0, [$boost])], $max_rng);
check_turn(count(turn_events($result, 'status_prevent')) === 2, 'Frozen opponents each lose their own action');
check_turn(!turn_events($result, 'stage_change'), 'Prevented moves cannot apply their temporary effects');
foreach (['ally', 'enemy'] as $side) {
    check_turn($result['state']['sides'][$side][0]['effects'] === [$passive], "$side retains only its original effects after a prevented turn");
}
check_turn($result['pp_refund'] === false, 'Status prevention still consumes the selected move PP');

// An effect inflicted by the faster ally must prevent the enemy's upcoming move.
$freeze = ['code' => 'status_inflict', 'kind' => 'move', 'hooks' => ['on_hit'], 'params' => ['status' => 'freeze', 'chance' => 100], 'version' => 1];
$result = battle_core_apply_action(turn_state(), ['type' => 'move', 'skill' => turn_move(40, [$freeze]), 'enemy_move' => turn_move(0, [$boost])], $max_rng);
check_turn(count(turn_events($result, 'status_inflict', 'enemy')) === 1 && count(turn_events($result, 'status_prevent', 'enemy')) === 1, 'Newly inflicted freeze blocks the same turn enemy action');
check_turn(!turn_events($result, 'stage_change') && $result['state']['sides']['ally'][0]['hp'] === 500, 'Frozen enemy cannot execute its queued effect');

// Persistent stages survive a save/load boundary without retaining the move's effects.
$state = turn_state();
$state['battle_id'] = 7;
foreach (['ally', 'enemy'] as $side) $state['sides'][$side][0]['effects'] = [$passive];
$result = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move(0, [$boost]), 'enemy_move' => turn_move(0, [$boost])], $max_rng);
$rows = battle_state_to_rows($result['state']);
$rows['battle']['id'] = 7;
$resumed = battle_state_from_rows($rows['battle'], $rows['units']);
$next = battle_core_apply_action($resumed, ['type' => 'move', 'skill' => turn_move(0), 'enemy_move' => turn_move(0)], $max_rng);
foreach (['ally', 'enemy'] as $side) {
    check_turn($next['state']['sides'][$side][0]['stages']['atk'] === 1, "$side skill applies once across save/load and the next turn");
    check_turn($next['state']['sides'][$side][0]['effects'] === [$passive], "$side temporary skill effect never persists");
}
check_turn(!turn_events($next, 'stage_change'), 'A later plain move cannot retrigger a previous skill effect');

// Added enemy status checks remain reproducible with the built-in RNG and persisted counter.
$state = turn_state();
$state['battle_id'] = 7;
$state['sides']['enemy'][0]['status'] = ['code' => 'paralysis', 'turns_left' => 0];
$action = ['type' => 'move', 'skill' => turn_move(), 'enemy_move' => turn_move()];
$first = battle_core_apply_action($state, $action);
$rows = battle_state_to_rows($first['state']);
$rows['battle']['id'] = 7;
$resumed = battle_state_from_rows($rows['battle'], $rows['units']);
check_turn(battle_core_apply_action($first['state'], $action) == battle_core_apply_action($resumed, $action), 'Status turns replay identically after persistence');
check_turn(battle_core_apply_action($state, $action) === $first, 'Same seed and status action produce identical state and events');

// The endpoint decides whether a fainted ally has a replacement in the player's party.
foreach ([true, false] as $ally_first) {
    $state = turn_state($ally_first ? 60 : 10, 30);
    $state['sides']['ally'][0]['hp'] = 1;
    $state['sides']['ally'][0]['effects'] = [$passive];
    $result = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move(0, [$boost]), 'enemy_move' => turn_move()], $max_rng);
    check_turn($result['state']['phase'] === 'awaiting_switch' && $result['state']['result'] === null, 'Enemy skill KO leaves replacement decision to endpoint');
    check_turn(count(turn_events($result, 'switch_required')) === 1 && !turn_events($result, 'battle_end'), 'Enemy skill KO emits one switch request without premature defeat');
    check_turn($result['pp_refund'] === !$ally_first, 'Enemy skill KO refunds PP only if ally never acted');
    check_turn($result['state']['sides']['ally'][0]['effects'] === [$passive], 'Enemy skill KO strips temporary ally effects');
}

// Once residual damage interrupts the turn, no later unit can overwrite the outcome.
$state = turn_state();
foreach (['ally', 'enemy'] as $side) {
    $state['sides'][$side][0]['hp'] = 1;
    $state['sides'][$side][0]['status'] = ['code' => 'poison', 'turns_left' => 0];
}
$result = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move(0), 'enemy_move' => turn_move(0)], $max_rng);
check_turn($result['state']['phase'] === 'awaiting_switch' && $result['state']['result'] === null, 'Ally poison KO waits for a replacement');
check_turn(count(turn_events($result, 'status_damage')) === 1 && $result['state']['sides']['enemy'][0]['hp'] === 1, 'Status resolution stops as soon as the turn is interrupted');
check_turn(count(turn_events($result, 'switch_required')) === 1 && !turn_events($result, 'battle_end'), 'Residual damage cannot emit contradictory battle outcomes');

$state = turn_state();
$state['sides']['enemy'][0]['hp'] = 1;
$state['sides']['enemy'][0]['status'] = ['code' => 'poison', 'turns_left' => 0];
$result = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move(0), 'enemy_move' => turn_move(0)], $max_rng);
check_turn($result['state']['phase'] === 'ended' && $result['state']['result'] === 'victory', 'Enemy poison KO still produces victory');
check_turn(count(turn_events($result, 'battle_end')) === 1, 'Enemy poison KO ends the battle exactly once');

$state = turn_state();
$state['sides']['ally'][0]['hp'] = $state['sides']['enemy'][0]['hp'] = 1;
$state['sides']['ally'][0]['status'] = ['code' => 'poison', 'turns_left' => 0];
$result = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move()], $max_rng);
check_turn($result['state']['result'] === 'victory' && $result['state']['sides']['ally'][0]['hp'] === 1, 'Direct victory remains final before residual damage');
check_turn(!turn_events($result, 'status_damage') && count(turn_events($result, 'battle_end')) === 1, 'No residual ticks after direct victory');

// Stage-adjusted turn order can make a slower enemy act first and miss.
$state = turn_state(50, 30);
$state['sides']['enemy'][0]['stages']['speed'] = 2;
$miss_rng = function ($min, $max) { return $min === 1 && $max === 100 ? 1 : $max; };
$result = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move(), 'enemy_move' => turn_move()], $miss_rng);
check_turn(count(turn_events($result, 'miss', 'enemy')) === 1 && count(turn_events($result, 'damage', 'ally')) === 1, 'Enemy misses before the ally successfully hits');
check_turn($result['pp_refund'] === false, 'Enemy miss never refunds the ally skill PP');
$state = turn_state(30, 60);
$result = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move(), 'enemy_move' => turn_move()], $miss_rng);
check_turn(count(turn_events($result, 'miss', 'ally')) === 1 && $result['pp_refund'] === true, 'An actual ally miss still refunds PP');

// Existing version-one battles keep their pre-status rules and fixed counter.
$state = turn_state();
$state['rules_version'] = 1;
$state['sides']['enemy'][0]['status'] = ['code' => 'freeze', 'turns_left' => 3];
$result = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move(), 'enemy_move' => turn_move()], $max_rng);
check_turn(count(turn_events($result, 'counter')) === 1 && !turn_events($result, 'status_prevent'), 'Version-one battles retain their legacy counter behavior');

// Modern type directions follow Pokemon Showdown's typechart.ts damageTaken:
// 0 = neutral, 1 = weakness, 2 = resistance, 3 = immunity (sim/dex-data.ts).
// https://github.com/smogon/pokemon-showdown/blob/master/data/typechart.ts
foreach ([['妖精', '龙', 2.0], ['钢', '毒', 1.0], ['龙', '妖精', 0.0], ['毒', '钢', 0.0],
    ['格斗', '毒', 0.5], ['格斗', '虫', 0.5], ['火', '岩石', 0.5]] as $case) {
    check_turn(battle_core_type_effectiveness($case[0], [$case[1]]) === $case[2], "Modern chart direction $case[0] -> $case[1]");
}
foreach ([['普通', '幽灵'], ['格斗', '幽灵'], ['毒', '钢'], ['地面', '飞行'],
    ['电', '地面'], ['超能', '恶'], ['幽灵', '普通'], ['龙', '妖精']] as $pair) {
    $state = turn_state();
    $state['sides']['enemy'][0]['types'] = [$pair[1]];
    $move = array_merge(turn_move(), ['type' => $pair[0]]);
    $damage = battle_core_calc_damage($state, $state['sides']['ally'][0], $state['sides']['enemy'][0], $move, $max_rng);
    check_turn($damage['effectiveness'] === 0.0 && $damage['amount'] === 0, "Modern immunity deals zero damage: $pair[0] -> $pair[1]");
}
$state = turn_state();
$state['sides']['enemy'][0]['types'] = ['飞行', '地面'];
$electric = array_merge(turn_move(), ['type' => '电']);
$damage = battle_core_calc_damage($state, $state['sides']['ally'][0], $state['sides']['enemy'][0], $electric, $max_rng);
check_turn($damage['effectiveness'] === 0.0 && $damage['amount'] === 0, 'Dual-type weakness does not cancel immunity');

$poison_hit = ['code' => 'status_inflict', 'kind' => 'move', 'hooks' => ['on_hit'],
    'params' => ['status' => 'poison', 'chance' => 100], 'version' => 1];
check_turn(battle_core_validate_effect($poison_hit) === true, 'Immunity regression uses a supported secondary effect');
foreach (['ally', 'enemy'] as $actor_side) {
    foreach ([1, 100] as $hp) {
        $state = turn_state(60, 60);
        $target_side = $actor_side === 'ally' ? 'enemy' : 'ally';
        $state['sides'][$target_side][0]['types'] = ['幽灵'];
        $state['sides'][$target_side][0]['hp'] = $hp;
        $action = ['type' => 'move', 'skill' => turn_move(0), 'enemy_move' => turn_move(0)];
        $action[$actor_side === 'ally' ? 'skill' : 'enemy_move'] = turn_move(40, [$poison_hit]);
        $result = battle_core_apply_action($state, $action, $max_rng);
        check_turn($result['state']['sides'][$target_side][0]['hp'] === $hp,
            "$actor_side immune hit preserves target HP=$hp");
        check_turn($result['state']['phase'] === 'active' && !turn_events($result, 'faint') && !turn_events($result, 'battle_end'),
            "$actor_side immune hit cannot decide the battle at HP=$hp");
        check_turn($result['state']['sides'][$target_side][0]['status'] === null && !turn_events($result, 'status_inflict'),
            "$actor_side immune hit cannot apply its secondary poison at HP=$hp");
        check_turn(!$result['pp_refund'], "$actor_side immunity preserves consumed PP at HP=$hp");
    }
}

// Explicit v1 formula compatibility keeps its original chart and minimum damage.
foreach ([['普通', '幽灵'], ['妖精', '龙'], ['钢', '毒']] as $pair) {
    $state = turn_state();
    $state['rules_version'] = 1;
    $state['sides']['enemy'][0]['types'] = [$pair[1]];
    $move = array_merge(turn_move(), ['type' => $pair[0]]);
    $damage = battle_core_calc_damage($state, $state['sides']['ally'][0], $state['sides']['enemy'][0], $move, $max_rng);
    check_turn($damage['effectiveness'] === 0.0 && $damage['amount'] === 1, "Version-one damage stays compatible: $pair[0] -> $pair[1]");
}
foreach ([['格斗', '毒'], ['格斗', '虫'], ['火', '岩石']] as $pair) {
    $state = turn_state();
    $state['rules_version'] = 1;
    $state['sides']['enemy'][0]['types'] = [$pair[1]];
    $move = array_merge(turn_move(), ['type' => $pair[0]]);
    $damage = battle_core_calc_damage($state, $state['sides']['ally'][0], $state['sides']['enemy'][0], $move, $max_rng);
    check_turn($damage['effectiveness'] === 1.0 && $damage['amount'] > 0, "Version-one resistance stays compatible: $pair[0] -> $pair[1]");
}

// Fleeing from the replacement dialog is penalty-free. The core only holds the
// current combatant, so "awaiting_switch with no active ally unit" is the state
// the endpoint keeps while the party still has a bench: no roll, no enemy turn.
$min_rng = function ($min, $max) { return $min; };
$state = turn_state();
$state['sides']['ally'][0]['hp'] = 1;
$ko = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move(0), 'enemy_move' => turn_move()], $max_rng);
$flee_rolls = 0;
$flee_fail_rng = function ($min, $max) use (&$flee_rolls) {
    $flee_rolls++;
    return $max; // the highest roll always fails the level chance (capped at 0.9)
};
$fled = battle_core_try_flee($ko['state'], $flee_fail_rng);
check_turn($ko['state']['phase'] === 'awaiting_switch' && battle_core_active_unit($ko['state'], 'ally') === null, 'Enemy KO leaves the fainted ally side without an active unit');
check_turn($fled['fled'] === true && $flee_rolls === 0, 'Fleeing with no active ally unit succeeds without a roll');
check_turn($fled['state']['phase'] === 'ended' && $fled['state']['result'] === 'fled', 'Penalty-free flee ends the battle as fled');
check_turn(count($fled['events']) === 1 && $fled['events'][0]['type'] === 'battle_end' && $fled['events'][0]['payload'] === ['result' => 'fled'], 'Penalty-free flee emits exactly one battle_end event');
check_turn($fled['state']['sides']['enemy'][0]['hp'] === $ko['state']['sides']['enemy'][0]['hp'] && $fled['state']['turn'] === $ko['state']['turn'], 'Penalty-free flee leaves the enemy untouched and takes no turn');
check_turn(battle_core_try_flee($ko['state'])['state']['rng_counter'] === $ko['state']['rng_counter'], 'Penalty-free flee consumes no built-in RNG draw');

// The endpoint records defeat once no replacement is left; that ended battle is
// never rewritten into a flee.
$defeat = $ko['state'];
$defeat['phase'] = 'ended';
$defeat['result'] = 'defeat';
$not_fled = battle_core_try_flee($defeat, $min_rng);
check_turn($not_fled['fled'] === false && $not_fled['state'] === $defeat && $not_fled['events'] === [], 'Ended battle with a fainted ally is not rewritten by a flee attempt');

// With an active ally unit the level chance is still rolled.
$rolled = battle_core_try_flee(turn_state(), $min_rng);
check_turn($rolled['fled'] === true && $rolled['state']['phase'] === 'ended' && $rolled['state']['result'] === 'fled', 'Active ally still flees on the lowest roll');
check_turn(count(turn_events($rolled, 'battle_end')) === 1 && !turn_events($rolled, 'message'), 'Chance flee emits battle_end without flee_failed');
$rolled = battle_core_try_flee(turn_state(), $max_rng);
check_turn($rolled['fled'] === false && $rolled['state']['phase'] === 'active' && $rolled['state']['result'] === null && !turn_events($rolled, 'battle_end'), 'Active ally still fails on the highest roll');
check_turn(count(turn_events($rolled, 'message')) === 1 && turn_events($rolled, 'message')[0]['payload'] === ['text' => 'flee_failed'], 'Failed chance flee still emits flee_failed');
check_turn(battle_core_try_flee(turn_state())['state']['rng_counter'] === 1, 'Chance flee still consumes one built-in RNG draw');

// Nothing to flee from once the enemy side has no active unit.
$state = turn_state();
$state['sides']['enemy'][0]['hp'] = 1;
$won = battle_core_apply_action($state, ['type' => 'move', 'skill' => turn_move()], $max_rng);
$not_fled = battle_core_try_flee($won['state'], $min_rng);
check_turn($won['state']['phase'] === 'ended' && battle_core_active_unit($won['state'], 'enemy') === null, 'First-strike KO leaves the enemy side without an active unit');
check_turn($not_fled['fled'] === false && $not_fled['state'] === $won['state'] && $not_fled['events'] === [], 'No enemy unit means the flee attempt fails without touching the won battle');

if ($failures) {
    echo "$failures of $checks battle turn regression assertions failed\n";
    exit(1);
}
echo "$checks battle turn regression assertions passed\n";
