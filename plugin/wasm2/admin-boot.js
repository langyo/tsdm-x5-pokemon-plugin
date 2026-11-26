// 由 scripts/publish/build_tairitsu_admin.py 生成（模板：boot.js） — 请勿手改。
// tairitsu 组件的浏览器启动脚本：注册 importmap、装 glue 运行时、
// 载入 jco 转译产物并调用组件导出的 lifecycle.start。
//
// 与 poc.html 的区别：资源一律用绝对 URL（页面是 /plugin.php?...，
// 相对路径会解析到站点根）。
(function () {
  'use strict';

  var BUILD_ID = '1795683419';
  var GLUE_CORE_ID = '1795698832';
  var BASE = '/source/plugin/pokemon/wasm2';

  // importmap 必须在任何 module 载入前注册，且每页只能有一个。
  if (!document.querySelector('script[type="importmap"]')) {
    var map = document.createElement('script');
    map.type = 'importmap';
    map.textContent = JSON.stringify({ imports: {"@bytecodealliance/preview2-shim/cli": "/source/plugin/pokemon/wasm2/wasi/cli.js?v=1795683419", "@bytecodealliance/preview2-shim/clocks": "/source/plugin/pokemon/wasm2/wasi/clocks.js?v=1795683419", "@bytecodealliance/preview2-shim/io": "/source/plugin/pokemon/wasm2/wasi/io.js?v=1795683419", "@bytecodealliance/preview2-shim/random": "/source/plugin/pokemon/wasm2/wasi/random.js?v=1795683419", "tairitsu-browser:full/css-style-declaration": "/source/plugin/pokemon/wasm2/shims/css-style-declaration.js?v=1795683419", "tairitsu-browser:full/document": "/source/plugin/pokemon/wasm2/shims/document.js?v=1795683419", "tairitsu-browser:full/dom-token-list": "/source/plugin/pokemon/wasm2/shims/dom-token-list.js?v=1795683419", "tairitsu-browser:full/element": "/source/plugin/pokemon/wasm2/shims/element.js?v=1795683419", "tairitsu-browser:full/element-css-inline-style": "/source/plugin/pokemon/wasm2/shims/element-css-inline-style.js?v=1795683419", "tairitsu-browser:full/event": "/source/plugin/pokemon/wasm2/shims/event.js?v=1795683419", "tairitsu-browser:full/event-target": "/source/plugin/pokemon/wasm2/shims/event-target.js?v=1795683419", "tairitsu-browser:full/html-input-element": "/source/plugin/pokemon/wasm2/shims/html-input-element.js?v=1795683419", "tairitsu-browser:full/mouse-event": "/source/plugin/pokemon/wasm2/shims/mouse-event.js?v=1795683419", "tairitsu-browser:full/node": "/source/plugin/pokemon/wasm2/shims/node.js?v=1795683419", "tairitsu-browser:full/node-list": "/source/plugin/pokemon/wasm2/shims/node-list.js?v=1795683419", "tairitsu-browser:full/non-element-parent-node": "/source/plugin/pokemon/wasm2/shims/non-element-parent-node.js?v=1795683419", "tairitsu-browser:full/parent-node": "/source/plugin/pokemon/wasm2/shims/parent-node.js?v=1795683419", "tairitsu-browser:full/platform-helpers": "/source/plugin/pokemon/wasm2/shims/platform-helpers.js?v=1795683419", "tairitsu-browser:full/pointer-event": "/source/plugin/pokemon/wasm2/shims/pointer-event.js?v=1795683419", "tairitsu-browser:full/wheel-event": "/source/plugin/pokemon/wasm2/shims/wheel-event.js?v=1795683419", "tairitsu-browser:full/window": "/source/plugin/pokemon/wasm2/shims/window.js?v=1795683419", "tairitsu-browser:full/history": "/source/plugin/pokemon/wasm2/shims/history.js?v=1795683419", "tairitsu-browser:full/location": "/source/plugin/pokemon/wasm2/shims/location.js?v=1795683419", "tairitsu-browser:full/mutation-observer": "/source/plugin/pokemon/wasm2/shims/mutation-observer.js?v=1795683419", "tairitsu-browser:full/resize-observer": "/source/plugin/pokemon/wasm2/shims/resize-observer.js?v=1795683419", "tairitsu-browser:full/media-query-list": "/source/plugin/pokemon/wasm2/shims/media-query-list.js?v=1795683419", "tairitsu-browser:full/html-canvas-element": "/source/plugin/pokemon/wasm2/shims/html-canvas-element.js?v=1795683419", "tairitsu-browser:full/canvas-rect": "/source/plugin/pokemon/wasm2/shims/canvas-rect.js?v=1795683419", "tairitsu-browser:full/canvas-fill-stroke-styles": "/source/plugin/pokemon/wasm2/shims/canvas-fill-stroke-styles.js?v=1795683419"} });
    document.head.appendChild(map);
  }

  if (!window.__tairitsuRafPatched) {
    window.__tairitsuRafPatched = true;
    window.__tairitsuFrames = { scheduled: 0, ran: 0, fallback: 0 };
    var nativeRaf = window.requestAnimationFrame.bind(window);
    var nativeCaf = window.cancelAnimationFrame.bind(window);
    var FALLBACK_MS = 120;
    var rafSeq = 0;
    var rafEntries = new Map();
    window.requestAnimationFrame = function (cb) {
      window.__tairitsuFrames.scheduled++;
      var id = ++rafSeq;
      var entry = { cb: cb, nativeId: 0, watchdog: 0, done: false };
      rafEntries.set(id, entry);
      var run = function (ts) {
        if (entry.done) return;
        entry.done = true;
        clearTimeout(entry.watchdog);
        rafEntries.delete(id);
        window.__tairitsuFrames.ran++;
        try {
          entry.cb(ts);
        } catch (err) {
          console.error('[tairitsu] frame callback failed', err);
        }
      };
      entry.nativeId = nativeRaf(run);
      entry.watchdog = setTimeout(function () {
        nativeCaf(entry.nativeId);
        window.__tairitsuFrames.fallback++;
        run(performance.now());
      }, FALLBACK_MS);
      return id;
    };
    // 帧调度带真 cancel（组件侧 TimerOps::cancel_animation_frame 由此生效）。
    window.cancelAnimationFrame = function (id) {
      var entry = rafEntries.get(id);
      if (!entry) return;
      entry.done = true;
      clearTimeout(entry.watchdog);
      nativeCaf(entry.nativeId);
      rafEntries.delete(id);
    };
  }


  globalThis.__setWasmExports = globalThis.__setWasmExports || function (e) {
    globalThis.__wasmExports = e;
  };

  // glue-core 是普通 script（不是 module），用同步 script 注入保证
  // glue 的全局句柄表在组件模块求值前就位。
  function loadGlueCore() {
    if (globalThis.__TAIRITSU_GLUE__) return Promise.resolve();
    return new Promise(function (resolve, reject) {
      var script = document.createElement('script');
      script.src = BASE + '/glue-core.js?v=' + GLUE_CORE_ID;
      script.onload = resolve;
      script.onerror = reject;
      document.head.appendChild(script);
    });
  }

  function findStart(obj, depth, seen) {
    depth = depth || 0;
    seen = seen || new Set();
    if (!obj || typeof obj !== 'object' || depth > 3 || seen.has(obj)) return null;
    seen.add(obj);
    for (var name in obj) {
      var value = obj[name];
      if (typeof value === 'function' && name.toLowerCase() === 'start') return value;
    }
    for (var key in obj) {
      var nested = obj[key];
      if (nested && typeof nested === 'object') {
        var found = findStart(nested, depth + 1, seen);
        if (found) return found;
      }
    }
    return null;
  }

  // 挂载点：#app 由 PHP 模板预置。组件自身会往 #app 里渲染第一层。
  window.__tairitsuBoot = function () {
    return loadGlueCore().then(function () {
      return import(BASE + '/component-wrapper/_admin2.js?b=' + BUILD_ID);
    }).then(function (mod) {
      globalThis.__setWasmExports(mod);
      var start = findStart(mod);
      if (!start) throw new Error('lifecycle.start not found in component exports');
      return start();
    }).then(function () {
      var app = document.getElementById('app');
      if (app) app.removeAttribute('data-booting');
      // 渲染调度桥：组件的调度回调写 #app[data-flush-pending]=N（同步
      // 导入，任何 jco 隐式线程都可靠），这里看到后向 #app 派发 pm-flush
      // 事件；组件在 #app 上的 pm-flush 监听器里执行 flush_render()。
      // DOM 事件处理器任务上下文与挂载/点击一致（已验证可靠），
      // 从而完全绕开 jco 对异步导入（rAF/setTimeout）的排队与锁损坏。
      var last = null;
      var observer = new MutationObserver(function () {
        var v = app.getAttribute('data-flush-pending');
        if (v !== null && v !== last) {
          last = v;
          app.dispatchEvent(new CustomEvent('pm-flush'));
        }
      });
      observer.observe(app, { attributes: true, attributeFilter: ['data-flush-pending'] });
    });
  };
})();
