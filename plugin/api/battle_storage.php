<?php
/** Shared battle storage helpers; safe to load without routing a battle request. */
if (!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

/**
 * 每日宠物币上限所需的 pm_usersdata.money_day / money_today 是否可用。
 *
 * 默认可用；battle_ensure_tables() 补列失败时传 false 关闭，
 * 结算随即降级为不设每日上限、只加 money 的旧更新。
 */
function battle_money_daily_columns_available($set = null)
{
    static $available = true;
    if ($set !== null) {
        $available = (bool) $set;
    }
    return $available;
}

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
        `revision` int(10) unsigned NOT NULL DEFAULT 0,
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
    // Old installations need the same revision column before any transaction.
    $revision_column = DB::fetch_first("SHOW COLUMNS FROM " . pm_table('pm_battle') . " LIKE 'revision'");
    if (!$revision_column) {
        DB::query("ALTER TABLE " . pm_table('pm_battle') . " ADD COLUMN revision int(10) unsigned NOT NULL DEFAULT 0", 'SILENT');
        if (!DB::fetch_first("SHOW COLUMNS FROM " . pm_table('pm_battle') . " LIKE 'revision'")) {
            throw new RuntimeException('Battle revision migration failed');
        }
    }
    // 宠物币每日上限计数（money_day = 站台时区 yyyymmdd，money_today = 当日已得）。
    // 老站点惰性补列；与 revision 同样在任何事务之前完成（ALTER 会隐式提交）。
    // 失败时只关闭每日上限并记日志，不抛错。
    $money_columns = array(
        'money_day' => 'int(10) unsigned NOT NULL DEFAULT 0',
        'money_today' => 'bigint(20) NOT NULL DEFAULT 0',
    );
    foreach ($money_columns as $column => $definition) {
        if (DB::fetch_first("SHOW COLUMNS FROM " . pm_table('pm_usersdata') . " LIKE '$column'")) {
            continue;
        }
        DB::query("ALTER TABLE " . pm_table('pm_usersdata') . " ADD COLUMN $column $definition", 'SILENT');
        if (!DB::fetch_first("SHOW COLUMNS FROM " . pm_table('pm_usersdata') . " LIKE '$column'")) {
            // 补列失败（如数据库账号无 ALTER 权限）不拖垮战斗：本请求不启用每日上限
            battle_money_daily_columns_available(false);
            $message = "pm_usersdata.$column could not be added; the daily pet-coin cap is disabled until it exists";
            if (function_exists('writelog')) {
                writelog('pokemon', $message);
            } else {
                error_log('[pokemon] ' . $message);
            }
            break;
        }
    }
    DB::query("CREATE TABLE IF NOT EXISTS " . pm_table('pm_battle_action') . " (
        `uid` mediumint(8) unsigned NOT NULL,
        `request_id` varchar(64) CHARACTER SET ascii COLLATE ascii_bin NOT NULL,
        `action` varchar(24) NOT NULL,
        `payload_hash` char(64) CHARACTER SET ascii COLLATE ascii_bin NOT NULL,
        `battle_id` bigint(20) unsigned NOT NULL DEFAULT 0,
        `response_json` mediumtext DEFAULT NULL,
        `created_at` int(10) unsigned NOT NULL DEFAULT 0,
        PRIMARY KEY (`uid`, `request_id`),
        KEY `idx_uid_created` (`uid`, `created_at`)
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
 * 清理战斗状态
 */
function clear_battle_state($uid, $advance_revision = false)
{
    // 同步结束新引擎的进行中战斗（capture/use_item/switch 等路径收尾时联动）。
    // 已有 result 的战斗（fled/victory 已由新引擎路径写入）不覆盖 result；
    // 无 result 的（旧路径清状态）记为 abandoned。
    // 两条拆分而非 IF(result='')：Discuz querysafe 拒绝含空串字面量的表达式。
    battle_ensure_tables();
    $revision_clause = $advance_revision ? ", revision = revision + 1" : "";
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_battle') . "
        SET phase = 'ended', updated_at = %d" . $revision_clause . "
        WHERE uid = %d AND phase IN ('active', 'awaiting_switch') AND CHAR_LENGTH(result) > 0",
        time(), $uid
    ));
    DB::query(pm_sql(
        "UPDATE " . pm_table('pm_battle') . "
        SET phase = 'ended', result = 'abandoned', updated_at = %d" . $revision_clause . "
        WHERE uid = %d AND phase IN ('active', 'awaiting_switch') AND CHAR_LENGTH(result) = 0",
        time(), $uid
    ));
    // 这些列均为整数类型：写入 '' 在 MariaDB 严格模式(STRICT_TRANS_TABLES)下会直接报错，
    // 导致战斗状态无法清除（用户卡在战斗中），必须写 0
    DB::query(pm_sql("UPDATE " . pm_table('pm_usersdata') . "
        SET npcid = 0, level = 0, hp = 0, hpg = 0, atkg = 0, defg = 0,
            spatkg = 0, spdefg = 0, sdg = 0, allure = 0, capture = 0
        WHERE uid = %d", $uid));
}
