<?php
defined('IN_DISCUZ') || exit('Access Denied');

include template('common/header_common');
include template('common/header');
?>
<div id="pokemon-app"></div>
<link rel="stylesheet" href="source/plugin/pokemon/wasm/game.css">
<script type="module">
import init from 'source/plugin/pokemon/wasm/_game.js';
init('source/plugin/pokemon/wasm/_game_bg.wasm');
</script>
<?php
include template('common/footer');
