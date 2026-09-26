<?php
/**
 * Discuz X5 UCenter 独立模式配置（由 docker/entrypoint.sh 首次启动时拷贝）。
 *
 * X5 已移除独立 uc_server；此配置以独立模式（UC_STANDALONE=1）直连本库的
 * pre_ucenter_* 表，并复用 config_global.php 的数据库连接与 authkey。
 */
define('UC_CONNECT', 'mysql');
define('UC_STANDALONE', 1);

require 'config_global.php';
define('UC_DBHOST', $_config['db'][1]['dbhost']);
define('UC_DBUSER', $_config['db'][1]['dbuser']);
define('UC_DBPW', $_config['db'][1]['dbpw']);
define('UC_DBNAME', $_config['db'][1]['dbname']);
define('UC_DBTABLEPRE', '`'.$_config['db'][1]['dbname'].'`.'.$_config['db'][1]['tablepre'].'ucenter_');
define('UC_KEY', $_config['security']['authkey']);

define('UC_DBCHARSET', 'utf8mb4');
define('UC_DBCONNECT', 0);

define('UC_AVTURL', '');
define('UC_AVTPATH', '');

define('UC_CHARSET', 'utf-8');
define('UC_API', '');
define('UC_APPID', '1');
define('UC_IP', '');
define('UC_PPP', 20);
