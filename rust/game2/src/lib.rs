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

    let root_handle = get_element_by_id(0, "app").expect("#app mount point not found");
    let root_element = WitElement::from_raw(root_handle);
    tairitsu_web::init_runtime(root_element);

    // 调试探针：init_runtime 注册的 apply_patches 用 tracing::error 上报失败，
    // wasm 上无 tracing 订阅时完全静默。这里覆盖为带 eprintln 的版本以暴露问题。
    {
        let root_for_patches = root_element.clone();
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
                let callback_id = tairitsu_web::wit_platform::wasm_impl::next_callback_id();
                tairitsu_web::wit_platform::wasm_impl::ANIMATION_CALLBACKS.with(|m| {
                    m.borrow_mut().insert(
                        callback_id,
                        Some(Box::new(move |_ts| callback())),
                    );
                });
                eprintln!("[game2] schedule rAF id={callback_id}");
                tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::platform_helpers::request_animation_frame(callback_id);
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
