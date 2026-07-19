<?php
defined('IN_DISCUZ') || exit('Access Denied');

class plugin_pokemon {

    function global_header() {
        global $_G;
        $href = 'plugin.php?id=pokemon:game';
        $title = lang('plugin/pokemon', 'game');
        return '<li id="mn_pokemon"><a href="' . $href . '" hidefocus="true">' . $title . '</a></li>';
    }
}
