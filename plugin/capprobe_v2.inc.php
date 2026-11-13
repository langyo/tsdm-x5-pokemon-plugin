<?php
/**
 * tairitsu wasm component 能力探针（capability probe，管理员工具）。
 *
 * 入口：plugin.php?id=pokemon:game&index=capprobe
 * 构建产物见 scripts/publish/build_tairitsu_cap.py。
 *
 * 探针在真实浏览器里逐项执行 tairitsu 平台 API（DOM/定时器/事件/
 * observer/剪贴板/几何/媒体查询/滚动/导航/canvas/IDB/文件/音频/视频/
 * 地理/全屏/batch/ws），输出 Pass/Fail/Skip 三态结果。
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
$manifest_file = __DIR__ . '/wasm2/cap-manifest.json';
$manifest = is_file($manifest_file)
    ? json_decode(file_get_contents($manifest_file), true)
    : null;
$build_id = is_array($manifest) && isset($manifest['build_id'])
    ? $manifest['build_id']
    : (string) time();
$boot_url = $wasm2_path . '/cap-boot.js?v=' . $build_id;
$has_bundle = is_file(__DIR__ . '/wasm2/cap-boot.js');

include template('common/header_common');
include template('common/header');
?>
<div id="pt" class="bm cl">
<div class="z">
<a href="./" class="nvhm" title="首页"><?php echo $_G['setting']['bbname']; ?></a><em>&raquo;</em>
<a href="plugin.php?id=pokemon:game&index=capprobe">tairitsu 能力探针</a>
</div>
</div>

<style>
#ajaxwaitid { display:none !important; }
#loading { text-align:center; padding-top:40vh; }
.cap-layout { font-size:13px; color:#333; background:#fff; border:1px solid #e3e8ee; border-radius:8px; padding:16px; margin-bottom:24px; }
.cap-layout h2 { margin:0 0 8px; font-size:16px; }
.cap-summary { color:#7f8c8d; margin:0 0 12px; }
.cap-anchor { display:inline-block; padding:8px 16px; background:#eef2f7; border:1px solid #cfd8e3; border-radius:4px; }
.cap-tall { height:1px; }
.cap-triggers { display:flex; flex-wrap:wrap; gap:10px; align-items:center; margin:12px 0; padding:10px; background:#fafbfc; border:1px dashed #d8dee6; border-radius:6px; }
.cap-triggers input { padding:4px 8px; border:1px solid #cfd8e3; border-radius:4px; min-width:160px; }
.cap-triggers button { padding:4px 12px; }
.cap-table { width:100%; border-collapse:collapse; }
.cap-table th, .cap-table td { border:1px solid #e3e8ee; padding:4px 8px; text-align:left; font-size:12px; vertical-align:top; }
.cap-table th { background:#f4f6f9; }
.cap-pass { color:#1e8e3e; font-weight:600; }
.cap-fail { color:#c0392b; font-weight:600; }
.cap-skip { color:#b8860b; font-weight:600; }
.cap-pending { color:#7f8c8d; }
</style>

<div id="app" data-booting>
  <div id="loading">
    <p style="font-size:18px;color:#333;margin-bottom:16px">正在加载能力探针</p>
    <div class="lds-dual-ring"></div>
  </div>
</div>

<?php if ($has_bundle) { ?>
<script src="<?php echo $boot_url; ?>"></script>
<script type="module">
(function() {
  var app = document.getElementById('app');
  function fail(err) {
    var msg = String(err && err.stack ? err.stack : err);
    if (!app) return;
    app.innerHTML =
      '<p style="color:#c00;font-size:16px;margin:0 0 8px">能力探针加载失败</p>' +
      '<div style="max-height:240px;overflow:auto;background:#f5f5f5;border:1px solid #ddd;border-radius:4px;padding:8px;font-family:monospace;font-size:11px;word-break:break-all;white-space:pre-wrap">' + msg.replace(/</g, '&lt;') + '</div>';
    if (window.console) console.error('[capprobe] boot failed', err);
  }
  if (typeof window.__tairitsuBoot !== 'function') {
    fail(new Error('cap-boot.js 未执行（检查静态资源是否可访问）'));
    return;
  }
  window.__tairitsuBoot().catch(fail);
})();
</script>
<?php } else { ?>
<script>
document.addEventListener('DOMContentLoaded', function () {
  var el = document.getElementById('loading');
  if (el) { el.innerHTML = '<p style="color:#c0392b">探针产物缺失：请先运行 scripts/publish/build_tairitsu_cap.py</p>'; }
});
</script>
<?php } ?>

<?php include template('common/footer'); ?>
