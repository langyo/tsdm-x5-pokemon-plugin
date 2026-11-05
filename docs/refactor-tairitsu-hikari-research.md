# Dioxus → Tairitsu + Hikari 前端重构：系统研究报告

> 研究日期：2026-11-04。tairitsu @ master `4a93466e`（0.5.26，2026-11-03），hikari @ origin/master `9d571fb4`（npm 0.55.276，crates 0.5.1）。
> 研究对象：`celestia/tairitsu`、`celestia/hikari`、本仓库 `rust/game`（46 个 rs 源文件 / 11,115 行 + 15 个 SCSS / 9,841 行）。

---

## 1. 结论摘要（TL;DR）

1. **可行，且契合度远超预期**。hikari 的 Rust 端 prelude 自述为 "Compatibility layer for migrating from Dioxus to Tairitsu"（`packages/components/src/prelude.rs`），tairitsu 官方甚至带有 Dioxus 迁移指南（`docs/en/guides/migration/dioxus-to-tairitsu.md`）。rsx! 语法形状、`use_signal` + `.read()/.write()`、`MouseData` 类型别名、`dangerous_inner_html` 全部与 Dioxus 兼容或同名。
2. **组件覆盖矩阵几乎全覆盖**（见 §6）：商店宫格/分页/Tooltip/Toast/Modal/Drawer/Tabs/徽标/表单/侧栏/列表/树/右键菜单/战斗日志/地图 scene 家族，hikari 全部有现成或薄封装可用的组件；中文（zh-Hans）内置于 hikari-i18n。
3. **最大技术风险：无 async spawn**。tairitsu 没有 `spawn`/`spawn_local`/wasm-bindgen-futures；`use_resource` 的文档注释自认 "async execution not supported on WASM targets"（`hooks/src/suspense.rs:494`）。而现有 game 前端的所有 API 调用（购买/战斗/刷新）都跑在 `spawn(async move { … })` 上。**这是 POC 第一优先验证项**，候选方案见 §5.2。
4. **构建链完全换血**：wasm32-unknown-unknown + wasm-bindgen → **wasm32-wasip2 组件 + jco 转译**，产物是纯静态文件（JS 模块 + wasm + glue + wasi-shim），可由任意静态服务器（含 PHP 目录）伺服。`examples/website/README.md` 明确演示 `python3 -m http.server` 即可伺服。
5. **嵌入 Discuz 页面的可行路径已找到**，但生成的 index.html 用根绝对路径且硬编码 `#app`——需要手写 boot 脚本（参照 `scripts/build_cdn_demo.py` 的 importmap + 手动装配模式），Rust 侧手动 `get_element_by_id(0, "main") → init_runtime → store_initial_vnode`。
6. **许可证义务（SySL 1.0）**：分发即须携带「含 AI 生成代码」声明 + 模型披露（对象代码分发时须对最终用户可见，如关于页/文档）。这是不可豁免的条款（§2），需要在发布流程里落实。
7. **两库 API 高频漂移**（周级重写、双版本族 0.3→0.5 跳变），**必须以 git rev 精确锁定依赖**。

---

## 2. 框架定位与版本

| | Tairitsu | Hikari |
|---|---|---|
| 定位 | WASM 组件模型全栈框架（VDOM + hooks + rsx! + 编译期 scss!/svg! + SSR + dev/build 工具链） | 同一设计系统双端 UI 库：Rust/WASM（跑在 tairitsu 上）+ Vue3 端 |
| 版本 | 0.5.26（crates 锁步版本；注意 0.5.23 从未发布，消费须 `^0.5.24+`） | crates 0.5.1（2026-10-31）；npm `@celestia-island/hikari` 0.55.276 |
| 活跃度 | 每日提交；13 个 CI 工作流（含视觉回归/浏览器 e2e） | npm 近日更；PR 编号 > #1064；CI 含 wasm32-wasip2 check + vue-mirror 漂移检查 |
| 许可证 | SySL 1.0（Apache-2.0 等效授予 + AI 披露义务；仓库 Cargo.toml 却写 MIT，元数据不一致） | 同 SySL 1.0 |

Hikari 结构：Vue 3 包（`packages/vue`）是**规范源**；Rust 端每个组件文件自称 "mirror of the canonical Vue `Hk*`"，直接 `scss! { file: "src/styles/vue-mirror/...", no_hash }` 编译同一张 SCSS 表进二进制，文件头逐条记录与 Vue 端的"适配差异"（无 Teleport/无焦点陷阱/无自动翻转等）。**读组件头注释就是读文档**（docs/ 目录滞后于代码）。

---

## 3. 现有前端盘点（迁移工作量基线）

- `rust/game`：46 个 rs 文件 / 11,115 行。页面：shop / battle(+battle/) / adventure / my_pokemon（最大，约 2,500 行）/ inventory / pokemon_storage / pokemon_center / home / welcome；组件：layout（sidebar/header）、common（popup/modal/toast/type_badge/icon/pet_marks/loading_overlay/providers）、pokemon（mini_grid/tooltip）；状态：`state/ui.rs`（popup/modal/toast 全局态）+ 各 hooks。
- `rust/admin`：16,845 行 —— **本阶段不动**（管理端保持 Dioxus，待 game 端验证后再议）。
- 样式：15 个 SCSS 组件文件 + variables/mixins/fonts ≈ 9,841 行，含 `$color-*` 主题变量体系 —— 可整体迁移到 tairitsu 的 packager SCSS 管线（grass 编译、load_paths 支持），再逐步被 hikari 组件替换。
- 构建：`scripts/publish/build.py --build-wasm`（cargo + wasm-bindgen/dioxus-web）→ 需要新写 tairitsu 构建段（cargo wasm32-wasip2 + jco transpile + glue 装配）。

---

## 4. API 迁移映射（Dioxus 0.7 → Tairitsu）

| Dioxus 0.7 | Tairitsu 0.5.26 | 备注 |
|---|---|---|
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
2. **WIT setTimeout 轮询/poll**：把"future"改成状态机 + 定时器驱动的重试轮询（丑，兜底）。
3. **服务端代理成两段式**：提交即返回 + 轮询结果接口（改 PHP API 面不可取，除非万不得已）。
> 若 (1) 走通，API 层从"async fn"改为"发起 + 回调 set 信号"形态，`spawn(async move {…})` 处全部机械改写。

### 5.3 静态嵌入 Discuz 页面 —— 路径已明确，需手写 boot
- 产物：`<pkg>.wasm` + `component-wrapper/`（jco 产物）+ `component-wrapper-loader.js` + `browser-glue/__tairitsu_glue__.js` + `wasi-shim/*`（本地 bundle，避免 esm.sh 依赖）+ `styles.css`。
- 生成 index.html 的根绝对路径（`/component-wrapper-loader.js` 等）不适配 `/source/plugin/pokemon/wasm/` 子路径 → 参照 `scripts/build_cdn_demo.py` 手写 importmap + boot module；Rust 侧手动挂到 `#main`。
- 构建期 glue 漂移：fresh clone 缺 `packages/browser-glue/dist`（未跟踪的生成物）→ `TAIRITSU_RUNTIME_BUNDLE` 或 npm 安装 `@celestia-island/tairitsu-web-glue`。
- 需要 node + `@bytecodealliance/jco`（`npx --yes` 可用）；`rustup target add wasm32-wasip2`。
- 嵌入后与 Discuz 页面 CSS 共存：hikari 组件自带 vue-mirror SCSS（`no_hash` 全局类 `hk-*`），ThemeProvider 输出 CSS 变量；与 Discuz 样式冲突面小但需实测。

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
|---|---|---|
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
  1. 挂载 + hikari ThemeProvider + Button/Toast 渲染；
  2. **async 桥**：一次真实 API 往返（登录态下拉 `get::user_info` 类只读接口）；
  3. 信号驱动列表渲染 + rsx `if/for`；
  4. scss 管线出 `styles.css` 且与 Discuz 共存不互污；
  5. 内联 SVG 图标渲染正常（命名空间 fixup 验证）。
- **验收门**：POC 全绿才进入 Phase 2；async 若无解，回全体检讨（带 §5.2 的方案 2/3 或暂缓重构）。

### Phase 2 —— 基础设施层
全局状态（UI store / toast / popup）、API client（回调形态重写）、Icon 组件平移、路由（dioxus Routable → tairitsu router 或自管 signal）、主题变量桥接（`$color-*` → CSS 变量）。

### Phase 3 —— 页面逐个迁移（每页一个 PR）
顺序：**shop（本次痛点、最小闭环）→ welcome/home → inventory → pokemon_center → pokemon_storage → adventure → my_pokemon（最大）→ battle（最复杂，含子组件目录）**。
每页验收：功能等价（对照现有 e2e/浏览器回归）、视觉走查、与旧版并行开关可回退（`&ui=v2` 参数或灰度开关）。

### Phase 4 —— 收尾
删 dioxus/game 旧栈、build.py 移除旧构建段、样式去重、（可选）admin 端同路线评估。
每阶段遵循仓库 AGENTS.md：feat/<name> 分支、gitmoji、3 轮验证、本地门禁、squash PR。

---

## 8. 待确认决策点

1. **POC 先行，OK？**（Phase 1 全绿后再大规模动页面）
2. **范围**：先 game；admin 留 Dioxus，后续再议 —— 是否同意？
3. **SySL 披露义务**：接受并落实（关于页 + zip NOTICE）——是否 OK？
4. **依赖锁定**：tairitsu/hikari 以 git rev 锁定，升级走独立 PR —— 是否 OK？
# Dioxus → Tairitsu + Hikari 前端重构：系统研究报告

> 研究日期：2026-11-04。tairitsu @ master `4a93466e`（0.5.26，2026-11-03），hikari @ origin/master `9d571fb4`（npm 0.55.276，crates 0.5.1）。
> 研究对象：`celestia/tairitsu`、`celestia/hikari`、本仓库 `rust/game`（46 个 rs 源文件 / 11,115 行 + 15 个 SCSS / 9,841 行）。

---

## 1. 结论摘要（TL;DR）

1. **可行，且契合度远超预期**。hikari 的 Rust 端 prelude 自述为 "Compatibility layer for migrating from Dioxus to Tairitsu"（`packages/components/src/prelude.rs`），tairitsu 官方甚至带有 Dioxus 迁移指南（`docs/en/guides/migration/dioxus-to-tairitsu.md`）。rsx! 语法形状、`use_signal` + `.read()/.write()`、`MouseData` 类型别名、`dangerous_inner_html` 全部与 Dioxus 兼容或同名。
2. **组件覆盖矩阵几乎全覆盖**（见 §6）：商店宫格/分页/Tooltip/Toast/Modal/Drawer/Tabs/徽标/表单/侧栏/列表/树/右键菜单/战斗日志/地图 scene 家族，hikari 全部有现成或薄封装可用的组件；中文（zh-Hans）内置于 hikari-i18n。
3. **最大技术风险：无 async spawn**。tairitsu 没有 `spawn`/`spawn_local`/wasm-bindgen-futures；`use_resource` 的文档注释自认 "async execution not supported on WASM targets"（`hooks/src/suspense.rs:494`）。而现有 game 前端的所有 API 调用（购买/战斗/刷新）都跑在 `spawn(async move { … })` 上。**这是 POC 第一优先验证项**，候选方案见 §5.2。
4. **构建链完全换血**：wasm32-unknown-unknown + wasm-bindgen → **wasm32-wasip2 组件 + jco 转译**，产物是纯静态文件（JS 模块 + wasm + glue + wasi-shim），可由任意静态服务器（含 PHP 目录）伺服。`examples/website/README.md` 明确演示 `python3 -m http.server` 即可伺服。
5. **嵌入 Discuz 页面的可行路径已找到**，但生成的 index.html 用根绝对路径且硬编码 `#app`——需要手写 boot 脚本（参照 `scripts/build_cdn_demo.py` 的 importmap + 手动装配模式），Rust 侧手动 `get_element_by_id(0, "main") → init_runtime → store_initial_vnode`。
6. **许可证义务（SySL 1.0）**：分发即须携带「含 AI 生成代码」声明 + 模型披露（对象代码分发时须对最终用户可见，如关于页/文档）。这是不可豁免的条款（§2），需要在发布流程里落实。
7. **两库 API 高频漂移**（周级重写、双版本族 0.3→0.5 跳变），**必须以 git rev 精确锁定依赖**。

---

## 2. 框架定位与版本

| | Tairitsu | Hikari |
|---|---|---|
| 定位 | WASM 组件模型全栈框架（VDOM + hooks + rsx! + 编译期 scss!/svg! + SSR + dev/build 工具链） | 同一设计系统双端 UI 库：Rust/WASM（跑在 tairitsu 上）+ Vue3 端 |
| 版本 | 0.5.26（crates 锁步版本；注意 0.5.23 从未发布，消费须 `^0.5.24+`） | crates 0.5.1（2026-10-31）；npm `@celestia-island/hikari` 0.55.276 |
| 活跃度 | 每日提交；13 个 CI 工作流（含视觉回归/浏览器 e2e） | npm 近日更；PR 编号 > #1064；CI 含 wasm32-wasip2 check + vue-mirror 漂移检查 |
| 许可证 | SySL 1.0（Apache-2.0 等效授予 + AI 披露义务；仓库 Cargo.toml 却写 MIT，元数据不一致） | 同 SySL 1.0 |

Hikari 结构：Vue 3 包（`packages/vue`）是**规范源**；Rust 端每个组件文件自称 "mirror of the canonical Vue `Hk*`"，直接 `scss! { file: "src/styles/vue-mirror/...", no_hash }` 编译同一张 SCSS 表进二进制，文件头逐条记录与 Vue 端的"适配差异"（无 Teleport/无焦点陷阱/无自动翻转等）。**读组件头注释就是读文档**（docs/ 目录滞后于代码）。

---

## 3. 现有前端盘点（迁移工作量基线）

- `rust/game`：46 个 rs 文件 / 11,115 行。页面：shop / battle(+battle/) / adventure / my_pokemon（最大，约 2,500 行）/ inventory / pokemon_storage / pokemon_center / home / welcome；组件：layout（sidebar/header）、common（popup/modal/toast/type_badge/icon/pet_marks/loading_overlay/providers）、pokemon（mini_grid/tooltip）；状态：`state/ui.rs`（popup/modal/toast 全局态）+ 各 hooks。
- `rust/admin`：16,845 行 —— **本阶段不动**（管理端保持 Dioxus，待 game 端验证后再议）。
- 样式：15 个 SCSS 组件文件 + variables/mixins/fonts ≈ 9,841 行，含 `$color-*` 主题变量体系 —— 可整体迁移到 tairitsu 的 packager SCSS 管线（grass 编译、load_paths 支持），再逐步被 hikari 组件替换。
- 构建：`scripts/publish/build.py --build-wasm`（cargo + wasm-bindgen/dioxus-web）→ 需要新写 tairitsu 构建段（cargo wasm32-wasip2 + jco transpile + glue 装配）。

---

## 4. API 迁移映射（Dioxus 0.7 → Tairitsu）

| Dioxus 0.7 | Tairitsu 0.5.26 | 备注 |
|---|---|---|
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
2. **WIT setTimeout 轮询/poll**：把"future"改成状态机 + 定时器驱动的重试轮询（丑，兜底）。
3. **服务端代理成两段式**：提交即返回 + 轮询结果接口（改 PHP API 面不可取，除非万不得已）。
> 若 (1) 走通，API 层从"async fn"改为"发起 + 回调 set 信号"形态，`spawn(async move {…})` 处全部机械改写。

### 5.3 静态嵌入 Discuz 页面 —— 路径已明确，需手写 boot
- 产物：`<pkg>.wasm` + `component-wrapper/`（jco 产物）+ `component-wrapper-loader.js` + `browser-glue/__tairitsu_glue__.js` + `wasi-shim/*`（本地 bundle，避免 esm.sh 依赖）+ `styles.css`。
- 生成 index.html 的根绝对路径（`/component-wrapper-loader.js` 等）不适配 `/source/plugin/pokemon/wasm/` 子路径 → 参照 `scripts/build_cdn_demo.py` 手写 importmap + boot module；Rust 侧手动挂到 `#main`。
- 构建期 glue 漂移：fresh clone 缺 `packages/browser-glue/dist`（未跟踪的生成物）→ `TAIRITSU_RUNTIME_BUNDLE` 或 npm 安装 `@celestia-island/tairitsu-web-glue`。
- 需要 node + `@bytecodealliance/jco`（`npx --yes` 可用）；`rustup target add wasm32-wasip2`。
- 嵌入后与 Discuz 页面 CSS 共存：hikari 组件自带 vue-mirror SCSS（`no_hash` 全局类 `hk-*`），ThemeProvider 输出 CSS 变量；与 Discuz 样式冲突面小但需实测。

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
|---|---|---|
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
  1. 挂载 + hikari ThemeProvider + Button/Toast 渲染；
  2. **async 桥**：一次真实 API 往返（登录态下拉 `get::user_info` 类只读接口）；
  3. 信号驱动列表渲染 + rsx `if/for`；
  4. scss 管线出 `styles.css` 且与 Discuz 共存不互污；
  5. 内联 SVG 图标渲染正常（命名空间 fixup 验证）。
- **验收门**：POC 全绿才进入 Phase 2；async 若无解，回全体检讨（带 §5.2 的方案 2/3 或暂缓重构）。

### Phase 2 —— 基础设施层
全局状态（UI store / toast / popup）、API client（回调形态重写）、Icon 组件平移、路由（dioxus Routable → tairitsu router 或自管 signal）、主题变量桥接（`$color-*` → CSS 变量）。

### Phase 3 —— 页面逐个迁移（每页一个 PR）
顺序：**shop（本次痛点、最小闭环）→ welcome/home → inventory → pokemon_center → pokemon_storage → adventure → my_pokemon（最大）→ battle（最复杂，含子组件目录）**。
每页验收：功能等价（对照现有 e2e/浏览器回归）、视觉走查、与旧版并行开关可回退（`&ui=v2` 参数或灰度开关）。

### Phase 4 —— 收尾
删 dioxus/game 旧栈、build.py 移除旧构建段、样式去重、（可选）admin 端同路线评估。
每阶段遵循仓库 AGENTS.md：feat/<name> 分支、gitmoji、3 轮验证、本地门禁、squash PR。

---

## 8. 待确认决策点

1. **POC 先行，OK？**（Phase 1 全绿后再大规模动页面）
2. **范围**：先 game；admin 留 Dioxus，后续再议 —— 是否同意？
3. **SySL 披露义务**：接受并落实（关于页 + zip NOTICE）——是否 OK？
4. **依赖锁定**：tairitsu/hikari 以 git rev 锁定，升级走独立 PR —— 是否 OK？


---

## 9. Phase 1 POC 实施记录（2026-11-04，分支 feat/tairitsu-poc）

### 已验证通过
| 清单项 | 结果 |
|---|---|
| wasm32-wasip2 组件构建 + jco 转译 + 静态装配 | ✅ scripts/publish/build_tairitsu_poc.py |
| 嵌入 Discuz 页面（Caddy 白名单加 wasm2） | ✅ /source/plugin/pokemon/wasm2/poc.html |
| hikari ThemeProvider/Button 渲染（品牌色覆盖） | ✅ |
| 信号 → 重渲染 → diff → patch → DOM 全链路 | ✅（最小计数器 + 文本 span 更新） |
| fetch 回调桥（fetchPromise → promiseCallbacks → set 信号） | ✅ |

### 已定位的问题（阻塞列表类更新）
1. **上游 diff/apply 语义缺口**：列表追加场景 diff 产出
   `UpdateChild{index:N, [InsertChild{index:N}]}`（N == 旧子节点数，越界），
   apply 的 get_child_at 对越界索引返回 None 后**静默跳过**整段嵌套 patch，
   DOM 无任何变化（文本更新不受影响）。已在干净构建中用 MutationObserver +
   全链路探针复现。需上游修复或本地 workaround（见下）。
2. **npm glue(0.5.4) 与 0.5.26 组件的命名错位**：glue 回连
   on_animation_frame/on_timeout/on_promise_resolve（snake/旧名），组件导出
   实为 onFrame/onTimeout/onPromiseResolved（kebab→camel 新约定）。
   POC 已在装配层做别名代理修正（poc.html 内 CALLBACK_ALIASES）。
3. **glue 缺口补齐**：node.getChildNodes、node-list 接口整体、
   platform-helpers.fetchPromise（签名失配：传 Response 而非 String）
   均为装配层手写补齐（build_tairitsu_poc.py shims 段）。

### 关键实现要点（迁移时必须遵守）
- **状态模式**：tairitsu 的 use_signal 每次渲染创建新信号（受控组件设计）。
  持久状态必须：注册组件拿 id → `with_component(id, || 创建状态)` →
  `update_render_function(id, 闭包)` 共享同一组信号 → 经 props 传入组件。
  渲染闭包外创建的信号其 mark_dirty 会落到兜底空组件（静默失效）。
- **rsx 文本**：仅支持 `"{ident}"` 整串占位符简写；混合文本（`"前缀 {x}"`）
  不插值，需在组件体内 format! 后再以整串占位符渲染。
- **后台标签页 rAF 不触发**：自动化测试需以 setTimeout 模拟帧驱动；
  前台真实使用不受影响。
- **调试**：tracing 在 wasm 无订阅者，apply_patches 等失败完全静默；
  需要 eprintln 探针（stderr → console.error）。git rev 锁定时 checkout
  内改动会被 cargo 还原，调试需临时切 path 依赖。

### 追加发现（PR #117 验证过程中确认的上游缺陷）
1. **调度非确定性**：ReactiveSignal::set → mark_dirty → schedule 回调在部分
   页面加载中不触发（stderr 探针证实 set 完成但 schedule 未执行），疑似
   flush_render/rt.scheduled 状态机问题；同构建不同加载表现不一致。
2. **嵌套 InsertChild 丢失**：即使 flush+apply 携带完整 patch
   （span UpdateText 落地 ✓），ul 的 InsertChild{2, li} 不产生 DOM 变更
   （原生 createElement/appendChild 包装计数为 0）。疑似 apply 递归下降
   对 Fragment/深层子树索引解析仍有缺口。
以上两项需在上游用 browser e2e（scripts/browser_e2e.py + shirabe）建立
「挂载→点击→断言 DOM」回归用例后修复，消费侧无法绕过。

### 下一步（Phase 1 收尾选项）
A. 向上游反馈 diff/apply 缺口（附复现），期间游戏列表用「整棵重挂载」
   workaround（set_text_content(app,'') + mount_vnode_to_app 全量重渲染，
   游戏规模可接受）。
B. 【已执行】向 tairitsu 提交修复 PR：
   celestia-island/tairitsu#116（fix/init-runtime-rendered-root 分支）。
   根因实为 init_runtime 根元素接线错误：patch 相对「组件 vnode 根」计算，
   文档/示例/模板却传挂载父级 #app，顶层索引全部错位一层（首条撞巧命中、
   其余静默丢弃）。修复 = mount_vnode_to_app 返回渲染后的根元素，
   示例/模板/迁移文档统一改为以该根调用 init_runtime。
   合入后插件侧的「列表更新失效」即随之消失，届时重验 POC 清单即可。
C. 暂缓列表类页面，先迁无列表页面（welcome/home）验证范式后随上游修复推进。
