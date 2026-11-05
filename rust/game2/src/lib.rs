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

/// 复渲染闭包被调用的次数（诊断用，渲染进 DOM 便于在页面上直接读）。
#[cfg(target_family = "wasm")]
thread_local! {
    static RENDER_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
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
        // 诊断：记录 render_fn 被调用了几次，并让组件把它显示出来。
        // 「信号变了但界面不动」时先看这个数：不涨说明 render 没被调度到
        // （mark_dirty/flush 链路），涨了但界面不变说明 diff/apply 没落地。
        tairitsu_vdom::runtime::update_render_function(component_id, move || {
            let n = RENDER_CALLS.with(|c| {
                let v = c.get() + 1;
                c.set(v);
                v
            });
            app::MiniCounter(app::MiniCounterProps {
                count: Some(count_for_render.clone()),
                renders: Some(n),
                ..Default::default()
            })
        });
        app::MiniCounter(app::MiniCounterProps {
            count: Some(count.clone()),
            renders: Some(1),
            ..Default::default()
        })
    } else {
        let state = tairitsu_vdom::runtime::with_component(component_id, || app::PocState::new());
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

    // 渲染走 tairitsu 的增量路径：init_runtime 已接好 apply_patches（以上面
    // 的 vnode 根为基准）。早期这里覆盖过 apply_patches 做整树重挂载，因为
    // rsx 的 `for` 生成的 Fragment 被当成单个子节点、diff 索引与 DOM 索引
    // 错位，列表追加的 InsertChild 会落到不存在的下标上被静默丢弃。
    // 该缺陷已在上游修复（celestia-island/tairitsu#119：VElement 的
    // child/children 递归展平 Fragment），因此不再需要绕行。
    //
    // 调度策略（迁移期）：同步 flush，等价于 Dioxus 的同步重渲染。
    //
    // 曾试过两条异步路径，都不可靠：
    //   1. rAF（platform-helpers.requestAnimationFrame → glue 的 animationId
    //      映射 → animation-callbacks.onFrame）：实测帧被吞，`scheduled` 卡
    //      true，之后每次更新都走 AlreadyScheduled 被丢弃；
    //   2. setTimeout（TimerOps::set_timeout → timer-callbacks.onTimeout）：
    //      同样观察不到回调到达。
    // 两者都只有 mark_dirty 的 FRAME_DEADLINE 兜底能让更新生效，表现为
    // 「点几下不动，过一会儿一次性全冒出来」。同步 flush 行为确定，
    // 游戏单屏元素量下开销可接受；上游修好帧投递后可换回异步批处理。
    //
    // 重入安全：mark_dirty 在调用回调前已释放 RUNTIME 借用，flush_render
    // 自己取走 dirty 列表再渲染，不会重复进入。
    tairitsu_vdom::runtime::set_schedule_callback(Box::new(|callback: Box<dyn FnOnce()>| {
        callback();
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
