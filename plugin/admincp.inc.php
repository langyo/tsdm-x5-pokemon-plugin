<?php
defined('IN_DISCUZ') || exit('Access Denied');

$is_admin = ($_G['adminid'] == 1 || $_G['groupid'] == 1);
$is_moderator = false;
if (!$is_admin && $_G['uid']) {
    $modcheck = DB::result_first("SELECT COUNT(*) FROM " . DB::table('forum_moderator') . " WHERE uid=%d", [$_G['uid']]);
    $is_moderator = $modcheck > 0;
}
if (!$is_admin && !$is_moderator) {
    showmessage(lang('plugin/pokemon', 'admin_only'));
}

// Handle AJAX API calls from admin WASM
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    header('Content-Type: application/json; charset=utf-8');
    $action = isset($_POST['action']) ? $_POST['action'] : '';
    $op = strtok($action, ':');
    $target = strtok(':');
    $file = __DIR__ . '/admin/routes/' . $target . '.php';
    if ($target && file_exists($file)) {
        include $file;
    } else {
        echo json_encode(['success' => false, 'reason' => 'Unknown action: ' . $action]);
    }
    exit;
}

$wasmPath = 'source/plugin/pokemon/wasm';
$wasmVer = time();

include template('common/header_common');
include template('common/header');
?>
<div id="pt" class="bm cl">
<div class="z">
<a href="./" class="nvhm" title="首页"><?php echo $_G['setting']['bbname']; ?></a><em>&raquo;</em>
<a href="plugin.php?id=pokemon:game&index=admin">宠物管理</a>
</div>
</div>

<style>
#ajaxwaitid { display:none !important; }
#loading { text-align:center; padding-top:40vh; }
.lds-dual-ring { display:inline-block; width:32px; height:32px; }
.lds-dual-ring:after { content:" "; display:block; width:32px; height:32px; border-radius:50%; border:4px solid #d33774; border-color:#d33774 transparent #d33774 transparent; animation:lds-dual-ring 1.2s linear infinite; }
@keyframes lds-dual-ring { 0%{transform:rotate(0deg)} 100%{transform:rotate(360deg)} }
</style>
<link rel="stylesheet" href="/source/plugin/pokemon/wasm/admin.css">
<div id="main">
<div id="loading">
<p style="font-size:18px;color:#333;margin-bottom:16px">正在加载管理后台</p>
<div class="lds-dual-ring"></div>
</div>
</div>
<script type="importmap">
{
  "imports": {
    "./snippets/": "/<?php echo $wasmPath; ?>/snippets/",
    "./_admin_bg.wasm": "/<?php echo $wasmPath; ?>/_admin_bg.wasm"
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
</script>
<script type="module">
(async function() {
  const base = '/source/plugin/pokemon/wasm';
  try {
    const wasmModule = await import(base + '/_admin.js');
    await wasmModule.default(base + '/_admin_bg.wasm');
    const handle = new wasmModule.WebHandle();
    await handle.start();
    document.getElementById("loading").style.display = 'none';
  } catch(err) {
    var msg = String(err);
    var el = document.getElementById("loading");
    el.innerHTML = '<div style="max-width:600px;margin:0 auto;text-align:left">' +
      '<p style="color:red;font-size:16px;margin:0 0 8px">管理后台加载失败</p>' +
      '<div style="max-height:200px;overflow:auto;background:#f5f5f5;border:1px solid #ddd;border-radius:4px;padding:8px;font-family:monospace;font-size:11px;word-break:break-all;margin-bottom:8px">' + msg.replace(/</g,'&lt;').replace(/&/g,'&amp;').replace(/&lt;/g,'<') + '</div>' +
      '<button onclick="navigator.clipboard.writeText(this.previousElementSibling.textContent)" style="border:1px solid #ccc;background:#fff;padding:4px 12px;border-radius:4px;cursor:pointer;font-size:12px">复制错误信息</button>' +
      '</div>';
  }
})();
</script>
<?php
include template('common/footer');
