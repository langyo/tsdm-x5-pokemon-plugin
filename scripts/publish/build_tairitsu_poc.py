#!/usr/bin/env python3
"""Build the tairitsu+hikari POC bundle into plugin/wasm2/.

Phase 1 POC pipeline (docs/refactor-tairitsu-hikari-research.md §7):
  1. cargo build -p _game2 --target wasm32-wasip2 --release
  2. jco transpile the component into plugin/wasm2/component-wrapper/
  3. vendor glue domain modules + wasi browser shims (from npm tarballs,
     unpacked location set via TAIRITSU_POC_NPM_VENDOR)
  4. generate per-interface shim modules + a glue-core runtime IIFE
  5. write poc.html / poc.json for standalone verification on the dev stack

Usage:
    python scripts/publish/build_tairitsu_poc.py [--skip-build]

Requirements: rust wasm32-wasip2 target, node + @bytecodealliance/jco (npx).
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "plugin" / "wasm2"

# 站点内路径：Discuz 从站点根提供插件静态资源。嵌入页面（/plugin.php?...）
# 必须用绝对 URL，相对路径会解析到站点根而非 wasm2/。
OUT_REL = "source/plugin/pokemon/wasm2"
OUT_REL_URL = "/" + OUT_REL

# 组件构建产物（game2 被排除在根 workspace 之外，有独立的 target/）。
WASM_ARTIFACT = (
    ROOT / "rust" / "game2" / "target" / "wasm32-wasip2" / "release" / "_game2.wasm"
)

# glue/wasi 产物目录：glue 来自本地 tairitsu checkout 重编译后的 npm dist
# （含 0.5.26 兼容修复，celestia-island/tairitsu#118），wasi 为
# @bytecodealliance/preview2-shim 的 browser 子集。
#
# 两者都不是本仓库的产物，没有合理默认值可猜：必须通过环境变量指定，
# 或用 --skip-vendor 复用 plugin/wasm2/ 里已经装配好的 glue 与 wasi。
#
#   export TAIRITSU_POC_NPM_VENDOR=<tairitsu>/packages/npm/celestia-tairitsu-web-glue/dist
#   export TAIRITSU_POC_WASI_VENDOR=<unpacked preview2-shim>/dist/browser
NPM_VENDOR = os.environ.get("TAIRITSU_POC_NPM_VENDOR")
WASI_VENDOR = os.environ.get("TAIRITSU_POC_WASI_VENDOR")
WASI_VENDOR_URL = (
    "https://unpkg.com/@bytecodealliance/preview2-shim/dist/browser"
)

# 组件实际 import 的 tairitsu-browser 接口 → 覆盖它的 glue 域模块 + 函数清单。
# 函数清单以 component-wrapper/_game2.js 顶部 import 语句为准（构建后核对）。
INTERFACE_MAP = {
    "css-style-declaration": ("css", ["getPropertyValue", "setProperty"]),
    "document": ("dom", ["createElement", "createTextNode", "getBody"]),
    "dom-token-list": ("css", ["add", "contains", "remove"]),
    "element": ("dom", [
        "getAttribute", "getBoundingClientRect", "getClassList", "getClientHeight",
        "getScrollHeight", "getScrollTop", "getTagName", "removeAttribute",
        "setAttribute", "setInnerHtml", "setScrollTop",
    ]),
    "element-css-inline-style": ("css", ["getStyle"]),
    "event": ("events", ["getCurrentTarget", "getTarget"]),
    "event-target": ("events", [
        "addEventListener", "preventDefault", "removeEventListener",
        "stopPropagation",
    ]),
    "html-input-element": ("html", ["getValue"]),
    "mouse-event": ("events", [
        "getAltKey", "getButton", "getButtons", "getClientX", "getClientY",
        "getCtrlKey", "getMetaKey", "getOffsetX", "getOffsetY", "getPageX",
        "getPageY", "getScreenX", "getScreenY", "getShiftKey",
    ]),
    # node 的 getChildNodes 由 shim 生成段补写（0.5.26 world 新增时的
    # 历史原因，现已入 glue node_exports）；getNodeType/replaceChild 随
    # patch 路径的 text-slot 修复入 glue，这里平铺导出。
    "node": ("dom", [
        "appendChild", "getFirstChild", "getNodeType", "getParentElement",
        "insertBefore", "removeChild", "replaceChild", "setTextContent",
    ]),
    # node-list 整个接口 tarball 缺失，shim 生成段全手写。
    "node-list": (None, ["getLength", "item"]),
    "non-element-parent-node": ("dom", ["getElementById"]),
    "parent-node": ("dom", ["querySelector"]),
    "platform-helpers": ("platform", [
        "cancelAnimationFrame", "clearInterval", "clearTimeout",
        "copyToClipboard", "fetchPromise", "readClipboard",
        "requestAnimationFrame", "setInterval", "setTimeout",
    ]),
    "pointer-event": ("events", [
        "getHeight", "getIsPrimary", "getPointerId", "getPointerType",
        "getPressure", "getTangentialPressure", "getTiltX", "getTiltY",
        "getTwist", "getWidth",
    ]),
    "wheel-event": ("events", [
        "getDeltaMode", "getDeltaX", "getDeltaY", "getDeltaZ",
    ]),
    "window": ("html", ["getComputedStyle", "getInnerWidth"]),
}

# glue 运行时需要预置的句柄表/计数器（inventory 出自 tarball glue 全部 globalThis 引用）。
HANDLE_TABLES = [
    "__cssStyleDeclarationHandles", "__dispatchingEvents", "__documentHandles",
    "__domRectHandles", "__elementHandles", "__eventHandles",
    "__listenerHandles", "__mutationRecordHandles", "__nodeListHandles",
    "__resizeObserverEntryHandles", "__resizeObserverSizeHandles",
]
HANDLE_COUNTERS = [
    "__nextCssStyleDeclarationHandle", "__nextDomRectHandle",
    "__nextEventHandle", "__nextHandle", "__nextListenerHandle",
    "__nextMutationRecord", "__nextNodeList", "__nextResizeObserverEntry",
    "__nextResizeObserverSizeHandle",
]




def cargo_build(skip: bool) -> Path:
    # game2 被排除在根 workspace 之外（见根 Cargo.toml 的 exclude），
    # 因此必须显式指定 manifest，产物落在 game2 自己的 target/ 下。
    #
    # cwd 必须是 game2 目录：cargo 只从「当前目录向上」读取 .cargo/config.toml，
    # 在仓库根跑会拿到根 workspace 的配置，看不到 game2 的本地 [patch]
    # （指向 ../../celestia 工作副本），于是解析到 git rev 的 tairitsu/hikari。
    manifest = ROOT / "rust" / "game2" / "Cargo.toml"
    game2_dir = ROOT / "rust" / "game2"
    wasm = WASM_ARTIFACT
    if skip and wasm.exists():
        print("[poc] skip cargo build")
        return wasm
    # 固定参数列表：cargo build --manifest-path rust/game2/Cargo.toml
    #             --target wasm32-wasip2 --release
    result = subprocess.run(
        [
            "cargo", "build",
            "--manifest-path", str(manifest),
            "--target", "wasm32-wasip2",
            "--release",
        ],
        cwd=str(game2_dir),
        check=False,
        shell=False,
    )
    if result.returncode != 0:
        sys.exit(result.returncode)
    if not wasm.exists():
        sys.exit("wasm artifact missing after build")
    return wasm


def jco_transpile(wasm: Path) -> None:
    wrapper = OUT / "component-wrapper"
    if wrapper.exists():
        shutil.rmtree(wrapper)
    # Windows 下 CreateProcess 需要 npx.cmd 完整名（shutil.which 按 PATHEXT 解析）。
    npx = shutil.which("npx") or "npx.cmd"
    # 固定参数列表：npx --yes @bytecodealliance/jco transpile <wasm> -o <dir>
    result = subprocess.run(
        [
            npx, "--yes", "@bytecodealliance/jco",
            "transpile", str(wasm), "-o", str(wrapper),
        ],
        check=False,
        shell=False,
    )
    if result.returncode != 0:
        sys.exit(result.returncode)


def bust_wrapper_cache(build_id: str) -> None:
    """给 jco 产物内的 core-wasm 引用加版本参数。

    jco 产物 URL 固定，浏览器启发式缓存会让新 wasm 永远加载不到
    （POC 排查时最大的坑）。
    """
    wrapper = OUT / 'component-wrapper' / '_game2.js'
    js = wrapper.read_text(encoding='utf-8')
    js = js.replace('./_game2.core.wasm', './_game2.core.wasm?v=' + build_id)
    wrapper.write_text(js, encoding='utf-8')


def verify_imports() -> None:
    """从转译产物提取全部 tairitsu-browser 导入接口名，与 INTERFACE_MAP 对账。"""
    wrapper = OUT / "component-wrapper" / "_game2.js"
    specifiers = sorted(set(re.findall(
        r"from ['\"]tairitsu-browser:full/([a-z-]+)['\"]",
        wrapper.read_text(encoding="utf-8"),
    )))
    unknown = [s for s in specifiers if s not in INTERFACE_MAP]
    if unknown:
        sys.exit(f"wrapper imports interfaces missing from INTERFACE_MAP: {unknown}")
    print(f"[poc] wrapper imports {len(specifiers)} interfaces — all mapped")


def vendor_copy() -> None:
    """从 npm 解包目录拷贝 glue 域模块与 wasi browser shims。"""
    if not NPM_VENDOR:
        sys.exit(
            "TAIRITSU_POC_NPM_VENDOR is not set.\n"
            "Point it at the built glue dist, e.g.\n"
            "  <tairitsu>/packages/npm/celestia-tairitsu-web-glue/dist\n"
            "(build it with `pnpm -C <tairitsu>/packages/npm/celestia-tairitsu-web-glue build`),"
            "\nor pass --skip-vendor to reuse the glue already in plugin/wasm2/."
        )
    if not WASI_VENDOR:
        sys.exit(
            "TAIRITSU_POC_WASI_VENDOR is not set.\n"
            "Unpack @bytecodealliance/preview2-shim and point it at dist/browser, e.g.\n"
            f"  curl -L {WASI_VENDOR_URL}  (or the npm tarball) into a temp dir,\n"
            "or pass --skip-vendor to reuse the wasi shims already in plugin/wasm2/."
        )

    glue_src = Path(NPM_VENDOR)
    glue_dir = OUT / "glue"
    if glue_dir.exists():
        shutil.rmtree(glue_dir)
    for domain in sorted({dom for dom, _ in INTERFACE_MAP.values() if dom}):
        src = glue_src / domain / "index.js"
        dst = glue_dir / domain / "index.js"
        if not src.exists():
            sys.exit(f"vendor glue missing: {src}")
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst)

    wasi_dir = OUT / "wasi"
    if wasi_dir.exists():
        shutil.rmtree(wasi_dir)
    src_browser = Path(WASI_VENDOR)
    if not src_browser.exists():
        sys.exit(f"vendor wasi missing: {src_browser}")
    shutil.copytree(src_browser, wasi_dir, ignore=shutil.ignore_patterns("*.d.ts"))
    print("[poc] vendored glue domains + wasi browser shims")


def shims_from_glue(build_id: str) -> None:
    """为每个接口生成 shim 模块：从域模块的 INTERFACES 映射导出平铺函数。

    glue 导入必须带 ?v=构建戳：shim→glue 的相对导入在 HTTP 层可能命中
    旧缓存条目（早于 no-cache 头的条目按启发式新鲜度直接复用、不再回源），
    而模块实例按 URL 缓存。旧版 glue（0.5.4 tarball）回连的是 snake_case
    旧回调名（on_timeout/on_animation_frame/on_promise_resolve），经 `?.`
    可选链全部静默无操作——帧/定时器/promise 三条投递链路一起失效，
    且不报任何错。带版本参数的 URL 是新缓存键，彻底切断这条污染路径。

    tarball 缺口补写：
    - node.getChildNodes（0.5.26 world 新增，tarball 未实现）
    - node-list 接口整体缺失（手写句柄表实现）
    - platform-helpers.fetchPromise 签名失配（传 Response 而非 String），
      改写为 await resp.text() 后回连 promiseCallbacks。
    """
    shims = OUT / "shims"
    if shims.exists():
        shutil.rmtree(shims)
    shims.mkdir(parents=True)

    for iface, (domain, functions) in INTERFACE_MAP.items():
        if iface == "node-list":
            code = "\n".join([
                "// node-list 接口：tarball glue 缺失，按 0.5.26 world 手写。",
                "// 句柄表必须用 glue-core 的全局 __nodeListHandles（node.getChildNodes",
                "// 经 __storeNodeList 存的就是这张表），跨模块不能各用各的本地表。",
                "export function getLength(self) {",
                "  const list = globalThis.__nodeListHandles && globalThis.__nodeListHandles.get(self);",
                "  return list ? list.length : 0;",
                "}",
                "export function item(self, index) {",
                "  const list = globalThis.__nodeListHandles && globalThis.__nodeListHandles.get(self);",
                "  if (!list || index >= list.length) return undefined;",
                "  const node = list[index];",
                "  if (!node) return undefined;",
                "  return globalThis.__storeNode(node);",
                "}",
            ])
        else:
            lines = [
                f"// shim for tairitsu-browser:full/{iface}",
                f"import {{ INTERFACES }} from '../glue/{domain}/index.js?v={build_id}';",
                f"const obj = INTERFACES['@tairitsu-glue/{iface}'];",
            ]
            for fn in functions:
                lines.append(f"export const {fn} = obj.{fn};")
            if iface == "node":
                lines += [
                    "// getChildNodes：0.5.26 world 新增（tarball glue 未实现）。",
                    "// 必须返回 childNodes（全节点）：diff 的子索引把 Text vnode 也计入",
                    "// （如 span 的动态文本是 child 0），用 element-only 的 children 会",
                    "// 让 UpdateText/UpdateChild 寻址错位（静默失败）。",
                    "export function getChildNodes(self) {",
                    "  const node = globalThis.__lookupNode(self);",
                    "  const list = node.childNodes;",
                    "  return globalThis.__storeNodeList(list);",
                    "}",
                ]
            code = "\n".join(lines)
        (shims / f"{iface}.js").write_text(code + "\n", encoding="utf-8")
    print("[poc] wrote", len(INTERFACE_MAP), "interface shims")


def glue_core() -> None:
    lines = ["(function(){", "'use strict';"]
    for table in HANDLE_TABLES:
        # __dispatchingEvents 是 Set（events glue 用 .add() 去重），其余为 Map。
        kind = "new Set()" if table == "__dispatchingEvents" else "new Map()"
        # __listenerHandles 特殊：同 (element, type) 只允许一个条目。
        # 重渲染的 UpdateEvent 补丁是 remove+add，remove 半路被 jco 吞掉时
        # add 仍会执行——表里出现双活条目，而事件分发取 first-match，若命中的
        # 是旧僵尸（wasm 回调已被 take 走）则整次点击静默无效。插入时驱逐
        # 同 (element, type) 的旧条目，让 first-match 永远是最新注册的那个。
        if table == "__listenerHandles":
            lines.append(
                "if (!globalThis.__listenerHandles) globalThis.__listenerHandles = "
                "new (class extends Map {"
                "  set(h, info) {"
                "    for (const [k, v] of this) {"
                "      if (k !== h && v.element === info.element && v.type === info.type) {"
                "        try { v.element.removeEventListener(v.type, v.listener); } catch (e) {}"
                "        this.delete(k);"
                "      }"
                "    }"
                "    return super.set(h, info);"
                "  }"
                "})();"
            )
            continue
        lines.append(f"if (!globalThis.{table}) globalThis.{table} = {kind};")
    for counter in HANDLE_COUNTERS:
        lines.append(f"if (!globalThis.{counter}) globalThis.{counter} = 1n;")
    lines += [
        "",
        "// 统一句柄空间：元素/文本/文档都是节点，全部落在 __allNodes，",
        "// 再按类别写入分表（分表供域模块直接 .get() 的场景）。",
        "// handle 分配集中于 __nextHandle，保证跨表唯一。",
        "//",
        "// 句柄按对象幂等（WeakMap 反查）：同一个 DOM 节点每次被解析（如",
        "// apply 期间反复 get_child_at → nodeList.item）必须返回同一个句柄。",
        "// 否则 wasm 侧的「元素身份」每次都变，按 (句柄, 事件) 键的监听器",
        "// 去重全部失配——重渲染一次叠一个 DOM 监听器，第 N 次点击触发",
        "// N 次 handler（实测一个按钮上堆到 7 个 click 监听）。",
        "if (!globalThis.__allNodes) globalThis.__allNodes = new Map();",
        "if (!globalThis.__nodeIdentity) globalThis.__nodeIdentity = new WeakMap();",
        "const storeDom = function (obj, kindTable) {",
        "  let h = globalThis.__nodeIdentity.get(obj);",
        "  if (h !== undefined) {",
        "    if (kindTable) kindTable.set(h, obj);",
        "    return h;",
        "  }",
        "  h = globalThis.__nextHandle; globalThis.__nextHandle = h + 1n;",
        "  globalThis.__allNodes.set(h, obj);",
        "  globalThis.__nodeIdentity.set(obj, h);",
        "  if (kindTable) kindTable.set(h, obj);",
        "  return h;",
        "};",
        "if (!globalThis.__storeElement) globalThis.__storeElement = function (obj) {",
        "  return storeDom(obj, globalThis.__elementHandles);",
        "};",
        "if (!globalThis.__storeText) globalThis.__storeText = function (obj) {",
        "  return storeDom(obj, null);",
        "};",
        "if (!globalThis.__storeNode) globalThis.__storeNode = function (obj) {",
        "  return storeDom(obj, null);",
        "};",
        "if (!globalThis.__lookupNode) globalThis.__lookupNode = function (h) {",
        "  const obj = globalThis.__allNodes.get(h);",
        "  if (obj === undefined) throw new Error('node handle ' + h + ' not found');",
        "  return obj;",
        "};",
        "if (!globalThis.__lookupElement) globalThis.__lookupElement = function (h) {",
        "  const obj = globalThis.__elementHandles.get(h) ?? globalThis.__allNodes.get(h);",
        "  if (obj === undefined) throw new Error('element handle ' + h + ' not found');",
        "  return obj;",
        "};",
        "if (!globalThis.__lookupText) globalThis.__lookupText = function (h) {",
        "  const obj = globalThis.__allNodes.get(h);",
        "  if (obj === undefined) throw new Error('text handle ' + h + ' not found');",
        "  return obj;",
        "};",
        "if (!globalThis.__storeNodeList) globalThis.__storeNodeList = function (obj) {",
        "  const h = globalThis.__nextNodeList; globalThis.__nextNodeList = h + 1n;",
        "  globalThis.__nodeListHandles.set(h, obj); return h;",
        "};",
        "if (!globalThis.__setWasmExports) globalThis.__setWasmExports = function (exports) {",
        "  globalThis.__wasmExports = exports;",
        "};",
        "if (!globalThis.__TAIRITSU_GLUE__) globalThis.__TAIRITSU_GLUE__ = { INTERFACES: {} };",
        "",
        "})();",
    ]
    (OUT / "glue-core.js").write_text("\n".join(lines), encoding="utf-8")
    print("[poc] wrote glue-core.js")


def write_boot_assets() -> None:
    poc_json = json.dumps({
        "ok": True,
        "message": "POC fetch 往返成功：JS fetch → promiseCallbacks → Rust 信号",
        "items": ["回复药", "精灵球"],
    }, ensure_ascii=False, indent=2)
    (OUT / "poc.json").write_text(poc_json + "\n", encoding="utf-8")

    # shim 与入口地址统一带构建时间戳（wasm 的 mtime），避免浏览器缓存
    # 旧 shim / 旧 boot.js / 旧组件。必须取 wasm 的 mtime 而不是脚本自身：
    # 脚本没改、只重编 wasm 时，脚本 mtime 不变会把缓存当成有效。
    # wasi 一样要带：模块实例按 URL 缓存，且早于 no-cache 头的 HTTP 缓存
    # 条目按启发式新鲜度复用、不回源——版本参数是新缓存键，绕开两者。
    build_id = str(int(WASM_ARTIFACT.stat().st_mtime))
    import_map = {
        "@bytecodealliance/preview2-shim/cli": f"./wasi/cli.js?v={build_id}",
        "@bytecodealliance/preview2-shim/clocks": f"./wasi/clocks.js?v={build_id}",
        "@bytecodealliance/preview2-shim/io": f"./wasi/io.js?v={build_id}",
        "@bytecodealliance/preview2-shim/random": f"./wasi/random.js?v={build_id}",
    }
    for iface in INTERFACE_MAP:
        import_map[f"tairitsu-browser:full/{iface}"] = f"./shims/{iface}.js?v={build_id}"

    # 嵌入 Discuz 页面用的启动脚本：importmap 必须是绝对地址（页面 URL 是
    # /plugin.php?id=pokemon:game，相对路径会解析到站点根，不是 wasm2/）。
    abs_map = {k: f"/{OUT_REL}/{v.lstrip('./')}" for k, v in import_map.items()}
    # glue-core 的缓存键取它自身的 mtime：它的内容会独立于 wasm 变化
    # （如句柄幂等修复），若沿用 wasm 的 BUILD_ID，浏览器会继续用旧
    # 缓存里的 glue-core。
    glue_core_id = str(int((OUT / "glue-core.js").stat().st_mtime))
    boot_js = f"""// 由 scripts/publish/build_tairitsu_poc.py 生成 — 请勿手改。
// tairitsu 组件的浏览器启动脚本：注册 importmap、装 glue 运行时、
// 载入 jco 转译产物并调用组件导出的 lifecycle.start。
//
// 与 poc.html 的区别：资源一律用绝对 URL（页面是 /plugin.php?...，
// 相对路径会解析到站点根），帧调度换成 MessageChannel（后台标签页的
// rAF 会被节流到停摆，调度器会永久卡住）。
(function () {{
  'use strict';

  var BUILD_ID = {build_id!r};
  var GLUE_CORE_ID = {glue_core_id!r};
  var BASE = {OUT_REL_URL!r};

  // importmap 必须在任何 module 载入前注册，且每页只能有一个。
  if (!document.querySelector('script[type="importmap"]')) {{
    var map = document.createElement('script');
    map.type = 'importmap';
    map.textContent = JSON.stringify({{ imports: {json.dumps(abs_map)} }});
    document.head.appendChild(map);
  }}

  // tairitsu 的调度器在后台标签页会卡死（rAF 不触发 → rt.scheduled 无法复位）。
  // MessageChannel 不受节流影响，把 rAF 改写到它上面。
  //
  // 计数暴露在 window.__tairitsuFrames 上：帧投递失败时（组件点不动、
  // 列表不更新）第一个要看的就是它 —— scheduled=0 说明调度器根本没走
  // rAF，scheduled>0 而 ran=0 说明 MessageChannel 的帧没跑起来。
  if (!window.__tairitsuRafPatched) {{
    window.__tairitsuRafPatched = true;
    window.__tairitsuFrames = {{ scheduled: 0, ran: 0 }};
    var channel = new MessageChannel();
    var pending = [];
    channel.port1.onmessage = function () {{
      var callbacks = pending;
      pending = [];
      for (var i = 0; i < callbacks.length; i++) {{
        window.__tairitsuFrames.ran++;
        try {{
          callbacks[i](performance.now());
        }} catch (err) {{
          console.error('[tairitsu] frame callback failed', err);
        }}
      }}
    }};
    window.requestAnimationFrame = function (cb) {{
      window.__tairitsuFrames.scheduled++;
      pending.push(cb);
      channel.port2.postMessage(0);
      return pending.length;
    }};
    window.cancelAnimationFrame = function () {{}};
  }}

  globalThis.__setWasmExports = globalThis.__setWasmExports || function (e) {{
    globalThis.__wasmExports = e;
  }};

  // glue-core 是普通 script（不是 module），用同步 script 注入保证
  // glue 的全局句柄表在组件模块求值前就位。
  function loadGlueCore() {{
    if (globalThis.__TAIRITSU_GLUE__) return Promise.resolve();
    return new Promise(function (resolve, reject) {{
      var script = document.createElement('script');
      script.src = BASE + '/glue-core.js?v=' + GLUE_CORE_ID;
      script.onload = resolve;
      script.onerror = reject;
      document.head.appendChild(script);
    }});
  }}

  function findStart(obj, depth, seen) {{
    depth = depth || 0;
    seen = seen || new Set();
    if (!obj || typeof obj !== 'object' || depth > 3 || seen.has(obj)) return null;
    seen.add(obj);
    for (var name in obj) {{
      var value = obj[name];
      if (typeof value === 'function' && name.toLowerCase() === 'start') return value;
    }}
    for (var key in obj) {{
      var nested = obj[key];
      if (nested && typeof nested === 'object') {{
        var found = findStart(nested, depth + 1, seen);
        if (found) return found;
      }}
    }}
    return null;
  }}

  // 挂载点：#app 由 PHP 模板预置。组件自身会往 #app 里渲染第一层。
  window.__tairitsuBoot = function () {{
    return loadGlueCore().then(function () {{
      return import(BASE + '/component-wrapper/_game2.js?b=' + BUILD_ID);
    }}).then(function (mod) {{
      globalThis.__setWasmExports(mod);
      var start = findStart(mod);
      if (!start) throw new Error('lifecycle.start not found in component exports');
      return start();
    }}).then(function () {{
      var app = document.getElementById('app');
      if (app) app.removeAttribute('data-booting');
      // 渲染调度桥：组件的调度回调写 #app[data-flush-pending]=N（同步
      // 导入，任何 jco 隐式线程都可靠），这里看到后向 #app 派发 pm-flush
      // 事件；组件在 #app 上的 pm-flush 监听器里执行 flush_render()。
      // DOM 事件处理器任务上下文与挂载/点击一致（已验证可靠），
      // 从而完全绕开 jco 对异步导入（rAF/setTimeout）的排队与锁损坏。
      var last = null;
      var observer = new MutationObserver(function () {{
        var v = app.getAttribute('data-flush-pending');
        if (v !== null && v !== last) {{
          last = v;
          app.dispatchEvent(new CustomEvent('pm-flush'));
        }}
      }});
      observer.observe(app, {{ attributes: true, attributeFilter: ['data-flush-pending'] }});
    }});
  }};
}})();
"""
    (OUT / "boot.js").write_text(boot_js, encoding="utf-8")

    manifest = {
        "build_id": build_id,
        "base": OUT_REL_URL,
        "entry": f"{OUT_REL_URL}/boot.js?v={build_id}",
        "component": f"{OUT_REL_URL}/component-wrapper/_game2.js?b={build_id}",
        "interfaces": sorted(INTERFACE_MAP),
    }
    (OUT / "manifest.json").write_text(
        json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )

    html = f"""<!DOCTYPE html>
<html lang="zh-Hans">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>POC：tairitsu + hikari</title>
<script type="importmap">
{json.dumps({"imports": import_map}, indent=2)}
</script>
<script>
// 后台/隐藏标签页的 rAF 不触发，会让 tairitsu 调度器永久卡死（rt.scheduled
// 无法复位）。POC 页面用 MessageChannel 驱动帧；前台真实使用无此问题。
(function () {{
  (function () {{
    var ch = new MessageChannel();
    var pending = [];
    ch.port1.onmessage = function () {{
      var cbs = pending; pending = [];
      for (var i = 0; i < cbs.length; i++) cbs[i](performance.now());
    }};
    window.requestAnimationFrame = function (cb) {{
      pending.push(cb);
      ch.port2.postMessage(0);
      return pending.length;
    }};
    window.cancelAnimationFrame = function () {{}};
  }})();
}})();
</script>
<script src="./glue-core.js"></script>
<style>
body {{ margin: 0; background: #f7f9fa; font-family: "Noto Sans SC", system-ui, sans-serif; }}
#main {{ max-width: 960px; margin: 24px auto; padding: 0 12px; }}
#app {{ min-height: 120px; }}
#app[data-booting] {{ color: #999; padding: 40px 0; text-align: center; }}
</style>
</head>
<body>
<div id="main"><div id="app" data-booting>正在加载 POC…</div></div>
<script type="module">
const BUILD_ID = '__BUILD_ID__';
// POC boot：静态 import 经 importmap 解析 glue/wasi，随后调用组件导出的 lifecycle.start。
globalThis.__setWasmExports = globalThis.__setWasmExports || ((e) => {{ globalThis.__wasmExports = e; }});
const mod = await import('./component-wrapper/_game2.js?b=' + BUILD_ID);

globalThis.__setWasmExports(mod);

const findStart = (obj, depth = 0, seen = new Set()) => {{
  if (!obj || typeof obj !== 'object' || depth > 3 || seen.has(obj)) return null;
  seen.add(obj);
  for (const [name, value] of Object.entries(obj)) {{
    if (typeof value === 'function' && name.toLowerCase() === 'start') return value;
  }}
  for (const value of Object.values(obj)) {{
    if (value && typeof value === 'object') {{
      const found = findStart(value, depth + 1, seen);
      if (found) return found;
    }}
  }}
  return null;
}};

const start = findStart(mod);
if (!start) {{
  document.getElementById('app').textContent = '未找到 lifecycle.start 导出';
  throw new Error('lifecycle.start not found in component exports');
}}
await start();
const app = document.getElementById('app');
if (app) app.removeAttribute('data-booting');
console.log('[poc] component started');
</script>
</body>
</html>
"""
    (OUT / "poc.html").write_text(html, encoding="utf-8")
    print("[poc] wrote poc.html, boot.js, manifest.json")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-build", action="store_true", help="复用已有 wasm 产物")
    parser.add_argument(
        "--skip-vendor",
        action="store_true",
        help="复用 plugin/wasm2/ 里已装配的 glue 与 wasi（不需要 npm 解包目录）",
    )
    args = parser.parse_args()

    OUT.mkdir(parents=True, exist_ok=True)
    wasm = cargo_build(args.skip_build)
    jco_transpile(wasm)
    build_id = str(int(wasm.stat().st_mtime))
    bust_wrapper_cache(build_id)
    verify_imports()
    if not args.skip_vendor:
        vendor_copy()
    elif not (OUT / "glue").is_dir() or not (OUT / "wasi").is_dir():
        sys.exit("--skip-vendor requires existing plugin/wasm2/glue and plugin/wasm2/wasi")
    # glue getParentElement 补丁已退役（2026-11-07）：上游
    # celestia-island/tairitsu#124 合入并随 dist 重新 vendor，vendored
    # dom glue 自带 __storeNode 实现，无需再改写。
    shims_from_glue(build_id)
    glue_core()
    write_boot_assets()
    print("[poc] done →", OUT)


if __name__ == "__main__":
    main()
