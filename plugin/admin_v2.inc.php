<?php
/**
 * tairitsu + hikari 版管理后台（默认实现）。
 *
 * 独立页面，不套论坛的页头页脚：后台是隔离的控制台界面，hikari 的
 * AdminShell 独占整个视口——顶部 AdminHeader、左侧 NavSidebar、底部状态栏，
 * 与 shittim-chest 的 /backend、ayane 的 ShellLayout 是同一种壳。返回论坛、
 * 回到宠物中心的入口在头部的头像菜单里。
 *
 * 本文件是后台的默认入口；Dioxus 旧版保留在 admincp.inc.php（套论坛版式），
 * 通过逃生参数 &ui=v1 访问：
 *   plugin.php?id=pokemon:game&index=admin          → 本文件（tairitsu 版，默认）
 *   plugin.php?id=pokemon:game&index=admin&ui=v1    → Dioxus 旧版（admincp.inc.php）
 *
 * 本文件只做三件事：权限门禁（与 admincp.inc.php 同源）、把页面数据挂到
 * #app 的 data-* 上、挂样式与启动脚本。样式归前端（rust/admin2/styles/，
 * 由 scripts/publish/build_tairitsu_admin.py 编译），这里不写一行 CSS。
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

// 页面数据：前端的外壳（rust/admin2）从 #app 的 data-pm-* 读取——站点名、
// 登录者、插件版本，以及头像菜单和状态栏要跳转的几个地址。
$plugin_meta_file = __DIR__ . '/discuz_plugin_pokemon.json';
$plugin_meta = is_file($plugin_meta_file)
    ? (json_decode(file_get_contents($plugin_meta_file), true) ?: [])
    : [];
$pm_page = [
    'site-name' => (string) $_G['setting']['bbname'],
    'user-id' => (string) (int) $_G['uid'],
    'user-name' => (string) $_G['username'],
    'user-group' => (string) ($_G['group']['grouptitle'] ?? ''),
    'avatar' => (string) avatar($_G['uid'], 'small', 1),
    'plugin-version' => (string) ($plugin_meta['Data']['plugin']['version'] ?? ''),
    'forum-url' => (string) $_G['siteurl'],
    'game-url' => 'plugin.php?id=pokemon:game',
    'legacy-url' => 'plugin.php?id=pokemon:game&index=admin&ui=v1',
    'logout-url' => 'member.php?action=logout&formhash=' . FORMHASH,
];
$pm_attrs = '';
foreach ($pm_page as $pm_key => $pm_value) {
    $pm_attrs .= ' data-pm-' . $pm_key . '="' . dhtmlspecialchars($pm_value) . '"';
}
$pm_title = '宠物插件后台 - ' . $_G['setting']['bbname'];
?>
<!DOCTYPE html>
<html lang="zh-CN" data-mode="light">
<head>
<meta charset="<?php echo CHARSET; ?>">
<meta name="viewport" content="width=device-width, initial-scale=1">
<base href="<?php echo $_G['siteurl']; ?>">
<title><?php echo dhtmlspecialchars($pm_title); ?></title>
<script>
// 明暗模式先于首帧落定，避免闪一下。默认浅色，与论坛一致；切换后的选择存本地。
(function () {
  try {
    var mode = localStorage.getItem('pm-admin-mode');
    document.documentElement.setAttribute('data-mode', mode === 'dark' ? 'dark' : 'light');
  } catch (e) {}
})();
</script>
<link rel="stylesheet" href="<?php echo $wasm2_path; ?>/admin-hikari.css?v=<?php echo $build_id; ?>">
<link rel="stylesheet" href="<?php echo $wasm2_path; ?>/admin.css?v=<?php echo $build_id; ?>">
</head>
<body>
<div id="app" data-booting<?php echo $pm_attrs; ?>>
  <?php if (!$has_bundle) { ?>
  <div id="loading" class="admin-boot-failed">
    <p class="admin-error">tairitsu 后台构建产物缺失</p>
    <p>请先运行 <code>python scripts/publish/build_tairitsu_admin.py</code>。</p>
    <p><a href="<?php echo dhtmlspecialchars($pm_page['forum-url']); ?>">返回论坛</a></p>
  </div>
  <?php } else { ?>
  <div id="loading">
    <div class="pm-boot-ring"></div>
    <p>正在加载管理后台</p>
  </div>
  <?php } ?>
</div>

<?php if ($has_bundle) { ?>
<script>
// 请求约定与 Dioxus 版一致：同源请求带 formhash，服务端据此拒绝跨站伪造。
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
    // 独立页面没有论坛导航可退，失败页自己给出回去的路。
    app.removeAttribute('data-booting');
    app.innerHTML =
      '<div class="admin-boot-failed">' +
        '<p class="admin-error">管理后台加载失败</p>' +
        '<pre>' + msg.replace(/&/g, '&amp;').replace(/</g, '&lt;') + '</pre>' +
        '<p><a href="<?php echo dhtmlspecialchars($pm_page['forum-url']); ?>">返回论坛</a>' +
        '　<a href="<?php echo dhtmlspecialchars($pm_page['legacy-url']); ?>">改用旧版后台</a></p>' +
      '</div>';
    if (window.console) console.error('[admin2] boot failed', err);
  }
  if (typeof window.__tairitsuBoot !== 'function') {
    fail(new Error('admin-boot.js 未执行（检查静态资源是否可访问）'));
    return;
  }
  window.__tairitsuBoot().catch(fail);
})();
</script>
<?php } ?>
</body>
</html>
<?php
// 收尾与论坛版式的页脚一致：更新会话，再由 output() 按站点的伪静态规则
// 改写链接（本页的相对地址因此也跟着站点走）并跑计划任务。
updatesession();
output();
