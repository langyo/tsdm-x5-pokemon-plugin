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

    function global_header() {
        global $_G;
        if (empty($_G['pokemon_badge_js'])) return '';
        return "<script>
(function(){
 var u=[];document.querySelectorAll('.pls .authi cite a').forEach(function(a){var m=a.href.match(/uid=(\\d+)/);if(m)u.push(m[1])});
 if(!u.length)return;
 var x=new XMLHttpRequest();x.open('GET','plugin.php?id=pokemon:pokemon&endpoint=badges&uids='+u.join(','));
 x.onload=function(){if(x.status!=200)return;try{var d=JSON.parse(x.responseText);if(!d.success)return;
 for(var uid in d.data){var h=d.data[uid];if(!h)continue;var el=document.querySelector('.pls_favatar_'+uid);if(!el)continue;
 var div=document.createElement('div');div.className='pbm bbda cl';div.style.textAlign='center';div.innerHTML=h;
 el.parentNode.insertBefore(div,el.nextSibling);}}catch(e){}};x.send();
})();
</script>";
    }

    function viewthread_sidebottom_output() { return []; }
}
