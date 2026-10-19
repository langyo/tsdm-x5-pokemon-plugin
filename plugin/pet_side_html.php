<?php
defined('IN_DISCUZ') || exit('Access Denied');

/**
 * 帖子侧边栏宠物徽章共享渲染库。
 *
 * 使用方：
 *  - postspm.class.php  viewthread 钩子服务端注入（主路径）
 *  - api/badges.php      badge_js.php 前端兜底注入（备用路径）
 *
 * 图片走"论坛 CDN 优先、本地素材兜底"的回退链（img 标签带 data-pm-chain，
 * 由 pm_side_assets() 的全局 error 监听逐级切换）：
 *  - 大图  CDN {pm|pmsg}/id.gif → 本地 images/{pm|pmsg}/id.png（静态）
 *  - 小图  本地 images/spm/id.gif → CDN spm/id.gif
 * X3 时代侧栏小图走独立图床（img.mikudm.net），该图床已下线；本地 spm 素材
 * 即当年从它同步所得，官方 CDN 的 spm 目录同样覆盖自定宠，作为缺失兜底。
 */

function pm_side_cdn()
{
    return 'https://img.tsdm39.com/Pokemon';
}

function pm_side_big_chain($species, $is_shiny = 0)
{
    $dir = $is_shiny ? 'pmsg' : 'pm';
    return [
        pm_side_cdn() . "/{$dir}/{$species}.gif",
        "source/plugin/pokemon/images/{$dir}/{$species}.png",
    ];
}

function pm_side_small_chain($species)
{
    return [
        "source/plugin/pokemon/images/spm/{$species}.gif",
        pm_side_cdn() . "/spm/{$species}.gif",
    ];
}

function pm_side_chain_attr(array $urls)
{
    return htmlspecialchars(json_encode(array_values($urls), JSON_UNESCAPED_SLASHES), ENT_QUOTES);
}

// 兼容两种数据形态：X5 刷新徽章写入的 species_id/nickname，
// 以及 X3 历史数据遗留的 pmno/nowname。
function pm_side_species($pet)
{
    return intval($pet['species_id'] ?? $pet['pmno'] ?? 0);
}

function pm_side_name($pet)
{
    return (string)($pet['nickname'] ?? $pet['nowname'] ?? '');
}

function pm_side_level($pet)
{
    return intval($pet['level'] ?? 0);
}

function pm_side_shiny($pet)
{
    return intval($pet['is_shiny'] ?? 0) === 1;
}

function pm_side_pet_title($pet)
{
    return htmlspecialchars(
        (pm_side_shiny($pet) ? '✨' : '') . pm_side_name($pet) . ' Lv:' . pm_side_level($pet),
        ENT_QUOTES
    );
}

/** 首发宠物：CDN 大图（闪光走 pmsg），名字行同样点击出数据卡。 */
function pm_side_first_html($pet)
{
    $id = intval($pet['id'] ?? 0);
    $title = pm_side_pet_title($pet);
    $chain = pm_side_big_chain(pm_side_species($pet), pm_side_shiny($pet) ? 1 : 0);
    $img = '<img src="' . htmlspecialchars($chain[0], ENT_QUOTES) . '"'
        . ' data-pm-chain="' . pm_side_chain_attr($chain) . '"'
        . ' alt="' . $title . '">';
    $name = htmlspecialchars(pm_side_name($pet), ENT_QUOTES);
    $shiny = pm_side_shiny($pet) ? '<span class="pm-shiny">✨</span>' : '';
    $html = '<div class="pm-first">'
        . '<a class="pm-pet" href="javascript:;" data-petid="' . $id . '" title="' . $title . '">' . $img . '</a>'
        . '<div class="pm-first-name">' . $shiny
        . '<a class="pm-pet" href="javascript:;" data-petid="' . $id . '">' . $name . '</a>'
        . ' Lv.' . pm_side_level($pet) . '</div>'
        . '</div>';
    return $html;
}

/** 跟随宠物图标：默认小图（spm），大图模式下 JS 换用 data-pm-chain-lg 大图链。 */
function pm_side_creep_html($pet)
{
    $id = intval($pet['id'] ?? 0);
    $title = pm_side_pet_title($pet);
    $species = pm_side_species($pet);
    $sm = pm_side_small_chain($species);
    $lg = pm_side_big_chain($species, pm_side_shiny($pet) ? 1 : 0);
    return '<a class="pm-pet" href="javascript:;" data-petid="' . $id . '" title="' . $title . '">'
        . '<img src="' . htmlspecialchars($sm[0], ENT_QUOTES) . '"'
        . ' data-pm-chain="' . pm_side_chain_attr($sm) . '"'
        . ' data-pm-chain-lg="' . pm_side_chain_attr($lg) . '"'
        . ' alt="' . $title . '"></a>';
}

/** 尺寸切换按钮组：两个尺寸图标按钮 + 滑块。 */
function pm_side_toggle_html()
{
    return '<div class="pm-creeps-bar">'
        . '<div class="pm-size-toggle" data-on="sm" role="group" aria-label="切换宠物图标尺寸">'
        . '<i class="pm-size-thumb"></i>'
        . '<button type="button" class="pm-size-btn is-on" data-pm-size="sm" title="小图">'
        . '<svg viewBox="0 0 10 10" width="10" height="10"><rect width="10" height="10" rx="2"/></svg>'
        . '</button>'
        . '<button type="button" class="pm-size-btn" data-pm-size="lg" title="大图">'
        . '<svg viewBox="0 0 14 14" width="14" height="14"><rect width="14" height="14" rx="3"/></svg>'
        . '</button>'
        . '</div></div>';
}

/**
 * 由 common_member_field_forum.pokemon 反序列化数据构建徽章 HTML。
 * 返回空串表示该用户无可展示宠物。
 */
function pm_side_badge_html($data)
{
    $html = '';
    $first = $data['first'] ?? null;
    if (is_array($first) && pm_side_species($first)) {
        $html .= pm_side_first_html($first);
    }
    $creeps = $data['creeps'] ?? null;
    if (is_array($creeps)) {
        $creeps_html = '';
        foreach ($creeps as $pet) {
            if (!is_array($pet) || !pm_side_species($pet)) {
                continue;
            }
            $creeps_html .= pm_side_creep_html($pet);
        }
        if ($creeps_html !== '') {
            $html .= pm_side_toggle_html()
                . '<div class="pm-creeps">' . $creeps_html . '</div>';
        }
    }
    if ($html === '') {
        return '';
    }
    return '<div class="pm-side-badge">' . $html . '</div>';
}

/**
 * 徽章所需的 <style> + <script>，每页只输出一次（调用方控制；脚本内亦有
 * __pmSideInit 防重入）。职责：
 *  - 图片回退链：img 加载失败时按 data-pm-chain 逐级切换图床；
 *  - 大小图切换：按钮组滑块，跟随宠物在小图/大图间切换，localStorage 记忆；
 *  - 宠物数据卡弹窗：点击任意宠物图标拉取 card 接口展示数据。
 */
function pm_side_assets()
{
    $css = <<<'CSS'
.pm-side-badge{text-align:center;font-size:12px;line-height:1.6}
.pm-side-badge .pm-first img{height:80px;width:auto;padding:0 24px;border:0}
.pm-side-badge .pm-first-name{margin-top:2px}
.pm-side-badge .pm-first-name a{color:inherit}
.pm-side-badge .pm-shiny{margin-right:2px}
.pm-creeps-bar{display:flex;justify-content:center;margin:6px 0 2px}
.pm-size-toggle{position:relative;display:inline-flex;border:1px solid #cfd6e0;border-radius:10px;background:#f5f7fa;padding:1px;user-select:none;overflow:hidden}
.pm-size-thumb{position:absolute;top:1px;left:1px;width:calc(50% - 1px);height:calc(100% - 2px);border-radius:8px;background:#fff;box-shadow:0 1px 2px rgba(0,0,0,.18);transition:transform .18s ease}
.pm-size-toggle[data-on="lg"] .pm-size-thumb{transform:translateX(100%)}
.pm-size-btn{position:relative;z-index:1;box-sizing:content-box;width:24px;height:20px;padding:0;border:0;background:transparent;cursor:pointer;display:flex;align-items:center;justify-content:center;opacity:.4;transition:opacity .18s}
.pm-size-btn.is-on{opacity:1}
.pm-size-btn svg{display:block;fill:#4a5a6a}
.pm-creeps{display:flex;flex-wrap:wrap;justify-content:center;gap:2px}
.pm-creeps img{width:32px;height:32px;border:0;transition:width .18s ease,height .18s ease}
.pm-side-badge.pm-size-lg .pm-creeps img{width:56px;height:56px}
.pm-modal{position:fixed;left:0;top:0;right:0;bottom:0;display:none;z-index:99999;align-items:center;justify-content:center}
.pm-modal.pm-open{display:flex}
.pm-modal-mask{position:absolute;left:0;top:0;right:0;bottom:0;background:rgba(0,0,0,.45)}
.pm-modal-box{position:relative;background:#fff;border-radius:10px;box-shadow:0 8px 30px rgba(0,0,0,.25);max-width:320px;width:calc(100vw - 48px);max-height:calc(100vh - 60px);overflow:auto;padding:18px 16px 14px;box-sizing:border-box}
.pm-modal-close{position:absolute;top:4px;right:8px;border:0;background:transparent;font-size:22px;line-height:1;cursor:pointer;color:#999;padding:4px}
.pm-modal-close:hover{color:#333}
.pm-loading{padding:24px 0;text-align:center;color:#888}
.pm-card-img{display:block;height:96px;width:auto;margin:0 auto}
.pm-card-head{text-align:center;margin:6px 0 2px;font-size:15px;font-weight:700}
.pm-card-sub{text-align:center;color:#999;font-size:12px;margin-bottom:6px}
.pm-card-tag{display:inline-block;margin:0 2px 2px;padding:1px 6px;border-radius:8px;background:#eef2f7;color:#4a5a6a;font-size:11px}
.pm-card-tag.pm-tag-fire{background:#fdeae0;color:#c0491b}
.pm-gender-m{color:#2d7ff9}
.pm-gender-f{color:#f0548a}
.pm-card-hp{margin:8px 0 4px}
.pm-card-hp-bar{position:relative;height:8px;border-radius:4px;background:#e8ecf1;overflow:hidden}
.pm-card-hp-fill{position:absolute;left:0;top:0;bottom:0;border-radius:4px;background:#52c41a;transition:width .25s ease}
.pm-card-hp-fill.pm-hp-mid{background:#faad14}
.pm-card-hp-fill.pm-hp-low{background:#f5222d}
.pm-card-hp-num{text-align:right;font-size:11px;color:#777;margin-top:2px}
.pm-card-exp{margin:4px 0 0}
.pm-card-exp-fill{position:absolute;left:0;top:0;bottom:0;border-radius:4px;background:#6f9ee8;transition:width .25s ease}
.pm-card-stats{display:grid;grid-template-columns:repeat(3,1fr);gap:4px;margin:8px 0 4px}
.pm-card-stat{background:#f7f9fc;border-radius:6px;padding:4px 2px;text-align:center}
.pm-card-stat b{display:block;font-size:14px}
.pm-card-stat span{font-size:11px;color:#888}
.pm-card-skills{margin-top:8px}
.pm-card-skills h4{margin:0 0 4px;font-size:12px;color:#666}
.pm-card-skill{display:flex;justify-content:space-between;gap:6px;padding:3px 6px;border-radius:6px;background:#f7f9fc;font-size:12px;margin-bottom:2px}
.pm-card-skill .pm-skill-name{font-weight:600;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.pm-card-skill .pm-skill-meta{color:#888;white-space:nowrap}
CSS;

    $js = <<<'JS'
(function(){
if (window.__pmSideInit) return;
window.__pmSideInit = true;

var LS_KEY = 'pm_side_size';

function esc(s){
  return String(s == null ? '' : s).replace(/[&<>"']/g, function(c){
    return {'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c];
  });
}

// 图片回退链：加载失败按 data-pm-chain 依次尝试下一个地址
document.addEventListener('error', function(e){
  var img = e.target;
  if (!img || img.tagName !== 'IMG') return;
  var raw = img.getAttribute('data-pm-chain');
  if (!raw) return;
  var chain;
  try { chain = JSON.parse(raw); } catch (ex) { return; }
  var used = parseInt(img.getAttribute('data-pm-used') || '0', 10);
  if (used + 1 < chain.length) {
    img.setAttribute('data-pm-used', String(used + 1));
    img.src = chain[used + 1];
  }
}, true);

// 大小图切换：全页统一，localStorage 记忆
function applySize(size){
  var lg = size === 'lg';
  var badges = document.querySelectorAll('.pm-side-badge');
  for (var i = 0; i < badges.length; i++) {
    if (lg) { badges[i].classList.add('pm-size-lg'); }
    else { badges[i].classList.remove('pm-size-lg'); }
  }
  var toggles = document.querySelectorAll('.pm-size-toggle');
  for (var j = 0; j < toggles.length; j++) {
    toggles[j].setAttribute('data-on', lg ? 'lg' : 'sm');
    var btns = toggles[j].querySelectorAll('.pm-size-btn');
    for (var k = 0; k < btns.length; k++) {
      var on = btns[k].getAttribute('data-pm-size') === size;
      btns[k].className = 'pm-size-btn' + (on ? ' is-on' : '');
    }
  }
  var imgs = document.querySelectorAll('.pm-creeps img');
  for (var m = 0; m < imgs.length; m++) {
    var raw = imgs[m].getAttribute(lg ? 'data-pm-chain-lg' : 'data-pm-chain') || imgs[m].getAttribute('data-pm-chain');
    var first = '';
    try { first = JSON.parse(raw)[0]; } catch (ex) {}
    if (first) {
      imgs[m].removeAttribute('data-pm-used');
      imgs[m].src = first;
    }
  }
  try { localStorage.setItem(LS_KEY, size); } catch (ex) {}
}

// 宠物数据卡弹窗
var modal, modalBody;

function ensureModal(){
  if (modal) return;
  modal = document.createElement('div');
  modal.className = 'pm-modal';
  modal.innerHTML = '<div class="pm-modal-mask"></div>'
    + '<div class="pm-modal-box"><button type="button" class="pm-modal-close" title="关闭">×</button>'
    + '<div class="pm-modal-body"></div></div>';
  modalBody = modal.querySelector('.pm-modal-body');
  modal.querySelector('.pm-modal-mask').onclick = hideModal;
  modal.querySelector('.pm-modal-close').onclick = hideModal;
  document.body.appendChild(modal);
}

function hideModal(){
  if (modal) modal.classList.remove('pm-open');
}

var GENDER = {1: ['♂', 'pm-gender-m'], 2: ['♀', 'pm-gender-f']};
var STAT_NAMES = [['hp', 'HP'], ['attack', '攻击'], ['defense', '防御'], ['sp_attack', '特攻'], ['sp_defense', '特防'], ['speed', '速度']];

function renderCard(p){
  var h = '';
  if (p.image_urls && p.image_urls.length) {
    h += '<img class="pm-card-img" src="' + esc(p.image_urls[0]) + '"'
      + ' data-pm-chain="' + esc(JSON.stringify(p.image_urls)) + '" alt="' + esc(p.name) + '">';
  }
  var g = GENDER[p.gender] || ['', ''];
  h += '<div class="pm-card-head">' + (p.is_shiny ? '<span class="pm-shiny">✨</span>' : '')
    + esc(p.name) + (p.nickname ? '（' + esc(p.nickname) + '）' : '')
    + ' <span class="' + g[1] + '">' + g[0] + '</span></div>';
  h += '<div class="pm-card-sub">Lv.' + esc(p.level) + (p.owner ? ' · 主人：' + esc(p.owner) : '') + '</div>';
  h += '<div>';
  if (p.type_1) h += '<span class="pm-card-tag">' + esc(p.type_1) + '</span>';
  if (p.type_2) h += '<span class="pm-card-tag">' + esc(p.type_2) + '</span>';
  if (p.state_text) h += '<span class="pm-card-tag">' + esc(p.state_text) + '</span>';
  h += '<span class="pm-card-tag">好感 ' + esc(p.affection) + '</span></div>';
  var pct = p.max_hp > 0 ? Math.round(p.hp / p.max_hp * 100) : 0;
  var hpCls = pct > 50 ? '' : (pct > 20 ? ' pm-hp-mid' : ' pm-hp-low');
  h += '<div class="pm-card-hp"><div class="pm-card-hp-bar"><i class="pm-card-hp-fill' + hpCls + '" style="width:' + pct + '%"></i></div>'
    + '<div class="pm-card-hp-num">HP ' + esc(p.hp) + ' / ' + esc(p.max_hp) + '</div></div>';
  // 经验进度：本级已获得 / 升级所需，与个人中心属性栏的基础数据对齐；
  // 旧端点未返回经验字段时显示 0/0，不影响其余卡片内容
  var expBase = +p.exp_for_current_level || 0, expNext = +p.exp_for_next_level || 0;
  var expSpan = Math.max(0, expNext - expBase);
  var expCur = Math.max(0, Math.min((+p.exp || 0) - expBase, expSpan));
  var expPct = expSpan > 0 ? Math.round(expCur / expSpan * 100) : 0;
  h += '<div class="pm-card-hp pm-card-exp"><div class="pm-card-hp-bar"><i class="pm-card-exp-fill" style="width:' + expPct + '%"></i></div>'
    + '<div class="pm-card-hp-num">EXP ' + esc(expCur) + ' / ' + esc(expSpan) + '</div></div>';
  h += '<div class="pm-card-stats">';
  for (var i = 0; i < STAT_NAMES.length; i++) {
    var v = p.stats ? p.stats[STAT_NAMES[i][0]] : 0;
    h += '<div class="pm-card-stat"><b>' + esc(v == null ? '-' : v) + '</b><span>' + STAT_NAMES[i][1] + '</span></div>';
  }
  h += '</div>';
  if (p.skills && p.skills.length) {
    h += '<div class="pm-card-skills"><h4>技能</h4>';
    for (var j = 0; j < p.skills.length; j++) {
      var sk = p.skills[j];
      if (!sk.name) continue;
      var pp = sk.max_pp > 0 ? sk.pp + '/' + sk.max_pp : '∞';
      h += '<div class="pm-card-skill"><span class="pm-skill-name">' + esc(sk.name)
        + (sk.element ? '<span class="pm-card-tag">' + esc(sk.element) + '</span>' : '')
        + '</span><span class="pm-skill-meta">威力 ' + esc(sk.power) + ' · PP ' + esc(pp) + '</span></div>';
    }
    h += '</div>';
  }
  modalBody.innerHTML = h;
}

function showCard(petEl){
  ensureModal();
  modal.classList.add('pm-open');
  modalBody.innerHTML = '<div class="pm-loading">加载中…</div>';
  var url = 'plugin.php?id=pokemon:pokemon&endpoint=pokemon&action=card&petid='
    + encodeURIComponent(petEl.getAttribute('data-petid') || '');
  var x = new XMLHttpRequest();
  x.open('GET', url);
  x.onload = function(){
    var d = null;
    try { d = JSON.parse(x.responseText); } catch (ex) {}
    if (!d || !d.success) {
      modalBody.innerHTML = '<div class="pm-loading">' + esc(d && d.error ? d.error : '加载失败，请稍后再试') + '</div>';
      return;
    }
    renderCard(d.data);
  };
  x.onerror = function(){
    modalBody.innerHTML = '<div class="pm-loading">网络错误，请稍后再试</div>';
  };
  x.send();
}

document.addEventListener('click', function(e){
  var t = e.target;
  while (t && t !== document.body) {
    if (t.classList && t.classList.contains('pm-size-btn')) {
      applySize(t.getAttribute('data-pm-size') || 'sm');
      return;
    }
    if (t.classList && t.classList.contains('pm-pet') && t.getAttribute('data-petid')) {
      e.preventDefault();
      showCard(t);
      return;
    }
    t = t.parentNode;
  }
});

document.addEventListener('keydown', function(e){
  if (e.keyCode === 27) hideModal();
});

// 初始尺寸：DOM 就绪后统一应用（徽章可能出现在本脚本之后的帖子里）
function initSize(){
  var saved = 'sm';
  try { saved = localStorage.getItem(LS_KEY) || 'sm'; } catch (ex) {}
  if (saved === 'lg') applySize('lg');
}
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initSize);
} else {
  initSize();
}
})();
JS;

    return '<style>' . $css . '</style><script>' . $js . '</script>';
}
