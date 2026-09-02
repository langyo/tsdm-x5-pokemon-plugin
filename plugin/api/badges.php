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
    if (!is_array($pdata)) continue;
    $html = '';
    // 兼容两种数据形态：X5 刷新徽章写入的 species_id/nickname，
    // 以及 X3 历史数据遗留的 pmno/nowname。
    $first = $pdata['first'] ?? null;
    if (is_array($first)) {
        $species = intval($first['species_id'] ?? $first['pmno'] ?? 0);
        $name = htmlspecialchars((string)($first['nickname'] ?? $first['nowname'] ?? ''), ENT_QUOTES);
        $level = intval($first['level'] ?? 0);
        if ($species) {
            $img = "source/plugin/pokemon/images/pm/{$species}.png";
            $html .= '<div style="padding:4px 0">';
            $html .= '<a href="plugin.php?id=pokemon:game" target="_blank">';
            $html .= '<img src="' . $img . '" style="width:auto;height:80px;padding:0 24px" border="0">';
            $html .= '</a>';
            $html .= '<div style="margin-top:2px;font-size:12px">' . $name . ' Lv.' . $level . '</div>';
            $html .= '</div>';
        }
    }
    $creeps = $pdata['creeps'] ?? null;
    if (is_array($creeps)) {
        $creeps_html = '';
        foreach ($creeps as $creep) {
            if (!is_array($creep)) continue;
            $species = intval($creep['species_id'] ?? $creep['pmno'] ?? 0);
            if (!$species) continue;
            $src = "source/plugin/pokemon/images/spm/{$species}.gif";
            $title = htmlspecialchars(
                (string)($creep['nickname'] ?? $creep['nowname'] ?? '') . ' Lv:' . intval($creep['level'] ?? 0),
                ENT_QUOTES
            );
            $creeps_html .= '<img src="' . $src . '" title="' . $title . '" border="0" style="width:32px;height:32px;margin:1px">';
        }
        if ($creeps_html !== '') {
            $html .= '<div style="text-align:center">' . $creeps_html . '</div>';
        }
    }
    if ($html) $data[$uid] = $html;
}

echo json_encode(['success' => true, 'data' => $data], JSON_UNESCAPED_UNICODE);
