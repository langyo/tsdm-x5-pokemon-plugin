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

    // 调试探针：init_runtime 注册的 apply_patches 用 tracing::error 上报失败，
    // wasm 上无 tracing 订阅时完全静默。这里覆盖为带 eprintln 的版本以暴露问题。
    {
        let root_for_patches = root_element;
        tairitsu_vdom::runtime::set_apply_patches_callback(
            move |component_id: tairitsu_vdom::ComponentId, patches: Vec<tairitsu_vdom::Patch>| {
                eprintln!("[game2] apply_patches comp={component_id:?} n={}", patches.len());
                let root = root_for_patches.clone();
                let platform = WitPlatform::new();
                match platform {
                    Ok(platform) => {
                        let result = platform.apply_patches(&root, &patches);
                        if let Err(e) = result {
                            eprintln!("[game2] apply_patches ERROR: {e:?}");
                        }
                    }
                    Err(e) => eprintln!("[game2] platform ERROR: {e:?}"),
                }
            },
        );
        tairitsu_vdom::runtime::set_schedule_callback(Box::new(
            |callback: Box<dyn FnOnce()>| {
                // 后台标签页的 rAF 永不触发会让调度器永久卡死；timer 在任何
                // 可见性下都会触发，保证 flush 不被饿死。
                let platform = WitPlatform::new()
                    .expect("platform for schedule");
                use tairitsu_vdom::TimerOps;
                platform.set_timeout(callback, 0);
            },
        ));
    }

    tairitsu_vdom::runtime::store_initial_vnode(component_id, vnode);
    eprintln!("[game2] initial vnode stored, component={component_id:?}");
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
