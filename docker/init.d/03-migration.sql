-- ============================================================
-- TSDM Pokemon Plugin - X3 到 X5 数据库迁移脚本
-- ============================================================
-- 用途：从旧版 Discuz X3.x plugin_pokemon 数据库迁移到 X5
-- 说明：此脚本在保留原有数据的前提下进行表结构调整
-- ============================================================

-- ============================================================
-- 第一阶段：备份与安全检查
-- ============================================================

-- 1.1 创建迁移日志表
CREATE TABLE IF NOT EXISTS `pm_migration_log` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `step` varchar(50) NOT NULL COMMENT '迁移步骤',
    `status` tinyint(1) NOT NULL DEFAULT 0 COMMENT '0=未执行 1=成功 2=失败',
    `message` text,
    `executed_at` int(10) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;

-- ============================================================
-- 第二阶段：废弃字段标记与清理
-- ============================================================

-- 2.1 pm_map.exp 字段已废弃（原地图经验值，不再使用）
-- 保留字段但不再写入，待后续版本彻底移除
-- ALTER TABLE pm_map DROP COLUMN exp; -- 暂不执行，待数据确认后手动执行

-- 2.2 pm_data.shop 字段已废弃（商店直接购买功能已移除）
-- ALTER TABLE pm_data DROP COLUMN shop; -- 暂不执行

-- 2.3 pm_data.met 字段已废弃（旧版遇见概率，改用新的 encounter_rate）
-- ALTER TABLE pm_data DROP COLUMN met; -- 暂不执行

-- 2.4 pm_mypm.sx 字段已废弃（旧版属性缩写，改用 xs 字段）
-- ALTER TABLE pm_mypm DROP COLUMN sx; -- 暂不执行

-- ============================================================
-- 第三阶段：表引擎升级（MyISAM -> InnoDB）
-- ============================================================

-- 3.1 升级 pm_usersdata 表引擎
ALTER TABLE pm_usersdata ENGINE = InnoDB;

-- 3.2 升级 pm_data 表引擎
ALTER TABLE pm_data ENGINE = InnoDB;

-- 3.3 升级 pm_mypm 表引擎
ALTER TABLE pm_mypm ENGINE = InnoDB;

-- 3.4 升级 pm_itemdata 表引擎
ALTER TABLE pm_itemdata ENGINE = InnoDB;

-- 3.5 升级 pm_myitem 表引擎
ALTER TABLE pm_myitem ENGINE = InnoDB;

-- 3.6 升级 pm_skill 表引擎
ALTER TABLE pm_skill ENGINE = InnoDB;

-- 3.7 升级 pm_myskill 表引擎
ALTER TABLE pm_myskill ENGINE = InnoDB;

-- 3.8 升级 pm_map 表引擎
ALTER TABLE pm_map ENGINE = InnoDB;

-- ============================================================
-- 第四阶段：字符集升级（utf8mb3 -> utf8mb4）
-- ============================================================

-- 4.1 升级 pm_usersdata
ALTER TABLE pm_usersdata CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 4.2 升级 pm_data
ALTER TABLE pm_data CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 4.3 升级 pm_mypm
ALTER TABLE pm_mypm CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 4.4 升级 pm_itemdata
ALTER TABLE pm_itemdata CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 4.5 升级 pm_myitem
ALTER TABLE pm_myitem CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- ============================================================
-- 第五阶段：字段类型标准化
-- ============================================================

-- 5.1 修正 pm_map.exp 类型 (int(255) -> int(10))
ALTER TABLE pm_map MODIFY COLUMN `exp` int(10) NOT NULL DEFAULT 0;

-- 5.2 修正 pm_map 补充空值默认
UPDATE pm_map SET `expn` = '' WHERE `expn` IS NULL;
UPDATE pm_map SET `region` = '' WHERE `region` IS NULL;

-- ============================================================
-- 第六阶段：新增表（X5 新增功能）
-- ============================================================

-- 6.1 宠物盒子表
CREATE TABLE IF NOT EXISTS `pm_box` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(8) unsigned NOT NULL,
    `box_index` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `pm_id` mediumint(8) unsigned NOT NULL,
    `slot` tinyint(3) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_uid_box` (`uid`, `box_index`),
    KEY `idx_pm_id` (`pm_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 6.2 进化链表
CREATE TABLE IF NOT EXISTS `pm_evolution` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `from_id` mediumint(8) unsigned NOT NULL,
    `to_id` mediumint(8) unsigned NOT NULL,
    `method` varchar(20) NOT NULL DEFAULT 'level',
    `condition_value` varchar(50) NOT NULL DEFAULT '',
    `priority` int(10) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_from` (`from_id`),
    KEY `idx_to` (`to_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 6.3 对战日志表
CREATE TABLE IF NOT EXISTS `pm_battle_log` (
    `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
    `uid_1` mediumint(8) unsigned NOT NULL,
    `uid_2` mediumint(8) unsigned NOT NULL,
    `pm_id_1` mediumint(8) unsigned NOT NULL,
    `pm_id_2` mediumint(8) unsigned NOT NULL,
    `result` tinyint(1) NOT NULL DEFAULT 0,
    `detail` text NOT NULL,
    `created_at` int(10) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_uid_1` (`uid_1`),
    KEY `idx_created_at` (`created_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 6.4 PC中心寄存表
CREATE TABLE IF NOT EXISTS `pm_pc` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(8) unsigned NOT NULL,
    `pm_id` mediumint(8) unsigned NOT NULL,
    `deposited_at` int(10) unsigned NOT NULL DEFAULT 0,
    `healed_at` int(10) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_uid` (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- ============================================================
-- 第七阶段：数据迁移（旧结构 -> 新结构）
-- ============================================================

-- 7.1 从旧版用户数据迁移宠物盒子信息
-- (pm_usersdata.boxnum 记录了盒子数量，迁移为实际盒子槽位)
-- 此步骤需根据实际数据情况手动调整

-- 7.2 清理过期交换请求
-- 超过7天的交换记录可安全清理
-- DELETE FROM pm_usersdata WHERE exchanguid > 0 AND ppktime < UNIX_TIMESTAMP() - 604800;

-- ============================================================
-- 第八阶段：索引优化
-- ============================================================

-- 8.1 为 pm_mypm 添加联合查询索引
-- ALTER TABLE pm_mypm ADD INDEX idx_uid_state (`uid`, `state`);

-- 8.2 为 pm_myskill 添加技能查询索引
-- ALTER TABLE pm_myskill ADD INDEX idx_skillid (`skillid`);

-- ============================================================
-- 迁移完成标记
-- ============================================================
INSERT INTO pm_migration_log (`step`, `status`, `message`, `executed_at`)
VALUES ('migration_v1_x3_to_x5', 1, 'X3 到 X5 迁移脚本执行完毕', UNIX_TIMESTAMP());
