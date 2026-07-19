<?php
defined('IN_DISCUZ') || exit('Access Denied');

class plugin_pokemon {

    function global_usernav_extra1() {
        global $_G;
        $href = 'plugin.php?id=pokemon:game';
        $title = lang('plugin/pokemon', 'game');
        return '<span class="pipe">|</span><a href="' . $href . '">' . $title . '</a>' . "\n";
    }
}
