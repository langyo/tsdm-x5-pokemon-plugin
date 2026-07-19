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
?>
<iframe src="/source/plugin/pokemon/wasm/index.html" style="width:100%;min-height:600px;border:none;"></iframe>
<?php
include template('common/footer');
