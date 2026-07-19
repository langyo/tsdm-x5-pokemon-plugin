<?php

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

$sql = <<<EOF

DROP TABLE IF EXISTS pm_pc;
DROP TABLE IF EXISTS pm_battle_log;
DROP TABLE IF EXISTS pm_evolution;
DROP TABLE IF EXISTS pm_box;
DROP TABLE IF EXISTS pm_myskill;
DROP TABLE IF EXISTS pm_skill;
DROP TABLE IF EXISTS pm_map;
DROP TABLE IF EXISTS pm_config;
DROP TABLE IF EXISTS pm_myitem;
DROP TABLE IF EXISTS pm_itemdata;
DROP TABLE IF EXISTS pm_mypm;
DROP TABLE IF EXISTS pm_data;
DROP TABLE IF EXISTS pm_usersdata;

EOF;

runquery($sql);

$finish = TRUE;
