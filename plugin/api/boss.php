<?php

/**
 * Boss 系统 API
 *
 * 端点:
 * - GET  ?action=get_config        获取地图 Boss 配置
 * - GET  ?action=try_spawn         尝试刷新 Boss
 */

if (defined('API_ROUTED')) {
    require_once __DIR__ . '/index.php';
} elseif (!defined('IN_DISCUZ')) {
    require_once __DIR__ . '/bootstrap.php';
    require_once __DIR__ . '/index.php';
} else {
    require_once __DIR__ . '/index.php';
}

require_once __DIR__ . '/constants.php';

global $_G;

// 仅在作为 boss 端点被路由或直接访问时处理 action；
// 如果是被其他文件包含复用函数（如 battle.php），则不执行
if (
    (defined('API_ROUTED') && defined('API_ENDPOINT') && API_ENDPOINT === 'boss')
    || basename($_SERVER['PHP_SELF']) === 'boss.php'
    || basename($_SERVER['SCRIPT_NAME']) === 'boss.php'
) {
    $action = get_param('action', '');

    switch ($action) {
        case 'get_config':
            api_get_boss_config();
            break;

        case 'try_spawn':
            api_try_spawn_boss();
            break;

        default:
            api_error('Invalid boss action', 400);
    }
}

/**
 * 获取地图的 Boss 配置
 *
 * 参数:
 * - map_id: 地图 ID
 *
 * 返回:
 * - success: 是否成功
 * - boss_config: Boss 配置 (如果没有则返回 null)
 */
function api_get_boss_config()
{
    global $_G;

    $map_id = get_param('map_id', 0);
    $map_id = intval($map_id);

    if ($map_id <= 0) {
        api_error('Invalid map_id', 400);
    }

    // 查询地图信息
    $map = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_map') . " WHERE id = %d LIMIT 1",
        $map_id
    ));

    if (!$map) {
        api_error('Map not found', 404);
    }

    $boss_config = get_map_boss_config_from_map($map);

    api_json([
        'success' => true,
        'boss_config' => $boss_config,
    ]);
}

/**
 * 尝试刷新 Boss
 *
 * 参数:
 * - map_id: 地图 ID
 *
 * 返回:
 * - success: 是否成功
 * - boss: Boss 数据 (如果刷新成功则返回 Boss 信息，否则返回 null)
 * - message: 消息说明
 */
function api_try_spawn_boss()
{
    global $_G;

    $map_id = get_param('map_id', 0);
    $map_id = intval($map_id);

    if ($map_id <= 0) {
        api_error('Invalid map_id', 400);
    }

    // 查询地图信息
    $map = DB::fetch_first(pm_sql(
        "SELECT * FROM " . pm_table('pm_map') . " WHERE id = %d LIMIT 1",
        $map_id
    ));

    if (!$map) {
        api_error('Map not found', 404);
    }

    // 获取 Boss 配置
    $boss_config = get_map_boss_config_from_map($map);

    if (!$boss_config || empty($boss_config['bosses'])) {
        api_json([
            'success' => true,
            'boss' => null,
            'message' => '该地图没有 Boss 配置',
        ]);
    }

    // 尝试刷新 Boss
    $boss = try_spawn_boss_from_config($boss_config);

    if ($boss !== null) {
        // 查询 Boss 宠物详细信息
        $pet = DB::fetch_first(
            "SELECT * FROM " . pm_table('pm_data') . "
            WHERE id = " . intval($boss['pokemon_type_id']) . " LIMIT 1"
        );

        if ($pet) {
            api_json([
                'success' => true,
                'boss' => [
                    'pokemon_type_id' => $boss['pokemon_type_id'],
                    'pokemon_name' => $boss['pokemon_name'],
                    'level' => $boss['level'],
                    'boss_multiplier' => $boss['boss_multiplier'],
                    'pet' => [
                        'id' => $pet['id'],
                        'name' => $pet['name'],
                        'capture' => $pet['capture'] ?: 100,
                    ],
                ],
                'message' => 'Boss 刷新成功',
            ]);
        } else {
            api_json([
                'success' => false,
                'boss' => null,
                'message' => 'Boss 宠物数据不存在',
            ]);
        }
    } else {
        api_json([
            'success' => true,
            'boss' => null,
            'message' => 'Boss 未刷新',
        ]);
    }
}

/**
 * 从地图数据获取 Boss 配置
 * 内部函数，不直接暴露为 API
 */
function get_map_boss_config_from_map($map)
{
    $boss_config = isset($map['boss_config']) ? $map['boss_config'] : '0';

    // 尝试解析为 JSON
    $boss_config = json_decode($boss_config, true);

    if (is_array($boss_config) && isset($boss_config['bosses']) && is_array($boss_config['bosses'])) {
        return $boss_config;
    }

    return null;
}

/**
 * 尝试从配置刷新 Boss
 * 内部函数，不直接暴露为 API
 */
function try_spawn_boss_from_config($boss_config)
{
    if (empty($boss_config['bosses'])) {
        return null;
    }

    // 随机选择一个 Boss
    $available_bosses = array_filter($boss_config['bosses'], function ($boss) {
        return isset($boss['pokemon_type_id']) && $boss['pokemon_type_id'] > 0;
    });

    if (empty($available_bosses)) {
        return null;
    }

    $boss = $available_bosses[array_rand($available_bosses)];

    return $boss;
}
