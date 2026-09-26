<?php
/**
 * Discuz X5 全局配置模板（由 docker/entrypoint.sh 首次启动时渲染）
 * @@*@@ 占位符会被实际环境值替换。
 */
$_config = array();
// ----------------------------  CONFIG DB  ----------------------------- //
$_config['db'][1]['dbhost'] = '@@DB_HOST@@';
$_config['db'][1]['dbuser'] = '@@DB_USER@@';
$_config['db'][1]['dbpw'] = '@@DB_PASSWORD@@';
$_config['db'][1]['dbcharset'] = 'utf8mb4';
$_config['db'][1]['pconnect'] = 0;
$_config['db'][1]['dbname'] = '@@DB_NAME@@';
$_config['db'][1]['tablepre'] = 'pre_';
$_config['db']['slave'] = '';
$_config['db']['common']['slave_except_table'] = '';
$_config['db']['common']['engine'] = 'innodb';
// --------------------------  CONFIG MEMORY  --------------------------- //
$_config['memory']['prefix'] = '@@MEMPRE@@';
$_config['memory']['redis']['server'] = '';
$_config['memory']['redis']['port'] = 6379;
$_config['memory']['redis']['pconnect'] = 1;
$_config['memory']['redis']['timeout'] = 0;
$_config['memory']['redis']['requirepass'] = '';
$_config['memory']['redis']['db'] = 0;
$_config['memory']['memcache']['server'] = '';
$_config['memory']['memcache']['port'] = 11211;
$_config['memory']['memcache']['pconnect'] = 1;
$_config['memory']['memcache']['timeout'] = 1;
$_config['memory']['memcached']['server'] = '';
$_config['memory']['memcached']['port'] = 11211;
$_config['memory']['apc'] = 0;
$_config['memory']['apcu'] = 0;
$_config['memory']['xcache'] = 0;
$_config['memory']['eaccelerator'] = 0;
$_config['memory']['wincache'] = 0;
$_config['memory']['yac'] = 0;
$_config['memory']['file']['server'] = '';
// --------------------------  CONFIG OUTPUT  --------------------------- //
$_config['output']['charset'] = 'utf-8';
$_config['output']['forceheader'] = 1;
$_config['output']['gzip'] = 0;
$_config['output']['tplrefresh'] = 1;
$_config['output']['language'] = 'zh_cn';
$_config['output']['staticurl'] = 'static/';
$_config['output']['ajaxvalidate'] = 0;
$_config['output']['upgradeinsecure'] = 0;
$_config['output']['forcehttps'] = 0;
// --------------------------  CONFIG COOKIE  --------------------------- //
$_config['cookie']['cookiepre'] = '@@COOKIEPRE@@';
$_config['cookie']['cookiedomain'] = '';
$_config['cookie']['cookiepath'] = '/';
// -------------------------  CONFIG SECURITY  -------------------------- //
$_config['security']['authkey'] = '@@AUTHKEY@@';
$_config['security']['urlxssdefend'] = 1;
$_config['security']['attackevasive'] = 0;
$_config['security']['onlyremoteaddr'] = 1;
$_config['security']['useipban'] = 1;
$_config['security']['querysafe']['status'] = 1;
$_config['security']['querysafe']['dfunction'] = array('load_file', 'hex', 'substring', 'if', 'ord', 'char');
$_config['security']['querysafe']['daction'] = array('@', 'intooutfile', 'intodumpfile', 'unionselect', '(select', 'unionall', 'uniondistinct');
$_config['security']['querysafe']['dnote'] = array('/*', '*/', '#', '--', '"');
$_config['security']['querysafe']['dlikehex'] = 1;
$_config['security']['querysafe']['afullnote'] = 0;
$_config['security']['creditsafe']['second'] = 0;
$_config['security']['creditsafe']['times'] = 10;
$_config['security']['fsockopensafe']['status'] = 0;
$_config['security']['error']['showerror'] = '1';
$_config['security']['error']['guessplugin'] = '1';
// --------------------------  CONFIG ADMINCP  -------------------------- //
$_config['admincp']['founder'] = '1';
$_config['admincp']['forcesecques'] = 0;
$_config['admincp']['checkip'] = 1;
$_config['admincp']['runquery'] = 0;
$_config['admincp']['dbimport'] = 1;
$_config['admincp']['mustlogin'] = 0;
$_config['admincp']['synclogin_front'] = 0;
$_config['admincp']['qrcode_only'] = 0;
$_config['admincp']['validate']['method'] = 'default';
$_config['admincp']['validate']['user'] = '';
$_config['admincp']['validate']['pass'] = '';
// --------------------------  CONFIG REMOTE  --------------------------- //
$_config['remote']['on'] = 0;
$_config['remote']['dir'] = 'remote';
$_config['remote']['appkey'] = '';
$_config['remote']['cron'] = 0;
// ----------------------------  CONFIG LOG  ---------------------------- //
$_config['log']['type'] = 'mysql';
// ---------------------------  CONFIG IPDB  ---------------------------- //
$_config['ipdb']['setting']['fullstack'] = '';
$_config['ipdb']['setting']['default'] = '';
$_config['ipdb']['setting']['ipv4'] = 'system';
$_config['ipdb']['setting']['ipv6'] = 'v6wry';
// -------------------------  CONFIG IPGETTER  -------------------------- //
$_config['ipgetter']['setting'] = 'header';
$_config['ipgetter']['header']['header'] = 'HTTP_X_FORWARDED_FOR';
$_config['ipgetter']['iplist']['header'] = 'HTTP_X_FORWARDED_FOR';
$_config['ipgetter']['iplist']['list'] = array('127.0.0.1');
