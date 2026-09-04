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
        if (defined('CURSCRIPT') && CURSCRIPT === 'forum' && defined('CURMODULE') && CURMODULE === 'viewthread') {
            $_G['pokemon_badge_js'] = true;
        }
    }

    // 帖子页宠物徽章由 postspm.class.php 的 viewthread_sidebottom_output 服务端注入，
    // 不再走 global_header 的 JS 注入（旧选择器只匹配部分模板，且会与钩子重复渲染）。
    // 注意：这里不要声明 viewthread_sidebottom_output 桩方法——两个 type-11 模块
    // 的类都会被 hookscript 缓存扫描，桩会与 postspm 的真实输出重复注册。
    function global_header() {
        return '';
    }
}
