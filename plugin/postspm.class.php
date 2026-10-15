<?php
defined('IN_DISCUZ') || exit('Access Denied');

require_once __DIR__ . '/pet_side_html.php';

class plugin_pokemon_forum
{
    function viewthread_sidebottom_output()
    {
        global $postlist, $_G;
        static $served = false;
        if ($served) {
            // 钩子在个别站点缓存/模板状态下可能被调用多次，第二次起直接返回空，避免徽章重复。
            return [];
        }
        $served = true;
        $pmpostsshow = $_G['cache']['plugin']['pokemon']['pmpostsshow'];
        if (!$_GET['tid'] || !$postlist || !$pmpostsshow) {
            return [];
        }

        $uids = [];
        foreach ($postlist as $post) {
            $uids[] = (int)$post['uid'];
        }
        $uids = array_unique($uids);
        if (!$uids) {
            return [];
        }

        $rows = DB::fetch_all('SELECT uid, pokemon FROM %t WHERE uid IN (%n)', [
            'common_member_field_forum', $uids
        ]);

        $badge = [];
        $assets_pending = true;
        foreach ($rows as $row) {
            $uid = $row['uid'];
            $data = $row['pokemon'];
            if (empty($data)) {
                continue;
            }
            $data = dunserialize($data);
            if (!is_array($data)) {
                continue;
            }
            $html = pm_side_badge_html($data);
            if ($html === '') {
                continue;
            }
            // 样式与脚本随首个徽章输出一次即可（脚本内有 __pmSideInit 防重入）。
            if ($assets_pending) {
                $html .= pm_side_assets();
                $assets_pending = false;
            }
            $badge[$uid] = $html;
        }

        $out = [];
        foreach ($postlist as $post) {
            $uid = $post['uid'];
            $out[] = isset($badge[$uid]) ? '<div class="tns xg2">' . $badge[$uid] . '</div>' : '';
        }
        return $out;
    }
}
