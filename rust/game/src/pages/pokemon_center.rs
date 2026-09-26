use crate::prelude::*;

use crate::{
    components::layout::IMG_PATH,
    state::{
        refresh_pokemon_list, refresh_user_profile_state, show_error, show_success, show_warning,
        use_pokemon_state, POKEMON_STATE, USER_STATE,
    },
    utils::{
        api_client::NewApiClient,
        pokemon::{hp_class, hp_percent, is_negative_state},
    },
};

#[component]
pub fn PokemonCenter() -> Element {
    use_pokemon_state();

    // 每次打开治疗中心时都刷新宠物列表，确保数据是最新的
    let _resource = use_resource(move || async move {
        refresh_pokemon_list();
        refresh_user_profile_state();
    });

    let loading = use_signal(|| false);

    let is_in_battle_state = {
        USER_STATE
            .read()
            .profile
            .as_ref()
            .map(|p| p.npcid > 0)
            .unwrap_or(false)
    };

    let (injured_pokemons, has_healable_pokemons, all_healthy, is_loading, list_is_empty) = {
        let pokemon_list = POKEMON_STATE.read();
        // 医疗只针对身上携带的宠物（site 1/2，上限 6），箱子里的不在这里显示
        let carried: Vec<_> = pokemon_list
            .list
            .iter()
            .filter(|p| p.site == 1 || p.site == 2)
            .cloned()
            .collect();
        let injured: Vec<_> = carried
            .iter()
            .filter(|p| p.hp < p.max_hp || is_negative_state(p.state))
            .cloned()
            .collect();

        let healable = carried.iter().any(|p| {
            let needs_healing = p.hp < p.max_hp || is_negative_state(p.state);
            needs_healing && !(is_in_battle_state && p.site == 1)
        });

        let healthy = injured.is_empty() && !carried.is_empty();
        let ld = pokemon_list.loading;
        let empty = carried.is_empty();
        (injured, healable, healthy, ld, empty)
    };

    let loading_for_heal_all = loading;
    let is_in_battle_for_heal_all = is_in_battle_state;
    let heal_all = move |_| {
        let pokemons: Vec<(u64, String, u8)> = POKEMON_STATE
            .read()
            .list
            .iter()
            .filter(|p| {
                if !(p.site == 1 || p.site == 2) {
                    return false;
                }
                let is_negative = is_negative_state(p.state);
                let needs_healing = p.hp < p.max_hp || is_negative;
                needs_healing && !(is_in_battle_for_heal_all && p.site == 1)
            })
            .map(|p| (p.id, p.name.clone(), p.site))
            .collect();

        if pokemons.is_empty() {
            show_warning("没有需要治疗的宠物！");
            return;
        }

        let api = NewApiClient::new();
        let mut loading_clone = loading_for_heal_all;
        spawn(async move {
            loading_clone.set(true);
            let mut total_cost = 0i64;
            let mut healed_count = 0u32;
            let mut last_error = None::<String>;
            for (pid, pname, _site) in &pokemons {
                match api.heal_pokemon(*pid).await {
                    Ok(result) => {
                        total_cost += result.cost;
                        healed_count += 1;
                    }
                    Err(e) => {
                        last_error = Some(format!("{}: {}", pname, e));
                    }
                }
            }
            if healed_count > 0 {
                show_success(format!(
                    "已治疗 {} 只宠物，共花费 {} 金币",
                    healed_count, total_cost
                ));
                refresh_pokemon_list();
            } else if let Some(err) = last_error {
                show_error(format!("治疗失败: {}", err));
            }
            loading_clone.set(false);
        });
    };

    rsx! {
        div { class: "page-pokemon-center",
            div { class: "center-card",
                div { class: "center-card-header",
                    img {
                        class: "header-icon-img",
                        src: "{IMG_PATH}/rpg/center_health.gif",
                        alt: "",
                    }
                    span { "宠物中心" }
                }

                div { class: "center-card-body",
                    div { class: "center-welcome-banner",
                        img {
                            class: "welcome-icon-img",
                            src: "{IMG_PATH}/rpg/doctor.gif",
                            alt: "",
                        }
                        div { class: "welcome-text",
                            h3 { "欢迎来到宠物中心！" }
                            p { "辛苦了！这里可以恢复宠物的体力。" }
                        }
                        if has_healable_pokemons {
                            button {
                                class: "heal-all-btn",
                                onclick: heal_all,
                                disabled: *loading.read(),
                                "全部治疗"
                            }
                        }
                    }

                    if is_loading {
                        div { class: "center-loading", "正在加载宠物数据..." }
                    } else if list_is_empty {
                        div { class: "center-empty",
                            img {
                                class: "empty-icon-img",
                                src: "{IMG_PATH}/item/jlq.gif",
                                alt: "",
                            }
                            p { "你还没有宠物" }
                        }
                    } else if all_healthy {
                        div { class: "center-healthy",
                            img {
                                class: "healthy-icon-img",
                                src: "{IMG_PATH}/rpg/center_health.gif",
                                alt: "",
                            }
                            p { "所有宠物都很健康，不需要治疗！" }
                        }
                    } else {
                        div { class: "center-pokemon-scroll",
                            div { class: "center-pokemon-list",
                                for pm in injured_pokemons.iter() {
                                    {
                                        let pokemon_id = pm.id;
                                        let pokemon_name_for_display = pm.name.clone();
                                        let pokemon_name_for_handler = pm.name.clone();
                                        let type_id = pm.type_id;
                                        let current_hp = pm.hp;
                                        let max_hp = pm.max_hp;
                                        let level = pm.level;
                                        let hp_pct = hp_percent(current_hp, max_hp);
                                        let hp_cls = hp_class(current_hp, max_hp);

                                        let pm_state_text = pm.state_text.clone();
                                        let pm_state_class = pm.state_class.clone();

                                        let is_in_battle = is_in_battle_state && pm.site == 1;

                                        let loading_clone = loading;
                                        let onclick_handler = move |_| {
                                            let name = pokemon_name_for_handler.clone();
                                            let api = NewApiClient::new();
                                            let mut loading_ref = loading_clone;
                                            let in_battle = is_in_battle;
                                            spawn(async move {
                                                loading_ref.set(true);

                                                if in_battle {
                                                    match api.heal_and_flee(pokemon_id).await {
                                                        Ok(_) => {
                                                            show_success(format!("{}已脱战并治疗", name));
                                                            refresh_pokemon_list();
                                                            refresh_user_profile_state();
                                                        }
                                                        Err(e) => {
                                                            show_error(format!("脱战治疗失败: {}", e));
                                                        }
                                                    }
                                                } else {
                                                    match api.heal_pokemon(pokemon_id).await {
                                                        Ok(result) => {
                                                            show_success(
                                                                format!("{}已治疗，花费 {} 金币", name, result.cost),
                                                            );
                                                            refresh_pokemon_list();
                                                        }
                                                        Err(e) => {
                                                            show_error(format!("治疗失败: {}", e));
                                                        }
                                                    }
                                                }
                                                loading_ref.set(false);
                                            });
                                        };
                                        let btn_class = if is_in_battle {
                                            "heal-btn heal-btn-warning"
                                        } else {
                                            "heal-btn"
                                        };
                                        let btn_text = if is_in_battle { "脱战并治疗" } else { "治疗" };
                                        rsx! {
                                            div { class: "heal-pokemon-card",
                                                div { class: "heal-pm-sprite",
                                                    img {
                                                        src: "{IMG_PATH}/spm/{type_id}.gif",
                                                        alt: "{pokemon_name_for_display}",
                                                    }
                                                }
                                                div { class: "heal-pm-info",
                                                    div { class: "heal-pm-name-row",
                                                        span { class: "heal-pm-name", "{pokemon_name_for_display}" }
                                                        span { class: "heal-pm-level", "Lv.{level}" }
                                                    }
                                                    div { class: "heal-pm-state",
                                                        span { class: "state-label state-{pm_state_class}", "{pm_state_text}" }
                                                    }
                                                    div { class: "heal-pm-hp",
                                                        span { class: "hp-label", "HP" }
                                                        div { class: "hp-track",
                                                            div { class: "hp-fill {hp_cls}", style: "width: {hp_pct}%" }
                                                        }
                                                        span { class: "hp-text", "{current_hp}/{max_hp}" }
                                                    }
                                                }
                                                button {
                                                    class: "{btn_class}",
                                                    onclick: onclick_handler,
                                                    disabled: *loading.read(),
                                                    "{btn_text}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
