//! tairitsu + hikari 版游戏前端。
//!
//! 第一个真实页面：商店（1:1 移植自 Dioxus 版，见 src/shop.rs）。
//! 启动流程与状态模式记录在 docs/refactor-tairitsu-hikari-research.md §9/§10。

#[cfg(target_family = "wasm")]
pub mod api;
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

    // 调度（双通道）：
    //   - 事件/挂载线程：rAF 预订（tairitsu 经典路径，POC 已验证）。
    //   - promise 线程：异步导入（rAF 等）会被 jco 排队且锁损坏后永不
    //     执行，因此那边的回调用 inner().set + mark_dirty_deferred
    //     （均纯内存）绕过本回调，再写 data-flush-pending（同步导入）
    //     触发 boot.js 的 MutationObserver → pm-flush 事件 → 下面的
    //     监听器在事件任务里 flush（挂载期已验证该上下文能完整渲染）。
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
                tairitsu_vdom::runtime::flush_render();
            }),
        );
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
