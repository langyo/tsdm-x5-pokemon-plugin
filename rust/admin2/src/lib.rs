//! 后台管理前端（tairitsu 版）。
//!
//! 渲染架构（细粒度 diff/patch，见 docs/refactor-tairitsu-hikari-research.md
//! §10.1.4 的勘误与 §10.1.5 的落地记录）：
//!
//! 状态变更 → [`request_remount`] → `runtime::mark_dirty(ROOT)` →
//! rAF 调度（boot.js 用 MessageChannel 驱动 `window.requestAnimationFrame`，
//! glue 派发到 wrapper 导出的 `onFrame` 重入 wasm）→ `flush_render` →
//! render_route 重算 vnode + diff(old, new) → `apply_patches` 只落差异。
//!
//! 旧的「写 data-flush-pending 属性 → boot.js MutationObserver → pm-flush
//! 事件 → 整树重挂」桥已退役：§10.1.4 曾按 `onAnimationFrame` 这个名字 grep
//! 判定 wrapper 缺导出，实际导出名是 `onFrame`，整条 rAF→wasm 链路存在且
//! 实测可达。`data-dbg-*` 属性是管线仪表（写在 #app 上，验收脚本可读），
//! 稳定后可摘除。

#[cfg(target_family = "wasm")]
pub mod api;
pub mod app_state;
/// 七张数据列表页共用的页面壳。
#[cfg(target_family = "wasm")]
pub mod list_page;
#[cfg(target_family = "wasm")]
pub mod pages;
/// hikari 标准件搭的外壳（AdminShell / AdminHeader / NavSidebar / 状态栏 / Toast）。
#[cfg(target_family = "wasm")]
pub mod shell;
/// hikari 标准件的宿主适配层（受控组件的样板收在这里）。
#[cfg(target_family = "wasm")]
pub mod hk;
pub mod state;

#[cfg(target_family = "wasm")]
use anyhow::Result;
#[cfg(target_family = "wasm")]
use tairitsu_web::WitPlatform;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::WitElement;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::node::get_child_nodes;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::node_list::item;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::non_element_parent_node::get_element_by_id;

#[cfg(target_family = "wasm")]
pub fn element_handle(id: &str) -> Option<u64> {
    get_element_by_id(0, id)
}

#[cfg(target_family = "wasm")]
thread_local! {
    /// 根组件 ID：request_remount 据此 mark_dirty（0 = 尚未完成启动）。
    static ROOT_COMPONENT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// vnode 根元素（#app 的首个子节点）：diff 补丁相对它而非 #app 落地。
    static PATCH_ROOT: std::cell::RefCell<Option<WitElement>> =
        const { std::cell::RefCell::new(None) };
    /// 管线仪表计数（data-dbg-*），键为属性名。
    static DBG_COUNTERS: std::cell::RefCell<std::collections::HashMap<String, u64>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// 把增量渲染请求交给响应式运行时：mark_dirty → rAF → flush → diff → patch。
///
/// 名字保留 `request_remount` 是为了不动 pages.rs / delegation.rs 里的三百多处
/// 调用点；语义已从「整树重挂」变为「标脏 + 帧调度增量渲染」。
#[cfg(target_family = "wasm")]
pub fn request_remount() {
    ROOT_COMPONENT.with(|id| {
        if id.get() != 0 {
            bump_dbg("data-dbg-req");
            tairitsu_vdom::runtime::mark_dirty(id.get());
        }
    });
}

/// 在 #app 上写一个诊断属性（管线仪表）。
#[cfg(target_family = "wasm")]
pub fn dbg_attr(name: &str, value: &str) {
    use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
    if let Some(app) = get_element_by_id(0, "app") {
        let _ = set_attribute(app, name, value);
    }
}

/// 补丁摘要（Patch 未 derive Debug，手写变体名 + 关键字段供仪表用）。
#[cfg(target_family = "wasm")]
fn patch_summary(p: &tairitsu_vdom::Patch, depth: usize) -> String {
    use tairitsu_vdom::Patch;
    if depth > 5 {
        return "…".into();
    }
    match p {
        Patch::CreateNode { .. } => "CreateNode".into(),
        Patch::ReplaceNode { .. } => "ReplaceNode".into(),
        Patch::RemoveNode => "RemoveNode".into(),
        Patch::UpdateText { text } => format!("UpdateText({}字)", text.chars().count()),
        Patch::UpdateAttribute { name, .. } => format!("UpdAttr({name})"),
        Patch::AddAttribute { name, .. } => format!("AddAttr({name})"),
        Patch::RemoveAttribute { name } => format!("RmAttr({name})"),
        Patch::UpdateStyle { .. } => "UpdStyle".into(),
        Patch::UpdateClass { .. } => "UpdClass".into(),
        Patch::InsertChild { index, .. } => format!("InsertChild({index})"),
        Patch::RemoveChild { index } => format!("RemoveChild({index})"),
        Patch::UpdateChild { index, patches } => format!(
            "UpdateChild({index},[{}])",
            patches
                .iter()
                .map(|cp| patch_summary(cp, depth + 1))
                .collect::<Vec<_>>()
                .join(",")
        ),
        Patch::AddEvent { name, .. } => format!("AddEvent({name})"),
        Patch::UpdateEvent { name, .. } => format!("UpdEvent({name})"),
        Patch::RemoveEvent { name } => format!("RmEvent({name})"),
        _ => "Other".into(),
    }
}

/// 计数仪表：每次调用把 data-dbg-{name} 加一（name 需带 data-dbg- 前缀传入）。
#[cfg(target_family = "wasm")]
pub fn bump_dbg(name: &str) {
    DBG_COUNTERS.with(|c| {
        let mut map = c.borrow_mut();
        let v = map.entry(name.to_string()).or_insert(0);
        *v += 1;
        let n = *v;
        drop(map);
        dbg_attr(name, &n.to_string());
    });
}

/// 累加仪表：data-dbg-{name} 累加 n（如补丁条数总量）。
#[cfg(target_family = "wasm")]
pub fn add_dbg(name: &str, n: u64) {
    DBG_COUNTERS.with(|c| {
        let mut map = c.borrow_mut();
        let v = map.entry(name.to_string()).or_insert(0);
        *v += n;
        let total = *v;
        drop(map);
        dbg_attr(name, &total.to_string());
    });
}

/// 补丁批次滚动日志（最近 3 批），诊断 keyed/indexed 掉 op 用。
#[cfg(target_family = "wasm")]
fn push_patch_log(entry: String) {
    thread_local! {
        static PLOG: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    PLOG.with(|l| {
        let mut v = l.borrow_mut();
        v.push(entry);
        let len = v.len();
        if len > 3 {
            v.drain(0..len - 3);
        }
        let s = v
            .iter()
            .enumerate()
            .map(|(i, e)| format!("[{i}] {e}"))
            .collect::<Vec<_>>()
            .join(" ||| ");
        let s: String = s.chars().take(900).collect();
        dbg_attr("data-dbg-plog", &s);
    });
}

/// 读 PHP 挂在 `#app` 上的页面数据（`data-pm-*`，见 plugin/admin_v2.inc.php）。
/// 读不到的键留空：外壳按「没有这一项」渲染，不猜。
#[cfg(target_family = "wasm")]
fn read_boot_info() -> crate::state::BootInfo {
    use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::get_attribute;
    let app = get_element_by_id(0, "app");
    let read = |name: &str| -> String {
        app.and_then(|handle| get_attribute(handle, name))
            .unwrap_or_default()
    };
    crate::state::BootInfo {
        site_name: read("data-pm-site-name"),
        user_id: read("data-pm-user-id"),
        user_name: read("data-pm-user-name"),
        user_group: read("data-pm-user-group"),
        avatar: read("data-pm-avatar"),
        plugin_version: read("data-pm-plugin-version"),
        forum_url: read("data-pm-forum-url"),
        game_url: read("data-pm-game-url"),
        legacy_url: read("data-pm-legacy-url"),
        logout_url: read("data-pm-logout-url"),
    }
}

/// 把视口宽度喂给外壳（hikari 的壳不自己量视口，见 FaceShell 头注）：
/// 启动时量一次，再挂一条跨过桌面 / 移动分界的媒体查询，翻越时重量。
#[cfg(target_family = "wasm")]
fn watch_viewport(state: &app_state::AppState) {
    use tairitsu_vdom::platform::{LayoutOps, MediaQueryOps};
    let Ok(platform) = WitPlatform::new() else {
        return;
    };
    state
        .shell
        .viewport_width
        .inner()
        .set(platform.inner_width() as f64);
    let list = platform.match_media("(min-width: 1024px)");
    let st = state.clone();
    platform.media_query_list_add_listener(
        list,
        Box::new(move |_matches: bool| {
            let width = WitPlatform::new()
                .map(|platform| platform.inner_width())
                .unwrap_or(0);
            st.shell.viewport_width.inner().set(width as f64);
            request_remount();
        }),
    );
}

/// 一批补丁的签名（管线仪表，写在 `#app[data-dbg-sig]`）：递归统计每种补丁的
/// 条数，属性补丁按属性名单列。排查「状态已变、DOM 没跟上」时，先看这一批
/// 里到底有没有那条补丁，再看 `data-patch-error`（落地失败的原因）。
#[cfg(target_family = "wasm")]
fn patch_signature(
    patches: &[tairitsu_vdom::Patch],
    out: &mut std::collections::BTreeMap<String, u64>,
) {
    use tairitsu_vdom::Patch;
    for patch in patches {
        match patch {
            Patch::UpdateChild { patches, .. } => {
                *out.entry("UpdateChild".to_string()).or_default() += 1;
                patch_signature(patches, out);
            }
            Patch::UpdateAttribute { name, .. }
            | Patch::AddAttribute { name, .. }
            | Patch::RemoveAttribute { name } => {
                let key = format!("{}:{name}", patch_variant(patch));
                *out.entry(key).or_default() += 1;
            }
            other => {
                let key = patch_variant(other).to_string();
                *out.entry(key).or_default() += 1;
            }
        }
    }
}

/// 补丁的变体名（`Patch` 未 derive Debug）。
#[cfg(target_family = "wasm")]
fn patch_variant(p: &tairitsu_vdom::Patch) -> &'static str {
    use tairitsu_vdom::Patch;
    match p {
        Patch::CreateNode { .. } => "CreateNode",
        Patch::ReplaceNode { .. } => "ReplaceNode",
        Patch::RemoveNode => "RemoveNode",
        Patch::UpdateText { .. } => "UpdateText",
        Patch::UpdateAttribute { .. } => "UpdateAttribute",
        Patch::AddAttribute { .. } => "AddAttribute",
        Patch::RemoveAttribute { .. } => "RemoveAttribute",
        Patch::UpdateStyle { .. } => "UpdateStyle",
        Patch::UpdateClass { .. } => "UpdateClass",
        Patch::InsertChild { .. } => "InsertChild",
        Patch::RemoveChild { .. } => "RemoveChild",
        Patch::UpdateChild { .. } => "UpdateChild",
        Patch::AddEvent { .. } => "AddEvent",
        Patch::UpdateEvent { .. } => "UpdateEvent",
        Patch::RemoveEvent { .. } => "RemoveEvent",
        _ => "Other",
    }
}

#[cfg(target_family = "wasm")]
fn run_app() -> Result<()> {
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("[admin2] PANIC: {info}");
        {
            use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
            if let Some(app) = get_element_by_id(0, "app") {
                let _ = set_attribute(app, "data-panic", &msg);
            }
        }
        eprintln!("{msg}");
    }));

    let component_id = tairitsu_vdom::use_component(|| tairitsu_vdom::VNode::empty());
    let boot = read_boot_info();
    let state = tairitsu_vdom::runtime::with_component(component_id, || {
        app_state::AppState::new(boot)
    });
    state.component_id.set(component_id as u32);
    state.config.component_id.set(component_id as u32);
    ROOT_COMPONENT.with(|id| id.set(component_id));
    watch_viewport(&state);

    let state_for_render = state.clone();
    tairitsu_vdom::runtime::update_render_function(component_id, move || {
        pages::render_route(&state_for_render)
    });
    let vnode = pages::render_route(&state);

    let platform = WitPlatform::new()?;
    platform.mount_vnode_to_app(vnode.clone())?;

    let app_handle = get_element_by_id(0, "app").expect("#app mount point not found");
    let children_handle = get_child_nodes(app_handle);
    let vnode_root_handle = item(children_handle, 0).expect("rendered vnode root not found");
    let root_element = WitElement::from_raw(vnode_root_handle);
    {
        use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::get_tag_name;
        let tag = get_tag_name(root_element.as_raw());
        dbg_attr("data-dbg-root", &tag);
    }
    // init_runtime 注册调度（rAF）与补丁落地（相对 root）回调；随后的两个
    // set_*_callback 用同语义版本覆盖，仅叠加管线仪表。
    tairitsu_web::init_runtime(root_element.clone());
    PATCH_ROOT.with(|r| *r.borrow_mut() = Some(root_element));

    tairitsu_vdom::runtime::set_schedule_callback(Box::new(|callback: Box<dyn FnOnce()>| {
        bump_dbg("data-dbg-sched");
        let id = tairitsu_web::wit_platform::wasm_impl::next_callback_id();
        tairitsu_web::wit_platform::wasm_impl::ANIMATION_CALLBACKS.with(|m| {
            m.borrow_mut()
                .insert(id, Some(Box::new(move |_ts| {
                    bump_dbg("data-dbg-frame");
                    callback();
                    // flush 返回后的运行时快照：scheduled|flushing|dirty|components|drains|deferred
                    let (sched, flushing, dirty, comps) = tairitsu_vdom::runtime::debug_state();
                    let (drains, deferred) = tairitsu_vdom::runtime::debug_flush_counts();
                    dbg_attr(
                        "data-dbg-state-after",
                        &format!("{sched}|{flushing}|{dirty}|{comps}|{drains}|{deferred}"),
                    );
                })));
        });
        tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::platform_helpers::request_animation_frame(id);
    }));
    tairitsu_vdom::runtime::set_apply_patches_callback(
        |component_id: tairitsu_vdom::ComponentId,
         patches: Vec<tairitsu_vdom::Patch>| {
            bump_dbg("data-dbg-patch");
            add_dbg("data-dbg-patch-n", patches.len() as u64);
            if let Some(first) = patches.first() {
                let _ = first;
                let s = format!(
                    "n={} | {}",
                    patches.len(),
                    patches
                        .iter()
                        .map(|p| patch_summary(p, 0))
                        .collect::<Vec<_>>()
                        .join(" ; ")
                );
                let s: String = s.chars().take(400).collect();
                dbg_attr("data-dbg-p0", &s);
                push_patch_log(s);
            }
            let mut signature = std::collections::BTreeMap::new();
            patch_signature(&patches, &mut signature);
            dbg_attr("data-dbg-sig", &format!("{signature:?}"));
            let root = PATCH_ROOT.with(|r| r.borrow().clone());
            if let Some(root) = root {
                if let Ok(platform) = WitPlatform::new() {
                    tairitsu_web::wit_platform::wasm_impl::with_render_component(
                        component_id,
                        || {
                            if let Err(e) = platform.apply_patches(&root, &patches) {
                                let msg: String = format!("{e:?}").chars().take(400).collect();
                                dbg_attr("data-patch-error", &msg);
                                eprintln!("[admin2] apply_patches failed: {msg}");
                            }
                        },
                    );
                }
            }
        },
    );

    tairitsu_vdom::runtime::store_initial_vnode(component_id, vnode);
    Ok(())
}

#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn run() {
    if let Err(err) = run_app() {
        eprintln!("[admin2] run failed: {err:?}");
    }
}

#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn tairitsu_component_bootstrap() {
    run();
}

#[cfg(not(target_family = "wasm"))]
pub fn main_stub() {}
