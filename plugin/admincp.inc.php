<?php
defined('IN_DISCUZ') || exit('Access Denied');

// 管理后台暴露全站配置改写、SQL 控制台与文件读取能力，入口仅限：
// 1. 管理员（adminid=1 或管理用户组 groupid=1）；
// 2. 「宠物中心」板块的版主——按板块名查 fid 后核对 forum_moderator，
//    其它板块的版主与超级版主不放行，避免权限放大到论坛本体；
// 3. 插件配置 poke_smgly 中指名的宠物管理员（板块更名或未设版主时的后备）。
$is_admin = ($_G['adminid'] == 1 || $_G['groupid'] == 1);

$is_pokemon_staff = false;
if (!$is_admin && $_G['uid']) {
    $uid = intval($_G['uid']);

    $center_fids = array();
    foreach (DB::fetch_all("SELECT fid FROM " . DB::table('forum_forum') . " WHERE name='宠物中心'") as $row) {
        $center_fids[] = intval($row['fid']);
    }
    if ($center_fids) {
        $is_pokemon_staff = DB::result_first(
            "SELECT COUNT(*) FROM " . DB::table('forum_moderator') .
            " WHERE uid='" . $uid . "' AND fid IN (" . implode(',', $center_fids) . ")"
        ) > 0;
    }

    if (!$is_pokemon_staff) {
        $settings = isset($_G['cache']['plugin']['pokemon']) ? $_G['cache']['plugin']['pokemon'] : array();
        $staff = isset($settings['poke_smgly']) ? explode(',', $settings['poke_smgly']) : array();
        $is_pokemon_staff = in_array($_G['member']['username'], $staff);
    }
}

if (!$is_admin && !$is_pokemon_staff) {
    showmessage(lang('plugin/pokemon', 'admin_only'));
}

// Handle AJAX API calls from admin WASM
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    header('Content-Type: application/json; charset=utf-8');
    $action = isset($_POST['action']) ? $_POST['action'] : '';
    if (!$action) {
        $raw = file_get_contents('php://input');
        $json = json_decode($raw, true);
        if ($json && isset($json['action'])) {
            $action = $json['action'];
        }
    }
    $direct_actions = [
        'get_wild_pokemons_for_map',
        'add_pokemon_to_map',
        'remove_pokemon_from_map',
    ];
    $parts = explode('::', $action);
    $target = $parts[1] ?? '';
    if ($target || in_array($action, $direct_actions, true)) {
        include __DIR__ . '/admin/dispatch.php';
    } else {
        echo json_encode(['success' => false, 'reason' => 'Invalid action']);
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
<script src="/source/plugin/pokemon/wasm/lucide.min.js"></script>
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
      '<div id="err-msg" style="max-height:200px;overflow:auto;background:#f5f5f5;border:1px solid #ddd;border-radius:4px;padding:8px;font-family:monospace;font-size:11px;word-break:break-all;margin-bottom:8px">' + msg.replace(/</g,'&lt;') + '</div>' +
      '<button onclick="var t=document.getElementById(\'err-msg\').textContent;navigator.clipboard.writeText(t).then(function(){var s=document.getElementById(\'copy-ok\');s.style.display=\'block\';setTimeout(function(){s.style.display=\'none\'},2000)})" style="border:1px solid #d33774;color:#d33774;background:#fff;padding:4px 16px;border-radius:4px;cursor:pointer;font-size:12px">复制错误信息</button>' +
      '<span id="copy-ok" style="display:none;color:green;font-size:12px;margin-left:8px">已复制到剪贴板</span>' +
      '</div>';
  }
})();
</script>
<?php
include template('common/footer');
