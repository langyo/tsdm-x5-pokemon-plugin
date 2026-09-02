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
        $species = self::pet_species($pet);
        return "source/plugin/pokemon/images/pm/{$species}.png";
    }

    private static function pet_small_url($pet)
    {
        $species = self::pet_species($pet);
        return "source/plugin/pokemon/images/spm/{$species}.gif";
    }

    // 兼容两种数据形态：X5 刷新徽章写入的 species_id/nickname，
    // 以及 X3 历史数据遗留的 pmno/nowname。
    private static function pet_species($pet)
    {
        return intval($pet['species_id'] ?? $pet['pmno'] ?? 0);
    }

    private static function pet_name($pet)
    {
        return (string)($pet['nickname'] ?? $pet['nowname'] ?? '');
    }

    private static function pet_level($pet)
    {
        return intval($pet['level'] ?? 0);
    }

    private static function first_pet_html($pet)
    {
        $img_url = self::pet_img_url($pet);
        $name = self::pet_name($pet);
        $level = self::pet_level($pet);
        $href = "plugin.php?id=pokemon:game";
        return <<<HTML
<div style="padding:6px 0;text-align:center">
  <a href="$href" target="_blank">
    <img src="$img_url" style="width:auto;height:80px;padding:0 24px;" border="0">
  </a>
  <div style="margin-top:2px;font-size:12px">$name Lv.$level</div>
</div>
HTML;
    }

    private static function creep_pet_html($pet)
    {
        $src = self::pet_small_url($pet);
        $title = htmlspecialchars(self::pet_name($pet) . ' Lv:' . self::pet_level($pet), ENT_QUOTES);
        $href = "plugin.php?id=pokemon:pokemon&index=ajax_pm&petid=" . intval($pet['id'] ?? 0) . "&action=show&cshu=2";
        $onclick = "showWindow('pokemon',this.href);return false;";
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
            if (!is_array($data)) {
                continue;
            }
            $html = '';
            $first = $data['first'] ?? null;
            if (is_array($first) && self::pet_species($first)) {
                $html = self::first_pet_html($first);
            }
            $creeps = $data['creeps'] ?? null;
            if (is_array($creeps)) {
                $creeps_html = '';
                foreach ($creeps as $pet) {
                    if (!is_array($pet) || !self::pet_species($pet)) {
                        continue;
                    }
                    $creeps_html .= self::creep_pet_html($pet) . ' ';
                }
                if ($creeps_html !== '') {
                    $html .= '<div style="text-align:center">' . $creeps_html . '</div>';
                }
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
