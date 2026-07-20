-- ============================================================
-- TSDM Pokemon Plugin - X3 到 X5 数据库迁移脚本
-- ============================================================
-- 用途：旧版数据迁移和表结构调整
-- 说明：表结构已由 02-pokemon-schema.sql 创建，此脚本仅做结构修正
-- ============================================================

-- 迁移日志表
CREATE TABLE IF NOT EXISTS `pm_migration_log` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `step` varchar(50) NOT NULL COMMENT '迁移步骤',
    `status` tinyint(1) NOT NULL DEFAULT 0,
    `message` text,
    `executed_at` int(10) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;

-- ============================================================
-- 字段类型标准化（兼容旧数据）
-- ============================================================
ALTER TABLE pm_map MODIFY COLUMN `exp` int(10) NOT NULL DEFAULT 0;
UPDATE pm_map SET `expn` = '' WHERE `expn` IS NULL;
UPDATE pm_map SET `region` = '' WHERE `region` IS NULL;

-- ============================================================
-- 迁移完成标记
-- ============================================================
INSERT INTO pm_migration_log (`step`, `status`, `message`, `executed_at`)
VALUES ('migration_v1_x3_to_x5', 1, 'X3 到 X5 迁移脚本执行完毕', UNIX_TIMESTAMP());
