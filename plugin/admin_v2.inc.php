<?php
/**
 * tairitsu + hikari 版管理后台（默认实现，全量移植完成）。
 *
 * 本文件是后台的默认入口；Dioxus 旧版保留在 admincp.inc.php，
 * 通过逃生参数 &ui=v1 访问：
 *   plugin.php?id=pokemon:game&index=admin          → 本文件（tairitsu 版，默认）
 *   plugin.php?id=pokemon:game&index=admin&ui=v1    → Dioxus 旧版（admincp.inc.php）
 *
 * 权限门禁与 admincp.inc.php 同源（管理员/宠物管理员）。
 * 构建产物见 scripts/publish/build_tairitsu_admin.py。
 */

defined('IN_DISCUZ') || exit('Access Denied');

loadcache('plugin');
require_once __DIR__ . '/game_access.php';
$settings = $_G['cache']['plugin']['pokemon'] ?? [];
$is_admin = $_G['adminid'] == 1;
$is_pokemon_staff = pm_game_is_staff($settings);
if (!$is_admin && !$is_pokemon_staff) {
    showmessage(lang('plugin/pokemon', 'admin_only'));
}

$wasm2_path = '/source/plugin/pokemon/wasm2';
$manifest_file = __DIR__ . '/wasm2/admin-manifest.json';
$manifest = is_file($manifest_file)
    ? json_decode(file_get_contents($manifest_file), true)
    : null;
$build_id = is_array($manifest) && isset($manifest['build_id'])
    ? $manifest['build_id']
    : (string) time();
$boot_url = $wasm2_path . '/admin-boot.js?v=' . $build_id;
$has_bundle = is_file(__DIR__ . '/wasm2/admin-boot.js');

include template('common/header_common');
include template('common/header');
?>
<div id="pt" class="bm cl">
<div class="z">
<a href="./" class="nvhm" title="首页"><?php echo $_G['setting']['bbname']; ?></a><em>&raquo;</em>
<a href="plugin.php?id=pokemon:game&index=admin">宠物管理</a>
</div>
</div>

<?php // 样式归前端：由 rust/admin2/styles/main.scss 编译成 wasm2/admin.css。 ?>
<link rel="stylesheet" href="/source/plugin/pokemon/wasm2/admin.css?v=<?php echo $build_id; ?>">

<div class="pm-ui-switch">
  <span>界面版本：</span>
  <span class="current">tairitsu（默认）</span>
  <a href="plugin.php?id=pokemon:game&index=admin&ui=v1">Dioxus（旧版）</a>
</div>

<div id="main">
<?php if (!$has_bundle) { ?>
<div id="loading">
<p style="font-size:16px;color:#c00;margin-bottom:8px">tairitsu 后台构建产物缺失</p>
<p style="font-size:13px;color:#666">请先运行 <code>python scripts/publish/build_tairitsu_admin.py</code>。</p>
</div>
<?php } else { ?>
<div id="app" data-booting>
  <div id="loading">
    <p style="font-size:18px;color:#333;margin-bottom:16px">正在加载管理后台</p>
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
</script>

<script src="<?php echo $boot_url; ?>"></script>
<script type="module">
(function() {
  var app = document.getElementById('app');
  function fail(err) {
    var msg = String(err && err.stack ? err.stack : err);
    if (!app) return;
    app.innerHTML =
      '<p style="color:#c00;font-size:16px;margin:0 0 8px">管理后台加载失败</p>' +
      '<div style="max-height:240px;overflow:auto;background:#f5f5f5;border:1px solid #ddd;border-radius:4px;padding:8px;font-family:monospace;font-size:11px;word-break:break-all;white-space:pre-wrap">' + msg.replace(/</g, '&lt;') + '</div>';
    if (window.console) console.error('[admin2] boot failed', err);
  }
  if (typeof window.__tairitsuBoot !== 'function') {
    fail(new Error('admin-boot.js 未执行（检查静态资源是否可访问）'));
    return;
  }
  window.__tairitsuBoot().catch(fail);
})();
</script>
<?php
include template('common/footer');
