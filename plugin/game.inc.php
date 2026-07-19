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
#ajaxwaitid { display:none !important; }
#loading { text-align:center; padding-top:40vh; }
.lds-dual-ring { display:inline-block; width:32px; height:32px; }
.lds-dual-ring:after { content:" "; display:block; width:32px; height:32px; border-radius:50%; border:4px solid #d33774; border-color:#d33774 transparent #d33774 transparent; animation:lds-dual-ring 1.2s linear infinite; }
@keyframes lds-dual-ring { 0%{transform:rotate(0deg)} 100%{transform:rotate(360deg)} }
</style>
<link rel="stylesheet" href="/source/plugin/pokemon/wasm/game.css">
<div id="main">
<div id="loading">
<p style="font-size:18px;color:#333;margin-bottom:16px">正在加载宠物中心</p>
<div class="lds-dual-ring"></div>
</div>
</div>
<script type="importmap">
{
  "imports": {
    "./snippets/": "/<?php echo $wasmPath; ?>/snippets/",
    "./game_bg.wasm": "/<?php echo $wasmPath; ?>/game_bg.wasm"
  }
}
</script>
<script>
window.__dioxus_hmr_disabled = true;
window.__dioxus_no_hot_reload = true;
(function() {
  const _fetch = window.fetch;
  window.fetch = function(url, opts) {
    if (typeof url === 'string' && url.includes('/_dioxus'))
      return Promise.resolve(new Response('{}', {status:200,headers:{'Content-Type':'application/json'}}));
    return _fetch.apply(this, arguments);
  };
})();
(function() {
  const _push = history.pushState, _replace = history.replaceState;
  history.pushState = function(){}; history.replaceState = function(){};
})();
</script>
<script type="module">
(async function() {
  const base = '/source/plugin/pokemon/wasm';
  try {
    const gameUrl = base + '/game.js';
    const wasmUrl = base + '/game_bg.wasm';
    const wasmModule = await import(gameUrl);
    await wasmModule.default(wasmUrl);
    const handle = new wasmModule.WebHandle();
    await handle.start();
    document.getElementById("loading").style.display = 'none';
  } catch(err) {
    const msg = String(err);
    document.getElementById("loading").innerHTML =
      '<p style="color:red;font-size:16px">加载失败</p>' +
      '<p style="font-size:13px;cursor:pointer;color:#666" onclick="navigator.clipboard.writeText(this.textContent).then(()=>{const s=this.nextElementSibling;s.style.display=\'block\';setTimeout(()=>s.style.display=\'none\',1500)})" title="点击复制错误信息">' + msg.replace(/</g,'&lt;') + '</p>' +
      '<p style="display:none;color:green;font-size:12px">已复制到剪贴板</p>' +
      '<p style="font-size:13px;color:#999;margin-top:8px">请使用 Chrome 浏览器并确保 WebGL 可用</p>';
  }
})();
</script>
<?php
include template('common/footer');
