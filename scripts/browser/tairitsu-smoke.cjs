// tairitsu 前端（wasm2/game2）跨引擎冒烟：chromium / firefox / webkit。
//
// readiness 前置条件 3（多浏览器抽测）的无人值守实现：verify_game2.mjs 深度
// 绑定 CDP（Firefox 无 CDP，Safari 无 Windows 版），本套件改走 Playwright，
// 引擎由 PM_BROWSER 环境变量选择；webkit 与 Safari 同源于 WebCore，作为
// Safari 兼容性的代理信号。
//
// 断言刻意保持"启动级"：boot 完成（data-booting 移除）、首屏渲染
// （.app-layout）、调度器活着（__tairitsuFrames.ran > 0）、无 panic。
// 功能级深断言由 chromium 的 verify_game2.mjs / npm test 覆盖。
//
// 用法：
//   node scripts/browser/tairitsu-smoke.cjs              # chromium 基线
//   PM_BROWSER=firefox node scripts/browser/tairitsu-smoke.cjs
//   PM_BROWSER=webkit  node scripts/browser/tairitsu-smoke.cjs
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const http = require('node:http');
const path = require('node:path');
const { chromium, firefox, webkit } = require('@playwright/test');

const ENGINES = { chromium, firefox, webkit };
// 引擎二选一来源：argv（npm run 跨平台友好）优先于 PM_BROWSER 环境变量。
const engineName = process.argv[2] || process.env.PM_BROWSER || 'chromium';
const engine = ENGINES[engineName];
if (!engine) throw new Error(`未知引擎 PM_BROWSER=${engineName}（可选 chromium/firefox/webkit）`);

const wasmDirectory = path.resolve(__dirname, '../../plugin/wasm2');
const prefix = '/source/plugin/pokemon/wasm2/';
const artifacts = path.join(__dirname, 'artifacts/tairitsu-smoke');
const port = 18923 + ['chromium', 'firefox', 'webkit'].indexOf(engineName);

// MIME 与 Caddy 语义对齐；.d.ts 仅 tsc 用，text/plain 即可。
const MIME = {
  '.wasm': 'application/wasm', '.js': 'text/javascript', '.mjs': 'text/javascript',
  '.json': 'application/json', '.html': 'text/html', '.d.ts': 'text/plain',
};

// game2 的 API 面：/plugin.php?id=pokemon:pokemon&endpoint=...&action=...
// 冒烟只要求"合法 JSON + 不 500"，字段级对齐交给 verify_game2 的活栈回归。
function mockApi(url) {
  const endpoint = (url.searchParams.get('endpoint') || '').toLowerCase();
  const action = (url.searchParams.get('action') || '').toLowerCase();
  if (endpoint === 'topics') return { success: true, data: { topics: [], total: 0 } };
  if (endpoint === 'user') return { success: true, data: { logged_in: false } };
  if (endpoint === 'pokemon') return { success: true, data: [] };
  if (endpoint === 'shop') return { success: true, data: { items: [], money: 0 } };
  return { success: true, data: null, endpoint, action };
}

async function startServer() {
  const server = http.createServer(async (req, res) => {
    const url = new URL(req.url, 'http://127.0.0.1');
    try {
      if (url.pathname === '/plugin.php') {
        res.writeHead(200, { 'Content-Type': 'application/json; charset=utf-8' });
        res.end(JSON.stringify(mockApi(url)));
        return;
      }
      if (url.pathname.startsWith(prefix)) {
        const rel = url.pathname.slice(prefix.length).split('?')[0];
        const file = path.join(wasmDirectory, rel);
        if (!file.startsWith(wasmDirectory) || !(await fs.stat(file).catch(() => null))?.isFile()) {
          res.writeHead(404); res.end('not found'); return;
        }
        res.writeHead(200, {
          'Content-Type': MIME[path.extname(file)] || 'application/octet-stream',
          'Cache-Control': 'no-cache',
        });
        res.end(await fs.readFile(file));
        return;
      }
      // 页面骨架复刻 game_v2.inc.php：#app[data-booting] + boot.js + 启动脚本。
      const page = `<!doctype html><html lang="zh-CN"><meta charset="utf-8">
<title>tairitsu smoke</title><div id="app" data-booting></div>
<script src="${prefix}boot.js"></script>
<script type="module">
(function() {
  var app = document.getElementById('app');
  function fail(err) {
    var msg = String(err && err.stack ? err.stack : err);
    app.innerHTML = '<p style="color:#c00">加载失败</p><p style="white-space:pre-wrap">' + msg.replace(/</g, '&lt;') + '</p>';
    if (window.console) console.error('[tairitsu] boot failed', err);
  }
  if (typeof window.__tairitsuBoot !== 'function') { fail(new Error('boot.js 未执行')); return; }
  window.__tairitsuBoot().catch(fail);
})();
</script></html>`;
      res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
      res.end(page);
    } catch (err) {
      res.writeHead(500); res.end(String(err));
    }
  });
  await new Promise((resolve) => server.listen(port, '127.0.0.1', resolve));
  return server;
}

async function main() {
  const server = await startServer();
  const browser = await engine.launch({ headless: true });
  const context = await browser.newContext();
  const page = await context.newPage();
  const consoleErrors = [];
  page.on('console', (msg) => {
    if (msg.type() === 'error') consoleErrors.push(msg.text());
  });
  page.on('pageerror', (err) => consoleErrors.push(String(err)));

  let verdict = 'PASS';
  try {
    await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: 'load', timeout: 60000 });
    // 1) boot 完成：data-booting 被移除（boot.js:115 在 lifecycle.start 后执行）。
    await page.waitForSelector('#app:not([data-booting])', { timeout: 90000 });
    // 2) 首屏渲染：登录外壳（.app-layout）或未初始化欢迎页（.welcome-page）其一。
    await page.waitForSelector('.app-layout, .welcome-page', { timeout: 30000 });
    // 3) 调度器活着：boot.js 的帧计数必须推进过。
    const frames = await page.evaluate(() => window.__tairitsuFrames || { ran: 0 });
    assert.ok(frames.ran > 0, `调度器未出帧 __tairitsuFrames=${JSON.stringify(frames)}`);
    // 4) 无 panic / boot 失败。资源 404 类噪音不计入（mock API 面故意不全）。
    const fatal = consoleErrors.filter((t) => /panicked at|boot failed|WebAssembly/i.test(t));
    assert.deepStrictEqual(fatal, [], `console 致命错误: ${JSON.stringify(fatal)}`);
    console.log(`[${engineName}] boot OK · 首屏渲染 · frames.ran=${frames.ran} · 无 panic`);
  } catch (err) {
    verdict = 'FAIL';
    console.error(`[${engineName}] FAIL: ${err.message.split('\n')[0]}`);
    await fs.mkdir(artifacts, { recursive: true });
    await page.screenshot({ path: path.join(artifacts, `${engineName}-fail.png`), fullPage: true }).catch(() => {});
    await fs.writeFile(path.join(artifacts, `${engineName}-console.log`), consoleErrors.join('\n')).catch(() => {});
    process.exitCode = 1;
  } finally {
    await browser.close();
    server.close();
  }
  if (verdict === 'PASS') console.log(`[${engineName}] PASS`);
}

main();
