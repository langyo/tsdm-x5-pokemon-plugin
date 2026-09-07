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
 */

// 加载 API 辅助函数（包含 get_param, api_error 等）
require_once __DIR__ . '/index.php';

// 加载常量定义
require_once __DIR__ . '/constants.php';

global $_G;

// 初始化全局配置
$settings = isset($_G['cache']['plugin']['pokemon']) ? $_G['cache']['plugin']['pokemon'] : array();

$action = get_param('action', '');

// maps 接口不需要加载额外的依赖
if ($action === 'maps') {
    api_get_maps();
    exit;
}

// 以下接口需要加载战斗相关依赖
// 加载 API 工具函数（包含经验计算、状态修正等）
require_once __DIR__ . '/utils.php';

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

    // 保存战斗状态到 pm_usersdata（不存储 is_boss 和 boss_multiplier，这些是运行时状态）
    DB::query(pm_sql("UPDATE " . pm_table('pm_usersdata') . " SET
        npcid=%d,
        level=%d,
        hp=%d,
        hpg=%d,
        atkg=%d,
        defg=%d,
        spatkg=%d,
        spdefg=%d,
        sdg=%d,
        capture=%d,
        allure=%d
        WHERE uid=%d",
        $wild['npcid'],
        $wild['level'],
        $npcmhp,
        $npcmhp,
        $npcatk,
        $npcdef,
        $npcspatk,
        $npcspdef,
        $npcsd,
        $wild['capture'],
        $allure_value,
        $_G['uid']
    ));

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
 * 使用技能攻击
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

    // 获取地图信息
    $map = null;

    // 获取野怪信息
    $npcid = $myusersdata['npcid'];
    $npc_level = $myusersdata['level'];
    $npc_hp = $myusersdata['hp'];
    $npc_max_hp = $myusersdata['hpg'];
    $npc = pm_data($npcid);

    // 获取我方宠物基础数据
    $mydata = pm_data($mypokemon['species_id']);

    // 计算属性（已包含装备加成）
    list($mpmhp, $matk, $mdef, $mspatk, $mspdef, $msd) = battle_calc_my_stats($mydata, $mypokemon);

    list($npcmhp, $npcatk, $npcdef, $npcspatk, $npcspdef, $npcsd) = battle_calc_npc_stats(
        $npc,
        $myusersdata,
        $myusersdata['strength'] * $npc['strength']
    );

    // 获取技能信息
    $skillname = '普通攻击';
    $power = 30;
    $skill_type = $mydata['xs'];
    $skill_category = 0;

    if ($skill_id > 0) {
        $skilldata = DB::fetch_first(pm_sql("SELECT * FROM " . pm_table('pm_skill') . " WHERE id = %d", $skill_id));
        if ($skilldata) {
            $skillname = $skilldata['name'];
            $power = intval($skilldata['power']) ?: 40;
            // pm_skill 的属性列是 element（曾误用不存在的 sx 列导致技能属性恒为宠物自身属性）
            $skill_type = $skilldata['element'] ?: $mydata['xs'];
            // pm_skill.category 存中文（'物攻'/'特攻'），intval 恒为 0，需按字符串判断
            $skill_category = api_normalize_skill_category($skilldata['category']);

            // 检查PP值
            $myskill = DB::fetch_first(pm_sql(
                "SELECT * FROM " . pm_table('pm_myskill') . "
                WHERE skillid = %d AND uid = %d AND petid = %d",
                $skill_id, $_G['uid'], $mypokemon['id']
            ));

            if ($myskill && $myskill['skillnum'] <= 0 && $skilldata['max_uses'] != 0) {
                api_error('Skill PP is depleted', 400);
            }
        }
    }

    // 决定先手
    $my_first = ($msd >= $npcsd);

    $damage_log = [];
    $battle_status = 'active';
    $rewards = null;
    $level_up_info = null;

    // 我方攻击
    if ($my_first) {
        $damage = calculate_damage_legacy(
            $mypokemon['level'],
            $matk,
            $npcdef,
            $mspatk,
            $npcspdef,
            $power,
            $skill_type,
            $skill_category,
            $mydata,
            $npc
        );

        // 检查闪避
        if (($npcsd - $msd) >= 10 && rand(1, 20) <= 4) {
            $damage_log[] = "{$npc['name']}避开了{$mypokemon['nickname']}的攻击！";
        } else {
            $npc_hp -= $damage;
            if ($npc_hp < 0) $npc_hp = 0;
            $damage_log[] = "{$mypokemon['nickname']}使用了{$skillname}，对{$npc['name']}造成了{$damage}点伤害！";

            // 扣除PP
            if ($skill_id > 0 && $myskill && $skilldata['max_uses'] != 0) {
                DB::query(pm_sql("UPDATE " . pm_table('pm_myskill') . "
                    SET skillnum = skillnum - 1
                    WHERE skillid = %d AND uid = %d AND petid = %d",
                    $skill_id, $_G['uid'], $mypokemon['id']
                ));
            }
        }

        // 检查野怪是否倒下
        if ($npc_hp <= 0) {
            $battle_status = 'victory';
            $damage_log[] = "{$npc['name']}倒下了！";

            // 计算奖励
            $rewards = calculate_rewards($mypokemon, $myusersdata, $npc, $npc_level, $map);

            // 保存野怪信息，用于胜利响应
            $victory_npc_id = $myusersdata['npcid'];
            $victory_npc_name = $npc['name'];
            $victory_npc_level = $myusersdata['level'];
            $victory_npc_hp = 0;
            $victory_npc_max_hp = $myusersdata['hpg'];

            // 清理战斗状态
            clear_battle_state($_G['uid']);

            // 应用奖励并获取升级信息
            $level_up_info = apply_rewards($_G['uid'], $mypokemon, $rewards);

            // 重新获取数据
            $myusersdata = api_my_usersdata($_G['uid']);
            $mypokemon = api_my_pokemon($_G['username']);

            // 构建响应（临时恢复野怪信息以便正确显示）
            $myusersdata['npcid'] = $victory_npc_id;
            $myusersdata['level'] = $victory_npc_level;
            $myusersdata['hp'] = $victory_npc_hp;
            $myusersdata['hpg'] = $victory_npc_max_hp;

            $battle = build_battle_response($myusersdata, $mypokemon);
            $battle['status'] = $battle_status;
            $battle['message'] = implode("\n", $damage_log);
            $battle['turn'] = 0;

            if ($rewards) {
                $battle['rewards'] = $rewards;
            }

            if ($level_up_info && $level_up_info['level_up']) {
                $battle['level_up'] = $level_up_info;
                $pokemon_name = $mypokemon['nickname'] ?: $mypokemon['pmname'];
                $battle['message'] .= "\n🎉 {$pokemon_name}升级了！Lv.{$level_up_info['old_level']} → Lv.{$level_up_info['new_level']}";
            }

            api_success($battle);
            return;
        }
    }

    // 野怪攻击（如果我方没赢且不是我先手，或者我方先手但野怪没死）
    if ($battle_status === 'active') {
        if (!$my_first) {
            // 野怪先攻击
            $counter_damage = calculate_counter_damage_legacy(
                $npc_level,
                $npcatk,
                $mdef,
                $npcspatk,
                $mspdef,
                $npc
            );

            $my_hp = $mypokemon['hp'] - $counter_damage;
            if ($my_hp < 0) $my_hp = 0;

            $damage_log[] = "{$npc['name']}攻击了{$mypokemon['nickname']}，造成了{$counter_damage}点伤害！";

            // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
            $mypokemon['hp'] = strval($my_hp);
            $max_hp_for_validate = api_calculate_pokemon_max_hp($mypokemon);
            $hp_validation = api_validate_and_correct_hp($mypokemon, $my_hp, $max_hp_for_validate);
            $my_hp = $hp_validation['hp'];

            // 更新我方HP
            DB::query(pm_sql(
                "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
                intval($my_hp),
                intval($mypokemon['id'])
            ));

            if ($my_hp <= 0) {
                $battle_status = 'defeat';
                $damage_log[] = "{$mypokemon['nickname']}倒下了...";

                // 检查是否还有可用的替补宠物
                $available_count = DB::result_first(pm_sql(
                    "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . "
                    WHERE uid = %d AND site < 3 AND hp > 0 AND state != 0",
                    $_G['uid']
                ));

                // 只有当没有可用替补时才清除战斗状态
                if ($available_count == 0) {
                    clear_battle_state($_G['uid']);
                }
            }
        }

        // 我方后手攻击
        if (!$my_first && $battle_status === 'active') {
            $damage = calculate_damage_legacy(
                $mypokemon['level'],
                $matk,
                $npcdef,
                $mspatk,
                $npcspdef,
                $power,
                $skill_type,
                $skill_category,
                $mydata,
                $npc
            );

            if (($npcsd - $msd) >= 10 && rand(1, 20) <= 4) {
                $damage_log[] = "{$npc['name']}避开了{$mypokemon['nickname']}的攻击！";
            } else {
                $npc_hp -= $damage;
                if ($npc_hp < 0) $npc_hp = 0;
                $damage_log[] = "{$mypokemon['nickname']}使用了{$skillname}，对{$npc['name']}造成了{$damage}点伤害！";

                if ($skill_id > 0 && $myskill && $skilldata['max_uses'] != 0) {
                    DB::query(pm_sql("UPDATE " . pm_table('pm_myskill') . "
                        SET skillnum = skillnum - 1
                        WHERE skillid = %d AND uid = %d AND petid = %d",
                        $skill_id, $_G['uid'], $mypokemon['id']
                    ));
                }
            }

            if ($npc_hp <= 0) {
                $battle_status = 'victory';
                $damage_log[] = "{$npc['name']}倒下了！";

                // 保存野怪信息，用于胜利响应
                $victory_npc_id = $myusersdata['npcid'];
                $victory_npc_name = $npc['name'];
                $victory_npc_level = $myusersdata['level'];
                $victory_npc_hp = 0;
                $victory_npc_max_hp = $myusersdata['hpg'];

                $rewards = calculate_rewards($mypokemon, $myusersdata, $npc, $npc_level, $map);
                clear_battle_state($_G['uid']);
                $level_up_info = apply_rewards($_G['uid'], $mypokemon, $rewards);

                // 重新获取数据
                $myusersdata = api_my_usersdata($_G['uid']);
                $mypokemon = api_my_pokemon($_G['username']);

                // 构建响应（临时恢复野怪信息以便正确显示）
                $myusersdata['npcid'] = $victory_npc_id;
                $myusersdata['level'] = $victory_npc_level;
                $myusersdata['hp'] = $victory_npc_hp;
                $myusersdata['hpg'] = $victory_npc_max_hp;

                $battle = build_battle_response($myusersdata, $mypokemon);
                $battle['status'] = $battle_status;
                $battle['message'] = implode("\n", $damage_log);
                $battle['turn'] = 0;

                if ($rewards) {
                    $battle['rewards'] = $rewards;
                }

                if ($level_up_info && $level_up_info['level_up']) {
                    $battle['level_up'] = $level_up_info;
                    $pokemon_name = $mypokemon['nickname'] ?: $mypokemon['pmname'];
                    $battle['message'] .= "\n🎉 {$pokemon_name}升级了！Lv.{$level_up_info['old_level']} → Lv.{$level_up_info['new_level']}";
                }

                api_success($battle);
                return;
            }
        }

        // 我方先手后野怪反击
        if ($my_first && $battle_status === 'active') {
            $counter_damage = calculate_counter_damage_legacy(
                $npc_level,
                $npcatk,
                $mdef,
                $npcspatk,
                $mspdef,
                $npc
            );

            $my_hp = $mypokemon['hp'] - $counter_damage;
            if ($my_hp < 0) $my_hp = 0;

            $damage_log[] = "{$npc['name']}攻击了{$mypokemon['nickname']}，造成了{$counter_damage}点伤害！";

            // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
            $mypokemon['hp'] = strval($my_hp);
            $max_hp_for_validate = api_calculate_pokemon_max_hp($mypokemon);
            $hp_validation = api_validate_and_correct_hp($mypokemon, $my_hp, $max_hp_for_validate);
            $my_hp = $hp_validation['hp'];

            DB::query(pm_sql(
                "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
                intval($my_hp),
                intval($mypokemon['id'])
            ));

            if ($my_hp <= 0) {
                $battle_status = 'defeat';
                $damage_log[] = "{$mypokemon['nickname']}倒下了...";

                // 检查是否还有可用的替补宠物
                $available_count = DB::result_first(pm_sql(
                    "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . "
                    WHERE uid = %d AND site < 3 AND hp > 0 AND state != 0",
                    $_G['uid']
                ));

                // 只有当没有可用替补时才清除战斗状态
                if ($available_count == 0) {
                    clear_battle_state($_G['uid']);
                }
            }
        }
    }

    // 更新野怪HP
    if ($battle_status === 'active') {
        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_usersdata') . " SET hp = %d WHERE uid = %d",
            intval($npc_hp),
            intval($_G['uid'])
        ));
    }

    // 重新获取数据
    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    // 如果战斗结束（胜利/失败），需要恢复野怪信息以便正确显示
    if ($battle_status !== 'active') {
        // 保存野怪信息（使用之前保存的值或当前值）
        if (!isset($victory_npc_id)) {
            $victory_npc_id = $myusersdata['npcid'] ?: $npcid;
            $victory_npc_level = $myusersdata['level'] ?: $npc_level;
            $victory_npc_hp = $myusersdata['hp'];
            $victory_npc_max_hp = $myusersdata['hpg'];
        }

        // 临时恢复野怪信息
        $myusersdata['npcid'] = $victory_npc_id;
        $myusersdata['level'] = $victory_npc_level;
        $myusersdata['hp'] = isset($victory_npc_hp) ? $victory_npc_hp : 0;
        $myusersdata['hpg'] = $victory_npc_max_hp;
    }

    // 构建响应
    $battle = build_battle_response($myusersdata, $mypokemon);
    $battle['status'] = $battle_status;
    $battle['message'] = implode("\n", $damage_log);
    $battle['turn'] = ($battle_status === 'active') ? 1 : 0;

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
 */
function api_flee()
{
    require_login();

    global $_G, $myusersdata, $mypokemon;

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        api_error('No active battle found', 400);
    }

    $npc = pm_data($myusersdata['npcid']);

    // 计算逃跑成功率
    $flee_chance = 0.5 + ($mypokemon['level'] - $myusersdata['level']) * 0.05;
    $flee_chance = max(0.1, min(0.9, $flee_chance));

    $message = '';
    $status = 'active';

    if (rand(1, 100) / 100 <= $flee_chance) {
        $status = 'fled';
        $message = '成功逃脱了！';

        // 保存野怪信息，用于逃脱响应
        $flee_npc_id = $myusersdata['npcid'];
        $flee_npc_name = $npc['name'];
        $flee_npc_level = $myusersdata['level'];
        $flee_npc_hp = $myusersdata['hp'];
        $flee_npc_max_hp = $myusersdata['hpg'];

        clear_battle_state($_G['uid']);

        // 重新获取数据
        $myusersdata = api_my_usersdata($_G['uid']);
        $mypokemon = api_my_pokemon($_G['username']);

        // 构建响应（临时恢复野怪信息以便正确显示）
        $myusersdata['npcid'] = $flee_npc_id;
        $myusersdata['level'] = $flee_npc_level;
        $myusersdata['hp'] = $flee_npc_hp;
        $myusersdata['hpg'] = $flee_npc_max_hp;

        $battle = build_battle_response($myusersdata, $mypokemon);
        $battle['status'] = $status;
        $battle['message'] = $message;
        $battle['turn'] = 0;

        api_success($battle);
        return;
    } else {
        $message = '逃跑失败！';

        // 野怪攻击
        list($npcmhp, $npcatk, $npcdef, $npcspatk, $npcspdef, $npcsd) = battle_calc_npc_stats(
            $npc,
            $myusersdata,
            $myusersdata['strength'] * $npc['strength']
        );

        $mydata = pm_data($mypokemon['species_id']);
        list(, $matk, $mdef, $mspatk, $mspdef, $msd) = battle_calc_my_stats($mydata, $mypokemon);

        $counter_damage = calculate_counter_damage_legacy(
            $myusersdata['level'],
            $npcatk,
            $mdef,
            $npcspatk,
            $mspdef,
            $npc
        );

        $my_hp = $mypokemon['hp'] - $counter_damage;
        if ($my_hp < 0) $my_hp = 0;

        // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
        $mypokemon['hp'] = strval($my_hp);
        $max_hp_for_validate = api_calculate_pokemon_max_hp($mypokemon);
        $hp_validation = api_validate_and_correct_hp($mypokemon, $my_hp, $max_hp_for_validate);
        $my_hp = $hp_validation['hp'];

        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
            intval($my_hp),
            intval($mypokemon['id'])
        ));

        $message .= "\n{$npc['name']}攻击了{$mypokemon['nickname']}，造成了{$counter_damage}点伤害！";

        if ($my_hp <= 0) {
            $status = 'defeat';
            $message .= "\n{$mypokemon['nickname']}倒下了...";

            // 保存野怪信息，用于失败响应
            $defeat_npc_id = $myusersdata['npcid'];
            $defeat_npc_name = $npc['name'];
            $defeat_npc_level = $myusersdata['level'];
            $defeat_npc_hp = $myusersdata['hp'];
            $defeat_npc_max_hp = $myusersdata['hpg'];

            clear_battle_state($_G['uid']);

            // 重新获取数据
            $myusersdata = api_my_usersdata($_G['uid']);
            $mypokemon = api_my_pokemon($_G['username']);

            // 构建响应（临时恢复野怪信息以便正确显示）
            $myusersdata['npcid'] = $defeat_npc_id;
            $myusersdata['level'] = $defeat_npc_level;
            $myusersdata['hp'] = $defeat_npc_hp;
            $myusersdata['hpg'] = $defeat_npc_max_hp;

            $battle = build_battle_response($myusersdata, $mypokemon);
            $battle['status'] = $status;
            $battle['message'] = $message;
            $battle['turn'] = 0;

            api_success($battle);
            return;
        }
    }

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    $battle = build_battle_response($myusersdata, $mypokemon);
    $battle['status'] = $status;
    $battle['message'] = $message;
    $battle['turn'] = 0;

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
 * 计算伤害（旧版公式）
 */
function calculate_damage_legacy($level, $atk, $def, $spatk, $spdef, $power, $skill_type, $category, $attacker_data, $defender_data)
{
    // 基础伤害
    if ($category != 1) {
        $damage = (($level * 0.4 + 2) * $power * $atk / $def / 50 + 2);
    } else {
        $damage = (($level * 0.4 + 2) * $power * $spatk / $spdef / 50 + 2);
    }

    // 属性相克
    $xs2 = isset($defender_data['xs2']) ? $defender_data['xs2'] : '';
    $boost = get_pet_type_effectiveness($skill_type, $defender_data['xs'], $xs2);
    $damage *= $boost;

    // 属性一致加成
    $xs2_attacker = isset($attacker_data['xs2']) ? $attacker_data['xs2'] : '';
    if (in_array($skill_type, [$attacker_data['xs'], $xs2_attacker])) {
        $damage *= 1.5;
    }

    // 随机因子
    $damage *= rand(85, 100) / 100;

    // 会心一击
    if (rand(1, 20) == 1) {
        $damage *= 2;
    }

    return max(1, floor($damage));
}

/**
 * 计算反击伤害
 */
function calculate_counter_damage_legacy($level, $atk, $def, $spatk, $spdef, $npc)
{
    $power = 40;
    $skill_type = $npc['xs'];

    $damage = (($level * 0.4 + 2) * $power * $atk / $def / 50 + 2);
    $damage *= rand(85, 100) / 100;

    return max(1, floor($damage));
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
 * 计算野怪（已保存战斗状态）六维属性
 *
 * pm_usersdata 的 hpg/atkg/defg/spatkg/spdefg/sdg 在开战时已由
 * battle_calc_new_npc_stats 算好并写入（含 strength 倍率），这里直接读取即可。
 * 旧实现把这些完整属性当作 IV 再套一遍成长公式，导致野怪攻防每回合虚高约三成。
 *
 * @return array [hp, atk, def, spatk, spdef, sd]
 */
function battle_calc_npc_stats($data, $saved_state, $strength = 1)
{
    return [
        intval($saved_state['hpg']),
        intval($saved_state['atkg']),
        intval($saved_state['defg']),
        intval($saved_state['spatkg']),
        intval($saved_state['spdefg']),
        intval($saved_state['sdg']),
    ];
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
 * 计算属性相克加成
 *
 * @param string $attack_type 攻击技能属性
 * @param string $defender_type1 防御方第一属性
 * @param string $defender_type2 防御方第二属性（可选）
 * @return float 属性相克倍率（0=免疫, 0.5=效果不好, 1=正常, 2=效果拔群）
 */
function get_pet_type_effectiveness($attack_type, $defender_type1, $defender_type2 = '')
{
    // 属性相克表：攻击属性 => [克制属性, 被克制属性, 无效属性]
    $type_chart = [
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

    $total = 1.0;

    // 检查第一属性
    $total *= calculate_type_match($attack_type, $defender_type1, $type_chart);

    // 检查第二属性
    if (!empty($defender_type2)) {
        $total *= calculate_type_match($attack_type, $defender_type2, $type_chart);
    }

    return $total;
}

/**
 * 计算单个属性相克倍率
 */
function calculate_type_match($attack_type, $defend_type, $type_chart)
{
    if (!isset($type_chart[$attack_type])) {
        return 1.0;
    }

    $chart = $type_chart[$attack_type];

    // 检查免疫
    if (in_array($defend_type, $chart['immune'])) {
        return 0.0;
    }

    // 检查被克制（效果不好）
    if (in_array($defend_type, $chart['resisted'])) {
        return 0.5;
    }

    // 检查克制（效果拔群）
    if (in_array($defend_type, $chart['effective'])) {
        return 2.0;
    }

    return 1.0;
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
    // 这些列均为整数类型：写入 '' 在 MariaDB 严格模式(STRICT_TRANS_TABLES)下会直接报错，
    // 导致战斗状态无法清除（用户卡在战斗中），必须写 0
    DB::query(pm_sql("UPDATE " . pm_table('pm_usersdata') . "
        SET npcid = 0, level = 0, hp = 0, hpg = 0, atkg = 0, defg = 0,
            spatkg = 0, spdefg = 0, sdg = 0, allure = 0, capture = 0
        WHERE uid = %d", $uid));
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

    // 构建战斗响应
    $response = build_battle_response($myusersdata, $mypokemon);
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

    // 检查表是否有新字段
    $has_region_field = false;
    try {
        $column_result = DB::fetch_first("SHOW COLUMNS FROM {$map_table} LIKE 'region'");
        if ($column_result) {
            $has_region_field = true;
        }
    } catch (Exception $e) {
        $has_region_field = false;
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

    $maps = [];
    foreach ($maps_rows as $row) {
        $map_id = (int)$row['id'];

        $map_id_str = strval($map_id);
        $pokemon_rows = DB::fetch_all(pm_sql(
            "SELECT id, name FROM {$data_table}
            WHERE FIND_IN_SET(%d, mapid) > 0
               OR mapid LIKE CONCAT('%%,', %s, ',%%')
               OR mapid LIKE CONCAT(%s, ',%%')
               OR mapid LIKE CONCAT('%%,', %s)
               OR mapid = %d
            LIMIT 10",
            $map_id, $map_id_str, $map_id_str, $map_id_str, $map_id
        ));

        $pokemon_names = [];
        foreach ($pokemon_rows as $pokemon) {
            $pokemon_names[] = [
                'id' => (int)$pokemon['id'],
                'name' => $pokemon['name']
            ];
        }

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
 * 捕捉精灵
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

    // 获取野怪数据
    $npc = pm_data($myusersdata['npcid']);
    $npc_level = $myusersdata['level'];
    $npc_hp = $myusersdata['hp'];

    // 计算野怪最大HP
    list($npc_max_hp,,,,,) = battle_calc_npc_stats(
        $npc,
        $myusersdata,
        $myusersdata['strength'] * $npc['strength']
    );

    // 检查箱子容量
    $pokemon_count = DB::result_first(pm_sql(
        "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . " WHERE uid = %d",
        $_G['uid']
    ));
    if ($pokemon_count >= $myusersdata['boxnum']) {
        api_error('箱子容量不足，请扩展！', 400);
    }

    // 检查等级限制
    if ($npc_level > $mypokemon['level']) {
        api_error('无法捕捉比自己强大的精灵', 400);
    }

    // 计算捕捉率
    $captmax = $my_ball['captmax'] ?: 1;
    $capture_rate = (($npc_max_hp * 3 - $npc_hp * 2) * $myusersdata['capture'] * $captmax) / ($npc_max_hp * 3);
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

    // 扣除精灵球（使用正确的 myitem_id）
    if ($my_ball['nums'] == 1) {
        DB::query(pm_sql("DELETE FROM " . pm_table('pm_myitem') . " WHERE id = %d", intval($my_ball['myitem_id'])));
    } else {
        DB::query(pm_sql("UPDATE " . pm_table('pm_myitem') . " SET nums = nums - 1 WHERE id = %d", intval($my_ball['myitem_id'])));
    }

    if ($captured) {
        $status = 'captured';
        $message = "捕捉成功！{$npc['name']}已经被你收服了！";

        // 生成随机IV值
        $hpg = rand(0, 31);
        $atkg = rand(0, 31);
        $defg = rand(0, 31);
        $spatkg = rand(0, 31);
        $spdefg = rand(0, 31);
        $sdg = rand(0, 31);

        // 随机性别
        $sexrand = rand(1, 1000);
        if ($sexrand <= $npc['sex']) {
            $sex = 1;
        } elseif ($npc['sex'] < 0) {
            $sex = 0;
        } else {
            $sex = 2;
        }

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

        // 插入新宠物（itemevolve 是 pm_mypm 的列而 pm_data 没有，固定写 0）
        DB::query(pm_sql("INSERT INTO " . pm_table('pm_mypm') . "
            (uid, pmname, nickname, species_id, level, exp, sex, sx, hp,
             hpg, atkg, defg, spatkg, spdefg, sdg,
             good, itemevolve, ballid, site, state, statetime, gduptime, initialuid)
            VALUES (
                %d, %s, %s, %d, %d, 0, %d, %s,
                %d, %d, %d, %d, %d, %d, %d,
                70, 0, %d, %d, 1, %d, %d, %d
            )",
            $_G['uid'], $npc['name'], $npc['name'], $npc['id'], $npc_level, $sex, $npc['xs'],
            $npc_hp, $hpg, $atkg, $defg, $spatkg, $spdefg, $sdg,
            $my_ball['ballid'], $site, time(), time(), $_G['uid']
        ));

        clear_battle_state($_G['uid']);
    } else {
        $message = "捕捉失败！精灵球没有命中...";

        // 野怪反击
        list(, $npcatk,, $npcspatk,, $npcsd) = battle_calc_npc_stats(
            $npc,
            $myusersdata,
            $myusersdata['strength'] * $npc['strength']
        );

        $mydata = pm_data($mypokemon['species_id']);
        list(,, $mdef,, $mspdef, $msd) = battle_calc_my_stats($mydata, $mypokemon);

        $counter_damage = calculate_counter_damage_legacy(
            $npc_level,
            $npcatk,
            $mdef,
            $npcspatk,
            $mspdef,
            $npc
        );

        $my_hp = $mypokemon['hp'] - $counter_damage;
        if ($my_hp < 0) $my_hp = 0;

        // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
        $mypokemon['hp'] = strval($my_hp);
        $max_hp_for_validate = api_calculate_pokemon_max_hp($mypokemon);
        $hp_validation = api_validate_and_correct_hp($mypokemon, $my_hp, $max_hp_for_validate);
        $my_hp = $hp_validation['hp'];

        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
            intval($my_hp),
            intval($mypokemon['id'])
        ));

        $message .= "\n{$npc['name']}攻击了{$mypokemon['nickname']}，造成了{$counter_damage}点伤害！";

        if ($my_hp <= 0) {
            $status = 'defeat';
            $message .= "\n{$mypokemon['nickname']}倒下了...";
            clear_battle_state($_G['uid']);
        }
    }

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    $battle = build_battle_response($myusersdata, $mypokemon);
    $battle['status'] = $status;
    $battle['message'] = $message;
    $battle['turn'] = 0;

    api_success($battle);
}

/**
 * 在战斗中使用物品（回复药等）
 */
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

    // 根据物品类型处理
    switch ($item_type) {
        case '1': // 回复药
            $effects = json_decode($item_data['effects'] ?? '{}', true) ?: [];
            $addhp = intval($effects['hp'] ?? 0);
            $max_hp = api_calculate_pokemon_max_hp($mypokemon);
            $current_hp = intval($mypokemon['hp']);

            if ($current_hp >= $max_hp && $addhp > 0) {
                api_error("{$mypokemon['nickname']}不需要回复HP", 400);
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
            api_error('精灵球请通过捕捉功能使用', 400);
            break;

        case '4': // 强化道具（PP恢复类已在函数开头按模块路由）
            api_error('该物品无法在战斗中使用', 400);
            break;

        default:
            api_error('该物品无法在战斗中使用', 400);
    }

    // 扣除物品
    if ($my_item['nums'] == 1) {
        DB::query(pm_sql("DELETE FROM " . pm_table('pm_myitem') . " WHERE id = %d", intval($my_item['id'])));
    } else {
        DB::query(pm_sql("UPDATE " . pm_table('pm_myitem') . " SET nums = nums - 1 WHERE id = %d", intval($my_item['id'])));
    }

    // 野怪反击
    $npc = pm_data($myusersdata['npcid']);
    list(, $npcatk,, $npcspatk,,) = battle_calc_npc_stats(
        $npc,
        $myusersdata,
        $myusersdata['strength'] * $npc['strength']
    );

    $mydata = pm_data($mypokemon['species_id']);
    list(,, $mdef,, $mspdef,) = battle_calc_my_stats($mydata, $mypokemon);

    $counter_damage = calculate_counter_damage_legacy(
        $myusersdata['level'],
        $npcatk,
        $mdef,
        $npcspatk,
        $mspdef,
        $npc
    );

    $my_hp = $new_hp - $counter_damage;
    if ($my_hp < 0) $my_hp = 0;

    // 验证并纠正 HP（确保 HP 在 [0, max_hp] 范围内）
    $mypokemon['hp'] = strval($my_hp);
    $max_hp_for_validate = api_calculate_pokemon_max_hp($mypokemon);
    $hp_validation = api_validate_and_correct_hp($mypokemon, $my_hp, $max_hp_for_validate);
    $my_hp = $hp_validation['hp'];

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
        intval($my_hp),
        intval($mypokemon['id'])
    ));

    $message .= "\n{$npc['name']}攻击了{$mypokemon['nickname']}，造成了{$counter_damage}点伤害！";

    if ($my_hp <= 0) {
        $status = 'defeat';
        $message .= "\n{$mypokemon['nickname']}倒下了...";
        clear_battle_state($_G['uid']);
    }

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    $battle = build_battle_response($myusersdata, $mypokemon);
    $battle['status'] = $status;
    $battle['message'] = $message;
    $battle['turn'] = 0;

    api_success($battle);
}

/**
 * 在战斗中对指定技能使用物品（PP恢复等）
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
        api_error('缺少技能ID', 400);
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
        api_error('技能不存在', 404);
    }

    $max_pp = (int)$my_skill['max_pp'];
    $current_pp = (int)$my_skill['skillnum'];

    if ($current_pp >= $max_pp) {
        api_error('该技能PP已满', 400);
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

    // 野怪反击
    $npc = pm_data($myusersdata['npcid']);
    list(, $npcatk,, $npcspatk,,) = battle_calc_npc_stats(
        $npc,
        $myusersdata,
        $myusersdata['strength'] * $npc['strength']
    );

    $mydata = pm_data($mypokemon['species_id']);
    list(,, $mdef,, $mspdef,) = battle_calc_my_stats($mydata, $mypokemon);

    $counter_damage = calculate_counter_damage_legacy(
        $myusersdata['level'],
        $npcatk,
        $mdef,
        $npcspatk,
        $mspdef,
        $npc
    );

    $my_hp = $mypokemon['hp'] - $counter_damage;
    if ($my_hp < 0) $my_hp = 0;

    // 验证并纠正 HP
    $mypokemon['hp'] = strval($my_hp);
    $max_hp_for_validate = api_calculate_pokemon_max_hp($mypokemon);
    $hp_validation = api_validate_and_correct_hp($mypokemon, $my_hp, $max_hp_for_validate);
    $my_hp = $hp_validation['hp'];

    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
        intval($my_hp),
        intval($mypokemon['id'])
    ));

    $message = "成功使用{$item_data['name']}，恢复了{$restore_amount}点PP！\n{$npc['name']}攻击了{$mypokemon['nickname']}，造成了{$counter_damage}点伤害！";
    $status = 'active';

    if ($my_hp <= 0) {
        $status = 'defeat';
        $message .= "\n{$mypokemon['nickname']}倒下了...";
        clear_battle_state($_G['uid']);
    }

    $myusersdata = api_my_usersdata($_G['uid']);
    $mypokemon = api_my_pokemon($_G['username']);

    $battle = build_battle_response($myusersdata, $mypokemon);
    $battle['status'] = $status;
    $battle['message'] = $message;
    $battle['turn'] = 0;

    api_success($battle);
}

/**
 * 获取可以在战斗中使用的物品
 * 只返回直接恢复HP和PP的物品
 */
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
    $mypokemon = api_my_pokemon($_G['username']);

    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        api_error('没有进行中的战斗', 400);
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
            api_error('指定的宠物不可用', 400);
        }

        if ($specified_pet['id'] == $current_pet_id) {
            api_error('不能切换到当前上场的宠物', 400);
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
            api_error('没有可用的替补宠物', 400);
        }

        // 选择第一个可用的宠物（通常是 site=2 的第一个）
        $next_pokemon = $available_pokemon[0];
    }

    // 交换 site 值：当前宠物变为 site=2，新宠物变为 site=1
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET site = 2 WHERE id = %d",
        $current_pet_id
    ));
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET site = 1 WHERE id = %d",
        intval($next_pokemon['id'])
    ));

    // 野怪反击（30%概率）
    $npc = pm_data($myusersdata['npcid']);
    $counter_damage = 0;
    $counter_message = '';

    if (rand(1, 100) <= 30) {
        list(, $npcatk,, $npcspatk,,) = battle_calc_npc_stats(
            $npc,
            $myusersdata,
            $myusersdata['strength'] * $npc['strength']
        );

        // 使用新宠物的数据计算防御（而不是旧宠物）
        $next_pokemon_data = pm_data($next_pokemon['species_id']);
        list(, $next_hp,, $next_mdef,, $next_mspdef,) = battle_calc_my_stats($next_pokemon_data, $next_pokemon);

        $counter_damage = calculate_counter_damage_legacy(
            $myusersdata['level'],
            $npcatk,
            $next_mdef,
            $npcspatk,
            $next_mspdef,
            $npc
        );

        // 使用新宠物的 HP 作为起点
        $my_hp = $next_hp - $counter_damage;
        if ($my_hp < 0) $my_hp = 0;

        // 验证并纠正 HP
        $next_pokemon['hp'] = strval($my_hp);
        $max_hp_for_validate = api_calculate_pokemon_max_hp($next_pokemon);
        $hp_validation = api_validate_and_correct_hp($next_pokemon, $my_hp, $max_hp_for_validate);
        $my_hp = $hp_validation['hp'];

        DB::query(pm_sql(
            "UPDATE " . pm_table('pm_mypm') . " SET hp = %d WHERE id = %d",
            intval($my_hp),
            intval($next_pokemon['id'])
        ));

        $counter_message = "\n野怪抓住了机会！{$npc['name']}攻击了{$next_pokemon['nickname']}，造成了{$counter_damage}点伤害！";

        if ($my_hp <= 0) {
            // 新宠物倒下，但不立即结束战斗
            // 检查是否还有可用替换宠物
            $has_replacements = DB::result(pm_sql(
                "SELECT COUNT(*) FROM " . pm_table('pm_mypm') . "
                WHERE uid = %d AND site < 3 AND hp > 0 AND state != 0 AND id != %d",
                $_G['uid'],
                intval($next_pokemon['id'])
            )) > 0;

            if ($has_replacements) {
                // 还有可用宠物，保留战斗状态让前端处理替换
                // 不调用 clear_battle_state，让战斗继续
                $new_mypokemon = api_my_pokemon($_G['username']);
                $battle = build_battle_response($myusersdata, $new_mypokemon);
                $battle['status'] = 'active';
                $battle['message'] = "成功切换为 {$next_pokemon['nickname']}！" . $counter_message . "\n{$next_pokemon['nickname']}倒下了...";
                $battle['turn'] = 0;

                api_success($battle);
            } else {
                // 没有可用替换宠物，战斗失败
                // 先获取当前的野怪数据，用于构建最终响应
                $final_npc_id = $myusersdata['npcid'];
                $final_npc_hp = $myusersdata['hp'];
                $final_npc_max_hp = $myusersdata['hpg'];
                $final_npc_level = $myusersdata['level'];

                clear_battle_state($_G['uid']);

                // 重新获取新宠物数据构建响应
                $new_mypokemon = api_my_pokemon($_G['username']);
                $new_myusersdata = api_my_usersdata($_G['uid']);

                // 临时设置野怪数据以便 build_battle_response 能正确构建响应
                $new_myusersdata['npcid'] = $final_npc_id;
                $new_myusersdata['hp'] = $final_npc_hp;
                $new_myusersdata['hpg'] = $final_npc_max_hp;
                $new_myusersdata['level'] = $final_npc_level;

                $battle = build_battle_response($new_myusersdata, $new_mypokemon);
                $battle['status'] = 'defeat';
                $battle['message'] = "成功切换为 {$next_pokemon['nickname']}！" . $counter_message . "\n{$next_pokemon['nickname']}倒下了...";
                $battle['turn'] = 0;

                api_success($battle);
            }
        }
    } else {
        $counter_message = "\n野怪没有反应过来！";
    }

    // 重新获取数据构建响应
    $new_mypokemon = api_my_pokemon($_G['username']);
    $battle = build_battle_response($myusersdata, $new_mypokemon);
    $battle['status'] = 'active';
    $battle['message'] = "成功切换为 {$next_pokemon['nickname']}！" . $counter_message;
    $battle['turn'] = 0;

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
    $mypokemon = api_my_pokemon($_G['username']);

    if (empty($myusersdata['npcid']) || $myusersdata['npcid'] <= 0) {
        api_error('没有进行中的战斗', 400);
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
            api_error('指定的宠物不可用', 400);
        }

        if ($specified_pet['id'] == $current_pet_id) {
            api_error('不能切换到当前上场的宠物', 400);
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
            api_error('没有可用的替补宠物', 400);
        }

        $next_pokemon = $available_pokemon[0];
    }

    // 交换 site 值：当前宠物变为 site=2，新宠物变为 site=1
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET site = 2 WHERE id = %d",
        $current_pet_id
    ));
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_mypm') . " SET site = 1 WHERE id = %d",
        intval($next_pokemon['id'])
    ));

    // 被动切换不进行野怪反击，直接返回响应
    $new_mypokemon = api_my_pokemon($_G['username']);
    $battle = build_battle_response($myusersdata, $new_mypokemon);
    $battle['status'] = 'active';
    $battle['message'] = "成功切换为 {$next_pokemon['nickname']}！";
    $battle['turn'] = 0;

    api_success($battle);
}
