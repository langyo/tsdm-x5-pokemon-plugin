//! POC：tairitsu + hikari 重建游戏前端的最小骨架。
//! 验证清单见 docs/refactor-tairitsu-hikari-research.md Phase 1：
//! 1) 挂载进 Discuz 页面既有容器（boot 脚本在 #main 内预置 #app 挂载点）
//! 2) hikari ThemeProvider/Button/Toast 渲染
//! 3) 信号驱动列表（use_signal + rsx if/for）
//! 4) 内联 lucide SVG（dangerous_inner_html）
//! 5) 异步 API 往返（tairitsu_web::fetch_text 回调桥 → set 信号）

#[cfg(target_family = "wasm")]
pub mod app;

#[cfg(target_family = "wasm")]
use anyhow::Result;
#[cfg(target_family = "wasm")]
use tairitsu_web::WitPlatform;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::non_element_parent_node::get_element_by_id;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::WitElement;

/// 二分开关：true = 只挂最小计数器（诊断信号→重渲染→patch 链路）。
const MINIMAL_COUNTER_POC: bool = false;

// 整树重挂载策略用的最新状态快照（schedule 回调里重建整棵树时读取）。
// PocState 内部全是 Rc/信号句柄，clone 是廉价的引用复制。
#[cfg(target_family = "wasm")]
thread_local! {
    static STATE_FOR_REMOUNT: std::cell::RefCell<Option<app::PocState>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_family = "wasm")]
pub fn run_app() -> Result<()> {
    // wasm 里 tracing 无订阅者，dispatch_event 的 catch_unwind 会静默吞掉
    // handler panic——装 hook 让 panic 走 stderr 可见。
    std::panic::set_hook(Box::new(|info| {
        eprintln!("[game2] PANIC: {info}");
    }));
    // 状态归属：先注册组件拿到 id，再在 with_component 上下文里创建信号，
    // 这样 ReactiveSignal::set 的 mark_dirty 会落到本组件（而非兜底空组件）。
    let component_id = tairitsu_vdom::use_component(|| tairitsu_vdom::VNode::empty());

    let vnode = if MINIMAL_COUNTER_POC {
        let count = tairitsu_vdom::runtime::with_component(
            component_id,
            || tairitsu_hooks::use_signal(|| 0i32),
        );
        let count_for_render = count.clone();
        tairitsu_vdom::runtime::update_render_function(component_id, move || {
            app::MiniCounter(app::MiniCounterProps { count: Some(count_for_render.clone()), ..Default::default() })
        });
        app::MiniCounter(app::MiniCounterProps { count: Some(count.clone()), ..Default::default() })
    } else {
        let state = tairitsu_vdom::runtime::with_component(component_id, || app::PocState::new());
        STATE_FOR_REMOUNT.with(|slot| *slot.borrow_mut() = Some(state.clone()));
        let state_for_render = state.clone();
        // 重渲染闭包共享同一份 state（信号绑定本组件，set → mark_dirty(App)）。
        tairitsu_vdom::runtime::update_render_function(component_id, move || {
            app::App(app::AppProps {
                state: Some(state_for_render.clone()),
                ..Default::default()
            })
        });
        app::App(app::AppProps { state: Some(state.clone()), ..Default::default() })
    };

    let platform = WitPlatform::new()?;
    platform.mount_vnode_to_app(vnode.clone())?;

    // 关键：diff 产出的 patch 相对「组件 vnode 根元素」（mount 后 #app 的第一个
    // 子元素）计算，init_runtime/apply 必须以同一元素为根；传 #app 会差一层，
    // 顶层 patch 索引解析到 None 被静默丢弃（列表更新失效的根因）。
    let app_handle = get_element_by_id(0, "app").expect("#app mount point not found");
    let children_handle = tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::node::get_child_nodes(app_handle);
    let vnode_root_handle = tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::node_list::item(children_handle, 0)
        .expect("rendered vnode root not found");
    let root_element = WitElement::from_raw(vnode_root_handle);
    tairitsu_web::init_runtime(root_element.clone());

    // 渲染策略（POC 起）：tairitsu 0.5.26 的增量 diff/apply 对「子列表追加」
    // 的 UpdateChild/InsertChild 嵌套 patch 解析不可靠——列表不增长（已在
    // 上游用探针定位，见 docs/refactor-tairitsu-hikari-research.md §9）。
    // 游戏 UI 单屏规模有限（一屏 20–50 个瓦片），这里改为整树重挂载：
    // 按最新 state 重新挂载整棵树，行为确定，语义等价于 Dioxus 的完整
    // 重渲染。上游修好后可去掉此覆盖，恢复增量路径。
    {
        tairitsu_vdom::runtime::set_apply_patches_callback(move |_component_id, _patches| {
            let state = STATE_FOR_REMOUNT.with(|s| s.borrow().clone());
            let Some(state) = state else { return };
            if let Ok(platform) = WitPlatform::new() {
                let latest = app::App(app::AppProps {
                    state: Some(state),
                    ..Default::default()
                });
                // mount_vnode_to_app 会先清空 #app 再整棵渲染。
                let _ = platform.mount_vnode_to_app(latest);
            }
        });
    }

    // 调度：必须异步（mark_dirty 仍持有 RUNTIME 的 RefCell 借用，同步
    // flush_render 会重入失败），且不能用 setTimeout——后台/被节流标签页的
    // 定时器可能长时间不触发，导致 `scheduled` 卡 true、后续更新全丢。
    // rAF 在前台可靠触发；glue 的 animation-callbacks 已正确分发 onFrame。
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
