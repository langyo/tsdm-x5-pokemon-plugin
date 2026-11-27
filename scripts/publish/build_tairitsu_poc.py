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
import tempfile
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
# shim 生成段手写补出的函数：不在 glue 域模块的平铺导出里，所以不计入
# INTERFACE_MAP 的函数清单，但 wrapper 确实会 import 它们。
SHIM_PROVIDED = {
    "node": {"getChildNodes"},
    "node-list": {"getLength", "item"},
}

# 帧调度补丁，boot.js 与 poc.html 共用同一份（经 f-string 插值，所以这里
# 是普通 JS，花括号不用转义）。
#
# 帧调度走浏览器原生 rAF：与刷新率对齐，空闲时不占 CPU。后台标签页和
# 不可见的 webview 里原生 rAF 会被节流到停摆，而 tairitsu 的调度器要靠
# 一帧回来复位——所以每个帧请求配一个看门狗：原生帧迟迟不来就补投一次
# （并撤掉那个迟到的原生帧，保证回调只跑一次）。
#
# 早期 POC 用 MessageChannel 无条件接管 rAF：后台可用了，但前台也不再
# 对齐刷新率——实测任何 rAF 循环都会跑到每秒十万帧。
#
# 计数暴露在 window.__tairitsuFrames 上：帧投递失败时（组件点不动、
# 列表不更新）第一个要看的就是它 —— scheduled=0 说明调度器根本没走
# rAF，scheduled>0 而 ran=0 说明原生帧没跑起来且看门狗也没补上。
FRAME_SCHEDULER_JS = """  if (!window.__tairitsuRafPatched) {
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
"""

NPM_VENDOR = os.environ.get("TAIRITSU_POC_NPM_VENDOR")
WASI_VENDOR = os.environ.get("TAIRITSU_POC_WASI_VENDOR")
WASI_VENDOR_URL = (
    "https://unpkg.com/@bytecodealliance/preview2-shim/dist/browser"
)

# 组件实际 import 的 tairitsu-browser 接口 → 覆盖它的 glue 域模块 + 函数清单。
# 函数清单以 component-wrapper/_game2.js 顶部 import 语句为准（构建后核对）。
INTERFACE_MAP = {
    "css-style-declaration": ("css", ["getPropertyValue", "setProperty"]),
    "document": ("dom", [
        "createElement", "createElementNs", "createTextNode", "elementFromPoint",
        "execCommand", "exitFullscreen", "getBody",
    ]),
    "dom-token-list": ("css", ["add", "contains", "remove"]),
    "element": ("dom", [
        "closest", "getAttribute", "getBoundingClientRect", "getClassList",
        "getClientHeight", "getClientWidth", "getInnerHtml", "getScrollHeight",
        "getScrollTop", "getTagName", "removeAttribute", "setAttribute",
        "requestFullscreen", "setInnerHtml", "setScrollTop",
    ]),
    "element-css-inline-style": ("css", ["getStyle"]),
    "event": ("events", ["getCurrentTarget", "getTarget"]),
    "event-target": ("events", [
        "addEventListener", "preventDefault", "removeEventListener",
        "stopPropagation",
    ]),
    # getType/setValue/setChecked：tairitsu-web 给表单控件的 value / checked 落
    # 实时状态时读元素类型并写属性（getValue 是读进来那条路，getChecked 供
    # 能力探针读回复选框的实时状态）。
    "html-input-element": ("html", [
        "getChecked", "getType", "getValue", "setChecked", "setValue",
    ]),
    # setSelected：tairitsu-web 给 <option> 的 selected 落实时选中态（只写属性
    # 对用户动过的 option 不起作用）；getSelected 供能力探针逐项读回。
    "html-option-element": ("html", ["getSelected", "setSelected"]),
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
        "getParentNode", "insertBefore", "removeChild", "replaceChild",
        "setTextContent",
    ]),
    # node-list 整个接口 tarball 缺失，shim 生成段全手写。
    "node-list": (None, ["getLength", "item"]),
    "non-element-parent-node": ("dom", ["getElementById"]),
    "parent-node": ("dom", ["querySelector"]),
    "platform-helpers": ("platform", [
        "cancelAnimationFrame", "clearInterval", "clearTimeout",
        "clipboardWriteTextPromise", "connectWebSocket", "copyToClipboard",
        "createAudioContext", "createMutationObserver", "createResizeObserver",
        "drawQrcodeOnCanvasById", "fetchPromise", "getContenteditableState",
        "getCurrentPosition", "getElementRectById", "getGeolocationHandle",
        "getSelectionEnd", "getSelectionStart", "idbClear", "idbDelete",
        "idbGet", "idbOpen", "idbPut", "onScroll", "prefersDarkMode",
        "querySelector", "querySelectorAll", "readClipboard",
        "requestAnimationFrame", "scrollTo", "setContentEditable",
        "setInterval", "setTimeout",
    ]),
    "pointer-event": ("events", [
        "getHeight", "getIsPrimary", "getPointerId", "getPointerType",
        "getPressure", "getTangentialPressure", "getTiltX", "getTiltY",
        "getTwist", "getWidth",
    ]),
    "wheel-event": ("events", [
        "getDeltaMode", "getDeltaX", "getDeltaY", "getDeltaZ",
    ]),
    "window": ("html", [
        "getComputedStyle", "getInnerHeight", "getInnerWidth", "getScrollY",
        "matchMedia",
    ]),
    # 能力探针（capprobe）带出的接口：history/location 走 html 域；
    # mutation-observer/resize-observer 走各自域；media-query-list 与
    # canvas 2D 三件套在上游 npm glue 的 platform 域注册。
    "history": ("html", ["pushState", "replaceState"]),
    # location 全量：后台外壳的账号菜单靠 setHref 整页跳转（上游 glue 在
    # tairitsu#166 补齐，此前只有 getHash / getPathname 两个只读函数）。
    "location": ("html", [
        "assign", "getAncestorOrigins", "getHash", "getHref", "getHostname",
        "getOrigin", "getPathname", "getPort", "getProtocol", "getSearch",
        "reload", "replace", "setHash", "setHref", "setHostname", "setPathname",
        "setPort", "setProtocol", "setSearch",
    ]),
    "mutation-observer": ("observers", ["observe", "disconnect"]),
    "resize-observer": ("resize-observer", ["observe", "unobserve", "disconnect"]),
    "media-query-list": ("html", [
        "addListener", "getMatches", "getMedia", "removeListener",
    ]),
    "html-canvas-element": ("platform", ["getContext"]),
    "canvas-rect": ("platform", ["clearRect", "fillRect"]),
    "canvas-fill-stroke-styles": ("platform", ["setFillStyle"]),
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
    # component-wrapper/ 是多个组件共用的目录：admin2 / capprobe 也放在这里，
    # 且后台默认就走 admin2。整目录 rmtree 会连带删掉它们的产物（构建完
    # game2 后台就 404），所以只清理本次要重写的 _game2* 与 interfaces/。
    wrapper.mkdir(parents=True, exist_ok=True)
    for stale in wrapper.glob("_game2*"):
        if stale.is_file():
            stale.unlink()
    interfaces = wrapper / "interfaces"
    if interfaces.exists():
        shutil.rmtree(interfaces)
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


def wrapper_imports(wrapper: Path) -> dict[str, set[str]]:
    """一份转译产物从 tairitsu-browser 导入的 {接口名: {函数名}}。"""
    text = wrapper.read_text(encoding="utf-8")
    pairs = re.findall(
        r"import\s*\{([^}]*)\}\s*from\s*['\"]tairitsu-browser:full/([a-z0-9-]+)['\"]",
        text,
    )
    imported: dict[str, set[str]] = {}
    for names, iface in pairs:
        bucket = imported.setdefault(iface, set())
        bucket.update(
            name.strip().split(" as ")[0] for name in names.split(",") if name.strip()
        )
    return imported


def verify_imports() -> None:
    """从转译产物提取 tairitsu-browser 的接口与函数导入，与 INTERFACE_MAP 对账。

    只核对接口名不够：shim 是 `export const fn = obj.fn` 的平铺再导出，
    INTERFACE_MAP 漏掉某个函数时 shim 就不导出它，浏览器加载 wrapper 直接报
    "does not provide an export named …"，整页起不来（上游 Rust 侧开始调用
    create-element-ns 时实测踩到）。

    game2 / admin2 / capprobe 三份产物共用同一套 shim，所以三份都要对账：
    后台外壳用 location.setHref 跳转那次，漏的正是 admin2 独有的一条导入。
    """
    wrappers = sorted((OUT / "component-wrapper").glob("_*.js"))
    if not wrappers:
        sys.exit("no transpiled wrapper under component-wrapper/")
    unknown: list[str] = []
    missing: list[str] = []
    for wrapper in wrappers:
        imported = wrapper_imports(wrapper)
        unknown += [f"{wrapper.name}:{iface}" for iface in imported if iface not in INTERFACE_MAP]
        missing += [
            f"{wrapper.name}:{iface}.{fn}"
            for iface, fns in imported.items()
            for fn in fns
            if fn not in INTERFACE_MAP[iface][1] and fn not in SHIM_PROVIDED.get(iface, ())
        ]
    if unknown:
        sys.exit(f"wrapper imports interfaces missing from INTERFACE_MAP: {sorted(unknown)}")
    if missing:
        sys.exit(f"wrapper imports functions missing from INTERFACE_MAP: {sorted(missing)}")
    print(
        f"[poc] {len(wrappers)} wrappers checked ({', '.join(w.name for w in wrappers)}) — "
        "every imported interface and function is exported by its shim"
    )


def verify_glue_exports() -> None:
    """INTERFACE_MAP 里登记的函数，glue 域模块里必须真的有。

    shim 只是 `export const fn = obj.fn` 的转发：obj.fn 不存在时导出值是
    undefined，模块能加载，直到组件第一次调用才在运行时炸——而那时页面已经
    起来了。构建期对着 glue 源码（esbuild 产物保留属性名）点一遍名，缺哪个
    直接报哪个，提示去补上游 glue 而不是在宿主里绕。
    """
    absent: list[str] = []
    for iface, (domain, functions) in INTERFACE_MAP.items():
        if domain is None:
            continue
        glue = OUT / "glue" / domain / "index.js"
        if not glue.exists():
            sys.exit(f"glue domain module missing: {glue}")
        text = glue.read_text(encoding="utf-8")
        for fn in functions:
            if not re.search(rf"\b{re.escape(fn)}\s*\(", text):
                absent.append(f"{iface}.{fn} (glue {domain})")
    if absent:
        sys.exit(
            "INTERFACE_MAP lists functions the vendored glue does not implement:\n  "
            + "\n  ".join(sorted(absent))
            + "\nExtend the upstream glue (tairitsu packages/npm/celestia-tairitsu-web-glue)"
            " and re-vendor instead of working around it in the host."
        )
    print("[poc] every INTERFACE_MAP function is implemented by the vendored glue")


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

    # wasi browser shims 很少变：没给目录就沿用 plugin/wasm2/wasi 里现有的。
    wasi_dir = OUT / "wasi"
    if not WASI_VENDOR:
        if not wasi_dir.is_dir():
            sys.exit(
                "TAIRITSU_POC_WASI_VENDOR is not set and plugin/wasm2/wasi is missing.\n"
                "Unpack @bytecodealliance/preview2-shim and point it at dist/browser, e.g.\n"
                f"  curl -L {WASI_VENDOR_URL}  (or the npm tarball) into a temp dir."
            )
        print("[poc] vendored glue domains (wasi shims reused)")
        return
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
// 相对路径会解析到站点根）。
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

{FRAME_SCHEDULER_JS}

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
// 帧调度与 boot.js 同一份补丁（原生 rAF + 看门狗，说明见 FRAME_SCHEDULER_JS）。
(function () {{
{FRAME_SCHEDULER_JS}}})();
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


def root_custom_properties(css: str) -> dict[str, str]:
    """样式表顶层 `:root { … }` 上定义的自定义属性（@media 之类嵌套里的不算）。

    只按花括号配对扫，不认引号里的括号——令牌块里不会出现那种内容。
    """
    out: dict[str, str] = {}
    css = css.lstrip("\ufeff")  # 压缩产物开头可能是 BOM，strip() 不认它
    depth = 0
    sel_start = 0
    body_start = 0
    selector = ""
    for i, ch in enumerate(css):
        if ch == "{":
            if depth == 0:
                selector = css[sel_start:i].strip()
                body_start = i + 1
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                if not selector.startswith("@") and ":root" in (
                    part.strip() for part in selector.split(",")
                ):
                    for name, value in re.findall(r"(--[\w-]+):([^;{}]+)", css[body_start:i]):
                        out[name] = value.strip()
                sel_start = i + 1
    return out


# 「宿主的内容」：该拿 game.css 样式、不该被 hikari 的表碰到的元素。
#   - hikari 区域之外的一切；
#   - Modal 的正文里，不在（嵌套的）hikari 组件内的部分；
#   - 带着旧类名渲染的 hikari `Button` 及其内容（见 rust/game2/styles/main.scss）。
# 以后若用别的 hikari 容器组件（ThemeProvider 之类）包住旧版内容，得照 Modal 正文
# 的样子加一项：那类容器自己带 hk- 类，否则里面的旧内容会被当成 hikari 的。
# 这里是唯一的出处：撤销层和令牌规则都由它生成。每一项是一个复合选择器，不能写成
# `a, b` 的列表——后面还要接 `::before`、`:is(html)`，接在列表上只作用于最后一项。
HOST_CONTENT = (
    ':where(*):not([class*="hk-"], [class*="hk-"] *)',
    ":where(.hk-modal-body-inner, .hk-modal-body-inner *):not("
    ':where(.hk-modal-body-inner [class*="hk-"], .hk-modal-body-inner [class*="hk-"] *))',
    ".hk-btn",
    ".hk-btn *",
)


def stylesheet_rules(css: str, outer: tuple[str, ...] = ()):
    """逐条产出样式表里的规则：(选择器, 声明块, 所在的 @media 之类的外层条件)。

    @media / @supports / @layer 这类分组规则会进去继续找；@keyframes 不进。
    只按花括号配对扫，不认引号里的括号。
    """
    css = css.lstrip("\ufeff")  # 压缩产物开头可能是 BOM，strip() 不认它
    depth = 0
    sel_start = 0
    body_start = 0
    selector = ""
    for i, ch in enumerate(css):
        if ch == "{":
            if depth == 0:
                selector = css[sel_start:i].strip()
                body_start = i + 1
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                body = css[body_start:i]
                if selector.startswith(("@media", "@supports", "@layer", "@container")):
                    yield from stylesheet_rules(body, (*outer, selector))
                elif not selector.startswith("@"):
                    yield selector, body, outer
                sel_start = i + 1


def element_level_parts(selector: str) -> list[str]:
    """选择器列表里「元素级」的那些项：不含类、属性、id，只有标签名、`*`、伪类、
    伪元素。这种项会匹配页面里任何同类元素，不限于某个组件。按项判断，不按整条
    规则——hikari 的选区表里有 `input, textarea, select, [contenteditable]:not(…)`
    这样的混合列表，前三项是元素级的，第四项不是。"""
    return [part.strip() for part in selector.split(",") if not re.search(r"[.\[#]", part)]


def element_level_resets(hikari_css: str) -> str:
    """把 hikari 表里「元素级」的规则逐属性 `revert` 掉，只落在宿主的内容上。

    分层只裁决两边都声明的属性；hikari 的元素级规则（`* { box-sizing }`、
    `button { font: inherit }`、`svg { display: block }`……）在宿主样式里没有对手，
    得在这里撤。撤的是 hikari 实际声明的那几个属性，不用 `all: revert`：
    `revert` 会连呈现属性（SVG 的 `fill`、`d`、`stroke-width`……它们在层叠里算
    作者来源）一起退回浏览器默认值，图标就没了。清单从编译产物里读，hikari 升级
    后自动跟上。
    """
    rules: dict[tuple[tuple[str, ...], str], list[str]] = {}
    count = 0
    for selector, body, outer in stylesheet_rules(hikari_css):
        parts = element_level_parts(selector)
        if not parts:
            continue
        names: list[str] = []
        for name, _value in re.findall(r"([-\w]+)\s*:\s*([^;{}]+)", body):
            if not name.startswith("--") and name not in names:  # 自定义属性归令牌规则
                names.append(name)
        if not names:
            continue
        count += 1
        key = (outer, ",".join(parts))
        known = rules.setdefault(key, [])
        known.extend(name for name in names if name not in known)
    if not rules:
        return ""
    out = ["\n@layer hikari-reset{",
           "/* generated by build_tairitsu_poc.py: hikari's element-level rules, property by property, undone on the host's content */\n"]
    for (outer, selector), names in rules.items():
        targets = []
        for part in selector.split(","):
            for host in HOST_CONTENT:
                if part in ("*", "*::before", "*::after"):
                    targets.append(host + part[1:])
                else:
                    targets.append(f"{host}:is({part})")
        cond = "".join(f"{o} " for o in outer)
        out.append(f"{cond}{','.join(targets)}{{{';'.join(f'{n}:revert' for n in names)}}}\n")
    out.append("}\n")
    print(f"[poc] hikari.css: {count} element-level hikari rules undone on the host's content")
    return "".join(out)


def token_restore_rule(game: dict[str, str], hikari: dict[str, str]) -> str:
    """两条令牌规则：hikari 的元素拿 hikari 的 :root 令牌，宿主的内容拿 game.css 的。

    背景见 compile_skin_css：分层后 :root 上撞名的令牌是 game.css 的值。于是：

    - hikari 的元素上把 hikari 的 :root 令牌原样再声明。不只补撞名的那几个——hikari
      还有一批派生令牌（`--c-primary: rgb(var(--color-primary))`）是在 :root 上算好
      了再往下继承的，只补基础令牌，派生的仍是坏值；全部重声明才会重新算。
    - 宿主的内容——Modal 的正文 `.hk-modal-body-inner`、带着旧类名渲染的按钮
      `.hk-btn`——长在 hikari 的元素里面，自定义属性是继承的，所以要把 game.css 的
      :root 令牌在它们上面再声明，切断继承。正文里再嵌的 hikari 组件（自己带 hk- 类）
      由第一条的后半截定回 hikari 的值。

    都未分层、排在皮肤之后：要压得过 :root 上的声明。值从两份编译产物里读，不手抄。
    「宿主内容」的清单见 HOST_CONTENT。
    """
    shared = sorted(name for name in hikari if name in game)
    hikari_body = ";".join(f"{name}:{value}" for name, value in hikari.items())
    game_body = ";".join(f"{name}:{value}" for name, value in game.items())
    host = ":not(.hk-btn,.hk-btn *,.hk-modal-body-inner,.hk-modal-body-inner *)"
    print(
        f"[poc] hikari.css: {len(hikari)} hikari :root tokens re-declared on hikari elements,"
        f" {len(game)} game.css :root tokens on the host's content inside them"
        f" ({len(shared)} names defined by both: {', '.join(shared)})"
    )
    return (
        "\n/* generated by build_tairitsu_poc.py: hikari's :root tokens, re-declared on hikari's own elements */\n"
        f'[class*="hk-"]{host},'
        f".hk-modal-body-inner [class*=\"hk-\"]:not(.hk-btn,.hk-btn *){{{hikari_body}}}\n"
        "/* generated by build_tairitsu_poc.py: game.css's :root tokens, re-declared on the host's content inside hikari */\n"
        f".hk-modal-body-inner,.hk-btn{{{game_body}}}\n"
    )


def compile_skin_css() -> None:
    """编译 tairitsu 版专用皮肤 rust/game2/styles/main.scss → plugin/wasm2/hikari.css。

    页面组件仍与 Dioxus 版共用 game.css；这份皮肤只放「hikari 标准件 → 旧版
    观感」的适配，由 v2 页面排在 game.css 之后加载。

    **还要带上 hikari 组件的样式表本身。** 早先这里只编皮肤，靠 game.css 里
    的旧规则凑合——那时 game2 用到的 hikari 组件恰好都有旧样式兜着。换成上游
    `ToastStore` + `Toast` 之后，容器 `.hk-toast-container` 的定位规则只存在于
    hikari 的总表里，所以这里复用 admin2 构建脚本的组装逻辑（同一份
    `hikari.scss` 基础层 + 全部 vue-mirror 组件表），把组件表接到皮肤前面。

    **组件表整表在 `@layer hikari` 里。** hikari 的表和 game.css 是两套体系，
    同一个元素带着两边的类时（hikari `Button` 带着旧的 `.btn`），谁的规则算数
    不该取决于特异度和文件先后：hikari 的按钮基础规则与旧的 `.btn` 一样只有一个
    类，按先后是后加载的 hikari 赢，悬停态更是要到四个类才压得住。分了层，未分
    层的宿主样式（Discuz、game.css、本皮肤）声明了的属性一律压过分层的 hikari。
    hikari 的元素级规则在宿主样式里没有对手，由 `hikari-reset` 分层逐属性撤掉
    （见 element_level_resets；不能是 hikari 的子层，子层压不过父层）。
    所以组件表和皮肤分两次编译，中间拼，开头先声明层序。

    **hikari 的 :root 令牌在 hikari 元素上再声明一遍。** game.css 与 hikari 都在
    `:root` 上定义了几个同名的自定义属性而含义不同（hikari 的 `--color-primary` 是
    配合 `rgb(var(--color-primary))` 用的 RGB 三元组，game.css 的 `--color-primary`
    是一段渐变）。分层后未分层的 game.css 赢，hikari 组件里用到的那几个就会拿到
    game.css 的值，由它们算出来的派生令牌也跟着坏掉。所以在 hikari 自己的元素上把
    hikari 的令牌整体重新声明——从编译产物里读出来，不手抄。
    """
    # admin2 的构建脚本已经有一份「hikari 组件表怎么组装」的实现；直接借它，
    # 免得两处各维护一遍清单。
    import importlib.util

    admin_script = Path(__file__).resolve().parent / "build_tairitsu_admin.py"
    spec = importlib.util.spec_from_file_location("admin_build_for_poc", admin_script)
    admin = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(admin)

    source = ROOT / "rust" / "game2" / "styles" / "main.scss"
    output = OUT / "hikari.css"
    sass = shutil.which("sass")
    if not sass:
        sys.exit("sass not found on PATH (needed for rust/game2/styles/main.scss)")
    packages = admin.hikari_packages_dir()
    head = [
        line
        for line in (ROOT / "rust" / "admin2" / "styles" / "hikari.scss")
        .read_text(encoding="utf-8")
        .splitlines()
        if line.startswith("@use ") or line.startswith("@import ")
    ]
    mirror = [f'@import "{path}";' for path in admin.hikari_mirror_imports(packages)]
    with tempfile.TemporaryDirectory() as td:
        entry = Path(td) / "game-hikari-entry.scss"
        skin_entry = Path(td) / "game-skin-entry.scss"
        entry.write_text("\n".join(head + mirror) + "\n", encoding="utf-8")
        skin_entry.write_text(source.read_text(encoding="utf-8"), encoding="utf-8")
        parts = []
        for src, name in ((entry, "hikari-base.css"), (skin_entry, "game-skin.css")):
            out = Path(td) / name
            result = subprocess.run(
                [
                    sass, "--style=compressed", "--no-source-map", "--quiet",
                    "--load-path", str(packages),
                    # 组件表里的主题 partial 以裸名 @use（'variables' / 'mixins'）。
                    "--load-path", str(packages / "theme" / "styles"),
                    str(src), str(out),
                ],
                check=False,
                shell=False,
            )
            if result.returncode != 0:
                sys.exit(result.returncode)
            # 压缩模式下 sass 用 BOM 代替 @charset；BOM 只能出现在整个文件开头。
            text = out.read_text(encoding="utf-8")
            parts.append((text.startswith("\ufeff"), text.lstrip("\ufeff")))
    base, skin = parts[0][1], parts[1][1]
    restore = token_restore_rule(
        root_custom_properties((ROOT / "plugin" / "wasm" / "game.css").read_text(encoding="utf-8")),
        root_custom_properties(base),
    )
    output.write_text(
        ("\ufeff" if parts[0][0] or parts[1][0] else "")
        + "@layer hikari,hikari-reset;\n@layer hikari{"
        + base
        + "}\n"
        + skin
        + element_level_resets(base)
        + restore,
        encoding="utf-8",
        newline="\n",
    )
    print(
        f"[poc] hikari.css: {len(head)} base sheets + {len(mirror)} vue-mirror "
        f"sheets + game skin → {output.name} ({output.stat().st_size} bytes)"
    )


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
    compile_skin_css()
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
    verify_glue_exports()
    write_boot_assets()
    print("[poc] done →", OUT)


if __name__ == "__main__":
    main()
