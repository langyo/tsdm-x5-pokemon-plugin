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
//
// 凭据只从环境变量读取（源码不含任何字面量口令）。未提供凭据时跳过
// 登录，仅执行无需会话的「外壳」断言并以 SKIPPED 报告其余步骤。
//
// 断言范围刻意保持只读/不落库：不购买、不消耗道具；「使用」仅覆盖
// 纯客户端分支（装备道具提示）与只读弹窗（get_usable_pokemon）。

import { spawn } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
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
  // 新版 Chrome/Edge 要求 PUT；旧版 GET。
  let target;
  try {
    const r = await fetch(`${base}/json/new?about:blank`, { method: 'PUT' });
    target = await r.json();
  } catch {
    const r = await fetch(`${base}/json/new?about:blank`);
    target = await r.json();
  }
  return target.webSocketDebuggerUrl;
}

async function goto(url) {
  const loaded = cdp.waitEvent('Page.loadEventFired', 30000).catch(() => null);
  await cdp.send('Page.navigate', { url });
  await loaded;
  await sleep(600);
}

async function isLoggedIn() {
  return evaljs(() => !!document.querySelector('a[href*="action=logout"]'));
}

async function login() {
  if (!USER || !PASS) return false;
  await goto(`${BASE}/member.php?mod=logging&action=login`);
  const submitted = await evaljs((u, p) => {
    const form =
      document.querySelector('form[name="login"]') ??
      [...document.forms].find((f) => f.querySelector('input[type="password"]'));
    if (!form) return 'no-form';
    const un = form.querySelector('input[name="username"], input[name="username_ll"]');
    const pw = form.querySelector('input[type="password"]');
    if (!un || !pw) return 'no-fields';
    un.value = u;
    pw.value = p;
    form.submit();
    return 'submitted';
  }, USER, PASS);
  if (submitted !== 'submitted') {
    throw new Error(`登录表单不可用：${submitted}`);
  }
  await sleep(1500);
  return waitFor(isLoggedIn, 10000, 'login cookie');
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
  useId: c.querySelector('[data-pm-inv-use]')?.getAttribute('data-pm-inv-use') ?? null,
})));

const navTo = async (route) => {
  await evaljs((r) => {
    document.querySelector(`button[data-pm-nav="${r}"]`)?.click();
  }, route);
  await sleep(1600);
};

async function stepShell() {
  console.log('[1] 应用外壳');
  const shell = await evaljs(() => ({
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    hasLayout: !!document.querySelector('.app-layout') || (document.querySelector('#app')?.childElementCount ?? 0) > 0,
    navs: [...document.querySelectorAll('[data-pm-nav]')].map((b) => b.textContent?.trim()),
  }));
  ok('无 panic', shell.panic === null, `data-panic=${shell.panic}`);
  ok('渲染出页面骨架', shell.hasLayout);
  if (shell.navs.length === 0) {
    // 未初始化/访客停在欢迎页（整屏、无导航）——外壳成立，导航断言跳过。
    skip('导航含 首页/商店/背包', '当前停在欢迎页（无导航属预期）');
  } else {
    ok(
      '导航含 首页/商店/背包',
      ['首页', '商店', '背包'].every((n) => shell.navs.includes(n)),
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
    cats: document.querySelectorAll('[data-pm-inv-cat]').length,
  }));
  ok('无 panic', boot.panic === null, boot.panic ?? '');
  ok('背包导航激活', boot.active === '背包', boot.active ?? '');
  await waitFor(async () => (await evaljs(() => document.querySelectorAll('.shop-item-card').length > 0)), 15000, '物品列表');
  ok('分类按钮 6 个', boot.cats === 6, `got ${boot.cats}`);

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
  await evaljs(() => document.querySelector('[data-pm-inv-cat="1"]').click());
  await sleep(1600);
  const cat = await evaljs(() => ({
    active: document.querySelector('[data-pm-inv-cat].active')?.getAttribute('data-pm-inv-cat'),
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
      const input = document.querySelector('[data-pm-inv-search]');
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
    await evaljs(() => document.querySelector('[data-pm-inv-search-clear]').click());
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
  await evaljs(() => document.querySelector('[data-pm-inv-cat="all"]').click());
  await waitFor(async () => (await evaljs(() => document.querySelectorAll('.shop-item-card').length)) === before.length, 8000, '回到全部分类');
  const equip = before.find((c) => c.badges.includes('装备道具') && c.useId);
  if (equip) {
    await evaljs((id) => {
      document.querySelector(`[data-pm-inv-use="${id}"]`)?.click();
    }, equip.useId);
    await waitFor(async () => (await evaljs(() => !!document.querySelector('.equipment-tab'))), 10000, '装备页跳转');
    const jump = await evaljs(() => ({
      activeNav: document.querySelector('.nav-link.active')?.textContent?.trim() ?? null,
      tabActive: document.querySelector('[data-pm-my-pm-tab="equipment"]')?.className ?? '',
    }));
    ok('装备道具跳转我的宠物', jump.activeNav === '我的宠物', jump.activeNav);
    ok('落在装备标签页', (jump.tabActive ?? '').includes('active'), jump.tabActive);
    // 回背包继续后续断言
    await evaljs(() => document.querySelector('button[data-pm-nav="inventory"]')?.click());
    await waitFor(async () => (await evaljs(() => document.querySelectorAll('.shop-item-card').length)) === before.length, 10000, '回到背包');
  } else {
    skip('装备道具跳转', '数据中没有装备道具');
  }

  // 宝可梦目标弹窗：只读（get_usable_pokemon）→ 关闭
  const heal = before.find((c) => c.badges.includes('回复药') && c.useId);
  if (heal) {
    await evaljs((id) => {
      document.querySelector(`[data-pm-inv-use="${id}"]`)?.click();
    }, heal.useId);
    await waitFor(async () => (await evaljs(() => !!document.querySelector('.modal-overlay'))), 8000, '目标弹窗');
    const modal = await evaljs(() => ({
      title: document.querySelector('.modal-header h3')?.textContent?.trim() ?? null,
      picks: document.querySelectorAll('[data-pm-inv-pick]').length,
      body: document.querySelector('.modal-body')?.textContent?.trim() ?? null,
    }));
    ok('目标弹窗标题含物品名', (modal.title ?? '').includes(heal.name), modal.title ?? '');
    ok(
      '弹窗给出目标列表或空态文案',
      modal.picks > 0 || (modal.body ?? '').length > 0,
      `picks=${modal.picks}, body=${modal.body}`,
    );
    await evaljs(() => document.querySelector('[data-pm-inv-modal-close]')?.click());
    await sleep(900);
    ok('弹窗关闭', await evaljs(() => !document.querySelector('.modal-overlay')));
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
    healAll: !!document.querySelector('[data-pm-center-heal-all]'),
    first: (() => {
      const c = document.querySelector('.heal-pokemon-card');
      if (!c) return null;
      return {
        name: c.querySelector('.heal-pm-name')?.textContent?.trim() ?? null,
        hp: c.querySelector('.hp-text')?.textContent?.trim() ?? null,
        state: c.querySelector('.state-label')?.textContent?.trim() ?? null,
        healId: c.querySelector('[data-pm-center-heal]')?.getAttribute('data-pm-center-heal') ?? null,
        flee: c.querySelector('[data-pm-center-flee]')?.getAttribute('data-pm-center-flee') ?? null,
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
  ok('我的宠物导航激活', boot.active === '我的宠物', boot.active ?? '');
  ok('渲染我的宠物卡片', boot.hasPage);

  // 先切回属性页——背包步骤的装备跳转可能把标签页留在「装备」。
  await evaljs(() => document.querySelector('[data-pm-my-pm-tab="stats"]')?.click());
  await waitFor(
    async () => (await evaljs(() => !!document.querySelector('.stats-tab-layout'))),
    15000,
    '属性面板',
  );
  const view = await evaljs(() => {
    const radar = document.querySelector('.radar-svg');
    return {
      empty: !!document.querySelector('.empty-hint'),
      hasStats: !!document.querySelector('.stats-tab-layout'),
      name: document.querySelector('.pokemon-name')?.textContent?.trim() ?? null,
      level: document.querySelector('.heal-pm-level')?.textContent?.trim() ?? null,
      hpBar: !!document.querySelector('.hp-bar .bar-text'),
      expBar: !!document.querySelector('.exp-bar .bar-text'),
      ivChips: document.querySelectorAll('.iv-chip').length,
      qualityBadge: document.querySelector('.quality-badge')?.textContent?.trim() ?? null,
      radarPaths: radar ? radar.querySelectorAll('path').length : 0,
      editBtn: !!document.querySelector('[data-pm-my-pm-nick-edit]'),
      selectorChips: document.querySelectorAll('[data-pm-my-pm-select]').length,
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
  ok('昵称编辑入口存在', view.editBtn);

  // 昵称编辑：进入编辑态 → 取消（不写库）
  await evaljs(() => document.querySelector('[data-pm-my-pm-nick-edit]')?.click());
  await waitFor(
    async () => (await evaljs(() => !!document.querySelector('[data-pm-my-pm-nick-input]'))),
    8000,
    '昵称编辑框',
  );
  const editing = await evaljs(() => ({
    inputValue: document.querySelector('[data-pm-my-pm-nick-input]')?.value ?? null,
    hasSave: !!document.querySelector('[data-pm-my-pm-nick-save]'),
    hasCancel: !!document.querySelector('[data-pm-my-pm-nick-cancel]'),
  }));
  ok('编辑框带当前昵称', !!editing.inputValue, editing.inputValue ?? '');
  ok('编辑框有保存/取消', editing.hasSave && editing.hasCancel);
  await evaljs(() => document.querySelector('[data-pm-my-pm-nick-cancel]')?.click());
  await sleep(700);
  ok('取消后退出编辑态',
    await evaljs(() => !document.querySelector('[data-pm-my-pm-nick-input]')));

  // 装备标签页（只读断言：渲染与提示格式；真实装载/卸下属写操作，另行验收）
  await evaljs(() => document.querySelector('[data-pm-my-pm-tab="equipment"]')?.click());
  await waitFor(
    async () => (await evaljs(() => !!document.querySelector('.equipment-tab'))),
    15000,
    '装备页',
  );
  const eqView = await evaljs(() => ({
    slots: document.querySelectorAll('.equip-slot').length,
    invItems: document.querySelectorAll('[data-pm-eq-inv-item]').length,
    invTitle: document.querySelector('[data-pm-eq-inv-item]')?.getAttribute('title') ?? null,
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
  }));
  ok('装备页无 panic', eqView.panic === null, eqView.panic ?? '');
  ok('装备槽恒定 4 个', eqView.slots === 4, `got ${eqView.slots}`);
  if (eqView.invItems > 0) {
    ok('装备卡悬浮提示为加成格式', /(HP|攻击|防御|特攻|特防|速度)\s*[+-]\d+/.test(eqView.invTitle ?? ''),
      eqView.invTitle ?? '');
  } else {
    skip('装备卡细节', '没有可装备物品');
  }

  // 技能标签页（只读断言：卡片渲染 + 选中展开遗忘 + 弹窗列表 + 关闭）
  await evaljs(() => document.querySelector('[data-pm-my-pm-tab="skills"]')?.click());
  await waitFor(async () => (await evaljs(() => !!document.querySelector('.skills-grid'))), 15000, '技能页');
  const skView = await evaljs(() => ({
    cards: document.querySelectorAll('[data-pm-skill-card]').length,
    emptySlots: document.querySelectorAll('[data-pm-skill-learn-slot]').length,
    totalSlots: document.querySelectorAll('.skill-card-v2, [data-pm-skill-learn-slot]').length,
    pp: document.querySelector('.skill-card-v2 .skill-pp')?.textContent?.trim() ?? null,
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
  }));
  ok('技能页无 panic', skView.panic === null, skView.panic ?? '');
  ok('技能槽恒定 4 个（卡 + 空槽）', skView.totalSlots === 4,
    `cards=${skView.cards}, empty=${skView.emptySlots}`);
  if (skView.cards > 0) {
    ok('技能卡带 PP', /^PP\s*\d+\/\d+/.test(skView.pp ?? ''), skView.pp ?? '');
    // 选中卡片展开遗忘按钮，再点一次收起（不执行遗忘）
    await evaljs(() => document.querySelector('[data-pm-skill-card]')?.click());
    await waitFor(async () => (await evaljs(() => !!document.querySelector('[data-pm-skill-forget]'))), 8000, '遗忘按钮');
    ok('选中技能卡展开遗忘操作',
      await evaljs(() => !!document.querySelector('.skill-card-v2.selected')));
    await evaljs(() => document.querySelector('[data-pm-skill-card]')?.click());
    await sleep(500);
  } else {
    skip('技能卡细节', '宠物没有已学技能');
  }
  if (skView.emptySlots > 0) {
    await evaljs(() => document.querySelector('[data-pm-skill-learn-slot]')?.click());
    await waitFor(async () => (await evaljs(() => !!document.querySelector('.modal-overlay'))), 10000, '学习弹窗');
    await waitFor(async () => (await evaljs(() => !document.querySelector('.skill-learn-loading'))), 15000, '技能列表加载');
    const modal = await evaljs(() => ({
      title: document.querySelector('.modal-header h3')?.textContent?.trim() ?? null,
      sections: [...document.querySelectorAll('.skill-section-title')].map((s) => s.textContent?.trim()),
      bodyHasContent: (document.querySelector('.modal-body')?.textContent ?? '').length > 0,
    }));
    ok('学习弹窗标题带等级', /^学习新技能 \(Lv\.\d+\)$/.test(modal.title ?? ''), modal.title ?? '');
    ok('学习弹窗给出列表或空态', modal.bodyHasContent);
    await evaljs(() => document.querySelector('[data-pm-learn-close]')?.click());
    await sleep(700);
    ok('学习弹窗关闭', await evaljs(() => !document.querySelector('.modal-overlay')));
  } else {
    skip('学习弹窗', '没有空技能槽');
  }
  // 回属性页（保持后续步骤的页面状态干净）
  await evaljs(() => document.querySelector('[data-pm-my-pm-tab="stats"]')?.click());
  await sleep(600);
}

async function stepShopAndHome() {
  console.log('[5] 商店页');
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

  console.log('[6] 首页');
  await navTo('home');
  const home = await evaljs(() => ({
    panic: document.getElementById('app')?.getAttribute('data-panic') ?? null,
    active: document.querySelector('.nav-link.active')?.textContent?.trim() ?? null,
    main: !!document.querySelector('.app-content > *'),
  }));
  ok('无 panic', home.panic === null, home.panic ?? '');
  ok('首页导航激活', home.active === '首页', home.active ?? '');
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

  // 等 wasm 应用挂载
  await waitFor(async () => (await evaljs(() => !!document.querySelector('#app')?.firstElementChild
      || !!document.getElementById('app')?.getAttribute('data-panic'))), 30000, 'wasm 挂载');

  await stepShell();
  if (authed) {
    await stepInventory();
    await stepPokemonCenter();
    await stepMyPokemon();
    await stepShopAndHome();
  } else {
    console.log('[2] 背包页（跳过：未登录）');
    console.log('[3] 宠物中心（跳过：未登录）');
    console.log('[4] 我的宠物（跳过：未登录）');
    console.log('[5] 商店页（跳过：未登录）');
    console.log('[6] 首页（跳过：未登录）');
    skipped += 5;
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
