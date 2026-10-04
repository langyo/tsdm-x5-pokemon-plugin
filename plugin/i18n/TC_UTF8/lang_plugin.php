<?php

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

$scriptlang['pokemon'] = [
    'name' => 'TSDM 寵物小精靈',
    'desc' => '天使動漫論壇寵物小精靈插件系統',
    'game' => '寵物中心',
    'my' => '我的寵物',
    'shop' => '寵物商店',
    'battle' => '寵物對戰',
    'pc' => 'PC中心',
    'box' => '寵物盒子',
    'evolution' => '進化',
    'map' => '冒險地圖',
    'setting' => '寵物設置',
    'hp' => 'HP',
    'atk' => '攻擊',
    'def' => '防禦',
    'spatk' => '特攻',
    'spdef' => '特防',
    'speed' => '速度',
    'level' => '等級',
    'exp' => '經驗',
    'system_closed' => '系統關閉中',
    'invalid_route' => '無效的路由',
    'admin_only' => '管理員功能',
];

// 戰鬥引擎 2.0 戰鬥文案（battle_core_render_messages 的語言包覆蓋；
// 未配置的 key 回退引擎內建簡體）。模板佔位符：{ally}/{enemy}/{skill}/
// {amount}/{stat}/{status}/{who}
$scriptlang['pokemon']['battle_text'] = [
    'used_move' => '{ally}使用了{skill}，對{enemy}造成了{amount}點傷害！',
    'enemy_used_move' => '{enemy}使用了{skill}，對{ally}造成了{amount}點傷害！',
    'counter' => '{enemy}攻擊了{ally}，造成了{amount}點傷害！',
    'miss' => '{enemy}避開了{ally}的攻擊！',
    'faint_enemy' => '{enemy}倒下了！',
    'faint_ally' => '{ally}倒下了...',
    'stage_up' => '{who}的{stat}提高了！',
    'stage_down' => '{who}的{stat}降低了！',
    'status_inflict' => '{who}陷入了{status}狀態！',
    'status_damage' => '{who}受到了{status}的傷害，{amount}點！',
    'status_prevent' => '{who}因{status}無法行動！',
    'status_cure' => '{who}的{status}治好了！',
    'switch_required' => '還有可用的替補寵物，請更換寵物繼續戰鬥！',
    'struggle' => '普通攻擊',
    'stat_names' => [
        'atk' => '攻擊', 'def' => '防禦', 'spatk' => '特攻', 'spdef' => '特防',
        'speed' => '速度', 'accuracy' => '命中', 'evasion' => '閃避',
    ],
    'status_names' => [
        'poison' => '中毒', 'burn' => '灼傷', 'paralysis' => '麻痺',
        'sleep' => '睡眠', 'freeze' => '冰凍', 'confusion' => '混亂',
    ],
];
