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
#[cfg(target_family = "wasm")]
pub mod delegation;
#[cfg(target_family = "wasm")]
pub mod pages;
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
    let state = tairitsu_vdom::runtime::with_component(component_id, app_state::AppState::new);
    state.component_id.set(component_id as u32);
    state.config.component_id.set(component_id as u32);
    ROOT_COMPONENT.with(|id| id.set(component_id));

    let state_for_render = state.clone();
    let state_for_delegation = state.clone();
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

    {
        use tairitsu_vdom::platform::DomOps;
        let app = WitElement::from_raw(get_element_by_id(0, "app").expect("#app not found"));
        delegation::install(&app, &state_for_delegation);
    }

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
            let root = PATCH_ROOT.with(|r| r.borrow().clone());
            if let Some(root) = root {
                if let Ok(platform) = WitPlatform::new() {
                    tairitsu_web::wit_platform::wasm_impl::with_render_component(
                        component_id,
                        || {
                            if let Err(e) = platform.apply_patches(&root, &patches) {
                                eprintln!("[admin2] apply_patches failed: {e:?}");
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
