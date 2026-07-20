<?php

/**
 * TSDM Pokemon Plugin - X5 Install Script
 */

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

$sql = <<<EOF

-- 用户主数据表
CREATE TABLE IF NOT EXISTS `pm_usersdata` (
    `uid` mediumint(8) unsigned NOT NULL,
    `npcid` int(20) NOT NULL DEFAULT 0,
    `hpg` int(5) NOT NULL DEFAULT 0,
    `hp` int(10) NOT NULL DEFAULT 0,
    `atkg` int(5) NOT NULL DEFAULT 0,
    `spatkg` int(5) NOT NULL DEFAULT 0,
    `defg` int(5) NOT NULL DEFAULT 0,
    `spdefg` int(5) NOT NULL DEFAULT 0,
    `sdg` int(5) NOT NULL DEFAULT 0,
    `allure` int(10) NOT NULL DEFAULT 0,
    `capture` int(10) NOT NULL DEFAULT 0,
    `level` int(10) NOT NULL DEFAULT 0,
    `ppkname` varchar(20) NOT NULL DEFAULT '',
    `ppktime` int(10) unsigned NOT NULL DEFAULT 0,
    `ppkround` smallint(5) unsigned NOT NULL DEFAULT 0,
    `ppk` tinyint(2) unsigned NOT NULL DEFAULT 0,
    `ppkfight` tinyint(2) NOT NULL DEFAULT 0,
    `ppkdodge` smallint(5) NOT NULL DEFAULT 0,
    `ppkot` varchar(100) NOT NULL DEFAULT '',
    `ppkpriority` tinyint(2) unsigned NOT NULL DEFAULT 0,
    `exchanguid` int(10) unsigned NOT NULL DEFAULT 0,
    `exchangepmid` int(10) unsigned NOT NULL DEFAULT 0,
    `dataall` int(10) NOT NULL DEFAULT 0,
    `datawin` int(10) NOT NULL DEFAULT 0,
    `datalost` int(10) NOT NULL DEFAULT 0,
    `fullexp` bigint(20) NOT NULL DEFAULT 0,
    `npcsg` tinyint(1) NOT NULL DEFAULT 0,
    `boxnum` smallint(8) NOT NULL DEFAULT 9,
    `strength` tinyint(3) NOT NULL DEFAULT 1,
    `str` int(10) NOT NULL DEFAULT 100,
    `hpn` int(11) NOT NULL DEFAULT 0,
    `atkn` int(11) NOT NULL DEFAULT 0,
    `defn` int(11) NOT NULL DEFAULT 0,
    `spatkn` int(11) NOT NULL DEFAULT 0,
    `spdefn` int(11) NOT NULL DEFAULT 0,
    `sdn` int(11) NOT NULL DEFAULT 0,
    `money` bigint(20) NOT NULL DEFAULT 0,
    PRIMARY KEY (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 宠物图鉴表
CREATE TABLE IF NOT EXISTS `pm_data` (
    `id` mediumint(8) unsigned NOT NULL,
    `name` varchar(30) NOT NULL,
    `money` mediumint(5) unsigned NOT NULL DEFAULT 0,
    `txt` varchar(100) NOT NULL,
    `sex` smallint(5) NOT NULL DEFAULT 0,
    `xs` varchar(6) NOT NULL,
    `xs2` varchar(6) NOT NULL,
    `hp` smallint(6) NOT NULL DEFAULT 0,
    `atk` smallint(6) NOT NULL DEFAULT 0,
    `def` smallint(6) NOT NULL DEFAULT 0,
    `spatk` smallint(6) NOT NULL DEFAULT 0,
    `spdef` smallint(6) NOT NULL DEFAULT 0,
    `sd` smallint(6) NOT NULL DEFAULT 0,
    `mapid` mediumtext NOT NULL,
    `capture` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `met` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `shop` tinyint(1) NOT NULL DEFAULT 0,
    `hpn` smallint(6) NOT NULL DEFAULT 0,
    `atkn` smallint(4) NOT NULL DEFAULT 0,
    `defn` smallint(6) NOT NULL DEFAULT 0,
    `spatkn` smallint(6) NOT NULL DEFAULT 0,
    `spdefn` smallint(6) NOT NULL DEFAULT 0,
    `sdn` smallint(6) NOT NULL DEFAULT 0,
    `birth` tinyint(1) NOT NULL DEFAULT 0,
    `birthodds` tinyint(2) NOT NULL DEFAULT 0,
    `pnclevel` smallint(3) NOT NULL,
    `god` tinyint(1) NOT NULL DEFAULT 0,
    `minmoeny` int(1) NOT NULL DEFAULT 0,
    `mixmoeny` int(5) NOT NULL DEFAULT 0,
    `strength` tinyint(3) NOT NULL DEFAULT 1,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 用户宠物表
CREATE TABLE IF NOT EXISTS `pm_mypm` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `pctime` int(10) NOT NULL DEFAULT 0,
    `uid` mediumint(9) NOT NULL,
    `pmname` varchar(10) NOT NULL,
    `nowname` varchar(10) NOT NULL,
    `pmno` smallint(8) unsigned NOT NULL,
    `level` tinyint(5) unsigned NOT NULL DEFAULT 0,
    `exp` int(10) unsigned NOT NULL DEFAULT 0,
    `sex` tinyint(1) NOT NULL DEFAULT 0,
    `sx` varchar(10) NOT NULL,
    `hp` int(10) unsigned NOT NULL DEFAULT 0,
    `hpg` tinyint(2) NOT NULL DEFAULT 0,
    `atkg` tinyint(2) NOT NULL DEFAULT 0,
    `defg` tinyint(2) NOT NULL DEFAULT 0,
    `spatkg` tinyint(2) NOT NULL DEFAULT 0,
    `spdefg` tinyint(2) unsigned NOT NULL DEFAULT 0,
    `sdg` tinyint(2) NOT NULL DEFAULT 0,
    `good` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `itemevolve` tinyint(2) unsigned DEFAULT NULL,
    `ballid` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `site` tinyint(1) NOT NULL DEFAULT 0,
    `state` tinyint(2) unsigned NOT NULL DEFAULT 0,
    `statetime` int(10) unsigned NOT NULL DEFAULT 0,
    `gduptime` int(10) unsigned NOT NULL DEFAULT 0,
    `hpn` smallint(6) NOT NULL DEFAULT 0,
    `atkn` smallint(4) NOT NULL DEFAULT 0,
    `defn` smallint(6) NOT NULL DEFAULT 0,
    `spatkn` smallint(6) NOT NULL DEFAULT 0,
    `spdefn` smallint(6) NOT NULL DEFAULT 0,
    `sdn` smallint(6) NOT NULL DEFAULT 0,
    `initialuid` mediumint(9) NOT NULL DEFAULT 0,
    `swap` tinyint(1) NOT NULL DEFAULT 1,
    `sg` tinyint(1) NOT NULL DEFAULT 0,
    `equipmentid1` mediumint(8) NOT NULL DEFAULT 0,
    `equipmentid2` mediumint(8) NOT NULL DEFAULT 0,
    `equipmentid3` mediumint(8) NOT NULL DEFAULT 0,
    `equipmentid4` mediumint(8) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_uid` (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 物品数据表
CREATE TABLE IF NOT EXISTS `pm_itemdata` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(10) NOT NULL,
    `tpname` varchar(30) NOT NULL,
    `intro` varchar(255) NOT NULL DEFAULT '',
    `shop` tinyint(2) NOT NULL DEFAULT 0,
    `money` smallint(10) unsigned NOT NULL,
    `txt` varchar(255) NOT NULL,
    `type` tinyint(1) NOT NULL,
    `lvask` tinyint(3) unsigned NOT NULL,
    `xsask` varchar(5) NOT NULL,
    `addhp` smallint(5) unsigned NOT NULL DEFAULT 0,
    `addexp` smallint(5) unsigned NOT NULL DEFAULT 0,
    `addlv` tinyint(3) NOT NULL DEFAULT 0,
    `addgood` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `ballid` smallint(3) unsigned NOT NULL DEFAULT 0,
    `upitem` smallint(5) unsigned NOT NULL,
    `captmax` tinyint(3) unsigned NOT NULL,
    `captmin` tinyint(3) unsigned NOT NULL,
    `sitemname` varchar(40) NOT NULL,
    `sitemid` varchar(30) NOT NULL DEFAULT '',
    `ppkallow` tinyint(2) unsigned NOT NULL DEFAULT 0,
    `hot` varchar(255) NOT NULL,
    `zbtype` tinyint(2) NOT NULL,
    `equipment_hp` int(8) NOT NULL DEFAULT 0,
    `equipment_atk` int(8) NOT NULL DEFAULT 0,
    `equipment_def` int(8) NOT NULL DEFAULT 0,
    `equipment_spatk` int(8) NOT NULL DEFAULT 0,
    `equipment_spdef` int(8) NOT NULL DEFAULT 0,
    `equipment_sd` int(8) NOT NULL DEFAULT 0,
    `atk` smallint(6) NOT NULL DEFAULT 0,
    `def` smallint(6) NOT NULL DEFAULT 0,
    `spatk` smallint(6) NOT NULL DEFAULT 0,
    `spdef` smallint(6) NOT NULL DEFAULT 0,
    `speed` smallint(6) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 用户背包表
CREATE TABLE IF NOT EXISTS `pm_myitem` (
    `id` int(8) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(9) NOT NULL,
    `itemid` varchar(5) NOT NULL,
    `typeid` mediumint(8) unsigned NOT NULL DEFAULT 0,
    `nums` smallint(3) NOT NULL,
    `ball` int(11) DEFAULT NULL,
    `pmid` int(8) DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `idx_uid` (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 系统配置表
CREATE TABLE IF NOT EXISTS `pm_config` (
    `key` varchar(255) NOT NULL,
    `value` varchar(255) NOT NULL,
    `data_type` varchar(20) NOT NULL,
    PRIMARY KEY (`key`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 地图表
CREATE TABLE IF NOT EXISTS `pm_map` (
    `id` mediumint(8) NOT NULL AUTO_INCREMENT,
    `name` varchar(20) NOT NULL,
    `kg` tinyint(2) NOT NULL DEFAULT 0,
    `minlevel` tinyint(3) unsigned NOT NULL DEFAULT 1,
    `maxlevel` tinyint(3) unsigned NOT NULL DEFAULT 1,
    `exp` int(10) NOT NULL DEFAULT 0 COMMENT '已废弃',
    `site` mediumtext NOT NULL,
    `expn` TEXT NOT NULL DEFAULT '' COMMENT 'Boss配置JSON',
    `region` varchar(20) NOT NULL DEFAULT '' COMMENT '所属区域',
    `pos_x` tinyint(3) unsigned NOT NULL DEFAULT 50 COMMENT 'X坐标',
    `pos_y` tinyint(3) unsigned NOT NULL DEFAULT 50 COMMENT 'Y坐标',
    UNIQUE KEY `id` (`id`),
    KEY `idx_region` (`region`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 技能表
CREATE TABLE IF NOT EXISTS `pm_skill` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `pmid` mediumtext NOT NULL,
    `name` varchar(30) NOT NULL,
    `txt` mediumtext NOT NULL,
    `lv` smallint(5) NOT NULL,
    `powr` mediumint(10) NOT NULL DEFAULT 40,
    `num` smallint(5) NOT NULL DEFAULT 35,
    `type` tinyint(3) NOT NULL DEFAULT 0,
    `tn` varchar(6) NOT NULL DEFAULT '',
    `category` varchar(6) NOT NULL DEFAULT '',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 用户技能表
CREATE TABLE IF NOT EXISTS `pm_myskill` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
    `petid` mediumint(8) unsigned NOT NULL DEFAULT 0 COMMENT '宠物ID',
    `skillid` mediumint(8) unsigned NOT NULL DEFAULT 0 COMMENT '技能ID',
    `skillnum` smallint(5) NOT NULL DEFAULT 0 COMMENT '当前PP值',
    PRIMARY KEY (`id`),
    KEY `idx_uid_petid` (`uid`, `petid`),
    KEY `idx_petid` (`petid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 宠物盒子表 (X5 新增)
CREATE TABLE IF NOT EXISTS `pm_box` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(8) unsigned NOT NULL,
    `box_index` tinyint(3) unsigned NOT NULL DEFAULT 0 COMMENT '盒子编号',
    `pm_id` mediumint(8) unsigned NOT NULL COMMENT '宠物ID',
    `slot` tinyint(3) unsigned NOT NULL DEFAULT 0 COMMENT '槽位',
    PRIMARY KEY (`id`),
    KEY `idx_uid_box` (`uid`, `box_index`),
    KEY `idx_pm_id` (`pm_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 进化链表 (X5 新增)
CREATE TABLE IF NOT EXISTS `pm_evolution` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `from_id` mediumint(8) unsigned NOT NULL COMMENT '进化前宠物ID',
    `to_id` mediumint(8) unsigned NOT NULL COMMENT '进化后宠物ID',
    `method` varchar(20) NOT NULL DEFAULT 'level' COMMENT '进化方式',
    `condition_value` varchar(50) NOT NULL DEFAULT '' COMMENT '进化条件值',
    PRIMARY KEY (`id`),
    KEY `idx_from` (`from_id`),
    KEY `idx_to` (`to_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- 对战日志表 (X5 新增)
CREATE TABLE IF NOT EXISTS `pm_battle_log` (
    `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
    `uid_1` mediumint(8) unsigned NOT NULL COMMENT '挑战者',
    `uid_2` mediumint(8) unsigned NOT NULL COMMENT '对手',
    `pm_id_1` mediumint(8) unsigned NOT NULL COMMENT '挑战者宠物',
    `pm_id_2` mediumint(8) unsigned NOT NULL COMMENT '对手宠物',
    `result` tinyint(1) NOT NULL DEFAULT 0 COMMENT '结果 0=未完成 1=胜利 2=败北 3=平局',
    `detail` text NOT NULL COMMENT '对战详情JSON',
    `created_at` int(10) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_uid_1` (`uid_1`),
    KEY `idx_uid_2` (`uid_2`),
    KEY `idx_created_at` (`created_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- PC中心寄存表 (X5 新增)
CREATE TABLE IF NOT EXISTS `pm_pc` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(8) unsigned NOT NULL,
    `pm_id` mediumint(8) unsigned NOT NULL COMMENT '寄存宠物ID',
    `deposited_at` int(10) unsigned NOT NULL DEFAULT 0 COMMENT '寄存时间',
    `healed_at` int(10) unsigned NOT NULL DEFAULT 0 COMMENT '最后治疗时间',
    PRIMARY KEY (`id`),
    KEY `idx_uid` (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

ALTER TABLE pre_common_member_field_forum ADD COLUMN IF NOT EXISTS pokemon TEXT AFTER medals;

EOF;

runquery($sql);

$finish = TRUE;
