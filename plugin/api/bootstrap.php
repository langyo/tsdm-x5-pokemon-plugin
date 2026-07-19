<?php
/**
 * API Bootstrap - Discuz环境加载器
 * When API is accessed via plugin.php, Discuz is already initialized.
 */

if (!defined('IN_DISCUZ')) {
    ob_start();
    define('IN_DISCUZ', true);
    require_once __DIR__ . '/../../../../../source/class/class_core.php';
    $discuz = &discuz_core::instance();
    $discuz->cachelist = $discuz->init_cachelist();
    $discuz->init();
    $discuz->init_user();
    ob_end_clean();
}
