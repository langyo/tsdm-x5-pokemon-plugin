<?php
defined('IN_DISCUZ') || exit('Access Denied');

include template('common/header_common');
include template('common/header');

echo '<div id="pt" class="bm cl">';
echo '<div class="z">';
echo '<a href="./" class="nvhm" title="首页">' . $_G['setting']['bbname'] . '</a>';
echo '<em>&raquo;</em>';
echo '<a href="plugin.php?id=pokemon:game">宠物中心</a>';
echo '</div>';
echo '</div>';

echo '<div class="bm bw0">';
echo '<div class="bm_c" style="min-height:400px;text-align:center;padding:60px 20px;">';
echo '<p style="font-size:16px;color:#999;">TSDM Pokemon Plugin</p>';
echo '<p style="font-size:13px;color:#bbb;margin-top:8px;">Vue 3 重构中，敬请期待</p>';
echo '</div>';
echo '</div>';

include template('common/footer');
