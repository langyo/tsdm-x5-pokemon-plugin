<?php
/**
 * API Bootstrap - Discuz环境加载器
 *
 * 当API文件被直接访问时加载Discuz环境
 */

// 开启输出缓冲，防止 Discuz 核心输出任何内容
ob_start();

// 定义常量
define('IN_DISCUZ', true);

// 引入Discuz核心
require_once __DIR__ . '/../../../../../source/class/class_core.php';

$discuz = & discuz_core::instance();

$discuz->cachelist = $cachelist = $discuz->init_cachelist();
$discuz->init();
$discuz->init_user();

// 清除输出缓冲，丢弃 Discuz 初始化的任何输出
ob_end_clean();
