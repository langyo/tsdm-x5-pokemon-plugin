<?php
/**
 * tairitsu + hikari 版游戏页（迁移目标实现）。
 *
 * 与 game.inc.php（Dioxus 版）并存：两套前端各自独立加载、互不影响，
 * 用于迁移期间的逐页行为对照。入口选择见 pokemon.inc.php：
 *   plugin.php?id=pokemon:game        → Dioxus 版（存量实现）
 *   plugin.php?id=pokemon:game&ui=v2  → 本文件（tairitsu 版）
 *
 * 构建产物见 scripts/publish/build_tairitsu_poc.py 与
 * docs/refactor-tairitsu-hikari-research.md。
 */

defined('IN_DISCUZ') || exit('Access Denied');

loadcache('plugin');
require_once __DIR__ . '/game_access.php';
$settings = $_G['cache']['plugin']['pokemon'] ?? [];
if (!pm_game_is_staff($settings) && pm_game_is_closed($settings)) {
    showmessage(lang('plugin/pokemon', 'system_closed'));
}

$wasm2_path = '/source/plugin/pokemon/wasm2';
$manifest_file = __DIR__ . '/wasm2/manifest.json';
$manifest = is_file($manifest_file)
    ? json_decode(file_get_contents($manifest_file), true)
    : null;
// 构建产物的 build_id 用于 cache-bust；产物缺失时退回当前时间戳，
// 让浏览器至少能重新取到后来补上的文件。
$build_id = is_array($manifest) && isset($manifest['build_id'])
    ? $manifest['build_id']
    : (string) time();
$boot_url = $wasm2_path . '/boot.js?v=' . $build_id;
$has_bundle = is_file(__DIR__ . '/wasm2/boot.js');

include template('common/header_common');
include template('common/header');
?>
<div id="pt" class="bm cl">
<div class="z">
<a href="./" class="nvhm" title="首页"><?php echo $_G['setting']['bbname']; ?></a><em>&raquo;</em>
<a href="plugin.php?id=pokemon:game&ui=v2">宠物中心</a>
</div>
</div>

<style>
#ajaxwaitid { display:none !important; }
#loading { text-align:center; padding-top:40vh; }
.lds-dual-ring { display:inline-block; width:32px; height:32px; }
.lds-dual-ring:after { content:" "; display:block; width:32px; height:32px; border-radius:50%; border:4px solid #5cb8e6; border-color:#5cb8e6 transparent #5cb8e6 transparent; animation:lds-dual-ring 1.2s linear infinite; }
@keyframes lds-dual-ring { 0%{transform:rotate(0deg)} 100%{transform:rotate(360deg)} }
/* 迁移期对照条：标明当前是 tairitsu 版，并提供切回 Dioxus 版的入口。 */
.pm-ui-switch { display:flex; gap:8px; align-items:center; justify-content:flex-end; margin:0 0 8px; font-size:12px; color:#666; }
.pm-ui-switch a { color:#5cb8e6; text-decoration:none; }
.pm-ui-switch a:hover { text-decoration:underline; }
.pm-ui-switch .current { font-weight:600; color:#333; }
</style>
<?php
// tairitsu 版直接复用 Dioxus 版的 game.css：页面组件与 Dioxus 版
// 同名同类（.page-shop / .shop-tile / .buy-btn …），一份样式两套前端共用，
// 迁移期的行为对照才是像素级的。
$game_css_ver = time();
?>
<link rel="stylesheet" href="/source/plugin/pokemon/wasm/game.css?v=<?php echo $game_css_ver; ?>">

<div class="pm-ui-switch">
  <span>界面版本：</span>
  <a href="plugin.php?id=pokemon:game">Dioxus（存量）</a>
  <span class="current">tairitsu（迁移中）</span>
</div>

<div id="main">
<?php if (!$has_bundle) { ?>
<div id="loading">
<p style="font-size:16px;color:#c00;margin-bottom:8px">tairitsu 构建产物缺失</p>
<p style="font-size:13px;color:#666">请先运行 <code>python scripts/publish/build_tairitsu_poc.py</code>，产物输出到 <code>plugin/wasm2/</code>。</p>
</div>
<?php } else { ?>
<div id="app" data-booting>
  <div id="loading">
    <p style="font-size:18px;color:#333;margin-bottom:16px">正在加载宠物中心</p>
    <div class="lds-dual-ring"></div>
  </div>
</div>
<?php } ?>
</div>

<script>
// 与 Dioxus 版一致的请求约定：同源请求带 formhash，服务端据此拒绝跨站伪造。
(function() {
  var formhash = '<?php echo FORMHASH; ?>';
  if (!formhash) return;
  var _fetch = window.fetch;
  window.fetch = function(url, opts) {
    var target = new URL(url instanceof Request ? url.url : String(url), location.href);
    if (target.origin !== location.origin) return _fetch.apply(this, arguments);
    if (url instanceof Request) {
      var headers = new Headers(url.headers);
      headers.set('X-Pm-Formhash', formhash);
      return _fetch.call(this, new Request(url, {headers: headers}), opts);
    }
    var init = Object.assign({}, opts);
    var h = new Headers(init.headers || {});
    h.set('X-Pm-Formhash', formhash);
    init.headers = h;
    return _fetch.call(this, url, init);
  };
})();

// 图片兜底（头像与大图回退），与 Dioxus 版保持同一策略。
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
<?php if ($has_bundle) { ?>
<script src="<?php echo $boot_url; ?>"></script>
<script type="module">
(function() {
  var app = document.getElementById('app');
  function fail(err) {
    var msg = String(err && err.stack ? err.stack : err);
    if (!app) return;
    app.innerHTML =
      '<p style="color:#c00;font-size:16px">加载失败</p>' +
      '<p style="font-size:13px;color:#666;white-space:pre-wrap">' + msg.replace(/</g, '&lt;') + '</p>';
    if (window.console) console.error('[tairitsu] boot failed', err);
  }
  if (typeof window.__tairitsuBoot !== 'function') {
    fail(new Error('boot.js 未执行（检查静态资源是否可访问：<?php echo $boot_url; ?>）'));
    return;
  }
  window.__tairitsuBoot().catch(fail);
})();
</script>
<?php } ?>
<?php
include template('common/footer');
