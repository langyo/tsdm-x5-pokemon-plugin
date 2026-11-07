# Dioxus → Tairitsu + Hikari 前端重构：系统研究报告

> 研究日期：2026-11-04。tairitsu @ master `4a93466e`（0.5.26，2026-11-03），hikari @ origin/master `9d571fb4`（npm 0.55.276，crates 0.5.1）。
> 研究对象：`celestia/tairitsu`、`celestia/hikari`、本仓库 `rust/game`（46 个 rs 源文件 / 11,115 行 + 15 个 SCSS / 9,841 行）。

---

## 1. 结论摘要（TL;DR）

1. **可行，且契合度远超预期**。hikari 的 Rust 端 prelude 自述为 "Compatibility layer for migrating from Dioxus to Tairitsu"（`packages/components/src/prelude.rs`），tairitsu 官方甚至带有 Dioxus 迁移指南（`docs/en/guides/migration/dioxus-to-tairitsu.md`）。rsx! 语法形状、`use_signal` + `.read()/.write()`、`MouseData` 类型别名、`dangerous_inner_html` 全部与 Dioxus 兼容或同名。
1. **组件覆盖矩阵几乎全覆盖**（见 §6）：商店宫格/分页/Tooltip/Toast/Modal/Drawer/Tabs/徽标/表单/侧栏/列表/树/右键菜单/战斗日志/地图 scene 家族，hikari 全部有现成或薄封装可用的组件；中文（zh-Hans）内置于 hikari-i18n。
1. **最大技术风险：无 async spawn**。tairitsu 没有 `spawn`/`spawn_local`/wasm-bindgen-futures；`use_resource` 的文档注释自认 "async execution not supported on WASM targets"（`hooks/src/suspense.rs:494`）。而现有 game 前端的所有 API 调用（购买/战斗/刷新）都跑在 `spawn(async move { … })` 上。**这是 POC 第一优先验证项**，候选方案见 §5.2。
1. **构建链完全换血**：wasm32-unknown-unknown + wasm-bindgen → **wasm32-wasip2 组件 + jco 转译**，产物是纯静态文件（JS 模块 + wasm + glue + wasi-shim），可由任意静态服务器（含 PHP 目录）伺服。`examples/website/README.md` 明确演示 `python3 -m http.server` 即可伺服。
1. **嵌入 Discuz 页面的可行路径已找到**，但生成的 index.html 用根绝对路径且硬编码 `#app`——需要手写 boot 脚本（参照 `scripts/build_cdn_demo.py` 的 importmap + 手动装配模式），Rust 侧手动 `get_element_by_id(0, "main") → init_runtime → store_initial_vnode`。
1. **许可证义务（SySL 1.0）**：分发即须携带「含 AI 生成代码」声明 + 模型披露（对象代码分发时须对最终用户可见，如关于页/文档）。这是不可豁免的条款（§2），需要在发布流程里落实。
1. **两库 API 高频漂移**（周级重写、双版本族 0.3→0.5 跳变），**必须以 git rev 精确锁定依赖**。

---

## 2. 框架定位与版本

| | Tairitsu | Hikari |
| --- | --- | --- |
| 定位 | WASM 组件模型全栈框架（VDOM + hooks + rsx! + 编译期 scss!/svg! + SSR + dev/build 工具链） | 同一设计系统双端 UI 库：Rust/WASM（跑在 tairitsu 上）+ Vue3 端 |
| 版本 | 0.5.26（crates 锁步版本；注意 0.5.23 从未发布，消费须 `^0.5.24+`） | crates 0.5.1（2026-10-31）；npm `@celestia-island/hikari` 0.55.276 |
| 活跃度 | 每日提交；13 个 CI 工作流（含视觉回归/浏览器 e2e） | npm 近日更；PR 编号 > #1064；CI 含 wasm32-wasip2 check + vue-mirror 漂移检查 |
| 许可证 | SySL 1.0（Apache-2.0 等效授予 + AI 披露义务；仓库 Cargo.toml 却写 MIT，元数据不一致） | 同 SySL 1.0 |

Hikari 结构：Vue 3 包（`packages/vue`）是**规范源**；Rust 端每个组件文件自称 "mirror of the canonical Vue `Hk*`"，直接 `scss! { file: "src/styles/vue-mirror/...", no_hash }` 编译同一张 SCSS 表进二进制，文件头逐条记录与 Vue 端的"适配差异"（无 Teleport/无焦点陷阱/无自动翻转等）。**读组件头注释就是读文档**（docs/ 目录滞后于代码）。

---

## 3. 现有前端盘点（迁移工作量基线）

- `rust/game`：46 个 rs 文件 / 11,115 行。页面：shop / battle(+battle/) / adventure / `my_pokemon`（最大，约 2,500 行）/ inventory / `pokemon_storage` / `pokemon_center` / home / welcome；组件：layout（sidebar/header）、common（popup/modal/toast/type_badge/icon/pet_marks/loading_overlay/providers）、pokemon（mini_grid/tooltip）；状态：`state/ui.rs`（popup/modal/toast 全局态）+ 各 hooks。
- `rust/admin`：16,845 行 —— **本阶段不动**（管理端保持 Dioxus，待 game 端验证后再议）。
- 样式：15 个 SCSS 组件文件 + variables/mixins/fonts ≈ 9,841 行，含 `$color-*` 主题变量体系 —— 可整体迁移到 tairitsu 的 packager SCSS 管线（grass 编译、`load_paths` 支持），再逐步被 hikari 组件替换。
- 构建：`scripts/publish/build.py --build-wasm`（cargo + wasm-bindgen/dioxus-web）→ 需要新写 tairitsu 构建段（cargo wasm32-wasip2 + jco transpile + glue 装配）。

---

## 4. API 迁移映射（Dioxus 0.7 → Tairitsu）

| Dioxus 0.7 | Tairitsu 0.5.26 | 备注 |
| --- | --- | --- |
| `rsx!` 语法 | 同形：`tag { attr: v, "text", children }`、`if/match/for`、Fragment、字符串插值 `{expr}` | 官方迁移指南 |
| `#[component]` + `#[props(default)]` | `#[component]` + `#[default]` / `#[children]` | props 为 builder 结构体 |
| `use_signal` + `.read()/.write()/.set()` | `use_signal` → `ReactiveSignal`，`.read()`/`.write()`/`.set()` 全兼容 | 无 `.peek()`（get 即克隆） |
| `use_effect/use_memo/use_context` | 同名 | |
| `MouseData` | `pub type MouseData = MouseEvent`，含 `client_x/y` | `onmouseover/oncontextmenu` 不在已知名单——可编译但收到 `Box<dyn EventData>` 需手动 downcast |
| `key:` 属性 | **未接通**：rsx 的 `key:` 只是普通属性；keyed LIS diff 引擎存在但要走 `VElement::new(..).key(..)` 手工构造 | 长列表性能场景需注意 |
| `dangerous_inner_html` | 同名 | 内联 lucide SVG 路线可直接保留；注意 boot 后动态创建的 `<svg>` 需要命名空间修复（glue 有 post-boot fixup，动态节点需自行处理/验证） |
| `onmounted` | 无此事件；`use_dom_ref()`/`use_element_ref`（挂载时回填句柄）+ 平台 `get_bounding_client_rect` 等 | |
| web-sys 直连 | **全部经由 WIT 绑定**（`tairitsu_browser:full` 30+ 域）；窗口尺寸 `inner_width/height`、resize observer、matchMedia 都有 Platform trait | 应用代码也能 `use bindings::tairitsu_browser::full::*` 直达 |
| `spawn(async …)` | **无等价物**（见 §5.2） | 最大缺口 |
| CSS：独立 SCSS 文件 + dioxus 注入 | packager SCSS 管线（`[package.metadata.tairitsu.scss]` entry + load_paths，grass+autoprefixer+minify）或 `scss!`/`include_scss!` 宏（哈希类名/`no_hash`） | 15 个 SCSS 可整体走 packager 管线 |
| `svg!` / SafeSvg | `svg!` 编译期内联 + XSS 消毒 → `VElement::safe_svg()` | |
| dioxus-web 挂载 div | `mount_vnode_to_app` 硬编码 `#app`；或手动 `get_element_by_id(0,"main") → WitElement::from_raw → init_runtime → use_component + store_initial_vnode` | 嵌入场景走手动路线 |

---

## 5. 关键风险与缓解

### 5.1 无 web-sys/wasm-bindgen —— 已定论，按 WIT 世界观写代码

浏览器能力 = `tairitsu_browser:full` 的 30+ WIT 域（`tairitsu-browser-worlds` crate 内嵌，构建无需联网/注册表）。超出域的能力做不了。现有代码里直接 web-sys 的点：`web_sys::window().inner_width/inner_height`（shop tooltip 定位、popup 定位）、`js_sys::eval`（admin 的 lucide——game 端已内联化，无碍）。均有 WIT 等价。

### 5.2 无 async spawn —— **POC 第 0 优先级**

现状：game 前端每个写操作都是 `spawn(async move { api.buy_item(..).await })`。
候选方案（POC 中逐个验证，按优先序）：

1. **JS 回调桥**：自定义一个极小的 host→guest 通道（glue 层做 `fetch` + 回调 set 信号）。研究确认 glue 的 `fetch_text` 是"同步 WIT 调用"——同步 HTTP 会卡 UI 线程，不可取；需要验证 glue 是否有异步完成回调形态，或我们自己在 boot 脚本层加一个 `__PM_FETCH__` JS 桥（fetch → Promise → 回调进组件表 → set signal）。
1. **WIT setTimeout 轮询/poll**：把"future"改成状态机 + 定时器驱动的重试轮询（丑，兜底）。
1. **服务端代理成两段式**：提交即返回 + 轮询结果接口（改 PHP API 面不可取，除非万不得已）。

> 若 (1) 走通，API 层从"async fn"改为"发起 + 回调 set 信号"形态，`spawn(async move {…})` 处全部机械改写。

### 5.3 静态嵌入 Discuz 页面 —— 路径已明确，需手写 boot

- 产物：`<pkg>.wasm` + `component-wrapper/`（jco 产物）+ `component-wrapper-loader.js` + `browser-glue/__tairitsu_glue__.js` + `wasi-shim/*`（本地 bundle，避免 esm.sh 依赖）+ `styles.css`。
- 生成 index.html 的根绝对路径（`/component-wrapper-loader.js` 等）不适配 `/source/plugin/pokemon/wasm/` 子路径 → 参照 `scripts/build_cdn_demo.py` 手写 importmap + boot module；Rust 侧手动挂到 `#main`。
- 构建期 glue 漂移：fresh clone 缺 `packages/browser-glue/dist`（未跟踪的生成物）→ `TAIRITSU_RUNTIME_BUNDLE` 或 npm 安装 `@celestia-island/tairitsu-web-glue`。
- 需要 node + `@bytecodealliance/jco`（`npx --yes` 可用）；`rustup target add wasm32-wasip2`。
- 嵌入后与 Discuz 页面 CSS 共存：hikari 组件自带 vue-mirror SCSS（`no_hash` 全局类 `hk-*`），`ThemeProvider` 输出 CSS 变量；与 Discuz 样式冲突面小但需实测。

### 5.4 Hikari 已知缺口（影响游戏 UI 的）

- Tooltip 无翻转/夹紧（视口边缘会溢出）——商店 tooltip 我们已有自研定位算法，可保留自研版本；
- 无 Teleport/焦点陷阱/z 层带管理 —— **所有 fixed 定位元素的祖先不得有 transform/contain**；
- 无进出场过渡驱动（Modal/Drawer/Toast 静态出现）——需要动效就自己用 `hikari-animation` 驱动；
- 图标仅 MDI（`hikari-icons`）——lucide 风格内联 SVG 继续自绘（本次 PR #102 刚做的 Icon 组件体系可平移）；
- 虚拟滚动仅树形——背包长列表若卡再说；
- 右键菜单 `ContextMenuProvider` 是全受控（open/x/y 信号自己管，行内容自己给）——现有 popup 状态机可平移。

### 5.5 版本漂移

两库周级重构。策略：**git rev 精确锁定**（`{ git = "…", rev = "…" }`），升级作为显式小 PR 走 CI。

### 5.6 许可证（SySL 1.0）

分发（zip = 对象代码分发）须：① 携带"包含 AI 生成代码"声明且对最终用户可见（关于页/随附文档）；② 保留作者模型披露原文或链接；③ 自身改动做 AI/人工披露。建议在游戏"关于"页与发布 zip 的 NOTICE 落实。**需要你确认接受该义务。**

---

## 6. 组件覆盖矩阵（游戏 UI → Hikari）

| 游戏 UI | Hikari 组件 | 判定 |
| --- | --- | --- |
| 主题（蓝主/橙次） | `ThemeProvider`（palette 覆盖 / 自定义 `register_theme`） | 直接用 |
| 侧栏训练师面板 | `Sidebar`/`NavItem`/`NavSidebar` | 直接用 |
| 快捷背包 | `CardList`+`ListRow` | 直接用 |
| 商店宫格 | `SelectionGrid` / `Grid`+`Card` | 直接用 |
| 商店 tooltip | `Tooltip`（无翻转） | 保留自研定位版 |
| 右键菜单 | `ContextMenuProvider`（受控）+ `Menu` 行 | 薄封装 |
| 商店分页 | `Pagination` | 直接用 |
| 背包列表 | `CardList`/`Table` | 直接用 |
| 仓库宫格 | `SelectionGrid`+`DraggableGrid` | 直接用 |
| HP 条 | `ProgressBar` 重肤 + `hikari-animation` | 薄封装 |
| 技能按钮（冷却） | `Button`(shortcut/loading)+`ProgressRing` | 薄封装 |
| 战斗日志 | `LogWindow`（本体）+ 自加窗口 chrome | 薄封装 |
| 冒险地图 | `Board`/`NodeCanvas`/`Minimap`/`ZoomToolbar` | 薄封装（玩法自研） |
| Modal/抽屉/Toast/Tabs/徽标/表单/空态 | 全覆盖 | 直接用 |
| 中文 UI | `hikari-i18n` `Language::ChineseSimplified` + 组件串 | 直接用 |
| 自绘游戏图标 | 无（MDI only） | 自研内联 SVG（已有体系平移） |
| 进出场动效 | 镜像未驱动 | 薄封装/接受静态 |

---

## 7. 分阶段迁移计划

### Phase 1 —— POC（硬风险清零）

新建 `rust/game2`（与现有 game 并行共存，互不干扰）：

- Cargo：tairitsu-vdom/hooks/macros/web(wit-bindings, default-features=false) + hikari-components/icons/palette/i18n，**git rev 锁定**；
- 构建段：build.py 新增 `--build-tairitsu`（cargo wasm32-wasip2 → jco → glue/wasi-shim 装配到 `plugin/wasm2/`）；
- 页面嵌入：game.inc.php 加并行入口（如 `&ui=v2`）挂 `#main`；
- 验证清单（每项都要绿灯）：

  1. 挂载 + hikari `ThemeProvider` + Button/Toast 渲染；
  1. **async 桥**：一次真实 API 往返（登录态下拉 `get::user_info` 类只读接口）；
  1. 信号驱动列表渲染 + rsx `if/for`；
  1. scss 管线出 `styles.css` 且与 Discuz 共存不互污；
  1. 内联 SVG 图标渲染正常（命名空间 fixup 验证）。

- **验收门**：POC 全绿才进入 Phase 2；async 若无解，回全体检讨（带 §5.2 的方案 2/3 或暂缓重构）。

### Phase 2 —— 基础设施层

全局状态（UI store / toast / popup）、API client（回调形态重写）、Icon 组件平移、路由（dioxus Routable → tairitsu router 或自管 signal）、主题变量桥接（`$color-*` → CSS 变量）。

### Phase 3 —— 页面逐个迁移（每页一个 PR）

顺序：**shop（本次痛点、最小闭环）→ welcome/home → inventory → `pokemon_center` → `pokemon_storage` → adventure → `my_pokemon`（最大）→ battle（最复杂，含子组件目录）**。
每页验收：功能等价（对照现有 e2e/浏览器回归）、视觉走查、与旧版并行开关可回退（`&ui=v2` 参数或灰度开关）。

### Phase 4 —— 收尾

删 dioxus/game 旧栈、build.py 移除旧构建段、样式去重、（可选）admin 端同路线评估。
每阶段遵循仓库 AGENTS.md：feat/<name> 分支、gitmoji、3 轮验证、本地门禁、squash PR。

---

## 8. 待确认决策点

1. **POC 先行，OK？**（Phase 1 全绿后再大规模动页面）
1. **范围**：先 game；admin 留 Dioxus，后续再议 —— 是否同意？
1. **SySL 披露义务**：接受并落实（关于页 + zip NOTICE）——是否 OK？
1. **依赖锁定**：tairitsu/hikari 以 git rev 锁定，升级走独立 PR —— 是否 OK？

# Dioxus → Tairitsu + Hikari 前端重构：系统研究报告

> 研究日期：2026-11-04。tairitsu @ master `4a93466e`（0.5.26，2026-11-03），hikari @ origin/master `9d571fb4`（npm 0.55.276，crates 0.5.1）。
> 研究对象：`celestia/tairitsu`、`celestia/hikari`、本仓库 `rust/game`（46 个 rs 源文件 / 11,115 行 + 15 个 SCSS / 9,841 行）。

---

## 1. 结论摘要（TL;DR）

1. **可行，且契合度远超预期**。hikari 的 Rust 端 prelude 自述为 "Compatibility layer for migrating from Dioxus to Tairitsu"（`packages/components/src/prelude.rs`），tairitsu 官方甚至带有 Dioxus 迁移指南（`docs/en/guides/migration/dioxus-to-tairitsu.md`）。rsx! 语法形状、`use_signal` + `.read()/.write()`、`MouseData` 类型别名、`dangerous_inner_html` 全部与 Dioxus 兼容或同名。
1. **组件覆盖矩阵几乎全覆盖**（见 §6）：商店宫格/分页/Tooltip/Toast/Modal/Drawer/Tabs/徽标/表单/侧栏/列表/树/右键菜单/战斗日志/地图 scene 家族，hikari 全部有现成或薄封装可用的组件；中文（zh-Hans）内置于 hikari-i18n。
1. **最大技术风险：无 async spawn**。tairitsu 没有 `spawn`/`spawn_local`/wasm-bindgen-futures；`use_resource` 的文档注释自认 "async execution not supported on WASM targets"（`hooks/src/suspense.rs:494`）。而现有 game 前端的所有 API 调用（购买/战斗/刷新）都跑在 `spawn(async move { … })` 上。**这是 POC 第一优先验证项**，候选方案见 §5.2。
1. **构建链完全换血**：wasm32-unknown-unknown + wasm-bindgen → **wasm32-wasip2 组件 + jco 转译**，产物是纯静态文件（JS 模块 + wasm + glue + wasi-shim），可由任意静态服务器（含 PHP 目录）伺服。`examples/website/README.md` 明确演示 `python3 -m http.server` 即可伺服。
1. **嵌入 Discuz 页面的可行路径已找到**，但生成的 index.html 用根绝对路径且硬编码 `#app`——需要手写 boot 脚本（参照 `scripts/build_cdn_demo.py` 的 importmap + 手动装配模式），Rust 侧手动 `get_element_by_id(0, "main") → init_runtime → store_initial_vnode`。
1. **许可证义务（SySL 1.0）**：分发即须携带「含 AI 生成代码」声明 + 模型披露（对象代码分发时须对最终用户可见，如关于页/文档）。这是不可豁免的条款（§2），需要在发布流程里落实。
1. **两库 API 高频漂移**（周级重写、双版本族 0.3→0.5 跳变），**必须以 git rev 精确锁定依赖**。

---

## 2. 框架定位与版本

| | Tairitsu | Hikari |
| --- | --- | --- |
| 定位 | WASM 组件模型全栈框架（VDOM + hooks + rsx! + 编译期 scss!/svg! + SSR + dev/build 工具链） | 同一设计系统双端 UI 库：Rust/WASM（跑在 tairitsu 上）+ Vue3 端 |
| 版本 | 0.5.26（crates 锁步版本；注意 0.5.23 从未发布，消费须 `^0.5.24+`） | crates 0.5.1（2026-10-31）；npm `@celestia-island/hikari` 0.55.276 |
| 活跃度 | 每日提交；13 个 CI 工作流（含视觉回归/浏览器 e2e） | npm 近日更；PR 编号 > #1064；CI 含 wasm32-wasip2 check + vue-mirror 漂移检查 |
| 许可证 | SySL 1.0（Apache-2.0 等效授予 + AI 披露义务；仓库 Cargo.toml 却写 MIT，元数据不一致） | 同 SySL 1.0 |

Hikari 结构：Vue 3 包（`packages/vue`）是**规范源**；Rust 端每个组件文件自称 "mirror of the canonical Vue `Hk*`"，直接 `scss! { file: "src/styles/vue-mirror/...", no_hash }` 编译同一张 SCSS 表进二进制，文件头逐条记录与 Vue 端的"适配差异"（无 Teleport/无焦点陷阱/无自动翻转等）。**读组件头注释就是读文档**（docs/ 目录滞后于代码）。

---

## 3. 现有前端盘点（迁移工作量基线）

- `rust/game`：46 个 rs 文件 / 11,115 行。页面：shop / battle(+battle/) / adventure / `my_pokemon`（最大，约 2,500 行）/ inventory / `pokemon_storage` / `pokemon_center` / home / welcome；组件：layout（sidebar/header）、common（popup/modal/toast/type_badge/icon/pet_marks/loading_overlay/providers）、pokemon（mini_grid/tooltip）；状态：`state/ui.rs`（popup/modal/toast 全局态）+ 各 hooks。
- `rust/admin`：16,845 行 —— **本阶段不动**（管理端保持 Dioxus，待 game 端验证后再议）。
- 样式：15 个 SCSS 组件文件 + variables/mixins/fonts ≈ 9,841 行，含 `$color-*` 主题变量体系 —— 可整体迁移到 tairitsu 的 packager SCSS 管线（grass 编译、`load_paths` 支持），再逐步被 hikari 组件替换。
- 构建：`scripts/publish/build.py --build-wasm`（cargo + wasm-bindgen/dioxus-web）→ 需要新写 tairitsu 构建段（cargo wasm32-wasip2 + jco transpile + glue 装配）。

---

## 4. API 迁移映射（Dioxus 0.7 → Tairitsu）

| Dioxus 0.7 | Tairitsu 0.5.26 | 备注 |
| --- | --- | --- |
| `rsx!` 语法 | 同形：`tag { attr: v, "text", children }`、`if/match/for`、Fragment、字符串插值 `{expr}` | 官方迁移指南 |
| `#[component]` + `#[props(default)]` | `#[component]` + `#[default]` / `#[children]` | props 为 builder 结构体 |
| `use_signal` + `.read()/.write()/.set()` | `use_signal` → `ReactiveSignal`，`.read()`/`.write()`/`.set()` 全兼容 | 无 `.peek()`（get 即克隆） |
| `use_effect/use_memo/use_context` | 同名 | |
| `MouseData` | `pub type MouseData = MouseEvent`，含 `client_x/y` | `onmouseover/oncontextmenu` 不在已知名单——可编译但收到 `Box<dyn EventData>` 需手动 downcast |
| `key:` 属性 | **未接通**：rsx 的 `key:` 只是普通属性；keyed LIS diff 引擎存在但要走 `VElement::new(..).key(..)` 手工构造 | 长列表性能场景需注意 |
| `dangerous_inner_html` | 同名 | 内联 lucide SVG 路线可直接保留；注意 boot 后动态创建的 `<svg>` 需要命名空间修复（glue 有 post-boot fixup，动态节点需自行处理/验证） |
| `onmounted` | 无此事件；`use_dom_ref()`/`use_element_ref`（挂载时回填句柄）+ 平台 `get_bounding_client_rect` 等 | |
| web-sys 直连 | **全部经由 WIT 绑定**（`tairitsu_browser:full` 30+ 域）；窗口尺寸 `inner_width/height`、resize observer、matchMedia 都有 Platform trait | 应用代码也能 `use bindings::tairitsu_browser::full::*` 直达 |
| `spawn(async …)` | **无等价物**（见 §5.2） | 最大缺口 |
| CSS：独立 SCSS 文件 + dioxus 注入 | packager SCSS 管线（`[package.metadata.tairitsu.scss]` entry + load_paths，grass+autoprefixer+minify）或 `scss!`/`include_scss!` 宏（哈希类名/`no_hash`） | 15 个 SCSS 可整体走 packager 管线 |
| `svg!` / SafeSvg | `svg!` 编译期内联 + XSS 消毒 → `VElement::safe_svg()` | |
| dioxus-web 挂载 div | `mount_vnode_to_app` 硬编码 `#app`；或手动 `get_element_by_id(0,"main") → WitElement::from_raw → init_runtime → use_component + store_initial_vnode` | 嵌入场景走手动路线 |

---

## 5. 关键风险与缓解

### 5.1 无 web-sys/wasm-bindgen —— 已定论，按 WIT 世界观写代码

浏览器能力 = `tairitsu_browser:full` 的 30+ WIT 域（`tairitsu-browser-worlds` crate 内嵌，构建无需联网/注册表）。超出域的能力做不了。现有代码里直接 web-sys 的点：`web_sys::window().inner_width/inner_height`（shop tooltip 定位、popup 定位）、`js_sys::eval`（admin 的 lucide——game 端已内联化，无碍）。均有 WIT 等价。

### 5.2 无 async spawn —— **POC 第 0 优先级**

现状：game 前端每个写操作都是 `spawn(async move { api.buy_item(..).await })`。
候选方案（POC 中逐个验证，按优先序）：

1. **JS 回调桥**：自定义一个极小的 host→guest 通道（glue 层做 `fetch` + 回调 set 信号）。研究确认 glue 的 `fetch_text` 是"同步 WIT 调用"——同步 HTTP 会卡 UI 线程，不可取；需要验证 glue 是否有异步完成回调形态，或我们自己在 boot 脚本层加一个 `__PM_FETCH__` JS 桥（fetch → Promise → 回调进组件表 → set signal）。
1. **WIT setTimeout 轮询/poll**：把"future"改成状态机 + 定时器驱动的重试轮询（丑，兜底）。
1. **服务端代理成两段式**：提交即返回 + 轮询结果接口（改 PHP API 面不可取，除非万不得已）。

> 若 (1) 走通，API 层从"async fn"改为"发起 + 回调 set 信号"形态，`spawn(async move {…})` 处全部机械改写。

### 5.3 静态嵌入 Discuz 页面 —— 路径已明确，需手写 boot

- 产物：`<pkg>.wasm` + `component-wrapper/`（jco 产物）+ `component-wrapper-loader.js` + `browser-glue/__tairitsu_glue__.js` + `wasi-shim/*`（本地 bundle，避免 esm.sh 依赖）+ `styles.css`。
- 生成 index.html 的根绝对路径（`/component-wrapper-loader.js` 等）不适配 `/source/plugin/pokemon/wasm/` 子路径 → 参照 `scripts/build_cdn_demo.py` 手写 importmap + boot module；Rust 侧手动挂到 `#main`。
- 构建期 glue 漂移：fresh clone 缺 `packages/browser-glue/dist`（未跟踪的生成物）→ `TAIRITSU_RUNTIME_BUNDLE` 或 npm 安装 `@celestia-island/tairitsu-web-glue`。
- 需要 node + `@bytecodealliance/jco`（`npx --yes` 可用）；`rustup target add wasm32-wasip2`。
- 嵌入后与 Discuz 页面 CSS 共存：hikari 组件自带 vue-mirror SCSS（`no_hash` 全局类 `hk-*`），`ThemeProvider` 输出 CSS 变量；与 Discuz 样式冲突面小但需实测。

### 5.4 Hikari 已知缺口（影响游戏 UI 的）

- Tooltip 无翻转/夹紧（视口边缘会溢出）——商店 tooltip 我们已有自研定位算法，可保留自研版本；
- 无 Teleport/焦点陷阱/z 层带管理 —— **所有 fixed 定位元素的祖先不得有 transform/contain**；
- 无进出场过渡驱动（Modal/Drawer/Toast 静态出现）——需要动效就自己用 `hikari-animation` 驱动；
- 图标仅 MDI（`hikari-icons`）——lucide 风格内联 SVG 继续自绘（本次 PR #102 刚做的 Icon 组件体系可平移）；
- 虚拟滚动仅树形——背包长列表若卡再说；
- 右键菜单 `ContextMenuProvider` 是全受控（open/x/y 信号自己管，行内容自己给）——现有 popup 状态机可平移。

### 5.5 版本漂移

两库周级重构。策略：**git rev 精确锁定**（`{ git = "…", rev = "…" }`），升级作为显式小 PR 走 CI。

### 5.6 许可证（SySL 1.0）

分发（zip = 对象代码分发）须：① 携带"包含 AI 生成代码"声明且对最终用户可见（关于页/随附文档）；② 保留作者模型披露原文或链接；③ 自身改动做 AI/人工披露。建议在游戏"关于"页与发布 zip 的 NOTICE 落实。**需要你确认接受该义务。**

---

## 6. 组件覆盖矩阵（游戏 UI → Hikari）

| 游戏 UI | Hikari 组件 | 判定 |
| --- | --- | --- |
| 主题（蓝主/橙次） | `ThemeProvider`（palette 覆盖 / 自定义 `register_theme`） | 直接用 |
| 侧栏训练师面板 | `Sidebar`/`NavItem`/`NavSidebar` | 直接用 |
| 快捷背包 | `CardList`+`ListRow` | 直接用 |
| 商店宫格 | `SelectionGrid` / `Grid`+`Card` | 直接用 |
| 商店 tooltip | `Tooltip`（无翻转） | 保留自研定位版 |
| 右键菜单 | `ContextMenuProvider`（受控）+ `Menu` 行 | 薄封装 |
| 商店分页 | `Pagination` | 直接用 |
| 背包列表 | `CardList`/`Table` | 直接用 |
| 仓库宫格 | `SelectionGrid`+`DraggableGrid` | 直接用 |
| HP 条 | `ProgressBar` 重肤 + `hikari-animation` | 薄封装 |
| 技能按钮（冷却） | `Button`(shortcut/loading)+`ProgressRing` | 薄封装 |
| 战斗日志 | `LogWindow`（本体）+ 自加窗口 chrome | 薄封装 |
| 冒险地图 | `Board`/`NodeCanvas`/`Minimap`/`ZoomToolbar` | 薄封装（玩法自研） |
| Modal/抽屉/Toast/Tabs/徽标/表单/空态 | 全覆盖 | 直接用 |
| 中文 UI | `hikari-i18n` `Language::ChineseSimplified` + 组件串 | 直接用 |
| 自绘游戏图标 | 无（MDI only） | 自研内联 SVG（已有体系平移） |
| 进出场动效 | 镜像未驱动 | 薄封装/接受静态 |

---

## 7. 分阶段迁移计划

### Phase 1 —— POC（硬风险清零）

新建 `rust/game2`（与现有 game 并行共存，互不干扰）：

- Cargo：tairitsu-vdom/hooks/macros/web(wit-bindings, default-features=false) + hikari-components/icons/palette/i18n，**git rev 锁定**；
- 构建段：build.py 新增 `--build-tairitsu`（cargo wasm32-wasip2 → jco → glue/wasi-shim 装配到 `plugin/wasm2/`）；
- 页面嵌入：game.inc.php 加并行入口（如 `&ui=v2`）挂 `#main`；
- 验证清单（每项都要绿灯）：

  1. 挂载 + hikari `ThemeProvider` + Button/Toast 渲染；
  1. **async 桥**：一次真实 API 往返（登录态下拉 `get::user_info` 类只读接口）；
  1. 信号驱动列表渲染 + rsx `if/for`；
  1. scss 管线出 `styles.css` 且与 Discuz 共存不互污；
  1. 内联 SVG 图标渲染正常（命名空间 fixup 验证）。

- **验收门**：POC 全绿才进入 Phase 2；async 若无解，回全体检讨（带 §5.2 的方案 2/3 或暂缓重构）。

### Phase 2 —— 基础设施层

全局状态（UI store / toast / popup）、API client（回调形态重写）、Icon 组件平移、路由（dioxus Routable → tairitsu router 或自管 signal）、主题变量桥接（`$color-*` → CSS 变量）。

### Phase 3 —— 页面逐个迁移（每页一个 PR）

顺序：**shop（本次痛点、最小闭环）→ welcome/home → inventory → `pokemon_center` → `pokemon_storage` → adventure → `my_pokemon`（最大）→ battle（最复杂，含子组件目录）**。
每页验收：功能等价（对照现有 e2e/浏览器回归）、视觉走查、与旧版并行开关可回退（`&ui=v2` 参数或灰度开关）。

### Phase 4 —— 收尾

删 dioxus/game 旧栈、build.py 移除旧构建段、样式去重、（可选）admin 端同路线评估。
每阶段遵循仓库 AGENTS.md：feat/<name> 分支、gitmoji、3 轮验证、本地门禁、squash PR。

---

## 8. 待确认决策点

1. **POC 先行，OK？**（Phase 1 全绿后再大规模动页面）
1. **范围**：先 game；admin 留 Dioxus，后续再议 —— 是否同意？
1. **SySL 披露义务**：接受并落实（关于页 + zip NOTICE）——是否 OK？
1. **依赖锁定**：tairitsu/hikari 以 git rev 锁定，升级走独立 PR —— 是否 OK？

---

## 9. Phase 1 POC 实施记录（2026-11-04，分支 feat/tairitsu-poc）

### 已验证通过

| 清单项 | 结果 |
| --- | --- |
| wasm32-wasip2 组件构建 + jco 转译 + 静态装配 | ✅ scripts/publish/build_tairitsu_poc.py |
| 嵌入 Discuz 页面（Caddy 白名单加 wasm2） | ✅ /source/plugin/pokemon/wasm2/poc.html |
| hikari ThemeProvider/Button 渲染（品牌色覆盖） | ✅ |
| 信号 → 重渲染 → diff → patch → DOM 全链路 | ✅（最小计数器 + 文本 span 更新） |
| fetch 回调桥（fetchPromise → promiseCallbacks → set 信号） | ✅ |

### 已定位的问题（阻塞列表类更新）

1. **上游 diff/apply 语义缺口**：列表追加场景 diff 产出
   `UpdateChild{index:N, [InsertChild{index:N}]}`（N == 旧子节点数，越界），
   apply 的 `get_child_at` 对越界索引返回 None 后**静默跳过**整段嵌套 patch，
   DOM 无任何变化（文本更新不受影响）。已在干净构建中用 `MutationObserver` +
   全链路探针复现。需上游修复或本地 workaround（见下）。
1. **npm glue(0.5.4) 与 0.5.26 组件的命名错位**：glue 回连
   on_animation_frame/on_timeout/on_promise_resolve（snake/旧名），组件导出
   实为 onFrame/onTimeout/onPromiseResolved（kebab→camel 新约定）。
   POC 已在装配层做别名代理修正（poc.html 内 CALLBACK_ALIASES）。
1. **glue 缺口补齐**：node.`getChildNodes`、node-list 接口整体、
   platform-helpers.`fetchPromise`（签名失配：传 Response 而非 String）
   均为装配层手写补齐（build_tairitsu_poc.py shims 段）。

### 关键实现要点（迁移时必须遵守）

- **状态模式**：tairitsu 的 `use_signal` 每次渲染创建新信号（受控组件设计）。
  持久状态必须：注册组件拿 id → `with_component(id, || 创建状态)` →
  `update_render_function(id, 闭包)` 共享同一组信号 → 经 props 传入组件。
  渲染闭包外创建的信号其 `mark_dirty` 会落到兜底空组件（静默失效）。
- **rsx 文本**：仅支持 `"{ident}"` 整串占位符简写；混合文本（`"前缀 {x}"`）
  不插值，需在组件体内 format! 后再以整串占位符渲染。
- **后台标签页 rAF 不触发**：自动化测试需以 setTimeout 模拟帧驱动；
  前台真实使用不受影响。
- **调试**：tracing 在 wasm 无订阅者，`apply_patches` 等失败完全静默；
  需要 eprintln 探针（stderr → console.error）。git rev 锁定时 checkout
  内改动会被 cargo 还原，调试需临时切 path 依赖。

### 追加发现（PR #117 验证过程中确认的上游缺陷）

1. **调度非确定性**：`ReactiveSignal`::set → `mark_dirty` → schedule 回调在部分
   页面加载中不触发（stderr 探针证实 set 完成但 schedule 未执行），疑似
   flush_render/rt.scheduled 状态机问题；同构建不同加载表现不一致。
1. **嵌套 `InsertChild` 丢失**：即使 flush+apply 携带完整 patch
   （span UpdateText 落地 ✓），ul 的 `InsertChild`{2, li} 不产生 DOM 变更
   （原生 createElement/appendChild 包装计数为 0）。疑似 apply 递归下降
   对 Fragment/深层子树索引解析仍有缺口。
以上两项需在上游用 browser e2e（scripts/browser_e2e.py + shirabe）建立
「挂载→点击→断言 DOM」回归用例后修复，消费侧无法绕过。

补充判定（后台自动化标签的 rAF 节流会造成冻结假象）：前台可见标签页 +
原生 rAF 下，span 的 `DynamicText` effect 更新实时生效；列表追加在根元素
修正 + glue 修复齐备后亦曾观测到成功（`MutationObserver` 记录 +1 li）。
结论：页面在用户真实前台会话中可用；后台冻结问题建议上游给 flush 加
setTimeout 兜底调度（game2 的 schedule 回调已改为 timer 驱动作验证）。

### 下一步（Phase 1 收尾选项）

A. 向上游反馈 diff/apply 缺口（附复现），期间游戏列表用「整棵重挂载」
workaround（`set_text_content`(app,'') + `mount_vnode_to_app` 全量重渲染，
游戏规模可接受）。
B. 【已执行】向 tairitsu 提交修复 PR：
celestia-island/tairitsu#116（fix/init-runtime-rendered-root 分支）。
根因实为 `init_runtime` 根元素接线错误：patch 相对「组件 vnode 根」计算，
文档/示例/模板却传挂载父级 #app，顶层索引全部错位一层（首条撞巧命中、
其余静默丢弃）。修复 = `mount_vnode_to_app` 返回渲染后的根元素，
示例/模板/迁移文档统一改为以该根调用 `init_runtime`。
合入后插件侧的「列表更新失效」即随之消失，届时重验 POC 清单即可。
C. 暂缓列表类页面，先迁无列表页面（welcome/home）验证范式后随上游修复推进。

---

## 10. Phase 2：列表不增长的真正根因与上游修复

Phase 1 记录的「列表追加 patch 被吞」有一个更深的原因，不是 `init_runtime` 的
根元素接线，而是 **Fragment 的 vnode 形状与 DOM 形状不一致**：

- `rsx!` 把 `for` / `if` 的产物包成 `VNode::Fragment(...)`，作为元素的
  **一个** child 存进 `VElement::children`；
- 渲染器（`render_vnode`）、HTML 序列化（`write_html`）都把 Fragment 当透明
  容器，把成员直接 append 到父元素；
- diff 按 `VElement::children` 的下标算 patch，apply 却按 **DOM** 的下标去取
  节点。

于是只要出现一个 Fragment，它之后的所有下标都错位一格，patch 落到错误节点，
或取到 `None` 被静默丢弃。这正是「列表点几下不动」的根因，也解释了为什么
整树重挂载能绕过它（重挂载不经过 patch 下标）。

已在 `celestia-island/tairitsu#119` 修复（`VElement::child/children` 递归
展平 Fragment），同一 PR 还修了三个会让「一次点击永久失灵」的调度缺陷：

1. **丢一帧 = 永久冻结**：`scheduled` 只在 `flush_render` 里复位，帧一旦没
   送达（后台标签页节流、animation callback id 丢失、宿主中途换调度器），
   之后每次 `mark_dirty` 都在 `AlreadyScheduled` 提前返回，更新被静默丢弃。
   现在 booking 带请求时间，超过 `FRAME_DEADLINE`（32ms）就当丢失并直接
   flush，损失上界是一帧而不是整个会话。
1. **依赖列表无上限增长**：`track_signal` / `subscribe_component` 每次渲染
   都 push 一次组件 id，而 `notify_signal` 会遍历它——一次写入会触发
   O(已渲染次数) 次冗余渲染。两处现已去重。
1. **`flush_render` 只渲染一轮**：渲染期间被标脏的组件留在队列里，
   `scheduled` 已清空、无人再调度它们。现在循环排空队列。

### 10.1 迁移期采用的调度策略

插件侧（`rust/game2/src/lib.rs`）当前用**同步 flush**作为 schedule callback，
等价于 Dioxus 的同步重渲染。原因：上面 1 的根因在消费侧无法绕过，而
rAF 与 `TimerOps::set_timeout` 两条异步路径在本机实测都观察不到回调到达
（`onFrame` / `onTimeout` 均未被调用），只有 `FRAME_DEADLINE` 兜底能让更新
生效——表现为「点几下不动，过一会儿一次性全冒出来」。

同步 flush 行为确定，游戏单屏元素量（20–50 个瓦片）下开销可接受。
上游 #119 合入后应改回异步批处理，并补一条「点击 → 断言 DOM」的浏览器回归。

### 10.1.2 「三条投递链路全哑」的最终根因与全链路打通

§10.1.1 记录的 rAF / setTimeout / promise 三条回调链路「全部不达」，根因
**不是** wasm 重入限制，而是两层叠加：

1. **旧版 glue 的浏览器缓存**。shim→glue 的相对导入（`../glue/platform/index.js`）
   没带版本参数，浏览器一直复用 npm tarball 0.5.4 的旧 glue——它回连的是
   `snake_case` 旧回调名（`on_timeout` / `on_animation_frame` /
   `on_promise_resolve`），经 `?.` 可选链**全部静默无操作**。三条链路同时哑、
   且不报任何错，正是这个特性。构建脚本已给 shim→glue 导入与 wasi
   importmap 全部加 `?v=<构建戳>`，glue-core 单独用自己的 mtime 打戳。
1. **apply 路径残留的 eprintln 探针**（上游 #116 带进 master 的调试代码，
   #120/#119 移除）。wasm 的 stderr 是 wasi:cli 的 `OutputStream` **资源**，
   绑定在首次解析它的线程上；从 onFrame 的回调线程写 stderr 直接抛
   "Not a valid `OutputStream` resource"，把 `InsertChild` 的 apply 中途打断——
   而 Phase 3 已把新 vnode 存进 `component_vnodes`，后续 diff 全部视为无变化，
   UI 永久冻结且无任何报错。这也解释了 §10.1 时代「点几下不动、过一会儿
   一次性冒出来」：只有 FRAME_DEADLINE 兜底在 32ms 后把积压刷出。

另有两处独立缺陷一并修复（均在 tairitsu#119）：

- **重入 flush 双借用**：apply 回调内部的 `mark_dirty` 兜底路径会再次进入
  `flush_render`，嵌套 drain 重入 `render_component` 时撞上外层 apply 回调的
  RefCell 借用（"RefCell already borrowed"），unwind 后存储 vnode 已被污染。
  现在 flush 带重入守卫（RAII），嵌套调用让位给外层排空循环。
- **监听器每渲染叠一个**：`UpdateEvent` 补丁是 remove+add，remove 半路被吞时
  add 仍执行，DOM 上叠出 N 个监听（实测一个按钮 7 个 click），第 N 次点击
  触发 N 次 handler。上游 `add_event_listener` 现在先撤同 (element, event)
  的旧条目；消费侧 glue-core 的 `__listenerHandles` 也改为唯一表 + DOM 级
  驱逐（插入新条目时 detach 旧 DOM 闭包），双保险。
- **句柄幂等**（glue-core）：`__storeElement/__storeNode` 加 WeakMap 反查，
  同一 DOM 节点永远解析回同一句柄——否则 wasm 侧「元素身份」每次都变，
  监听去重全部失配。

**验证（真实浏览器 + Discuz 页面 + 异步 rAF 调度）**：连续 5 次点击每次
恰好 +1 行（2→3→4→5→6→7，增量 patch，无整树重挂载）；一次点击恰好一次
请求；fetch 异步往返打通（状态文本更新为「OK（163 字节）」）；rAF 帧投递
7/7；信号值与 DOM 行数一致无漂移。§10.1 的同步 flush 绕行已移除，恢复
异步批处理调度。

panic 上报注意：wasm 的 eprintln 在 jco 隐式线程里不可用（stderr 资源跨线程
无效，panic hook 里的 eprintln 自身会崩并把真 panic 吞掉）。game2 的 panic
hook 先写 `#app[data-panic]` 再尝试 stderr，任何 panic 都能在 DOM 上读到。

### 10.1.3 商店页移植与 jco 任务/锁机制的边界

商店页（shop.rs，首个真实页面）移植完成后，交互主循环在**带数据的树**上
暴露出 jco（componentize-js 运行时）任务机制的更深边界，与 POC（纯内存
列表）表现不同：

**已确认的事实（全部带 DOM 探针证据）**：

1. promise 回调线程里发起的**异步导入**（`request-animation-frame` 等）
   会被 jco 排队而非执行；一旦锁账本损坏（日志可见
   `exclusiveRelease ignoring foreign release`），排队操作永不落地——
   表现为 rAF 预订「完成」但 `window.requestAnimationFrame` 从未被调用。
1. promise 回调线程里直接 `flush_render()` 会在渲染路径的某次接口调用上
   **永久挂起**（排空标志卡 true，后续所有 flush 被重入守卫推迟）。
1. 同步 DOM 写入（`set-attribute`）在任何隐式线程都可靠。
1. 事件任务（DOM 事件处理器）上下文渲染**空树**（loading 态）完整可行。

**当前采用的调度设计（双通道）**：

- 事件/挂载线程 → 经典 rAF 预订（POC 验证过的路径）。
- promise 线程回调 → `Signal::inner().set()`（纯内存，不触发 `mark_dirty`
  的调度路径）+ `mark_dirty_deferred`（入队不调度）+ 写
  `#app[data-flush-pending]`（同步导入）；boot.js 的 `MutationObserver`
  看到后向 #app 派发 `pm-flush` 自定义事件，lib.rs 注册的监听器在事件
  任务里执行 `flush_render()`。

**剩余卡点（下一会话从这里开始）**：pm-flush 事件正常送达（on-generic-event
有完整的 enter/Return），但带数据（8 个瓦片）的重渲染仍未落地——dirty 在
队列里、监听器命中、事件返回，DOM 不变。怀疑点按优先级：

1. 渲染路径中某个特定接口调用在事件任务上仍被排队（用 JCO_DEBUG 对比
   挂载期 flush（成功）与 pm-flush flush（失败）的日志行差集可定位）；
1. `exclusiveRelease ignoring foreign release` 的锁账本损坏来源——检查
   jco wrapper 的 `exclusiveAcquire/Release` 配对与 promise 任务的
   task id 归属；
1. 规避方案：把整个渲染搬到 JS 侧不可行；可尝试 componentize-js 新版本
   或 `tairitsu-web` 增加「整树重挂载」降级 API（POC 时代验证过
   `mount_vnode_to_app` 在挂载任务上对任意规模树都可靠）。

上游配套修复（本轮，tairitsu#119 分支）：

- `mark_dirty` 的过期检测从 `Instant::now()`（wasi:clocks 同步导入，
  在 promise 线程会陷阱致死）改为**标记计数**（OVERDUE_MARKS=8，纯内存）；
- `flush_render` 增加 DEBUG_FLUSHES/DEBUG_DEFERRED_FLUSHES 原子计数与
  `debug_flush_counts()`，配合 `debug_state()` 把「调度未到/排空挂起/
  队列孤儿」三种冻结形态分开。

### 10.1.3 glue `getParentElement` 补丁的退役复查点

`scripts/publish/build_tairitsu_poc.py` 里的 `patch_glue()` 修的是 vendored
glue 的一个缺陷：`getParentElement` 用「元素对象」当 key 去查
`__elementHandles`——那张表的键是 bigint 句柄，元素当 key 永远查不到，
parent 链第一跳就断。事件委托全靠向上找祖先属性，所以这个缺陷会让
所有 `data-pm-*` 委托失效、点任何按钮都没反应。

本地修法：改走 glue-core 维护的 `__nodeIdentity`（WeakMap 元素 → 句柄）。
上游修法在 `celestia-island/tairitsu#124`，等价但用的是 `__storeNode`——
两者都是「元素 → 句柄」反查，只是辅助函数不同。**因此上游版落地后必须
删掉本地补丁，不能并存**（并存会让同一处代码被改两次，第二次替换不再命中，
且阅读时容易误判实际生效的是哪一版）。

**复查步骤**（三步都满足才可退役）：

1. `gh pr view 124 --repo celestia-island/tairitsu --json state,mergedAt`
   → 期望 `state=MERGED` 且 `mergedAt` 非空；
2. `gh release list --repo celestia-island/tairitsu --limit 3`
   → 期望出现晚于合并时间的新 release；
3. 升级 vendor 后 `grep '__elementHandles.get(t)' plugin/wasm2/glue/dom/index.js`
   → 期望不再命中（说明上游实现已就位）。

退役动作：删除 `patch_glue()` 及其两处调用
（`build_tairitsu_poc.py` 主流程、`build_tairitsu_admin.py`），重建产物，
然后重跑八路由后台回归 + `scripts/verify_game2.mjs`——事件委托全链路都
依赖 parent 链，回归必须覆盖到点击行为，不能只看页面能渲染。

**复查记录**：

| 日期 | PR 状态 | mergedAt | 最新 release | 结论 |
| --- | --- | --- | --- | --- |
| 2026-11-07 | OPEN | null | v0.5.26（2026-11-03） | 受上游阻塞，补丁保留 |
| 2026-11-07（同日晚些） | MERGED（squash `84ca3f7`） | 2026-11-07 | 未发版 | **补丁已退役**（见下） |

**退役执行记录（2026-11-07）**：PR 原分支基于 #123 之前的 master，直接
合并会把 #123 的 workflow 触发策略收缩整个拖回去（mergeStateStatus 只保证
文本无冲突）。因此先用 worktree 把 PR 的代码部分（glue-dom.ts + 测试）
rebase 到当前 master——顺带给测试脚本加了 `--experimental-strip-types`，
让 `npm run test` 在 Node 22 也能直接跑（14/14 过）——强推后 squash 合入
（`84ca3f7`）。随后从合并后的 master 构建 glue dist（33/33 域）、
走 `vendor_copy` 完整刷新 vendor，确认
`plugin/wasm2/glue/dom/index.js` 的 `getParentElement` 已走 `__storeNode`
且 `__elementHandles.get(t)` 不再命中，删除 `patch_glue()` 及两处调用并
重建产物。八路由后台回归与 `verify_game2.mjs`（71 通过 / 0 失败 / 1 跳过）
在新 glue 上全绿。

### 10.1.4 diff/patch 通道的工程化尝试与三重阻断（2026-11-07）

整树重挂是当前架构的最大性能税（最坏 1126 行弹窗开关 776-843ms）。本轮尝试切到
框架原生响应式管线（signal.set → notify_signal → mark_dirty → flush_render →
diff → apply_patches），接线全部正确（初始渲染在组件上下文内注册依赖、
apply_patches 挂到 vnode 根元素、schedule 回调与 pm-flush 桥就位），实测被 jco
wrapper 层三重阻断：

1. **rAF→wasm 不存在**：wrapper（jco 转译产物）未导出
   `onAnimationFrame`（`onTimeout`/`onPromiseResolve`/`onGenericEvent`/
   `onMouseEvent` 都有，唯独缺它）——`set_schedule_callback` 的 rAF 路径
   整条死路，这是 §10.1.1「三条投递链路全哑」的精确根因。
2. **promise 任务上 flush 卡死**：fetch 回调（promise 任务）里执行
   `flush_render` 后 `FLUSHING` 守卫永不复位（`debug_state()` 实测
   `true|true|1|1`），之后所有 flush 被静默跳过——即 §10.1.2 遗留清单
   「渲染路径中某个特定接口调用在事件任务上仍被排队」的可复现形态。
3. **事件任务重入一次后失活**：观察器→pm-flush→事件任务 flush 的第一
   次投递成功（render+apply 各 1 次），之后 MutationObserver 与 pm-flush
   监听器双双失活（JS 侧手动 setAttribute / dispatchEvent 均不再触发
   wasm 回调），wasm 实例疑似在首次重入中被毒化。

工作副本已回退到整树重挂（回归全绿）。**解锁 diff 路径需要上游动作**：
jco wrapper 补 `onAnimationFrame` 导出、修 promise 任务的渲染重入、或提供
显式的「帧调度导出」。这是 tairitsu 作为正式工程化框架（对标 Vue 生态）
的关键缺口——通用接口问题（wasm component）已解决，任务调度/回调投递
的可靠性是下一道门槛。

### 10.1.5 §10.1.4 勘误与 diff/patch 管线落地（2026-11-08）

**勘误**：§10.1.4 的「三重阻断」结论部分错误。复核 wrapper（jco 转译产物）
与 glue 后：

1. 「wrapper 未导出 onAnimationFrame」——**grep 错名**。WIT 的导出名是
   `on-frame`，jco 转译为 `onFrame`（实测 wrapper 内 11 处），glue 派发
   `exp.onFrame`，整条 rAF→wasm 链路结构完整且实测可达（boot.js 的
   MessageChannel 帧驱动 + `window.__tairitsuFrames` 计数器可观测）。
2. 「promise 任务上 flush 卡死」——真实现象、错误归因。卡死的
   `debug_state()=true|true|1|1` 是**补丁应用阶段 wasm trap** 的结果：
   首个交互补丁在 text 节点槽位上执行了元素专有 DOM 调用（见下），
   trap 不走 Rust 解栈（`FlushGuard::drop` 不执行），守卫永久卡住。
   promise 任务本身没有限制。
3. 「实例毒化」——机制确认：jco wrapper 的每个导出入口先
   `throwIfTrapped()`，wasm 调用内的任何 JS 异常都会 `markTrapped` 永久
   毒化实例（boot.js 帧驱动的 try/catch 恰好拦在 wrapper 之外时才幸免）。

**真实根因（全部在 tairitsu 上游，PR celestia-island/tairitsu#126 已合入）**：

- **ReplaceNode 是「原位重建内容」而非「替换节点」**：槽位换了节点种类后
  仍保留旧元素身份（stale `<table>`/modal overlay），后续所有按索引寻址
  的补丁逐批错位，DOM 与 diff 基线永久失同步。修复：按新 vnode 种类建
  新节点 + `replaceChild` 真替换。
- **元素专有操作落在 text 槽位上直接 trap**：glue 的 `setAttribute` 在
  text 节点上抛 TypeError → jco 下是 wasm trap。修复：用
  `node.get-node-type` 判别，非元素槽位 warn+skip。
- **同批 RemoveChild 必须降序执行**：位置 diff 的全部索引基于旧子表，
  先执行的删除会把后续索引顶偏（实测 `rm4(f,n=6)` 后 `rm5(found=false,
  n=5)`，overlay 幸存）。修复：非删除操作按原序先跑（数量中性、旧帧
  索引保持有效），删除操作按索引降序后跑。**关键在嵌套 UpdateChild 的
  子补丁列表同样要走这套纪律**——最初的修复只覆盖根层批次，弹窗关闭
  依然挂，事件序列（rmh 日志）揪出了递归层的朴素 for 循环。
- **npm glue 分发缺 node 三操作**：`getNodeType`/`insertBefore`/
  `replaceChild` 在运行时 glue 源里有、npm 分发源里没有——wrapper
  加载即报 "does not provide an export named 'getNodeType'"。

**附带发现（jco 生态坑，下游规避即可）**：帧任务（隐式线程）里 wasi
stderr 的 OutputStream 资源句柄无效——渲染/补丁路径上任何
`eprintln!`/panic-hook 落 stderr 都会 trap（"Not a valid
OutputStream resource"）。诊断输出必须走 DOM 属性（本仓库的
`data-dbg-*` 仪表即为此设计）。

**落地架构（两端均已切换，rev 锁定 7f2f6ef）**：

- 状态变更 → `request_remount()`（名字保留，语义=mark_dirty）→ rAF
  调度（init_runtime 注册 + boot.js MessageChannel 帧驱动）→
  `flush_render` → `diff(old,new)` → `apply_patches`。旧的
  data-flush-pending 属性 → MutationObserver → pm-flush → 整树重挂
  桥退役。
- **admin2：纯增量管线**。八路由 + 六编辑器套件（弹窗开闭往返、发放
  向导、下钻实例管理、野怪管理、六实体删除）全部实测通过；最坏场景
  性能提升 15–35×（1126 行弹窗开/关 776/843ms → 51/24ms；全量加载
  10.7s → 3.76s；千行加载更多 873–934ms → 440–459ms）。
- **game2：增量 + 结构性批次整树替换兜底**。深层嵌套 UpdateChild 的
  索引错位类缺口尚未根除（背包搜索实测复现：状态 1 项、DOM 残留 3 卡，
  服务端响应无辜），含 Insert/Remove/Replace/Create 的批次直接整树
  替换（游戏单屏 20–50 塔片，替换 ≈21–24ms 与旧重挂持平），换来
  「DOM 与基线永远同源」的硬保证；纯属性/文本批次（战斗回合 19–21ms）
  保持增量。玩家端 70 断言全绿、战斗回合/逃脱/结算链路实测通过。
- 管线仪表：#app 上的 `data-dbg-*` 属性（req/sched/frame/patch/
  patch-n/state-after/plog，game2 另有 [S] 标记），验收脚本与现场
  诊断共用。

**已知边界（上游后续）**：diff 基线是 stored vnode、apply 目标是活 DOM，
二者没有对账机制——错位一次即永久漂移（admin 的流程全对齐所以纯增量
安全；game2 兜底绕开）。根治需要基线/DOM 一致性校验或 reconcile 通道。

### 10.1.6 wasm component 能力探针：每个平台能力真实跑通（2026-11-08）

目标「给 tairitsu 的每个 wasm component 能力都真正可以跑起来」的落地：
新增第三个组件 **capprobe**（`rust/capprobe/`，入口
`plugin.php?id=pokemon:game&index=capprobe`，构建
`scripts/publish/build_tairitsu_cap.py`），在真实浏览器里顺序执行
48 项能力测试（DOM/布局/定时器/事件/网络/剪贴板/observer/媒体查询/
滚动/导航/canvas/IDB/文件/音频/视频/地理/全屏/batch/ws/misc），
每项输出 Pass / Fail / Skip（Skip 必须带环境或 API 面理由）。

**探针揪出并修复的上游缺陷（PR celestia-island/tairitsu#127 已合入，
本仓库 rev 锁定 `3e4c17d`）**：

1. **回调派发的 RefCell 锁纪律**：Rust 2021 的 if-let 条件临时值存活
   到块尾，`with(|m| { if let Some(cb) = m.borrow_mut().remove(..) { cb() } })`
   的守卫跨用户回调持有——回调里再注册 timer/interval/idb/ws 必撞
   「already borrowed」，jco 下 panic 即 wasm trap（守卫卡死 +
   markTrapped 毒化实例）。全部派发器改为 take-then-run（FnOnce）/
   take-run-put-back（FnMut）：timer/interval/idb/ws×4/observer×2/
   mql/scroll/window-resize/video/geo×2/file-reader×2。
2. **npm 分发 glue 缺整块能力**：setInterval/clearInterval（set_interval
   一调就 trap）、connectWebSocket（web-socket-callbacks 派发）、
   canvas 2D 三件套、element closest/getClientWidth/getInnerHtml/
   requestFullscreen（WIT 返回 u64，glue 必须返回数字）、document
   elementFromPoint/execCommand、node getParentNode、window
   matchMedia/getScrollY、media-query-list 接口组。
3. **wire 级修正**：onPromiseRejected 必须带 error 字符串（缺参被 jco
   拒，整个拒绝派发静默死在 JS 侧）；idbOpen 走 onIdbRequestSuccess
   回库句柄字符串（组件无 on-idb-open 导出）+ 升级兜底建 kv 店
   （IdbOps 无建店 API 面）；idbGet 结果 String 化；MutationObserver
   entries 必须是 WIT record（句柄数组会被记录校验拒掉）；
   history pushState/replaceState 单参形态兼容。

**最终结果：39 Pass / 0 Fail / 9 Skip**（每个 Skip 带理由：headless
剪贴板/地理权限、无视频源、无 input.files→blob 句柄获取途径、
MessageChannel 帧驱动的 cancel 缺口）。修复前：33/10/5 且两处 trap。

**Firefox 抽测（尝试记录）**：本机装有 Firefox，但其 CDP 端点已在上游
移除（--remote-debugging-port 不监听）、--screenshot 模式被站点自签
证书页挡住（headless 无证书豁免开关）——无法无人值守抽测，留待手动
或 geckodriver 方案。Chromium 系（Edge）为已验证环境。

### 10.2 双站点并行（行为对照）

入口按 `ui` 参数分流，两套前端共用同一 Discuz 外壳与 API：

| 入口 | 实现 | 状态 |
| --- | --- | --- |
| `plugin.php?id=pokemon:game` | `game.inc.php`（Dioxus，`plugin/wasm/`） | 存量，完整 |
| `plugin.php?id=pokemon:game&ui=v2` | `game_v2.inc.php`（tairitsu，`plugin/wasm2/`） | 迁移中，POC 骨架 |

两个页面顶部各有「界面版本」切换条，便于逐页对照。

### 10.3 构建与本地开发

- 构建：`python scripts/publish/build_tairitsu_poc.py`
  （`--skip-build` 复用 wasm，`--skip-vendor` 复用已装配的 glue/wasi）。
  产物写入 `plugin/wasm2/`：`boot.js`（页面启动脚本，绝对 URL + importmap +
  `MessageChannel` 帧驱动）、`manifest.json`（构建 id 与入口）、`poc.html`
  （独立验证页）。
- `game2` 已从根 workspace 的 `members` 中 `exclude`：它依赖整个
  tairitsu/hikari 组件库、只能在 `wasm32-wasip2` 下产出，并入会让
  `cargo clippy --workspace` 与 CI 的 rust 门禁背上无关成本。它的门禁就是
  上面那条构建脚本。
- 依赖用 git rev 锁定，CI 可复现；本地迭代在
  `rust/game2/.cargo/config.toml`（**不提交**，见 `.gitignore`）里用
  `[patch]` 指向 `../../celestia` 下的工作副本。
- `docker/entrypoint.sh` 每次启动都把 `/plugin` 同步进容器（`cp -R`，不用
  `cp -a`——后者保留源时间戳会跳过目标里更新的文件，构建产物改了不生效）。
