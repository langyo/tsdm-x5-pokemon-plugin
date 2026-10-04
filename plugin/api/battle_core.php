<?php

/**
 * 战斗引擎 2.0 —— 纯函数核心（issue #75 第一阶段 / E0 契约）
 *
 * 本文件是"和传输无关"的战斗核心：不碰 DB / HTTP / session / 全局状态，
 * 只做数组变换：apply_action(state, action, rng) -> {state, events}。
 * 端点（battle.php）只负责三件事：载入状态 -> 调核心 -> 落库 + 组响应。
 *
 * E0 契约（本阶段固化、后续阶段不返工的部分）：
 *  1. 状态支持多单位（sides.ally[] / sides.enemy[]），字段一开始就含
 *     能力等级 stages / 异常 status / 易失 volatile / buffs / 天气场地 field，
 *     为换宠、双打、团本铺路；
 *  2. 核心是纯函数，端点不写战斗逻辑；
 *  3. RNG 可注入（rng 回调），内建确定性 PRNG 以 seed+counter 驱动，
 *     每场持久化 rng_seed / rng_counter，同 seed + 同 action 序列 => 同结果（可重放）；
 *  4. 事件流格式固定、版本化（EVENT_SCHEMA_VERSION），战报/回放共用；
 *  5. 效果引擎的钩子集合与 params schema 在此定死，本阶段内置 1 个效果
 *     （stages_boost，变化技升降能力等级）打通端到端管线。
 *
 * 数值兼容：本阶段（rules_version 1）伤害/会心/闪避/先手公式与旧版
 * calculate_damage_legacy / calculate_counter_damage_legacy 逐项一致，
 * 客户端响应字段与文案不变；能力等级暂只记录（stage_change 事件），
 * 不改伤害公式，待后续规则版本切换。
 *
 * 持久化编解码同样是纯函数：
 *  battle_state_to_rows(state) / battle_state_from_rows() 与
 *  pm_battle + pm_battle_unit 行互转；battle_core_state_from_legacy()
 *  把 pm_usersdata 旧列里进行中的战斗升级为新状态（惰性迁移）。
 */

defined('IN_DISCUZ') || exit('Access Denied');

/** 引擎状态结构版本（state.version，结构不兼容变更时 +1 并写迁移） */
define('BATTLE_STATE_VERSION', 2);

/** 规则版本：数值规则（伤害公式/能力等级语义等）变更时 +1，老战斗按老规则收尾 */
define('BATTLE_RULES_VERSION', 2);

/** 事件流 schema 版本（事件类型集合 / payload 字段变更时 +1） */
define('BATTLE_EVENT_SCHEMA_VERSION', 1);

/**
 * 能力等级 -> 伤害/速度乘数（Gen3+ 经典表，rules_version 2 起）。
 * -6..-1: 2/8 2/7 2/6 2/5 1/2 2/3；+1..+6: 3/2 2 5/2 3 7/2 4。
 * @return float|int
 */
function battle_core_stage_multiplier($stage)
{
    $table = [
        -6 => 2 / 8, -5 => 2 / 7, -4 => 2 / 6, -3 => 2 / 5, -2 => 1 / 2, -1 => 2 / 3,
        0 => 1,
        1 => 3 / 2, 2 => 2, 3 => 5 / 2, 4 => 3, 5 => 7 / 2, 6 => 4,
    ];
    $stage = max(-6, min(6, (int)$stage));
    return $table[$stage];
}

/**
 * 命中/闪避等级 -> 命中率乘数（rules_version 2 起）。
 * accuracy 阶段每 +1 命中率 x1.33，每 -1 x0.75；evasion 对防守方反向同理。
 */
function battle_core_accuracy_multiplier($accuracy_stage, $evasion_stage)
{
    $mult = 1.0;
    for ($i = 0; $i < max(0, $accuracy_stage); $i++) {
        $mult *= 4 / 3;
    }
    for ($i = 0; $i < max(0, -$accuracy_stage); $i++) {
        $mult *= 3 / 4;
    }
    for ($i = 0; $i < max(0, $evasion_stage); $i++) {
        $mult *= 3 / 4;
    }
    for ($i = 0; $i < max(0, -$evasion_stage); $i++) {
        $mult *= 4 / 3;
    }
    return $mult;
}

/**
 * 内置异常状态定义目录（rules_version 2 的缺省数据；pm_status 表存在时
 * 端点可传入覆盖版本，核心保持纯函数）。behavior 字段：
 * - turn_damage: 每回合结束伤害比例（'1/8' / '1/16' max HP）
 * - skip_turn_chance: 行动前无法行动的概率（0-100，100=必定）
 * - speed_divisor: 速度除数（麻痹 2）
 * - wake_chance: 每回合解除概率（睡眠/冰冻）
 * - min_turns: 最短持续回合（睡眠）
 */
function battle_core_status_catalog()
{
    return [
        'poison' => ['name' => '中毒', 'turn_damage' => '1/8', 'skip_turn_chance' => 0, 'wake_chance' => 0],
        'burn' => ['name' => '灼烧', 'turn_damage' => '1/16', 'skip_turn_chance' => 0, 'wake_chance' => 0],
        'paralysis' => ['name' => '麻痹', 'turn_damage' => null, 'skip_turn_chance' => 25, 'speed_divisor' => 2, 'wake_chance' => 0],
        'sleep' => ['name' => '睡眠', 'turn_damage' => null, 'skip_turn_chance' => 100, 'wake_chance' => 33, 'min_turns' => 1],
        'freeze' => ['name' => '冰冻', 'turn_damage' => null, 'skip_turn_chance' => 100, 'wake_chance' => 20],
        'confusion' => ['name' => '混乱', 'turn_damage' => null, 'skip_turn_chance' => 0, 'wake_chance' => 0],
    ];
}

/**
 * 解析 turn_damage 比例字符串（'1/8'）为分数值。
 */
function battle_core_status_damage_ratio($turn_damage)
{
    if (!$turn_damage) {
        return 0;
    }
    if (preg_match('#^(\d+)/(\d+)$#', $turn_damage, $m)) {
        return (int)$m[1] / (int)$m[2];
    }
    return 0;
}

/**
 * 事件类型集合（固定格式：type + payload 纯数据，无文案；文案由渲染层生成）
 */
function battle_core_event_types()
{
    return [
        'battle_start',    // 开战 {kind, map_id}
        'turn_start',      // 回合开始 {turn}
        'switch_in',       // 单位入场 {side, slot, instance_id}
        'switch_out',      // 单位退场 {side, slot, instance_id}
        'move',            // 出招 {side, slot, skill|null, target_side, target_slot}
        'miss',            // 未命中 {side, slot, target_side, target_slot}
        'damage',          // 造成伤害 {side, slot, target_side, target_slot, amount, effectiveness, crit, stat}
        'counter',         // 野怪反击 {side, slot, target_side, target_slot, amount}
        'stage_change',    // 能力等级变化 {side, slot, stat, delta, stages}
        'faint',           // 单位倒下 {side, slot}
        'switch_required', // 我方倒下但有替补，等待换宠 {side}
        'status_inflict',  // 附加异常状态 {side, slot, status, turns}
        'status_damage',   // 异常状态每回合伤害 {side, slot, status, amount}
        'status_prevent',  // 异常状态阻止行动 {side, slot, status}
        'status_cure',     // 异常状态解除 {side, slot, status, natural}
        'battle_end',      // 战斗结束 {result}
        'message',         // 兜底文案事件 {text}
    ];
}

/**
 * 效果引擎：钩子集合（move/ability/item/weather/field/status 共用同一套）
 */
function battle_core_effect_hooks()
{
    return [
        'on_battle_start',   // 战斗开始（入场效果/天气场地挂载）
        'on_switch_in',      // 单位入场
        'on_switch_out',     // 单位退场
        'on_before_move',    // 出招前（可拦截/改招）
        'on_damage_calc',    // 伤害计算（修正倍率）
        'on_hit',            // 命中后（附加效果/异常）
        'on_after_move',     // 出招后（变化技主效果在此生效）
        'on_turn_end',       // 回合末结算（持续伤害/自然解除）
        'on_faint',          // 单位倒下
    ];
}

/**
 * 效果 params schema：code => [param => 类型描述]
 * 数据校验（battle_core_validate_effect）按此拒绝未知/坏数据。
 */
function battle_core_effect_schema()
{
    return [
        'stages_boost' => [
            'stat' => 'atk|def|spatk|spdef|speed|accuracy|evasion',
            'stages' => 'int -6..+6 相对变化量',
            'target' => 'self|opponent（缺省 self）',
        ],
        'status_inflict' => [
            'status' => 'poison|burn|paralysis|sleep|freeze|confusion',
            'chance' => 'int 0-100 命中后附加概率（缺省 100）',
        ],
    ];
}

/**
 * 校验效果声明（避免"存了不支持的效果把数据搞坏"）。
 *
 * 效果声明结构：{code, kind: move|ability|item|weather|field|status,
 * hooks: [..], params: {..}, version: int}
 *
 * @param array $effect
 * @return true|string true 合法；否则错误说明
 */
function battle_core_validate_effect($effect)
{
    if (!is_array($effect)) {
        return 'effect must be an array';
    }
    if (!isset($effect['code']) || !is_string($effect['code'])) {
        return 'effect.code missing';
    }
    $schema = battle_core_effect_schema();
    if (!isset($schema[$effect['code']])) {
        return "unknown effect code: {$effect['code']}";
    }
    if (!isset($effect['kind']) || !in_array($effect['kind'], ['move', 'ability', 'item', 'weather', 'field', 'status'], true)) {
        return "invalid effect kind";
    }
    if (!isset($effect['hooks']) || !is_array($effect['hooks'])) {
        return 'effect.hooks missing';
    }
    $known_hooks = battle_core_effect_hooks();
    foreach ($effect['hooks'] as $hook) {
        if (!in_array($hook, $known_hooks, true)) {
            return "unknown hook: {$hook}";
        }
    }
    if (!isset($effect['params']) || !is_array($effect['params'])) {
        return 'effect.params missing';
    }
    if ($effect['code'] === 'stages_boost') {
        $stat = isset($effect['params']['stat']) ? $effect['params']['stat'] : '';
        if (!in_array($stat, ['atk', 'def', 'spatk', 'spdef', 'speed', 'accuracy', 'evasion'], true)) {
            return 'stages_boost.params.stat invalid';
        }
        $stages = isset($effect['params']['stages']) ? intval($effect['params']['stages']) : 0;
        if (abs($stages) < 1 || abs($stages) > 6) {
            return 'stages_boost.params.stages out of range';
        }
        if (isset($effect['params']['target']) && !in_array($effect['params']['target'], ['self', 'opponent'], true)) {
            return 'stages_boost.params.target invalid';
        }
    }
    if ($effect['code'] === 'status_inflict') {
        $status = isset($effect['params']['status']) ? $effect['params']['status'] : '';
        if (!isset(battle_core_status_catalog()[$status])) {
            return 'status_inflict.params.status unknown';
        }
        $chance = isset($effect['params']['chance']) ? intval($effect['params']['chance']) : 100;
        if ($chance < 0 || $chance > 100) {
            return 'status_inflict.params.chance out of range';
        }
    }
    if (!isset($effect['version']) || intval($effect['version']) < 1) {
        return 'effect.version missing';
    }
    return true;
}

/**
 * 32 位安全乘法（PHP int 是 64 位，直接乘会溢出成 float 丢失低位、
 * 破坏 PRNG 雪崩特性），返回 (a*b) mod 2^32。
 */
function battle_core_mul32($a, $b)
{
    $a &= 0xFFFFFFFF;
    $b &= 0xFFFFFFFF;
    $ll = ($a & 0xFFFF) * ($b & 0xFFFF);
    $lh = ($a & 0xFFFF) * ($b >> 16);
    $hl = ($a >> 16) * ($b & 0xFFFF);
    $mid = $lh + $hl;
    return ($ll + (($mid & 0xFFFF) << 16)) & 0xFFFFFFFF;
}

/**
 * mulberry32：确定性 PRNG，(seed, counter) -> [0, 2^32) 均匀分布。
 * 每次 draw 消耗一个 counter，counter 随状态持久化，支持断点重放。
 */
function battle_core_rng_step($seed, $counter)
{
    $t = battle_core_mul32(((int)$seed) + battle_core_mul32((int)$counter, 0x9E3779B9), 1);
    // mulberry32 标准混合
    $t = $t ^ ($t >> 15);
    $t = battle_core_mul32($t, 0x85EBCA6B);
    $t = $t ^ ($t >> 13);
    $t = battle_core_mul32($t, 0xC2B2AE35);
    $t = $t ^ ($t >> 16);
    return $t & 0xFFFFFFFF;
}

/**
 * 取一次随机数 [min, max] 闭区间。
 *
 * 优先使用注入的 $rng 回调（callable($min, $max)），此时不消耗内建
 * counter；未注入时走内建确定性 PRNG 并推进 state.rng_counter。
 *
 * @param array &$state 引用仅为内部推进 rng_counter，对外仍是纯变换
 * @param callable|null $rng
 * @return int
 */
function battle_core_rand(&$state, $rng, $min, $max)
{
    if ($max < $min) {
        $tmp = $min;
        $min = $max;
        $max = $tmp;
    }
    if (is_callable($rng)) {
        return intval(call_user_func($rng, $min, $max));
    }
    $span = $max - $min + 1;
    $v = battle_core_rng_step($state['rng_seed'], $state['rng_counter']);
    $state['rng_counter'] = ((int)$state['rng_counter']) + 1;
    return $min + ($v % $span);
}

/**
 * 构造参战单位（E0：字段一次到位）
 *
 * @return array unit
 */
function battle_core_make_unit($opts)
{
    $stats = array_merge(
        ['max_hp' => 1, 'atk' => 1, 'def' => 1, 'spatk' => 1, 'spdef' => 1, 'speed' => 1],
        isset($opts['stats']) && is_array($opts['stats']) ? $opts['stats'] : []
    );
    $types = [];
    foreach ((isset($opts['types']) && is_array($opts['types']) ? $opts['types'] : []) as $t) {
        if ($t !== '' && $t !== null) {
            $types[] = $t;
        }
    }
    $effects = [];
    if (isset($opts['effects']) && is_array($opts['effects'])) {
        foreach ($opts['effects'] as $eff) {
            if (battle_core_validate_effect($eff) === true) {
                $effects[] = $eff;
            }
        }
    }
    return [
        'slot' => isset($opts['slot']) ? intval($opts['slot']) : 0,
        'instance_id' => isset($opts['instance_id']) ? intval($opts['instance_id']) : 0,
        'species_id' => isset($opts['species_id']) ? intval($opts['species_id']) : 0,
        'species_name' => isset($opts['species_name']) ? strval($opts['species_name']) : '',
        'name' => isset($opts['name']) ? strval($opts['name']) : '',
        'level' => isset($opts['level']) ? intval($opts['level']) : 1,
        'stats' => $stats,
        'hp' => isset($opts['hp']) ? intval($opts['hp']) : intval($stats['max_hp']),
        'types' => $types,
        // 能力等级 -6..+6（命中/闪避等级同表；本规则版本只记录不改公式）
        'stages' => ['atk' => 0, 'def' => 0, 'spatk' => 0, 'spdef' => 0, 'speed' => 0, 'accuracy' => 0, 'evasion' => 0],
        // 异常状态占位：{'code' => 'poison'|..., 'turns_left' => int}，行为数据后续阶段入 pm_status
        'status' => null,
        // 易失状态（混乱/束缚等，只活在这一场）
        'volatile' => [],
        // buff/ debuff 叠层占位
        'buffs' => [],
        // 挂载的效果声明（钩子由 battle_core_apply_effects 派发）
        'effects' => $effects,
        'fainted' => false,
        'gender' => isset($opts['gender']) ? intval($opts['gender']) : 0,
        'is_shiny' => !empty($opts['is_shiny']),
        'capture_rate' => isset($opts['capture_rate']) ? intval($opts['capture_rate']) : 0,
        'boss_multiplier' => isset($opts['boss_multiplier']) ? floatval($opts['boss_multiplier']) : 1.0,
    ];
}

/**
 * 构造初始战斗状态
 *
 * @param array $opts {kind, map_id, rng_seed, allies: [unit..], enemies: [unit..], field}
 * @return array state
 */
function battle_core_initial_state($opts)
{
    $state = [
        'version' => BATTLE_STATE_VERSION,
        'rules_version' => BATTLE_RULES_VERSION,
        'kind' => in_array(isset($opts['kind']) ? $opts['kind'] : 'wild', ['wild', 'boss'], true) ? $opts['kind'] : 'wild',
        'map_id' => isset($opts['map_id']) ? intval($opts['map_id']) : 0,
        // battle_id 由端点持久化后回填；纯核心不关心
        'battle_id' => null,
        'uid' => isset($opts['uid']) ? intval($opts['uid']) : 0,
        'turn' => 0,
        'phase' => 'active',
        'result' => null,
        'rng_seed' => isset($opts['rng_seed']) ? intval($opts['rng_seed']) : 0,
        'rng_counter' => 0,
        'event_seq' => 0,
        // E0：天气/场地引用占位（入场效果挂载点），null = 无
        'field' => array_merge(['weather' => null, 'terrain' => null], isset($opts['field']) && is_array($opts['field']) ? $opts['field'] : []),
        'sides' => [
            'ally' => [],
            'enemy' => [],
        ],
    ];
    foreach (['ally', 'enemy'] as $side) {
        $key = $side === 'ally' ? 'allies' : 'enemies';
        $slot = 0;
        foreach ((isset($opts[$key]) && is_array($opts[$key]) ? $opts[$key] : []) as $unit) {
            $unit['slot'] = $slot++;
            $state['sides'][$side][] = battle_core_make_unit($unit);
        }
    }
    return $state;
}

/**
 * 取某侧第一个未倒下的单位（本阶段双方都只有一个有效行动位）
 *
 * @return array|null 按 PHP 数组值语义返回（修改需写回）
 */
function battle_core_active_unit($state, $side)
{
    foreach ($state['sides'][$side] as $unit) {
        if (!$unit['fainted'] && $unit['hp'] > 0) {
            return $unit;
        }
    }
    return null;
}

function battle_core_emit(&$state, &$events, $type, $payload)
{
    $state['event_seq'] = ((int)$state['event_seq']) + 1;
    $events[] = [
        'turn' => (int)$state['turn'],
        'seq' => (int)$state['event_seq'],
        'type' => $type,
        'payload' => $payload,
    ];
}

/**
 * 属性相克表（攻击属性 => {effective, resisted, immune}，中文属性名与 pm_data.xs 一致）
 * 自旧版 get_pet_type_effectiveness 移植，数据驱动：调用方可传自定义 chart 覆盖。
 */
function battle_core_type_chart()
{
    return [
        '普通' => ['effective' => [], 'resisted' => ['岩石', '钢'], 'immune' => ['幽灵']],
        '格斗' => ['effective' => ['普通', '岩石', '钢', '冰', '恶'], 'resisted' => ['飞行', '超能', '妖精'], 'immune' => ['幽灵']],
        '飞行' => ['effective' => ['格斗', '虫', '草'], 'resisted' => ['岩石', '电', '钢'], 'immune' => []],
        '毒' => ['effective' => ['草', '妖精'], 'resisted' => ['毒', '地面', '岩石', '幽灵'], 'immune' => ['钢']],
        '地面' => ['effective' => ['火', '电', '毒', '岩石', '钢'], 'resisted' => ['草', '虫'], 'immune' => ['飞行']],
        '岩石' => ['effective' => ['飞行', '虫', '火', '冰'], 'resisted' => ['格斗', '地面', '钢'], 'immune' => []],
        '虫' => ['effective' => ['草', '超能', '恶'], 'resisted' => ['飞行', '格斗', '毒', '幽灵', '钢', '火', '妖精'], 'immune' => []],
        '幽灵' => ['effective' => ['超能', '幽灵'], 'resisted' => ['恶'], 'immune' => ['普通']],
        '钢' => ['effective' => ['岩石', '冰', '妖精'], 'resisted' => ['火', '水', '电', '钢'], 'immune' => ['毒']],
        '火' => ['effective' => ['草', '冰', '虫', '钢'], 'resisted' => ['火', '水', '龙'], 'immune' => []],
        '水' => ['effective' => ['火', '地面', '岩石'], 'resisted' => ['水', '草', '龙'], 'immune' => []],
        '草' => ['effective' => ['水', '地面', '岩石'], 'resisted' => ['飞行', '草', '毒', '虫', '钢', '火', '龙'], 'immune' => []],
        '电' => ['effective' => ['水', '飞行'], 'resisted' => ['电', '草', '龙'], 'immune' => ['地面']],
        '超能' => ['effective' => ['格斗', '毒'], 'resisted' => ['超能', '钢'], 'immune' => ['恶']],
        '冰' => ['effective' => ['草', '地面', '飞行', '龙'], 'resisted' => ['火', '水', '冰', '钢'], 'immune' => []],
        '龙' => ['effective' => ['龙'], 'resisted' => ['钢'], 'immune' => ['妖精']],
        '恶' => ['effective' => ['超能', '幽灵'], 'resisted' => ['格斗', '恶', '妖精'], 'immune' => []],
        '妖精' => ['effective' => ['格斗', '龙', '恶'], 'resisted' => ['火', '毒', '钢'], 'immune' => ['龙']],
    ];
}

/**
 * 单条属性相克倍率（0 免疫 / 0.5 抵抗 / 1 正常 / 2 拔群）
 */
function battle_core_type_match($attack_type, $defend_type, $chart)
{
    if (!isset($chart[$attack_type])) {
        return 1.0;
    }
    if (in_array($defend_type, $chart[$attack_type]['immune'])) {
        return 0.0;
    }
    if (in_array($defend_type, $chart[$attack_type]['resisted'])) {
        return 0.5;
    }
    if (in_array($defend_type, $chart[$attack_type]['effective'])) {
        return 2.0;
    }
    return 1.0;
}

/**
 * 攻击属性对防守方（可双属性）的合计相克倍率
 */
function battle_core_type_effectiveness($attack_type, $defender_types, $chart = null)
{
    $chart = $chart !== null ? $chart : battle_core_type_chart();
    $total = 1.0;
    foreach ((array)$defender_types as $defend_type) {
        if ($defend_type === '' || $defend_type === null) {
            continue;
        }
        $total *= battle_core_type_match($attack_type, $defend_type, $chart);
    }
    return $total;
}

/**
 * 我方出招伤害（rules_version 1：与旧版 calculate_damage_legacy 逐项一致）
 *
 * move: {power, type, category(0 物理/1 特殊), name, id}
 * 返回 {amount, effectiveness, crit, stat}
 */
function battle_core_calc_damage(&$state, $attacker, $defender, $move, $rng = null)
{
    $level = intval($attacker['level']);
    $power = intval($move['power']);
    $category = intval($move['category']);

    // rules_version 2：攻防两端按能力等级乘数修正（经典 -6..+6 表）
    $atk_stage_mult = 1;
    $def_stage_mult = 1;
    if ((int)$state['rules_version'] >= 2) {
        $atk_stage_mult = battle_core_stage_multiplier($category !== 1 ? $attacker['stages']['atk'] : $attacker['stages']['spatk']);
        $def_stage_mult = battle_core_stage_multiplier($category !== 1 ? $defender['stages']['def'] : $defender['stages']['spdef']);
    }

    if ($category !== 1) {
        $base = (($level * 0.4 + 2) * $power * ($attacker['stats']['atk'] * $atk_stage_mult) / ($defender['stats']['def'] * $def_stage_mult) / 50 + 2);
    } else {
        $base = (($level * 0.4 + 2) * $power * ($attacker['stats']['spatk'] * $atk_stage_mult) / ($defender['stats']['spdef'] * $def_stage_mult) / 50 + 2);
    }

    // 属性相克（on_damage_calc 钩子的注入点之一：效果可修正倍率）
    $effectiveness = battle_core_type_effectiveness($move['type'], $defender['types']);
    $damage = $base * $effectiveness;

    // 属性一致加成（STAB）
    if (in_array($move['type'], $attacker['types'], true)) {
        $damage *= 1.5;
    }

    // 随机因子 85%..100%
    $damage *= battle_core_rand($state, $rng, 85, 100) / 100;

    // 会心 1/20
    $crit = (battle_core_rand($state, $rng, 1, 20) === 1);
    if ($crit) {
        $damage *= 2;
    }

    return [
        'amount' => intval(max(1, floor($damage))),
        'effectiveness' => $effectiveness,
        'crit' => $crit,
        'stat' => $category !== 1 ? 'atk' : 'spatk',
    ];
}

/**
 * 野怪反击伤害（rules_version 1：与旧版 calculate_counter_damage_legacy 一致：
 * 固定威力 40、物理线、不吃属性相克、无会心）
 */
function battle_core_calc_counter_damage(&$state, $attacker, $defender, $rng = null)
{
    $level = intval($attacker['level']);
    $power = 40;
    $atk_value = $attacker['stats']['atk'];
    $def_value = $defender['stats']['def'];
    if ((int)$state['rules_version'] >= 2) {
        $atk_value *= battle_core_stage_multiplier($attacker['stages']['atk']);
        $def_value *= battle_core_stage_multiplier($defender['stages']['def']);
    }
    $damage = (($level * 0.4 + 2) * $power * $atk_value / $def_value / 50 + 2);
    $damage *= battle_core_rand($state, $rng, 85, 100) / 100;
    return intval(max(1, floor($damage)));
}

/**
 * 有效速度（rules_version 2：能力等级乘数 + 麻痹减半；v1 为原始速度）。
 */
function battle_core_effective_speed($state, $unit)
{
    $speed = (int)$unit['stats']['speed'];
    if ((int)$state['rules_version'] < 2) {
        return $speed;
    }
    if ($unit['status'] !== null && $unit['status']['code'] === 'paralysis') {
        $speed = intval($speed / 2);
    }
    return intval($speed * battle_core_stage_multiplier($unit['stages']['speed']));
}

/**
 * 命中判定（rules_version 1：防守方速度高出 >=10 时 20% 闪避，与旧版一致；
 * rules_version 2：在此基础上按 命中/闪避等级 修正概率）
 */
function battle_core_try_miss(&$state, $rng, $attacker, $defender)
{
    if (((int)$defender['stats']['speed']) - ((int)$attacker['stats']['speed']) < 10) {
        return false;
    }
    $miss_chance = 20; // v1 基线：20% 闪避
    if ((int)$state['rules_version'] >= 2) {
        $hit_mult = battle_core_accuracy_multiplier($attacker['stages']['accuracy'], $defender['stages']['evasion']);
        // 命中率乘数 <1 提高闪避概率，>1 降低；闪避概率夹在 [5%, 95%]
        $miss_chance = max(5, min(95, intval(100 - 80 * $hit_mult)));
    }
    return battle_core_rand($state, $rng, 1, 100) <= $miss_chance;
}

/**
 * 派发一个钩子：遍历双方所有单位挂载的效果，触发声明了该钩子的效果。
 *
 * 本阶段实现 stages_boost（on_after_move）：调整出招方能力等级并发出
 * stage_change 事件。其余效果码已在 schema 校验处拒绝，不会走到这里。
 *
 * @param array &$state
 * @param array &$events
 * @param string $hook
 * @param array $ctx {actor_side, actor_slot}
 */
function battle_core_apply_effects(&$state, &$events, $hook, $ctx, $rng = null)
{
    if (!in_array($hook, battle_core_effect_hooks(), true)) {
        return;
    }
    $opponent_side = $ctx['actor_side'] === 'ally' ? 'enemy' : 'ally';
    foreach (['ally', 'enemy'] as $side) {
        foreach ($state['sides'][$side] as $i => $unit) {
            foreach ($unit['effects'] as $effect) {
                if (!in_array($hook, $effect['hooks'], true)) {
                    continue;
                }
                if ($effect['code'] === 'stages_boost') {
                    // 效果载体是出招单位；target 缺省 self 作用于自身，
                    // opponent 作用于对手当前行动位（如泼沙降命中）
                    if ($side !== $ctx['actor_side'] || $unit['slot'] !== $ctx['actor_slot']) {
                        continue;
                    }
                    $target_mode = isset($effect['params']['target']) ? $effect['params']['target'] : 'self';
                    $target_side = $target_mode === 'opponent' ? $opponent_side : $side;
                    $target_unit = battle_core_active_unit($state, $target_side);
                    if ($target_unit === null) {
                        continue;
                    }
                    $stat = $effect['params']['stat'];
                    $delta = intval($effect['params']['stages']);
                    $old = intval($target_unit['stages'][$stat]);
                    $new = max(-6, min(6, $old + $delta));
                    if ($new === $old) {
                        continue;
                    }
                    foreach ($state['sides'][$target_side] as $ti => $tu) {
                        if ($tu['slot'] === $target_unit['slot']) {
                            $state['sides'][$target_side][$ti]['stages'][$stat] = $new;
                            break;
                        }
                    }
                    battle_core_emit($state, $events, 'stage_change', [
                        'side' => $target_side,
                        'slot' => $target_unit['slot'],
                        'stat' => $stat,
                        'delta' => $new - $old,
                        'stages' => $new,
                    ]);
                } elseif ($effect['code'] === 'status_inflict') {
                    // on_hit：命中后按概率给对手挂异常（rules_version 2）
                    if ($hook !== 'on_hit' || (int)$state['rules_version'] < 2) {
                        continue;
                    }
                    if ($side !== $ctx['actor_side'] || $unit['slot'] !== $ctx['actor_slot']) {
                        continue;
                    }
                    $defender = battle_core_active_unit($state, $opponent_side);
                    if ($defender === null || $defender['status'] !== null) {
                        continue; // 已有异常不覆盖（major 单槽语义）
                    }
                    $chance = isset($effect['params']['chance']) ? intval($effect['params']['chance']) : 100;
                    if ($chance < 100 && battle_core_rand($state, $rng, 1, 100) > $chance) {
                        continue;
                    }
                    $status_code = $effect['params']['status'];
                    $turns = in_array($status_code, ['sleep', 'freeze'], true) ? battle_core_rand($state, $rng, 1, 3) : 0;
                    foreach ($state['sides'][$opponent_side] as $j => $u) {
                        if ($u['slot'] === $defender['slot']) {
                            $state['sides'][$opponent_side][$j]['status'] = ['code' => $status_code, 'turns_left' => $turns];
                            break;
                        }
                    }
                    battle_core_emit($state, $events, 'status_inflict', [
                        'side' => $opponent_side,
                        'slot' => $defender['slot'],
                        'status' => $status_code,
                        'turns' => $turns,
                    ]);
                }
            }
        }
    }
}

/**
 * 我方单位对敌方单位出一次招（move 或普通攻击）。
 *
 * 返回 true 若战斗因敌方全倒而结束（victory）。
 * PP 不在核心内管理（资源由端点预扣/退还，miss 与"未及出手"由事件标记）。
 */
function battle_core_actor_move(&$state, &$events, $rng, $side, $move)
{
    $actor_side_key = $side;
    $target_side_key = $side === 'ally' ? 'enemy' : 'ally';
    $actor = battle_core_active_unit($state, $actor_side_key);
    $target = battle_core_active_unit($state, $target_side_key);
    if ($actor === null || $target === null) {
        return false;
    }

    battle_core_emit($state, $events, 'move', [
        'side' => $actor_side_key,
        'slot' => $actor['slot'],
        'skill' => isset($move['id']) || isset($move['name']) ? [
            'id' => isset($move['id']) ? intval($move['id']) : 0,
            'name' => isset($move['name']) ? $move['name'] : '',
            'power' => intval($move['power']),
            'type' => $move['type'],
            'category' => intval($move['category']),
        ] : null,
        'target_side' => $target_side_key,
        'target_slot' => $target['slot'],
    ]);
    battle_core_apply_effects($state, $events, 'on_before_move', ['actor_side' => $actor_side_key, 'actor_slot' => $actor['slot']]);

    // 变化技（威力 0）：不造成伤害，主效果走 on_after_move 钩子（如 stages_boost）
    if (intval($move['power']) <= 0) {
        battle_core_apply_effects($state, $events, 'on_after_move', ['actor_side' => $actor_side_key, 'actor_slot' => $actor['slot']]);
        return false;
    }

    // 命中判定
    if (battle_core_try_miss($state, $rng, $actor, $target)) {
        battle_core_emit($state, $events, 'miss', [
            'side' => $actor_side_key,
            'slot' => $actor['slot'],
            'target_side' => $target_side_key,
            'target_slot' => $target['slot'],
        ]);
        return false;
    }

    $result = battle_core_calc_damage($state, $actor, $target, $move, $rng);
    $new_hp = intval(max(0, ((int)$target['hp']) - $result['amount']));

    battle_core_emit($state, $events, 'damage', [
        'side' => $actor_side_key,
        'slot' => $actor['slot'],
        'target_side' => $target_side_key,
        'target_slot' => $target['slot'],
        'amount' => $result['amount'],
        'effectiveness' => $result['effectiveness'],
        'crit' => $result['crit'],
        'stat' => $result['stat'],
    ]);

    // 写回目标 HP
    foreach ($state['sides'][$target_side_key] as $i => $u) {
        if ($u['slot'] === $target['slot']) {
            $state['sides'][$target_side_key][$i]['hp'] = $new_hp;
            break;
        }
    }
    battle_core_apply_effects($state, $events, 'on_hit', ['actor_side' => $actor_side_key, 'actor_slot' => $actor['slot']], $rng);

    if ($new_hp <= 0) {
        foreach ($state['sides'][$target_side_key] as $i => $u) {
            if ($u['slot'] === $target['slot']) {
                $state['sides'][$target_side_key][$i]['fainted'] = true;
                break;
            }
        }
        battle_core_emit($state, $events, 'faint', ['side' => $target_side_key, 'slot' => $target['slot']]);
        battle_core_apply_effects($state, $events, 'on_faint', ['actor_side' => $target_side_key, 'actor_slot' => $target['slot']]);
        if (battle_core_active_unit($state, $target_side_key) === null) {
            $state['phase'] = 'ended';
            $state['result'] = $actor_side_key === 'ally' ? 'victory' : 'defeat';
            battle_core_emit($state, $events, 'battle_end', ['result' => $state['result']]);
            return true;
        }
    }
    return false;
}

/**
 * 野怪反击（固定威力 40 的物理攻击，不吃属性相克）
 */
function battle_core_counter_attack(&$state, &$events, $rng = null)
{
    $enemy = battle_core_active_unit($state, 'enemy');
    $ally = battle_core_active_unit($state, 'ally');
    if ($enemy === null || $ally === null) {
        return;
    }

    $amount = battle_core_calc_counter_damage($state, $enemy, $ally, $rng);
    $new_hp = intval(max(0, ((int)$ally['hp']) - $amount));

    battle_core_emit($state, $events, 'counter', [
        'side' => 'enemy',
        'slot' => $enemy['slot'],
        'target_side' => 'ally',
        'target_slot' => $ally['slot'],
        'amount' => $amount,
    ]);

    foreach ($state['sides']['ally'] as $i => $u) {
        if ($u['slot'] === $ally['slot']) {
            $state['sides']['ally'][$i]['hp'] = $new_hp;
            break;
        }
    }

    if ($new_hp <= 0) {
        foreach ($state['sides']['ally'] as $i => $u) {
            if ($u['slot'] === $ally['slot']) {
                $state['sides']['ally'][$i]['fainted'] = true;
                break;
            }
        }
        battle_core_emit($state, $events, 'faint', ['side' => 'ally', 'slot' => $ally['slot']]);
        battle_core_apply_effects($state, $events, 'on_faint', ['actor_side' => 'ally', 'actor_slot' => $ally['slot']]);
        // 有替补时战斗进入等待换宠（是否真有替补由端点判库，事件仅提示语义）；
        // phase 在端点确认无替补后才落为 ended/defeat。
        $state['phase'] = 'awaiting_switch';
        battle_core_emit($state, $events, 'switch_required', ['side' => 'ally']);
    }
}

/**
 * 解析一次行动为我方出招数据（普通攻击 = struggle，威力 30、属性取自身）
 *
 * action: {type: 'move'|'struggle', skill?: {id, name, power, type, category}, fallback_type?: string}
 */
function battle_core_resolve_move($state, $action)
{
    $ally = battle_core_active_unit($state, 'ally');
    $fallback_type = isset($action['fallback_type']) && $action['fallback_type'] !== '' ? $action['fallback_type'] : ($ally ? $ally['types'][0] : '普通');
    if (!isset($action['type']) || $action['type'] !== 'move' || empty($action['skill'])) {
        return [
            'id' => 0,
            'name' => null, // null => 普通攻击（渲染层给默认名）
            'power' => 30,
            'type' => $fallback_type,
            'category' => 0,
        ];
    }
    $skill = $action['skill'];
    // 威力语义：power 键缺失/空 => 旧数据无威力，回退 40；显式传 0 => 变化技
    // （造成 0 伤害、主效果走 on_after_move 钩子）。第一阶段端点按旧语义
    // `intval($power) ?: 40` 构造 action（power=0 的技能仍按 40 威力攻击结算，
    // 数值兼容），变化技分支由 rules_version 2 的端点启用。
    if (!array_key_exists('power', $skill) || $skill['power'] === null || $skill['power'] === '') {
        $power = 40;
    } else {
        $power = intval($skill['power']);
    }
    return [
        'id' => isset($skill['id']) ? intval($skill['id']) : 0,
        'name' => isset($skill['name']) ? strval($skill['name']) : '',
        'power' => $power,
        'type' => isset($skill['type']) && $skill['type'] !== '' ? strval($skill['type']) : $fallback_type,
        'category' => isset($skill['category']) ? intval($skill['category']) : 0,
        'effects' => isset($skill['effects']) && is_array($skill['effects']) ? $skill['effects'] : [],
    ];
}

/**
 * 剥离本回合行动临时挂载到出招单位的效果（一次性 move 效果不持久化）。
 */
function battle_core_strip_mounted_effects(&$state, $mounted)
{
    if ($mounted === null) {
        return;
    }
    list($i, $base_count) = $mounted;
    if (!isset($state['sides']['ally'][$i])) {
        return;
    }
    $state['sides']['ally'][$i]['effects'] = array_slice($state['sides']['ally'][$i]['effects'], 0, $base_count);
}

/**
 * 行动前异常判定（rules_version 2）：睡眠/冰冻/麻痹可能使我方无法行动。
 * 睡眠/冰冻在此处有机会自然解除（解除的回合仍无法行动，与经典规则一致）。
 *
 * @return bool true = 本回合无法出招（PP 不退：异常跳过视为已消耗回合）
 */
function battle_core_pre_move_status(&$state, &$events, $rng, $side)
{
    if ((int)$state['rules_version'] < 2) {
        return false;
    }
    $unit = battle_core_active_unit($state, $side);
    if ($unit === null || $unit['status'] === null) {
        return false;
    }
    $catalog = battle_core_status_catalog();
    $code = $unit['status']['code'];
    if (!isset($catalog[$code])) {
        return false;
    }
    $def = $catalog[$code];
    $wake = (int)(isset($def['wake_chance']) ? $def['wake_chance'] : 0);
    if ($wake > 0 && battle_core_rand($state, $rng, 1, 100) <= $wake) {
        // 自然解除 + 本回合仍无法行动
        foreach ($state['sides'][$side] as $i => $u) {
            if ($u['slot'] === $unit['slot']) {
                $state['sides'][$side][$i]['status'] = null;
                break;
            }
        }
        battle_core_emit($state, $events, 'status_cure', ['side' => $side, 'slot' => $unit['slot'], 'status' => $code, 'natural' => true]);
        battle_core_emit($state, $events, 'status_prevent', ['side' => $side, 'slot' => $unit['slot'], 'status' => $code]);
        return true;
    }
    $skip = (int)(isset($def['skip_turn_chance']) ? $def['skip_turn_chance'] : 0);
    if ($skip <= 0) {
        return false;
    }
    if ($skip >= 100 || battle_core_rand($state, $rng, 1, 100) <= $skip) {
        battle_core_emit($state, $events, 'status_prevent', ['side' => $side, 'slot' => $unit['slot'], 'status' => $code]);
        return true;
    }
    return false;
}

/**
 * 回合末异常结算（rules_version 2）：
 * - 持续伤害（中毒 1/8、灼烧 1/16 最大 HP），伤害可致倒下并触发胜负判定；
 * - 持续回合递减与自然解除（睡眠/冰冻）。
 * 战斗已结束（ended/awaiting_switch）的回合不结算（回合被战斗结果打断）。
 */
function battle_core_resolve_statuses(&$state, &$events, $rng = null)
{
    if ((int)$state['rules_version'] < 2) {
        return;
    }
    if ($state['phase'] !== 'active') {
        return;
    }
    $catalog = battle_core_status_catalog();
    foreach (['ally', 'enemy'] as $side) {
        foreach ($state['sides'][$side] as $i => $unit) {
            if ($unit['status'] === null || $unit['fainted'] || $unit['hp'] <= 0) {
                continue;
            }
            $code = $unit['status']['code'];
            if (!isset($catalog[$code])) {
                continue;
            }
            $def = $catalog[$code];

            // 持续伤害
            $ratio = battle_core_status_damage_ratio(isset($def['turn_damage']) ? $def['turn_damage'] : null);
            if ($ratio > 0) {
                $amount = max(1, intval(floor((int)$unit['stats']['max_hp'] * $ratio)));
                $new_hp = max(0, (int)$unit['hp'] - $amount);
                $state['sides'][$side][$i]['hp'] = $new_hp;
                battle_core_emit($state, $events, 'status_damage', [
                    'side' => $side, 'slot' => $unit['slot'], 'status' => $code, 'amount' => $amount,
                ]);
                if ($new_hp <= 0) {
                    $state['sides'][$side][$i]['fainted'] = true;
                    battle_core_emit($state, $events, 'faint', ['side' => $side, 'slot' => $unit['slot']]);
                    if (battle_core_active_unit($state, $side) === null) {
                        $state['phase'] = 'ended';
                        $state['result'] = $side === 'ally' ? 'defeat' : 'victory';
                        battle_core_emit($state, $events, 'battle_end', ['result' => $state['result']]);
                    } elseif ($side === 'ally') {
                        $state['phase'] = 'awaiting_switch';
                        battle_core_emit($state, $events, 'switch_required', ['side' => 'ally']);
                    }
                    continue; // 倒下后不再解除判定
                }
            }

            // 持续回合递减与自然解除
            $turns_left = (int)$unit['status']['turns_left'];
            if ($turns_left > 0) {
                $turns_left--;
                $state['sides'][$side][$i]['status']['turns_left'] = $turns_left;
            }
            $wake = (int)(isset($def['wake_chance']) ? $def['wake_chance'] : 0);
            if ($wake > 0) {
                $min_turns = (int)(isset($def['min_turns']) ? $def['min_turns'] : 0);
                $eligible = $turns_left <= 0 || $min_turns === 0;
                if ($eligible && battle_core_rand($state, $rng, 1, 100) <= $wake) {
                    $state['sides'][$side][$i]['status'] = null;
                    battle_core_emit($state, $events, 'status_cure', [
                        'side' => $side, 'slot' => $unit['slot'], 'status' => $code, 'natural' => true,
                    ]);
                }
            }
        }
    }
}

/**
 * 回合末统一收尾：on_turn_end 钩子 -> 异常结算 -> 剥离一次性效果。
 */
function battle_core_finish_turn(&$state, &$events, $ally_slot, $mounted, $rng = null, $enemy_mounted = null)
{
    battle_core_apply_effects($state, $events, 'on_turn_end', ['actor_side' => 'ally', 'actor_slot' => $ally_slot]);
    battle_core_resolve_statuses($state, $events, $rng);
    battle_core_strip_mounted_effects($state, $mounted);
    battle_core_strip_mounted_enemy_effects($state, $enemy_mounted);
}

/**
 * 解析指定方（当前仅敌方 AI）传入的招式为标准 move（轻解析：字段补全）。
 */
function battle_core_resolve_move_for_side($state, $side, $raw)
{
    $unit = battle_core_active_unit($state, $side);
    $fallback_type = $unit ? $unit['types'][0] : '普通';
    return [
        'id' => isset($raw['id']) ? intval($raw['id']) : 0,
        'name' => isset($raw['name']) ? strval($raw['name']) : '',
        'power' => array_key_exists('power', $raw) && $raw['power'] !== null && $raw['power'] !== '' ? intval($raw['power']) : 40,
        'type' => isset($raw['type']) && $raw['type'] !== '' ? strval($raw['type']) : $fallback_type,
        'category' => isset($raw['category']) ? intval($raw['category']) : 0,
        'effects' => isset($raw['effects']) && is_array($raw['effects']) ? $raw['effects'] : [],
    ];
}

/**
 * AI 选招评分（rules_version 2）：威力 x 相克 x STAB；变化技固定低分。
 * 评分纯数据驱动，供 battle_core_ai_pick_move 排序。
 */
function battle_core_ai_score_move($attacker, $defender, $move)
{
    $power = intval($move['power']);
    if ($power <= 0) {
        return 10; // 变化技基准分（有附加效果时由调用方加分）
    }
    $effectiveness = battle_core_type_effectiveness($move['type'], $defender['types']);
    $stab = in_array($move['type'], $attacker['types'], true) ? 1.5 : 1.0;
    $category = intval($move['category']);
    $atk = $category !== 1 ? $attacker['stats']['atk'] : $attacker['stats']['spatk'];
    $def = $category !== 1 ? $defender['stats']['def'] : $defender['stats']['spdef'];
    return $power * $effectiveness * $stab * ($atk / max(1, $def));
}

/**
 * AI 选招（rules_version 2）：
 * - 难度 chance_best（0-100）：选出评分最高招的概率，其余回合在候选中
 *   按 rng 随机（低强度野怪会"打偏"，Boss 可配 100 恒定最优）；
 * - 候选为空时返回 null（调用方回退到固定反击）。
 *
 * @param array $moves [{id,name,power,type,category,effects?}..]
 * @return array|null 选中的招
 */
function battle_core_ai_pick_move(&$state, $moves, $rng = null, $chance_best = 70)
{
    if (empty($moves)) {
        return null;
    }
    $enemy = battle_core_active_unit($state, 'enemy');
    $ally = battle_core_active_unit($state, 'ally');
    if ($enemy === null || $ally === null) {
        return null;
    }
    $scored = [];
    foreach ($moves as $i => $move) {
        $scored[] = ['index' => $i, 'score' => battle_core_ai_score_move($enemy, $ally, $move)];
    }
    usort($scored, function ($a, $b) {
        if ($b['score'] === $a['score']) {
            return $a['index'] - $b['index'];
        }
        return $b['score'] > $a['score'] ? 1 : -1;
    });
    $take_best = $chance_best >= 100 || battle_core_rand($state, $rng, 1, 100) <= $chance_best;
    if ($take_best) {
        return $moves[$scored[0]['index']];
    }
    $pick = battle_core_rand($state, $rng, 0, count($scored) - 1);
    return $moves[$scored[$pick]['index']];
}

/**
 * 解析敌方本回合行动（rules_version 2）：action.enemy_move 提供时走真出招
 * （含效果挂载），否则回退固定反击（v1 行为，数值兼容）。
 *
 * @return string 'move' | 'counter'
 */
function battle_core_enemy_action_kind(&$state, $action)
{
    if ((int)$state['rules_version'] < 2 || empty($action['enemy_move'])) {
        return 'counter';
    }
    return 'move';
}

/**
 * 把敌方招式的效果临时挂到敌方出招单位（回合末剥离）。
 * @return array|null mounted 标记（同 ally 挂载）
 */
function battle_core_mount_enemy_effects(&$state, $enemy_move)
{
    $effects = isset($enemy_move['effects']) && is_array($enemy_move['effects']) ? $enemy_move['effects'] : [];
    if (empty($effects)) {
        return null;
    }
    $enemy = battle_core_active_unit($state, 'enemy');
    if ($enemy === null) {
        return null;
    }
    foreach ($state['sides']['enemy'] as $i => $u) {
        if ($u['slot'] === $enemy['slot']) {
            $mounted = [$i, count($state['sides']['enemy'][$i]['effects'])];
            foreach ($effects as $eff) {
                if (battle_core_validate_effect($eff) === true) {
                    $state['sides']['enemy'][$i]['effects'][] = $eff;
                }
            }
            return $mounted;
        }
    }
    return null;
}

function battle_core_strip_mounted_enemy_effects(&$state, $mounted)
{
    if ($mounted === null || !isset($state['sides']['enemy'][$mounted[0]])) {
        return;
    }
    $state['sides']['enemy'][$mounted[0]]['effects'] = array_slice(
        $state['sides']['enemy'][$mounted[0]]['effects'], 0, $mounted[1]
    );
}

/**
 * 执行一整回合（核心入口，纯函数）。
 *
 * 管线：turn_start -> 先手判定(速度) -> 逐单位行动(move/damage/faint 钩子)
 *     -> 反击 -> on_turn_end -> 胜负判定。
 * rules_version 1 与旧版一回合语义一致：我方一招 + 野怪固定反击一次，
 * 先手 = 我方速度 >= 敌方速度；速度差 >=10 的高速度方有 20% 闪避。
 *
 * @param array $state
 * @param array $action {'type': 'move'|'struggle', 'skill': {...}|null}
 * @param callable|null $rng 注入 RNG（callable($min,$max)）；null 走内建 seed PRNG
 * @return array {'state': array, 'events': array, 'pp_refund': bool}
 *   pp_refund：本回合我方攻击被闪避、或后手未及出手，端点应退还预扣 PP
 */
function battle_core_apply_action($state, $action, $rng = null)
{
    $events = [];
    $pp_refund = false;

    // 效果挂载：行动携带的技能效果临时挂到出招单位（本回合的 on_after_move 等用）。
    // 记录挂载位置与数量，回合结束前剥离——它们是本次出招的一次性效果，
    // 不得随 to_rows 持久化到 pm_battle_unit，否则下一回合会重复触发并叠加。
    $move = battle_core_resolve_move($state, $action);
    $move_effects = isset($move['effects']) ? $move['effects'] : [];
    $mounted = null; // [side_index, count]
    if (!empty($move_effects)) {
        $ally = battle_core_active_unit($state, 'ally');
        if ($ally !== null) {
            foreach ($state['sides']['ally'] as $i => $u) {
                if ($u['slot'] === $ally['slot']) {
                    $mounted = [$i, count($state['sides']['ally'][$i]['effects'])];
                    foreach ($move_effects as $eff) {
                        if (battle_core_validate_effect($eff) === true) {
                            $state['sides']['ally'][$i]['effects'][] = $eff;
                        }
                    }
                    break;
                }
            }
        }
    }

    $ally = battle_core_active_unit($state, 'ally');
    $enemy = battle_core_active_unit($state, 'enemy');
    $state['turn'] = ((int)$state['turn']) + 1;

    // 上回合等待换宠、本回合已有可行动单位（端点注入了换上来的宠物）：战斗继续
    if ($state['phase'] === 'awaiting_switch' && $ally !== null) {
        $state['phase'] = 'active';
    }

    battle_core_emit($state, $events, 'turn_start', ['turn' => $state['turn']]);

    if ($ally === null || $enemy === null) {
        // 不应出现（端点在 ended 战斗上不该调核心）：防御性直接结束
        $state['phase'] = 'ended';
        battle_core_emit($state, $events, 'battle_end', ['result' => $state['result'] ? $state['result'] : 'defeat']);
        battle_core_strip_mounted_effects($state, $mounted);
        return ['state' => $state, 'events' => $events, 'pp_refund' => $pp_refund];
    }

    $my_first = battle_core_effective_speed($state, $ally) >= battle_core_effective_speed($state, $enemy);

    // rules_version 2：AI 招（敌方真出招，含效果挂载）；v1 保持固定反击
    $enemy_move = null;
    $enemy_mounted = null;
    if (battle_core_enemy_action_kind($state, $action) === 'move') {
        $enemy_move = battle_core_resolve_move_for_side($state, 'enemy', $action['enemy_move']);
        $enemy_mounted = battle_core_mount_enemy_effects($state, $enemy_move);
    }

    /** 敌方行动（先手/后手路径共用） */
    $enemy_act = function () use (&$state, &$events, $rng, $enemy_move) {
        if ($enemy_move !== null) {
            battle_core_actor_move($state, $events, $rng, 'enemy', $enemy_move);
        } else {
            battle_core_counter_attack($state, $events, $rng);
        }
    };

    if ($my_first) {
        // rules_version 2：行动前异常判定（睡眠/冰冻/麻痹可能无法出招，PP 不退）
        if (battle_core_pre_move_status($state, $events, $rng, 'ally')) {
            $enemy_act();
            if ($state['phase'] === 'awaiting_switch') {
                battle_core_finish_turn($state, $events, $ally['slot'], $mounted, $rng, $enemy_mounted);
                return ['state' => $state, 'events' => $events, 'pp_refund' => $pp_refund];
            }
            battle_core_finish_turn($state, $events, $ally['slot'], $mounted, $rng, $enemy_mounted);
            return ['state' => $state, 'events' => $events, 'pp_refund' => $pp_refund];
        }
        if (battle_core_pre_move_status($state, $events, $rng, 'ally')) {
            battle_core_finish_turn($state, $events, $ally['slot'], $mounted, $rng, $enemy_mounted);
            return ['state' => $state, 'events' => $events, 'pp_refund' => $pp_refund];
        }
        battle_core_actor_move($state, $events, $rng, 'ally', $move);
        if ($state['phase'] === 'ended') {
            // 先手击倒：回合结束（旧版此时野怪不再反击）
            battle_core_finish_turn($state, $events, $ally['slot'], $mounted, $rng, $enemy_mounted);
            return ['state' => $state, 'events' => $events, 'pp_refund' => $pp_refund];
        }
        if (battle_core_events_has($events, 'miss')) {
            $pp_refund = true;
        }
        $enemy_act();
        if ($state['phase'] === 'awaiting_switch') {
            // 我方倒下待换宠：敌方行动已发生，回合结算交还给端点
            battle_core_finish_turn($state, $events, $ally['slot'], $mounted, $rng, $enemy_mounted);
            return ['state' => $state, 'events' => $events, 'pp_refund' => $pp_refund];
        }
    } else {
        $enemy_act();
        if ($state['phase'] === 'awaiting_switch') {
            // 后手被击倒、未及出手：按旧版语义退还 PP
            $pp_refund = true;
            battle_core_finish_turn($state, $events, $ally['slot'], $mounted, $rng, $enemy_mounted);
            return ['state' => $state, 'events' => $events, 'pp_refund' => $pp_refund];
        }
        if (battle_core_pre_move_status($state, $events, $rng, 'ally')) {
            battle_core_finish_turn($state, $events, $ally['slot'], $mounted, $rng, $enemy_mounted);
            return ['state' => $state, 'events' => $events, 'pp_refund' => $pp_refund];
        }
        battle_core_actor_move($state, $events, $rng, 'ally', $move);
        if ($state['phase'] === 'ended') {
            battle_core_finish_turn($state, $events, $ally['slot'], $mounted, $rng, $enemy_mounted);
            return ['state' => $state, 'events' => $events, 'pp_refund' => $pp_refund];
        }
        if (battle_core_events_has($events, 'miss')) {
            $pp_refund = true;
        }
    }

    battle_core_finish_turn($state, $events, $ally['slot'], $mounted, $rng, $enemy_mounted);
    return ['state' => $state, 'events' => $events, 'pp_refund' => $pp_refund];
}

function battle_core_events_has($events, $type)
{
    foreach ($events as $e) {
        if ($e['type'] === $type) {
            return true;
        }
    }
    return false;
}

/**
 * 逃跑判定（rules_version 1 与旧版 api_flee 一致）：
 * 基础 0.5 + (我方等级 - 敌方等级) * 0.05，夹在 [0.1, 0.9]
 *
 * @return array {'state', 'events', 'fled': bool}
 */
function battle_core_try_flee($state, $rng = null)
{
    $events = [];
    $ally = battle_core_active_unit($state, 'ally');
    $enemy = battle_core_active_unit($state, 'enemy');
    if ($ally === null || $enemy === null) {
        return ['state' => $state, 'events' => $events, 'fled' => false];
    }
    $chance = 0.5 + (((int)$ally['level']) - ((int)$enemy['level'])) * 0.05;
    $chance = max(0.1, min(0.9, $chance));
    $roll = battle_core_rand($state, $rng, 1, 100) / 100;
    if ($roll <= $chance) {
        $state['phase'] = 'ended';
        $state['result'] = 'fled';
        battle_core_emit($state, $events, 'battle_end', ['result' => 'fled']);
        return ['state' => $state, 'events' => $events, 'fled' => true];
    }
    battle_core_emit($state, $events, 'message', ['text' => 'flee_failed']);
    return ['state' => $state, 'events' => $events, 'fled' => false];
}

/**
 * 战斗文案缺省语言包（简体中文，与旧版 damage_log 逐条对应）。
 * 端点可传入站点语言包覆盖（battle_lang()），模板支持 {ally}/{enemy}/
 * {skill}/{amount}/{stat}/{status}/{who} 占位符。
 */
function battle_core_default_lang()
{
    return [
        'used_move' => '{ally}使用了{skill}，对{enemy}造成了{amount}点伤害！',
        'enemy_used_move' => '{enemy}使用了{skill}，对{ally}造成了{amount}点伤害！',
        'counter' => '{enemy}攻击了{ally}，造成了{amount}点伤害！',
        'miss' => '{enemy}避开了{ally}的攻击！',
        'faint_enemy' => '{enemy}倒下了！',
        'faint_ally' => '{ally}倒下了...',
        'stage_up' => '{who}的{stat}提高了！',
        'stage_down' => '{who}的{stat}降低了！',
        'status_inflict' => '{who}陷入了{status}状态！',
        'status_damage' => '{who}受到了{status}的伤害，{amount}点！',
        'status_prevent' => '{who}因{status}无法行动！',
        'status_cure' => '{who}的{status}治好了！',
        'switch_required' => '还有可用的替补宠物，请更换宠物继续战斗！',
        'struggle' => '普通攻击',
        'stat_names' => [
            'atk' => '攻击', 'def' => '防御', 'spatk' => '特攻', 'spdef' => '特防',
            'speed' => '速度', 'accuracy' => '命中', 'evasion' => '闪避',
        ],
        'status_names' => [
            'poison' => '中毒', 'burn' => '灼烧', 'paralysis' => '麻痹',
            'sleep' => '睡眠', 'freeze' => '冰冻', 'confusion' => '混乱',
        ],
    ];
}

function battle_core_render_template($template, $vars)
{
    foreach (['ally', 'enemy', 'skill', 'amount', 'stat', 'status', 'who'] as $key) {
        $template = str_replace('{' . $key . '}', isset($vars[$key]) ? strval($vars[$key]) : '', $template);
    }
    return $template;
}

/**
 * 把事件流渲染为战斗文案（i18n：文案模板来自 $lang，缺省为内置简体中文）。
 *
 * move 与 damage/miss 事件成对出现（出招声明 + 结算），渲染时合并为
 * 旧版单条文案"X使用了Y，对Z造成了N点伤害！"。
 *
 * @param array $events
 * @param array $names {'ally': 显示名, 'enemy': 种族名}
 * @return array string[]
 */
function battle_core_render_messages($events, $names, $lang = null)
{
    $lang = is_array($lang) ? array_merge(battle_core_default_lang(), $lang) : battle_core_default_lang();
    $ally_name = isset($names['ally']) ? $names['ally'] : '我方';
    $enemy_name = isset($names['enemy']) ? $names['enemy'] : '野怪';
    $out = [];
    $pending_skill_name = null;
    foreach ($events as $e) {
        $p = $e['payload'];
        $who = null;
        if (isset($p['side'])) {
            $who = ($p['side'] === 'ally') ? $ally_name : $enemy_name;
        }
        switch ($e['type']) {
            case 'move':
                // 出招声明：记录技能名，与后续 damage/miss 合并成一条文案
                if (isset($p['skill']) && ($p['skill'] !== null)) {
                    $pending_skill_name = ($p['skill']['name'] !== null && $p['skill']['name'] !== '')
                        ? $p['skill']['name'] : $lang['struggle'];
                }
                break;
            case 'damage': {
                $amount = isset($p['amount']) ? (int)$p['amount'] : 0;
                if ($p['side'] === 'ally') {
                    $skill_name = $pending_skill_name !== null ? $pending_skill_name : $lang['struggle'];
                    $out[] = battle_core_render_template($lang['used_move'], ['ally' => $ally_name, 'enemy' => $enemy_name, 'skill' => $skill_name, 'amount' => $amount]);
                } elseif ($pending_skill_name !== null) {
                    $out[] = battle_core_render_template($lang['enemy_used_move'], ['ally' => $ally_name, 'enemy' => $enemy_name, 'skill' => $pending_skill_name, 'amount' => $amount]);
                } else {
                    $out[] = battle_core_render_template($lang['counter'], ['ally' => $ally_name, 'enemy' => $enemy_name, 'amount' => $amount]);
                }
                $pending_skill_name = null;
                break;
            }
            case 'miss':
                $out[] = battle_core_render_template($lang['miss'], ['ally' => $ally_name, 'enemy' => $enemy_name]);
                $pending_skill_name = null;
                break;
            case 'counter':
                $out[] = battle_core_render_template($lang['counter'], ['ally' => $ally_name, 'enemy' => $enemy_name, 'amount' => isset($p['amount']) ? (int)$p['amount'] : 0]);
                break;
            case 'faint':
                $out[] = battle_core_render_template($p['side'] === 'enemy' ? $lang['faint_enemy'] : $lang['faint_ally'], ['ally' => $ally_name, 'enemy' => $enemy_name]);
                break;
            case 'stage_change': {
                $stat = isset($lang['stat_names'][$p['stat']]) ? $lang['stat_names'][$p['stat']] : $p['stat'];
                $tpl = $p['delta'] > 0 ? $lang['stage_up'] : $lang['stage_down'];
                $out[] = battle_core_render_template($tpl, ['who' => $who, 'stat' => $stat]);
                break;
            }
            case 'status_inflict': {
                $status = isset($lang['status_names'][$p['status']]) ? $lang['status_names'][$p['status']] : $p['status'];
                $out[] = battle_core_render_template($lang['status_inflict'], ['who' => $who, 'status' => $status]);
                break;
            }
            case 'status_damage': {
                $status = isset($lang['status_names'][$p['status']]) ? $lang['status_names'][$p['status']] : $p['status'];
                $out[] = battle_core_render_template($lang['status_damage'], ['who' => $who, 'status' => $status, 'amount' => isset($p['amount']) ? (int)$p['amount'] : 0]);
                break;
            }
            case 'status_prevent': {
                $status = isset($lang['status_names'][$p['status']]) ? $lang['status_names'][$p['status']] : $p['status'];
                $out[] = battle_core_render_template($lang['status_prevent'], ['who' => $who, 'status' => $status]);
                break;
            }
            case 'status_cure': {
                $status = isset($lang['status_names'][$p['status']]) ? $lang['status_names'][$p['status']] : $p['status'];
                $out[] = battle_core_render_template($lang['status_cure'], ['who' => $who, 'status' => $status]);
                break;
            }
            case 'switch_required':
                $out[] = $lang['switch_required'];
                break;
        }
    }
    return $out;
}


/* ----------------------------------------------------------------------
 * 持久化编解码（纯函数）：state <-> pm_battle + pm_battle_unit 行
 * -------------------------------------------------------------------- */

/**
 * state -> 表行。返回 {'battle': {字段=>值}, 'units': [行..]}。
 * units 的 JSON 列用 json_encode；无 JSON 扩展时 json_encode 不可用的环境
 * 由端点层保证（X5 运行环境均带 json）。
 */
function battle_state_to_rows($state)
{
    $battle = [
        'uid' => intval($state['uid']),
        'kind' => $state['kind'],
        'map_id' => intval($state['map_id']),
        'turn' => intval($state['turn']),
        'phase' => $state['phase'],
        'result' => $state['result'] === null ? '' : $state['result'],
        'rng_seed' => intval($state['rng_seed']),
        'rng_counter' => intval($state['rng_counter']),
        'event_seq' => intval($state['event_seq']),
        'rules_version' => intval($state['rules_version']),
        'field_json' => json_encode($state['field']),
        'state_version' => intval($state['version']),
    ];

    $units = [];
    foreach (['ally', 'enemy'] as $side) {
        foreach ($state['sides'][$side] as $unit) {
            $units[] = [
                'battle_id' => isset($state['battle_id']) ? intval($state['battle_id']) : 0,
                'side' => $side,
                'slot' => intval($unit['slot']),
                'instance_id' => intval($unit['instance_id']),
                'species_id' => intval($unit['species_id']),
                'name' => $unit['name'],
                'species_name' => $unit['species_name'],
                'level' => intval($unit['level']),
                'stats_json' => json_encode($unit['stats']),
                'types_json' => json_encode($unit['types']),
                'hp' => intval($unit['hp']),
                'stages_json' => json_encode($unit['stages']),
                'status_json' => json_encode($unit['status']),
                'volatile_json' => json_encode($unit['volatile']),
                'buffs_json' => json_encode($unit['buffs']),
                'effects_json' => json_encode($unit['effects']),
                'fainted' => $unit['fainted'] ? 1 : 0,
                'gender' => intval($unit['gender']),
                'is_shiny' => $unit['is_shiny'] ? 1 : 0,
                'capture_rate' => intval($unit['capture_rate']),
                'boss_multiplier' => floatval($unit['boss_multiplier']),
            ];
        }
    }
    return ['battle' => $battle, 'units' => $units];
}

/**
 * 表行 -> state（battle_state_to_rows 的逆变换，往返无损）。
 *
 * @param array $battle_row pm_battle 行（含 id/uid）
 * @param array $unit_rows pm_battle_unit 行（属本战斗）
 * @return array state
 */
function battle_state_from_rows($battle_row, $unit_rows)
{
    $field = json_decode(isset($battle_row['field_json']) ? $battle_row['field_json'] : '{}', true);
    $state = [
        'version' => intval($battle_row['state_version']),
        'rules_version' => intval($battle_row['rules_version']),
        'kind' => strval($battle_row['kind']),
        'map_id' => intval($battle_row['map_id']),
        'battle_id' => intval($battle_row['id']),
        'uid' => intval($battle_row['uid']),
        'turn' => intval($battle_row['turn']),
        'phase' => strval($battle_row['phase']),
        'result' => $battle_row['result'] !== '' && $battle_row['result'] !== null ? strval($battle_row['result']) : null,
        'rng_seed' => intval($battle_row['rng_seed']),
        'rng_counter' => intval($battle_row['rng_counter']),
        'event_seq' => intval($battle_row['event_seq']),
        'field' => is_array($field) ? array_merge(['weather' => null, 'terrain' => null], $field) : ['weather' => null, 'terrain' => null],
        'sides' => ['ally' => [], 'enemy' => []],
    ];

    foreach ($unit_rows as $row) {
        $stats = json_decode($row['stats_json'], true);
        $types = json_decode($row['types_json'], true);
        $stages = json_decode($row['stages_json'], true);
        $status = json_decode($row['status_json'], true);
        $volatile = json_decode($row['volatile_json'], true);
        $buffs = json_decode($row['buffs_json'], true);
        $effects = json_decode($row['effects_json'], true);
        $state['sides'][$row['side']][] = [
            'slot' => intval($row['slot']),
            'instance_id' => intval($row['instance_id']),
            'species_id' => intval($row['species_id']),
            'species_name' => strval($row['species_name']),
            'name' => strval($row['name']),
            'level' => intval($row['level']),
            'stats' => is_array($stats) ? $stats : [],
            'types' => is_array($types) ? $types : [],
            'hp' => intval($row['hp']),
            'stages' => is_array($stages) ? $stages : [],
            'status' => is_array($status) ? $status : null,
            'volatile' => is_array($volatile) ? $volatile : [],
            'buffs' => is_array($buffs) ? $buffs : [],
            'effects' => is_array($effects) ? array_values(array_filter($effects, 'battle_core_is_valid_effect')) : [],
            'fainted' => !empty($row['fainted']),
            'gender' => intval($row['gender']),
            'is_shiny' => !empty($row['is_shiny']),
            'capture_rate' => intval($row['capture_rate']),
            'boss_multiplier' => floatval($row['boss_multiplier']),
        ];
    }
    foreach (['ally', 'enemy'] as $side) {
        usort($state['sides'][$side], function ($a, $b) {
            return $a['slot'] - $b['slot'];
        });
    }
    return $state;
}

function battle_core_is_valid_effect($effect)
{
    return is_array($effect) && battle_core_validate_effect($effect) === true;
}

/**
 * 旧列战斗状态（pm_usersdata.npcid 系列列）-> 新引擎 state（惰性迁移组装）。
 *
 * 升级部署瞬间进行中的老战斗在下一次回合接口触碰时升级到新表：
 * 野怪六维快照取旧列现值，我方单位由端点每回合现算注入（stats 快照随回合刷新，
 * 与旧版"每回合重算我方属性"的行为一致）。
 *
 * @param array $legacy pm_usersdata 行（npcid/level/hp/hpg/atkg/defg/spatkg/spdefg/sdg/allure/capture）
 * @param array $ally_unit_opts 我方单位构造参数（stats 由端点现算后传入）
 * @param array $enemy_meta {species_name, types, is_boss, boss_multiplier, map_id, rng_seed}
 * @return array state
 */
function battle_core_state_from_legacy($legacy, $ally_unit_opts, $enemy_meta)
{
    $allure = intval($legacy['allure']);
    $state = battle_core_initial_state([
        'kind' => !empty($enemy_meta['is_boss']) ? 'boss' : 'wild',
        'map_id' => isset($enemy_meta['map_id']) ? intval($enemy_meta['map_id']) : 0,
        'rng_seed' => isset($enemy_meta['rng_seed']) ? intval($enemy_meta['rng_seed']) : 0,
        'allies' => [$ally_unit_opts],
        'enemies' => [[
            'slot' => 0,
            'instance_id' => 0,
            'species_id' => intval($legacy['npcid']),
            'species_name' => isset($enemy_meta['species_name']) ? $enemy_meta['species_name'] : '',
            'name' => isset($enemy_meta['species_name']) ? $enemy_meta['species_name'] : '',
            'level' => intval($legacy['level']),
            'hp' => intval($legacy['hp']),
            'stats' => [
                'max_hp' => intval($legacy['hpg']),
                'atk' => intval($legacy['atkg']),
                'def' => intval($legacy['defg']),
                'spatk' => intval($legacy['spatkg']),
                'spdef' => intval($legacy['spdefg']),
                'speed' => intval($legacy['sdg']),
            ],
            'types' => isset($enemy_meta['types']) && is_array($enemy_meta['types']) ? $enemy_meta['types'] : [],
            'gender' => ($allure >> 1) & 1,
            'is_shiny' => ($allure & 1) === 1,
            'capture_rate' => intval($legacy['capture']),
            'boss_multiplier' => isset($enemy_meta['boss_multiplier']) ? floatval($enemy_meta['boss_multiplier']) : 1.0,
        ]],
    ]);
    return $state;
}
