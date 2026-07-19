<?php

$_SERVER['HTTP_HOST']       = 'localhost';
$_SERVER['REQUEST_URI']     = '/misc.php?mod=initsys';
$_SERVER['REMOTE_ADDR']     = '127.0.0.1';
$_SERVER['REQUEST_METHOD']  = 'GET';
$_SERVER['SERVER_PORT']     = '80';
$_SERVER['SCRIPT_FILENAME'] = __DIR__ . '/../../../misc.php';
$_SERVER['SCRIPT_NAME']     = '/misc.php';
$_SERVER['PHP_SELF']        = '/misc.php';
$_SERVER['DOCUMENT_ROOT']   = __DIR__ . '/../../..';

define('APPTYPEID', 100);
define('CURSCRIPT', 'misc');

require __DIR__ . '/../../../source/class/class_core.php';

$discuz = &discuz_core::instance();

$discuz->init_session = false;
$discuz->init_user    = false;
$discuz->init_cron    = false;
$discuz->init_mobile  = false;
$discuz->init();

require_once libfile('function/cache');

updatecache();

echo "All caches rebuilt successfully.\n";
