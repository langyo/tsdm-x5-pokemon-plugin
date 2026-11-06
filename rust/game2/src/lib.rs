//! tairitsu + hikari 版游戏前端。
//!
//! 第一个真实页面：商店（1:1 移植自 Dioxus 版，见 src/shop.rs）。
//! 启动流程与状态模式记录在 docs/refactor-tairitsu-hikari-research.md §9/§10。

#[cfg(target_family = "wasm")]
pub mod api;
#[cfg(target_family = "wasm")]
pub mod delegation;
pub mod icon;
#[cfg(target_family = "wasm")]
pub mod shop;
pub mod shop_logic;
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

/// 按元素 id 取原始句柄（诊断/同步写入用）。
#[cfg(target_family = "wasm")]
pub fn element_handle(id: &str) -> Option<u64> {
    get_element_by_id(0, id)
}

#[cfg(target_family = "wasm")]
thread_local! {
    /// 最新 ShopState：remount 通道由此取状态构建整树。
    /// （pm-flush 监听器是 'static 闭包，不能捕获局部 state。）
    static LATEST_STATE: std::cell::RefCell<Option<crate::state::ShopState>> =
        const { std::cell::RefCell::new(None) };
}

/// 请求一次重挂载：由 boot.js 的观察器把属性变化转成 pm-flush 事件，
/// 再由 remount_latest 执行。可从任何线程调用（只做同步导入）。
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

/// 整树重挂载：清空 #app 后按最新信号值重新渲染整棵 Shop 树。
///
/// 为什么不走 flush_render（diff/apply）：jco 下 apply 路径的嵌套
/// UpdateChild 索引解析会挂起（带数据的 patch 在事件任务上实测卡死，
/// 见研究文档 §10.1.3）；而 mount_vnode_to_app 的「create + append」
/// 纯构造链在挂载任务上对任意规模树都可靠。游戏单屏 20–50 个瓦片，
/// 重挂载开销可接受；上游修好 apply 后换回增量路径。
#[cfg(target_family = "wasm")]
fn remount_latest() {
    let state = LATEST_STATE.with(|s| s.borrow().clone());
    let Some(state) = state else { return };
    let vnode = shop::Shop(shop::ShopProps {
        state: Some(Box::new(state)),
        ..Default::default()
    });
    if let Ok(platform) = WitPlatform::new() {
        let _ = platform.mount_vnode_to_app(vnode);
    }
}

#[cfg(target_family = "wasm")]
pub fn run_app() -> Result<()> {
    // panic 上报：写进 #app 的 data-panic 属性，再尝试 stderr。
    //
    // 不能只走 eprintln！wasm 的 stderr 是 wasi:cli 的 OutputStream 资源，
    // jco 把资源句柄登记在「发起调用的隐式线程」的表里：从 lifecycle.start
    // 所在线程之外（如 onFrame/onTimeout 的回调线程）调 get-stderr 会抛
    // "Not a valid OutputStream resource"——panic hook 里的 eprintln 自身
    // 崩掉，把真正的 panic 信息整个吞掉。DOM 属性在任何线程都能写，先写它。
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("[game2] PANIC: {info}");
        {
            use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
            if let Some(app) = get_element_by_id(0, "app") {
                let _ = set_attribute(app, "data-panic", &msg);
            }
        }
        eprintln!("{msg}");
    }));

    // 状态归属：先注册组件拿 id，再在 with_component 上下文里创建信号，
    // 这样 ReactiveSignal::set 的 mark_dirty 会落到本组件。
    let component_id = tairitsu_vdom::use_component(|| tairitsu_vdom::VNode::empty());
    let state = tairitsu_vdom::runtime::with_component(component_id, state::ShopState::new);
    state.component_id.set(component_id as u32);

    // 重渲染闭包共享同一份 state（信号绑定本组件，set → mark_dirty(Shop)）。
    let state_for_render = state.clone();
    let latest_for_remount = state.clone();
    let state_for_delegation = state.clone();
    tairitsu_vdom::runtime::update_render_function(component_id, move || {
        shop::Shop(shop::ShopProps {
            state: Some(Box::new(state_for_render.clone())),
            ..Default::default()
        })
    });
    let vnode = shop::Shop(shop::ShopProps {
        state: Some(Box::new(state)),
        ..Default::default()
    });
    LATEST_STATE.with(|s| *s.borrow_mut() = Some(latest_for_remount));

    let platform = WitPlatform::new()?;
    platform.mount_vnode_to_app(vnode.clone())?;

    // diff 产出的 patch 相对「组件 vnode 根元素」（mount 后 #app 的第一个
    // 子元素）计算，init_runtime/apply 必须以同一元素为根；传 #app 会差一层，
    // 顶层 patch 索引解析到 None 被静默丢弃（tairitsu#116 的教训）。
    let app_handle = get_element_by_id(0, "app").expect("#app mount point not found");
    let children_handle = get_child_nodes(app_handle);
    let vnode_root_handle = item(children_handle, 0).expect("rendered vnode root not found");
    let root_element = WitElement::from_raw(vnode_root_handle);
    tairitsu_web::init_runtime(root_element.clone());

    // 调度（重挂载通道）：
    //   - 任何线程的数据更新 → request_remount 写 data-flush-pending
    //     （同步导入，全线程可靠）→ boot.js 的 MutationObserver 派发
    //     pm-flush 事件 → 下面的监听器在事件任务里整树重挂载
    //     （mount 链路对任意规模树已验证可靠；diff/apply 会挂起，
    //     见研究文档 §10.1.3）。
    //   - 交互：事件委托（delegation.rs）在 #app 上注册一次，重挂载
    //     零监听器开销。
    {
        use tairitsu_vdom::platform::DomOps;
        let platform = WitPlatform::new()?;
        let app = WitElement::from_raw(
            get_element_by_id(0, "app").expect("#app mount point not found"),
        );
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
        eprintln!("[game2] run failed: {err:?}");
    }
}

#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn tairitsu_component_bootstrap() {
    run();
}

#[cfg(not(target_family = "wasm"))]
pub fn main_stub() {}
