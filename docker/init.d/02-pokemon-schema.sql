-- Pokemon Plugin Schema — all tables for fresh install
SET sql_mode = '';
SET NAMES utf8mb4;

-- System config
CREATE TABLE IF NOT EXISTS `pm_config` (
    `key` varchar(255) NOT NULL,
    `value` varchar(255) NOT NULL,
    `data_type` varchar(20) NOT NULL DEFAULT 'string',
    PRIMARY KEY (`key`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Pokemon species data (Pokedex)
CREATE TABLE IF NOT EXISTS `pm_data` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(30) NOT NULL,
    `money` mediumint(5) unsigned NOT NULL DEFAULT 0,
    `txt` varchar(100) NOT NULL,
    `sex` smallint(5) NOT NULL DEFAULT 0,
    `xs` varchar(6) NOT NULL DEFAULT '',
    `xs2` varchar(6) NOT NULL DEFAULT '',
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
    `pnclevel` smallint(3) NOT NULL DEFAULT 0,
    `god` tinyint(1) NOT NULL DEFAULT 0,
    `minmoeny` int(1) NOT NULL DEFAULT 0,
    `mixmoeny` int(5) NOT NULL DEFAULT 0,
    `strength` tinyint(3) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Items catalog
CREATE TABLE IF NOT EXISTS `pm_itemdata` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(10) NOT NULL,
    `tpname` varchar(30) NOT NULL,
    `intro` varchar(255) NOT NULL DEFAULT '',
    `shop` tinyint(2) NOT NULL DEFAULT 0,
    `money` smallint(10) unsigned NOT NULL DEFAULT 0,
    `txt` varchar(255) NOT NULL,
    `type` tinyint(1) NOT NULL DEFAULT 0,
    `lvask` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `xsask` varchar(5) NOT NULL DEFAULT '',
    `addhp` smallint(5) unsigned NOT NULL DEFAULT 0,
    `addexp` smallint(5) unsigned NOT NULL DEFAULT 0,
    `addlv` tinyint(3) NOT NULL DEFAULT 0,
    `addgood` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `ballid` smallint(3) unsigned NOT NULL DEFAULT 0,
    `upitem` smallint(5) unsigned NOT NULL DEFAULT 0,
    `captmax` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `captmin` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `sitemname` varchar(40) NOT NULL DEFAULT '',
    `sitemid` varchar(30) NOT NULL DEFAULT '',
    `ppkallow` tinyint(2) unsigned NOT NULL DEFAULT 0,
    `hot` varchar(255) NOT NULL DEFAULT '',
    `zbtype` tinyint(2) NOT NULL DEFAULT 0,
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

-- Maps
CREATE TABLE IF NOT EXISTS `pm_map` (
    `id` mediumint(8) NOT NULL AUTO_INCREMENT,
    `name` varchar(20) NOT NULL,
    `kg` tinyint(2) NOT NULL DEFAULT 0,
    `minlevel` tinyint(3) unsigned NOT NULL DEFAULT 1,
    `maxlevel` tinyint(3) unsigned NOT NULL DEFAULT 1,
    `exp` int(10) NOT NULL DEFAULT 0,
    `site` mediumtext NOT NULL,
    `expn` text NOT NULL,
    `region` varchar(20) NOT NULL DEFAULT '',
    `pos_x` tinyint(3) unsigned NOT NULL DEFAULT 50,
    `pos_y` tinyint(3) unsigned NOT NULL DEFAULT 50,
    PRIMARY KEY (`id`),
    KEY `idx_region` (`region`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Skills catalog
CREATE TABLE IF NOT EXISTS `pm_skill` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `pmid` mediumtext NOT NULL,
    `name` varchar(30) NOT NULL,
    `txt` mediumtext NOT NULL,
    `lv` smallint(5) NOT NULL DEFAULT 0,
    `powr` mediumint(10) NOT NULL DEFAULT 40,
    `num` smallint(5) NOT NULL DEFAULT 35,
    `type` tinyint(3) NOT NULL DEFAULT 0,
    `tn` varchar(6) NOT NULL DEFAULT '',
    `category` varchar(6) NOT NULL DEFAULT '',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Evolution chains
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

-- User game data
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
    `boxnum` smallint(8) NOT NULL DEFAULT 0,
    `strength` tinyint(3) NOT NULL DEFAULT 0,
    `str` int(10) NOT NULL DEFAULT 0,
    `hpn` int(11) NOT NULL DEFAULT 0,
    `atkn` int(11) NOT NULL DEFAULT 0,
    `defn` int(11) NOT NULL DEFAULT 0,
    `spatkn` int(11) NOT NULL DEFAULT 0,
    `spdefn` int(11) NOT NULL DEFAULT 0,
    `sdn` int(11) NOT NULL DEFAULT 0,
    `money` bigint(20) NOT NULL DEFAULT 0,
    PRIMARY KEY (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- User's owned Pokemon
CREATE TABLE IF NOT EXISTS `pm_mypm` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `pctime` int(10) NOT NULL DEFAULT 0,
    `uid` mediumint(9) NOT NULL DEFAULT 0,
    `pmname` varchar(10) NOT NULL DEFAULT '',
    `nowname` varchar(10) NOT NULL DEFAULT '',
    `pmno` smallint(8) unsigned NOT NULL DEFAULT 0,
    `level` tinyint(5) unsigned NOT NULL DEFAULT 1,
    `exp` int(10) unsigned NOT NULL DEFAULT 0,
    `sex` tinyint(1) NOT NULL DEFAULT 0,
    `sx` varchar(10) NOT NULL DEFAULT '',
    `hp` int(10) unsigned NOT NULL DEFAULT 0,
    `hpg` tinyint(2) NOT NULL DEFAULT 0,
    `atkg` tinyint(2) NOT NULL DEFAULT 0,
    `defg` tinyint(2) NOT NULL DEFAULT 0,
    `spatkg` tinyint(2) NOT NULL DEFAULT 0,
    `spdefg` tinyint(2) unsigned NOT NULL DEFAULT 0,
    `sdg` tinyint(2) NOT NULL DEFAULT 0,
    `good` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `itemevolve` tinyint(2) unsigned NOT NULL DEFAULT 0,
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
    `swap` tinyint(1) NOT NULL DEFAULT 0,
    `sg` tinyint(1) NOT NULL DEFAULT 0,
    `equipmentid1` mediumint(8) NOT NULL DEFAULT 0,
    `equipmentid2` mediumint(8) NOT NULL DEFAULT 0,
    `equipmentid3` mediumint(8) NOT NULL DEFAULT 0,
    `equipmentid4` mediumint(8) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `uid` (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- User's owned items
CREATE TABLE IF NOT EXISTS `pm_myitem` (
    `id` int(8) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(9) NOT NULL DEFAULT 0,
    `itemid` varchar(5) NOT NULL DEFAULT '',
    `typeid` mediumint(8) unsigned NOT NULL DEFAULT 0,
    `nums` smallint(3) NOT NULL DEFAULT 0,
    `ball` int(11) NOT NULL DEFAULT 0,
    `pmid` int(8) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- User's owned skills
CREATE TABLE IF NOT EXISTS `pm_myskill` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
    `petid` mediumint(8) unsigned NOT NULL DEFAULT 0,
    `skillid` mediumint(8) unsigned NOT NULL DEFAULT 0,
    `skillnum` smallint(5) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- PC storage
CREATE TABLE IF NOT EXISTS `pm_pc` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(8) unsigned NOT NULL,
    `pm_id` mediumint(8) unsigned NOT NULL,
    `deposited_at` int(10) unsigned NOT NULL DEFAULT 0,
    `healed_at` int(10) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Pokemon boxes
CREATE TABLE IF NOT EXISTS `pm_box` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(8) unsigned NOT NULL,
    `box_index` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `pm_id` mediumint(8) unsigned NOT NULL,
    `slot` tinyint(3) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Battle log
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
