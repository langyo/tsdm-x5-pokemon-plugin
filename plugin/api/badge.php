<?php
/**
 * 训练师徽章图片接口 — 转发到插件根目录的 badge.inc.php。
 * pre_forum_medal 中勋章图片地址引用 plugin.php?id=pokemon:pokemon&endpoint=badge。
 */

if (!defined('IN_DISCUZ') && !defined('API_ROUTED')) exit;

require_once dirname(__DIR__) . '/badge.inc.php';
