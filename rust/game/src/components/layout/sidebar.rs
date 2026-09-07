use dioxus::prelude::*;

use super::{Page, CURRENT_PAGE, INITIAL_INVENTORY_CATEGORY};
use crate::{
    components::pokemon::{MiniPokemonInfo, MiniSkillInfo, PokemonMiniGrid},
    state::{
        refresh_pokemon_list, request_switch_pokemon, show_error, show_success, BATTLE_STATE,
        POKEMON_STATE, SELECTED_POKEMON_INDEX, USER_STATE,
    },
    utils::api_client::NewApiClient,
};
use _utils::types::api_pokemon::PokemonBasic;

fn avatar_url(uid: u64) -> String {
    format!(
        "plugin.php?id=pokemon:pokemon&endpoint=avatar&uid={}&size=middle",
        uid
    )
}

fn battle_scene_instance_id(scene: &Option<_utils::types::api_battle::BattleScene>) -> Option<u64> {
    scene.as_ref().map(|s| s.my_pokemon.instance_id)
}

fn to_mini_info(pm: &PokemonBasic, is_in_battle: bool) -> MiniPokemonInfo {
    let skills = pm
        .skills
        .iter()
        .map(|s| MiniSkillInfo {
            name: s.name.clone(),
            pp: s.pp,
            max_pp: s.max_pp,
            skill_type: s.skill_type.clone(),
        })
        .collect();

    MiniPokemonInfo {
        type_id: pm.type_id,
        name: pm.name.clone(),
        nickname: pm.nickname.clone(),
        level: pm.level,
        hp: pm.hp,
        max_hp: pm.max_hp,
        is_in_battle,
        skills,
        state: pm.state,
        state_text: pm.state_text.clone(),
        exp: pm.exp,
        exp_for_current_level: pm.exp_for_current_level,
        exp_for_next_level: pm.exp_for_next_level,
    }
}

fn arrow_right_icon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M8.59 16.59L13.17 12 8.59 7.41 10 6l6 6-6 6-1.41-1.41z" }
        }
    }
}

#[component]
pub fn Sidebar() -> Element {
    let current_page = *CURRENT_PAGE.read();

    let is_user_in_battle = {
        let user_state = USER_STATE.read();
        user_state.is_in_battle()
    };

    let (
        mini_pokemons,
        team_pokemon_ids,
        active_index,
        username,
        money,
        strength_level,
        wins,
        losses,
        uid,
        is_admin,
        category_counts,
    ) = {
        let pokemon_state = POKEMON_STATE.read();
        let user_state = USER_STATE.read();

        let mini_pokemons: Vec<_> = pokemon_state
            .list
            .iter()
            .filter(|p| p.site == 1 || p.site == 2)
            .take(6)
            .map(|p| to_mini_info(p, is_user_in_battle && p.site == 1))
            .collect();

        let team_pokemon_ids: Vec<u64> = pokemon_state
            .list
            .iter()
            .filter(|p| p.site == 1 || p.site == 2)
            .take(6)
            .map(|p| p.id)
            .collect();

        let active_index = if is_user_in_battle {
            battle_scene_instance_id(&BATTLE_STATE.read().scene)
                .and_then(|inst_id| team_pokemon_ids.iter().position(|&id| id == inst_id))
        } else {
            let selected_id = *SELECTED_POKEMON_INDEX.read();
            selected_id.and_then(|id| team_pokemon_ids.iter().position(|&tid| tid == id))
        };

        let username = user_state.get_username().to_string();
        let money = user_state.get_money();
        let strength_level = user_state
            .profile
            .as_ref()
            .and_then(|p| p.strength_level)
            .unwrap_or(0) as u32;
        let wins = user_state
            .profile
            .as_ref()
            .and_then(|p| p.wins)
            .unwrap_or(0) as u32;
        let losses = user_state
            .profile
            .as_ref()
            .and_then(|p| p.losses)
            .unwrap_or(0) as u32;
        let uid = user_state.profile.as_ref().map(|p| p.uid).unwrap_or(0);
        let is_admin = user_state
            .profile
            .as_ref()
            .map(|p| p.is_admin)
            .unwrap_or(false);
        let category_counts: Vec<(u64, i64)> = user_state
            .inventory
            .as_ref()
            .map(|s| s.categories.iter().map(|c| (c.type_id, c.count)).collect())
            .unwrap_or_default();

        (
            mini_pokemons,
            team_pokemon_ids,
            active_index,
            username,
            money,
            strength_level,
            wins,
            losses,
            uid,
            is_admin,
            category_counts,
        )
    };

    let get_count = |type_id: u64| -> i64 {
        category_counts
            .iter()
            .find(|(id, _)| *id == type_id)
            .map(|(_, c)| *c)
            .unwrap_or(0)
    };

    let mut refreshing_badge = use_signal(|| false);
    let mut badge_hidden = use_signal(|| false);

    // 挂载时同步一次当前徽章可见状态
    let _badge_status_res = use_resource(move || async move {
        let api = NewApiClient::new();
        if let Ok(status) = api.get_badge_status().await {
            badge_hidden.set(status.hidden);
        }
    });

    let refresh_badge = move |_| {
        refreshing_badge.set(true);
        let api = NewApiClient::new();
        spawn(async move {
            match api.refresh_forum_badge(false).await {
                Ok(status) => {
                    badge_hidden.set(status.hidden);
                    show_success("帖子徽章已刷新");
                }
                Err(e) => show_error(format!("刷新失败: {}", e)),
            }
            refreshing_badge.set(false);
        });
    };

    // 切换帖子旁是否显示携带的宠物（隐藏 = 清空徽章数据，显示 = 重新同步并显示）
    let toggle_badge_visibility = move |_| {
        refreshing_badge.set(true);
        let target_hidden = !*badge_hidden.read();
        let api = NewApiClient::new();
        spawn(async move {
            match api.refresh_forum_badge(target_hidden).await {
                Ok(status) => {
                    badge_hidden.set(status.hidden);
                    show_success(if status.hidden {
                        "帖子徽章已隐藏"
                    } else {
                        "帖子徽章已显示"
                    });
                }
                Err(e) => {
                    badge_hidden.set(!target_hidden);
                    show_error(format!("操作失败: {}", e));
                }
            }
            refreshing_badge.set(false);
        });
    };

    rsx! {
        aside { class: "app-sidebar",
            div { class: "sidebar-card",
                div { class: "sidebar-card-body",
                    div { class: "trainer-profile",
                        div { class: "trainer-avatar",
                            img { src: avatar_url(uid), alt: username.clone() }
                            div { class: "strength-medal",
                                span { class: "medal-value", "{strength_level}" }
                            }
                        }
                        div { class: "trainer-info",
                            div { class: "trainer-left",
                                div { class: "trainer-name-row",
                                    div { class: "trainer-name", "{username}" }
                                }
                                div { class: "trainer-stats-row",
                                    span { class: "stat-chip",
                                        span { class: "stat-icon", "💰" }
                                        "{money}"
                                    }
                                    span { class: "stat-chip",
                                        span { class: "stat-icon", "⚔️" }
                                        "{wins}/{losses}"
                                    }
                                }
                            }
                            div { class: "trainer-right",
                                button {
                                    class: "admin-btn badge-refresh-btn",
                                    disabled: *refreshing_badge.read(),
                                    onclick: refresh_badge,
                                    if *refreshing_badge.read() {
                                        "刷新中..."
                                    } else {
                                        "刷新徽章"
                                    }
                                }
                                button {
                                    class: "admin-btn badge-toggle-btn",
                                    disabled: *refreshing_badge.read(),
                                    onclick: toggle_badge_visibility,
                                    title: "控制帖子旁是否显示携带的宠物",
                                    if *badge_hidden.read() {
                                        "显示宠物"
                                    } else {
                                        "隐藏宠物"
                                    }
                                }
                                if is_admin {
                                    a {
                                        class: "admin-btn",
                                        href: "plugin.php?id=pokemon:pokemon&index=admin",
                                        target: "_blank",
                                        "后台管理"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "sidebar-card",
                div { class: "sidebar-card-header",
                    span { class: "sidebar-section-title", "宠物" }
                    button {
                        class: "sidebar-arrow-btn",
                        title: "宠物仓库",
                        onclick: move |_| *CURRENT_PAGE.write() = Page::PokemonStorage,
                        {arrow_right_icon()}
                    }
                }
                div { class: "sidebar-card-body",
                    PokemonMiniGrid {
                        pokemons: mini_pokemons,
                        active_index,
                        on_select: move |index: usize| {
                            if is_user_in_battle {
                                request_switch_pokemon();
                                return;
                            }

                            if let Some(&pokemon_id) = team_pokemon_ids.get(index) {
                                *SELECTED_POKEMON_INDEX.write() = Some(pokemon_id);

                                if current_page == Page::MyPokemon
                                    || current_page == Page::PokemonStorage
                                {
                                    return;
                                }

                                if current_page != Page::Adventure {
                                    *CURRENT_PAGE.write() = Page::MyPokemon;
                                    return;
                                }

                                let api = NewApiClient::new();
                                spawn(async move {
                                    match api.set_first_pokemon(pokemon_id).await {
                                        Ok(_) => {
                                            show_success("已切换操作宠物");
                                            refresh_pokemon_list();
                                        }
                                        Err(e) => {
                                            show_error(format!("切换失败: {}", e));
                                        }
                                    }
                                });
                            }
                        },
                    }
                }
            }

            div { class: "sidebar-card",
                div { class: "sidebar-card-header",
                    span { class: "sidebar-section-title", "背包" }
                    button {
                        class: "sidebar-arrow-btn",
                        title: "查看背包",
                        onclick: move |_| *CURRENT_PAGE.write() = Page::Inventory,
                        {arrow_right_icon()}
                    }
                }
                div { class: "sidebar-card-body",
                    div { class: "bag-grid-mini",
                        div {
                            class: "bag-grid-item",
                            title: "查看回复药",
                            onclick: move |_| {
                                *INITIAL_INVENTORY_CATEGORY.write() = Some(1);
                                if current_page != Page::Inventory {
                                    *CURRENT_PAGE.write() = Page::Inventory;
                                }
                            },
                            span { class: "bag-item-icon", "💊" }
                            span { class: "bag-item-label", "回复药" }
                            span { class: "bag-count-badge", "{get_count(1)}" }
                        }
                        div {
                            class: "bag-grid-item",
                            title: "查看精灵球",
                            onclick: move |_| {
                                *INITIAL_INVENTORY_CATEGORY.write() = Some(2);
                                if current_page != Page::Inventory {
                                    *CURRENT_PAGE.write() = Page::Inventory;
                                }
                            },
                            span { class: "bag-item-icon", "🔴" }
                            span { class: "bag-item-label", "精灵球" }
                            span { class: "bag-count-badge", "{get_count(2)}" }
                        }
                        div {
                            class: "bag-grid-item",
                            title: "查看进化石",
                            onclick: move |_| {
                                *INITIAL_INVENTORY_CATEGORY.write() = Some(3);
                                if current_page != Page::Inventory {
                                    *CURRENT_PAGE.write() = Page::Inventory;
                                }
                            },
                            span { class: "bag-item-icon", "💎" }
                            span { class: "bag-item-label", "进化石" }
                            span { class: "bag-count-badge", "{get_count(3)}" }
                        }
                        div {
                            class: "bag-grid-item",
                            title: "查看强化道具",
                            onclick: move |_| {
                                *INITIAL_INVENTORY_CATEGORY.write() = Some(4);
                                if current_page != Page::Inventory {
                                    *CURRENT_PAGE.write() = Page::Inventory;
                                }
                            },
                            span { class: "bag-item-icon", "⚡" }
                            span { class: "bag-item-label", "强化" }
                            span { class: "bag-count-badge", "{get_count(4)}" }
                        }
                    }
                }
            }
        }
    }
}
