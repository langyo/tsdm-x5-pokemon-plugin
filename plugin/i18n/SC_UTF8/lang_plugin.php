<?php

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

$scriptlang['pokemon'] = [
    'name' => 'TSDM 宠物小精灵',
    'desc' => '天使动漫论坛宠物小精灵插件系统',
    'game' => '宠物中心',
    'my' => '我的宠物',
    'shop' => '宠物商店',
    'battle' => '宠物对战',
    'pc' => 'PC中心',
    'box' => '宠物盒子',
    'evolution' => '进化',
    'map' => '冒险地图',
    'setting' => '宠物设置',
    'hp' => 'HP',
    'atk' => '攻击',
    'def' => '防御',
    'spatk' => '特攻',
    'spdef' => '特防',
    'speed' => '速度',
    'level' => '等级',
    'exp' => '经验',
    'system_closed' => '系统关闭中',
    'invalid_route' => '无效的路由',
    'admin_only' => '管理员功能',
];

// 战斗引擎 2.0 战斗文案（battle_core_render_messages 的语言包覆盖；
// 未配置的 key 回退引擎内置简体）。模板占位符：{ally}/{enemy}/{skill}/
// {amount}/{stat}/{status}/{who}
$scriptlang['pokemon']['battle_text'] = [
    'used_move' => '{ally}使用了{skill}，对{enemy}造成了{amount}点伤害！',
    'enemy_used_move' => '{enemy}使用了{skill}，对{ally}造成了{amount}点伤害！',
    'counter' => '{enemy}攻击了{ally}，造成了{amount}点伤害！',
    'miss' => '{enemy}避开了{ally}的攻击！',
    'faint_enemy' => '{enemy}倒下了！',
    'faint_ally' => '{ally}倒下了...',
    'stage_up' => '{who}的{stat}提高了！',
    'stage_down' => '{who}的{stat}降低了！',
    'status_inflict' => '{who}陷入了{status}状态！',
    'status_damage' => '{who}受到了{status}的伤害，{amount}点！',
    'status_prevent' => '{who}因{status}无法行动！',
    'status_cure' => '{who}的{status}治好了！',
    'switch_required' => '还有可用的替补宠物，请更换宠物继续战斗！',
    'struggle' => '普通攻击',
    'stat_names' => [
        'atk' => '攻击', 'def' => '防御', 'spatk' => '特攻', 'spdef' => '特防',
        'speed' => '速度', 'accuracy' => '命中', 'evasion' => '闪避',
    ],
    'status_names' => [
        'poison' => '中毒', 'burn' => '灼烧', 'paralysis' => '麻痹',
        'sleep' => '睡眠', 'freeze' => '冰冻', 'confusion' => '混乱',
    ],
];
