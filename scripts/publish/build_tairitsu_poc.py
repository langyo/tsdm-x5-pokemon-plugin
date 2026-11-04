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

# npm 解包目录：`npm pack @celestia-island/tairitsu-web-glue@0.5.4` 与
# `npm pack @bytecodealliance/preview2-shim`，解压后合并为同一个 package/ 目录。
NPM_VENDOR = Path(os.environ.get(
    "TAIRITSU_POC_NPM_VENDOR",
    r"C:\Users\langy\AppData\Local\Temp\pm-poc\package",
))

# 组件实际 import 的 tairitsu-browser 接口 → 覆盖它的 glue 域模块 + 函数清单。
# 函数清单以 component-wrapper/_game2.js 顶部 import 语句为准（构建后核对）。
INTERFACE_MAP = {
    "css-style-declaration": ("css", ["getPropertyValue", "setProperty"]),
    "document": ("dom", ["createElement", "createTextNode", "getBody"]),
    "dom-token-list": ("css", ["add", "contains", "remove"]),
    "element": ("dom", [
        "getBoundingClientRect", "getClassList", "getClientHeight",
        "getScrollHeight", "getScrollTop", "removeAttribute", "setAttribute",
        "setInnerHtml", "setScrollTop",
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
    # node 缺 getChildNodes（0.5.26 world 新增），由 shim 生成段补写。
    "node": ("dom", [
        "appendChild", "getFirstChild", "insertBefore", "removeChild",
        "setTextContent",
    ]),
    # node-list 整个接口 tarball 缺失，shim 生成段全手写。
    "node-list": (None, ["getLength", "item"]),
    "non-element-parent-node": ("dom", ["getElementById"]),
    "parent-node": ("dom", ["querySelector"]),
    "platform-helpers": ("platform", [
        "cancelAnimationFrame", "clearInterval", "clearTimeout",
        "fetchPromise", "requestAnimationFrame", "setInterval", "setTimeout",
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

FETCH_PROMISE_PATCH = """// fetchPromise：tarball glue 传原始 Response（WIT 签名期望 String），修正为 text()。
export function fetchPromise(url, optionsJson) {
  const id = globalThis.__nextHandle ?? 1n;
  globalThis.__nextHandle = id + 1n;
  let options;
  try { options = optionsJson ? JSON.parse(optionsJson) : undefined; } catch { options = undefined; }
  const pcb = () => {
    const exports = globalThis.__wasmExports || {};
    return exports["tairitsu-browser:full/promise-callbacks@0.2.0"] || {};
  };
  fetch(url, options).then(async (resp) => {
    const text = await resp.text();
    const iface = pcb();
    (iface.onPromiseResolved || iface.on_promise_resolved || (() => {}))(id, text);
  }).catch((err) => {
    const iface = pcb();
    (iface.onPromiseRejected || iface.on_promise_rejected || (() => {}))(id, String(err));
  });
  return id;
}"""


def cargo_build(skip: bool) -> Path:
    wasm = ROOT / "target" / "wasm32-wasip2" / "release" / "_game2.wasm"
    if skip and wasm.exists():
        print("[poc] skip cargo build")
        return wasm
    # 固定参数列表：cargo build -p _game2 --target wasm32-wasip2 --release
    result = subprocess.run(
        ["cargo", "build", "-p", "_game2", "--target", "wasm32-wasip2", "--release"],
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
    glue_dir = OUT / "glue"
    if glue_dir.exists():
        shutil.rmtree(glue_dir)
    for domain in sorted({dom for dom, _ in INTERFACE_MAP.values() if dom}):
        src = NPM_VENDOR / "dist" / domain / "index.js"
        dst = glue_dir / domain / "index.js"
        if not src.exists():
            sys.exit(f"vendor glue missing: {src}")
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst)

    wasi_dir = OUT / "wasi"
    if wasi_dir.exists():
        shutil.rmtree(wasi_dir)
    src_browser = NPM_VENDOR / "dist" / "browser"
    if not src_browser.exists():
        sys.exit(f"vendor wasi missing: {src_browser}")
    shutil.copytree(src_browser, wasi_dir, ignore=shutil.ignore_patterns("*.d.ts"))
    print("[poc] vendored glue domains + wasi browser shims")


def shims_from_glue() -> None:
    """为每个接口生成 shim 模块：从域模块的 INTERFACES 映射导出平铺函数。

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
                f"import {{ INTERFACES }} from '../glue/{domain}/index.js';",
                f"const obj = INTERFACES['@tairitsu-glue/{iface}'];",
            ]
            for fn in functions:
                # node.getChildNodes 与 platform.fetchPromise 由下方补写段提供。
                if iface == "node" and fn == "getChildNodes":
                    continue
                if iface == "platform-helpers" and fn == "fetchPromise":
                    continue
                lines.append(f"export const {fn} = obj.{fn};")
            if iface == "node":
                lines += [
                    "// getChildNodes：0.5.26 world 新增（tarball glue 未实现）。",
                    "// 关键：返回 Element-only 集合（node.children 而非 childNodes）——",
                    "// diff 的子索引只数元素节点；走 childNodes 会因空白文本节点错位，",
                    "// UpdateChild/InsertChild 会打到错误的 DOM 节点（静默失败）。",
                    "export function getChildNodes(self) {",
                    "  const node = globalThis.__lookupNode(self);",
                    "  const list = node.children;",
                    "  return globalThis.__storeNodeList(list);",
                    "}",
                ]
            if iface == "platform-helpers":
                lines.append(FETCH_PROMISE_PATCH)
            code = "\n".join(lines)
        (shims / f"{iface}.js").write_text(code + "\n", encoding="utf-8")
    print("[poc] wrote", len(INTERFACE_MAP), "interface shims")


def glue_core() -> None:
    lines = ["(function(){", "'use strict';"]
    for table in HANDLE_TABLES:
        # __dispatchingEvents 是 Set（events glue 用 .add() 去重），其余为 Map。
        kind = "new Set()" if table == "__dispatchingEvents" else "new Map()"
        lines.append(f"if (!globalThis.{table}) globalThis.{table} = {kind};")
    for counter in HANDLE_COUNTERS:
        lines.append(f"if (!globalThis.{counter}) globalThis.{counter} = 1n;")
    lines += [
        "",
        "// 统一句柄空间：元素/文本/文档都是节点，全部落在 __allNodes，",
        "// 再按类别写入分表（分表供域模块直接 .get() 的场景）。",
        "// handle 分配集中于 __nextHandle，保证跨表唯一。",
        "if (!globalThis.__allNodes) globalThis.__allNodes = new Map();",
        "const storeDom = function (obj, kindTable) {",
        "  const h = globalThis.__nextHandle; globalThis.__nextHandle = h + 1n;",
        "  globalThis.__allNodes.set(h, obj);",
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

    import_map = {
        "@bytecodealliance/preview2-shim/cli": "./wasi/cli.js",
        "@bytecodealliance/preview2-shim/clocks": "./wasi/clocks.js",
        "@bytecodealliance/preview2-shim/io": "./wasi/io.js",
        "@bytecodealliance/preview2-shim/random": "./wasi/random.js",
    }
    # shim 地址带构建时间戳，避免浏览器缓存旧 shim（POC 期间频繁重生成）。
    build_id = str(int(Path(__file__).stat().st_mtime))
    for iface in INTERFACE_MAP:
        import_map[f"tairitsu-browser:full/{iface}"] = f"./shims/{iface}.js?v={build_id}"

    html = f"""<!DOCTYPE html>
<html lang="zh-Hans">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>POC：tairitsu + hikari</title>
<script type="importmap">
{json.dumps({"imports": import_map}, indent=2)}
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
// POC boot：静态 import 经 importmap 解析 glue/wasi，随后调用组件导出的 lifecycle.start。
globalThis.__setWasmExports = globalThis.__setWasmExports || ((e) => {{ globalThis.__wasmExports = e; }});
const mod = await import('./component-wrapper/_game2.js');

// 命名对齐：npm glue（0.5.4）按 snake_case 回连旧组件回调，而 0.5.26 组件导出
// camelCase（WIT kebab 提升）。用 Proxy 给回调接口加 snake_case 别名。
const CALLBACK_ALIASES = {{
  'animation-callbacks': {{ on_animation_frame: 'onFrame' }},
  'timer-callbacks': {{ on_timeout: 'onTimeout', on_interval: 'onInterval' }},
  'promise-callbacks': {{ on_promise_resolve: 'onPromiseResolved', on_promise_reject: 'onPromiseRejected' }},
  'scroll-callbacks': {{ on_scroll_event: 'onScrollEvent' }},
  'resize-observer-callbacks': {{ on_resize: 'onResize' }},
  'mutation-observer-callbacks': {{ on_mutate: 'onMutate' }},
  'media-query-list-callbacks': {{ on_change: 'onChange' }},
  'window-resize-callbacks': {{ on_window_resize: 'onWindowResize' }},
}};
const aligned = {{ ...mod }};
for (const [iface, aliases] of Object.entries(CALLBACK_ALIASES)) {{
  const key = 'tairitsu-browser:full/' + iface + '@0.2.0';
  const src = aligned[key];
  if (!src) continue;
  aligned[key] = new Proxy(src, {{
    get(t, p) {{
      if (typeof p === 'string' && aliases[p] !== undefined) {{
        // 返回包装：记录调用并转发到 camelCase 真名（debug 由 poc 模式自行扩展）
        const real = t[aliases[p]];
        return function (...args) {{ return real.apply(t, args); }};
      }}
      return t[p];
    }},
  }});
}}
globalThis.__setWasmExports(aligned);

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

const start = findStart(aligned);
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
    print("[poc] wrote poc.html")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-build", action="store_true", help="复用已有 wasm 产物")
    args = parser.parse_args()

    OUT.mkdir(parents=True, exist_ok=True)
    wasm = cargo_build(args.skip_build)
    jco_transpile(wasm)
    verify_imports()
    vendor_copy()
    shims_from_glue()
    glue_core()
    write_boot_assets()
    print("[poc] done →", OUT)


if __name__ == "__main__":
    main()
