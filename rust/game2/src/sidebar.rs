//! 右侧边栏（对齐 Dioxus 版 `components/layout/sidebar.rs`）。
//!
//! 三张卡：
//! - trainer 卡：头像 / 强度勋章 / 名字 / 金币与战绩 chip / 徽章按钮 /
//!   管理员可见的「后台管理」入口（旧版就在这里，不在顶部导航）。
//! - 宠物卡：携带宠物 2×3 迷你格 + 去仓库的箭头。
//! - 背包卡：四个分类迷你块（数量取自 inventory_stats）+ 去背包的箭头。

use hikari_components::{Button, ButtonProps};
use std::rc::Rc;

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use crate::app_state::AppState;
use crate::icon::{Icon, IconName, IconProps};
use crate::state::{PokemonBasic, ToastKind, UserProfile};
use crate::toast;

fn avatar_url(uid: u64) -> String {
    format!("plugin.php?id=pokemon:pokemon&endpoint=avatar&uid={uid}&size=middle")
}

/// 首次进入主界面时装载侧栏数据（缺什么补什么）。
pub fn ensure_loaded(state: &AppState) {
    if state.center.pokemons.get().is_none() {
        crate::pokemon_center::ensure_loaded(state);
    }
    if state.sidebar.bag_counts.get().is_none() {
        load_bag_counts(state);
    }
    if state.sidebar.badge_hidden.get().is_none() {
        load_badge(state);
    }
}

fn load_bag_counts(state: &AppState) {
    let st = state.clone();
    crate::api::inventory_stats(Box::new(move |result| {
        st.sidebar.bag_counts.inner().set(Some(result));
        crate::request_remount();
    }));
}

fn load_badge(state: &AppState) {
    let st = state.clone();
    crate::api::badge_status(Box::new(move |result| {
        st.sidebar
            .badge_hidden
            .inner()
            .set(Some(result.map(|s| s.hidden)));
        crate::request_remount();
    }));
}

/// 刷新 / 隐藏帖子徽章。`hide = Some(true)` 隐藏，`None` 或 `Some(false)` 刷新显示。
fn apply_badge(state: &AppState, hide: Option<bool>) {
    if state.sidebar.badge_busy.get() {
        return;
    }
    state.sidebar.badge_busy.inner().set(true);
    let st = state.clone();
    let target = hide.unwrap_or(false);
    crate::api::refresh_forum_badge(
        target,
        Box::new(move |result| {
            match result {
                Ok(status) => {
                    st.sidebar.badge_hidden.inner().set(Some(Ok(status.hidden)));
                    toast::show(
                        ToastKind::Success,
                        if status.hidden {
                            "帖子徽章已隐藏"
                        } else {
                            "帖子徽章已显示"
                        },
                    );
                }
                Err(e) => {
                    st.sidebar.badge_hidden.inner().set(Some(Err(e.clone())));
                    toast::show_error(format!("操作失败: {e}"));
                }
            }
            st.sidebar.badge_busy.inner().set(false);
            crate::request_remount();
        }),
    );
}

/// 携带中的宠物（site 1/2），最多 6 只进迷你格。
fn team(state: &AppState) -> Vec<PokemonBasic> {
    state
        .center
        .pokemons
        .get()
        .and_then(|r| r.ok())
        .map(|list| {
            list.into_iter()
                .filter(|p| p.site == 1 || p.site == 2)
                .take(6)
                .collect()
        })
        .unwrap_or_default()
}

/// 迷你格点击：战斗中开换宠弹窗；否则选中该宠物并跳转（对齐旧版）。
fn select_slot(state: &AppState, team: &[PokemonBasic], index: usize) {
    use crate::app_state::Route;

    let in_battle = crate::battle::user_in_battle(state);
    if in_battle {
        crate::battle::open_switch_modal(state);
        return;
    }
    let Some(pm) = team.get(index) else {
        return;
    };
    let pid = pm.id;
    state.sidebar.selected_id.inner().set(Some(pid));
    // 旧版迷你格和个人中心页读的是同一个 SELECTED_POKEMON_INDEX：点格子就是
    // 选中「个人中心要显示的那只」。这边分成两个信号，所以两处都要写。
    crate::my_pokemon::select_pokemon(state, pid);

    match state.route.get() {
        Route::MyPokemon | Route::PokemonStorage => {}
        Route::Adventure => {
            let st = state.clone();
            crate::api::set_first_pokemon(
                pid,
                Box::new(move |result| {
                    match result {
                        Ok(()) => {
                            toast::show(ToastKind::Success, "已切换操作宠物");
                            reload_team(&st);
                        }
                        Err(e) => toast::show_error(format!("切换失败: {e}")),
                    }
                    crate::request_remount();
                }),
            );
        }
        _ => {
            crate::navigate(state, Route::MyPokemon);
        }
    }
}

fn reload_team(state: &AppState) {
    let st = state.clone();
    crate::api::pokemon_list(Box::new(move |result| {
        st.center.pokemons.inner().set(Some(result));
        crate::request_remount();
    }));
}

/// 背包分类迷你块的数据（type_id / 标签 / 图标 / tooltip）。图标是内联的 lucide
/// SVG，与旧版同一份（`IconName::Pill`…），不是物品图片。
fn bag_tiles() -> [(u64, &'static str, IconName, &'static str); 4] {
    [
        (1, "回复药", IconName::Pill, "查看回复药"),
        (2, "精灵球", IconName::Circle, "查看精灵球"),
        (3, "进化石", IconName::Gem, "查看进化石"),
        (4, "强化", IconName::Zap, "查看强化道具"),
    ]
}

#[component]
pub fn Sidebar(state: Box<AppState>) -> VNode {
    let state = &*state;
    let profile = state.sidebar.profile.get().and_then(|r| r.ok());
    let Some(profile) = profile else {
        return VNode::empty();
    };
    let team = team(state);
    // 高亮的格子：战斗里是在场的那只（旧版按 my_pokemon.instance_id 找），
    // 平时是用户点选的那只。
    let active_id = match state.battle.scene.get() {
        Some(Some(ref scene)) if crate::battle::user_in_battle(state) => {
            Some(scene.my_pokemon.instance_id)
        }
        _ => state.sidebar.selected_id.get(),
    };

    // 点击迷你格：先把 team 摘出来给闭包（'static）。
    let team_for_click: Rc<Vec<PokemonBasic>> = Rc::new(team.clone());
    let click_state = state.clone();
    let on_select: Rc<dyn Fn(usize)> = Rc::new(move |index| {
        select_slot(&click_state, &team_for_click, index);
    });
    // 悬浮态：信号不跨渲染保留，显隐挂在 sidebar.hover_slot 上。
    let hover_slot = state.sidebar.hover_slot.get();
    let hover_state = state.clone();
    let set_hover: Rc<dyn Fn(Option<usize>)> = Rc::new(move |slot| {
        hover_state.sidebar.hover_slot.inner().set(slot);
        crate::request_remount();
    });

    let trainer = trainer_card(state, &profile);
    let pets = pets_card(state, &team, active_id, hover_slot, set_hover, on_select);
    let bag = bag_card(state);

    rsx! {
        aside { class: "app-sidebar",
            ..vec![trainer, pets, bag]
        }
    }
}

fn trainer_card(state: &AppState, profile: &UserProfile) -> VNode {
    let username = profile.username.clone();
    let alt = username.clone();
    let avatar = avatar_url(profile.uid);
    let money = profile.money.to_string();
    let record = format!("{}/{}", profile.wins, profile.losses);
    let strength = profile.strength_level.to_string();
    let is_admin = profile.is_admin;

    let busy = state.sidebar.badge_busy.get();
    let known = state.sidebar.badge_hidden.get();
    let (toggle_label, toggle_hide) = match &known {
        None => ("加载徽章状态...".to_string(), true),
        Some(Err(_)) => ("加载徽章状态...".to_string(), true),
        Some(Ok(true)) => ("显示宠物".to_string(), false),
        Some(Ok(false)) => ("隐藏宠物".to_string(), true),
    };
    let toggle_disabled = busy || known.is_none() || matches!(known, Some(Err(_)));

    let refresh_state = state.clone();
    let toggle_state = state.clone();
    let refresh_label = if busy { "刷新中..." } else { "刷新徽章" };

    rsx! {
        div { class: "sidebar-card",
            div { class: "sidebar-card-body",
                div { class: "trainer-profile",
                    div { class: "trainer-avatar",
                        img { src: avatar, alt: alt }
                        div { class: "strength-medal",
                            span { class: "medal-value", "{strength}" }
                        }
                    }
                    div { class: "trainer-info",
                        div { class: "trainer-left",
                            div { class: "trainer-name-row",
                                div { class: "trainer-name", "{username}" }
                            }
                            div { class: "trainer-stats-row",
                                span { class: "stat-chip",
                                    // 与旧版同一份 lucide 路径（IconName::Coins / Swords）。
                                    {Icon(IconProps {
                                        name: Some(IconName::Coins),
                                        size: Some(12),
                                        class: Some("stat-icon".to_string()),
                                    })}
                                    "{money}"
                                }
                                span { class: "stat-chip",
                                    {Icon(IconProps {
                                        name: Some(IconName::Swords),
                                        size: Some(12),
                                        class: Some("stat-icon".to_string()),
                                    })}
                                    "{record}"
                                }
                            }
                        }
                        div { class: "trainer-right",
                            Button {
                                class: ("admin-btn badge-refresh-btn").to_string(),
                                disabled: busy,
                                onclick: crate::button::click(move || apply_badge(&refresh_state, None)),
                                "{refresh_label}"
                                                        }
                            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                                class: ("admin-btn badge-toggle-btn").to_string(),
                                disabled: toggle_disabled,
                                onclick: crate::button::click(move || apply_badge(&toggle_state, Some(toggle_hide))),
                                "{toggle_label}"
                                                        } }, [("title", ("控制帖子旁是否显示携带的宠物").to_string())]) }
                            if is_admin {
                                a {
                                    class: "admin-btn",
                                    href: "plugin.php?id=pokemon:pokemon&index=admin",
                                    target: "_blank",
                                    rel: "noopener",
                                    "后台管理"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn pets_card(
    state: &AppState,
    team: &[PokemonBasic],
    active_id: Option<u64>,
    hover_slot: Option<usize>,
    set_hover: Rc<dyn Fn(Option<usize>)>,
    on_select: Rc<dyn Fn(usize)>,
) -> VNode {
    // 与 select_slot 同一个「在战斗里」的判断。
    let user_in_battle = crate::battle::user_in_battle(state);
    let grid = crate::mini_grid::mini_grid(
        team,
        active_id,
        user_in_battle,
        hover_slot,
        set_hover,
        on_select,
    );
    let storage_state = state.clone();
    rsx! {
        div { class: "sidebar-card",
            div { class: "sidebar-card-header",
                span { class: "sidebar-section-title", "宠物" }
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: ("sidebar-arrow-btn").to_string(),
                    onclick: crate::button::click(move || {
                        crate::navigate(&storage_state, crate::app_state::Route::PokemonStorage)
                    }),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 24 24",
                        fill: "currentColor",
                        dangerous_inner_html: r#"<path d="M8.59 16.59L13.17 12 8.59 7.41 10 6l6 6-6 6-1.41-1.41z" />"#,
                    }
                                } }, [("title", ("宠物仓库").to_string()), ("data-test-nav", ("storage").to_string())]) }
            }
            div { class: "sidebar-card-body", ..vec![grid] }
        }
    }
}

fn bag_card(state: &AppState) -> VNode {
    let counts = state
        .sidebar
        .bag_counts
        .get()
        .and_then(|r| r.ok())
        .unwrap_or_default();
    let tiles: Vec<VNode> = bag_tiles()
        .iter()
        .map(|(type_id, label, icon, tip)| {
            let count = counts
                .iter()
                .find(|(id, _)| id == type_id)
                .map(|(_, c)| *c)
                .unwrap_or(0)
                .to_string();
            let tile_tip = tip.to_string();
            let cat = *type_id;
            let tile_state = state.clone();
            let label = *label;
            rsx! {
                div {
                    class: "bag-grid-item",
                    title: tile_tip,
                    onclick: move |_| {
                        // 旧版：只记下初始分类，不在背包页时才切过去（背包页挂载时
                        // 取用，见 inventory::on_enter）。已经在背包页时什么都不发生。
                        tile_state.inv.entry_category.set(Some(cat as u32));
                        if tile_state.route.get() != crate::app_state::Route::Inventory {
                            crate::navigate(&tile_state, crate::app_state::Route::Inventory);
                        }
                    },
                    {Icon(IconProps {
                        name: Some(*icon),
                        size: Some(16),
                        class: Some("bag-item-icon".to_string()),
                    })}
                    span { class: "bag-item-label", "{label}" }
                    span { class: "bag-count-badge", "{count}" }
                }
            }
        })
        .collect();
    let inv_state = state.clone();
    rsx! {
        div { class: "sidebar-card",
            div { class: "sidebar-card-header",
                span { class: "sidebar-section-title", "背包" }
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: ("sidebar-arrow-btn").to_string(),
                    onclick: crate::button::click(move || {
                        crate::navigate(&inv_state, crate::app_state::Route::Inventory)
                    }),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 24 24",
                        fill: "currentColor",
                        dangerous_inner_html: r#"<path d="M8.59 16.59L13.17 12 8.59 7.41 10 6l6 6-6 6-1.41-1.41z" />"#,
                    }
                                } }, [("title", ("查看背包").to_string()), ("data-test-nav", ("inventory").to_string())]) }
            }
            div { class: "sidebar-card-body",
                div { class: "bag-grid-mini", ..tiles }
            }
        }
    }
}
