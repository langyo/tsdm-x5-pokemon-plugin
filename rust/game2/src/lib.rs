//! tairitsu + hikari 版游戏前端。
//!
//! 已移植页面：欢迎页（welcome.rs）、首页（home.rs）、商店（shop.rs）。
//! 启动流程与状态模式记录在 docs/refactor-tairitsu-hikari-research.md §9/§10。

#[cfg(target_family = "wasm")]
pub mod api;
pub mod app_state;
#[cfg(target_family = "wasm")]
pub mod battle;
pub mod battle_types;
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
    /// 根组件 ID：request_remount 据此 mark_dirty（0 = 尚未完成启动）。
    static ROOT_COMPONENT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// TEMP DIAG：vnode 根元素（apply 回调用）。
    static PATCH_ROOT: std::cell::RefCell<Option<WitElement>> =
        const { std::cell::RefCell::new(None) };
    /// 兜底整树替换用的最新状态。
    static STATE_CACHE: std::cell::RefCell<Option<crate::app_state::AppState>> =
        const { std::cell::RefCell::new(None) };
}

/// 请求一次增量渲染：mark_dirty → rAF（boot.js 的 MessageChannel 帧驱动）
/// → flush_render → diff(old,new) → apply_patches 只落差异。
///
/// 名字保留 request_remount 是为了不动各页面模块里的调用点；语义自
/// §10.1.5 起从「整树重挂」改为「标脏 + 帧调度增量渲染」（旧观察器桥
/// 与整树重挂通道已退役）。
#[cfg(target_family = "wasm")]
pub fn request_remount() {
    ROOT_COMPONENT.with(|id| {
        if id.get() != 0 {
            tairitsu_vdom::runtime::mark_dirty(id.get());
        }
    });
}

/// 批次是否含结构性操作（增删/替换子节点）——递归扫描嵌套 UpdateChild。
#[cfg(target_family = "wasm")]
fn patch_has_structural(ps: &[tairitsu_vdom::Patch]) -> bool {
    use tairitsu_vdom::Patch;
    ps.iter().any(|p| match p {
        Patch::InsertChild { .. }
        | Patch::RemoveChild { .. }
        | Patch::ReplaceNode { .. }
        | Patch::CreateNode { .. } => true,
        Patch::UpdateChild { patches, .. } => patch_has_structural(patches),
        _ => false,
    })
}

/// TEMP DIAG：补丁变体摘要（最近 3 批滚动日志用）。
#[cfg(target_family = "wasm")]
fn patch_summary(p: &tairitsu_vdom::Patch, depth: usize) -> String {
    use tairitsu_vdom::Patch;
    if depth > 5 {
        return "…".into();
    }
    match p {
        Patch::CreateNode { .. } => "CreateNode".into(),
        Patch::ReplaceNode { .. } => "ReplaceNode".into(),
        Patch::RemoveNode => "RemoveNode".into(),
        Patch::UpdateText { text } => format!("UpdateText({}字)", text.chars().count()),
        Patch::UpdateAttribute { name, .. } => format!("UpdAttr({name})"),
        Patch::AddAttribute { name, .. } => format!("AddAttr({name})"),
        Patch::RemoveAttribute { name } => format!("RmAttr({name})"),
        Patch::UpdateStyle { .. } => "UpdStyle".into(),
        Patch::UpdateClass { .. } => "UpdClass".into(),
        Patch::InsertChild { index, .. } => format!("InsertChild({index})"),
        Patch::RemoveChild { index } => format!("RemoveChild({index})"),
        Patch::UpdateChild { index, patches } => format!(
            "UpdateChild({index},[{}])",
            patches
                .iter()
                .map(|cp| patch_summary(cp, depth + 1))
                .collect::<Vec<_>>()
                .join(",")
        ),
        Patch::AddEvent { name, .. } => format!("AddEvent({name})"),
        Patch::UpdateEvent { name, .. } => format!("UpdEvent({name})"),
        Patch::RemoveEvent { name } => format!("RmEvent({name})"),
        _ => "Other".into(),
    }
}

/// 整树重挂载：清空 #app 后按最新信号值与路由重建整棵树。
///
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
    if state.route.get() == Route::Adventure {
        battle::ensure_loaded(state);
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
        Route::Adventure => battle::Adventure(battle::AdventureProps {
            state: Some(Box::new(state.clone())),
            ..Default::default()
        }),
    };

    let home_active = state.route.get() == Route::Home;
    let shop_active = state.route.get() == Route::Shop;
    let inv_active = state.route.get() == Route::Inventory;
    let center_active = state.route.get() == Route::PokemonCenter;
    let mypm_active = state.route.get() == Route::MyPokemon;
    let storage_active = state.route.get() == Route::PokemonStorage;
    let adventure_active = state.route.get() == Route::Adventure;
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
    let adventure_cls = if adventure_active {
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
                        button { class: adventure_cls, "data-pm-nav": "adventure", "冒险" }
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
    state.battle.component_id.set(component_id as u32);

    let state_for_render = state.clone();
    let state_for_delegation = state.clone();
    tairitsu_vdom::runtime::update_render_function(component_id, move || {
        render_route(&state_for_render)
    });
    let vnode = render_route(&state);
    ROOT_COMPONENT.with(|id| id.set(component_id));

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
    PATCH_ROOT.with(|p| *p.borrow_mut() = Some(root_element.clone()));

    STATE_CACHE.with(|c| *c.borrow_mut() = Some(state.clone()));
    // 补丁批次滚动日志（data-dbg-plog，最近 10 批）+ 结构性兜底（§10.1.5）：
    // 含 Insert/Remove/Replace/Create 的批次整树替换——游戏单屏 20–50 塔片，
    // 替换开销与旧重挂相当（21–24ms），换来 DOM 与 diff 基线永远同源；
    // 纯属性/文本批次（战斗回合等热路径）保持增量。增量索引在深层
    // UpdateChild 路径上存在错位类缺口（背包搜索实测复现，见研究文档）。
    tairitsu_vdom::runtime::set_apply_patches_callback(
        |component_id: tairitsu_vdom::ComponentId,
         patches: Vec<tairitsu_vdom::Patch>| {
            let structural = patch_has_structural(&patches);
            let summary = format!(
                "{}n={} | {}",
                if structural { "[S] " } else { "" },
                patches.len(),
                patches
                    .iter()
                    .map(|p| patch_summary(p, 0))
                    .collect::<Vec<_>>()
                    .join(" ; ")
            );
            thread_local! {
                static PLOG: std::cell::RefCell<Vec<String>> =
                    const { std::cell::RefCell::new(Vec::new()) };
            }
            PLOG.with(|l| {
                let mut v = l.borrow_mut();
                v.push(summary);
                let len = v.len();
                if len > 10 {
                    v.drain(0..len - 10);
                }
                let joined = v
                    .iter()
                    .enumerate()
                    .map(|(i, e)| format!("[{i}] {e}"))
                    .collect::<Vec<_>>()
                    .join(" ||| ");
                let joined: String = joined.chars().take(900).collect();
                if let Some(app) = get_element_by_id(0, "app") {
                    use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
                    let _ = set_attribute(app, "data-dbg-plog", &joined);
                }
            });
            if structural {
                // 结构性批次：整树替换（§10.1.5 兜底策略，注释见 run_app 头部）。
                let state = STATE_CACHE.with(|s| s.borrow().clone());
                let Some(state) = state else { return };
                let vnode = render_route(&state);
                if let Ok(platform) = WitPlatform::new() {
                    let _ = platform.mount_vnode_to_app(vnode.clone());
                }
                if let Some(app_handle) = get_element_by_id(0, "app") {
                    let children_handle = get_child_nodes(app_handle);
                    if let Some(root_handle) = item(children_handle, 0) {
                        let root_element = WitElement::from_raw(root_handle);
                        // 不重跑 init_runtime：它会覆盖本回调。调度回调与根
                        // 无关仍然有效；apply 走本回调自己的 PATCH_ROOT。
                        PATCH_ROOT.with(|p| *p.borrow_mut() = Some(root_element));
                    }
                }
                tairitsu_vdom::runtime::store_initial_vnode(component_id, vnode);
                return;
            }
            let root = PATCH_ROOT.with(|p| p.borrow().clone());
            if let Some(root) = root {
                if let Ok(platform) = WitPlatform::new() {
                    tairitsu_web::wit_platform::wasm_impl::with_render_component(
                        component_id,
                        || {
                            let _ = platform.apply_patches(&root, &patches);
                        },
                    );
                }
            }
        },
    );

    // 调度（增量渲染通道，§10.1.5）：
    //   - 数据更新 → request_remount → mark_dirty → init_runtime 注册的
    //     rAF 调度（boot.js 用 MessageChannel 驱动 rAF）→ flush_render →
    //     diff → apply_patches。
    //   - 交互：事件委托（delegation.rs）在 #app 上注册一次。
    //   - 旧的 data-flush-pending 观察器桥与 pm-flush 整树重挂通道已退役
    //     （boot.js 的观察器不再有属性变化可观察，自然静默）。
    {
        use tairitsu_vdom::platform::DomOps;
        let app =
            WitElement::from_raw(get_element_by_id(0, "app").expect("#app mount point not found"));
        delegation::install(&app, &state_for_delegation);
    }

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
