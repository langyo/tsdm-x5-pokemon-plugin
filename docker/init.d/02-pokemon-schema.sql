-- Pokemon Plugin Schema — all tables for fresh install (canonical X5 new-column names)
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
    `money` mediumint(8) unsigned NOT NULL DEFAULT 0,
    `description` varchar(255) NOT NULL DEFAULT '',
    `sex` smallint(5) NOT NULL DEFAULT 0,
    `xs` varchar(6) NOT NULL DEFAULT '',
    `xs2` varchar(6) NOT NULL DEFAULT '',
    `hp` smallint(6) NOT NULL DEFAULT 0,
    `atk` smallint(6) NOT NULL DEFAULT 0,
    `def` smallint(6) NOT NULL DEFAULT 0,
    `spatk` smallint(6) NOT NULL DEFAULT 0,
    `spdef` smallint(6) NOT NULL DEFAULT 0,
    `speed` smallint(6) NOT NULL DEFAULT 0,
    `mapid` mediumtext NOT NULL,
    `capture` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `met` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `shop` tinyint(1) NOT NULL DEFAULT 0,
    `effort_values` text NOT NULL,
    `birth` tinyint(1) NOT NULL DEFAULT 0,
    `is_legendary` tinyint(1) NOT NULL DEFAULT 0,
    `drop_money` varchar(60) NOT NULL DEFAULT '',
    `strength` tinyint(3) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Items catalog
CREATE TABLE IF NOT EXISTS `pm_itemdata` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(30) NOT NULL,
    `tpname` varchar(60) NOT NULL DEFAULT '',
    `description` varchar(255) NOT NULL DEFAULT '',
    `shop` tinyint(1) NOT NULL DEFAULT 0,
    `money` int(10) unsigned NOT NULL DEFAULT 0,
    `type` tinyint(1) NOT NULL DEFAULT 0,
    `module` varchar(30) NOT NULL DEFAULT '',
    `lvask` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `xsask` varchar(5) NOT NULL DEFAULT '',
    `effects` text NOT NULL,
    `ballid` smallint(3) unsigned NOT NULL DEFAULT 0,
    `upitem` smallint(5) unsigned NOT NULL DEFAULT 0,
    `captmax` tinyint(3) unsigned NOT NULL DEFAULT 0,
    `sitemname` varchar(40) NOT NULL DEFAULT '',
    `zbtype` tinyint(2) NOT NULL DEFAULT 0,
    `equipment` text NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Maps
CREATE TABLE IF NOT EXISTS `pm_map` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(20) NOT NULL,
    `is_enabled` tinyint(1) NOT NULL DEFAULT 1,
    `min_level` tinyint(3) unsigned NOT NULL DEFAULT 1,
    `max_level` tinyint(3) unsigned NOT NULL DEFAULT 1,
    `experience` int(10) NOT NULL DEFAULT 0,
    `site` mediumtext NOT NULL,
    `boss_config` text NOT NULL,
    `region` varchar(20) NOT NULL DEFAULT '',
    `pos_x` tinyint(3) unsigned NOT NULL DEFAULT 50,
    `pos_y` tinyint(3) unsigned NOT NULL DEFAULT 50,
    PRIMARY KEY (`id`),
    KEY `idx_region` (`region`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Skills catalog
CREATE TABLE IF NOT EXISTS `pm_skill` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `available_pokemons` mediumtext NOT NULL,
    `name` varchar(30) NOT NULL,
    `description` mediumtext NOT NULL,
    `level_required` smallint(5) NOT NULL DEFAULT 0,
    `power` mediumint(8) NOT NULL DEFAULT 40,
    `max_uses` smallint(5) NOT NULL DEFAULT 35,
    `type` tinyint(3) NOT NULL DEFAULT 0,
    `element` varchar(6) NOT NULL DEFAULT '',
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
    `npcid` int(10) NOT NULL DEFAULT 0,
    `hpg` int(10) NOT NULL DEFAULT 0,
    `hp` int(10) NOT NULL DEFAULT 0,
    `atkg` int(10) NOT NULL DEFAULT 0,
    `spatkg` int(10) NOT NULL DEFAULT 0,
    `defg` int(10) NOT NULL DEFAULT 0,
    `spdefg` int(10) NOT NULL DEFAULT 0,
    `sdg` int(10) NOT NULL DEFAULT 0,
    `allure` int(10) NOT NULL DEFAULT 0,
    `capture` int(10) NOT NULL DEFAULT 0,
    `level` int(10) NOT NULL DEFAULT 0,
    `dataall` int(10) NOT NULL DEFAULT 0,
    `datawin` int(10) NOT NULL DEFAULT 0,
    `datalost` int(10) NOT NULL DEFAULT 0,
    `fullexp` bigint(20) NOT NULL DEFAULT 0,
    `boxnum` smallint(8) NOT NULL DEFAULT 9,
    `strength` tinyint(3) NOT NULL DEFAULT 1,
    `str` int(10) NOT NULL DEFAULT 100,
    `money` bigint(20) NOT NULL DEFAULT 0,
    PRIMARY KEY (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- User's owned Pokemon
CREATE TABLE IF NOT EXISTS `pm_mypm` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `pctime` int(10) NOT NULL DEFAULT 0,
    `uid` mediumint(9) NOT NULL DEFAULT 0,
    `pmname` varchar(30) NOT NULL DEFAULT '',
    `nickname` varchar(30) NOT NULL DEFAULT '',
    `species_id` smallint(8) unsigned NOT NULL DEFAULT 0,
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
    `is_shiny` tinyint(1) NOT NULL DEFAULT 0,
    `equipmentid1` mediumint(8) NOT NULL DEFAULT 0,
    `equipmentid2` mediumint(8) NOT NULL DEFAULT 0,
    `equipmentid3` mediumint(8) NOT NULL DEFAULT 0,
    `equipmentid4` mediumint(8) NOT NULL DEFAULT 0,
    `created_at` int(10) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_uid` (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- User's owned items
CREATE TABLE IF NOT EXISTS `pm_myitem` (
    `id` int(8) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(9) NOT NULL DEFAULT 0,
    `itemid` varchar(5) NOT NULL DEFAULT '',
    `nums` smallint(5) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_uid` (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- User's owned skills
CREATE TABLE IF NOT EXISTS `pm_myskill` (
    `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
    `petid` mediumint(8) unsigned NOT NULL DEFAULT 0,
    `skillid` mediumint(8) unsigned NOT NULL DEFAULT 0,
    `skillnum` smallint(5) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_uid_petid` (`uid`, `petid`),
    KEY `idx_petid` (`petid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- PC storage
CREATE TABLE IF NOT EXISTS `pm_pc` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `uid` mediumint(8) unsigned NOT NULL,
    `pm_id` mediumint(8) unsigned NOT NULL,
    `deposited_at` int(10) unsigned NOT NULL DEFAULT 0,
    `healed_at` int(10) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_uid` (`uid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Pokemon boxes
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
    KEY `idx_uid_2` (`uid_2`),
    KEY `idx_created_at` (`created_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- Battle engine 2.0: battle header (issue #75 phase 1; one row per battle)
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

-- Battle engine 2.0: participating units (stats snapshot, stages, status, volatile, buffs)
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

-- Battle engine 2.0: battle report / replay event stream
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
