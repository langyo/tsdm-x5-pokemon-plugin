//! tairitsu + hikari 版游戏前端。
//!
//! 已移植页面：欢迎页（welcome.rs）、首页（home.rs）、商店（shop.rs）。
//! 启动流程与状态模式记录在 docs/refactor-tairitsu-hikari-research.md §9/§10。

#[cfg(target_family = "wasm")]
pub mod api;
pub mod app_state;
#[cfg(target_family = "wasm")]
pub mod delegation;
#[cfg(target_family = "wasm")]
pub mod home;
pub mod icon;
#[cfg(target_family = "wasm")]
pub mod inventory;
#[cfg(target_family = "wasm")]
pub mod my_pokemon;
#[cfg(target_family = "wasm")]
pub mod pokemon_center;
#[cfg(target_family = "wasm")]
pub mod pokemon_storage;
pub mod quality;
#[cfg(target_family = "wasm")]
pub mod shop;
pub mod shop_logic;
pub mod state;
pub mod storage_logic;
#[cfg(target_family = "wasm")]
pub mod welcome;

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
    /// 最新 AppState：remount 通道由此取状态构建整树。
    /// （pm-flush 监听器是 'static 闭包，不能捕获局部 state。）
    static LATEST_STATE: std::cell::RefCell<Option<crate::app_state::AppState>> =
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

/// 整树重挂载：清空 #app 后按最新信号值与路由重建整棵树。
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
    let vnode = render_route(&state);
    if let Ok(platform) = WitPlatform::new() {
        let _ = platform.mount_vnode_to_app(vnode);
    }
}

/// 按路由渲染页面：欢迎页整屏；其余页面带顶部导航（对齐 Dioxus 版
/// Header 的 首页/商店 入口，未移植页面暂不列出）。
#[cfg(target_family = "wasm")]
fn render_route(state: &app_state::AppState) -> tairitsu_vdom::VNode {
    use app_state::Route;
    use tairitsu_macros::rsx;
    // 首页装载（对齐 Dioxus 版 use_resource 首次求值时机）。
    if state.route.get() == Route::Home {
        home::ensure_loaded(state);
    }
    if state.route.get() == Route::Inventory {
        inventory::ensure_loaded(state);
    }
    if state.route.get() == Route::PokemonCenter {
        pokemon_center::ensure_loaded(state);
    }
    if state.route.get() == Route::MyPokemon {
        my_pokemon::ensure_loaded(state);
    }
    if state.route.get() == Route::PokemonStorage {
        pokemon_storage::ensure_loaded(state);
    }
    if state.route.get() == Route::Welcome {
        return welcome::Welcome(welcome::WelcomeProps {
            state: Some(Box::new(state.clone())),
            ..Default::default()
        });
    }

    let page = match state.route.get() {
        Route::Home => home::Home(home::HomeProps {
            state: Some(Box::new(state.clone())),
            ..Default::default()
        }),
        // Welcome 已在上方整屏返回；此臂只为穷尽匹配。
        Route::Welcome => unreachable!(),
        Route::Shop => shop::Shop(shop::ShopProps {
            state: Some(Box::new(state.shop.clone())),
            ..Default::default()
        }),
        Route::Inventory => inventory::Inventory(inventory::InventoryProps {
            state: Some(Box::new(state.clone())),
            ..Default::default()
        }),
        Route::PokemonCenter => pokemon_center::PokemonCenter(pokemon_center::PokemonCenterProps {
            state: Some(Box::new(state.clone())),
            ..Default::default()
        }),
        Route::MyPokemon => my_pokemon::MyPokemon(my_pokemon::MyPokemonProps {
            state: Some(Box::new(state.clone())),
            ..Default::default()
        }),
        Route::PokemonStorage => {
            pokemon_storage::PokemonStorage(pokemon_storage::PokemonStorageProps {
                state: Some(Box::new(state.clone())),
                ..Default::default()
            })
        }
    };

    let home_active = state.route.get() == Route::Home;
    let shop_active = state.route.get() == Route::Shop;
    let inv_active = state.route.get() == Route::Inventory;
    let center_active = state.route.get() == Route::PokemonCenter;
    let mypm_active = state.route.get() == Route::MyPokemon;
    let storage_active = state.route.get() == Route::PokemonStorage;
    let home_cls = if home_active {
        "nav-link active"
    } else {
        "nav-link"
    };
    let shop_cls = if shop_active {
        "nav-link active"
    } else {
        "nav-link"
    };
    let inv_cls = if inv_active {
        "nav-link active"
    } else {
        "nav-link"
    };
    let center_cls = if center_active {
        "nav-link active"
    } else {
        "nav-link"
    };
    let mypm_cls = if mypm_active {
        "nav-link active"
    } else {
        "nav-link"
    };
    let storage_cls = if storage_active {
        "nav-link active"
    } else {
        "nav-link"
    };
    let img_logo = "source/plugin/pokemon/images/item/jlq.gif";

    rsx! {
        div { class: "app-layout",
            header { class: "app-header",
                div { class: "header-inner",
                    div { class: "header-logo",
                        button {
                            class: "logo-link",
                            "data-pm-nav": "home",
                            img { class: "logo-mascot", src: img_logo }
                            span { class: "logo-text", "Online Void Pokemon" }
                        }
                    }
                    nav { class: "header-nav",
                        button { class: home_cls, "data-pm-nav": "home", "首页" }
                        button { class: shop_cls, "data-pm-nav": "shop", "商店" }
                        button { class: inv_cls, "data-pm-nav": "inventory", "背包" }
                        button { class: center_cls, "data-pm-nav": "center", "宠物中心" }
                        button { class: mypm_cls, "data-pm-nav": "my-pokemon", "我的宠物" }
                        button { class: storage_cls, "data-pm-nav": "storage", "仓库" }
                    }
                }
            }
            div { class: "app-main",
                div { class: "app-content", ..vec![page] }
            }
        }
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
    let state = tairitsu_vdom::runtime::with_component(component_id, app_state::AppState::new);
    state.component_id.set(component_id as u32);
    state.shop.component_id.set(component_id as u32);
    state.inv.component_id.set(component_id as u32);
    state.center.component_id.set(component_id as u32);
    state.my_pm.component_id.set(component_id as u32);
    state.storage.component_id.set(component_id as u32);

    let state_for_render = state.clone();
    let latest_for_remount = state.clone();
    let state_for_delegation = state.clone();
    tairitsu_vdom::runtime::update_render_function(component_id, move || {
        render_route(&state_for_render)
    });
    let vnode = render_route(&state);
    LATEST_STATE.with(|s| *s.borrow_mut() = Some(latest_for_remount));

    // 启动路由判定（对齐 Dioxus 版 Layout 的 needs_welcome）：
    // user/profile → is_new_player 或 total_pokemons==0 → Welcome，
    // 否则 Home。promise 回调只写内部值 + request_remount。
    {
        let state = state.clone();
        api::profile(Box::new(move |result| match result {
            Ok(needs_welcome) => {
                state.needs_welcome.inner().set(needs_welcome);
                let route = if needs_welcome {
                    app_state::Route::Welcome
                } else {
                    app_state::Route::Home
                };
                state.route.inner().set(route);
                request_remount();
            }
            // 资料拉取失败：留在欢迎页（用户可手动初始化或刷新）。
            Err(_) => {}
        }));
    }

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
        let app =
            WitElement::from_raw(get_element_by_id(0, "app").expect("#app mount point not found"));
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
