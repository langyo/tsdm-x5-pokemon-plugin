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
    "./admin_bg.wasm": "/<?php echo $wasmPath; ?>/admin_bg.wasm"
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
    const wasmModule = await import(base + '/admin.js');
    await wasmModule.default(base + '/admin_bg.wasm');
    const handle = new wasmModule.WebHandle();
    await handle.start();
    document.getElementById("loading").style.display = 'none';
  } catch(err) {
    const msg = String(err);
    document.getElementById("loading").innerHTML =
      '<p style="color:red;font-size:16px">加载失败</p>' +
      '<p style="font-size:13px;cursor:pointer;color:#666" onclick="navigator.clipboard.writeText(this.textContent).then(()=>{const s=this.nextElementSibling;s.style.display=\'block\';setTimeout(()=>s.style.display=\'none\',1500)})" title="点击复制错误信息">' + msg.replace(/</g,'&lt;') + '</p>' +
      '<p style="display:none;color:green;font-size:12px">已复制到剪贴板</p>';
  }
})();
</script>
<?php
include template('common/footer');
