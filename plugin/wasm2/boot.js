// 由 scripts/publish/build_tairitsu_poc.py 生成 — 请勿手改。
// tairitsu 组件的浏览器启动脚本：注册 importmap、装 glue 运行时、
// 载入 jco 转译产物并调用组件导出的 lifecycle.start。
//
// 与 poc.html 的区别：资源一律用绝对 URL（页面是 /plugin.php?...，
// 相对路径会解析到站点根），帧调度换成 MessageChannel（后台标签页的
// rAF 会被节流到停摆，调度器会永久卡住）。
(function () {
  'use strict';

  var BUILD_ID = '1793959276';
  var GLUE_CORE_ID = '1793959284';
  var BASE = '/source/plugin/pokemon/wasm2';

  // importmap 必须在任何 module 载入前注册，且每页只能有一个。
  if (!document.querySelector('script[type="importmap"]')) {
    var map = document.createElement('script');
    map.type = 'importmap';
    map.textContent = JSON.stringify({ imports: {"@bytecodealliance/preview2-shim/cli": "/source/plugin/pokemon/wasm2/wasi/cli.js?v=1793959276", "@bytecodealliance/preview2-shim/clocks": "/source/plugin/pokemon/wasm2/wasi/clocks.js?v=1793959276", "@bytecodealliance/preview2-shim/io": "/source/plugin/pokemon/wasm2/wasi/io.js?v=1793959276", "@bytecodealliance/preview2-shim/random": "/source/plugin/pokemon/wasm2/wasi/random.js?v=1793959276", "tairitsu-browser:full/css-style-declaration": "/source/plugin/pokemon/wasm2/shims/css-style-declaration.js?v=1793959276", "tairitsu-browser:full/document": "/source/plugin/pokemon/wasm2/shims/document.js?v=1793959276", "tairitsu-browser:full/dom-token-list": "/source/plugin/pokemon/wasm2/shims/dom-token-list.js?v=1793959276", "tairitsu-browser:full/element": "/source/plugin/pokemon/wasm2/shims/element.js?v=1793959276", "tairitsu-browser:full/element-css-inline-style": "/source/plugin/pokemon/wasm2/shims/element-css-inline-style.js?v=1793959276", "tairitsu-browser:full/event": "/source/plugin/pokemon/wasm2/shims/event.js?v=1793959276", "tairitsu-browser:full/event-target": "/source/plugin/pokemon/wasm2/shims/event-target.js?v=1793959276", "tairitsu-browser:full/html-input-element": "/source/plugin/pokemon/wasm2/shims/html-input-element.js?v=1793959276", "tairitsu-browser:full/mouse-event": "/source/plugin/pokemon/wasm2/shims/mouse-event.js?v=1793959276", "tairitsu-browser:full/node": "/source/plugin/pokemon/wasm2/shims/node.js?v=1793959276", "tairitsu-browser:full/node-list": "/source/plugin/pokemon/wasm2/shims/node-list.js?v=1793959276", "tairitsu-browser:full/non-element-parent-node": "/source/plugin/pokemon/wasm2/shims/non-element-parent-node.js?v=1793959276", "tairitsu-browser:full/parent-node": "/source/plugin/pokemon/wasm2/shims/parent-node.js?v=1793959276", "tairitsu-browser:full/platform-helpers": "/source/plugin/pokemon/wasm2/shims/platform-helpers.js?v=1793959276", "tairitsu-browser:full/pointer-event": "/source/plugin/pokemon/wasm2/shims/pointer-event.js?v=1793959276", "tairitsu-browser:full/wheel-event": "/source/plugin/pokemon/wasm2/shims/wheel-event.js?v=1793959276", "tairitsu-browser:full/window": "/source/plugin/pokemon/wasm2/shims/window.js?v=1793959276"} });
    document.head.appendChild(map);
  }

  // tairitsu 的调度器在后台标签页会卡死（rAF 不触发 → rt.scheduled 无法复位）。
  // MessageChannel 不受节流影响，把 rAF 改写到它上面。
  //
  // 计数暴露在 window.__tairitsuFrames 上：帧投递失败时（组件点不动、
  // 列表不更新）第一个要看的就是它 —— scheduled=0 说明调度器根本没走
  // rAF，scheduled>0 而 ran=0 说明 MessageChannel 的帧没跑起来。
  if (!window.__tairitsuRafPatched) {
    window.__tairitsuRafPatched = true;
    window.__tairitsuFrames = { scheduled: 0, ran: 0 };
    var channel = new MessageChannel();
    var pending = [];
    channel.port1.onmessage = function () {
      var callbacks = pending;
      pending = [];
      for (var i = 0; i < callbacks.length; i++) {
        window.__tairitsuFrames.ran++;
        try {
          callbacks[i](performance.now());
        } catch (err) {
          console.error('[tairitsu] frame callback failed', err);
        }
      }
    };
    window.requestAnimationFrame = function (cb) {
      window.__tairitsuFrames.scheduled++;
      pending.push(cb);
      channel.port2.postMessage(0);
      return pending.length;
    };
    window.cancelAnimationFrame = function () {};
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
      return import(BASE + '/component-wrapper/_game2.js?b=' + BUILD_ID);
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
