<?php
defined('IN_DISCUZ') || exit('Access Denied');

class plugin_pokemon_forum
{
    private static function get_imgdir()
    {
        global $_G;
        return $_G['cache']['plugin']['pokemon']['imgdir'] ?: 'source/plugin/pokemon/images';
    }

    private static function pet_img_url($pet)
    {
        return "source/plugin/pokemon/images/pm/{$pet['pmno']}.png";
    }

    private static function pet_small_url($pmno)
    {
        return "source/plugin/pokemon/images/spm/$pmno.gif";
    }

    private static function first_pet_html($pet)
    {
        $img_url = self::pet_img_url($pet);
        $href = "plugin.php?id=pokemon:game";
        return <<<HTML
<div style="padding:6px 0;text-align:center">
  <a href="$href" target="_blank">
    <img src="$img_url" style="width:auto;height:80px;padding:0 24px;" border="0">
  </a>
  <div style="margin-top:2px;font-size:12px">{$pet['nowname']} Lv.{$pet['level']}</div>
</div>
HTML;
    }

    private static function creep_pet_html($pet)
    {
        $href = "plugin.php?id=pokemon:pokemon&index=ajax_pm&petid={$pet['id']}&action=show&cshu=2";
        $onclick = "showWindow('pokemon',this.href);return false;";
        $src = self::pet_small_url($pet['pmno']);
        $title = htmlspecialchars("{$pet['nowname']} Lv:{$pet['level']}");
        return <<<HTML
<a href="$href" onclick="$onclick"><img src="$src" title="$title" border="0"></a>
HTML;
    }

    function viewthread_sidebottom_output()
    {
        global $postlist, $_G;
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
        foreach ($rows as $row) {
            $uid = $row['uid'];
            $data = $row['pokemon'];
            if (empty($data)) {
                continue;
            }
            $data = dunserialize($data);
            $html = '';
            $first = $data['first'];
            if ($first) {
                $html = self::first_pet_html($first);
            }
            $creeps = $data['creeps'];
            if ($creeps) {
                $html .= '<div style="text-align:center">';
                foreach ($creeps as $pet) {
                    $html .= self::creep_pet_html($pet) . ' ';
                }
                $html .= '</div>';
            }
            if ($html) {
                $badge[$uid] = $html;
            }
        }

        $out = [];
        foreach ($postlist as $post) {
            $uid = $post['uid'];
            $out[] = isset($badge[$uid]) ? '<div class="tns xg2">' . $badge[$uid] . '</div>' : '';
        }
        return $out;
    }
}
