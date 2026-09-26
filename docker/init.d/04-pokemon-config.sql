-- 插件全局配置（pm_config）。历史提交曾将本文件压成单行，导致 "-- MariaDB dump" 行内注释
-- 吞掉了全部 INSERT；已按语句还原为多行。
SET sql_mode = '';
SET NAMES utf8mb4;











INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('ann_title','','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('ann_url','','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('cancel_pvp_price','100','integer');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('catch_price_in_catch_area','10','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('drop_money_config_by_global','','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('drop_money_on_pve','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('drop_money_percent_max_on_pve','0','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('drop_money_percent_min_on_pve','0','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('egg_price','10','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('hatch_egg_intimacy','10','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('intimacy_increase_multiple','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('intimacy_increase_per_earn_xp','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('intimacy_increase_per_hour','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('is_enable_buy_egg','','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('is_enable_buy_pokemon','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('is_enable_catch','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('is_enable_earn_xp_on_pve','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('is_enable_earn_xp_on_pvp','0','boolean');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('is_enable_egg','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('is_enable_multiple_eggs','','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('is_enable_pvp','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('is_open','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('medical_price','5','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('news_announcements','[]','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('posts_count_for_wakeup','10','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('pve_catch_level','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('pve_catch_xp_multiple','1','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('pvp_xp_multiple','1','integer');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('revive_time','10','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('skill_rules_json','{\"1-60+150\": 10, \"\": 80}','string');

INSERT INTO `pm_config` (`key`, `value`, `data_type`) VALUES ('version','Unknown','string');









