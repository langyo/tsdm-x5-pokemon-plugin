<?php
defined('IN_DISCUZ') || exit('Access Denied');

class plugin_pokemon {

    function common() {
        global $_G;
        $exists = false;
        if (!empty($_G['setting']['navs'])) {
            foreach ($_G['setting']['navs'] as $nav) {
                if (isset($nav['navid']) && $nav['navid'] === 'pokemon') {
                    $exists = true;
                    break;
                }
            }
        }
        if (!$exists) {
            $_G['setting']['navs'][] = [
                'navid' => 'pokemon',
                'name' => '宠物中心',
                'parent' => '0',
                'filename' => 'plugin.php?id=pokemon:game',
                'available' => '1',
                'level' => '0',
                'subid' => '0',
                'icon' => '',
                'nav' => '<li id="mn_pokemon"><a href="plugin.php?id=pokemon:game" hidefocus="true">宠物中心</a></li>',
            ];
        }
    }

    function global_nav_extra() {
        return '';
    }
}
