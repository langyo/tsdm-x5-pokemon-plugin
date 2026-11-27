//! tairitsu + hikari 版游戏前端。
//!
//! 外壳（顶部导航 / 内容区 / 侧栏 / 页脚）和各页面都是同一个根组件的渲染函数：
//! 状态在 `run_app` 里、于根组件上下文内创建一次，交互用 tairitsu 原生事件
//! 绑定，变更经 signal → mark_dirty → 下一帧 flush → diff → apply_patches，
//! 只把差异落到 DOM。来龙去脉见 docs/refactor-tairitsu-hikari-research.md。

#[cfg(target_family = "wasm")]
pub mod api;
pub mod app_state;
#[cfg(target_family = "wasm")]
pub mod battle;
pub mod battle_types;
pub mod button;
#[cfg(target_family = "wasm")]
pub mod footer;
#[cfg(target_family = "wasm")]
pub mod home;
pub mod icon;
#[cfg(target_family = "wasm")]
pub mod inventory;
#[cfg(target_family = "wasm")]
pub mod mini_grid;
#[cfg(target_family = "wasm")]
pub mod modal;
#[cfg(target_family = "wasm")]
pub mod my_pokemon;
/// 宠物状态说明表（纯数据 + 单测，可在 host 上测）。
pub mod pet_state;
#[cfg(target_family = "wasm")]
pub mod pokemon_center;
#[cfg(target_family = "wasm")]
pub mod pokemon_storage;
pub mod quality;
#[cfg(target_family = "wasm")]
pub mod shop;
pub mod shop_logic;
#[cfg(target_family = "wasm")]
pub mod sidebar;
pub mod state;
pub mod storage_logic;
#[cfg(target_family = "wasm")]
pub mod toast;
#[cfg(target_family = "wasm")]
pub mod welcome;

#[cfg(target_family = "wasm")]
use hikari_components::{Button, ButtonProps};
use anyhow::Result;
#[cfg(target_family = "wasm")]
use tairitsu_web::WitPlatform;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::non_element_parent_node::get_element_by_id;

#[cfg(target_family = "wasm")]
thread_local! {
    /// 根组件 ID：request_remount 据此 mark_dirty（0 = 尚未完成启动）。
    static ROOT_COMPONENT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 请求一次增量渲染：mark_dirty → 下一帧 → flush_render → diff → apply_patches。
///
/// 名字保留 request_remount 是为了不动各页面模块里的调用点；「整树重挂」的
/// 语义早已退役。
#[cfg(target_family = "wasm")]
pub fn request_remount() {
    ROOT_COMPONENT.with(|id| {
        if id.get() != 0 {
            tairitsu_vdom::runtime::mark_dirty(id.get());
        }
    });
}

/// 按路由渲染页面：欢迎页整屏；其余页面是「顶部导航 + 内容区（+ 侧栏）+ 页脚」，
/// 结构与类名对齐 Dioxus 版 `components/layout`（Layout / Header / PageContainer /
/// Sidebar / Footer）。
#[cfg(target_family = "wasm")]
fn render_route(state: &app_state::AppState) -> tairitsu_vdom::VNode {
    use app_state::Route;
    use sidebar::{Sidebar, SidebarProps};
    use tairitsu_macros::rsx;

    // 账户资料回来之前是加载态；拉取失败给出「重新载入」（对齐 Dioxus 版 Layout）。
    match &state.sidebar.profile.get() {
        None => return loading_container(state, None),
        Some(Err(err)) => return loading_container(state, Some(err.as_str())),
        Some(Ok(_)) => {}
    }

    if state.route.get() == Route::Welcome {
        return welcome::Welcome(welcome::WelcomeProps {
            state: Some(Box::new(state.clone())),
            ..Default::default()
        });
    }
    // 各页首次进入时装载（对齐 Dioxus 版 use_resource 首次求值时机）。
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
    if state.route.get() == Route::Adventure {
        battle::ensure_loaded(state);
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
        Route::Adventure => battle::Adventure(battle::AdventureProps {
            state: Some(Box::new(state.clone())),
            ..Default::default()
        }),
    };
    // 侧栏数据（携带宠物 / 背包分类件数 / 徽章状态）与页面无关，进主界面时装一次。
    sidebar::ensure_loaded(state);

    // 顶部导航与 Dioxus 版 Header 完全一致：个人中心 / 商店 / 宠物中心 / 野外冒险。
    // 首页走左上角 logo，背包与仓库走侧栏。
    let nav_cls = |active: bool| {
        if active {
            "nav-link active"
        } else {
            "nav-link"
        }
    };
    let mypm_cls = nav_cls(state.route.get() == Route::MyPokemon);
    let shop_cls = nav_cls(state.route.get() == Route::Shop);
    let center_cls = nav_cls(state.route.get() == Route::PokemonCenter);
    let adventure_cls = nav_cls(state.route.get() == Route::Adventure);
    // 首页与冒险页全宽，其余页面是内容区 + 侧栏（对齐 Dioxus 版 Layout / PageContainer）。
    let full_width = matches!(state.route.get(), Route::Home | Route::Adventure);
    let container_cls = if full_width {
        "page-container page-container--full-width"
    } else {
        "page-container page-container--default"
    };
    let show_sidebar = state.route.get() != Route::Home;
    let img_logo = "source/plugin/pokemon/images/item/jlq.gif";
    let (logo_state, mypm_state, shop_state, center_state, adventure_state) = (
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
    );
    // 顶部提示容器常驻根部（对齐 Dioxus 版 AppProviders → ToastProvider）。
    let toast_container = toast::container();
    // 页脚常驻；“关于本项目”弹窗按状态挂载（结构与类名对齐 Dioxus 版）。
    let footer_node = footer::Footer(footer::FooterProps {
        state: Some(Box::new(state.clone())),
        ..Default::default()
    });
    let about_node = if state.show_about.get() {
        footer::about_modal(state)
    } else {
        tairitsu_vdom::VNode::empty()
    };

    rsx! {
        div { class: "app-layout",
            ..vec![toast_container]
            div { class: "toast-spacer" }
            header { class: "app-header",
                div { class: "header-inner",
                    div { class: "header-logo",
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: ("logo-link").to_string(),
                            onclick: crate::button::click(move || navigate(&logo_state, Route::Home)),
                            img { class: "logo-mascot", src: img_logo, alt: "Pokeball" }
                            span { class: "logo-text", "Online Void Pokemon" }
                                                } }, [("data-test-nav", ("home").to_string())]) }
                    }
                    nav { class: "header-nav",
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: (mypm_cls).to_string(),
                            onclick: crate::button::click(move || navigate(&mypm_state, Route::MyPokemon)),
                            "个人中心"
                                                } }, [("data-test-nav", ("my-pokemon").to_string())]) }
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: (shop_cls).to_string(),
                            onclick: crate::button::click(move || navigate(&shop_state, Route::Shop)),
                            "商店"
                                                } }, [("data-test-nav", ("shop").to_string())]) }
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: (center_cls).to_string(),
                            onclick: crate::button::click(move || navigate(&center_state, Route::PokemonCenter)),
                            "宠物中心"
                                                } }, [("data-test-nav", ("center").to_string())]) }
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: (adventure_cls).to_string(),
                            onclick: crate::button::click(move || navigate(&adventure_state, Route::Adventure)),
                            "野外冒险"
                                                } }, [("data-test-nav", ("adventure").to_string())]) }
                    }
                }
            }
            div { class: "app-main",
                div { class: "app-content",
                    div { class: container_cls, ..vec![page] }
                }
                if show_sidebar {
                    Sidebar { state: Some(Box::new(state.clone())) }
                }
            }
            ..vec![footer_node, about_node]
        }
    }
}

/// 账户资料加载中 / 加载失败（对齐 Dioxus 版 Layout 的 loading-container）。
#[cfg(target_family = "wasm")]
fn loading_container(state: &app_state::AppState, error: Option<&str>) -> tairitsu_vdom::VNode {
    use tairitsu_macros::rsx;

    let Some(err) = error else {
        return rsx! {
            div { class: "loading-container",
                div { class: "loading-spinner" }
                p { "加载中..." }
            }
        };
    };
    let message = format!("账户载入失败：{err}");
    let reload_state = state.clone();
    rsx! {
        div { class: "loading-container",
            p { role: "alert", "{message}" }
            Button {
                class: ("btn btn-primary").to_string(),
                onclick: crate::button::click(move || {
                    reload_state.sidebar.profile.inner().set(None);
                    request_remount();
                    load_profile(&reload_state);
                }),
                "重新载入"
                        }
        }
    }
}

/// 拉取用户资料并据此选起始页（对齐 Dioxus 版 Layout 的 needs_welcome）：
/// is_new_player 或 total_pokemons==0 → Welcome，否则 Home。
#[cfg(target_family = "wasm")]
fn load_profile(state: &app_state::AppState) {
    use app_state::Route;

    let state = state.clone();
    api::profile(Box::new(move |result| {
        match result {
            Ok(summary) => {
                let route = if summary.needs_welcome {
                    Route::Welcome
                } else {
                    Route::Home
                };
                state.needs_welcome.inner().set(summary.needs_welcome);
                state.sidebar.profile.inner().set(Some(Ok(summary)));
                state.route.inner().set(route);
            }
            Err(e) => state.sidebar.profile.inner().set(Some(Err(e))),
        }
        request_remount();
    }));
}

/// 页面切换（对齐 Dioxus 版 `*CURRENT_PAGE.write() = page`）。
#[cfg(target_family = "wasm")]
pub fn navigate(state: &app_state::AppState, route: app_state::Route) {
    state.route.inner().set(route);
    request_remount();
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
    state.battle.component_id.set(component_id as u32);
    // 提示列表一变就标脏本组件（容器渲染在根部）。
    toast::init();

    let state_for_render = state.clone();
    tairitsu_vdom::runtime::update_render_function(component_id, move || {
        render_route(&state_for_render)
    });
    let vnode = render_route(&state);
    ROOT_COMPONENT.with(|id| id.set(component_id));

    // 资料回来前先画加载态；回调里改路由 + 标脏即切到 Welcome / Home。
    load_profile(&state);

    let platform = WitPlatform::new()?;
    // diff 的补丁相对「组件 vnode 根元素」计算，init_runtime 必须拿 mount 返回
    // 的同一个根：传挂载父级 #app 会差一层，顶层补丁索引全部错位（首条撞巧
    // 命中、其余静默丢弃）。
    let root = platform.mount_vnode_to_app(vnode.clone())?;
    tairitsu_web::init_runtime(root);

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
