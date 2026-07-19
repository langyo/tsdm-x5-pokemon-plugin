<?php
defined('IN_DISCUZ') || exit('Access Denied');

$wasmPath = 'source/plugin/pokemon/wasm';
$wasmVer = time();

include template('common/header_common');
include template('common/header');
?>
<div id="pt" class="bm cl">
<div class="z">
<a href="./" class="nvhm" title="首页"><?php echo $_G['setting']['bbname']; ?></a><em>&raquo;</em>
<a href="plugin.php?id=pokemon:game">宠物中心</a>
</div>
</div>

<style>
#main { position:relative; max-width:1200px; margin:20px auto; min-height:80vh; }
#loading { position:absolute; top:50%; left:50%; transform:translate(-50%,-50%); font-size:24px; text-align:center; user-select:none; pointer-events:none; }
.lds-dual-ring { display:inline-block; width:24px; height:24px; }
.lds-dual-ring:after { content:" "; display:block; width:24px; height:24px; border-radius:50%; border:3px solid #d33774; border-color:#d33774 transparent #d33774 transparent; animation:lds-dual-ring 1.2s linear infinite; }
@keyframes lds-dual-ring { 0%{transform:rotate(0deg)} 100%{transform:rotate(360deg)} }
</style>
<div id="main">
<div id="loading">
<p style="font-size:16px">正在加载宠物中心</p>
<div class="lds-dual-ring"></div>
</div>
</div>
<script type="importmap">
{
  "imports": {
    "./snippets/": "<?php echo $wasmPath; ?>/snippets/",
    "./game_bg.wasm": "<?php echo $wasmPath; ?>/game_bg.wasm"
  }
}
</script>
<script type="module">
(async function() {
  try {
    const gameUrl = "<?php echo $wasmPath; ?>/game.js";
    const wasmUrl = "<?php echo $wasmPath; ?>/game_bg.wasm";
    const wasmModule = await import(gameUrl);
    await wasmModule.default(wasmUrl);
    const handle = new wasmModule.WebHandle();
    await handle.start();
    document.getElementById("loading").innerHTML = '';
  } catch(err) {
    document.getElementById("loading").innerHTML =
      '<p style="color:red;font-size:16px">加载失败</p>' +
      '<p style="font-size:14px">' + err + '</p>' +
      '<p style="font-size:14px">请使用 Chrome 浏览器并确保 WebGL 可用</p>';
  }
})();
</script>
<?php
include template('common/footer');
