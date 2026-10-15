<?php
if (!defined('API_ROUTED')) exit;

// 与 postspm.class.php 服务端注入共用同一渲染库（pet_side_html.php），
// 保证两条注入路径输出一致：图床回退链、大小图切换、点击查看宠物数据卡。
require_once dirname(__DIR__) . '/pet_side_html.php';

$uids = isset($_GET['uids']) ? $_GET['uids'] : '';
$uidList = array_filter(array_map('intval', explode(',', $uids)));
if (empty($uidList)) {
    echo json_encode(['success' => false, 'error' => 'no uids']);
    exit;
}

$rows = DB::fetch_all(
    'SELECT uid, pokemon FROM %t WHERE uid IN (%n)',
    ['common_member_field_forum', $uidList]
);

$data = [];
foreach ($rows as $row) {
    $uid = $row['uid'];
    $pdata = $row['pokemon'];
    if (empty($pdata)) continue;
    $pdata = dunserialize($pdata);
    if (!is_array($pdata)) continue;
    $html = pm_side_badge_html($pdata);
    if ($html !== '') $data[$uid] = $html;
}

echo json_encode(['success' => true, 'data' => $data], JSON_UNESCAPED_UNICODE);
