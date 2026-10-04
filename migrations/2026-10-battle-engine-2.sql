-- ============================================================
-- TSDM Pokemon Plugin — Battle engine 2.0 migration (issue #75)
-- ============================================================
-- 用途：为已安装站点升级到战斗引擎 2.0 补齐表结构与效果数据。
-- 幂等：全部对象经 information_schema / IF NOT EXISTS 判定后执行，
--       可重复运行（seed 使用 INSERT ... ON DUPLICATE KEY UPDATE）。
-- 全新安装无需本脚本：install.php 与 docker/init.d 已包含全部对象。
-- 对应运行时兜底：plugin/api/battle.php 的 battle_ensure_tables() 会
--       惰性补建同样的表与列（内容与本脚本一致）。
-- ============================================================

-- 迁移日志表（若目标库尚未执行过 03-migration.sql）
CREATE TABLE IF NOT EXISTS `pm_migration_log` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `step` varchar(50) NOT NULL COMMENT '迁移步骤',
    `status` tinyint(1) NOT NULL DEFAULT 0,
    `message` text,
    `executed_at` int(10) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;

-- ------------------------------------------------------------
-- [1/6] 战斗引擎三表（对局 / 参战单位 / 战报事件）
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS `pm_battle` (
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
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `pm_battle_unit` (
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
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `pm_battle_event` (
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
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- ------------------------------------------------------------
-- [2/6] 异常状态定义表
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS `pm_status` (
    `code` varchar(20) NOT NULL,
    `name` varchar(30) NOT NULL DEFAULT '',
    `behavior_json` text NOT NULL,
    `overlap` varchar(10) NOT NULL DEFAULT 'replace',
    `version` int(10) unsigned NOT NULL DEFAULT 1,
    PRIMARY KEY (`code`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- ------------------------------------------------------------
-- [3/6] 效果模板表
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS `pm_effect` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `code` varchar(40) NOT NULL,
    `kind` varchar(10) NOT NULL DEFAULT 'move',
    `hooks_json` text NOT NULL,
    `params_json` text NOT NULL,
    `description` varchar(255) NOT NULL DEFAULT '',
    `version` int(10) unsigned NOT NULL DEFAULT 1,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- ------------------------------------------------------------
-- [4/6] pm_skill.effect_id 列（仅旧库无该列时补列）
-- ------------------------------------------------------------
SET @has_effect_id := (
    SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = DATABASE() AND table_name = 'pm_skill' AND column_name = 'effect_id'
);
SET @stmt := IF(@has_effect_id = 0,
    'ALTER TABLE pm_skill ADD COLUMN effect_id int(10) unsigned NOT NULL DEFAULT 0 AFTER element',
    'SELECT ''battle-engine-2: pm_skill.effect_id 已存在，跳过''');
PREPARE _s FROM @stmt; EXECUTE _s; DEALLOCATE PREPARE _s;

-- ------------------------------------------------------------
-- [5/6] 效果模板 seed（与 battle_core 内置能力一一对应）
-- ------------------------------------------------------------
INSERT INTO `pm_effect` (`id`, `code`, `kind`, `hooks_json`, `params_json`, `description`, `version`) VALUES
(1, 'sword_dance', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"atk","stages":2,"target":"self"}', '剑舞：大幅提高自己的攻击。', 1),
(2, 'sand_attack', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"accuracy","stages":-1,"target":"opponent"}', '泼沙：降低对手的命中。', 1),
(3, 'tail_whip', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"def","stages":-1,"target":"opponent"}', '摇尾巴：降低对手的防御。', 1),
(4, 'leer', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"def","stages":-1,"target":"opponent"}', '瞪眼：用犀利的眼神威吓，降低对手的防御。', 1),
(5, 'growl', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"atk","stages":-1,"target":"opponent"}', '叫声：可爱的叫声让对手疏忽，降低对手的攻击。', 1),
(6, 'harden', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"def","stages":1,"target":"self"}', '变硬：提高自己的防御。', 1),
(7, 'poison_point', 'move', '["on_hit"]', '{"code":"status_inflict","status":"poison","chance":30}', '毒针附加：命中后有概率使对手中毒。', 1),
(8, 'flame_body', 'move', '["on_hit"]', '{"code":"status_inflict","status":"burn","chance":30}', '火焰之躯附加：命中后有概率使对手灼烧。', 1),
(9, 'static_abil', 'move', '["on_hit"]', '{"code":"status_inflict","status":"paralysis","chance":30}', '静电附加：命中后有概率使对手麻痹。', 1),
(10, 'sleep_powder', 'move', '["on_after_move"]', '{"code":"status_inflict","status":"sleep","chance":75}', '催眠粉：高概率使对手入睡。', 1)
ON DUPLICATE KEY UPDATE
    `kind` = VALUES(`kind`), `hooks_json` = VALUES(`hooks_json`),
    `params_json` = VALUES(`params_json`), `description` = VALUES(`description`),
    `version` = VALUES(`version`);

-- ------------------------------------------------------------
-- [6/6] 既有变化技的效果映射（按技能名，跨库 id 稳健）
-- ------------------------------------------------------------
UPDATE `pm_skill` SET `effect_id` = 1 WHERE `name` = '剑舞' AND `effect_id` = 0;
UPDATE `pm_skill` SET `effect_id` = 2 WHERE `name` = '泼沙' AND `effect_id` = 0;
UPDATE `pm_skill` SET `effect_id` = 3 WHERE `name` = '摇尾巴' AND `effect_id` = 0;
UPDATE `pm_skill` SET `effect_id` = 4 WHERE `name` = '瞪眼' AND `effect_id` = 0;
UPDATE `pm_skill` SET `effect_id` = 5 WHERE `name` = '叫声' AND `effect_id` = 0;

-- 迁移完成标记
INSERT INTO pm_migration_log (`step`, `status`, `message`, `executed_at`)
VALUES ('battle_engine_2', 1, '战斗引擎 2.0 迁移执行完毕（pm_battle/pm_battle_unit/pm_battle_event/pm_status/pm_effect + pm_skill.effect_id + 效果 seed）', UNIX_TIMESTAMP());
