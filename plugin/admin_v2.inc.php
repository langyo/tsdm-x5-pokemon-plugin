<?php
/**
 * tairitsu + hikari 版管理后台（迁移目标实现，切片 1：壳 + 全局配置）。
 *
 * 与 admincp.inc.php（Dioxus 版）并存：
 *   plugin.php?id=pokemon:game&index=admin        → Dioxus 版（存量实现）
 *   plugin.php?id=pokemon:game&index=admin&ui=v2  → 本文件（tairitsu 版）
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
<a href="plugin.php?id=pokemon:game&index=admin&ui=v2">宠物管理（tairitsu）</a>
</div>
</div>

<style>
#ajaxwaitid { display:none !important; }
#loading { text-align:center; padding-top:40vh; }
.lds-dual-ring { display:inline-block; width:32px; height:32px; }
.lds-dual-ring:after { content:" "; display:block; width:32px; height:32px; border-radius:50%; border:4px solid #d33774; border-color:#d33774 transparent #d33774 transparent; animation:lds-dual-ring 1.2s linear infinite; }
@keyframes lds-dual-ring { 0%{transform:rotate(0deg)} 100%{transform:rotate(360deg)} }
.pm-ui-switch { display:flex; gap:8px; align-items:center; justify-content:flex-end; margin:0 0 8px; font-size:12px; color:#666; }
.pm-ui-switch a { color:#d33774; text-decoration:none; }
.pm-ui-switch a:hover { text-decoration:underline; }
.pm-ui-switch .current { font-weight:600; color:#333; }

/* tairitsu 后台壳样式（Dioxus 版用 admin.css；这里自持一份最小布局）。 */
.admin-layout { display:flex; min-height:70vh; border:1px solid #e0e0e0; border-radius:8px; overflow:hidden; background:#fff; }
.admin-sidebar { width:200px; background:#2c3e50; color:#ecf0f1; display:flex; flex-direction:column; }
.admin-sidebar-header { padding:16px; border-bottom:1px solid rgba(255,255,255,.12); }
.admin-sidebar-title { display:block; font-weight:600; font-size:14px; }
.admin-sidebar-sub { display:block; font-size:11px; opacity:.65; margin-top:2px; }
.admin-nav { display:flex; flex-direction:column; padding:8px 0; }
.admin-nav-item { display:flex; align-items:center; gap:8px; padding:10px 16px; background:none; border:none; color:#ecf0f1; text-align:left; cursor:pointer; font-size:13px; }
.admin-nav-item:hover { background:rgba(255,255,255,.08); }
.admin-nav-item.active { background:rgba(255,255,255,.16); font-weight:600; }
.admin-nav-badge { font-size:10px; padding:1px 6px; border-radius:8px; background:#8e44ad; color:#fff; }
.admin-main { flex:1; display:flex; flex-direction:column; min-width:0; }
.admin-topbar { display:flex; align-items:center; gap:12px; padding:12px 20px; border-bottom:1px solid #eee; background:#fafafa; }
.admin-topbar-title { font-size:14px; font-weight:600; color:#333; }
.admin-busy { font-size:12px; color:#e67e22; }
.admin-dirty { font-size:12px; color:#c0392b; }
.admin-main > section { padding:20px; }
.admin-page-header h2 { margin:0 0 16px; font-size:18px; color:#333; }
.admin-card { background:#fff; border:1px solid #e8e8e8; border-radius:8px; padding:16px; }
.admin-placeholder p { color:#666; font-size:14px; }
.admin-placeholder-hint { font-size:12px; color:#999; margin-top:8px; }
.admin-loading { color:#666; padding:32px; text-align:center; }
.admin-error p { color:#c0392b; margin-bottom:12px; }
.admin-btn { border:1px solid #d0d0d0; background:#fff; color:#333; border-radius:6px; padding:6px 16px; cursor:pointer; font-size:13px; }
.admin-btn:hover { background:#f5f5f5; }
.admin-btn-primary { border-color:#d33774; color:#fff; background:#d33774; }
.admin-btn-primary:hover { background:#b52e63; }
.admin-btn.disabled, .admin-btn:disabled { opacity:.5; cursor:not-allowed; }
.admin-config-toolbar { display:flex; align-items:center; gap:12px; margin-bottom:12px; flex-wrap:wrap; }
.admin-config-hint { font-size:12px; color:#888; flex:1; min-width:160px; }
.admin-config-textarea { width:100%; min-height:320px; font-family:Consolas,Monaco,monospace; font-size:12px; line-height:1.5; border:1px solid #d8d8d8; border-radius:6px; padding:12px; box-sizing:border-box; white-space:pre; overflow:auto; }
.admin-config-textarea:focus { outline:none; border-color:#d33774; }

/* 宝可梦数据页（切片 2） */
.adm-count { font-size:12px; color:#888; font-weight:normal; margin-left:12px; }
.adm-filter-bar { display:flex; gap:8px; align-items:center; margin-bottom:16px; flex-wrap:wrap; }
.adm-filter-input { width:140px; border:1px solid #d8d8d8; border-radius:6px; padding:6px 10px; font-size:13px; }
.adm-filter-kind { width:110px; }
.adm-table { width:100%; border-collapse:collapse; font-size:13px; }
.adm-table th { text-align:left; padding:8px 10px; background:#fafafa; border-bottom:1px solid #eee; color:#555; font-weight:600; }
.adm-table td { padding:8px 10px; border-bottom:1px solid #f2f2f2; color:#333; }
.adm-table tbody tr:hover { background:#fdf7f9; }
.adm-pk-legendary { display:inline-block; margin-left:6px; font-size:10px; padding:1px 6px; border-radius:8px; background:#f39c12; color:#fff; }
.adm-pk-stats { font-size:12px; color:#777; }
.adm-load-more { text-align:center; padding:14px 0 4px; }
.admin-empty-hint { color:#999; padding:24px; text-align:center; }

/* 宝可梦编辑弹窗（切片 8） */
.modal-overlay { position:fixed; inset:0; background:rgba(0,0,0,.45); display:flex; align-items:flex-start; justify-content:center; padding:40px 16px; overflow:auto; z-index:900; }
.adm-editor-modal { background:#fff; border-radius:8px; width:720px; max-width:100%; box-shadow:0 8px 32px rgba(0,0,0,.25); }
.modal-header { display:flex; align-items:center; justify-content:space-between; padding:14px 18px; border-bottom:1px solid #eee; }
.modal-header h3 { margin:0; font-size:15px; color:#333; }
.modal-close { border:none; background:none; font-size:20px; line-height:1; color:#999; cursor:pointer; padding:0 4px; }
.modal-close:hover { color:#333; }
.adm-editor-body { padding:16px 18px; }
.adm-editor-grid { display:grid; grid-template-columns:repeat(3, minmax(0, 1fr)); gap:10px 12px; }
.adm-editor-field { display:flex; flex-direction:column; gap:4px; font-size:12px; color:#666; }
.adm-editor-field .adm-filter-input { width:100%; box-sizing:border-box; }
.adm-editor-wide { display:flex; flex-direction:column; gap:4px; margin-top:12px; }
.adm-editor-wide .adm-editor-textarea { width:100%; box-sizing:border-box; }
.adm-editor-section-title { margin:16px 0 8px; font-size:13px; font-weight:600; color:#333; }
.adm-editor-textarea { font-family:Consolas,Monaco,monospace; font-size:12px; line-height:1.5; border:1px solid #d8d8d8; border-radius:6px; padding:8px 10px; resize:vertical; }
.adm-editor-textarea:focus { outline:none; border-color:#d33774; }
.adm-editor-toggle { align-self:flex-end; }
.adm-editor-error { color:#c0392b; font-size:12px; margin:12px 0 0; }
.adm-editor-footer { display:flex; justify-content:flex-end; gap:10px; padding:12px 18px; border-top:1px solid #eee; }
.adm-row-edit { padding:3px 12px; font-size:12px; }
.adm-editor-hint { font-size:12px; color:#999; align-self:center; margin:0; }
.adm-editor-sprite { display:block; width:48px; height:48px; object-fit:contain; margin-top:12px; image-rendering:pixelated; }
.adm-row-actions { display:flex; gap:6px; }
.adm-grant-results { display:flex; flex-wrap:wrap; gap:6px; margin-top:10px; max-height:180px; overflow:auto; }
.adm-grant-result { padding:4px 10px; font-size:12px; }
.adm-grant-picked { display:flex; align-items:center; gap:10px; margin:4px 0 12px; }

/* 全局配置分节表单（切片 14） */
.adm-cfg-section { margin:0 0 18px; }
.adm-cfg-section:last-child { margin-bottom:4px; }
.adm-cfg-section-title { font-size:13px; font-weight:600; color:#333; margin:0 0 10px; padding-bottom:6px; border-bottom:1px solid #eee; }
.adm-cfg-grid { display:grid; grid-template-columns:repeat(2, minmax(0, 1fr)); gap:10px 18px; }
.adm-cfg-field { display:flex; flex-direction:column; gap:4px; font-size:12px; }
.adm-cfg-label { color:#555; }
.adm-cfg-hint { font-size:11px; color:#999; }
.adm-cfg-input { width:100%; box-sizing:border-box; }
.adm-cfg-toggle { align-self:flex-start; padding:3px 14px; font-size:12px; }
.adm-cfg-ann-list { display:flex; flex-direction:column; gap:6px; margin-bottom:10px; }
.adm-cfg-ann-row { display:flex; align-items:center; gap:10px; padding:6px 10px; background:#fafafa; border:1px solid #eee; border-radius:6px; }
.adm-cfg-ann-title { font-weight:600; color:#333; }
.adm-cfg-ann-url { flex:1; min-width:0; color:#888; font-size:11px; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }
.adm-cfg-ann-actions { display:flex; gap:6px; flex-shrink:0; }
.adm-cfg-add { margin-top:2px; }
.adm-editor-modal--narrow { width:520px; }
.adm-row-del { padding:3px 12px; font-size:12px; border-color:#e74c3c; color:#c0392b; }
.adm-row-del:hover { background:#fdf0ef; }
.admin-btn-danger { border-color:#e74c3c; background:#e74c3c; color:#fff; }
.admin-btn-danger:hover { background:#c0392b; }
.adm-tab-btn { padding:2px 10px; font-size:11px; margin-left:6px; }
.adm-editor-section-title { display:flex; align-items:center; }
</style>

<div class="pm-ui-switch">
  <span>界面版本：</span>
  <a href="plugin.php?id=pokemon:game&index=admin">Dioxus（存量）</a>
  <span class="current">tairitsu（迁移中）</span>
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
