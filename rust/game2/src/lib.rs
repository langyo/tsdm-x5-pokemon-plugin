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
    // panic 上报：写进 #app 的 data-panic 属性，再尝试 stderr。
    //
    // 不能只走 eprintln！wasm 的 stderr 是 wasi:cli 的 OutputStream 资源，
    // jco 把资源句柄登记在「发起调用的隐式线程」的表里：从 lifecycle.start
    // 所在线程之外（如 onFrame/onTimeout 的回调线程）调 get-stderr 会抛
    // "Not a valid OutputStream resource"——panic hook 里的 eprintln 自身
    // 崩掉，把真正的 panic 信息整个吞掉（实测踩坑，见研究文档 §10.1.2）。
    // DOM 属性在任何线程都能写，先写它保底。
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("[game2] PANIC: {info}");
        #[cfg(target_family = "wasm")]
        {
            use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::{
                element::set_attribute, non_element_parent_node::get_element_by_id,
            };
            if let Some(app) = get_element_by_id(0, "app") {
                let _ = set_attribute(app, "data-panic", &msg);
            }
        }
        eprintln!("{msg}");
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
    // 调度：异步 rAF。mark_dirty 可能仍在 RUNTIME 借用的调用栈上，回调
    // 不能同步 flush（会重入），因此注册进 ANIMATION_CALLBACKS 后请求
    // 下一帧，由 glue 的 onFrame 投递。
    //
    // 历史教训（为什么这里曾经退化成同步 flush）：shim→glue 的相对导入
    // 没带构建戳时，浏览器会一直用旧版 glue（0.5.4 tarball）——它回连的
    // 是 snake_case 旧回调名，经 `?.` 可选链全部静默无操作，帧/定时器/
    // promise 三条投递链路一起失效。构建脚本已给全部 glue/wasi 导入加
    // ?v=构建戳（见 build_tairitsu_poc.py 的 shims_from_glue），此路径
    // 恢复可用。boot.js 在后台标签页把 rAF 改写到 MessageChannel 上，
    // 规避节流；mark_dirty 的 FRAME_DEADLINE 兜底（上游 #119）保证即使
    // 帧真的丢了也不会永久冻结。
    tairitsu_vdom::runtime::set_schedule_callback(Box::new(|callback: Box<dyn FnOnce()>| {
        use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::{
            element::set_attribute, non_element_parent_node::get_element_by_id,
        };
        let write_diag = |msg: &str| {
            if let Some(app) = get_element_by_id(0, "app") {
                let _ = set_attribute(app, "data-sched", msg);
            }
        };
        let id = tairitsu_web::wit_platform::wasm_impl::next_callback_id();
        tairitsu_web::wit_platform::wasm_impl::ANIMATION_CALLBACKS.with(|m| {
            m.borrow_mut()
                .insert(id, Some(Box::new(move |_ts| callback())));
        });
        // 宿主调用可能抛（jco 异步机制 / glue），而 dispatch_event 会把
        // handler 里的 panic 静默吞掉——捕捉后写进 DOM 才能看到。
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::platform_helpers::request_animation_frame(id);
        }));
        match result {
            Ok(()) => write_diag(&format!("ok id={id}")),
            Err(e) => {
                let msg = e
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_else(|| "unknown".to_string());
                write_diag(&format!("THREW id={id}: {msg}"));
            }
        }
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
