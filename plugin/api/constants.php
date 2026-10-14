<?php

/**
 * Pokemon Plugin - API Constants
 *
 * 集中管理所有魔法数字和常量定义
 */

// 禁止直接访问
if (!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

// ==================== 战斗系统常量 ====================

// 战斗奖励
define('BATTLE_POWER_MIN', 10);
define('BATTLE_POWER_MAX', 100);
define('BATTLE_BASE_EXP', 50);
define('BATTLE_MAX_SKILLS', 4);

// 战斗计算
define('CRITICAL_HIT_CHANCE', 5); // 5% 会心一击概率
define('CRITICAL_HIT_MULTIPLIER', 2); // 会心一击2倍伤害
define('DAMAGE_VARIANCE_MIN', 85);
define('DAMAGE_VARIANCE_MAX', 100);

// 逃跑概率
define('FLEE_CHECK_MAX', 100);

// 野怪生成
define('SHINY_CHANCE', 4096); // 1/4096 概率闪光
define('GENDER_UNKNOWN', 2); // 未知性别

// ==================== 物品和商店常量 ====================

// 分页
// 商店宫格为 4 列 × 2 行，每页正好铺满一屏，翻页交给前端换页器。
define('SHOP_ITEMS_PER_PAGE', 8);
define('USER_ITEMS_PER_PAGE', 50);

// 物品状态
define('ITEM_AVAILABLE', 1);

// 物品治疗效果
define('ITEM_HEAL_MAX_HP', -1); // 完全回复HP

// 成就阈值
define('HIGH_LEVEL_THRESHOLD', 100);
define('COLLECTOR_THRESHOLD', 50);

// ==================== 用户等级常量 ====================

define('USER_ADMIN_ID', 1); // 管理员ID

// ==================== API响应常量 ====================

// HTTP状态码
define('HTTP_OK', 200);
define('HTTP_BAD_REQUEST', 400);
define('HTTP_UNAUTHORIZED', 401);
define('HTTP_INTERNAL_ERROR', 500);

// ==================== 游戏机制常量 ====================

// 经验值表类型
define('EXP_TABLE_SLOW', 60);
define('EXP_TABLE_MEDIUM', 80);
define('EXP_TABLE_FAST', 100);

// 宠物状态
define('POKEMON_IS_ZD', 1); // 站场宠物
