// Real game WASM -> PHP CGI -> an isolated MariaDB database. No API JSON stubs.
const assert = require('node:assert/strict');
const {execFile} = require('node:child_process');
const {randomBytes} = require('node:crypto');
const fs = require('node:fs/promises');
const http = require('node:http');
const path = require('node:path');
const {promisify} = require('node:util');
const {chromium, expect: baseExpect} = require('@playwright/test');
const execute = promisify(execFile);
const expect = baseExpect.configure({timeout: 12000});
const root = path.resolve(__dirname, '../..');
const wasmDirectory = process.env.GAME_WASM_DIRECTORY || path.join(root, 'plugin/wasm');
const artifacts = path.join(__dirname, 'artifacts/live-player');
const prefix = '/source/plugin/pokemon/wasm/';
const pixel = Buffer.from('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7', 'base64');
// Ignore any caller-supplied TSDM_TEST_DB: this run owns one fresh random database.
const database = `tsdm_test_browser_${randomBytes(16).toString('hex')}`;
const environment = {...process.env, TSDM_TEST_DB: database, TSDM_TEST_OWNER: randomBytes(16).toString('hex'),
  TSDM_TEST_UID: '7', TSDM_TEST_READY: '', TSDM_TEST_FAIL_SQL: '', TSDM_TEST_FAIL_TYPE: '',
  TSDM_TEST_LEGACY_PROFILE: ''};
const phpArgs = [
  ...(process.env.TSDM_PHP_INI ? ['-c', process.env.TSDM_PHP_INI] : []),
  '-d', 'display_errors=0', '-d', 'log_errors=1', '-d', 'error_log=',
  '-d', 'opcache.jit=0', '-d', 'opcache.jit_buffer_size=0',
];
const requests = [];
const errors = [];
const activeCgi = new Set();
let assertions = 0;

function check(condition, message) {
  assert(condition, message);
  assertions++;
  console.log(`PASS ${message}`);
}

async function fixture(operation) {
  const result = await execute(process.env.TSDM_PHP_CLI || 'php',
    [...phpArgs, path.join(root, 'scripts/test/support/browser_database.php'), operation],
    {env: environment, timeout: 25000, maxBuffer: 8 * 1024 * 1024});
  assert.equal(result.stderr, '', 'PHP fixture must not emit warnings');
  return JSON.parse(result.stdout);
}

async function cgi(request, url, body) {
  const endpoint = url.searchParams.get('endpoint');
  const operation = execute(process.env.TSDM_PHP_CGI || 'php-cgi', phpArgs, {
    env: {...environment, TSDM_TEST_ENDPOINT: endpoint, REDIRECT_STATUS: '200',
      GATEWAY_INTERFACE: 'CGI/1.1', REQUEST_METHOD: request.method,
      SCRIPT_FILENAME: path.join(root, 'scripts/test/support/live_api.php'),
      SCRIPT_NAME: '/live_api.php', QUERY_STRING: url.searchParams.toString(),
      REQUEST_URI: request.url, SERVER_PROTOCOL: 'HTTP/1.1', SERVER_NAME: '127.0.0.1',
      CONTENT_TYPE: request.headers['content-type'] || '', CONTENT_LENGTH: String(body.length),
      HTTP_X_PM_FORMHASH: request.headers['x-pm-formhash'] || ''},
    encoding: 'buffer', timeout: 25000, maxBuffer: 8 * 1024 * 1024,
  });
  activeCgi.add(operation);
  operation.child.stdin.end(body);
  let result;
  try { result = await operation; } finally { activeCgi.delete(operation); }
  assert.equal(result.stderr.toString(), '', 'PHP CGI must not emit warnings');
  const boundary = result.stdout.indexOf('\r\n\r\n');
  assert(boundary >= 0, 'PHP CGI must return headers');
  const headers = {};
  let status = 200;
  for (const line of result.stdout.subarray(0, boundary).toString().split('\r\n')) {
    const colon = line.indexOf(':');
    assert(colon > 0, `Invalid CGI header: ${line}`);
    const key = line.slice(0, colon).toLowerCase();
    const value = line.slice(colon + 1).trim();
    if (key === 'status') status = Number(value.split(' ')[0]);
    else headers[key] = value;
  }
  const output = result.stdout.subarray(boundary + 4);
  const record = {endpoint, action: url.searchParams.get('action'), method: request.method,
    input: body.length ? JSON.parse(body.toString()) : null, status};
  if (endpoint !== 'avatar') {
    record.result = JSON.parse(output.toString());
    // Empty recovery is an expected response before the first battle.
    if (!record.result.success && !(endpoint === 'battle' && record.action === 'recover' && status === 404)) {
      errors.push(`API ${endpoint}/${record.action}: ${output.toString()}`);
    }
    assert.equal(request.headers['x-pm-formhash'], 'test-formhash', 'The page must supply the real formhash header');
  }
  requests.push(record);
  return {status, headers, output};
}

async function serve() {
  // Reuse the production page's fetch wrapper so PHP validates a browser-sent header.
  const template = await fs.readFile(path.join(root, 'plugin/game.inc.php'), 'utf8');
  const wrapper = template.match(/\(function\(\) \{\s+const _fetch = window\.fetch;[\s\S]+?\}\)\(\);/);
  assert(wrapper, 'Production formhash wrapper is present');
  const script = wrapper[0].replace('<?php echo FORMHASH; ?>', 'test-formhash');
  assert(!script.includes('<?php'), 'Only the test formhash is substituted');
  const html = `<!doctype html><html><meta charset="utf-8"><link rel="stylesheet" href="${prefix}game.css"><div id="main"></div><script>${script}</script><script type="module">import init,{WebHandle} from '${prefix}_game.js';await init({module_or_path:'${prefix}_game_bg.wasm'});await new WebHandle().start();</script></html>`;
  const server = http.createServer(async (request, response) => {
    try {
      const url = new URL(request.url, 'http://127.0.0.1');
      if (url.pathname === '/plugin.php' && url.searchParams.has('endpoint')) {
        const chunks = [];
        for await (const chunk of request) chunks.push(chunk);
        const result = await cgi(request, url, Buffer.concat(chunks));
        response.writeHead(result.status, result.headers);
        response.end(result.output);
      } else if (url.pathname.startsWith(prefix)) {
        const filename = path.resolve(wasmDirectory, decodeURIComponent(url.pathname.slice(prefix.length)));
        assert(filename.startsWith(path.resolve(wasmDirectory) + path.sep));
        const data = await fs.readFile(filename);
        response.writeHead(200, {'Content-Type': {'.js': 'text/javascript', '.wasm': 'application/wasm', '.css': 'text/css'}[path.extname(filename)] || 'application/octet-stream'});
        response.end(data);
      } else if (url.pathname.includes('/images/')) {
        // Cosmetic images only; no API response is synthesized by this server.
        response.writeHead(200, {'Content-Type': 'image/gif'});
        response.end(pixel);
      } else if (url.pathname === '/plugin.php' || url.pathname === '/') {
        response.writeHead(200, {'Content-Type': 'text/html; charset=utf-8'});
        response.end(html);
      } else {
        throw Error(`Unexpected local path ${url.pathname}`);
      }
    } catch (error) {
      errors.push(String(error));
      if (!response.headersSent) response.writeHead(500, {'Content-Type': 'text/plain'});
      response.end('Test proxy failed');
    }
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  return {url: `http://127.0.0.1:${server.address().port}/plugin.php?id=pokemon:pokemon`,
    close: async () => {
      server.closeAllConnections();
      await new Promise(resolve => server.close(resolve));
      // Finish in-flight requests before dropping their database, including on failure.
      await Promise.allSettled([...activeCgi]);
    }};
}

function responseFor(page, endpoint, action) {
  const pending = page.waitForResponse(response => {
    const url = new URL(response.url());
    return url.searchParams.get('endpoint') === endpoint && url.searchParams.get('action') === action;
  }).then(async response => {
    const result = await response.json();
    assert(result.success, JSON.stringify(result));
    return result.data;
  });
  // A failed click may leave the response wait pending while finally cleans up.
  pending.catch(() => {});
  return pending;
}

async function run() {
  await fs.mkdir(artifacts, {recursive: true});
  let app, browser, context, page;
  try {
    await fixture('create');
    app = await serve();
    browser = await chromium.launch({headless: true});
    context = await browser.newContext({viewport: {width: 1440, height: 1100}});
    context.setDefaultTimeout(12000);
    context.setDefaultNavigationTimeout(20000);
    await context.route('**/*', route => {
      if (new URL(route.request().url()).origin === new URL(app.url).origin) return route.continue();
      if (route.request().resourceType() === 'image') return route.fulfill({status: 200, contentType: 'image/gif', body: pixel});
      errors.push(`Unexpected external request ${route.request().url()}`);
      return route.abort();
    });
    await context.tracing.start({screenshots: true, snapshots: true});
    page = await context.newPage();
    page.on('pageerror', error => errors.push(String(error)));
    await page.goto(app.url);

    await page.getByText('商店', {exact: true}).click();
    const purchase = responseFor(page, 'shop', 'buy');
    await page.locator('.shop-tile').filter({hasText: '整合测试药水'}).locator('.buy-btn').click();
    check((await purchase).remaining_money === 40, 'WASM purchase receives PHP committed balance');
    await expect(page.locator('.app-sidebar')).toContainText('40');
    await page.getByTitle('查看背包', {exact: true}).click();
    await expect(page.locator('[data-item-id="24"]')).toContainText('整合测试药水');
    await expect(page.locator('[data-item-id="24"] .item-meta')).toContainText('×1');
    const bought = await fixture('snapshot');
    check(Number(bought.user.money) === 40 && bought.inventory.length === 1
      && Number(bought.inventory[0].itemid) === 24 && Number(bought.inventory[0].nums) === 1,
    'Browser inventory agrees with real money debit and one purchased item');
    await page.screenshot({path: path.join(artifacts, 'purchased-inventory.png'), fullPage: true});

    const targetsResponse = responseFor(page, 'user', 'get_usable_pokemon');
    await page.locator('[data-item-id="24"] .buy-btn').click();
    const targets = await targetsResponse;
    check(targets.usable_pokemon.length === 1 && targets.usable_pokemon[0].id === 501,
      'The real item eligibility endpoint offers the owned Pokemon instance');
    const useResponse = responseFor(page, 'user', 'use_item');
    await page.locator('.pokemon-select-item').evaluate(node => { node.click(); node.click(); node.click(); });
    await useResponse;
    await expect(page.locator('[data-item-id="24"]')).toHaveCount(0);
    const healed = await fixture('snapshot');
    check(healed.pokemon_hp === bought.pokemon_hp + 20 && Number(healed.user.money) === 40
      && healed.inventory.reduce((count, item) => count + Number(item.nums), 0) === 0,
    'Using the purchased medicine commits HP restoration and consumes exactly one item');
    await page.locator('.pm-mini-slot').first().hover();
    await expect(page.locator('.stat-hp-bar .stat-value-inside')).toHaveText(`${healed.pokemon_hp}/${targets.usable_pokemon[0].max_hp}`);
    await expect(page.locator('.toast-item.error')).toHaveCount(0);
    check(requests.filter(request => request.endpoint === 'user' && request.action === 'use_item').length === 1
      && requests.filter(request => request.endpoint === 'user' && request.action === 'get_usable_pokemon').length === 1,
    'Repeated target clicks send one real mutation and the depleted item is not queried again');

    await page.getByText('野外冒险', {exact: true}).click();
    await page.locator('.map-marker').click();
    await page.getByRole('button', {name: '查看详情', exact: true}).click();
    const startResponse = responseFor(page, 'battle', 'start');
    await page.getByRole('button', {name: '开始冒险', exact: true}).click();
    const started = await startResponse;
    await expect(page.locator('.page-battle')).toBeVisible();
    check(started.engine_battle_id > 0 && started.revision === 1 && started.my_pokemon.instance_id === 501,
      'Battle start preserves engine and owned Pokemon IDs across PHP and WASM');
    const turnResponse = responseFor(page, 'battle', 'turn');
    await page.locator('.skill-btn').filter({hasText: '整合测试招式'}).click();
    const turn = await turnResponse;
    await expect(page.locator('.turn-number')).toHaveText(String(turn.turn));
    await expect(page.locator('.skill-btn .skill-pp')).toContainText('4/20');
    const fought = await fixture('snapshot');
    check(fought.battles.length === 1 && fought.skill_pp === 4
      && Number(fought.battles[0].revision) === 2 && fought.battles[0].phase === 'active',
    'One browser turn commits exactly one revision and one PP decrement');
    const sent = requests.filter(request => request.endpoint === 'battle' && ['start', 'turn'].includes(request.action));
    check(sent.length === 2 && sent[1].input.skill_id === 12
      && sent[1].input.engine_battle_id === started.engine_battle_id && sent[1].input.expected_revision === 1
      && fought.receipts.length === 2 && sent.every(request => fought.receipts.some(receipt =>
        receipt.request_id === request.input.request_id && JSON.parse(receipt.response_json).success)),
    'Browser request IDs and revision metadata match real committed receipts');

    const recovery = responseFor(page, 'battle', 'recover');
    await page.reload();
    const recovered = await recovery;
    await expect(page.locator('.page-battle')).toBeVisible();
    await expect(page.locator('.turn-number')).toHaveText(String(turn.turn));
    await expect(page.locator('.skill-btn .skill-pp')).toContainText('4/20');
    check(recovered.engine_battle_id === turn.engine_battle_id && recovered.revision === turn.revision
      && recovered.my_pokemon.hp === turn.my_pokemon.hp && recovered.wild_pokemon.hp === turn.wild_pokemon.hp,
    'Reload restores the same committed battle and HP through the real recover endpoint');
    const reportResponse = responseFor(page, 'battle', 'battle_log');
    await page.getByTestId('battle-log-open').click();
    const report = await reportResponse;
    // Textarea DOM values normalize line endings, including PHP source CRLF on Windows.
    await expect(page.getByTestId('battle-share-text')).toHaveValue(report.bbcode.replace(/\r\n?/g, '\n'));
    const after = await fixture('snapshot');
    check(report.battle_id === turn.engine_battle_id && report.events.length === after.events.length
      && report.events.length > 0 && after.skill_pp === 4 && after.receipts.length === 2,
    'Rendered battle report matches stored events; reload and log reads do not replay the turn');
    await page.screenshot({path: path.join(artifacts, 'recovered-battle-report.png'), fullPage: true});
    assert.deepEqual(errors, []);
  } catch (error) {
    if (page) await page.screenshot({path: path.join(artifacts, 'failure.png'), fullPage: true}).catch(() => {});
    console.error('PHP/browser errors:', errors);
    throw error;
  } finally {
    try {
      if (context) await context.tracing.stop({path: path.join(artifacts, 'trace.zip')}).catch(() => {});
    } finally {
      try {
        if (browser) await browser.close();
      } finally {
        try {
          if (app) await app.close();
          await fs.writeFile(path.join(artifacts, 'requests.json'), JSON.stringify(requests, null, 2));
        } finally {
          const cleanup = await fixture('drop');
          check(cleanup.dropped, 'Only this run\'s generated database is removed');
        }
      }
    }
  }
  assert.deepEqual(errors, [], 'No late PHP or browser errors');
  console.log(`PASS live WASM/PHP/MariaDB integration: 2 flows, ${assertions} state assertions`);
}

run().catch(error => { console.error(error); process.exitCode = 1; });
