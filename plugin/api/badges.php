<?php
if (!defined('API_ROUTED')) exit;

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
    $html = '';
    if (!empty($pdata['first'])) {
        $pet = $pdata['first'];
        $img = "source/plugin/pokemon/images/pm/{$pet['pmno']}.png";
        $html .= '<div style="padding:4px 0">';
        $html .= '<a href="plugin.php?id=pokemon:game" target="_blank">';
        $html .= '<img src="' . $img . '" style="width:auto;height:80px;padding:0 24px" border="0">';
        $html .= '</a>';
        $html .= '<div style="margin-top:2px;font-size:12px">' . htmlspecialchars($pet['nowname']) . ' Lv.' . $pet['level'] . '</div>';
        $html .= '</div>';
    }
    if (!empty($pdata['creeps'])) {
        $html .= '<div style="text-align:center">';
        foreach ($pdata['creeps'] as $creep) {
            $src = "source/plugin/pokemon/images/spm/{$creep['pmno']}.gif";
            $title = htmlspecialchars("{$creep['nowname']} Lv:{$creep['level']}");
            $html .= '<img src="' . $src . '" title="' . $title . '" border="0" style="width:32px;height:32px;margin:1px">';
        }
        $html .= '</div>';
    }
    if ($html) $data[$uid] = $html;
}

echo json_encode(['success' => true, 'data' => $data], JSON_UNESCAPED_UNICODE);
