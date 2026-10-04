<?php

/**
 * 战斗系统 API - 兼容旧版存储方式
 *
 * 使用 pm_usersdata 表存储战斗状态，无需创建新表
 *
 * 端点:
 * - POST ?action=start         开始战斗
 * - POST ?action=turn          使用技能攻击
 * - POST ?action=flee          逃跑
 * - POST ?action=switch_pokemon 切换上场宠物（主动切换，有概率被反击）
 * - POST ?action=replace_pokemon 替换上场宠物（被动切换，不反击）
 * - POST ?action=capture       捕捉精灵
 * - POST ?action=use_item      使用物品
 * - POST ?action=use_item_on_skill 对技能使用物品
 * - GET  ?action=get_battle_items 获取战斗可用物品
 * - GET  ?action=maps          获取地图列表
 * - GET  ?action=recover       恢复战斗状态
 * - GET  ?action=battle_log   查询战报（事件流 + 可分享 BBCode）
 */

// 加载 API 辅助函数（包含 get_param, api_error 等）
require_once __DIR__ . '/index.php';

// 加载常量定义
require_once __DIR__ . '/constants.php';

global $_G;

// 初始化全局配置
$settings = isset($_G['cache']['plugin']['pokemon']) ? $_G['cache']['plugin']['pokemon'] : array();

$action = get_param('action', '');

// battle_log 只依赖战斗核心与持久化帮助函数
if ($action === 'battle_log') {
    require_once __DIR__ . '/index.php';
    require_once __DIR__ . '/battle_core.php';
    battle_api_get_battle_log();
    exit;
}

// maps 接口不需要加载额外的依赖
if ($action === 'maps') {
    api_get_maps();
    exit;
}

// 以下接口需要加载战斗相关依赖
// 加载 API 工具函数（包含经验计算、状态修正等）
require_once __DIR__ . '/utils.php';

// 加载战斗引擎 2.0 纯函数核心（issue #75 第一阶段）
require_once __DIR__ . '/battle_core.php';

// 加载 Boss 系统 API 函数
require_once __DIR__ . '/boss.php';

// recover 接口用于恢复战斗状态（需要依赖文件）
if ($action === 'recover') {
    api_recover_battle();
    exit;
}

// capture 接口用于捕捉精灵
if ($action === 'capture') {
    api_capture_pokemon();
    exit;
}

// maps 接口用于获取地图列表
if ($action === 'maps') {
    api_get_maps();
    exit;
}

// use_item 接口用于在战斗中使用物品
if ($action === 'use_item') {
    api_use_item_in_battle();
    exit;
}

// use_item_on_skill 接口用于在战斗中对指定技能使用物品（如PP恢复）
if ($action === 'use_item_on_skill') {
    api_use_item_on_skill_in_battle();
    exit;
}

// get_battle_items 接口用于获取可以在战斗中使用的物品
if ($action === 'get_battle_items') {
    api_get_battle_items();
    exit;
}

// switch_pokemon 接口用于切换上场宠物
if ($action === 'switch_pokemon') {
    api_switch_pokemon();
    exit;
}

// replace_pokemon 接口用于被动切换（宠物被打死后替换，不反击）
if ($action === 'replace_pokemon') {
    api_replace_pokemon();
    exit;
}

/**
 * 获取宠物基础种族数据
 *
 * @param int $id 宠物图鉴编号 (pm_data.id / pmno)
 * @return array|false 基础数据关联数组
 */
function pm_data($id)
{
    $id = intval($id);
    if ($id <= 0) {
        return false;
    }
    return DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_data') . " WHERE id = %d", $id));
}

/* ======================================================================
 * 战斗引擎 2.0 持久化层（issue #75 第一阶段）
 *
 * 权威状态在 pm_battle / pm_battle_unit / pm_battle_event；pm_usersdata
 * 旧列（npcid/level/hp/...）作为镜像投影继续写入：尚未迁移到新引擎的
 * 端点（capture / use_item / switch / replace）与 build_battle_response
 * 都读旧列，镜像保证它们行为不变。后续阶段逐端点迁移后再废弃旧列。
 * ==================================================================== */

/**
 * 惰性建表（老站点升级路径，幂等）。
 * DDL 与 docker/init.d/02-pokemon-schema.sql、plugin/install.php 保持一致。
 */
function battle_ensure_tables()
{
    static $done = false;
    if ($done) {
        return;
    }
    $done = true;
    DB::query("CREATE TABLE IF NOT EXISTS " . pm_table('pm_battle') . " (
        `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
        `uid` mediumint(8) unsigned NOT NULL,
        `kind` varchar(10) NOT NULL DEFAULT 'wild',
        `map_id` int(10) unsigned NOT NULL DEFAULT 0,
        `turn` int(10) unsigned NOT NULL DEFAULT 0,
        `phase` varchar(20) NOT NULL DEFAULT 'active',
        `result` varchar(10) NOT NULL DEFAULT '',
        `rng_seed` bigint(20) NOT NULL DEFAULT 0,
        `rng_counter` int(10) unsigned NOT NULL DEFAULT 0,
        `event_seq` int(10) unsigned NOT NULL DEFAULT 0,
        `rules_version` int(10) unsigned NOT NULL DEFAULT 1,
        `state_version` int(10) unsigned NOT NULL DEFAULT 2,
        `field_json` text NOT NULL,
        `created_at` int(10) unsigned NOT NULL DEFAULT 0,
        `updated_at` int(10) unsigned NOT NULL DEFAULT 0,
        PRIMARY KEY (`id`),
        KEY `idx_uid` (`uid`),
        KEY `idx_uid_phase` (`uid`, `phase`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci");
    DB::query("CREATE TABLE IF NOT EXISTS " . pm_table('pm_battle_unit') . " (
        `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
        `battle_id` bigint(20) unsigned NOT NULL,
        `side` varchar(5) NOT NULL DEFAULT 'ally',
        `slot` tinyint(3) unsigned NOT NULL DEFAULT 0,
        `instance_id` int(10) unsigned NOT NULL DEFAULT 0,
        `species_id` mediumint(8) unsigned NOT NULL DEFAULT 0,
        `name` varchar(60) NOT NULL DEFAULT '',
        `species_name` varchar(60) NOT NULL DEFAULT '',
        `level` smallint(5) unsigned NOT NULL DEFAULT 1,
        `stats_json` text NOT NULL,
        `types_json` text NOT NULL,
        `hp` int(10) NOT NULL DEFAULT 0,
        `stages_json` text NOT NULL,
        `status_json` text NOT NULL,
        `volatile_json` text NOT NULL,
        `buffs_json` text NOT NULL,
        `effects_json` text NOT NULL,
        `fainted` tinyint(1) NOT NULL DEFAULT 0,
        `gender` tinyint(1) NOT NULL DEFAULT 0,
        `is_shiny` tinyint(1) NOT NULL DEFAULT 0,
        `capture_rate` smallint(5) unsigned NOT NULL DEFAULT 0,
        `boss_multiplier` float NOT NULL DEFAULT 1,
        PRIMARY KEY (`id`),
        KEY `idx_battle` (`battle_id`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci");
    DB::query("CREATE TABLE IF NOT EXISTS " . pm_table('pm_effect') . " (
        `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
        `code` varchar(40) NOT NULL,
        `kind` varchar(10) NOT NULL DEFAULT 'move',
        `hooks_json` text NOT NULL,
        `params_json` text NOT NULL,
        `description` varchar(255) NOT NULL DEFAULT '',
        `version` int(10) unsigned NOT NULL DEFAULT 1,
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_code` (`code`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci");
    // 旧库 pm_skill 无 effect_id 列时惰性补列（幂等）
    $skill_col = DB::fetch_first("SHOW COLUMNS FROM " . pm_table('pm_skill') . " LIKE 'effect_id'");
    if (!$skill_col) {
        DB::query("ALTER TABLE " . pm_table('pm_skill') . " ADD COLUMN effect_id int(10) unsigned NOT NULL DEFAULT 0 AFTER element");
    }
    DB::query("CREATE TABLE IF NOT EXISTS " . pm_table('pm_status') . " (
        `code` varchar(20) NOT NULL,
        `name` varchar(30) NOT NULL DEFAULT '',
        `behavior_json` text NOT NULL,
        `overlap` varchar(10) NOT NULL DEFAULT 'replace',
        `version` int(10) unsigned NOT NULL DEFAULT 1,
        PRIMARY KEY (`code`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci");
    DB::query("CREATE TABLE IF NOT EXISTS " . pm_table('pm_battle_event') . " (
        `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
        `battle_id` bigint(20) unsigned NOT NULL,
        `turn` int(10) unsigned NOT NULL DEFAULT 0,
        `seq` int(10) unsigned NOT NULL DEFAULT 0,
        `type` varchar(30) NOT NULL DEFAULT '',
        `payload_json` text NOT NULL,
        `schema_version` smallint(5) unsigned NOT NULL DEFAULT 1,
        `created_at` int(10) unsigned NOT NULL DEFAULT 0,
        PRIMARY KEY (`id`),
        KEY `idx_battle_turn` (`battle_id`, `turn`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci");
}

/**
 * 载入用户进行中的战斗（新表优先）。
 *
 * 升级部署瞬间进行中的老战斗（pm_usersdata.npcid>0、新表无行）在此惰性
 * 迁移：旧列快照升级为引擎状态并落库，之后按新表走，客户端无感知。
 *
 * @return array|null 引擎状态；无进行中战斗返回 null
 */
function battle_load_active($uid, $myusersdata, $mypokemon)
{
    // 生命周期：超过 24h 无更新的进行中战斗视为超时放弃（惰性清理 + 镜像清零）
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_battle') . "
        SET phase = 'ended', result = 'abandoned', updated_at = %d
        WHERE uid = %d AND phase IN ('active', 'awaiting_switch')
          AND updated_at > 0 AND updated_at < %d",
        time(), $uid, time() - 86400
    ));

    $row = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_battle') . "
        WHERE uid = %d AND phase IN ('active', 'awaiting_switch')
        ORDER BY id DESC LIMIT 1",
        $uid
    ));
    if ($row) {
        $units = DB::fetch_all(pm_sql(
            "SELECT * FROM " . pm_table('pm_battle_unit') . " WHERE battle_id = %d",
            $row['id']
        ));
        return battle_state_from_rows($row, $units);
    }

    // 惰性迁移：旧列里有进行中的战斗
    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        return null;
    }
    $npc = pm_data($myusersdata['npcid']);
    if (!$npc) {
        return null;
    }

    $mydata = pm_data($mypokemon['species_id']);
    if (!$mydata) {
        return null;
    }
    list($mpmhp, $matk, $mdef, $mspatk, $mspdef, $msd) = battle_calc_my_stats($mydata, $mypokemon);

    $state = battle_core_state_from_legacy(
        $myusersdata,
        [
            'instance_id' => intval($mypokemon['id']),
            'species_id' => intval($mypokemon['species_id']),
            'name' => $mypokemon['nickname'] ?: $mypokemon['pmname'],
            'species_name' => $mypokemon['pmname'],
            'level' => intval($mypokemon['level']),
            'hp' => intval($mypokemon['hp']),
            'stats' => [
                'max_hp' => $mpmhp, 'atk' => $matk, 'def' => $mdef,
                'spatk' => $mspatk, 'spdef' => $mspdef, 'speed' => $msd,
            ],
            'types' => [$mydata['xs'], $mydata['xs2']],
        ],
        [
            'species_name' => $npc['name'],
            'types' => [$npc['xs'], $npc['xs2']],
            // 旧列未存地图（start 未落库 map_id），迁移后 map_id=0 与 recover 行为一致
            'map_id' => 0,
            'rng_seed' => mt_rand(1, 2147483647),
            'is_boss' => false,
            'boss_multiplier' => 1.0,
        ]
    );
    $state['uid'] = intval($uid);

    $events = [];
    battle_core_emit($state, $events, 'battle_start', ['kind' => $state['kind'], 'map_id' => 0, 'migrated' => true]);
    return battle_persist_state($state, $events);
}

/**
 * 每回合把我方最新状态注入引擎单位（HP/等级/六维）。
 *
 * 与旧版"每回合 battle_calc_my_stats 现算"的行为一致：战斗中治疗、升级、
 * 换装都会在下一回合生效；野怪六维是开战快照（旧版同样直接读旧列）。
 */
function battle_inject_ally_fresh_state(&$state, $mypokemon, $mydata)
{
    list($mpmhp, $matk, $mdef, $mspatk, $mspdef, $msd) = battle_calc_my_stats($mydata, $mypokemon);
    $fresh = [
        'level' => intval($mypokemon['level']),
        'hp' => intval($mypokemon['hp']),
        'stats' => [
            'max_hp' => $mpmhp, 'atk' => $matk, 'def' => $mdef,
            'spatk' => $mspatk, 'spdef' => $mspdef, 'speed' => $msd,
        ],
    ];
    foreach ($state['sides']['ally'] as $i => $unit) {
        if ($unit['instance_id'] !== intval($mypokemon['id'])) {
            continue;
        }
        $state['sides']['ally'][$i] = array_merge($unit, $fresh);
        $state['sides']['ally'][$i]['fainted'] = ((int)$mypokemon['hp'] <= 0);
        return;
    }
    // 上场宠物不在战斗单位里（主动/被动换宠后 instance_id 变了）：
    // 优先替换倒下位（被动替换），否则替换当前行动位（主动切换，旧宠物可能仍存活）。
    // 若不替换，回合会按旧宠物快照结算、并把旧宠物的反击后 HP 写进新宠物的 pm_mypm 行。
    $replace_index = null;
    foreach ($state['sides']['ally'] as $i => $unit) {
        if ($unit['fainted']) {
            $replace_index = $i;
            break;
        }
    }
    if ($replace_index === null) {
        foreach ($state['sides']['ally'] as $i => $unit) {
            if (!$unit['fainted']) {
                $replace_index = $i;
                break;
            }
        }
    }
    if ($replace_index !== null) {
        $state['sides']['ally'][$replace_index] = battle_core_make_unit([
            'slot' => $state['sides']['ally'][$replace_index]['slot'],
            'instance_id' => intval($mypokemon['id']),
            'species_id' => intval($mypokemon['species_id']),
            'name' => $mypokemon['nickname'] ?: $mypokemon['pmname'],
            'species_name' => $mypokemon['pmname'],
            'level' => intval($mypokemon['level']),
            'hp' => intval($mypokemon['hp']),
            'stats' => $fresh['stats'],
            'types' => [$mydata['xs'], $mydata['xs2']],
        ]);
    }
}

/**
 * 敌方 AI 选招（rules_version 2）：按 pm_skill.available_pokemons 匹配野怪
 * 种族（FIND_IN_SET 容忍 k 哨兵），level_required <= 野怪等级，取前 4 招
 * 评分选择。Boss 恒定最优，普通野怪 70% 最优（难度分级）。
 * v1 战斗或无可用技能返回 null（核心回退固定反击，数值兼容）。
 */
function battle_pick_enemy_move(&$state)
{
    if ((int)$state['rules_version'] < 2) {
        return null;
    }
    $enemy = battle_core_active_unit($state, 'enemy');
    if ($enemy === null) {
        return null;
    }
    static $cache = [];
    $species_id = (int)$enemy['species_id'];
    $level = (int)$enemy['level'];
    $key = $species_id . ':' . $level;
    if (!isset($cache[$key])) {
        $rows = DB::fetch_all(pm_sql(
            "SELECT s.id, s.name, s.power, s.element, s.category, s.effect_id
             FROM " . pm_table('pm_skill') . " s
             WHERE FIND_IN_SET(%d, REPLACE(s.available_pokemons, '|', ',')) > 0
               AND s.level_required <= %d
             ORDER BY s.id LIMIT 4",
            $species_id, $level
        ));
        $moves = [];
        foreach ((array)$rows as $row) {
            $moves[] = [
                'id' => (int)$row['id'],
                'name' => $row['name'],
                'power' => (int)$row['power'],
                'type' => $row['element'] ?: '',
                'category' => api_normalize_skill_category($row['category']),
                'effects' => battle_skill_effects($row),
            ];
        }
        $cache[$key] = $moves;
    }
    if (empty($cache[$key])) {
        return null;
    }
    $chance = ($state['kind'] === 'boss') ? 100 : 70;
    return battle_core_ai_pick_move($state, $cache[$key], null, $chance);
}

/**
 * 技能效果装载：pm_skill.effect_id -> pm_effect 行 -> 核心效果声明。
 * params_json 内嵌核心效果 code 与参数；声明经 battle_core_validate_effect
 * 严格校验，未知/坏数据直接丢弃（返回空），不进入战斗。
 * 按请求静态缓存。
 *
 * @return array 核心效果声明列表（0 或 1 条；未来可扩展多效果）
 */
function battle_skill_effects($skilldata)
{
    $effect_id = isset($skilldata['effect_id']) ? intval($skilldata['effect_id']) : 0;
    if ($effect_id <= 0) {
        return [];
    }
    static $cache = [];
    if (isset($cache[$effect_id])) {
        return $cache[$effect_id];
    }
    $cache[$effect_id] = [];
    $row = DB::fetch_first(pm_sql(
        "SELECT id, code, kind, hooks_json, params_json, version FROM " . pm_table('pm_effect') . " WHERE id = %d",
        $effect_id
    ));
    if ($row) {
        $params = json_decode($row['params_json'], true);
        $hooks = json_decode($row['hooks_json'], true);
        if (is_array($params) && isset($params['code']) && is_array($hooks)) {
            $effect = [
                'code' => strval($params['code']),
                'kind' => strval($row['kind']),
                'hooks' => $hooks,
                'params' => $params,
                'version' => intval($row['version']),
            ];
            if (battle_core_validate_effect($effect) === true) {
                $cache[$effect_id] = [$effect];
            }
        }
    }
    return $cache[$effect_id];
}

/**
 * 异常状态定义目录：pm_status 表行覆盖核心内置目录（数据驱动），
 * 按请求静态缓存；表为空/未建成时回退内置定义。
 */
function battle_status_catalog()
{
    static $catalog = null;
    if ($catalog !== null) {
        return $catalog;
    }
    $catalog = battle_core_status_catalog();
    $rows = DB::fetch_all("SELECT code, name, behavior_json FROM " . pm_table('pm_status'));
    foreach ((array)$rows as $row) {
        $behavior = json_decode($row['behavior_json'], true);
        if (!is_array($behavior)) {
            continue;
        }
        $catalog[$row['code']] = array_merge(['name' => $row['name']], $behavior);
    }
    return $catalog;
}

/**
 * 引擎状态落库（pm_battle + pm_battle_unit + pm_battle_event + 旧列镜像）。
 *
 * 单位行用 DELETE + 重插（单位数 <=2、回合级调用，简单可靠）；事件只追加。
 * ended 状态由 battle_finish / clear_battle_state 负责清镜像，这里不重复。
 *
 * @param array $state
 * @param array $new_events 本回合新增事件（写 pm_battle_event）
 * @return array 落库后的状态（battle_id 回填）
 */
function battle_persist_state($state, $new_events = [])
{
    $rows = battle_state_to_rows($state);
    $b = $rows['battle'];
    $now = time();

    if (!empty($state['battle_id'])) {
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_battle') . " SET
                kind = %s, map_id = %d, turn = %d, phase = %s, result = %s,
                rng_seed = %d, rng_counter = %d, event_seq = %d,
                rules_version = %d, state_version = %d, field_json = %s, updated_at = %d
            WHERE id = %d",
            $b['kind'], $b['map_id'], $b['turn'], $b['phase'], $b['result'],
            $b['rng_seed'], $b['rng_counter'], $b['event_seq'],
            $b['rules_version'], $b['state_version'], $b['field_json'], $now,
            $state['battle_id']
        ));
    } else {
        DB::query(pm_sql(
            "INSERT INTO " . pm_table('pm_battle') . "
                (uid, kind, map_id, turn, phase, result, rng_seed, rng_counter,
                 event_seq, rules_version, state_version, field_json, created_at, updated_at)
            VALUES (%d, %s, %d, %d, %s, %s, %d, %d, %d, %d, %d, %s, %d, %d)",
            $b['uid'], $b['kind'], $b['map_id'], $b['turn'], $b['phase'], $b['result'],
            $b['rng_seed'], $b['rng_counter'], $b['event_seq'],
            $b['rules_version'], $b['state_version'], $b['field_json'], $now, $now
        ));
        $state['battle_id'] = intval(DB::insert_id());
    }

    DB::query(pm_sql("DELETE FROM " . pm_table('pm_battle_unit') . " WHERE battle_id = %d", $state['battle_id']));
    foreach ($rows['units'] as $u) {
        DB::query(pm_sql(
            "INSERT INTO " . pm_table('pm_battle_unit') . "
                (battle_id, side, slot, instance_id, species_id, name, species_name, level,
                 stats_json, types_json, hp, stages_json, status_json, volatile_json,
                 buffs_json, effects_json, fainted, gender, is_shiny, capture_rate, boss_multiplier)
            VALUES (%d, %s, %d, %d, %d, %s, %s, %d, %s, %s, %d, %s, %s, %s, %s, %s, %d, %d, %d, %d, %s)",
            $state['battle_id'], $u['side'], $u['slot'], $u['instance_id'], $u['species_id'],
            $u['name'], $u['species_name'], $u['level'], $u['stats_json'], $u['types_json'],
            $u['hp'], $u['stages_json'], $u['status_json'], $u['volatile_json'],
            $u['buffs_json'], $u['effects_json'], $u['fainted'], $u['gender'],
            $u['is_shiny'], $u['capture_rate'], sprintf('%.4F', $u['boss_multiplier'])
        ));
    }

    foreach ($new_events as $e) {
        DB::query(pm_sql(
            "INSERT INTO " . pm_table('pm_battle_event') . "
                (battle_id, turn, seq, type, payload_json, schema_version, created_at)
            VALUES (%d, %d, %d, %s, %s, %d, %d)",
            $state['battle_id'], intval($e['turn']), intval($e['seq']),
            strval($e['type']), json_encode($e['payload']), BATTLE_EVENT_SCHEMA_VERSION, $now
        ));
    }

    if ($state['phase'] !== 'ended') {
        battle_mirror_legacy($state);
    }
    return $state;
}

/**
 * 把引擎状态镜像写入 pm_usersdata 旧列（兼容投影，见文件头说明）。
 * 野怪信息取敌方第一个单位；allure 编码与旧版一致 (gender<<1)|is_shiny。
 */
function battle_mirror_legacy($state)
{
    if (empty($state['sides']['enemy'])) {
        return;
    }
    $enemy = $state['sides']['enemy'][0];
    $allure = (((int)$enemy['gender']) << 1) | ($enemy['is_shiny'] ? 1 : 0);
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_usersdata') . " SET
            npcid = %d, level = %d, hp = %d, hpg = %d,
            atkg = %d, defg = %d, spatkg = %d, spdefg = %d, sdg = %d,
            capture = %d, allure = %d
        WHERE uid = %d",
        $enemy['species_id'], $enemy['level'], $enemy['hp'], $enemy['stats']['max_hp'],
        $enemy['stats']['atk'], $enemy['stats']['def'],
        $enemy['stats']['spatk'], $enemy['stats']['spdef'], $enemy['stats']['speed'],
        $enemy['capture_rate'], $allure,
        $state['uid']
    ));
}

/**
 * 引擎侧反击结算（捕捉失败 / 战斗用道具 / 主动换宠被反击共用）。
 *
 * 载入战斗 -> 注入我方最新状态（治疗等已先行落库）-> 核心反击 ->
 * 我方 HP 落库 -> 死亡判定（有替补保持 awaiting_switch，无替补 ended/defeat）
 * -> persist。调用方必须已在 pm_usersdata 行锁事务内。
 *
 * @return array|null [state, events, battle_status, battle_ended, can_switch]；
 *                    无进行中战斗返回 null（调用方按旧语义报错）
 */
function battle_resolve_engine_counter($uid, $mypokemon)
{
    $myusersdata = api_my_usersdata($uid);
    $state = battle_load_active($uid, $myusersdata, $mypokemon);
    if ($state === null) {
        return null;
    }
    $mydata = pm_data($mypokemon['species_id']);
    if (!$mydata) {
        pm_abort_battle_transaction('宠物数据异常', 500);
    }
    battle_inject_ally_fresh_state($state, $mypokemon, $mydata);

    $events = [];
    battle_core_counter_attack($state, $events);

    // 我方 HP 落库（反击伤害写回 pm_mypm，含 [0, max_hp] 范围校正）
    $target_ally = null;
    foreach ($state['sides']['ally'] as $u) {
        if ($u['instance_id'] === intval($mypokemon['id'])) {
            $target_ally = $u;
            break;
        }
    }
    if ($target_ally !== null) {
        $mypokemon['hp'] = strval($target_ally['hp']);
        $max_hp_for_validate = api_calculate_pokemon_max_hp($mypokemon);
        $hp_validation = api_validate_and_correct_hp($mypokemon, intval($target_ally['hp']), $max_hp_for_validate);
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
            intval($hp_validation['hp']), intval($mypokemon['id'])
        ));
    }

    $battle_status = 'active';
    $battle_ended = false;
    $can_switch = false;
    if ($state['phase'] === 'awaiting_switch') {
        $battle_status = 'defeat';
        list($battle_ended, $can_switch) = handle_my_pokemon_fainted($uid, $mypokemon['id']);
        if (!$can_switch) {
            $state['phase'] = 'ended';
            $state['result'] = 'defeat';
        }
    }
    $state = battle_persist_state($state, $events);
    if (!$can_switch && $battle_status === 'defeat' && $battle_ended) {
        clear_battle_state($uid);
    }
    return [$state, $events, $battle_status, $battle_ended, $can_switch];
}

/**
 * 渲染一次反击结算的事件文案（供 capture/use_item/switch 复用）。
 * 与旧版逐条文案一致；无替补时过滤换宠提示。
 */
function battle_render_counter_messages($events, $mypokemon, $enemy_name, $can_switch)
{
    $render_events = $events;
    if (!$can_switch) {
        $render_events = array_values(array_filter($render_events, function ($e) {
            return $e['type'] !== 'switch_required';
        }));
    }
    return battle_core_render_messages($render_events, array(
        'ally' => $mypokemon['nickname'] ?: $mypokemon['pmname'],
        'enemy' => $enemy_name,
    ));
}

/**
 * 查询战报（GET ?action=battle_log&battle_id=N）。
 *
 * 返回事件流（版本化）、按事件渲染的逐行文案，以及可直接粘贴到帖子的
 * BBCode 摘要（对局信息 + 逐回合战报）。只能查询本人参与的对局。
 */
function battle_api_get_battle_log()
{
    require_login();
    battle_ensure_tables();

    global $_G;
    $battle_id = intval(get_param('battle_id', 0));
    if ($battle_id <= 0) {
        api_error('Invalid battle_id', 400);
    }

    $battle = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_battle') . " WHERE id = %d AND uid = %d",
        $battle_id, $_G['uid']
    ));
    if (!$battle) {
        api_error('Battle not found', 404);
    }
    $units = DB::fetch_all(pm_sql(
        "SELECT * FROM " . pm_table('pm_battle_unit') . " WHERE battle_id = %d",
        $battle_id
    ));
    $event_rows = DB::fetch_all(pm_sql(
        "SELECT turn, seq, type, payload_json FROM " . pm_table('pm_battle_event') . "
        WHERE battle_id = %d ORDER BY seq ASC",
        $battle_id
    ));

    $events = [];
    foreach ((array)$event_rows as $row) {
        $events[] = [
            'turn' => (int)$row['turn'],
            'seq' => (int)$row['seq'],
            'type' => strval($row['type']),
            'payload' => json_decode($row['payload_json'], true),
        ];
    }

    // 显示名：从单位快照取（我方显示名/野怪种族名）
    $names = ['ally' => '我方', 'enemy' => '野怪'];
    foreach ((array)$units as $u) {
        if ($u['side'] === 'ally') {
            $names['ally'] = $u['name'] ?: $u['species_name'];
        } else {
            $names['enemy'] = $u['species_name'] ?: $u['name'];
        }
    }
    $lines = battle_core_render_messages($events, $names);

    // BBCode：可直接分享到帖子
    $bbcode = "[quote]" . ($battle['kind'] === 'boss' ? '[BOSS战]' : '[野外战斗]') . " 回合数 {$battle['turn']}
";
    $bbcode .= "{$names['ally']} vs {$names['enemy']}
";
    foreach ($lines as $line) {
        $bbcode .= $line . "
";
    }
    $bbcode .= "[/quote]";

    api_success([
        'battle_id' => (int)$battle_id,
        'kind' => strval($battle['kind']),
        'turn' => (int)$battle['turn'],
        'phase' => strval($battle['phase']),
        'result' => strval($battle['result']),
        'rules_version' => (int)$battle['rules_version'],
        'schema_version' => BATTLE_EVENT_SCHEMA_VERSION,
        'names' => $names,
        'lines' => $lines,
        'events' => $events,
        'bbcode' => $bbcode,
    ]);
}

switch ($action) {
    case 'start':
    case 'start':
        api_start_battle();
        break;

    case 'recover':
        api_recover_battle();
        break;

    case 'turn':
        api_use_skill();
        break;

    case 'flee':
        api_flee();
        break;

    default:
        api_error('Invalid action', 400);
}

/**
 * 开始战斗
 */
function api_start_battle()
{
    require_login();

    $input = get_json_input();
    $map_id = isset($input['map_id']) ? intval($input['map_id']) : 0;

    if ($map_id <= 0) {
        $map_id = get_param('map_id', 0);
        $map_id = intval($map_id);
    }

    if ($map_id <= 0) {
        api_error('Invalid map_id', 400);
    }

    $boss_pokemon_type_id = isset($input['boss_pokemon_type_id']) ? intval($input['boss_pokemon_type_id']) : 0;

    global $_G, $myusersdata, $mypokemon, $settings;

    // 获取用户数据
    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    // 检查是否有出战宠物
    if (!$mypokemon || !is_array($mypokemon) || $mypokemon['species_id'] <= 0) {
        api_error('No active pokemon found. Please select a pokemon first.', 400);
    }

    // 检查是否已有进行中的战斗
    if (!empty($myusersdata['npcid']) && $myusersdata['npcid'] > 0) {
        // 返回现有战斗状态
        $battle = build_battle_response($myusersdata, $mypokemon);
        api_success($battle);
    }

    // 获取地图信息
    $map = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_map') . " WHERE id = %d", $map_id));
    if (!$map) {
        api_error('Map not found', 404);
    }

    // 检查宠物HP
    if ($mypokemon['hp'] <= 0) {
        api_error('你的宠物已晕倒，请先前往宠物中心治疗！', 400);
    }

    // 检查宠物是否为濒危状态
    if ((int)$mypokemon['state'] === 0) {
        api_error('你的宠物处于濒危状态，请先前往宠物中心治疗！', 400);
    }

    // 生成野怪
    $wild = generate_wild_pokemon_legacy($map, $myusersdata['strength'], $boss_pokemon_type_id > 0 ? $boss_pokemon_type_id : null);

    // 计算 Boss 属性倍率
    $is_boss = isset($wild['is_boss']) && $wild['is_boss'];
    $boss_multiplier = isset($wild['boss_multiplier']) ? $wild['boss_multiplier'] : 1.0;

    // 计算野怪属性
    $npc = pm_data($wild['npcid']);

    // 如果是 Boss，应用属性倍率
    if ($is_boss) {
        $npc['strength'] = $npc['strength'] * $boss_multiplier;
    }

    list($npcmhp, $npcatk, $npcdef, $npcspatk, $npcspdef, $npcsd) = battle_calc_new_npc_stats(
        $npc,
        $wild['level'],
        $myusersdata['strength'] * $npc['strength']
    );

    // 生成野怪的性别和闪光状态（只生成一次，战斗过程中保持不变）
    $gender = 0;
    $sexrand = rand(1, 1000);
    if ($npc['sex'] > 0) {
        $gender = ($sexrand <= $npc['sex']) ? 0 : 1;
    }
    $is_shiny = (rand(1, 4096) === 1);

    // 编码到 allure 字段: (gender << 1) | (is_shiny ? 1 : 0)
    $allure_value = ($gender << 1) | ($is_shiny ? 1 : 0);

    // ===== 战斗引擎 2.0：状态入 pm_battle/pm_battle_unit，旧列作为镜像投影 =====
    battle_ensure_tables();

    // 一人多战约束：开新战前结束该用户遗留的进行中战斗（断线/超时残留；
    // 正常路径镜像旧列已清，这里只兜底引擎侧孤儿行）
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_battle') . "
        SET phase = 'ended', result = IF(result = '', 'abandoned', result), updated_at = %d
        WHERE uid = %d AND phase IN ('active', 'awaiting_switch')",
        time(), $_G['uid']
    ));

    // 战报保留 30 天：清理该用户过期的已结束对局与事件（战报留存期）
    $stale = DB::fetch_all(pm_sql(
        "SELECT id FROM " . pm_table('pm_battle') . "
        WHERE uid = %d AND phase = 'ended' AND updated_at > 0 AND updated_at < %d",
        $_G['uid'], time() - 2592000
    ));
    foreach ((array)$stale as $stale_row) {
        DB::query(pm_sql("DELETE FROM " . pm_table('pm_battle_event') . " WHERE battle_id = %d", intval($stale_row['id'])));
        DB::query(pm_sql("DELETE FROM " . pm_table('pm_battle_unit') . " WHERE battle_id = %d", intval($stale_row['id'])));
        DB::query(pm_sql("DELETE FROM " . pm_table('pm_battle') . " WHERE id = %d", intval($stale_row['id'])));
    }

    $mydata = pm_data($mypokemon['species_id']);
    list($mpmhp, $matk, $mdef, $mspatk, $mspdef, $msd) = battle_calc_my_stats($mydata, $mypokemon);

    $state = battle_core_initial_state([
        'uid' => intval($_G['uid']),
        'kind' => $is_boss ? 'boss' : 'wild',
        'map_id' => $map_id,
        // 每场一个随机种子，回合内随机全部由 seed+counter 推导（可确定性重放）
        'rng_seed' => mt_rand(1, 2147483647),
        'allies' => [[
            'instance_id' => intval($mypokemon['id']),
            'species_id' => intval($mypokemon['species_id']),
            'name' => $mypokemon['nickname'] ?: $mypokemon['pmname'],
            'species_name' => $mypokemon['pmname'],
            'level' => intval($mypokemon['level']),
            'hp' => intval($mypokemon['hp']),
            'stats' => [
                'max_hp' => $mpmhp, 'atk' => $matk, 'def' => $mdef,
                'spatk' => $mspatk, 'spdef' => $mspdef, 'speed' => $msd,
            ],
            'types' => [$mydata['xs'], $mydata['xs2']],
        ]],
        'enemies' => [[
            'species_id' => intval($wild['npcid']),
            'name' => $npc['name'],
            'species_name' => $npc['name'],
            'level' => intval($wild['level']),
            'hp' => intval($npcmhp),
            'stats' => [
                'max_hp' => $npcmhp, 'atk' => $npcatk, 'def' => $npcdef,
                'spatk' => $npcspatk, 'spdef' => $npcspdef, 'speed' => $npcsd,
            ],
            'types' => [$npc['xs'], $npc['xs2']],
            'gender' => $gender,
            'is_shiny' => $is_shiny,
            'capture_rate' => intval($wild['capture']),
            'boss_multiplier' => $boss_multiplier,
        ]],
    ]);

    $start_events = [];
    battle_core_emit($state, $start_events, 'battle_start', ['kind' => $state['kind'], 'map_id' => $map_id]);
    $state = battle_persist_state($state, $start_events);

    // 重新获取用户数据
    $myusersdata = api_my_usersdata($_G['uid']);

    // 构建响应（传递 is_boss 和 boss_multiplier 作为参数）
    $battle = build_battle_response($myusersdata, $mypokemon, $map, $is_boss, $boss_multiplier);

    // 根据是否是 Boss 显示不同的消息
    if ($is_boss) {
        $battle['message'] = "野生的 {$battle['wild_pokemon']['name']} (Boss) 出现了！属性倍率: {$boss_multiplier}x";
    } else {
        $battle['message'] = "野生的 {$battle['wild_pokemon']['name']} 出现了！";
    }

    api_success($battle);
}

/**
 * 归还 api_use_skill 预扣的技能 PP。
 *
 * PP 在进入战斗计算前原子预扣；攻击被闪避、或后手时宠物未及出手就倒下
 * 的回合按原有语义不消耗 PP，这两类路径在此处原路退还。
 * skillnum < max_uses 封顶：预扣与退还之间若有并发的回满操作（PP 道具），
 * 退还不越过上限。
 */
function pm_refund_reserved_skill_pp($skill_id, $uid, $pet_id, $max_uses)
{
    if ($skill_id > 0 && $max_uses != 0) {
        DB::query(pm_sql("UPDATE " . pm_table('pm_myskill') . "
            SET skillnum = skillnum + 1
            WHERE skillid = %d AND uid = %d AND petid = %d AND skillnum < %d",
            $skill_id, $uid, $pet_id, $max_uses
        ));
    }
}

/**
 * 使用技能攻击
 *
 * 战斗引擎 2.0：一回合一个事务（pm_usersdata 行锁串行化，沿用 #69-71
 * 锁策略）。端点只做载入 -> battle_core_apply_action 纯核心 -> 落库 +
 * 组响应；伤害/会心/闪避/先手公式与响应字段与旧版逐项一致。
 */
function api_use_skill()
{
    require_login();

    $input = get_json_input();
    $skill_id = isset($input['skill_id']) ? intval($input['skill_id']) : 0;

    global $_G, $myusersdata, $mypokemon, $settings, $petbasisexp;

    // 获取用户数据
    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    // 检查是否有进行中的战斗
    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        api_error('No active battle found', 400);
    }

    // 倒下后有替补时仍保留战斗状态，但当前宠物不能继续出招或领取胜利奖励。
    if (!$mypokemon) {
        api_error('没有上场宠物', 400);
    }
    if ((int)$mypokemon['hp'] <= 0 || (int)$mypokemon['state'] === 0) {
        api_error('当前宠物已倒下，请先更换宠物', 400);
    }

    battle_ensure_tables();

    // ===== 一回合一个事务 =====
    // 以 pm_usersdata 战斗状态行的排他锁串行化同账号并发回合（#69-71 同款锁点）。
    // api_error() 走 exit 语义，事务内的错误出口统一走 pm_abort_battle_transaction()。
    DB::query("START TRANSACTION");
    try {
        DB::fetch_first(pm_sql(
            "SELECT uid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $_G['uid']
        ));

        // 先确认当前宠物已学会技能，并原子预扣 PP，再进入战斗计算。
        // 预扣是条件 UPDATE（skillnum > 0 才扣减）：并发请求只有一个能扣到，
        // 扣不到说明 PP 已被并发回合消耗，直接拒绝，不产生任何战斗写入。
        // 攻击被闪避或未及出手的回合由 pm_refund_reserved_skill_pp() 退还。
        $skilldata = null;
        $myskill = null;
        if ($skill_id > 0) {
            $skilldata = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_skill') . " WHERE id = %d", $skill_id));
            if (!$skilldata) {
                pm_abort_battle_transaction('技能不存在', 404);
            }

            $myskill = DB::fetch_first(pm_sql(
                "SELECT * FROM " . pm_table('pm_myskill') . "
                WHERE skillid = %d AND uid = %d AND petid = %d",
                $skill_id, $_G['uid'], $mypokemon['id']
            ));
            if (!$myskill) {
                pm_abort_battle_transaction('当前宠物尚未学会该技能', 400);
            }
            if ($myskill['skillnum'] <= 0 && $skilldata['max_uses'] != 0) {
                pm_abort_battle_transaction('Skill PP is depleted', 400);
            }
            if ($skilldata['max_uses'] != 0) {
                DB::query(pm_sql("UPDATE " . pm_table('pm_myskill') . "
                    SET skillnum = skillnum - 1
                    WHERE skillid = %d AND uid = %d AND petid = %d AND skillnum > 0",
                    $skill_id, $_G['uid'], $mypokemon['id']
                ));
                if (!DB::affected_rows()) {
                    pm_abort_battle_transaction('Skill PP is depleted', 400);
                }
            }
        }

        // ===== 载入战斗（新表优先；升级部署前的老战斗在此惰性迁移）=====
        $myusersdata = api_my_usersdata($_G['uid']);
        $state = battle_load_active($_G['uid'], $myusersdata, $mypokemon);
        if ($state === null) {
            pm_abort_battle_transaction('No active battle found', 400);
        }

        // 我方属性每回合现算注入（装备/治疗/换宠下一回合生效，与旧版行为一致）
        $mydata = pm_data($mypokemon['species_id']);
        if (!$mydata) {
            pm_abort_battle_transaction('宠物数据异常', 500);
        }
        battle_inject_ally_fresh_state($state, $mypokemon, $mydata);

        // 构造行动（技能规约与旧版一致：power 空/0 回退 40、element 回退宠物 xs、中文 category 归一化）
        if ($skill_id > 0) {
            $action = array('type' => 'move', 'skill' => array(
                'id' => intval($skilldata['id']),
                'name' => $skilldata['name'],
                // rules_version 1：power=0 的技能按 40 威力攻击结算（数值兼容）；
                // rules_version 2 起 power=0 是变化技（伤害 0，主效果走 on_after_move）
                'power' => ((int)$state['rules_version'] >= 2) ? intval($skilldata['power']) : (intval($skilldata['power']) ?: 40),
                'type' => $skilldata['element'] ?: $mydata['xs'],
                'category' => api_normalize_skill_category($skilldata['category']),
                'effects' => battle_skill_effects($skilldata),
            ));
        } else {
            $action = array('type' => 'struggle', 'fallback_type' => $mydata['xs']);
        }

        // 敌方 AI 选招（v2）：野怪从自身种族可用技能里选招，而非固定反击
        $enemy_move = battle_pick_enemy_move($state);
        if ($enemy_move !== null) {
            $action['enemy_move'] = $enemy_move;
        }

        // ===== 纯核心计算一整回合（先手判定 -> 出招 -> 反击 -> 胜负）=====
        $result = battle_core_apply_action($state, $action);
        $state = $result['state'];
        $turn_events = $result['events'];

        // 攻击被闪避或未及出手：退还预扣 PP（#68 语义不变）
        if ($result['pp_refund']) {
            pm_refund_reserved_skill_pp($skill_id, $_G['uid'], $mypokemon['id'], $skilldata ? $skilldata['max_uses'] : 0);
        }

        // 我方 HP 落库（反击伤害写回 pm_mypm，含 [0, max_hp] 范围校正）
        $target_ally = null;
        foreach ($state['sides']['ally'] as $u) {
            if ($u['instance_id'] === intval($mypokemon['id'])) {
                $target_ally = $u;
                break;
            }
        }
        if ($target_ally === null) {
            $target_ally = battle_core_active_unit($state, 'ally');
        }
        if ($target_ally !== null) {
            $mypokemon['hp'] = strval($target_ally['hp']);
            $max_hp_for_validate = api_calculate_pokemon_max_hp($mypokemon);
            $hp_validation = api_validate_and_correct_hp($mypokemon, intval($target_ally['hp']), $max_hp_for_validate);
            DB::query(pm_sql(
                "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
                intval($hp_validation['hp']), intval($mypokemon['id'])
            ));
        }

        // ===== 结果分类（语义与旧版一一对应）=====
        $battle_status = 'active';
        $battle_ended = false;
        $can_switch = false;
        $rewards = null;
        $level_up_info = null;

        $enemy_unit = !empty($state['sides']['enemy']) ? $state['sides']['enemy'][0] : null;
        // 落库前的野怪显示快照（战斗结束后镜像清零，响应组装时临时恢复）
        $display_npc = $enemy_unit ? array(
            'npcid' => intval($enemy_unit['species_id']),
            'level' => intval($enemy_unit['level']),
            'hp' => intval($enemy_unit['hp']),
            'hpg' => intval($enemy_unit['stats']['max_hp']),
        ) : null;

        if ($state['phase'] === 'ended' && $state['result'] === 'victory') {
            // 胜利：结算奖励并结束
            $battle_status = 'victory';
            $battle_ended = true;

            $npc = pm_data($enemy_unit['species_id']);
            $rewards = calculate_rewards($mypokemon, $myusersdata, $npc, $enemy_unit['level'], null);

            $state = battle_persist_state($state, $turn_events);
            // 清镜像旧列（新行已 ended，clear 的联动更新幂等不命中）
            clear_battle_state($_G['uid']);
            $level_up_info = apply_rewards($_G['uid'], $mypokemon, $rewards);
            $display_npc['hp'] = 0;
        } elseif ($state['phase'] !== 'active') {
            // 我方被反击打倒（awaiting_switch）：有替补时战斗继续，无替补才真正结束
            $battle_status = 'defeat';
            list($battle_ended, $can_switch) = handle_my_pokemon_fainted($_G['uid'], $mypokemon['id']);
            if (!$can_switch) {
                $state['phase'] = 'ended';
                $state['result'] = 'defeat';
            }
            $state = battle_persist_state($state, $turn_events);
            if (!$can_switch) {
                // handle 内部已 clear 过一次；重复调用幂等，确保镜像与事件一致
                clear_battle_state($_G['uid']);
            }
        } else {
            // 战斗继续：落库（镜像野怪 HP，含等待替补时保留已造成的伤害）
            $state = battle_persist_state($state, $turn_events);
        }

        DB::query("COMMIT");
    } catch (Throwable $txn_error) {
        DB::query("ROLLBACK");
        throw $txn_error;
    }

    // ===== 响应组装（字段与旧版完全一致）=====
    $render_events = $turn_events;
    if ($battle_status === 'defeat' && !$can_switch) {
        // 无替补时过滤 switch_required：核心无法预知替补，端点判定后修正文案
        $render_events = array_values(array_filter($render_events, function ($e) {
            return $e['type'] !== 'switch_required';
        }));
    }
    $damage_log = battle_core_render_messages($render_events, array(
        'ally' => $mypokemon['nickname'] ?: $mypokemon['pmname'],
        'enemy' => $enemy_unit['species_name'],
    ));

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    if ($battle_status !== 'active' && $display_npc) {
        // 临时恢复野怪信息以便正确显示（旧版模式）
        $myusersdata['npcid'] = $display_npc['npcid'];
        $myusersdata['level'] = $display_npc['level'];
        $myusersdata['hp'] = $display_npc['hp'];
        $myusersdata['hpg'] = $display_npc['hpg'];
    }

    $battle = build_battle_response($myusersdata, $mypokemon);
    $battle['status'] = $battle_status;
    $battle['message'] = implode("\n", $damage_log);
    $battle['turn'] = ($battle_status === 'active') ? 1 : 0;
    // defeat 且还有替补时战斗并未结束（battle_over=false），
    // 客户端应依据 can_continue_switch 弹出换宠选择，而不是把 defeat 当终局
    $battle['battle_over'] = $battle_ended;
    $battle['can_continue_switch'] = $can_switch;
    // 引擎 2.0 新增可选字段：真实回合数与本回合事件流（老客户端可安全忽略）
    $battle['battle_turn'] = intval($state['turn']);
    $battle['events'] = array();
    foreach ($turn_events as $e) {
        $battle['events'][] = array('turn' => $e['turn'], 'seq' => $e['seq'], 'type' => $e['type'], 'payload' => $e['payload']);
    }

    if ($rewards) {
        $battle['rewards'] = $rewards;
    }

    if ($level_up_info && $level_up_info['level_up']) {
        $battle['level_up'] = $level_up_info;
        $pokemon_name = $mypokemon['nickname'] ?: $mypokemon['pmname'];
        $battle['message'] .= "\n🎉 {$pokemon_name}升级了！Lv.{$level_up_info['old_level']} → Lv.{$level_up_info['new_level']}";
    }

    api_success($battle);
}

/**
 * 逃跑
 *
 * 判定与失败反击走引擎核心，同样在一个事务内完成（#69-71 锁策略）。
 */
function api_flee()
{
    require_login();

    global $_G, $myusersdata, $mypokemon;

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        // 服务端已无进行中的战斗（例如胜利后前端仍停在战斗界面）。
        // 幂等返回 fled，前端据此正常收尾。
        $battle = build_battle_response($myusersdata, $mypokemon);
        $battle['status'] = 'fled';
        $battle['message'] = '战斗已经结束。';
        $battle['turn'] = 0;
        $battle['battle_over'] = true;
        $battle['can_continue_switch'] = false;
        api_success($battle);
    }

    battle_ensure_tables();

    DB::query("START TRANSACTION");
    try {
        DB::fetch_first(pm_sql(
            "SELECT uid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $_G['uid']
        ));

        $myusersdata = api_my_usersdata($_G['uid']);
        $state = battle_load_active($_G['uid'], $myusersdata, $mypokemon);
        if ($state === null) {
            pm_abort_battle_transaction('No active battle found', 400);
        }

        $mydata = pm_data($mypokemon['species_id']);
        if (!$mydata) {
            pm_abort_battle_transaction('宠物数据异常', 500);
        }
        battle_inject_ally_fresh_state($state, $mypokemon, $mydata);

        // ===== 逃跑判定（纯核心，公式与旧版一致）=====
        $flee = battle_core_try_flee($state);
        $state = $flee['state'];
        $turn_events = $flee['events'];

        $enemy_unit = !empty($state['sides']['enemy']) ? $state['sides']['enemy'][0] : null;
        $display_npc = $enemy_unit ? array(
            'npcid' => intval($enemy_unit['species_id']),
            'level' => intval($enemy_unit['level']),
            'hp' => intval($enemy_unit['hp']),
            'hpg' => intval($enemy_unit['stats']['max_hp']),
        ) : null;

        $message = '';
        $status = 'active';
        $battle_ended = false;
        $can_switch = false;

        if ($flee['fled']) {
            $status = 'fled';
            $message = '成功逃脱了！';
            $battle_ended = true;

            $state = battle_persist_state($state, $turn_events);
            clear_battle_state($_G['uid']);
        } else {
            $message = '逃跑失败！';

            // 野怪反击（纯核心 counter）
            battle_core_counter_attack($state, $turn_events);

            // 我方 HP 落库（含范围校正）
            $target_ally = null;
            foreach ($state['sides']['ally'] as $u) {
                if ($u['instance_id'] === intval($mypokemon['id'])) {
                    $target_ally = $u;
                    break;
                }
            }
            if ($target_ally !== null) {
                $mypokemon['hp'] = strval($target_ally['hp']);
                $max_hp_for_validate = api_calculate_pokemon_max_hp($mypokemon);
                $hp_validation = api_validate_and_correct_hp($mypokemon, intval($target_ally['hp']), $max_hp_for_validate);
                DB::query(pm_sql(
                    "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
                    intval($hp_validation['hp']), intval($mypokemon['id'])
                ));
            }

            if ($state['phase'] === 'awaiting_switch') {
                // 反击致死：与 use_skill 保持一致，有替补继续、无替补结束
                $status = 'defeat';
                list($battle_ended, $can_switch) = handle_my_pokemon_fainted($_G['uid'], $mypokemon['id']);
                if (!$can_switch) {
                    $state['phase'] = 'ended';
                    $state['result'] = 'defeat';
                }
            }

            $state = battle_persist_state($state, $turn_events);
            if (!$can_switch && $status === 'defeat' && $battle_ended) {
                clear_battle_state($_G['uid']);
            }
        }

        DB::query("COMMIT");
    } catch (Throwable $txn_error) {
        DB::query("ROLLBACK");
        throw $txn_error;
    }

    // 响应文案：逃跑结果 + 反击/倒下（渲染覆盖倒下/替补提示，不重复手动拼接）
    $extra = '';
    if (!$flee['fled']) {
        $render_events = $turn_events;
        if ($status === 'defeat' && !$can_switch) {
            // 无替补时过滤 switch_required：核心无法预知替补，端点判定后修正文案
            $render_events = array_values(array_filter($render_events, function ($e) {
                return $e['type'] !== 'switch_required';
            }));
        }
        $rendered = battle_core_render_messages($render_events, array(
            'ally' => $mypokemon['nickname'] ?: $mypokemon['pmname'],
            'enemy' => $enemy_unit['species_name'],
        ));
        $extra = implode("\n", $rendered);
    }

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    if ($status !== 'active' && $display_npc) {
        $myusersdata['npcid'] = $display_npc['npcid'];
        $myusersdata['level'] = $display_npc['level'];
        $myusersdata['hp'] = $display_npc['hp'];
        $myusersdata['hpg'] = $display_npc['hpg'];
    }

    $battle = build_battle_response($myusersdata, $mypokemon);
    $battle['status'] = $status;
    $battle['message'] = $extra !== '' ? ($message . "\n" . $extra) : $message;
    $battle['turn'] = 0;
    $battle['battle_over'] = $battle_ended;
    $battle['can_continue_switch'] = $can_switch;
    $battle['battle_turn'] = intval($state['turn']);
    $battle['events'] = array();
    foreach ($turn_events as $e) {
        $battle['events'][] = array('turn' => $e['turn'], 'seq' => $e['seq'], 'type' => $e['type'], 'payload' => $e['payload']);
    }

    api_success($battle);
}

/**
 * 生成野怪（旧版逻辑）
 */
function generate_wild_pokemon_legacy($map, $strength, $force_boss_type_id = null)
{
    $map_id = $map['id'];

    // 检查是否有 Boss 配置（使用 Boss API 函数）
    $boss_config = get_map_boss_config_from_map($map);

    if ($boss_config && !empty($boss_config['bosses'])) {
        if ($force_boss_type_id > 0) {
            // 强制挑战指定 Boss（跳过刷新概率判定）
            $boss = null;
            foreach ($boss_config['bosses'] as $b) {
                if (isset($b['pokemon_type_id']) && intval($b['pokemon_type_id']) === intval($force_boss_type_id)) {
                    $boss = $b;
                    break;
                }
            }
        } else {
            // 尝试刷新 Boss（使用 Boss API 函数）
            $boss = try_spawn_boss_from_config($boss_config);
        }

        if ($boss !== null) {
            // 成功刷新出 Boss
            $pet = DB::fetch_first(pm_sql(
                "SELECT * FROM " . pm_table('pm_data') . "
                WHERE id = %d LIMIT 1",
                intval($boss['pokemon_type_id'])
            ));

            if ($pet) {
                $level = $boss['level'];
                $boss_multiplier = $boss['boss_multiplier'];

                return [
                    'npcid' => $pet['id'],
                    'level' => $level,
                    'capture' => $pet['capture'] ?: 100,
                    'is_boss' => true,
                    'boss_multiplier' => $boss_multiplier,
                ];
            }
        }
    }

    // 随机选择一个可遇到的宠物
    for ($i = 0; $i < 100; $i++) {
        $pet = DB::fetch_first(pm_sql(
            "SELECT * FROM " . pm_table('pm_data') . "
            WHERE mapid LIKE %s OR mapid = '999'
            ORDER BY rand() LIMIT 1",
            '%' . $map_id . '%'
        ));

        if (!$pet) {
            api_error('No pokemon found in this area', 500);
        }

        $met = rand(1, 100);
        $pet_met = $pet['met'] ?: 101;

        if ($met < $pet_met) {
            break;
        }
    }

    // 确定等级（pm_data 没有 pve_catch_level 列，按地图等级区间随机）
    global $settings;
    if (!empty($settings['pve_catch_level']) && $settings['pve_catch_level'] > 0) {
        $level = (int)$settings['pve_catch_level'];
    } else {
        $level = rand((int)$map['min_level'], (int)$map['max_level']);
    }

    return [
        'npcid' => $pet['id'],
        'level' => $level,
        'capture' => $pet['capture'] ?: 100,
        'is_boss' => false,
    ];
}



/**
 * 归一化技能攻击分类
 * pm_skill.category 为中文字符串（'物攻'/'特攻'），返回 1 表示特殊攻击，0 表示物理攻击
 */
function api_normalize_skill_category($category)
{
    $category = trim(strval($category));
    if ($category === '1' || $category === '特攻') {
        return 1;
    }
    return 0;
}

/**
 * 计算我方宠物六维属性（从宠物实例数据读取IV/EV，应用状态修正）
 * @return array [hp, atk, def, spatk, spdef, sd]
 */
function battle_calc_my_stats($data, $pokemon)
{
    $level = intval($pokemon['level']);
    $flash = intval($pokemon['is_shiny']);
    $s = intval($pokemon['state']);

    // pm_mypm 中速度的 IV/EV 列是 sdg/sdn，状态修正变量是 $statesd（见 utils.php），
    // 与其他五项的命名规则不同，必须单独映射，否则速度恒为 0（永远后手）
    $stat_columns = [
        'hp'    => ['iv' => 'hpg',    'ev' => 'hpn',    'state' => 'statehp'],
        'atk'   => ['iv' => 'atkg',   'ev' => 'atkn',   'state' => 'stateatk'],
        'def'   => ['iv' => 'defg',   'ev' => 'defn',   'state' => 'statedef'],
        'spatk' => ['iv' => 'spatkg', 'ev' => 'spatkn', 'state' => 'statespatk'],
        'spdef' => ['iv' => 'spdefg', 'ev' => 'spdefn', 'state' => 'statespdef'],
        'speed' => ['iv' => 'sdg',    'ev' => 'sdn',    'state' => 'statesd'],
    ];

    $stats = [];
    foreach ($stat_columns as $stat => $cols) {
        $base = $data[$stat];
        $iv = intval($pokemon[$cols['iv']]);
        $ev = intval($pokemon[$cols['ev']]);
        $is_hp = ($stat === 'hp');
        $boost = $is_hp ? (10 + $level) : 5;
        if ($flash == 1) $boost *= 2;
        $state_arr = isset($GLOBALS[$cols['state']]) ? $GLOBALS[$cols['state']] : [];
        $state_mult = isset($state_arr[$s]) ? (float)$state_arr[$s] : 1.0;
        $stats[] = floor(((2 * $base + $iv + $ev / 4) * $level / 100 + $boost) * $state_mult);
    }

    // 装备加成：api_parse_pet_wear_items 通过第三个引用参数把 HP 加成累加进 $eq_hp_total，
    // 其余五项从返回数组读取；不得对返回数组的 [0] 再累加（它是对 $eq_hp_total 的引用）
    $eq_hp_total = 0;
    $equipment_bonuses = api_parse_pet_wear_items($pokemon, false, $eq_hp_total);
    if (!empty($equipment_bonuses)) {
        $stats[0] += (int)$eq_hp_total;
        for ($i = 1; $i <= 5; $i++) {
            $stats[$i] += (int)$equipment_bonuses[$i];
        }
    }

    return $stats;
}


/**
 * 生成新野怪属性（随机IV/EV，10% 闪光概率）
 * @return array [hp, atk, def, spatk, spdef, sd]
 */
function battle_calc_new_npc_stats($data, $level, $strength = 1)
{
    $level = intval($level);
    $flash = rand(1, 100) > 90 ? 1 : 0;
    if ($strength <= 0) $strength = 1;
    $stats = [];
    foreach (['hp', 'atk', 'def', 'spatk', 'spdef', 'speed'] as $stat) {
        $base = $data[$stat];
        $iv = rand(0, 31);
        $ev = min(85, rand(0, 85) * $level / 100);
        $is_hp = ($stat === 'hp');
        $boost = $is_hp ? (10 + $level) : 5;
        if ($flash == 1) $boost *= 2;
        $stats[] = floor((2 * $base + $iv + $ev / 4) * $level / 100 + $boost) * $strength;
    }
    return $stats;
}



/**
 * 计算奖励
 */
function calculate_rewards($mypokemon, $myusersdata, $npc, $npc_level, $map)
{
    global $petbasisexp, $settings;

    // 经验
    $base_exp = isset($petbasisexp[$myusersdata['npcid']]) ? $petbasisexp[$myusersdata['npcid']] : 64;
    $exp_multiplier = !empty($settings['pve_catch_xp_multiple']) ? $settings['pve_catch_xp_multiple'] : 1;

    $getexp = floor(($base_exp * $npc_level / 7) * 1.5 * $exp_multiplier / 4);

    // 金币
    $drop_money = json_decode($npc['drop_money'], true);
    $money_min = !empty($drop_money[0]) ? $drop_money[0] : 10;
    $money_max = !empty($drop_money[1]) ? $drop_money[1] : 50;
    $money = rand($money_min, $money_max) * $myusersdata['strength'];

    return [
        'exp' => $getexp,
        'money' => $money,
    ];
}

/**
 * 应用奖励（包含升级判断）
 */
function apply_rewards($uid, $mypokemon, $rewards)
{
    global $_G;

    // pm_mypm 的经验列是 exp，calculate_rewards 返回的键也是 exp；
    // 旧代码读 experience 两处都取不到，导致每次胜利把经验写成 0、永远无法升级
    $new_exp = intval($mypokemon['exp']) + intval($rewards['exp']);
    $old_level = intval($mypokemon['level']);
    $pmno = intval($mypokemon['species_id']);

    $new_level = api_get_pet_exp_level($pmno, $new_exp);
    $level_up = false;

    if ($new_level > $old_level) {
        if ($new_level > 100) {
            $new_level = 100;
        }
        $level_up = true;

        // 升级时，需要先更新宠物数据的等级，然后计算新的最大 HP
        $mypokemon_for_calc = $mypokemon;
        $mypokemon_for_calc['level'] = $new_level;
        $new_max_hp = api_calculate_pokemon_max_hp($mypokemon_for_calc);
        $current_hp = (int) $mypokemon['hp'];
        // 升级时回满血
        $new_hp = $new_max_hp;

        DB::query(pm_sql("UPDATE " . pm_table('pm_mypm') . "
            SET exp = %d, level = %d, hp = %d
            WHERE id = %d",
            $new_exp, $new_level, $new_hp, $mypokemon['id']
        ));
    } else {
        DB::query(pm_sql("UPDATE " . pm_table('pm_mypm') . "
            SET exp = %d
            WHERE id = %d",
            $new_exp, $mypokemon['id']
        ));
    }

    DB::query(pm_sql("UPDATE " . pm_table('pm_usersdata') . "
        SET money = money + %d,
            dataall = dataall + 1,
            datawin = datawin + 1
        WHERE uid = %d",
        $rewards['money'],
        $uid
    ));

    return [
        'level_up' => $level_up,
        'old_level' => $old_level,
        'new_level' => $new_level,
        'new_exp' => $new_exp,
    ];
}

/**
 * 清理战斗状态
 */
function clear_battle_state($uid)
{
    // 同步结束新引擎的进行中战斗（capture/use_item/switch 等路径收尾时联动）。
    // 已有 result 的战斗（fled/victory 已由新引擎路径写入）不覆盖 result；
    // 无 result 的（旧路径清状态）记为 abandoned。
    battle_ensure_tables();
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_battle') . "
        SET phase = 'ended', result = IF(result = '', 'abandoned', result), updated_at = %d
        WHERE uid = %d AND phase IN ('active', 'awaiting_switch')",
        time(), $uid
    ));
    // 这些列均为整数类型：写入 '' 在 MariaDB 严格模式(STRICT_TRANS_TABLES)下会直接报错，
    // 导致战斗状态无法清除（用户卡在战斗中），必须写 0
    DB::query(pm_sql("UPDATE " . pm_table('pm_usersdata') . "
        SET npcid = 0, level = 0, hp = 0, hpg = 0, atkg = 0, defg = 0,
            spatkg = 0, spdefg = 0, sdg = 0, allure = 0, capture = 0
        WHERE uid = %d", $uid));
}

/**
 * 我方宠物倒下时的统一判定。
 *
 * 还有可用替补（site<3 且 hp>0 且 state!=0，排除当前宠物）时战斗继续：
 * 保留 pm_usersdata 里的战斗状态，交给换宠/替换接口接管；
 * 没有任何可用替补才真正结束（清空战斗状态）。
 *
 * 返回 [battle_over, can_continue_switch]：
 * - battle_over：战斗是否已在服务端结束；
 * - can_continue_switch：宠物倒下但战斗继续，客户端应弹出换宠选择。
 */
function handle_my_pokemon_fainted($uid, $current_pet_id)
{
    $available_count = DB::result_first(pm_sql(
        "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . "
        WHERE uid = %d AND site < 3 AND hp > 0 AND state != 0 AND id != %d",
        $uid,
        $current_pet_id
    ));

    if ($available_count > 0) {
        return [false, true];
    }

    clear_battle_state($uid);
    return [true, false];
}

/**
 * 构建战斗响应
 * @param array $myusersdata 用户数据
 * @param array $mypokemon 宠物数据
 * @param array|null $map 地图数据
 * @param bool|null $is_boss 是否为 Boss（可选，默认从数据库读取或 false）
 * @param float|null $boss_multiplier Boss 倍率（可选，默认 1.0）
 */
function build_battle_response($myusersdata, $mypokemon, $map = null, $is_boss = null, $boss_multiplier = null)
{
    $is_in_battle = !empty($myusersdata['npcid']) && $myusersdata['npcid'] > 0;

    // 获取我方宠物技能（从 pm_myskill 表查询）
    $skills = [];
    $petid = $mypokemon['id'];
    $uid = $myusersdata['uid'];

    $skills_raw = DB::fetch_all(
        "SELECT ms.skillid, ms.skillnum, s.name, s.power, s.max_uses as max_pp, s.element, s.category "
            . "FROM " . pm_table('pm_myskill') . " ms "
            . "LEFT JOIN " . pm_table('pm_skill') . " s ON ms.skillid = s.id "
            . "WHERE ms.petid = $petid AND ms.uid = $uid "
            . "LIMIT 4"
    );

    foreach ($skills_raw as $sk) {
        $skills[] = [
            'id' => (int)$sk['skillid'],
            'name' => $sk['name'],
            'power' => (int)($sk['power'] ?: 40),
            'pp' => (int)$sk['skillnum'],
            'max_pp' => (int)$sk['max_pp'],
            'skill_type' => $sk['element'] ?: '',
            'category' => $sk['category'] ?: '',
        ];
    }

    // 计算我方宠物 max_hp（使用统一计算函数）
    $my_max_hp = api_calculate_pokemon_max_hp($mypokemon);

    // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
    $hp_validation = api_validate_and_correct_hp($mypokemon, null, $my_max_hp);
    $mypokemon['hp'] = strval($hp_validation['hp']);

    $response = [
        'battle_id' => 'battle_' . $myusersdata['uid'],
        'map_id' => $map ? (int)$map['id'] : 0,
        'map_name' => $map ? $map['name'] : '',
        'turn' => $is_in_battle ? 1 : 0,
        'status' => $is_in_battle ? 'active' : 'idle',
        // 明确的结束/换宠语义：status 在「宠物倒下但还有替补」时仍报 defeat（兼容旧客户端），
        // 但 battle_over=false 且 can_continue_switch=true 表示战斗仍在继续、应弹换宠
        'battle_over' => !$is_in_battle,
        'can_continue_switch' => false,
        'message' => '',
        'my_pokemon' => [
            'id' => (int)$mypokemon['species_id'],
            'instance_id' => (int)$mypokemon['id'],  // 数据库唯一 ID
            'name' => $mypokemon['nickname'] ?: $mypokemon['pmname'],
            'level' => (int)$mypokemon['level'],
            'hp' => (int)$mypokemon['hp'],
            'max_hp' => $my_max_hp,
            'skills' => $skills,
        ],
        'wild_pokemon' => [
            'id' => 0,
            'name' => '',
            'level' => 0,
            'hp' => 0,
            'max_hp' => 0,
            'gender' => 0,
            'is_shiny' => false,
            'boss_multiplier' => 1.0,
        ],
    ];

    if ($is_in_battle) {
        $npc = pm_data($myusersdata['npcid']);

        // 确定性别和闪光状态（从存储中读取或生成）
        $gender = 0;
        $is_shiny = false;

        // 从 allure 字段读取存储的性别和闪光状态
        // 编码格式: allure = (gender << 1) | (is_shiny ? 1 : 0)
        // gender: 0=雄性, 1=雌性
        // is_shiny: 0=普通, 1=闪光
        // 注意: allure=0 是有效值（雄性且不闪光），需要用 isset 检查
        if (isset($myusersdata['allure']) && $myusersdata['allure'] !== '') {
            $stored_attr = intval($myusersdata['allure']);
            $gender = ($stored_attr >> 1) & 1;
            $is_shiny = ($stored_attr & 1) === 1;
        } else {
            // 生成新的随机属性
            $sexrand = rand(1, 1000);
            if ($npc['sex'] > 0) {
                $gender = ($sexrand <= $npc['sex']) ? 0 : 1;
            }
            $is_shiny = (rand(1, 4096) === 1);

            // 存储到 allure 字段
            $attr_value = ($gender << 1) | ($is_shiny ? 1 : 0);
            DB::query(pm_sql("UPDATE " . pm_table('pm_usersdata') . " SET allure = %d WHERE uid = %d",
                $attr_value, $myusersdata['uid']));
        }

        // Boss 宠物名称添加前缀
        $pokemon_name = $npc['name'];
        if ($is_boss === true) {
            $pokemon_name = '[BOSS] ' . $pokemon_name;
        }

        $response['wild_pokemon'] = [
            'id' => (int)$myusersdata['npcid'],
            'name' => $pokemon_name,
            'level' => (int)$myusersdata['level'],
            'hp' => (int)$myusersdata['hp'],
            'max_hp' => (int)$myusersdata['hpg'],
            'gender' => $gender,
            'is_shiny' => $is_shiny,
            'is_boss' => $is_boss === true,
        ];

        // 如果是 Boss，添加倍率信息
        if ($is_boss === true) {
            $response['wild_pokemon']['boss_multiplier'] = $boss_multiplier !== null ? (float)$boss_multiplier : 1.0;
        }
    }

    return $response;
}

/**
 * 恢复战斗状态
 * 检查用户是否有进行中的战斗，如果有则返回战斗场景
 */
function api_recover_battle()
{
    require_login();

    global $_G;
    $uid = $_G['uid'];

    // 获取用户数据
    $myusersdata = api_my_usersdata($_G['uid']);

    if (!$myusersdata) {
        api_error('User data not found', 404);
    }

    // 检查是否有进行中的战斗
    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        // 没有进行中的战斗
        api_error('No active battle', 404);
    }

    // 获取用户当前的宝可梦
    $mypokemon = api_my_pokemon($_G['username']);

    if (!$mypokemon || $mypokemon['species_id'] <= 0) {
        api_error('Pokemon not found', 404);
    }

    // 引擎优先：从 pm_battle 恢复 Boss 标识与真实回合数（旧列不存这些信息）
    battle_ensure_tables();
    $engine_state = battle_load_active($uid, $myusersdata, $mypokemon);
    $is_boss = false;
    $boss_multiplier = 1.0;
    $engine_turn = null;
    if ($engine_state !== null) {
        $is_boss = ($engine_state['kind'] === 'boss');
        if (!empty($engine_state['sides']['enemy'])) {
            $boss_multiplier = (float)$engine_state['sides']['enemy'][0]['boss_multiplier'];
        }
        $engine_turn = (int)$engine_state['turn'];
    }

    // 构建战斗响应
    $response = build_battle_response($myusersdata, $mypokemon, null, $is_boss, $boss_multiplier);
    if ($engine_turn !== null) {
        $response['battle_turn'] = $engine_turn;
    }

    // 恢复的是一场当前宠物已倒下的战斗时，明确告知客户端需要换宠
    if ((int)$mypokemon['hp'] <= 0) {
        list(, $can_switch) = handle_my_pokemon_fainted($uid, intval($mypokemon['id']));
        // 战斗仍在（npcid>0），helper 若无替补会顺手清掉状态——此时响应降级为无战斗
        if ($can_switch) {
            $response['can_continue_switch'] = true;
        } else {
            $response = build_battle_response(api_my_usersdata($uid), $mypokemon);
        }
    }

    api_success($response);
}

/**
 * 获取地图列表
 */
function api_get_maps()
{
    require_login();

    global $_G;

    $min_level_filter = get_param('min_level', null);
    $max_level_filter = get_param('max_level', null);

    $where_clauses = ["is_enabled = 1"];
    $where_params = [];

    if ($min_level_filter !== null && is_numeric($min_level_filter)) {
        $min_level_filter = (int)$min_level_filter;
        $where_clauses[] = "max_level >= %d";
        $where_params[] = $min_level_filter;
    }

    if ($max_level_filter !== null && is_numeric($max_level_filter)) {
        $max_level_filter = (int)$max_level_filter;
        $where_clauses[] = "min_level <= %d";
        $where_params[] = $max_level_filter;
    }

    $where_sql = implode(' AND ', $where_clauses);

    // 使用 pm_table 获取正确的表名（不带 Discuz 前缀）
    $map_table = pm_table('pm_map');
    $data_table = pm_table('pm_data');

    // 检查表是否有新字段（静态缓存：同一请求内只探测一次，
    // 避免多次调用时重复 SHOW COLUMNS 往返）
    static $has_region_field = null;
    if ($has_region_field === null) {
        $has_region_field = false;
        try {
            $column_result = DB::fetch_first("SHOW COLUMNS FROM {$map_table} LIKE 'region'");
            if ($column_result) {
                $has_region_field = true;
            }
        } catch (Exception $e) {
            $has_region_field = false;
        }
    }

    // 根据字段情况选择查询
    if ($has_region_field) {
        $sql = "SELECT id, name, site, region, pos_x, pos_y, is_enabled, min_level, max_level, experience, boss_config
            FROM {$map_table}
            WHERE {$where_sql}
            ORDER BY region ASC, min_level ASC, id ASC";
    } else {
        $sql = "SELECT id, name, site, is_enabled, min_level, max_level, experience, boss_config
            FROM {$map_table}
            WHERE {$where_sql}
            ORDER BY min_level ASC, id ASC";
    }

    // 使用 pm_sql 来处理参数
    $final_sql = pm_sql_v($sql, $where_params);
    $maps_rows = DB::fetch_all($final_sql);

    // 一次查出全部种族的地图分布，再在 PHP 内按地图分组。
    // 此前对每张地图各执行一次 FIND_IN_SET 查询，地图越多越慢（N+1）。
    // mapid 为逗号分隔的整数串（如 "102,103"），与原 SQL 的
    // FIND_IN_SET/LIKE 组合等价；ORDER BY id 保持原先 LIMIT 10 的取序。
    $wild_by_map = [];
    $all_species = DB::fetch_all(
        "SELECT id, name, mapid FROM {$data_table} ORDER BY id ASC"
    );
    foreach ($all_species as $species) {
        $raw_mapid = isset($species['mapid']) ? trim((string)$species['mapid']) : '';
        if ($raw_mapid === '') {
            continue;
        }
        foreach (explode(',', $raw_mapid) as $map_token) {
            $map_token = trim($map_token);
            if ($map_token === '' || !ctype_digit($map_token)) {
                continue;
            }
            $wild_by_map[(int)$map_token][] = [
                'id' => (int)$species['id'],
                'name' => $species['name'],
            ];
        }
    }

    $maps = [];
    foreach ($maps_rows as $row) {
        $map_id = (int)$row['id'];

        $pokemon_names = array_slice(
            isset($wild_by_map[$map_id]) ? $wild_by_map[$map_id] : [],
            0,
            10
        );

        $area_type_name = translate_map_alpha_to_full_name($row['site']);

        // 根据地图名称推断区域
        $region = 'unknown';
        $pos_x = 50;
        $pos_y = 50;

        if ($has_region_field && isset($row['region']) && !empty($row['region'])) {
            // 直接使用数据库存储的region值（支持中英文）
            $region = $row['region'];
            $pos_x = isset($row['pos_x']) ? (int)$row['pos_x'] : 50;
            $pos_y = isset($row['pos_y']) ? (int)$row['pos_y'] : 50;
        } else {
            // 根据地图名称和site字段推断区域（返回中文region以兼容旧版Rust客户端）
            $map_name = $row['name'];
            $site = isset($row['site']) ? strtolower(trim($row['site'])) : '';

            // 首先根据地图名称推断 - 使用丰缘地区实际地理分布，坐标分散避免重叠
            if (strpos($map_name, '天元') !== false || strpos($map_name, '山木') !== false || strpos($map_name, '金水') !== false) {
                $region = '中央';
                $pos_x = 50;
                $pos_y = 50;
            } elseif (strpos($map_name, '104') !== false || strpos($map_name, '森林') !== false || strpos($map_name, '石之洞窟') !== false) {
                $region = '北部';
                $pos_x = 50;
                $pos_y = 20;
            } elseif (strpos($map_name, '110') !== false || strpos($map_name, '流星') !== false || strpos($map_name, '115') !== false) {
                $region = '东部';
                $pos_x = 80;
                $pos_y = 50;
            } elseif (strpos($map_name, '烟特') !== false || strpos($map_name, '凸凹') !== false || strpos($map_name, '日落') !== false) {
                $region = '南部';
                $pos_x = 50;
                $pos_y = 80;
            } elseif (strpos($map_name, '118') !== false || strpos($map_name, '121') !== false || strpos($map_name, '墓') !== false) {
                $region = '西部';
                $pos_x = 20;
                $pos_y = 50;
            } elseif (strpos($map_name, '水道') !== false || strpos($map_name, '海') !== false) {
                $region = '海洋';
                $pos_x = 85;
                $pos_y = 25;
            } elseif (strpos($map_name, '海底') !== false || strpos($map_name, '双鹿') !== false || strpos($map_name, '觉醒') !== false) {
                $region = '洞窟';
                $pos_x = 70;
                $pos_y = 70;
            } elseif (strpos($map_name, '火山') !== false || strpos($map_name, '殿元') !== false || strpos($map_name, '梦之') !== false) {
                $region = '特殊';
                $pos_x = 15;
                $pos_y = 25;
            } else {
                // 根据site字段推断区域和坐标 - 分散布局避免重叠
                switch ($site) {
                    case 's': // 海洋
                    case 'b': // 海底
                    case 'o': // 深海
                    case 'p': // 水池
                        $region = '海洋';
                        $pos_x = 85;
                        $pos_y = 25;
                        break;
                    case 'c': // 山洞
                        $region = '洞窟';
                        $pos_x = 70;
                        $pos_y = 70;
                        break;
                    case 'm': // 山谷
                        $region = '山脉';
                        $pos_x = 30;
                        $pos_y = 25;
                        break;
                    case 'h': // 天空
                        $region = '天空';
                        $pos_x = 50;
                        $pos_y = 12;
                        break;
                    case 'd': // 沙漠
                        $region = '沙漠';
                        $pos_x = 20;
                        $pos_y = 80;
                        break;
                    case 'f': // 工厂
                    case 't': // 基地
                    case 'v': // 市镇
                    case 'n': // 道馆
                        $region = '城市';
                        $pos_x = 60;
                        $pos_y = 55;
                        break;
                    case 'k': // 熔岩
                        $region = '火山';
                        $pos_x = 25;
                        $pos_y = 75;
                        break;
                    case 'g': // 草丛
                    case 'l': // 平原
                    default:
                        $region = '野外';
                        $pos_x = 38;
                        $pos_y = 38;
                        break;
                }
            }
        }

        // 解析地图模式配置
        // 使用 expn 字段判断地图模式：
        // - expn 是有效 Boss JSON -> Boss 模式
        // - 其他 -> 野生模式
        //
        // Rust 的 #[serde(tag = "mode")] 会生成扁平化格式：
        // Wild: {"mode":"wild"}
        // Boss: {"mode":"boss","bosses":[...]}
        $bosses = [];
        $expn_raw = isset($row['boss_config']) ? $row['boss_config'] : '';

        // 尝试解析 expn 为 Boss 配置
        $boss_json = json_decode($expn_raw, true);
        $has_boss_config = is_array($boss_json) && isset($boss_json['bosses']) && is_array($boss_json['bosses']);

        if ($has_boss_config) {
            foreach ($boss_json['bosses'] as $b) {
                if (!empty($b['pokemon_type_id'])) {
                    $bosses[] = [
                        'pokemon_type_id' => (int)$b['pokemon_type_id'],
                        'pokemon_name'    => isset($b['pokemon_name']) ? (string)$b['pokemon_name'] : '',
                        'level'           => isset($b['level']) ? (int)$b['level'] : 1,
                        'boss_multiplier' => isset($b['boss_multiplier']) ? (float)$b['boss_multiplier'] : 1.0,
                    ];
                }
            }
            usort($bosses, function ($a, $b) {
                return $a['level'] - $b['level'];
            });
        }

        // 确定地图模式并生成扁平化格式
        if ($has_boss_config) {
            // Boss 模式: {"mode":"boss","bosses":[...]}
            $maps[] = [
                'id' => $map_id,
                'name' => $row['name'],
                'area_type' => $row['site'],
                'area_type_name' => $area_type_name,
                'region' => $region,
                'pos_x' => $pos_x,
                'pos_y' => $pos_y,
                'is_enabled' => (bool)$row['is_enabled'],
                'min_level' => (int)$row['min_level'],
                'max_level' => (int)$row['max_level'],
                'mode' => 'boss',
                'bosses' => $bosses,
                'wild_pokemons' => $pokemon_names,
            ];
        } else {
            // 野生模式: {"mode":"wild"}
            $maps[] = [
                'id' => $map_id,
                'name' => $row['name'],
                'area_type' => $row['site'],
                'area_type_name' => $area_type_name,
                'region' => $region,
                'pos_x' => $pos_x,
                'pos_y' => $pos_y,
                'is_enabled' => (bool)$row['is_enabled'],
                'min_level' => (int)$row['min_level'],
                'max_level' => (int)$row['max_level'],
                'mode' => 'wild',
                'wild_pokemons' => $pokemon_names,
            ];
        }
    }

    api_success([
        'maps' => $maps,
        'total' => count($maps),
    ]);
}

/**
 * 地图类型字母转全名
 */
function translate_map_alpha_to_full_name($alpha)
{
    $map_types = [
        'l' => '平原',
        'g' => '草丛',
        'p' => '水池',
        's' => '海洋',
        'b' => '海底',
        'm' => '山谷',
        'c' => '山洞',
        'd' => '沙漠',
        'f' => '工厂',
        't' => '基地',
        'v' => '市镇',
        'n' => '道馆',
        'h' => '天空',
        'o' => '深海',
        'k' => '熔岩',
    ];

    $alpha = strtolower(trim($alpha));
    return isset($map_types[$alpha]) ? $map_types[$alpha] : '未知';
}

/**
 * 捕捉精灵（战斗引擎 2.0：敌方快照取自引擎状态，结算在一个事务内）
 */
function api_capture_pokemon()
{
    require_login();

    global $_G, $myusersdata, $mypokemon;

    $input = get_json_input();
    $ball_id = isset($input['ball_id']) ? intval($input['ball_id']) : 0;

    if ($ball_id <= 0) {
        api_error('缺少精灵球ID', 400);
    }

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        api_error('没有进行中的战斗', 400);
    }

    $debug_info = array(
        'received_ball_id' => $ball_id,
        'uid' => $_G['uid'],
    );

    // 检查用户是否拥有该精灵球
    // 明确指定字段以避免 id 字段冲突
    $sql = pm_sql("SELECT m.id as myitem_id, m.itemid, m.nums, m.uid,
        i.id as itemdata_id, i.name, i.type, i.captmax, i.ballid
        FROM " . pm_table('pm_myitem') . " m
        LEFT JOIN " . pm_table('pm_itemdata') . " i ON m.itemid = i.id
        WHERE m.uid = %d AND m.itemid = %s AND i.type = 2",
        $_G['uid'], strval($ball_id));

    $my_ball = DB::fetch_first($sql);

    $debug_info['query_result'] = $my_ball ? 'found' : 'not_found';

    if (!$my_ball || $my_ball['nums'] <= 0) {
        api_error('您没有该精灵球', 400, $debug_info);
    }

    battle_ensure_tables();

    // ===== 捕捉一个事务：扣球 / 插入宠物 / 战斗收尾原子化（#69-71 锁策略）=====
    try {
        DB::query("START TRANSACTION");
        DB::fetch_first(pm_sql(
            "SELECT uid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $_G['uid']
        ));

        // 引擎状态为权威源（敌方等级/HP/捕捉基率/性别闪光快照）
        $myusersdata = api_my_usersdata($_G['uid']);
        $state = battle_load_active($_G['uid'], $myusersdata, $mypokemon);
        if ($state === null) {
            pm_abort_battle_transaction('没有进行中的战斗', 400);
        }
        $enemy = $state['sides']['enemy'][0];
        $npc = pm_data($enemy['species_id']);
        if (!$npc) {
            pm_abort_battle_transaction('野怪数据异常', 500);
        }
        $npc_level = (int)$enemy['level'];
        $npc_hp = (int)$enemy['hp'];
        $npc_max_hp = (int)$enemy['stats']['max_hp'];

        // 检查箱子容量
        $pokemon_count = DB::result_first(pm_sql(
            "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = %d",
            $_G['uid']
        ));
        if ($pokemon_count >= $myusersdata['boxnum']) {
            pm_abort_battle_transaction('箱子容量不足，请扩展！');
        }

        // 检查等级限制
        if ($npc_level > (int)$mypokemon['level']) {
            pm_abort_battle_transaction('无法捕捉比自己强大的精灵');
        }

        // 计算捕捉率（公式与旧版一致；捕捉基率取自引擎快照，与镜像等价）
        $captmax = $my_ball['captmax'] ?: 1;
        $capture_rate = (($npc_max_hp * 3 - $npc_hp * 2) * $enemy['capture_rate'] * $captmax) / ($npc_max_hp * 3);
        // 下限 1：capture 为 0 时避免后续 16711680/$capture_rate 除零
        $capture_rate = max(1, min(255, $capture_rate));

        $shake_check = intval(1048560 / pow(16711680 / $capture_rate, 0.25));

        $captured = false;
        if ($capture_rate >= 255) {
            $captured = true;
        } else {
            $all_pass = true;
            for ($i = 0; $i < 4; $i++) {
                if (rand(0, 65535) > $shake_check) {
                    $all_pass = false;
                    break;
                }
            }
            $captured = $all_pass;
        }

        $message = '';
        $status = 'active';
        $battle_ended = false;
        $can_switch = false;
        $turn_events = [];

        // 扣除精灵球（使用正确的 myitem_id）
        if ($my_ball['nums'] == 1) {
            DB::query(pm_sql("DELETE FROM " . pm_table('pm_myitem') . " WHERE id = %d", intval($my_ball['myitem_id'])));
        } else {
            DB::query(pm_sql("UPDATE " . pm_table('pm_myitem') . " SET nums = nums - 1 WHERE id = %d", intval($my_ball['myitem_id'])));
        }

        if ($captured) {
            $status = 'captured';
            $battle_ended = true;
            $message = "捕捉成功！{$npc['name']}已经被你收服了！";

            // 生成随机IV值
            $hpg = rand(0, 31);
            $atkg = rand(0, 31);
            $defg = rand(0, 31);
            $spatkg = rand(0, 31);
            $spdefg = rand(0, 31);
            $sdg = rand(0, 31);

            // 保留遭遇时的性别和闪光属性（引擎快照；性别 0/1 -> 宠物表 1/2，无性别种族 0）
            $sex = $npc['sex'] < 0 ? 0 : (((int)$enemy['gender']) & 1) + 1;
            $is_shiny = $enemy['is_shiny'] ? 1 : 0;

            // 检查是否有首位宠物
            $has_first = DB::result_first(pm_sql(
                "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = %d AND site = 1",
                $_G['uid']
            ));

            // 确定位置
            if ($has_first == 0) {
                // 没有首位宠物，新捕捉的宠物成为首位
                $site = 1;
            } else {
                $active_count = DB::result_first(pm_sql(
                    "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = %d AND site < 3",
                    $_G['uid']
                ));
                $site = $active_count >= 6 ? 3 : 2;
            }

            // 捕获保留野生等级，经验必须同步取该等级在经验表中的下限；
            // 写 0 会让 exp 低于当前等级门槛，经验条 saturating_sub 后永远显示 0
            require_once __DIR__ . '/pokemon_utils.php';
            $initial_exp = calculate_initial_exp((int) $npc['id'], (int) $npc_level);

            // 插入新宠物（itemevolve 是 pm_mypm 的列而 pm_data 没有，固定写 0）
            DB::query(pm_sql("INSERT INTO " . pm_table('pm_mypm') . "
                (uid, pmname, nickname, species_id, level, exp, sex, is_shiny, sx, hp,
                 hpg, atkg, defg, spatkg, spdefg, sdg,
                 good, itemevolve, ballid, site, state, statetime, gduptime, initialuid)
            VALUES (
                %d, %s, %s, %d, %d, %d, %d, %d, %s,
                %d, %d, %d, %d, %d, %d, %d,
                70, 0, %d, %d, 1, %d, %d, %d
            )",
                $_G['uid'], $npc['name'], $npc['name'], $npc['id'], $npc_level, $initial_exp, $sex, $is_shiny, $npc['xs'],
                $npc_hp, $hpg, $atkg, $defg, $spatkg, $spdefg, $sdg,
                $my_ball['ballid'], $site, time(), time(), $_G['uid']
            ));

            // 引擎收尾：battle_end(captured) 事件 + ended 落库，镜像清零
            $state['phase'] = 'ended';
            $state['result'] = 'captured';
            battle_core_emit($state, $turn_events, 'battle_end', ['result' => 'captured']);
            battle_persist_state($state, $turn_events);
            clear_battle_state($_G['uid']);
        } else {
            $message = "捕捉失败！精灵球没有命中...";

            // 野怪反击（引擎核心）
            $resolved = battle_resolve_engine_counter($_G['uid'], $mypokemon);
            if ($resolved === null) {
                pm_abort_battle_transaction('没有进行中的战斗', 400);
            }
            list($state, $turn_events, $status, $battle_ended, $can_switch) = $resolved;

            $rendered = battle_render_counter_messages($turn_events, $mypokemon, $npc['name'], $can_switch);
            if ($rendered) {
                $message .= "\n" . implode("\n", $rendered);
            }
        }

        // 落库前的野怪显示快照（captured 后镜像清零，响应组装时临时恢复）
        $display_npc = array(
            'npcid' => (int)$enemy['species_id'],
            'level' => (int)$enemy['level'],
            'hp' => $captured ? 0 : (int)$enemy['hp'],
            'hpg' => (int)$enemy['stats']['max_hp'],
        );

        DB::query("COMMIT");
    } catch (Throwable $txn_error) {
        DB::query("ROLLBACK");
        throw $txn_error;
    }

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    if ($status !== 'active' && $display_npc) {
        // 临时恢复野怪信息以便正确显示（旧版模式）
        $myusersdata['npcid'] = $display_npc['npcid'];
        $myusersdata['level'] = $display_npc['level'];
        $myusersdata['hp'] = $display_npc['hp'];
        $myusersdata['hpg'] = $display_npc['hpg'];
    }

    $battle = build_battle_response($myusersdata, $mypokemon);
    $battle['status'] = $status;
    $battle['message'] = $message;
    $battle['turn'] = 0;
    $battle['battle_over'] = ($status === 'captured') ? true : $battle_ended;
    $battle['can_continue_switch'] = $can_switch;
    $battle['battle_turn'] = intval($state['turn']);
    $battle['events'] = array();
    foreach ($turn_events as $e) {
        $battle['events'][] = array('turn' => $e['turn'], 'seq' => $e['seq'], 'type' => $e['type'], 'payload' => $e['payload']);
    }

    api_success($battle);
}

function api_use_item_in_battle()
{
    require_login();

    global $_G, $myusersdata, $mypokemon;

    $input = get_json_input();
    $item_id = isset($input['item_id']) ? intval($input['item_id']) : 0;

    if ($item_id <= 0) {
        api_error('缺少物品ID', 400);
    }

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        api_error('没有进行中的战斗', 400);
    }

    // 获取物品数据
    $item_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE id = %d",
        $item_id
    ));

    if (!$item_data) {
        api_error('物品不存在', 404);
    }

    // 检查用户是否拥有该物品
    $item_sql = pm_sql("SELECT * FROM " . pm_table('pm_myitem') . " WHERE uid = %d AND itemid = %s",
        $_G['uid'], strval($item_id));
    $my_item = DB::fetch_first($item_sql);

    if (!$my_item || $my_item['nums'] <= 0) {
        api_error('您没有该物品', 400);
    }

    $item_type = $item_data['type'];
    // 旧数据 module 列为空，模块名在 sitemname/tpname 中，需回退读取
    $item_module = api_get_item_module($item_data);
    $message = '';
    $status = 'active';
    $battle_ended = false;
    $can_switch = false;

    // PP 恢复道具：迁移数据中 type=1（回复药），必须先于类型分支按模块路由，
    // 否则会被回复药分支当作 0 点回复消耗掉
    if (in_array($item_module, ['pp5', 'pp10', 'pp15', 'pp99'])) {
        $skills_raw = DB::fetch_all(pm_sql(
            "SELECT ms.id, ms.skillid, ms.skillnum, s.max_uses as max_pp, s.name as skill_name
             FROM " . pm_table('pm_myskill') . " ms
             LEFT JOIN " . pm_table('pm_skill') . " s ON ms.skillid = s.id
             WHERE ms.uid = %d AND ms.petid = %d AND ms.skillnum < s.max_uses
             ORDER BY ms.id",
            $_G['uid'],
            $mypokemon['id']
        ));

        $available_skills = [];
        foreach ($skills_raw as $skill) {
            $available_skills[] = [
                'id' => (int)$skill['id'],
                'skill_id' => (int)$skill['skillid'],
                'name' => $skill['skill_name'],
                'current_pp' => (int)$skill['skillnum'],
                'max_pp' => (int)$skill['max_pp'],
            ];
        }

        if (empty($available_skills)) {
            api_error('所有技能PP都已满', 400);
        }

        // 返回技能列表，需要用户选择
        api_success([
            'requires_skill_selection' => true,
            'item_id' => $item_id,
            'item_name' => $item_data['name'],
            'available_skills' => $available_skills,
            'message' => '请选择要恢复PP的技能',
        ]);
        return;
    }

    battle_ensure_tables();

    // ===== 使用与反击一个事务（#69-71 锁策略）=====
    try {
        DB::query("START TRANSACTION");
        DB::fetch_first(pm_sql(
            "SELECT uid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $_G['uid']
        ));

    // 根据物品类型处理
    switch ($item_type) {
        case '1': // 回复药
            $effects = json_decode($item_data['effects'] ?? '{}', true) ?: [];
            $addhp = intval($effects['hp'] ?? 0);
            $max_hp = api_calculate_pokemon_max_hp($mypokemon);
            $current_hp = intval($mypokemon['hp']);

            if ($current_hp >= $max_hp && $addhp > 0) {
                pm_abort_battle_transaction("{$mypokemon['nickname']}不需要回复HP");
            }

            $new_hp = min($max_hp, $current_hp + $addhp);

            // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
            $mypokemon['hp'] = strval($new_hp);
            $hp_validation = api_validate_and_correct_hp($mypokemon, $new_hp, $max_hp);
            $new_hp = $hp_validation['hp'];

            DB::query(pm_sql(
                "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
                intval($new_hp),
                intval($mypokemon['id'])
            ));

            $message = "成功对{$mypokemon['nickname']}使用了{$item_data['name']}，恢复了{$addhp}点HP！";
            break;

        case '2': // 精灵球
            pm_abort_battle_transaction('精灵球请通过捕捉功能使用');
            break;

        case '4': // 强化道具（PP恢复类已在函数开头按模块路由）
            pm_abort_battle_transaction('该物品无法在战斗中使用');
            break;

        default:
            pm_abort_battle_transaction('该物品无法在战斗中使用');
    }

    // 扣除物品
    if ($my_item['nums'] == 1) {
        DB::query(pm_sql("DELETE FROM " . pm_table('pm_myitem') . " WHERE id = %d", intval($my_item['id'])));
    } else {
        DB::query(pm_sql("UPDATE " . pm_table('pm_myitem') . " SET nums = nums - 1 WHERE id = %d", intval($my_item['id'])));
    }

    // 野怪反击（引擎核心；治疗已先行落库，反击从治疗后的 HP 起算）
    $resolved = battle_resolve_engine_counter($_G['uid'], $mypokemon);
    if ($resolved === null) {
        pm_abort_battle_transaction('没有进行中的战斗', 400);
    }
    list($state, $turn_events, $resolved_status, $battle_ended, $can_switch) = $resolved;
    if ($resolved_status === 'defeat') {
        $status = 'defeat';
    }
    $npc_name = $state['sides']['enemy'][0]['species_name'];
    $rendered = battle_render_counter_messages($turn_events, $mypokemon, $npc_name, $can_switch);
    if ($rendered) {
        $message .= "\n" . implode("\n", $rendered);
    }

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    $battle = build_battle_response($myusersdata, $mypokemon);
    $battle['status'] = $status;
    $battle['message'] = $message;
    $battle['turn'] = 0;
    $battle['battle_over'] = $battle_ended;
    $battle['can_continue_switch'] = $can_switch;
    $battle['battle_turn'] = intval($state['turn']);
    $battle['events'] = array();
    foreach ($turn_events as $e) {
        $battle['events'][] = array('turn' => $e['turn'], 'seq' => $e['seq'], 'type' => $e['type'], 'payload' => $e['payload']);
    }

    DB::query("COMMIT");
    } catch (Throwable $txn_error) {
        DB::query("ROLLBACK");
        throw $txn_error;
    }

    api_success($battle);
}

/**
 * 在战斗中对指定技能使用物品（PP 恢复，引擎反击，一个事务）
 */
function api_use_item_on_skill_in_battle()
{
    require_login();

    global $_G, $myusersdata, $mypokemon;

    $input = get_json_input();
    $item_id = isset($input['item_id']) ? intval($input['item_id']) : 0;
    $skill_record_id = isset($input['skill_record_id']) ? intval($input['skill_record_id']) : 0;

    if ($item_id <= 0) {
        api_error('缺少物品ID', 400);
    }
    if ($skill_record_id <= 0) {
        api_error('缺少技能记录ID', 400);
    }

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        api_error('没有进行中的战斗', 400);
    }

    // 获取物品数据
    $item_data = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_itemdata') . " WHERE id = %d",
        $item_id
    ));

    if (!$item_data) {
        api_error('物品不存在', 404);
    }

    // 检查用户是否拥有该物品
    $item_sql = pm_sql("SELECT * FROM " . pm_table('pm_myitem') . " WHERE uid = %d AND itemid = %s",
        $_G['uid'], strval($item_id));
    $my_item = DB::fetch_first($item_sql);

    if (!$my_item || $my_item['nums'] <= 0) {
        api_error('您没有该物品', 400);
    }

    // 旧数据 module 列为空，模块名在 sitemname/tpname 中，需回退读取
    $item_module = api_get_item_module($item_data);

    // 验证是否是PP恢复物品
    $pp_amount = 0;
    switch ($item_module) {
        case 'pp5': $pp_amount = 5; break;
        case 'pp10': $pp_amount = 10; break;
        case 'pp15': $pp_amount = 15; break;
        case 'pp99': $pp_amount = 999; break;
        default:
            api_error('该物品不支持技能选择使用', 400);
    }

    battle_ensure_tables();

    // ===== 恢复与反击一个事务（#69-71 锁策略）=====
    try {
        DB::query("START TRANSACTION");
        DB::fetch_first(pm_sql(
            "SELECT uid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $_G['uid']
        ));

        // 获取技能信息
        $my_skill = DB::fetch_first(pm_sql(
            "SELECT ms.*, s.max_uses as max_pp
             FROM " . pm_table('pm_myskill') . " ms
             LEFT JOIN " . pm_table('pm_skill') . " s ON ms.skillid = s.id
             WHERE ms.id = %d AND ms.uid = %d AND ms.petid = %d",
            $skill_record_id,
            $_G['uid'],
            $mypokemon['id']
        ));

        if (!$my_skill) {
            pm_abort_battle_transaction('技能不存在', 404);
        }

        $max_pp = (int)$my_skill['max_pp'];
        $current_pp = (int)$my_skill['skillnum'];

        if ($current_pp >= $max_pp) {
            pm_abort_battle_transaction('该技能PP已满');
        }

        $restore_amount = $pp_amount >= 999 ? ($max_pp - $current_pp) : min($pp_amount, $max_pp - $current_pp);
        $new_pp = $current_pp + $restore_amount;

        // 恢复PP
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_myskill') . " SET skillnum = %d WHERE id = %d",
            $new_pp,
            $skill_record_id
        ));

        // 扣除物品
        if ($my_item['nums'] == 1) {
            DB::query(pm_sql("DELETE FROM " . pm_table('pm_myitem') . " WHERE id = %d", intval($my_item['id'])));
        } else {
            DB::query(pm_sql("UPDATE " . pm_table('pm_myitem') . " SET nums = nums - 1 WHERE id = %d", intval($my_item['id'])));
        }

        // 野怪反击（引擎核心）
        $resolved = battle_resolve_engine_counter($_G['uid'], $mypokemon);
        if ($resolved === null) {
            pm_abort_battle_transaction('没有进行中的战斗', 400);
        }
        list($state, $turn_events, $status, $battle_ended, $can_switch) = $resolved;

        $message = "成功使用{$item_data['name']}，恢复了{$restore_amount}点PP！";
        $npc_name = $state['sides']['enemy'][0]['species_name'];
        $rendered = battle_render_counter_messages($turn_events, $mypokemon, $npc_name, $can_switch);
        if ($rendered) {
            $message .= "\n" . implode("\n", $rendered);
        }

        $myusersdata = api_my_usersdata($_G['uid']);
        $mypokemon = api_my_pokemon($_G['username']);

        $battle = build_battle_response($myusersdata, $mypokemon);
        $battle['status'] = $status;
        $battle['message'] = $message;
        $battle['turn'] = 0;
        $battle['battle_over'] = $battle_ended;
        $battle['can_continue_switch'] = $can_switch;
        $battle['battle_turn'] = intval($state['turn']);
        $battle['events'] = array();
        foreach ($turn_events as $e) {
            $battle['events'][] = array('turn' => $e['turn'], 'seq' => $e['seq'], 'type' => $e['type'], 'payload' => $e['payload']);
        }

        DB::query("COMMIT");
    } catch (Throwable $txn_error) {
        DB::query("ROLLBACK");
        throw $txn_error;
    }

    api_success($battle);
}

function api_get_battle_items()
{
    require_login();

    global $_G;
    $uid = validate_uid($_G['uid']);

    // 旧库可能缺 module 列，先自愈；SELECT 亦不引用该列，api_get_item_module
    // 会从行数据的 sitemname/tpname 回退解析模块名
    api_ensure_itemdata_module_column();

    // 获取用户的物品
    $my_items = DB::fetch_all(pm_sql(
        "SELECT mi.*, i.type, i.sitemname, i.effects, i.name, i.tpname
         FROM " . pm_table('pm_myitem') . " mi
         INNER JOIN " . pm_table('pm_itemdata') . " i ON mi.itemid = i.id
         WHERE mi.uid = %d AND mi.nums > 0
         ORDER BY i.type, i.id",
        $uid
    ));

    $battle_items = [];
    $pp_restore_modules = ['pp5', 'pp10', 'pp15', 'pp99'];

    foreach ($my_items as $item) {
        $item_type = $item['type'];
        // 旧数据 module 列为空，模块名在 sitemname/tpname 中，需回退读取
        $item_module = api_get_item_module($item);
        $effects = json_decode($item['effects'] ?? '{}', true) ?: [];
        $heal_hp = intval($effects['hp'] ?? 0);

        // 只返回可以在战斗中使用的物品
        // 1. PP恢复道具（按模块识别；迁移数据中其 type=1 而非 4）
        if (in_array($item_module, $pp_restore_modules)) {
            $battle_items[] = [
                'id' => (int) $item['itemid'],
                'name' => $item['name'],
                'img' => $item['tpname'],
                'nums' => (int) $item['nums'],
                'item_type' => (int) $item_type,
                'module' => $item_module,
            ];
        }
        // 2. HP恢复药水（type=1，有hp效果，且非PP道具）
        elseif ($item_type == '1' && $heal_hp > 0) {
            $battle_items[] = [
                'id' => (int) $item['itemid'],
                'name' => $item['name'],
                'img' => $item['tpname'],
                'nums' => (int) $item['nums'],
                'item_type' => (int) $item_type,
                'module' => $item_module,
                'addhp' => $heal_hp,
            ];
        }
    }

    api_success(['items' => $battle_items]);
}

/**
 * 切换上场宠物
 * 用户在战斗中可以切换宠物，但有被反击的风险
 * 支持可选的 pokemon_id 参数来指定要切换的宠物
 */
function api_switch_pokemon()
{
    require_login();

    global $_G, $myusersdata, $mypokemon;

    $input = get_json_input();
    $pokemon_id = isset($input['pokemon_id']) ? intval($input['pokemon_id']) : 0;

    $myusersdata = api_my_usersdata($_G['uid']);

    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        api_error('没有进行中的战斗', 400);
    }

    // 与被动替换相同的串行化：以 pm_usersdata 战斗状态行的排他锁串行化
    // 同账号的换宠与战斗结算，两笔 site 写入与野怪反击在同一个事务里生效，
    // 观察者不会看到「换了一半」或反击打到已经下场的宠物。
    battle_ensure_tables(); // 事务前建表：CREATE TABLE 在事务内会触发隐式提交（clear_battle_state 的联动更新需要 pm_battle）
    DB::query("START TRANSACTION");
    // 事务体内的任何异常（含数据库错误）都要显式回滚：常驻 worker 的连接
    // 不随请求关闭，未提交事务和行锁泄漏会阻塞该用户后续的所有战斗操作。
    // api_error() 走 exit 语义，其回滚由 pm_abort_battle_transaction() 负责。
    $battle = null;
    try {
        $battle_owner = DB::fetch_first(pm_sql(
            "SELECT uid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $_G['uid']
        ));
        if (!$battle_owner) {
            pm_abort_battle_transaction('没有进行中的战斗');
        }

        // 锁内重读：并发请求可能已在本请求预检后改变了战斗或上场宠物
        $myusersdata = api_my_usersdata($_G['uid']);
        if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
            pm_abort_battle_transaction('没有进行中的战斗');
        }
        $mypokemon = api_my_pokemon($_G['username']);
        if (!$mypokemon) {
            pm_abort_battle_transaction('没有上场宠物');
        }

        $current_pet_id = intval($mypokemon['id']);

        // 如果指定了 pokemon_id，验证该宠物是否可用
        if ($pokemon_id > 0) {
            // 检查指定的宠物是否存在且可用
            $specified_pet = DB::fetch_first(pm_sql(
                "SELECT * FROM " . pm_table('pm_mypm') . "
                WHERE uid = %d AND id = %d AND site < 3 AND hp > 0 AND state != 0",
                $_G['uid'],
                $pokemon_id
            ));

            if (!$specified_pet) {
                pm_abort_battle_transaction('指定的宠物不可用');
            }

            if ($specified_pet['id'] == $current_pet_id) {
                pm_abort_battle_transaction('不能切换到当前上场的宠物');
            }

            $next_pokemon = $specified_pet;
        } else {
            // 没有指定 pokemon_id，自动选择第一个可用宠物
            // 获取用户的所有宠物（按 site 排序，site=1 是首发，site=2 是替补，site=3 是箱子）
            $all_pokemon_rows = DB::fetch_all(pm_sql(
                "SELECT * FROM " . pm_table('pm_mypm') . "
                WHERE uid = %d AND site < 3 AND hp > 0 AND state != 0
                ORDER BY site ASC, id ASC",
                $_G['uid']
            ));

            $available_pokemon = [];

            foreach ($all_pokemon_rows as $pet) {
                $pet_id = intval($pet['id']);
                // 排除当前上场的宠物
                if ($pet_id !== $current_pet_id) {
                    $available_pokemon[] = $pet;
                }
            }

            if (empty($available_pokemon)) {
                pm_abort_battle_transaction('没有可用的替补宠物');
            }

            // 选择第一个可用的宠物（通常是 site=2 的第一个）
            $next_pokemon = $available_pokemon[0];
        }

        // 交换 site 值：当前宠物变为 site=2，新宠物变为 site=1。
        // 两笔写入都带资格条件（同被动替换）：读到宠物之后、写入之前，
        // 宠物可能被并发操作装箱、打倒或移动；升位失败会整体回滚。
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . "
SET site = 2 WHERE id = %d AND uid = %d AND site = 1",
            $current_pet_id,
            $_G['uid']
        ));
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . "
SET site = 1 WHERE id = %d AND uid = %d AND site < 3 AND hp > 0 AND state != 0",
            intval($next_pokemon['id']),
            $_G['uid']
        ));
        if (!DB::affected_rows()) {
            pm_abort_battle_transaction($pokemon_id > 0 ? '指定的宠物不可用' : '没有可用的替补宠物');
        }

        // 引擎侧记录换人（switch_in 事件 + 新宠物入单位位；反击在下方按概率结算）
        $state = battle_load_active($_G['uid'], $myusersdata, $next_pokemon);
        if ($state !== null) {
            $next_pokemon_data = pm_data($next_pokemon['species_id']);
            if ($next_pokemon_data) {
                battle_inject_ally_fresh_state($state, $next_pokemon, $next_pokemon_data);
                if ($state['phase'] === 'awaiting_switch') {
                    $state['phase'] = 'active';
                }
                $switch_events = [];
                battle_core_emit($state, $switch_events, 'switch_in', ['side' => 'ally', 'slot' => 0, 'instance_id' => intval($next_pokemon['id'])]);
                battle_persist_state($state, $switch_events);
            }
        }

        // 野怪反击（30%概率），与新宠物的血量结算同处一个事务（引擎核心）
        $counter_message = "\n野怪没有反应过来！";

        if (rand(1, 100) <= 30) {
            $resolved = battle_resolve_engine_counter($_G['uid'], $next_pokemon);
            if ($resolved !== null) {
                list($state, $turn_events, $resolved_status, $battle_ended, $can_switch) = $resolved;
                $npc_name = $state['sides']['enemy'][0]['species_name'];
                $rendered = battle_render_counter_messages($turn_events, $next_pokemon, $npc_name, $can_switch);
                $counter_message = "\n野怪抓住了机会！" . ($rendered ? implode("\n", $rendered) : '');

                if ($resolved_status === 'defeat') {
                    if ($can_switch) {
                        // 还有可用宠物，保留战斗状态让前端处理替换
                        $new_mypokemon = api_my_pokemon($_G['username']);
                        $battle = build_battle_response($myusersdata, $new_mypokemon);
                        $battle['status'] = 'active';
                        $battle['message'] = "成功切换为 {$next_pokemon['nickname']}！" . $counter_message;
                        $battle['turn'] = 0;
                        $battle['battle_over'] = false;
                        $battle['can_continue_switch'] = true;
                    } else {
                        // 没有可用替换宠物，战斗失败（resolve 已 persist ended/defeat 并清镜像）
                        $new_mypokemon = api_my_pokemon($_G['username']);
                        $new_myusersdata = api_my_usersdata($_G['uid']);

                        // 临时设置野怪数据以便 build_battle_response 能正确构建响应
                        $new_myusersdata['npcid'] = $myusersdata['npcid'];
                        $new_myusersdata['hp'] = $myusersdata['hp'];
                        $new_myusersdata['hpg'] = $myusersdata['hpg'];
                        $new_myusersdata['level'] = $myusersdata['level'];

                        $battle = build_battle_response($new_myusersdata, $new_mypokemon);
                        $battle['status'] = 'defeat';
                        $battle['message'] = "成功切换为 {$next_pokemon['nickname']}！" . $counter_message;
                        $battle['turn'] = 0;
                        $battle['battle_over'] = true;
                        $battle['can_continue_switch'] = false;
                    }
                }
            }
        }

        if ($battle === null) {
            // 重新获取数据构建响应
            $new_mypokemon = api_my_pokemon($_G['username']);
            $battle = build_battle_response($myusersdata, $new_mypokemon);
            $battle['status'] = 'active';
            $battle['message'] = "成功切换为 {$next_pokemon['nickname']}！" . $counter_message;
            $battle['turn'] = 0;
            $battle['battle_over'] = false;
            $battle['can_continue_switch'] = false;
        }

        DB::query("COMMIT");
    } catch (Throwable $txn_error) {
        DB::query("ROLLBACK");
        throw $txn_error;
    }

    api_success($battle);
}

/**
 * 被动替换上场宠物（宠物被打死后替换，不反击）
 * 用于野怪攻击后我方宠物倒下的情况
 */
function api_replace_pokemon()
{
    require_login();

    global $_G, $myusersdata, $mypokemon;

    $input = get_json_input();
    $pokemon_id = isset($input['pokemon_id']) ? intval($input['pokemon_id']) : 0;

    $myusersdata = api_my_usersdata($_G['uid']);

    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        api_error('没有进行中的战斗', 400);
    }

    // 以战斗状态行（pm_usersdata，每用户一行）的排他锁串行化同账号的替换：
    // 两个并发替换只有一个能完成换位，另一个在锁内重读时会发现上场宠物
    // 已被换成健康的新宠物而走「尚未倒下」拒绝；野怪的战斗结算（如
    // clear_battle_state）也要先写这一行，同样被此锁挡在事务之外。
    battle_ensure_tables(); // 事务前建表：CREATE TABLE 在事务内会触发隐式提交（clear_battle_state 的联动更新需要 pm_battle）
    DB::query("START TRANSACTION");
    // 事务体内的任何异常（含数据库错误）都要显式回滚：常驻 worker 的连接
    // 不随请求关闭，未提交事务和行锁泄漏会阻塞该用户后续的所有战斗操作。
    // api_error() 走 exit 语义，其回滚由 pm_abort_battle_transaction() 负责。
    try {
        $battle_owner = DB::fetch_first(pm_sql(
            "SELECT uid FROM " . pm_table('pm_usersdata') . " WHERE uid = %d FOR UPDATE",
            $_G['uid']
        ));
        if (!$battle_owner) {
            pm_abort_battle_transaction('没有进行中的战斗');
        }

        // 锁内重读：并发请求可能已在本请求预检后改变了战斗或上场宠物
        $myusersdata = api_my_usersdata($_G['uid']);
        if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
            pm_abort_battle_transaction('没有进行中的战斗');
        }
        $mypokemon = api_my_pokemon($_G['username']);
        if (!$mypokemon) {
            pm_abort_battle_transaction('没有上场宠物');
        }
        if ($mypokemon['hp'] > 0) {
            pm_abort_battle_transaction('当前宠物尚未倒下，请使用主动切换');
        }

        $current_pet_id = intval($mypokemon['id']);

        // 如果指定了 pokemon_id，验证该宠物是否可用
        if ($pokemon_id > 0) {
            // 检查指定的宠物是否存在且可用
            $specified_pet = DB::fetch_first(pm_sql(
                "SELECT * FROM " . pm_table('pm_mypm') . "
                WHERE uid = %d AND id = %d AND site < 3 AND hp > 0 AND state != 0",
                $_G['uid'],
                $pokemon_id
            ));

            if (!$specified_pet) {
                pm_abort_battle_transaction('指定的宠物不可用');
            }

            if ($specified_pet['id'] == $current_pet_id) {
                pm_abort_battle_transaction('不能切换到当前上场的宠物');
            }

            $next_pokemon = $specified_pet;
        } else {
            // 没有指定 pokemon_id，自动选择第一个可用宠物
            $all_pokemon_rows = DB::fetch_all(pm_sql(
                "SELECT * FROM " . pm_table('pm_mypm') . "
                WHERE uid = %d AND site < 3 AND hp > 0 AND state != 0
                ORDER BY site ASC, id ASC",
                $_G['uid']
            ));

            $available_pokemon = [];

            foreach ($all_pokemon_rows as $pet) {
                $pet_id = intval($pet['id']);
                // 排除当前上场的宠物
                if ($pet_id !== $current_pet_id) {
                    $available_pokemon[] = $pet;
                }
            }

            if (empty($available_pokemon)) {
                pm_abort_battle_transaction('没有可用的替补宠物');
            }

            $next_pokemon = $available_pokemon[0];
        }

        // 交换 site 值：当前宠物变为 site=2，新宠物变为 site=1。
        // 两笔写入都带资格条件：读到宠物之后、写入之前，宠物可能被并发操作
        // 装箱、打倒或移动；条件不满足则 0 行受影响，升位失败会整体回滚。
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . "
SET site = 2 WHERE id = %d AND uid = %d AND site = 1",
            $current_pet_id,
            $_G['uid']
        ));
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . "
SET site = 1 WHERE id = %d AND uid = %d AND site < 3 AND hp > 0 AND state != 0",
            intval($next_pokemon['id']),
            $_G['uid']
        ));
        if (!DB::affected_rows()) {
            pm_abort_battle_transaction($pokemon_id > 0 ? '指定的宠物不可用' : '没有可用的替补宠物');
        }

        // 引擎侧记录替补上场（switch_in 事件 + 新宠物入单位位，战斗回到 active）
        $myusersdata_for_engine = api_my_usersdata($_G['uid']);
        $state = battle_load_active($_G['uid'], $myusersdata_for_engine, $next_pokemon);
        if ($state !== null) {
            $next_pokemon_data = pm_data($next_pokemon['species_id']);
            if ($next_pokemon_data) {
                battle_inject_ally_fresh_state($state, $next_pokemon, $next_pokemon_data);
                if ($state['phase'] === 'awaiting_switch') {
                    $state['phase'] = 'active';
                }
                $switch_events = [];
                battle_core_emit($state, $switch_events, 'switch_in', ['side' => 'ally', 'slot' => 0, 'instance_id' => intval($next_pokemon['id'])]);
                battle_persist_state($state, $switch_events);
            }
        }

        DB::query("COMMIT");
    } catch (Throwable $txn_error) {
        DB::query("ROLLBACK");
        throw $txn_error;
    }

    // 被动切换不进行野怪反击，直接返回响应
    $new_mypokemon = api_my_pokemon($_G['username']);
    $battle = build_battle_response($myusersdata, $new_mypokemon);
    $battle['status'] = 'active';
    $battle['message'] = "成功切换为 {$next_pokemon['nickname']}！";
    $battle['turn'] = 0;
    $battle['battle_over'] = false;
    $battle['can_continue_switch'] = false;

    api_success($battle);
}
