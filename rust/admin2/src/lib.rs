//! 后台管理前端（tairitsu 版，切片 1：壳 + 全局配置页）。
//!
//! 架构与 _game2 完全一致（data-pm-* 委托 + request_remount 整树重挂，
//! 见 docs/refactor-tairitsu-hikari-research.md §9/§10）。后台数据面较大
//! （Dioxus 版 8 页 / 1.6 万行），按页面切片移植：本切片交付壳与全局
//! 配置（读+写），其余路由在导航中列出并显示「移植中」占位。

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
    /// 最新 AppState：pm-flush 监听器由此取状态整树重挂。
    static LATEST_STATE: std::cell::RefCell<Option<crate::app_state::AppState>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_family = "wasm")]
pub fn request_remount() {
    use std::cell::Cell;
    thread_local! {
        static TICK: Cell<u64> = const { Cell::new(0) };
    }
    use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
    TICK.with(|t| {
        let n = t.get() + 1;
        t.set(n);
        if let Some(app) = element_handle("app") {
            let _ = set_attribute(app, "data-flush-pending", &n.to_string());
        }
    });
}

#[cfg(target_family = "wasm")]
fn remount_latest() {
    let state = LATEST_STATE.with(|s| s.borrow().clone());
    let Some(state) = state else { return };
    let vnode = pages::render_route(&state);
    if let Ok(platform) = WitPlatform::new() {
        let _ = platform.mount_vnode_to_app(vnode);
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
    let state = tairitsu_vdom::runtime::with_component(component_id, app_state::AppState::new);
    state.component_id.set(component_id as u32);
    state.config.component_id.set(component_id as u32);

    let state_for_render = state.clone();
    let latest_for_remount = state.clone();
    let state_for_delegation = state.clone();
    tairitsu_vdom::runtime::update_render_function(component_id, move || {
        pages::render_route(&state_for_render)
    });
    let vnode = pages::render_route(&state);
    LATEST_STATE.with(|s| *s.borrow_mut() = Some(latest_for_remount));

    let platform = WitPlatform::new()?;
    platform.mount_vnode_to_app(vnode.clone())?;

    let app_handle = get_element_by_id(0, "app").expect("#app mount point not found");
    let children_handle = get_child_nodes(app_handle);
    let vnode_root_handle = item(children_handle, 0).expect("rendered vnode root not found");
    let root_element = WitElement::from_raw(vnode_root_handle);
    tairitsu_web::init_runtime(root_element.clone());

    {
        use tairitsu_vdom::platform::DomOps;
        let platform = WitPlatform::new()?;
        let app = WitElement::from_raw(get_element_by_id(0, "app").expect("#app not found"));
        platform.add_event_listener(
            &app,
            "pm-flush",
            Box::new(|_| {
                remount_latest();
            }),
        );
        delegation::install(&app, &state_for_delegation);
    }
    tairitsu_vdom::runtime::set_schedule_callback(Box::new(|callback: Box<dyn FnOnce()>| {
        let id = tairitsu_web::wit_platform::wasm_impl::next_callback_id();
        tairitsu_web::wit_platform::wasm_impl::ANIMATION_CALLBACKS.with(|m| {
            m.borrow_mut()
                .insert(id, Some(Box::new(move |_ts| callback())));
        });
        tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::platform_helpers::request_animation_frame(id);
    }));

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
