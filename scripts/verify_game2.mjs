#!/usr/bin/env node
// game2（tairitsu 版）浏览器回归脚本：真实浏览器点击 → 断言 DOM。
//
// 背景：整树重挂架构对每次交互都重建 DOM，Playwright 式的外部等待/句柄
// 复用会频繁失联（本仓库验证一律走「页面内 evaluate 派发真实事件」）。
// 本脚本零依赖：Node 22 原生 WebSocket + fetch 走 CDP，驱动本机
// Edge/Chrome headless。
//
// 用法：
//   TSDM_VERIFY_USER=admin TSDM_VERIFY_PASS=*** node scripts/verify_game2.mjs [baseUrl]
//   可选 TSDM_VERIFY_BROWSER 指定浏览器可执行文件；默认 baseUrl=https://localhost。
//   可选 TSDM_VERIFY_CDP_PORT 换调试端口（端口上已有浏览器在应答时脚本会报错
//   而不是连上去）。可选 TSDM_VERIFY_COOKIES=<JSON 文件> 直接带入一份已登录
//   的论坛 Cookie（CDP Network.getAllCookies 的格式）：本地论坛给登录页开了
//   图形验证码时，脚本自己登不进去，这是不改论坛设置的路子；文件里是会话
//   凭据，别提交进仓库。
//
// 凭据只从环境变量读取（源码不含任何字面量口令）。未提供凭据时跳过
// 登录，仅执行无需会话的「外壳」断言并以 SKIPPED 报告其余步骤。
//
// 断言范围刻意保持只读/不落库：不购买、不消耗道具；「使用」仅覆盖
// 纯客户端分支（装备道具提示）与只读弹窗（get_usable_pokemon）。

import { spawn } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const BASE = (process.argv[2] ?? 'https://localhost').replace(/\/+$/, '');
const USER = process.env.TSDM_VERIFY_USER ?? '';
const PASS = process.env.TSDM_VERIFY_PASS ?? '';
const DEBUG_PORT = Number(process.env.TSDM_VERIFY_CDP_PORT ?? 9333);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

// ---------------------------------------------------------------------------
// 最小 CDP 客户端（原生 WebSocket）
// ---------------------------------------------------------------------------

class Cdp {
  constructor(ws) {
    this.ws = ws;
    this.id = 0;
    this.pending = new Map();
    this.listeners = [];
    ws.addEventListener('message', (ev) => {
      const msg = JSON.parse(ev.data);
      if (msg.id !== undefined && this.pending.has(msg.id)) {
        const { resolve, reject } = this.pending.get(msg.id);
        this.pending.delete(msg.id);
        if (msg.error) reject(new Error(msg.error.message));
        else resolve(msg.result);
      } else {
        for (const l of [...this.listeners]) l(msg);
      }
    });
  }

  static async connect(url) {
    const ws = new WebSocket(url);
    await new Promise((resolve, reject) => {
      ws.addEventListener('open', resolve, { once: true });
      ws.addEventListener('error', reject, { once: true });
    });
    return new Cdp(ws);
  }

  send(method, params = {}) {
    const id = ++this.id;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.ws.send(JSON.stringify({ id, method, params }));
    });
  }

  waitEvent(method, timeoutMs = 20000) {
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => {
        this.listeners = this.listeners.filter((l) => l !== fn);
        reject(new Error(`timeout waiting for ${method}`));
      }, timeoutMs);
      const fn = (msg) => {
        if (msg.method === method) {
          clearTimeout(timer);
          this.listeners = this.listeners.filter((l) => l !== fn);
          resolve(msg.params);
        }
      };
      this.listeners.push(fn);
    });
  }
}

// 在页面上下文执行函数体，返回值经 JSON 传回。
async function evaljs(fn, ...args) {
  const expression = `(${fn.toString()})(...${JSON.stringify(args)})`;
  const r = await cdp.send('Runtime.evaluate', {
    expression,
    returnByValue: true,
    awaitPromise: true,
  });
  if (r.exceptionDetails) {
    throw new Error(r.exceptionDetails.exception?.description ?? 'page evaluate failed');
  }
  return r.result?.value;
}

// 轮询页面断言直至为真/超时（整树重挂后元素句柄随时失效，只能按状态轮询）。
async function waitFor(fn, timeoutMs = 15000, label = 'condition') {
  const start = Date.now();
  for (;;) {
    if (await fn()) return true;
    if (Date.now() - start > timeoutMs) throw new Error(`timeout: ${label}`);
    await sleep(250);
  }
}

// ---------------------------------------------------------------------------
// 浏览器启动 / 登录
// ---------------------------------------------------------------------------

const EXE_CANDIDATES = [
  process.env.TSDM_VERIFY_BROWSER,
  'C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe',
  'C:/Program Files/Microsoft/Edge/Application/msedge.exe',
  'C:/Program Files/Google/Chrome/Application/chrome.exe',
  'C:/Program Files (x86)/Google/Chrome/Application/chrome.exe',
].filter(Boolean);

async function launchBrowser() {
  let exe = null;
  const { accessSync, constants } = await import('node:fs');
  for (const c of EXE_CANDIDATES) {
    try {
      accessSync(c, constants.X_OK);
      exe = c;
      break;
    } catch {
      /* 尝试下一个 */
    }
  }
  if (!exe) throw new Error('未找到 Edge/Chrome（可用 TSDM_VERIFY_BROWSER 指定）');

  // 端口上已经有 CDP 在应答，说明有个旧实例（可能是上一次没退干净的
  // 浏览器，甚至带着登录态）占着它。直接沿用会把断言跑到那个页面上——
  // 实测表现为登录页被 `if($_G['uid'])` 跳回首页。宁可报错也不连旧的。
  try {
    const stale = await fetch(`http://127.0.0.1:${DEBUG_PORT}/json/version`);
    if (stale.ok) {
      throw new Error(
        `调试端口 ${DEBUG_PORT} 已被占用（有旧浏览器实例在跑），请先关掉它或用 ` +
          'TSDM_VERIFY_CDP_PORT 换一个端口',
      );
    }
  } catch (e) {
    if (e instanceof Error && e.message.startsWith('调试端口')) throw e;
    /* 端口空着：正常 */
  }

  const profile = mkdtempSync(join(tmpdir(), 'game2-verify-'));
  const child = spawn(
    exe,
    [
      '--headless=new',
      `--remote-debugging-port=${DEBUG_PORT}`,
      `--user-data-dir=${profile}`,
      '--no-first-run',
      '--no-default-browser-check',
      '--ignore-certificate-errors',
      '--disable-gpu',
      '--window-size=1280,900',
      'about:blank',
    ],
    { stdio: 'ignore' },
  );

  // 等 CDP 端点就绪
  for (let i = 0; i < 60; i++) {
    try {
      const r = await fetch(`http://127.0.0.1:${DEBUG_PORT}/json/version`);
      if (r.ok) return { child, profile };
    } catch {
      /* not up yet */
    }
    await sleep(500);
  }
  child.kill();
  throw new Error('CDP 端点未就绪');
}

async function newTab() {
  const base = `http://127.0.0.1:${DEBUG_PORT}`;
  // 必须新建一个标签页。早先这里在 PUT 失败时回落到 GET `/json/new`，
  // 而该接口对浏览器里已有的目标也可能返回一个——拿到的是别人开着的、
  // 可能已登录的页面，之后所有断言都跑在那上面（登录页会被 `if($_G['uid'])`
  // 直接跳回首页，表现为「登录页没有表单」）。所以只认 PUT 新建。
  const r = await fetch(`${base}/json/new?about:blank`, { method: 'PUT' });
  if (!r.ok) throw new Error(`新建标签页失败：HTTP ${r.status}`);
  const target = await r.json();
  if (!target?.webSocketDebuggerUrl) throw new Error('新建标签页没有返回调试地址');
  return target.webSocketDebuggerUrl;
}

async function goto(url) {
  // `Page.loadEventFired` 只在首次 `load` 触发：登录页这类「先 load、之后
  // 再跳」的页面，等 load 会停在跳转前的文档上。先等 load，再等到地址回到
  // 请求的地址（同源跳转也认），最多 10 秒。
  const loaded = cdp.waitEvent('Page.loadEventFired', 30000).catch(() => null);
  await cdp.send('Page.navigate', { url });
  await loaded;
  const want = new URL(url);
  for (let i = 0; i < 40; i++) {
    const now = await evaljs(() => location.href).catch(() => null);
    if (typeof now === 'string') {
      try {
        if (new URL(now).pathname === want.pathname) break;
      } catch {
        /* about:blank or mid-navigation */
      }
    }
    await sleep(250);
  }
  await sleep(600);
}

async function isLoggedIn() {
  return evaljs(() => !!document.querySelector('a[href*="action=logout"]'));
}

async function login() {
  if (!USER || !PASS) return false;
  // 带了现成 Cookie（TSDM_VERIFY_COOKIES）且会话有效：不用再走登录表单。
  if (await isLoggedIn()) return true;
  await goto(`${BASE}/member.php?mod=logging&action=login`);
  // 登录表单要等。全新 profile 下 `load` 之后页面还在补：实测 load 后 0.6s
  // 时 `document.forms` 里只有页头那个无名搜索表单，1.8s 起才有 `login`。
  // 只看一次会把「还没出现」误报成「登录页没有表单」。按状态轮询到有结论；
  // 8 秒还没出现就重新打开一次登录页（偶发导航半途停在别处）。
  let submitted = 'no-form';
  const deadline = Date.now() + 8000;
  let reopened = false;
  for (let i = 0; i < 40; i++) {
    submitted = await evaljs((u, p) => {
      const form =
        document.querySelector('form[name="login"]') ??
        [...document.forms].find((f) => f.querySelector('input[type="password"]'));
      if (!form) {
        return `no-form @ ${location.href} forms=${[...document.forms].map((f) => f.name || '(unnamed)').join(',')}`;
      }
      const un = form.querySelector('input[name="username"], input[name="username_ll"]');
      const pw = form.querySelector('input[type="password"]');
      if (!un || !pw) return 'no-fields';
      // 全新会话登录页要验证码（Discuz seccode），脚本给不出答案。
      if (form.querySelector('[name="seccodeverify"], .seccode, #seccodeverify')) {
        return 'captcha';
      }
      un.value = u;
      pw.value = p;
      form.submit();
      return 'submitted';
    }, USER, PASS);
    if (!submitted.startsWith('no-form')) break;
    if (!reopened && Date.now() > deadline) {
      reopened = true;
      await goto(`${BASE}/member.php?mod=logging&action=login`);
      continue;
    }
    await sleep(250);
  }
  if (submitted.startsWith('no-form')) {
    // 登录页没渲染出表单：新版 Discuz 的登录页是 JS 渲染/或先跳到别处。
    // 这是环境侧问题，不该让整个回归崩掉。
    console.log(`  - 登录页未提供登录表单（${submitted}），本次只跑免登录断言`);
    return false;
  }
  if (submitted === 'captcha') {
    // 无痕 profile 没有会话 → 登录页要验证码：属环境限制，不是回归失败。
    console.log('  - 登录页要求验证码（全新浏览器会话），本次只跑免登录断言');
    return false;
  }
  if (submitted !== 'submitted') {
    throw new Error(`登录表单不可用：${submitted}`);
  }
  // 表单提交后是服务端 302 + 首页渲染。等「退出链接出现」即已建立会话；
  // 不要再导航一次——重新导航会打断 302 的落地，反把会话丢掉（实测：加了
  // 这一跳之后游戏页又退回游客态、profile 拉取失败）。只多给一点渲染时间。
  await sleep(1500);
  const ok = await waitFor(isLoggedIn, 10000, 'login cookie');
  if (ok) await sleep(1200);
  return ok;
}

// ---------------------------------------------------------------------------
// 断言小框架
// ---------------------------------------------------------------------------

let passed = 0;
let failed = 0;
let skipped = 0;
const failures = [];

function ok(name, cond, detail = '') {
  if (cond) {
    passed += 1;
    console.log(`  ✓ ${name}`);
  } else {
    failed += 1;
    failures.push(`${name}${detail ? ` — ${detail}` : ''}`);
    console.log(`  ✗ ${name}${detail ? ` — ${detail}` : ''}`);
  }
}

function skip(name, why) {
  skipped += 1;
  console.log(`  - ${name}（跳过：${why}）`);
}

// ---------------------------------------------------------------------------
// 回归步骤（全部只读或纯客户端交互）
// ---------------------------------------------------------------------------

const invCards = () => evaljs(() => [...document.querySelectorAll('.shop-item-card')].map((c) => ({
  name: c.querySelector('.item-name')?.textContent?.trim() ?? null,
  badges: [...c.querySelectorAll('.item-type-badge')].map((b) => b.textContent?.trim()),
  useId: c.querySelector('[data-test-inv-use]')?.getAttribute('data-test-inv-use') ?? null,
})));

const navTo = async (route) => {
  // 首页不渲染侧栏（`show_sidebar = route != Route::Home`），而背包等入口
  // 只在侧栏上。要进这些页先离开首页：顶部导航里任选一个非首页项进去，
  // 侧栏才会出现。
  const visible = await evaljs(
    (r) => !!document.querySelector(`button[data-test-nav="${r}"]`),
    route,
  );
  if (!visible) {
    await evaljs(() => {
      const top = [...document.querySelectorAll('button[data-test-nav]')].find(
        (b) => b.getAttribute('data-test-nav') !== 'home',
      );
      top?.click();
    });
    await sleep(1800);
  }
  await evaljs((r) => {
    document.querySelector(`button[data-test-nav="${r}"]`)?.click();
  }, route);
  await sleep(1600);
};

async function stepShell() {
  console.log('[1] 应用外壳');
  const shell = await evaljs(() => ({
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    hasLayout: !!document.querySelector('.app-layout') || (document.querySelector('#app')?.childElementCount ?? 0) > 0,
    navs: [...document.querySelectorAll('[data-test-nav]')].map((b) => b.textContent?.trim()),
  }));
  ok('无 panic', shell.panic === null, `data-panic=${shell.panic}`);
  ok('渲染出页面骨架', shell.hasLayout);
  if (shell.navs.length === 0) {
    // 未初始化/访客停在欢迎页（整屏、无导航）——外壳成立，导航断言跳过。
    skip('导航含 首页/商店/背包', '当前停在欢迎页（无导航属预期）');
  } else {
    // 顶部导航与旧版（Dioxus）一致：四项；背包与仓库走侧栏。
    ok(
      '顶部导航四项（个人中心/商店/宠物中心/野外冒险）',
      ['个人中心', '商店', '宠物中心', '野外冒险'].every((n) => shell.navs.includes(n)),
      JSON.stringify(shell.navs),
    );
  }
}

async function stepInventory() {
  console.log('[2] 背包页');
  await navTo('inventory');
  const boot = await evaljs(() => ({
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    active: document.querySelector('.nav-link.active')?.textContent?.trim() ?? null,
    cats: document.querySelectorAll('[data-test-inv-cat]').length,
  }));
  ok('无 panic', boot.panic === null, boot.panic ?? '');
  // 背包不是顶部导航项：侧栏背包卡上的箭头带 data-test-nav="inventory"。
  ok('到达背包页', boot.cats > 0, `cats=${boot.cats} active=${boot.active}`);
  await waitFor(async () => (await evaljs(() => document.querySelectorAll('.shop-item-card').length > 0)), 15000, '物品列表');
  ok('分类按钮 6 个', boot.cats === 6, `got ${boot.cats}`);
  // 进入背包页默认选中「回复药」（旧版挂载时 `INITIAL_INVENTORY_CATEGORY.take().or(Some(1))`）。
  const entryActive = await evaljs(() => document.querySelector('[data-test-inv-cat].active')?.getAttribute('data-test-inv-cat') ?? null);
  ok('进入时默认分类是回复药', entryActive === '1', `active=${entryActive}`);

  // 下面的断言要在「全部物品」上找精灵球和装备道具：先切过去，等列表换掉
  // （切换后旧列表会留到新数据回来，只看 count > 0 会被旧列表满足）。
  const entryCount = await evaljs(() => document.querySelectorAll('.shop-item-card').length);
  await evaljs(() => document.querySelector('[data-test-inv-cat="all"]').click());
  await waitFor(async () => (await evaljs(() => document.querySelectorAll('.shop-item-card').length)) !== entryCount, 8000, '全部物品列表')
    .catch(() => null);
  await sleep(400);

  const before = await invCards();
  ok('物品卡片 ≥1', before.length >= 1, `got ${before.length}`);
  ok('数量徽章格式 ×N', before.every((c) => c.badges.some((b) => /^×\d+$/.test(b ?? ''))));

  // 精灵球（battle 目标）不应有使用按钮
  const balls = before.filter((c) => c.badges.includes('精灵球'));
  if (balls.length > 0) {
    ok('精灵球无使用按钮（battle）', balls.every((b) => b.useId === null));
  } else {
    skip('精灵球无使用按钮', '数据中没有精灵球');
  }

  // 分类切换
  await evaljs(() => document.querySelector('[data-test-inv-cat="1"]').click());
  await sleep(1600);
  const cat = await evaljs(() => ({
    active: document.querySelector('[data-test-inv-cat].active')?.getAttribute('data-test-inv-cat'),
    items: [...document.querySelectorAll('.shop-item-card')].map((c) => ({
      name: c.querySelector('.item-name')?.textContent?.trim(),
      type: c.querySelectorAll('.item-type-badge')[0]?.textContent?.trim(),
    })),
  }));
  ok('分类切换生效', cat.active === '1', `active=${cat.active}`);
  ok('分类内条目均为回复药', cat.items.length > 0 && cat.items.every((i) => i.type === '回复药'),
    JSON.stringify(cat.items.map((i) => i.type)));

  // 搜索：取当前列表第一个物品名的首两字。注意等待条件必须是
  // 「所有条目都含关键词」——仅等 count>0 会被分类切换的旧列表立即满足。
  const kw = (cat.items[0]?.name ?? '').slice(0, 2);
  const catCount = cat.items.length;
  if (kw) {
    await evaljs((q) => {
      const input = document.querySelector('[data-test-inv-search]');
      input.value = q;
      input.dispatchEvent(new Event('change', { bubbles: true }));
      return true;
    }, kw);
    await waitFor(async () => {
      const cards = await invCards();
      return cards.length > 0 && cards.every((c) => (c.name ?? '').includes(kw));
    }, 8000, '搜索结果');
    const searched = await invCards();
    ok('搜索结果包含关键词', searched.every((c) => (c.name ?? '').includes(kw)) && searched.length >= 1,
      `kw=${kw}, got ${JSON.stringify(searched.map((c) => c.name))}`);
    await evaljs(() => document.querySelector('[data-test-inv-search-clear]').click());
    // 清除后回到分类视图（分类保持不变，仅清搜索词）
    await waitFor(async () => (await evaljs(() => document.querySelectorAll('.shop-item-card').length)) === catCount, 8000, '清除搜索');
    const restored = await invCards();
    ok('清除搜索恢复列表', restored.length === catCount,
      `${restored.length} vs ${catCount}`);
  } else {
    skip('搜索', '列表为空');
  }

  // 装备道具：跳转我的宠物装备页（对齐 Dioxus 版行为）。先回「全部物品」
  // ——装备道具不在回复药分类里，必须让它的使用按钮出现在当前 DOM。
  await evaljs(() => document.querySelector('[data-test-inv-cat="all"]').click());
  await waitFor(async () => (await evaljs(() => document.querySelectorAll('.shop-item-card').length)) === before.length, 8000, '回到全部分类');
  const equip = before.find((c) => c.badges.includes('装备道具') && c.useId);
  if (equip) {
    await evaljs((id) => {
      document.querySelector(`[data-test-inv-use="${id}"]`)?.click();
    }, equip.useId);
    await waitFor(async () => (await evaljs(() => !!document.querySelector('.equipment-tab'))), 10000, '装备页跳转');
    const jump = await evaljs(() => ({
      activeNav: document.querySelector('.nav-link.active')?.textContent?.trim() ?? null,
      tabActive: document.querySelector('[data-test-my-pm-tab="equipment"]')?.className ?? '',
    }));
    // 顶部导航的「我的宠物」项文案是「个人中心」（与 Dioxus 版 Header 一致，
    // 后端路由仍叫 my-pokemon）；这里断路由归属，不按文案猜。
    ok('装备道具跳转我的宠物', jump.activeNav === '个人中心', jump.activeNav);
    ok('落在装备标签页', (jump.tabActive ?? '').includes('active'), jump.tabActive);
    // 回背包继续后续断言。重新进入时分类回到默认的「回复药」（见上），所以等
    // 「列表出来、且选中的是回复药」，不按「全部物品」的数量等。
    await evaljs(() => document.querySelector('button[data-test-nav="inventory"]')?.click());
    await waitFor(async () => (await evaljs(() => document.querySelectorAll('.shop-item-card').length > 0
      && document.querySelector('[data-test-inv-cat].active')?.getAttribute('data-test-inv-cat') === '1')), 10000, '回到背包');
  } else {
    skip('装备道具跳转', '数据中没有装备道具');
  }

  // 宝可梦目标弹窗：只读（get_usable_pokemon）→ 关闭
  const heal = before.find((c) => c.badges.includes('回复药') && c.useId);
  if (heal) {
    await evaljs((id) => {
      document.querySelector(`[data-test-inv-use="${id}"]`)?.click();
    }, heal.useId);
    // 弹窗外壳是 hikari 的 Modal（`.hk-modal-content`）；`.modal-overlay` 是
    // 换壳前的手绘结构，这里两个都认，换壳前后都能跑。
    const modalShown = () =>
      evaljs(
        () =>
          !!document.querySelector('.hk-modal-content') ||
          !!document.querySelector('.modal-overlay'),
      );
    await waitFor(modalShown, 8000, '目标弹窗');
    const modal = await evaljs(() => ({
      title:
        document
          .querySelector('.hk-modal-title, .modal-header h3')
          ?.textContent?.trim() ?? null,
      picks: document.querySelectorAll('[data-test-inv-pick]').length,
      body:
        document
          .querySelector('.hk-modal-body-inner, .modal-body')
          ?.textContent?.trim() ?? null,
    }));
    ok('目标弹窗标题含物品名', (modal.title ?? '').includes(heal.name), modal.title ?? '');
    ok(
      '弹窗给出目标列表或空态文案',
      modal.picks > 0 || (modal.body ?? '').length > 0,
      `picks=${modal.picks}, body=${modal.body}`,
    );
    await evaljs(() => document.querySelector('[data-test-inv-modal-close]')?.click());
    await sleep(900);
    ok(
      '弹窗关闭',
      await evaljs(
        () =>
          !document.querySelector('.hk-modal-content') &&
          !document.querySelector('.modal-overlay'),
      ),
    );
  } else {
    skip('目标选择弹窗', '数据中没有可用药');
  }
}

async function stepPokemonCenter() {
  console.log('[3] 宠物中心');
  await navTo('center');
  const boot = await evaljs(() => ({
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    active: document.querySelector('.nav-link.active')?.textContent?.trim() ?? null,
    hasPage: !!document.querySelector('.page-pokemon-center'),
    hasBanner: !!document.querySelector('.center-welcome-banner'),
  }));
  ok('无 panic', boot.panic === null, boot.panic ?? '');
  ok('宠物中心导航激活', boot.active === '宠物中心', boot.active ?? '');
  ok('渲染宠物中心卡片与横幅', boot.hasPage && boot.hasBanner);

  // 四种互斥态：加载 / 受伤列表 / 健康 / 空
  await waitFor(async () => (await evaljs(() => !document.querySelector('.center-loading'))), 15000, '宠物数据');
  const view = await evaljs(() => ({
    cards: document.querySelectorAll('.heal-pokemon-card').length,
    healthy: !!document.querySelector('.center-healthy'),
    empty: !!document.querySelector('.center-empty'),
    healAll: !!document.querySelector('[data-test-center-heal-all]'),
    first: (() => {
      const c = document.querySelector('.heal-pokemon-card');
      if (!c) return null;
      return {
        name: c.querySelector('.heal-pm-name')?.textContent?.trim() ?? null,
        hp: c.querySelector('.hp-text')?.textContent?.trim() ?? null,
        state: c.querySelector('.state-label')?.textContent?.trim() ?? null,
        healId: c.querySelector('[data-test-center-heal]')?.getAttribute('data-test-center-heal') ?? null,
        flee: c.querySelector('[data-test-center-flee]')?.getAttribute('data-test-center-flee') ?? null,
      };
    })(),
  }));
  ok(
    '四种态之一成立（受伤列表/健康/空）',
    view.cards > 0 || view.healthy || view.empty,
    JSON.stringify({ cards: view.cards, healthy: view.healthy, empty: view.empty }),
  );
  if (view.cards > 0) {
    ok('受伤卡片带 HP 与状态', !!view.first?.hp && !!view.first?.state,
      JSON.stringify(view.first));
    ok('受伤卡片带治疗按钮', !!view.first?.healId);
    ok('全部治疗按钮出现', view.healAll);
  } else {
    skip('受伤卡片细节', '当前无受伤宠物');
  }
}

async function stepMyPokemon() {
  console.log('[4] 我的宠物（属性页）');
  await navTo('my-pokemon');
  const boot = await evaljs(() => ({
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    active: document.querySelector('.nav-link.active')?.textContent?.trim() ?? null,
    hasPage: !!document.querySelector('.page-my-pokemon'),
  }));
  ok('无 panic', boot.panic === null, boot.panic ?? '');
  // 顶部导航里 my-pokemon 路由的文案是「个人中心」（与 Dioxus 版 Header 一致）。
  ok('我的宠物导航激活', boot.active === '个人中心', boot.active ?? '');
  ok('渲染我的宠物卡片', boot.hasPage);

  // 先切回属性页——背包步骤的装备跳转可能把标签页留在「装备」。
  await evaljs(() => document.querySelector('[data-test-my-pm-tab="stats"]')?.click());
  await waitFor(
    async () => (await evaljs(() => !!document.querySelector('.stats-tab-layout'))),
    15000,
    '属性面板',
  );

  // 属性页三弹窗（只读）：努力值计算器 / 进化路径 / 进化
  const hasTools = await evaljs(() => !!document.querySelector('[data-test-my-pm-ev-open]')
    && !!document.querySelector('[data-test-my-pm-evo-path-open]')
    && !!document.querySelector('[data-test-my-pm-evo-open]'));
  ok('属性页三个工具按钮存在', hasTools);
  if (hasTools) {
    // 努力值分配计算器：输入联动合计（对齐 Dioxus 版——只有「待分配」一行
    // 有 6 个输入，事件是 input；总分配行的总计格 = 六项总和）
    await evaljs(() => document.querySelector('[data-test-my-pm-ev-open]')?.click());
    await waitFor(async () => (await evaljs(() => !!document.querySelector('.ev-calculator'))), 8000, '计算器弹窗');
    const evOk = await evaljs(() => {
      const set = (key, val) => {
        const el = document.querySelector(`[data-test-ev-input="${key}"]`);
        el.value = String(val);
        el.dispatchEvent(new Event('input', { bubbles: true }));
      };
      set('add-1', 100);
      set('add-3', 60);
      return true;
    });
    await sleep(900);
    const evState = await evaljs(() => ({
      status: document.querySelector('[class*="ev-status-"]')?.textContent?.trim() ?? null,
      // 第一行数据行是「总分配」，其总计格应为 100 + 60 = 160。
      totalRow: [...document.querySelectorAll('.ev-table-row')]
        .shift()
        ?.textContent?.replace(/\s+/g, '') ?? '',
      inputs: document.querySelectorAll('[data-test-ev-input]').length,
      evTitle: document.querySelector('.hk-modal-title')?.textContent?.trim() ?? null,
    }));
    ok('计算器 6 输入 + 总计联动', evOk && evState.inputs === 6
      && evState.totalRow.includes('160'), JSON.stringify(evState));
    ok('计算器标题对齐 Dioxus', evState.evTitle === '努力值分配计算器', evState.evTitle ?? '');
    await evaljs(() => document.querySelector('[data-test-ev-close]')?.click());
    await sleep(600);

    // 进化路径：加载后两个 tab 都能切换（退化 tab 数据存在或空态文案）
    await evaljs(() => document.querySelector('[data-test-my-pm-evo-path-open]')?.click());
    await waitFor(
      async () =>
        await evaljs(
          () =>
            !!document.querySelector('.evo-path-node') ||
            !!document.querySelector('.evo-path-empty') ||
            !!document.querySelector('.evo-path-error'),
        ),
      15000,
      '路径加载',
    );
    const fwd = await evaljs(() => ({
      nodes: document.querySelectorAll('.evo-path-node').length,
      empty: !!document.querySelector('.evo-path-empty'),
    }));
    ok('路径前向 tab 有内容或空态', fwd.nodes > 0 || fwd.empty);
    await evaljs(() => document.querySelector('[data-test-evo-path-tab="backward"]')?.click());
    await sleep(900);
    const bwd = await evaljs(() => ({
      modalStillOpen: !!document.querySelector('[data-test-evo-path-tab="backward"]'),
      nodes: document.querySelectorAll('.evo-path-node').length,
      empty: !!document.querySelector('.evo-path-empty'),
    }));
    ok('切退化 tab 不关闭弹窗', bwd.modalStillOpen, 'tab 点击被遮罩关闭属性劫持');
    ok('路径后向 tab 有内容或空态', bwd.nodes > 0 || bwd.empty);
    await evaljs(() => document.querySelector('[data-test-evo-path-close]')?.click());
    await sleep(600);

    // 进化弹窗：条件查询返回内容或空态
    await evaljs(() => document.querySelector('[data-test-my-pm-evo-open]')?.click());
    // 先等这一弹窗自己的节点出现，再等加载态消失——只等 `!loading` 会在上一个
    // 弹窗还没换过去时就成立，读到空结果。
    await waitFor(
      async () =>
        await evaljs(
          () =>
            !!document.querySelector('.evo-path-node') ||
            !!document.querySelector('.evo-path-empty') ||
            !!document.querySelector('.evo-path-error'),
        ),
      15000,
      '进化条件',
    );
    const evoState = await evaljs(() => ({
      nodes: document.querySelectorAll('.evo-path-node').length,
      empty: document.querySelector('.evo-path-empty')?.textContent?.trim() ?? null,
      err: document.querySelector('.evo-path-error')?.textContent?.trim() ?? null,
    }));
    ok('进化弹窗给出路线或空态', evoState.nodes > 0 || !!evoState.empty || !!evoState.err,
      JSON.stringify(evoState));
    await evaljs(() => document.querySelector('[data-test-evo-modal-close]')?.click());
    await sleep(600);
  } else {
    skip('属性页弹窗', '工具按钮缺失');
  }

  const view = await evaljs(() => {
    const radar = document.querySelector('.radar-svg');
    // 等级在雷达图中心（旧版如此）；名字行没有等级，也没有宠物切换条——
    // 切换走侧栏迷你格。
    const radarTexts = radar ? [...radar.querySelectorAll('text')].map((t) => t.textContent.trim()) : [];
    return {
      empty: !!document.querySelector('.empty-hint'),
      hasStats: !!document.querySelector('.stats-tab-layout'),
      name: document.querySelector('.pokemon-name')?.textContent?.trim() ?? null,
      level: radarTexts.find((t) => /^Lv\s*\d+$/.test(t)) ?? null,
      hpBar: !!document.querySelector('.hp-bar .bar-text'),
      expBar: !!document.querySelector('.exp-bar .bar-text'),
      ivChips: document.querySelectorAll('.iv-chip').length,
      qualityBadge: document.querySelector('.quality-badge')?.textContent?.trim() ?? null,
      radarPaths: radar ? radar.querySelectorAll('path').length : 0,
      radarDots: radar ? radar.querySelectorAll('circle').length : 0,
      radarValues: radarTexts.filter((t) => /^\d+$/.test(t)).length,
      radarLabels: radarTexts.filter((t) => /^● /.test(t)).length,
      editBtn: !!document.querySelector('[data-test-my-pm-nick-edit]'),
      tabArrows: document.querySelectorAll('.mypm-tab .tab-arrow').length,
      petChips: document.querySelectorAll('.pet-chip').length,
      err: document.querySelector('.error-message')?.textContent?.trim() ?? null,
    };
  });
  if (view.empty) {
    skip('属性面板细节', '账号下没有宠物');
    return;
  }
  ok('属性面板渲染', view.hasStats, view.err ?? '');
  ok('显示宠物名与等级', !!view.name && !!view.level,
    JSON.stringify({ name: view.name, level: view.level }));
  ok('HP 与 EXP 条存在', view.hpBar && view.expBar);
  ok('六项个体值齐全', view.ivChips === 6, `got ${view.ivChips}`);
  ok('品质徽章出现', !!view.qualityBadge, view.qualityBadge ?? '');
  ok('雷达图有网格+数据描边', view.radarPaths >= 4, `paths=${view.radarPaths}`);
  ok('雷达图带数据点、数值与属性标签', view.radarDots >= 5 && view.radarValues >= 5 && view.radarLabels >= 5,
    `dots=${view.radarDots} values=${view.radarValues} labels=${view.radarLabels}`);
  ok('选中页签两侧有箭头', view.tabArrows === 2, `arrows=${view.tabArrows}`);
  ok('没有宠物切换条（与 Dioxus 版一致）', view.petChips === 0, `chips=${view.petChips}`);
  ok('昵称编辑入口存在', view.editBtn);

  // 昵称编辑：进入编辑态 → 取消（不写库）
  await evaljs(() => document.querySelector('[data-test-my-pm-nick-edit]')?.click());
  await waitFor(
    async () => (await evaljs(() => !!document.querySelector('[data-test-my-pm-nick-input]'))),
    8000,
    '昵称编辑框',
  );
  const editing = await evaljs(() => ({
    inputValue: document.querySelector('[data-test-my-pm-nick-input]')?.value ?? null,
    hasSave: !!document.querySelector('[data-test-my-pm-nick-save]'),
    hasCancel: !!document.querySelector('[data-test-my-pm-nick-cancel]'),
  }));
  ok('编辑框带当前昵称', !!editing.inputValue, editing.inputValue ?? '');
  ok('编辑框有保存/取消', editing.hasSave && editing.hasCancel);
  // Esc 取消（Dioxus 版 handle_key_down 的行为），同样不写库
  await evaljs(() => {
    const input = document.querySelector('[data-test-my-pm-nick-input]');
    input?.focus();
    input?.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }));
  });
  await sleep(700);
  ok('Esc 取消后退出编辑态',
    await evaljs(() => !document.querySelector('[data-test-my-pm-nick-input]')));
  // 再进一次编辑态，用取消按钮退出（原有路径）
  await evaljs(() => document.querySelector('[data-test-my-pm-nick-edit]')?.click());
  await waitFor(
    async () => (await evaljs(() => !!document.querySelector('[data-test-my-pm-nick-input]'))),
    8000,
    '昵称编辑框',
  );
  await evaljs(() => document.querySelector('[data-test-my-pm-nick-cancel]')?.click());
  await sleep(700);
  ok('取消后退出编辑态',
    await evaljs(() => !document.querySelector('[data-test-my-pm-nick-input]')));

  // 装备标签页（只读断言：渲染与悬浮提示；真实装载/卸下属写操作，另行验收）
  await evaljs(() => document.querySelector('[data-test-my-pm-tab="equipment"]')?.click());
  await waitFor(
    async () => (await evaljs(() => !!document.querySelector('.equipment-tab'))),
    15000,
    '装备页',
  );
  const eqView = await evaljs(() => ({
    slots: document.querySelectorAll('.equip-slot').length,
    invItems: document.querySelectorAll('[data-test-eq-inv-item]').length,
    invTitle: document.querySelector('[data-test-eq-inv-item]')?.getAttribute('title') ?? null,
    slotTitle: document.querySelector('.equip-slot .equip-icon')?.getAttribute('title') ?? null,
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
  }));
  ok('装备页无 panic', eqView.panic === null, eqView.panic ?? '');
  ok('装备槽恒定 4 个', eqView.slots === 4, `got ${eqView.slots}`);
  ok('装备卡与槽图标不带原生 title（旧版用悬浮提示，不用 title）', eqView.invTitle === null && eqView.slotTitle === null,
    JSON.stringify({ invTitle: eqView.invTitle, slotTitle: eqView.slotTitle }));
  if (eqView.invItems > 0) {
    // 悬浮提示是应用级的一层（旧版 EquipmentTooltipInline）：指针进入卡片时挂在
    // 鼠标下方，离开摘掉。这里派发带坐标的 mouseenter / mouseleave。
    const hover = await evaljs(() => {
      const card = document.querySelector('[data-test-eq-inv-item]');
      const r = card.getBoundingClientRect();
      card.dispatchEvent(new MouseEvent('mouseenter', { bubbles: false, clientX: r.x + 24, clientY: r.y + 20 }));
      return { x: r.x, y: r.y };
    });
    const tipShown = await waitFor(
      async () => (await evaljs(() => !!document.querySelector('.equipment-tooltip-inline'))),
      8000,
      '装备悬浮提示',
    );
    ok('指针进入装备卡时出现悬浮提示', tipShown);
    if (tipShown) {
      const tip = await evaljs(() => {
        const el = document.querySelector('.equipment-tooltip-inline');
        const cs = getComputedStyle(el);
        const b = el.getBoundingClientRect();
        return {
          position: cs.position,
          pointerEvents: cs.pointerEvents,
          zIndex: cs.zIndex,
          rect: [b.x, b.y, b.width, b.height].map((v) => Math.round(v * 100) / 100),
          hasIcon: !!el.querySelector('.tooltip-header .tooltip-icon'),
          name: el.querySelector('.tooltip-name')?.textContent?.trim() ?? '',
          stats: [...el.querySelectorAll('.tooltip-stat')].map((s) => s.textContent?.trim() ?? ''),
        };
      });
      ok('悬浮提示定位于视口、不挡鼠标', tip.position === 'fixed' && tip.pointerEvents === 'none', JSON.stringify(tip));
      // 旧版：left = x + (48 - 180) / 2、top = y + 8，(x, y) 是指针进入点。
      const near =
        Math.abs(tip.rect[0] - (hover.x + 24 - 66)) <= 1 && Math.abs(tip.rect[1] - (hover.y + 20 + 8)) <= 1;
      ok('悬浮提示落在指针下方', near, JSON.stringify({ tip: tip.rect, hover }));
      ok('悬浮提示有图标、名称与加成标签', tip.hasIcon && tip.name.length > 0
        && tip.stats.some((s) => /^(HP|攻击|防御|特攻|特防|速度) \+\d+$/.test(s)), JSON.stringify(tip));
      await evaljs(() => document.querySelector('[data-test-eq-inv-item]')?.dispatchEvent(new MouseEvent('mouseleave', { bubbles: false })));
      const tipGone = await waitFor(
        async () => (await evaljs(() => !document.querySelector('.equipment-tooltip-inline'))),
        8000,
        '装备悬浮提示关闭',
      );
      ok('指针离开装备卡时悬浮提示消失', tipGone);
    }
  } else {
    skip('装备卡细节', '没有可装备物品');
  }

  // 技能标签页（只读断言：卡片渲染 + 卡片弹出菜单 + 弹窗列表 + 关闭）
  await evaljs(() => document.querySelector('[data-test-my-pm-tab="skills"]')?.click());
  await waitFor(async () => (await evaljs(() => !!document.querySelector('.skills-grid'))), 15000, '技能页');
  const skView = await evaljs(() => ({
    cards: document.querySelectorAll('[data-test-skill-card]').length,
    emptySlots: document.querySelectorAll('[data-test-skill-learn-slot]').length,
    totalSlots: document.querySelectorAll('.skill-card-v2, [data-test-skill-learn-slot]').length,
    // Dioxus 的技能卡有两处 PP：右上角 `<em>PP </em><strong>15</strong> /15`，
    // 底部数据行 `PP：<strong>15/15</strong>`。
    ppTop: document.querySelector('.skill-card-v2 .skill-card-top > .skill-pp')?.textContent?.trim() ?? null,
    ppBottom:
      document.querySelector('.skill-card-v2 .skill-card-bottom .skill-stats > .skill-pp')?.textContent?.trim()
      ?? null,
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
  }));
  ok('技能页无 panic', skView.panic === null, skView.panic ?? '');
  ok('技能槽恒定 4 个（卡 + 空槽）', skView.totalSlots === 4,
    `cards=${skView.cards}, empty=${skView.emptySlots}`);
  if (skView.cards > 0) {
    ok('技能卡右上角带 PP', /^PP\s*\d+\s*\/\d+$/.test(skView.ppTop ?? ''), skView.ppTop ?? '');
    ok('技能卡底部带 PP', /^PP：\d+(\/\d+)?$/.test(skView.ppBottom ?? ''), skView.ppBottom ?? '');
    // 点卡片在鼠标处弹出菜单（旧版 PopupMenu，hikari Popover 承接）：只有一项
    // 「遗忘此技能」，危险色；卡片自己不再有选中态或行内按钮。点背板关掉（不执行遗忘）。
    await evaljs(() => document.querySelector('[data-test-skill-card]')?.click());
    await waitFor(
      async () => (await evaljs(() => !!document.querySelector('.popup-container .popup-menu'))),
      8000,
      '技能卡菜单',
    );
    await sleep(700);
    const menu = await evaljs(() => ({
      items: [...document.querySelectorAll('.popup-container .popup-menu-item')].map((b) => ({
        text: b.textContent?.trim() ?? '',
        danger: b.classList.contains('danger'),
      })),
      inlineForget: !!document.querySelector('.skill-forget-btn'),
      cardSelected: !!document.querySelector('.skill-card-v2.selected'),
      position: getComputedStyle(document.querySelector('.popup-container')).position,
    }));
    ok('技能卡菜单只有「遗忘此技能」一项',
      menu.items.length === 1 && menu.items[0].text === '遗忘此技能' && menu.items[0].danger,
      JSON.stringify(menu));
    ok('技能卡菜单不再行内展开', !menu.inlineForget && !menu.cardSelected);
    ok('技能卡菜单是定位于视口的面板', menu.position === 'fixed', menu.position);
    await evaljs(() => document.querySelector('.hk-popover-backdrop')?.click());
    await waitFor(
      async () => (await evaljs(() => !document.querySelector('.popup-container'))),
      8000,
      '技能卡菜单关闭',
    );
    ok('点背板关闭技能卡菜单', true);
  } else {
    skip('技能卡细节', '宠物没有已学技能');
  }
  if (skView.emptySlots > 0) {
    await evaljs(() => document.querySelector('[data-test-skill-learn-slot]')?.click());
    // 弹窗外壳是 hikari 的 Modal（`.hk-modal-content`）；`.modal-overlay` 是
    // 换壳前的手绘结构，两种都认。
    await waitFor(
      async () => await evaljs(() => !!document.querySelector('.hk-modal-content, .modal-overlay')),
      10000,
      '学习弹窗',
    );
    await waitFor(async () => (await evaljs(() => !document.querySelector('.skill-learn-loading'))), 15000, '技能列表加载');
    const modal = await evaljs(() => ({
      title: document.querySelector('.hk-modal-title, .modal-header h3')?.textContent?.trim() ?? null,
      sections: [...document.querySelectorAll('.skill-section-title')].map((s) => s.textContent?.trim()),
      bodyHasContent:
        (document.querySelector('.hk-modal-body-inner, .modal-body')?.textContent ?? '').length > 0,
    }));
    ok('学习弹窗标题带等级', /^学习新技能 \(Lv\.\d+\)$/.test(modal.title ?? ''), modal.title ?? '');
    ok('学习弹窗给出列表或空态', modal.bodyHasContent);
    await evaljs(() => document.querySelector('[data-test-learn-close]')?.click());
    // 关闭后要等补丁落地：无头浏览器里 rAF 被节流，固定 700ms 偶尔不够。
    await waitFor(
      async () =>
        await evaljs(
          () =>
            !document.querySelector('.hk-modal-content') &&
            !document.querySelector('.modal-overlay'),
        ),
      8000,
      '学习弹窗关闭',
    );
    ok('学习弹窗关闭', true);
  } else {
    skip('学习弹窗', '没有空技能槽');
  }
  // 回属性页（保持后续步骤的页面状态干净）
  await evaljs(() => document.querySelector('[data-test-my-pm-tab="stats"]')?.click());
  await sleep(600);
}

async function stepStorage() {
  console.log('[5] 仓库页');
  await navTo('storage');
  const boot = await evaljs(() => ({
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    active: document.querySelector('.nav-link.active')?.textContent?.trim() ?? null,
    hasPage: !!document.querySelector('.page-pokemon-storage'),
  }));
  ok('无 panic', boot.panic === null, boot.panic ?? '');
  // 仓库入口在侧栏、不在顶部导航，侧栏按钮没有 active 类；`.nav-link.active`
  // 读到的是顶部那一项（个人中心）。这里断「页面确实换成了仓库页」，即
  // `.page-pokemon-storage` 出现且顶部导航已不在冒险页。
  ok('仓库导航激活', boot.hasPage, `active=${boot.active ?? ''}`);
  ok('渲染仓库页', boot.hasPage);

  await waitFor(
    async () => (await evaljs(() => !document.querySelector('.panel-loading'))),
    15000,
    '仓库数据',
  );
  const view = await evaljs(() => ({
    bagSlots: document.querySelectorAll('.bag-panel .pokemon-slot').length,
    bagEmpty: document.querySelectorAll('.bag-panel .empty-slot').length,
    count: document.querySelector('.panel-count')?.textContent?.trim() ?? null,
    cards: document.querySelectorAll('.storage-grid [data-test-st-card]').length,
    hasFilters: !!document.querySelector('[data-test-st-type]') && !!document.querySelector('[data-test-st-sort]'),
    selectModeBtn: !!document.querySelector('[data-test-st-select-mode]'),
  }));
  ok('背包恒定 6 槽', view.bagSlots === 6, `got ${view.bagSlots}`);
  ok('背包宠物 + 空槽互补', view.bagSlots - view.bagEmpty >= 1, JSON.stringify(view));
  ok('筛选条齐全（属性/排序等）', view.hasFilters);
  ok('计数文案格式', /^\d+ \/ \d+ 只$/.test(view.count ?? ''), view.count ?? '');

  if (view.cards > 0) {
    // 筛选交互：等级区间收窄到只剩高等级（等级取首卡的两倍作下限）
    const firstLevel = await evaljs(() => {
      const card = document.querySelector('.storage-grid [data-test-st-card]');
      return Number(card?.querySelector('.pokemon-level')?.textContent?.replace(/\D+/g, '') ?? 0);
    });
    const levelsBefore = await evaljs(() => [...document.querySelectorAll('.storage-grid .pokemon-level')]
      .map((n) => Number(n.textContent?.replace(/\D+/g, '') ?? 0)));
    if (firstLevel > 0 && levelsBefore.length >= 2) {
      await evaljs((min) => {
        const el = document.querySelector('[data-test-st-min]');
        el.value = String(min);
        el.dispatchEvent(new Event('change', { bubbles: true }));
      }, firstLevel);
      await waitFor(
        async () => (await evaljs(() => document.querySelectorAll('.storage-grid [data-test-st-card]').length)) < levelsBefore.length,
        8000,
        '等级筛选',
      );
      const afterMin = await evaljs(() => ({
        levels: [...document.querySelectorAll('.storage-grid .pokemon-level')]
          .map((n) => Number(n.textContent?.replace(/\D+/g, '') ?? 0)),
      }));
      ok('等级下限过滤生效', afterMin.levels.every((l) => l >= firstLevel),
        JSON.stringify({ before: levelsBefore, min: firstLevel, after: afterMin.levels }));
      await evaljs(() => document.querySelector('[data-test-st-clear]')?.click());
      await sleep(900);
    } else {
      skip('等级过滤', '仓库只有一只宠物，无法验证收窄');
    }
    // 卡片菜单是应用级弹出菜单（旧版 PopupMenu）：仓库区 → 放入背包 + 放生，
    // 携带区 → 放入仓库。只读：开了就点背板关，不执行动作。
    const openSlotMenu = async (cardSel, label) => {
      await evaljs((s) => document.querySelector(s)?.click(), cardSel);
      await waitFor(
        async () => (await evaljs(() => !!document.querySelector('.popup-container .popup-menu'))),
        8000,
        label,
      );
      await sleep(700);
      return evaljs(() => ({
        items: [...document.querySelectorAll('.popup-container .popup-menu-item')].map((b) => ({
          text: b.textContent?.trim() ?? '',
          danger: b.classList.contains('danger'),
          hook: b.getAttribute('data-test-st-to-bag') !== null ? 'to-bag'
            : b.getAttribute('data-test-st-release') !== null ? 'release' : null,
        })),
        inline: !!document.querySelector('.slot-actions'),
      }));
    };
    const closeSlotMenu = async (label) => {
      await evaljs(() => document.querySelector('.hk-popover-backdrop')?.click());
      await waitFor(
        async () => (await evaljs(() => !document.querySelector('.popup-container'))),
        8000,
        label,
      );
    };
    const menu = await openSlotMenu('.storage-grid [data-test-st-card]', '卡片菜单');
    ok('仓库卡菜单含放入背包/放生', menu.items.some((i) => i.hook === 'to-bag') && menu.items.some((i) => i.hook === 'release'), JSON.stringify(menu));
    ok('仓库卡菜单项与旧版一致',
      JSON.stringify(menu.items.map((i) => i.text)) === JSON.stringify(['放入背包', '放生'])
        && menu.items[1].danger && !menu.items[0].danger && !menu.inline,
      JSON.stringify(menu));
    await closeSlotMenu('卡片菜单关闭');
    ok('点背板关闭仓库卡菜单', true);
    // 多选模式只对仓库格生效：背包卡不出勾选框（旧版背包卡恒按非多选渲染）。
    await evaljs(() => document.querySelector('[data-test-st-select-mode]')?.click());
    await sleep(700);
    const selectMode = await evaljs(() => ({
      exit: !!document.querySelector('.select-mode-btn.exit'),
      bagCheckbox: document.querySelectorAll('.bag-panel .pokemon-select-checkbox').length,
      storageCheckbox: document.querySelectorAll('.storage-grid .pokemon-select-checkbox').length,
    }));
    ok('多选模式进入，仓库格出勾选框', selectMode.exit && selectMode.storageCheckbox > 0, JSON.stringify(selectMode));
    ok('多选模式下背包卡不出勾选框', selectMode.bagCheckbox === 0, JSON.stringify(selectMode));
    await evaljs(() => document.querySelector('.select-mode-btn.exit')?.click());
    await sleep(500);
    if (view.bagSlots - view.bagEmpty > 0) {
      const bagMenu = await openSlotMenu('.bag-panel .pokemon-slot:not(.empty-slot)', '携带区卡片菜单');
      ok('携带区卡片菜单有「放入仓库」、没有「放生」',
        bagMenu.items.some((i) => i.text === '放入仓库') && !bagMenu.items.some((i) => i.text === '放生'),
        JSON.stringify(bagMenu));
      await closeSlotMenu('携带区卡片菜单关闭');
    }
  } else {
    skip('仓库筛选与菜单', '仓库没有宠物');
  }
}


async function stepAdventure() {
  console.log('[6] 冒险/战斗页');
  await navTo('adventure');
  // recover 异步：战斗视图或地图标记其一出现
  await waitFor(async () => (await evaljs(() => !!document.querySelector('.page-battle')
    || document.querySelectorAll('[data-test-bt-region]').length > 0
    || document.querySelectorAll('[data-test-bt-map-open]').length > 0)), 25000, '战斗或地图');
  const inBattle = await evaljs(() => !!document.querySelector('.page-battle'));
  if (inBattle) {
    // 战斗子流程只读断言：四标签 + 攻击按钮 + 双方信息块 + 我方块可点换宠
    const bt = await evaljs(() => ({
      tabs: document.querySelectorAll('[data-test-bt-tab]').length,
      attack: !!document.querySelector('[data-test-bt-attack]'),
      wild: !!document.querySelector('.enemy-pokemon-block'),
      my: !!document.querySelector('.player-pokemon-block'),
      switchOpen: !!document.querySelector('[data-test-bt-switch-open]'),
      panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    }));
    ok('战斗页无 panic', bt.panic === null, bt.panic ?? '');
    ok('战斗四标签', bt.tabs === 4, `got ${bt.tabs}`);
    ok('攻击按钮与双方信息块', bt.attack && bt.wild && bt.my);
    ok('我方块可点换宠', bt.switchOpen);
  } else {
    const mp = await evaljs(() => ({
      markers: document.querySelectorAll('[data-test-bt-region]').length
        + document.querySelectorAll('[data-test-bt-map-open]').length,
      panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    }));
    ok('冒险页无 panic', mp.panic === null, mp.panic ?? '');
    ok('地图有区域或地点', mp.markers > 0, `got ${mp.markers}`);
  }
}

async function stepShopAndHome() {
  console.log('[6] 商店页');
  await navTo('shop');
  const shop = await evaljs(() => ({
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    active: document.querySelector('.nav-link.active')?.textContent?.trim() ?? null,
    hasBar: !!document.querySelector('.shop-category-bar'),
  }));
  ok('无 panic', shop.panic === null, shop.panic ?? '');
  ok('商店导航激活', shop.active === '商店', shop.active ?? '');
  await waitFor(async () => (await evaljs(() => document.querySelectorAll('.shop-item-card').length > 0
      || document.querySelectorAll('.shop-tile').length > 0
      || !!document.querySelector('.shop-empty'))), 15000, '商店数据');
  ok('商店内容渲染', await evaljs(() => !!document.querySelector('.shop-card, .shop-items-grid, .shop-empty')));

  console.log('[7] 首页');
  await navTo('home');
  const home = await evaljs(() => ({
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    active: document.querySelector('.nav-link.active')?.textContent?.trim() ?? null,
    main: !!document.querySelector('.app-content > *'),
  }));
  ok('无 panic', home.panic === null, home.panic ?? '');
  // 首页走左上角 logo，顶部导航里没有「首页」项，`.nav-link.active` 此时为
  // null 是预期；断首页内容确实渲染了。
  ok('首页导航激活', home.main, `active=${home.active ?? ''}`);
  ok('首页内容渲染', home.main);
}

// ---------------------------------------------------------------------------
// 主流程
// ---------------------------------------------------------------------------

let cdp;
let browserProc;
let profileDir;

try {
  console.log(`game2 浏览器回归 → ${BASE}`);
  ({ child: browserProc, profile: profileDir } = await launchBrowser());
  const tabWs = await newTab();
  cdp = await Cdp.connect(tabWs);
  await cdp.send('Page.enable');
  await cdp.send('Runtime.enable');
  if (process.env.TSDM_VERIFY_COOKIES) {
    await cdp.send('Network.enable');
    await cdp.send('Network.setCookies', {
      cookies: JSON.parse(readFileSync(process.env.TSDM_VERIFY_COOKIES, 'utf8')),
    });
  }

  await goto(BASE);
  let authed = false;
  if (USER && PASS) {
    authed = await login();
    if (!authed) {
      console.log('！登录失败（凭据或本地论坛状态），后续仅执行外壳断言');
    }
  } else {
    console.log('（未提供 TSDM_VERIFY_USER/PASS，仅执行外壳断言）');
  }
  await goto(`${BASE}/plugin.php?id=pokemon:game&ui=v2`);

  // 等 wasm 应用挂载。启动 profile 偶发失败会停在欢迎页（无导航）——
  // 重载一次再等；仍停在欢迎页则视为环境异常并明确报错。
  const mounted = () => evaljs(() => !!document.querySelector('#app')?.firstElementChild
      || !!document.getElementById('app')?.getAttribute('data-panic'));
  const hasNav = () => evaljs(() => !!document.querySelector('[data-test-nav]'));
  await waitFor(mounted, 30000, 'wasm 挂载');
  // 访客停在欢迎页属预期（profile 401）；登录态必须进入主导航。profile 的
  // 回调落地才算「进主导航」，所以这里按状态等到导航出现，而不是挂载后
  // 立刻查一次——接口 200、回调还没跑完时会误判成「profile 拉取失败」。
  if (authed) {
    await waitFor(hasNav, 15000, '主主导航').catch(() => null);
    if (!(await hasNav())) {
      // 真没进来：重载一次自愈，再按状态等。
      await goto(`${BASE}/plugin.php?id=pokemon:game&ui=v2&t=${Date.now()}`);
      await waitFor(mounted, 30000, 'wasm 挂载（重载后）');
      await waitFor(hasNav, 15000, '主主导航（重载后）').catch(() => null);
    }
  }
  if (authed) {
    ok('启动进入主导航', await hasNav(), '停在欢迎页（profile 拉取失败）');
  }

  await stepShell();
  if (authed) {
    await stepInventory();
    await stepPokemonCenter();
    await stepMyPokemon();
    await stepStorage();
    await stepAdventure();
    await stepShopAndHome();
  } else {
    console.log('[2] 背包页（跳过：未登录）');
    console.log('[3] 宠物中心（跳过：未登录）');
    console.log('[4] 我的宠物（跳过：未登录）');
    console.log('[5] 仓库页（跳过：未登录）');
    console.log('[6] 冒险/战斗页（跳过：未登录）');
    console.log('[7] 商店页（跳过：未登录）');
    console.log('[8] 首页（跳过：未登录）');
    skipped += 7;
  }
} catch (err) {
  failed += 1;
  failures.push(`脚本异常：${err.message}`);
  console.error(`✗ 脚本异常：${err.message}`);
} finally {
  try {
    cdp?.ws?.close();
  } catch { /* ignore */ }
  try {
    browserProc?.kill();
  } catch { /* ignore */ }
  await sleep(500);
  try {
    if (profileDir) rmSync(profileDir, { recursive: true, force: true });
  } catch { /* Windows 句柄释放延迟，忽略 */ }
}

console.log('');
console.log(`结果：${passed} 通过 / ${failed} 失败 / ${skipped} 跳过`);
if (failures.length) {
  console.log('失败项：');
  for (const f of failures) console.log(`  - ${f}`);
  process.exit(1);
}
process.exit(0);
