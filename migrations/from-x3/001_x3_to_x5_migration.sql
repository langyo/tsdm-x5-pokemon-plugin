-- ============================================================
-- TSDM Pokemon Plugin — X3/X2 to X5 canonical schema migration
-- ============================================================
-- Idempotent: safe to run on fresh installs (no-op), partially
-- migrated databases, and old X3/X2 schemas.  Each table block
-- checks whether the TABLE exists and whether OLD columns still
-- exist before attempting any UPDATE / DROP.  Uses a stored
-- procedure so that IF logic is available; the procedure is
-- created, executed, and dropped.
-- ============================================================

SET sql_mode = '';
SET NAMES utf8mb4;

-- Migration log (always ensure it exists)
CREATE TABLE IF NOT EXISTS `pm_migration_log` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `step` varchar(80) NOT NULL,
    `status` tinyint(1) NOT NULL DEFAULT 0,
    `message` text,
    `executed_at` int(10) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;

-- ============================================================
-- Stored procedure: conditional migration
-- ============================================================

DROP PROCEDURE IF EXISTS `pm_migrate_x3_to_x5`;

DELIMITER $$

CREATE PROCEDURE `pm_migrate_x3_to_x5`()
BEGIN
    DECLARE tbl_exists INT DEFAULT 0;
    DECLARE has_old    INT DEFAULT 0;

    -- ========================================================
    -- Helper: check table existence + old-column existence
    -- ========================================================

    -- --------------------------------------------------------
    -- pm_data
    -- --------------------------------------------------------
    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_data';

    IF tbl_exists > 0 THEN
        SELECT COUNT(*) INTO has_old
          FROM information_schema.COLUMNS
         WHERE TABLE_SCHEMA = DATABASE()
           AND TABLE_NAME = 'pm_data'
           AND COLUMN_NAME IN ('txt','sd','god','hpn','minmoney');

        -- Always ensure new columns exist (idempotent)
        ALTER TABLE `pm_data`
          ADD COLUMN IF NOT EXISTS `description`   varchar(255) NOT NULL DEFAULT '' AFTER `money`,
          ADD COLUMN IF NOT EXISTS `speed`         smallint(6)  NOT NULL DEFAULT 0  AFTER `spdef`,
          ADD COLUMN IF NOT EXISTS `is_legendary`  tinyint(1)   NOT NULL DEFAULT 0  AFTER `birth`,
          ADD COLUMN IF NOT EXISTS `effort_values` text         NOT NULL            AFTER `shop`,
          ADD COLUMN IF NOT EXISTS `drop_money`    varchar(60)  NOT NULL DEFAULT '' AFTER `is_legendary`;

        IF has_old > 0 THEN
            UPDATE `pm_data` SET
              `description`   = IFNULL(`txt`,  `description`),
              `speed`         = IFNULL(`sd`,   `speed`),
              `is_legendary`  = IFNULL(`god`,  `is_legendary`),
              `effort_values` = IF(`effort_values` = '' OR `effort_values` IS NULL,
                  JSON_OBJECT('hp',   IFNULL(`hpn`,0),
                              'atk',  IFNULL(`atkn`,0),
                              'def',  IFNULL(`defn`,0),
                              'spatk',IFNULL(`spatkn`,0),
                              'spdef',IFNULL(`spdefn`,0),
                              'spd',  IFNULL(`sdn`,0)),
                  `effort_values`),
              `drop_money`    = IF(`drop_money` = '' OR `drop_money` IS NULL,
                  CONCAT('[', IFNULL(`minmoney`,0), ',', IFNULL(`maxmoney`,0), ']'),
                  `drop_money`)
            WHERE `txt` IS NOT NULL OR `sd` IS NOT NULL OR `god` IS NOT NULL
               OR `hpn` IS NOT NULL OR `minmoney` IS NOT NULL;

            ALTER TABLE `pm_data`
              DROP COLUMN IF EXISTS `txt`,
              DROP COLUMN IF EXISTS `sd`,
              DROP COLUMN IF EXISTS `god`,
              DROP COLUMN IF EXISTS `hpn`,
              DROP COLUMN IF EXISTS `atkn`,
              DROP COLUMN IF EXISTS `defn`,
              DROP COLUMN IF EXISTS `spatkn`,
              DROP COLUMN IF EXISTS `spdefn`,
              DROP COLUMN IF EXISTS `sdn`,
              DROP COLUMN IF EXISTS `minmoney`,
              DROP COLUMN IF EXISTS `maxmoney`,
              DROP COLUMN IF EXISTS `minmoeny`,
              DROP COLUMN IF EXISTS `mixmoeny`,
              DROP COLUMN IF EXISTS `birthodds`,
              DROP COLUMN IF EXISTS `pnclevel`;

            INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
            VALUES ('pm_data', 1, 'Migrated old columns to new schema', UNIX_TIMESTAMP());
        ELSE
            INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
            VALUES ('pm_data', 1, 'Already new schema (skipped)', UNIX_TIMESTAMP());
        END IF;
    ELSE
        INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
        VALUES ('pm_data', 0, 'Table does not exist (skipped)', UNIX_TIMESTAMP());
    END IF;

    -- --------------------------------------------------------
    -- pm_mypm
    -- --------------------------------------------------------
    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_mypm';

    IF tbl_exists > 0 THEN
        SELECT COUNT(*) INTO has_old
          FROM information_schema.COLUMNS
         WHERE TABLE_SCHEMA = DATABASE()
           AND TABLE_NAME = 'pm_mypm'
           AND COLUMN_NAME IN ('nowname','pmno','sg');

        ALTER TABLE `pm_mypm`
          ADD COLUMN IF NOT EXISTS `nickname`   varchar(30) NOT NULL DEFAULT '' AFTER `pmname`,
          ADD COLUMN IF NOT EXISTS `species_id` smallint(8) unsigned NOT NULL DEFAULT 0 AFTER `uid`,
          ADD COLUMN IF NOT EXISTS `is_shiny`   tinyint(1) NOT NULL DEFAULT 0 AFTER `swap`,
          ADD COLUMN IF NOT EXISTS `created_at` int(10) unsigned NOT NULL DEFAULT 0;

        IF has_old > 0 THEN
            UPDATE `pm_mypm` SET
              `nickname`   = IFNULL(`nowname`, `nickname`),
              `species_id` = IFNULL(`pmno`,    `species_id`),
              `is_shiny`   = IFNULL(`sg`,      `is_shiny`)
            WHERE `nowname` IS NOT NULL OR `pmno` IS NOT NULL OR `sg` IS NOT NULL;

            ALTER TABLE `pm_mypm`
              DROP COLUMN IF EXISTS `nowname`,
              DROP COLUMN IF EXISTS `pmno`,
              DROP COLUMN IF EXISTS `sg`,
              DROP COLUMN IF EXISTS `pcid`;

            INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
            VALUES ('pm_mypm', 1, 'Migrated old columns to new schema', UNIX_TIMESTAMP());
        ELSE
            INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
            VALUES ('pm_mypm', 1, 'Already new schema (skipped)', UNIX_TIMESTAMP());
        END IF;
    ELSE
        INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
        VALUES ('pm_mypm', 0, 'Table does not exist (skipped)', UNIX_TIMESTAMP());
    END IF;

    -- --------------------------------------------------------
    -- pm_skill
    -- --------------------------------------------------------
    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_skill';

    IF tbl_exists > 0 THEN
        SELECT COUNT(*) INTO has_old
          FROM information_schema.COLUMNS
         WHERE TABLE_SCHEMA = DATABASE()
           AND TABLE_NAME = 'pm_skill'
           AND COLUMN_NAME IN ('pmid','txt','lv','powr','num','tn');

        ALTER TABLE `pm_skill`
          ADD COLUMN IF NOT EXISTS `available_pokemons` mediumtext NOT NULL AFTER `id`,
          ADD COLUMN IF NOT EXISTS `description`        mediumtext NOT NULL AFTER `name`,
          ADD COLUMN IF NOT EXISTS `level_required`     smallint(5) NOT NULL DEFAULT 0 AFTER `description`,
          ADD COLUMN IF NOT EXISTS `power`              mediumint(8) NOT NULL DEFAULT 40 AFTER `level_required`,
          ADD COLUMN IF NOT EXISTS `max_uses`           smallint(5) NOT NULL DEFAULT 35 AFTER `power`,
          ADD COLUMN IF NOT EXISTS `element`            varchar(6) NOT NULL DEFAULT '' AFTER `type`;

        IF has_old > 0 THEN
            UPDATE `pm_skill` SET
              `available_pokemons` = IFNULL(`pmid`, `available_pokemons`),
              `description`        = IFNULL(`txt`,  `description`),
              `level_required`     = IFNULL(`lv`,   `level_required`),
              `power`              = IFNULL(`powr`, `power`),
              `max_uses`           = IFNULL(`num`,  `max_uses`),
              `element`            = IFNULL(`tn`,   `element`)
            WHERE `pmid` IS NOT NULL OR `txt` IS NOT NULL OR `lv` IS NOT NULL;

            ALTER TABLE `pm_skill`
              DROP COLUMN IF EXISTS `pmid`,
              DROP COLUMN IF EXISTS `txt`,
              DROP COLUMN IF EXISTS `lv`,
              DROP COLUMN IF EXISTS `powr`,
              DROP COLUMN IF EXISTS `num`,
              DROP COLUMN IF EXISTS `tn`;

            INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
            VALUES ('pm_skill', 1, 'Migrated old columns to new schema', UNIX_TIMESTAMP());
        ELSE
            INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
            VALUES ('pm_skill', 1, 'Already new schema (skipped)', UNIX_TIMESTAMP());
        END IF;
    ELSE
        INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
        VALUES ('pm_skill', 0, 'Table does not exist (skipped)', UNIX_TIMESTAMP());
    END IF;

    -- --------------------------------------------------------
    -- pm_map
    -- --------------------------------------------------------
    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_map';

    IF tbl_exists > 0 THEN
        SELECT COUNT(*) INTO has_old
          FROM information_schema.COLUMNS
         WHERE TABLE_SCHEMA = DATABASE()
           AND TABLE_NAME = 'pm_map'
           AND COLUMN_NAME IN ('kg','minlevel','maxlevel','exp','expn');

        ALTER TABLE `pm_map`
          ADD COLUMN IF NOT EXISTS `is_enabled`  tinyint(1) NOT NULL DEFAULT 1 AFTER `name`,
          ADD COLUMN IF NOT EXISTS `min_level`   tinyint(3) unsigned NOT NULL DEFAULT 1 AFTER `is_enabled`,
          ADD COLUMN IF NOT EXISTS `max_level`   tinyint(3) unsigned NOT NULL DEFAULT 1 AFTER `min_level`,
          ADD COLUMN IF NOT EXISTS `experience`  int(10) NOT NULL DEFAULT 0 AFTER `max_level`,
          ADD COLUMN IF NOT EXISTS `boss_config` text NOT NULL AFTER `site`;

        IF has_old > 0 THEN
            UPDATE `pm_map` SET
              `is_enabled`  = IFNULL(`kg`,       `is_enabled`),
              `min_level`   = IFNULL(`minlevel`, `min_level`),
              `max_level`   = IFNULL(`maxlevel`, `max_level`),
              `experience`  = IFNULL(`exp`,      `experience`),
              `boss_config` = IF((`boss_config` = '' OR `boss_config` IS NULL) AND `expn` IS NOT NULL,
                                 `expn`, `boss_config`)
            WHERE `kg` IS NOT NULL OR `minlevel` IS NOT NULL OR `exp` IS NOT NULL OR `expn` IS NOT NULL;

            ALTER TABLE `pm_map`
              DROP COLUMN IF EXISTS `kg`,
              DROP COLUMN IF EXISTS `minlevel`,
              DROP COLUMN IF EXISTS `maxlevel`,
              DROP COLUMN IF EXISTS `exp`,
              DROP COLUMN IF EXISTS `expn`;

            INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
            VALUES ('pm_map', 1, 'Migrated old columns to new schema', UNIX_TIMESTAMP());
        ELSE
            INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
            VALUES ('pm_map', 1, 'Already new schema (skipped)', UNIX_TIMESTAMP());
        END IF;
    ELSE
        INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
        VALUES ('pm_map', 0, 'Table does not exist (skipped)', UNIX_TIMESTAMP());
    END IF;

    -- --------------------------------------------------------
    -- pm_itemdata
    -- --------------------------------------------------------
    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_itemdata';

    IF tbl_exists > 0 THEN
        SELECT COUNT(*) INTO has_old
          FROM information_schema.COLUMNS
         WHERE TABLE_SCHEMA = DATABASE()
           AND TABLE_NAME = 'pm_itemdata'
           AND COLUMN_NAME IN ('txt','addhp','equipment_hp');

        ALTER TABLE `pm_itemdata`
          ADD COLUMN IF NOT EXISTS `description` varchar(255) NOT NULL DEFAULT '' AFTER `tpname`,
          ADD COLUMN IF NOT EXISTS `module`      varchar(30)  NOT NULL DEFAULT '' AFTER `type`,
          ADD COLUMN IF NOT EXISTS `effects`     text NOT NULL AFTER `xsask`,
          ADD COLUMN IF NOT EXISTS `equipment`   text NOT NULL AFTER `zbtype`;

        IF has_old > 0 THEN
            UPDATE `pm_itemdata` SET
              `description` = IFNULL(`txt`, `description`),
              `effects`     = IF(`effects` = '' OR `effects` IS NULL,
                  JSON_OBJECT('hp',       IFNULL(`addhp`,0),
                              'exp',      IFNULL(`addexp`,0),
                              'level',    IFNULL(`addlv`,0),
                              'intimacy', IFNULL(`addgood`,0)),
                  `effects`),
              `equipment`   = IF(`equipment` = '' OR `equipment` IS NULL,
                  JSON_OBJECT('hp',   IFNULL(`equipment_hp`,0),
                              'atk',  IFNULL(`equipment_atk`,0),
                              'def',  IFNULL(`equipment_def`,0),
                              'spatk',IFNULL(`equipment_spatk`,0),
                              'spdef',IFNULL(`equipment_spdef`,0),
                              'spd',  IFNULL(`equipment_sd`,0)),
                  `equipment`)
            WHERE `txt` IS NOT NULL OR `addhp` IS NOT NULL OR `equipment_hp` IS NOT NULL;

            ALTER TABLE `pm_itemdata`
              DROP COLUMN IF EXISTS `txt`,
              DROP COLUMN IF EXISTS `intro`,
              DROP COLUMN IF EXISTS `addhp`,
              DROP COLUMN IF EXISTS `addexp`,
              DROP COLUMN IF EXISTS `addlv`,
              DROP COLUMN IF EXISTS `addgood`,
              DROP COLUMN IF EXISTS `captmin`,
              DROP COLUMN IF EXISTS `sitemid`,
              DROP COLUMN IF EXISTS `ppkallow`,
              DROP COLUMN IF EXISTS `hot`,
              DROP COLUMN IF EXISTS `equipment_hp`,
              DROP COLUMN IF EXISTS `equipment_atk`,
              DROP COLUMN IF EXISTS `equipment_def`,
              DROP COLUMN IF EXISTS `equipment_spatk`,
              DROP COLUMN IF EXISTS `equipment_spdef`,
              DROP COLUMN IF EXISTS `equipment_sd`,
              DROP COLUMN IF EXISTS `atk`,
              DROP COLUMN IF EXISTS `def`,
              DROP COLUMN IF EXISTS `spatk`,
              DROP COLUMN IF EXISTS `spdef`,
              DROP COLUMN IF EXISTS `speed`;

            INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
            VALUES ('pm_itemdata', 1, 'Migrated old columns to new schema', UNIX_TIMESTAMP());
        ELSE
            INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
            VALUES ('pm_itemdata', 1, 'Already new schema (skipped)', UNIX_TIMESTAMP());
        END IF;
    ELSE
        INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
        VALUES ('pm_itemdata', 0, 'Table does not exist (skipped)', UNIX_TIMESTAMP());
    END IF;

    -- --------------------------------------------------------
    -- pm_evolution: add priority if missing
    -- --------------------------------------------------------
    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_evolution';

    IF tbl_exists > 0 THEN
        ALTER TABLE `pm_evolution`
          ADD COLUMN IF NOT EXISTS `priority` int(10) NOT NULL DEFAULT 0;
    END IF;

    -- --------------------------------------------------------
    -- pm_usersdata: drop dead columns (always safe with IF EXISTS)
    -- --------------------------------------------------------
    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_usersdata';

    IF tbl_exists > 0 THEN
        ALTER TABLE `pm_usersdata`
          DROP COLUMN IF EXISTS `pcid`,
          DROP COLUMN IF EXISTS `ppkname`,
          DROP COLUMN IF EXISTS `ppktime`,
          DROP COLUMN IF EXISTS `ppkround`,
          DROP COLUMN IF EXISTS `ppk`,
          DROP COLUMN IF EXISTS `ppkfight`,
          DROP COLUMN IF EXISTS `ppkdodge`,
          DROP COLUMN IF EXISTS `ppkot`,
          DROP COLUMN IF EXISTS `ppkpriority`,
          DROP COLUMN IF EXISTS `exchanguid`,
          DROP COLUMN IF EXISTS `exchangepmid`,
          DROP COLUMN IF EXISTS `npcsg`,
          DROP COLUMN IF EXISTS `hpn`,
          DROP COLUMN IF EXISTS `atkn`,
          DROP COLUMN IF EXISTS `defn`,
          DROP COLUMN IF EXISTS `spatkn`,
          DROP COLUMN IF EXISTS `spdefn`,
          DROP COLUMN IF EXISTS `sdn`;
    END IF;

    -- --------------------------------------------------------
    -- pm_myitem / pm_myskill / pm_pc / pm_box: drop dead columns
    -- --------------------------------------------------------
    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_myitem';

    IF tbl_exists > 0 THEN
        ALTER TABLE `pm_myitem`
          DROP COLUMN IF EXISTS `pcid`,
          DROP COLUMN IF EXISTS `typeid`,
          DROP COLUMN IF EXISTS `ball`,
          DROP COLUMN IF EXISTS `pmid`;
    END IF;

    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_myskill';

    IF tbl_exists > 0 THEN
        ALTER TABLE `pm_myskill` DROP COLUMN IF EXISTS `pcid`;
    END IF;

    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_pc';

    IF tbl_exists > 0 THEN
        ALTER TABLE `pm_pc` DROP COLUMN IF EXISTS `pcid`;
    END IF;

    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_box';

    IF tbl_exists > 0 THEN
        ALTER TABLE `pm_box` DROP COLUMN IF EXISTS `pcid`;
    END IF;

    -- --------------------------------------------------------
    -- Indexes
    -- --------------------------------------------------------
    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_mypm';

    IF tbl_exists > 0 THEN
        ALTER TABLE `pm_mypm` ADD INDEX IF NOT EXISTS `idx_uid` (`uid`);
    END IF;

    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_myitem';

    IF tbl_exists > 0 THEN
        ALTER TABLE `pm_myitem` ADD INDEX IF NOT EXISTS `idx_uid` (`uid`);
    END IF;

    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_myskill';

    IF tbl_exists > 0 THEN
        ALTER TABLE `pm_myskill` ADD INDEX IF NOT EXISTS `idx_uid_petid` (`uid`, `petid`);
        ALTER TABLE `pm_myskill` ADD INDEX IF NOT EXISTS `idx_petid` (`petid`);
    END IF;

    SELECT COUNT(*) INTO tbl_exists
      FROM information_schema.TABLES
     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'pm_battle_log';

    IF tbl_exists > 0 THEN
        ALTER TABLE `pm_battle_log` ADD INDEX IF NOT EXISTS `idx_uid_2` (`uid_2`);
    END IF;

    INSERT INTO `pm_migration_log` (`step`,`status`,`message`,`executed_at`)
    VALUES ('x3_to_x5_full', 1, 'Migration complete', UNIX_TIMESTAMP());
END$$

DELIMITER ;

-- Execute and clean up
CALL `pm_migrate_x3_to_x5`();
DROP PROCEDURE IF EXISTS `pm_migrate_x3_to_x5`;
