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
        return "https://img.tsdm39.com/Pokemon/pm/{$pet['pmno']}.gif";
    }

    private static function pet_small_url($pmno)
    {
        $dir = self::get_imgdir();
        return "$dir/spm/$pmno.gif";
    }

    private static function first_pet_html($pet)
    {
        $img_url = self::pet_img_url($pet);
        $href = "plugin.php?id=pokemon:pokemon&index=ajax_pm&petid={$pet['id']}&action=show&cshu=2";
        $onclick = "showWindow('pokemon',this.href);return false;";
        return <<<HTML
<div align="center">
  <a href="$href" onclick="$onclick" target="_blank">
    <img src="$img_url" border="0">
  </a>
</div>
<div align="center">
  <a href="$href" onclick="$onclick">{$pet['nowname']}</a> Lv:{$pet['level']}
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
