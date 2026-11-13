<?php
defined('IN_DISCUZ') || exit('Access Denied');

if (isset($_GET['index']) && $_GET['index'] === 'capprobe') {
    // tairitsu 能力探针（管理员工具，仅管理员）。
    include_once __DIR__ . '/capprobe_v2.inc.php';
    return;
}

if (isset($_GET['index']) && $_GET['index'] === 'admin') {
    // 迁移期双后台：&ui=v2 走 tairitsu 版（admin_v2.inc.php），默认仍是
    // Dioxus 版（admincp.inc.php）。
    if (isset($_GET['ui']) && $_GET['ui'] === 'v2') {
        include_once __DIR__ . '/admin_v2.inc.php';
        return;
    }
    include_once __DIR__ . '/admincp.inc.php';
    return;
}

// 迁移期双前端：Discuz 的插件路由直接把 index=game 映射到本文件，
// 不经过 pokemon.inc.php，所以入口判断必须放在这里。
//   plugin.php?id=pokemon:game        → 本文件（Dioxus 版，存量）
//   plugin.php?id=pokemon:game&ui=v2  → game_v2.inc.php（tairitsu 版）
if (isset($_GET['ui']) && $_GET['ui'] === 'v2') {
    include_once __DIR__ . '/game_v2.inc.php';
    return;
}

loadcache('plugin');
require_once __DIR__ . '/game_access.php';
$settings = $_G['cache']['plugin']['pokemon'] ?? [];
if (!pm_game_is_staff($settings) && pm_game_is_closed($settings)) {
    showmessage(lang('plugin/pokemon', 'system_closed'));
}

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
/* 迁移期对照条：标明当前是 Dioxus 版，并提供切到 tairitsu 版的入口。 */
.pm-ui-switch { display:flex; gap:8px; align-items:center; justify-content:flex-end; margin:0 0 8px; font-size:12px; color:#666; }
.pm-ui-switch a { color:#5cb8e6; text-decoration:none; }
.pm-ui-switch a:hover { text-decoration:underline; }
.pm-ui-switch .current { font-weight:600; color:#333; }
</style>
<link rel="stylesheet" href="/source/plugin/pokemon/wasm/game.css?v=<?php echo $wasmVer; ?>">
<div class="pm-ui-switch">
  <span>界面版本：</span>
  <span class="current">Dioxus（存量）</span>
  <a href="plugin.php?id=pokemon:game&ui=v2">tairitsu（迁移中）</a>
</div>
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
    "./_game_bg.wasm": "/<?php echo $wasmPath; ?>/_game_bg.wasm?v=<?php echo $wasmVer; ?>"
  }
}
</script>
<script>
window.__dioxus_hmr_disabled = true;
window.__dioxus_no_hot_reload = true;
(function() {
  const _fetch = window.fetch;
  const formhash = '<?php echo FORMHASH; ?>';
  window.fetch = function(url, opts) {
    if (typeof url === 'string' && url.includes('/_dioxus'))
      return Promise.resolve(new Response('{}', {status:200,headers:{'Content-Type':'application/json'}}));
    // 同源请求带上 formhash，服务端据此拒绝跨站伪造的请求
    const target = new URL(url instanceof Request ? url.url : String(url), location.href);
    if (target.origin === location.origin) {
      if (url instanceof Request) {
        const headers = new Headers(url.headers);
        headers.set('X-Pm-Formhash', formhash);
        return _fetch.call(this, new Request(url, {headers: headers}), opts);
      }
      const init = Object.assign({}, opts);
      const headers = new Headers(init.headers || {});
      headers.set('X-Pm-Formhash', formhash);
      init.headers = headers;
      return _fetch.call(this, url, init);
    }
    return _fetch.apply(this, arguments);
  };
})();
(function() {
  const _push = history.pushState, _replace = history.replaceState;
  history.pushState = function(){}; history.replaceState = function(){};
})();
// 图片兜底：
// 1) 头像 — avatar.php 不可用（未打补丁/占位图缺失）时换用插件自带的默认头像。
// 2) 大图 — CDN 动画 GIF (img.tsdm39.com/Pokemon) 缺图或不可达时回退到本地静态 PNG。
(function() {
  document.addEventListener('error', function(e) {
    var img = e.target;
    if (!img || img.tagName !== 'IMG') return;
    var src = img.getAttribute('src') || '';
    if (src.indexOf('endpoint=avatar') !== -1 && !img.dataset.avatarFallback) {
      img.dataset.avatarFallback = '1';
      img.src = '/source/plugin/pokemon/images/site/noavatar.svg';
      return;
    }
    var cdnAt = src.indexOf('img.tsdm39.com/Pokemon/');
    if (cdnAt !== -1 && /\.gif(\?|$)/i.test(src) && !img.dataset.pmFallback) {
      img.dataset.pmFallback = '1';
      img.src = '/source/plugin/pokemon/images/' +
        src.slice(cdnAt + 'img.tsdm39.com/Pokemon/'.length).replace(/\.gif(\?|$)/i, '.png$1');
    }
  }, true);
})();
</script>
<script type="module">
(async function() {
  const base = '/source/plugin/pokemon/wasm';
  try {
    const wasmUrl = base + '/_game_bg.wasm?v=<?php echo $wasmVer; ?>';
    const wasmModule = await import(base + '/_game.js?v=<?php echo $wasmVer; ?>');
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
