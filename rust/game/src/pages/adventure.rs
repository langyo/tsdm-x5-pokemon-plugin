use dioxus::prelude::*;

use crate::{
    components::common::Modal,
    components::layout::IMG_PATH_REMOTE,
    pages::BattlePage,
    state::{
        clear_battle_scene, refresh_inventory_state, refresh_pokemon_list,
        refresh_user_profile_state, set_battle_scene, show_error, use_pokemon_state,
        BATTLE_STATE, POKEMON_STATE,
    },
    utils::api_client::NewApiClient,
};
use _utils::types::{
    api_map::MapInfo, api_map_region::MapRegion,
    api_pokemon::{ApiResponse as PokemonApiResponse, PokemonBasic}, api_user::InventoryItem,
};

#[component]
pub fn Adventure() -> Element {
    use_pokemon_state();
    let mut loading = use_signal(|| false);
    let mut message_log = use_signal(Vec::<String>::new);
    let mut selected_region = use_signal(|| None::<String>);
    let mut selected_map_for_modal = use_signal(|| None::<MapInfo>);
    let min_level_filter = use_signal(|| None::<u32>);
    let max_level_filter = use_signal(|| None::<u32>);

    let mut maps: Signal<Vec<MapInfo>> = use_signal(Vec::new);
    let mut maps_loading = use_signal(|| true);

    let mut battle_items = use_signal(Vec::<InventoryItem>::new);
    let mut battle_balls = use_signal(Vec::<InventoryItem>::new);

    let mut skill_selection_mode = use_signal(|| None::<(u64, String, Vec<_utils::types::api_battle::BattleSkill>)>);

    let _resource = use_resource(move || async move {
        let api = NewApiClient::new();

        match api.recover_battle().await {
            Ok(scene) => {
                set_battle_scene(Some(scene.clone()));
                message_log.set(vec!["战斗已恢复".to_string()]);

                let api2 = NewApiClient::new();
                if let Ok(data) = api2.get_user_inventory(Some(1), 1).await {
                    battle_items.set(data.items);
                }
                let api3 = NewApiClient::new();
                if let Ok(data) = api3.get_user_inventory(Some(2), 1).await {
                    battle_balls.set(data.items);
                }
            }
            Err(_) => {
                clear_battle_scene();
            }
        }
    });

    let _resource = use_resource(move || {
        let min = *min_level_filter.read();
        let max = *max_level_filter.read();
        async move {
            maps_loading.set(true);

            let api = NewApiClient::new();
            match api.get_maps(min, max).await {
                Ok(data) => {
                    maps.set(data.maps);
                }
                Err(e) => {
                    show_error(format!("加载地图失败：{}", e));
                }
            }
            maps_loading.set(false);
        }
    });

    let mut start_boss_adventure = move |map_id: u64, boss_type_id: u64| {
        selected_map_for_modal.set(None);

        let api = NewApiClient::new();
        spawn(async move {
            loading.set(true);

            match api.start_battle(map_id, Some(boss_type_id)).await {
                Ok(scene) => {
                    let map_name = scene.map_name.clone();
                    let pokemon_name = scene.wild_pokemon.name.clone();

                    set_battle_scene(Some(scene.clone()));
                    message_log.set(vec![format!(
                        "在地图 {} 遇到了 Boss {}!",
                        map_name, pokemon_name
                    )]);

                    let api2 = NewApiClient::new();
                    if let Ok(data) = api2.get_battle_items().await {
                        battle_items.set(data.items);
                    }
                    if let Ok(data) = api2.get_user_inventory(Some(2), 1).await {
                        battle_balls.set(data.items);
                    }
                }
                Err(e) => {
                    show_error(format!("遇敵失败：{}", e));
                }
            }
            loading.set(false);
        });
    };

    let regions = use_memo(move || MapRegion::from_maps(&maps.read()));

    let mut start_adventure = move |map_id: u64| {
        selected_map_for_modal.set(None);

        let api = NewApiClient::new();
        spawn(async move {
            loading.set(true);

            match api.start_battle(map_id, None).await {
                Ok(scene) => {
                    let map_name = scene.map_name.clone();
                    let pokemon_name = scene.wild_pokemon.name.clone();

                    set_battle_scene(Some(scene.clone()));
                    message_log.set(vec![format!(
                        "在地图 {} 遇到了野生的 {}！",
                        map_name, pokemon_name
                    )]);

                    let api2 = NewApiClient::new();
                    if let Ok(data) = api2.get_battle_items().await {
                        battle_items.set(data.items);
                    }
                    if let Ok(data) = api2.get_user_inventory(Some(2), 1).await {
                        battle_balls.set(data.items);
                    }
                }
                Err(e) => {
                    show_error(format!("遇敌失败：{}", e));
                }
            }
            loading.set(false);
        });
    };

    let use_skill = move |skill_id: u64| {
        if let Some(current_battle) = &BATTLE_STATE.read().scene {
            let battle_id = current_battle.battle_id.clone();
            let api = NewApiClient::new();
            spawn(async move {
                loading.set(true);

                match api.use_skill(&battle_id, skill_id).await {
                    Ok(scene) => {
                        let mut log = message_log.read().clone();
                        log.push("你使用了技能！".to_string());
                        if scene.status != _utils::types::api_battle::BattleStatus::Active {
                            if scene.status == _utils::types::api_battle::BattleStatus::Victory {
                                log.push(format!(
                                    "战斗胜利！获得 {} 经验和 {} 金币",
                                    scene.rewards.as_ref().map(|r| r.exp).unwrap_or(0),
                                    scene.rewards.as_ref().map(|r| r.money).unwrap_or(0)
                                ));
                            } else if scene.status
                                == _utils::types::api_battle::BattleStatus::Defeat
                            {
                                log.push(format!("{} 倒下了！", scene.my_pokemon.name));
                            }
                            set_battle_scene(Some(scene.clone()));
                        } else {
                            set_battle_scene(Some(scene.clone()));
                        }
                        message_log.set(log);
                        refresh_pokemon_list();
                    }
                    Err(e) => {
                        show_error(format!("攻击失败：{}", e));
                    }
                }
                loading.set(false);
            });
        }
    };

    let use_item_on_skill = move |item_id: u64, skill_id: u64| {
        let api = NewApiClient::new();
        spawn(async move {
            loading.set(true);

            match api.use_item_on_skill(item_id, skill_id).await {
                Ok(scene) => {
                    skill_selection_mode.set(None);
                    message_log.write().push("对技能使用了PP恢复道具！".to_string());

                    let api2 = NewApiClient::new();
                    if let Ok(data) = api2.get_battle_items().await {
                        battle_items.set(data.items);
                    }
                    let api3 = NewApiClient::new();
                    if let Ok(data) = api3.get_user_inventory(Some(2), 1).await {
                        battle_balls.set(data.items);
                    }

                    set_battle_scene(Some(scene.clone()));
                    refresh_pokemon_list();
                }
                Err(e) => {
                    show_error(format!("使用物品失败：{}", e));
                }
            }
            loading.set(false);
        });
    };

    let use_item_in_battle = move |item_id: u64| {
        let api = NewApiClient::new();
        spawn(async move {
            loading.set(true);

            match api.use_item_in_battle_raw(item_id).await {
                Ok(response_text) => {
                    if let Ok(skill_resp) = serde_json::from_str::<PokemonApiResponse<_utils::types::api_battle::SkillSelectionResponse>>(&response_text) {
                        if skill_resp.success {
                            if let Some(data) = skill_resp.data {
                                if data.requires_skill_selection {
                                    skill_selection_mode.set(Some((data.item_id, data.item_name, data.available_skills)));
                                    loading.set(false);
                                    return;
                                }
                            }
                        }
                    }

                    match api.use_item_in_battle(item_id).await {
                        Ok(scene) => {
                            message_log.write().push("使用了物品！".to_string());

                            let api2 = NewApiClient::new();
                            if let Ok(data) = api2.get_battle_items().await {
                                battle_items.set(data.items);
                            }
                            let api3 = NewApiClient::new();
                            if let Ok(data) = api3.get_user_inventory(Some(2), 1).await {
                                battle_balls.set(data.items);
                            }

                            set_battle_scene(Some(scene.clone()));
                            refresh_pokemon_list();
                        }
                        Err(e) => {
                            show_error(format!("使用物品失败：{}", e));
                        }
                    }
                }
                Err(e) => {
                    show_error(format!("使用物品失败：{}", e));
                }
            }
            loading.set(false);
        });
    };

    let attack = move || {
        if let Some(current_battle) = &BATTLE_STATE.read().scene {
            let battle_id = current_battle.battle_id.clone();
            let api = NewApiClient::new();
            spawn(async move {
                loading.set(true);
                match api.use_skill(&battle_id, 0).await {
                    Ok(scene) => {
                        let mut log = message_log.read().clone();
                        log.push("使用了普通攻击！".to_string());
                        if scene.status != _utils::types::api_battle::BattleStatus::Active {
                            if scene.status == _utils::types::api_battle::BattleStatus::Victory {
                                log.push(format!(
                                    "战斗胜利！获得 {} 经验和 {} 金币",
                                    scene.rewards.as_ref().map(|r| r.exp).unwrap_or(0),
                                    scene.rewards.as_ref().map(|r| r.money).unwrap_or(0)
                                ));
                            } else if scene.status
                                == _utils::types::api_battle::BattleStatus::Defeat
                            {
                                log.push(format!("{} 倒下了！", scene.my_pokemon.name));
                            }
                            set_battle_scene(Some(scene.clone()));
                        } else {
                            set_battle_scene(Some(scene.clone()));
                        }
                        message_log.set(log);
                        refresh_pokemon_list();
                    }
                    Err(e) => {
                        show_error(format!("攻击失败：{}", e));
                    }
                }
                loading.set(false);
            });
        }
    };

    let capture = move |ball_id: u64| {
        let api = NewApiClient::new();
        spawn(async move {
            loading.set(true);
            match api.capture(ball_id).await {
                Ok(scene) => {
                    message_log.write().push(scene.message.clone());
                    set_battle_scene(Some(scene.clone()));

                    let api2 = NewApiClient::new();
                    if let Ok(data) = api2.get_user_inventory(Some(2), 1).await {
                        battle_balls.set(data.items);
                    }

                    refresh_pokemon_list();
                }
                Err(e) => {
                    show_error(format!("捕捉失败：{}", e));
                }
            }
            loading.set(false);
        });
    };

    let flee_battle = move |_| {
        if let Some(current_battle) = &BATTLE_STATE.read().scene {
            let battle_id = current_battle.battle_id.clone();
            let api = NewApiClient::new();
            spawn(async move {
                loading.set(true);

                match api.flee(&battle_id).await {
                    Ok(scene) => {
                        let mut log = message_log.read().clone();
                        if scene.status == _utils::types::api_battle::BattleStatus::Fled {
                            log.push("成功逃跑！".to_string());
                            refresh_user_profile_state();
                        } else {
                            log.push("逃跑失败！".to_string());
                        }
                        set_battle_scene(Some(scene.clone()));
                        message_log.set(log);
                        refresh_pokemon_list();
                    }
                    Err(e) => {
                        show_error(format!("逃跑失败：{}", e));
                    }
                }
                loading.set(false);
            });
        }
    };

    let refresh_battle_items = move |_| {
        let api = NewApiClient::new();
        spawn(async move {
            if let Ok(data) = api.get_battle_items().await {
                battle_items.set(data.items);
            }
        });
    };

    let refresh_battle_balls = move |_| {
        let api = NewApiClient::new();
        spawn(async move {
            if let Ok(data) = api.get_user_inventory(Some(2), 1).await {
                battle_balls.set(data.items);
            }
        });
    };

    let mut end_battle_with_refresh = move |_| {
        clear_battle_scene();
        message_log.set(Vec::new());
        refresh_pokemon_list();
        refresh_user_profile_state();
        refresh_inventory_state();
    };

    let switch_pokemon = move || {
        if let Some(current_battle) = &BATTLE_STATE.read().scene {
            let battle_id = current_battle.battle_id.clone();
            let api = NewApiClient::new();
            spawn(async move {
                loading.set(true);
                match api.switch_pokemon(&battle_id).await {
                    Ok(scene) => {
                        let mut log = message_log.read().clone();
                        log.push("切换了上场宠物！".to_string());
                        if scene.status != _utils::types::api_battle::BattleStatus::Active {
                            if scene.status == _utils::types::api_battle::BattleStatus::Victory {
                                log.push("战斗胜利！".to_string());
                            } else if scene.status
                                == _utils::types::api_battle::BattleStatus::Defeat
                            {
                                log.push("战斗失败...".to_string());
                            }
                            set_battle_scene(Some(scene.clone()));
                        } else {
                            set_battle_scene(Some(scene.clone()));
                        }
                        message_log.set(log);
                        refresh_pokemon_list();
                    }
                    Err(e) => {
                        show_error(format!("切换宠物失败：{}", e));
                    }
                }
                loading.set(false);
            });
        }
    };

    let replace_pokemon = move |pokemon_id: u64| {
        if let Some(current_battle) = &BATTLE_STATE.read().scene {
            let battle_id = current_battle.battle_id.clone();
            let api = NewApiClient::new();
            spawn(async move {
                loading.set(true);

                match api.replace_pokemon(&battle_id, pokemon_id).await {
                    Ok(scene) => {
                        let mut log = message_log.read().clone();
                        log.push("更换了上场宠物！".to_string());

                        if scene.status != _utils::types::api_battle::BattleStatus::Active {
                            if scene.status == _utils::types::api_battle::BattleStatus::Victory {
                                log.push("战斗胜利！".to_string());
                            } else if scene.status
                                == _utils::types::api_battle::BattleStatus::Defeat
                            {
                                log.push("战斗失败...".to_string());
                            }
                            set_battle_scene(Some(scene.clone()));
                        } else {
                            set_battle_scene(Some(scene.clone()));
                        }
                        message_log.set(log);
                        refresh_pokemon_list();
                    }
                    Err(e) => {
                        show_error(format!("切换宠物失败：{}", e));
                    }
                }
                loading.set(false);
            });
        }
    };

    let get_recommendation = |map: &MapInfo, pokemons: &[PokemonBasic]| -> String {
        if pokemons.is_empty() {
            return "请先获得一只宝可梦".to_string();
        }

        let pokemon_level = pokemons[0].level;

        if pokemon_level < map.min_level {
            format!("等级不足，建议等级 Lv.{} 以上", map.min_level)
        } else if pokemon_level > map.max_level + 10 {
            "等级过高，经验收益较低".to_string()
        } else if pokemon_level >= map.min_level && pokemon_level <= map.max_level {
            "等级适中，推荐挑战".to_string()
        } else {
            "可以挑战".to_string()
        }
    };

    let get_difficulty_class = |map: &MapInfo, pokemons: &[PokemonBasic]| -> &'static str {
        if pokemons.is_empty() {
            return "difficulty-unknown";
        }

        let pokemon_level = pokemons[0].level;

        if pokemon_level < map.min_level {
            "difficulty-hard"
        } else if pokemon_level > map.max_level + 10 {
            "difficulty-easy"
        } else if pokemon_level >= map.min_level && pokemon_level <= map.max_level {
            "difficulty-medium"
        } else {
            "difficulty-normal"
        }
    };

    let selected_map_clone = selected_map_for_modal.read().clone();
    let modal_title = selected_map_clone
        .as_ref()
        .map(|m| format!("{} - 详细信息", m.name))
        .unwrap_or_default();
    let modal_is_open = selected_map_clone.is_some();

    let (pokemon_list_empty, pokemon_list_for_recommendation, current_battle_scene, can_continue_battle, last_map_id) = {
        let ps = POKEMON_STATE.read();
        let bs = BATTLE_STATE.read();
        let scene = bs.scene.as_ref();
        let is_victory = scene.map(|s| s.status == _utils::types::api_battle::BattleStatus::Victory).unwrap_or(false);
        let my_pokemon_alive = scene.map(|s| s.my_pokemon.hp > 0).unwrap_or(false);
        let map_id = scene.map(|s| s.map_id).unwrap_or(0);
        (ps.list.is_empty(), ps.list.clone(), bs.scene.clone(), is_victory && my_pokemon_alive, map_id)
    };

    let mut continue_battle = move |_| {
        if last_map_id > 0 {
            clear_battle_scene();
            message_log.set(Vec::new());

            let api = NewApiClient::new();
            let map_id = last_map_id;

            let injured: Vec<(u64, String)> = POKEMON_STATE
                .read()
                .get_injured_pokemons()
                .into_iter()
                .map(|p| (p.id, p.name))
                .collect();

            spawn(async move {
                loading.set(true);

                for (pid, _pname) in &injured {
                    let _ = api.heal_pokemon(*pid).await;
                }
                if !injured.is_empty() {
                    refresh_pokemon_list();
                    refresh_user_profile_state();
                }

                match api.start_battle(map_id, None).await {
                    Ok(scene) => {
                        let map_name = scene.map_name.clone();
                        let pokemon_name = scene.wild_pokemon.name.clone();

                        set_battle_scene(Some(scene.clone()));
                        message_log.set(vec![format!(
                            "在地图 {} 遇到了野生的 {}！",
                            map_name, pokemon_name
                        )]);

                        let api2 = NewApiClient::new();
                        if let Ok(data) = api2.get_battle_items().await {
                            battle_items.set(data.items);
                        }
                        if let Ok(data) = api2.get_user_inventory(Some(2), 1).await {
                            battle_balls.set(data.items);
                        }
                    }
                    Err(e) => {
                        show_error(format!("继续冒险失败：{}", e));
                    }
                }
                loading.set(false);
            });
        }
    };

    rsx! {
        div { class: "page-adventure",
            Modal {
                is_open: modal_is_open,
                on_close: move |_| selected_map_for_modal.set(None),
                title: modal_title,
                if let Some(map) = selected_map_clone {
                    div { class: "modal-map-details",
                        div { class: "map-detail-header",
                            span { class: "map-icon", "{map.get_area_icon()}" }
                            h2 { "{map.name}" }
                            span { class: "area-type-badge badge-area-{map.get_area_color()}",
                                "{map.area_type_name}"
                            }
                        }

                        if map.mode.has_wild_pokemon() {
                            div { class: "map-detail-section",
                                h4 { "挑战建议" }
                                p { class: "recommendation-text",
                                    "{get_recommendation(&map, &pokemon_list_for_recommendation)}"
                                }
                            }
                        }

                        div { class: "map-detail-section",
                            h4 { "基本信息" }
                            div { class: "detail-grid",
                                if map.mode.has_wild_pokemon() {
                                    div { class: "detail-item",
                                        span { class: "detail-label", "等级范围" }
                                        span { class: "detail-value", "{map.get_level_range_text()}" }
                                    }
                                }
                                div { class: "detail-item",
                                    span { class: "detail-label", "地形类型" }
                                    span { class: "detail-value", "{map.area_type_name}" }
                                }
                                div { class: "detail-item",
                                    span { class: "detail-label", "地图模式" }
                                    span { class: "detail-value",
                                        {
                                            match &map.mode {
                                                _utils::types::api_map::MapMode::Wild => "野生模式",
                                                _utils::types::api_map::MapMode::Boss { .. } => "Boss 挑战",
                                                _utils::types::api_map::MapMode::Hybrid { .. } => "混合模式",
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if !map.wild_pokemons.is_empty() && map.mode.has_wild_pokemon() {
                            div { class: "map-detail-section",
                                h4 { "可能出现" }
                                div { class: "pokemon-list",
                                    for p in map.wild_pokemons.iter() {
                                        span { class: "pokemon-tag", "{p.name}" }
                                    }
                                }
                            }
                        }

                        {
                            let bosses = map.mode.get_bosses();
                            if !bosses.is_empty() {
                                let map_id_for_boss = map.id;
                                let bosses_vec: Vec<_> = bosses.into_iter().collect();
                                let mid = map_id_for_boss;
                                rsx! {
                                    div { class: "map-detail-section boss-challenge-section",
                                        h4 { "👑 Boss 挑战" }
                                        div { class: "boss-list-simple",
                                            Fragment {
                                                for boss in bosses_vec {
                                                    button {
                                                        key: "boss-{boss.pokemon_type_id}",
                                                        class: "boss-row-btn",
                                                         disabled: *loading.read() || pokemon_list_empty,
                                                         onclick: move |_| start_boss_adventure(mid, boss.pokemon_type_id),
                                                         div { class: "boss-row-content",
                                                             img {
                                                                 src: "{IMG_PATH_REMOTE}/pm/{boss.pokemon_type_id}.gif",
                                                                 class: "boss-row-avatar",
                                                                 alt: "{boss.pokemon_name}",
                                                             }
                                                             div { class: "boss-row-info",
                                                                 span { class: "boss-row-name", "{boss.pokemon_name}" }
                                                                 span { class: "boss-row-details",
                                                                     "Lv.{boss.level} · 倍率 x{boss.boss_multiplier}"
                                                                 }
                                                             }
                                                             span { class: "boss-row-action", "⚔️ 挑战" }
                                                         }
                                                     }
                                                 }
                                             }
                                        }
                                    }
                                }
                            } else {
                                rsx! {}
                            }
                        }

                        {
                            let map_id_for_adventure = map.id;
                            let has_wild = map.mode.has_wild_pokemon();
                            rsx! {
                                if has_wild {
                                    button {
                                        class: "modal-action-btn",
                                        onclick: move |_| start_adventure(map_id_for_adventure),
                                    disabled: *loading.read() || pokemon_list_empty,
                                        "开始冒险"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(current_battle) = &current_battle_scene {
                BattlePage {
                    battle: current_battle.clone(),
                    loading: *loading.read(),
                    on_use_skill: move |skill_id: u64| use_skill(skill_id),
                    on_flee: move |_| flee_battle(()),
                    on_end: move |_| end_battle_with_refresh(()),
                    on_use_item: move |item_id: u64| use_item_in_battle(item_id),
                    on_attack: attack,
                    on_capture: move |ball_id: u64| capture(ball_id),
                    items: battle_items.read().clone(),
                    balls: battle_balls.read().clone(),
                    skill_selection_mode: skill_selection_mode.read().clone(),
                    on_select_skill: move |skill_id: u64| {
                        if let Some((item_id, _, _)) = *skill_selection_mode.read() {
                            use_item_on_skill(item_id, skill_id);
                        }
                    },
                    on_cancel_skill_selection: move |_| {
                        skill_selection_mode.set(None);
                    },
                    on_enter_items_tab: move |_| refresh_battle_items(()),
                    on_enter_capture_tab: move |_| refresh_battle_balls(()),
                    on_switch_pokemon: switch_pokemon,
                    on_replace_pokemon: move |pokemon_id: u64| replace_pokemon(pokemon_id),
                    can_continue: can_continue_battle,
                    on_continue: move |_| continue_battle(()),
                }
            } else {
                div { class: "adventure-map-wrapper",
                    if *maps_loading.read() {
                        div { class: "loading", "加载地图数据..." }
                    } else {
                        if let Some(selected_region_id) = &*selected_region.read() {
                            {
                                let regions_data = regions.read();
                                let selected_region_data = regions_data
                                    .iter()
                                    .find(|r| &r.id == selected_region_id);
                                if let Some(region) = selected_region_data {
                                    let region_name = region.name.clone();
                                    let region_maps: Vec<MapInfo> = maps
                                        .read()
                                        .iter()
                                        .filter(|m| m.region == *selected_region_id)
                                        .cloned()
                                        .collect();
                                    let x = region.maps.first().map(|m| m.x).unwrap_or(50.0);
                                    let y = region.maps.first().map(|m| m.y).unwrap_or(50.0);
                                    rsx! {
                                        div { class: "hoenn-map-zoomed",
                                            button {
                                                class: "back-to-world-btn",
                                                onclick: move |_| selected_region.set(None),
                                                "← 返回世界地图"
                                            }

                                            div { class: "zoomed-region-title",
                                                h2 { "{region_name}" }
                                                p { "共 {region_maps.len()} 个冒险地点" }
                                            }

                                            div {
                                                class: "zoomed-map-bg",
                                                style: format!("background-position: {}% {}%;", x, y),
                                            }

                                            div { class: "zoomed-map-list",
                                                for map in region_maps.iter() {
                                                    {
                                                        let map_id = map.id;
                                                        let map_clone = map.clone();
                                                        let map_clone2 = map.clone();
                                                        let recommendation = get_recommendation(&map_clone, &pokemon_list_for_recommendation);
                                                        let difficulty_class = get_difficulty_class(
                                                            &map_clone,
                                                            &pokemon_list_for_recommendation,
                                                        );
                                                        rsx! {
                                                            div {
                                                                key: "{map_id}",
                                                                class: "zoomed-map-card {difficulty_class}",
                                                                onclick: move |_| selected_map_for_modal.set(Some(map_clone.clone())),
                                                                div { class: "zoomed-map-header",
                                                                    span { class: "map-icon", "{map_clone2.get_area_icon()}" }
                                                                    h4 { "{map_clone2.name}" }
                                                                    span { class: "area-type-badge badge-area-{map_clone2.get_area_color()}",
                                                                        "{map_clone2.area_type_name}"
                                                                    }
                                                                }
                                                                p { class: "map-desc", "{recommendation}" }
                                                                div { class: "zoomed-map-info",
                                                                    div { class: "info-row",
                                                                        if map_clone2.mode.has_wild_pokemon() {
                                                                            span { "等级：{map_clone2.get_level_range_text()}" }
                                                                        } else {
                                                                            {
                                                                                let boss_count = map_clone2.mode.get_bosses().len();
                                                                                rsx! {
                                                                                    span { "Boss 数量：{boss_count}" }
                                                                                }
                                                                            }
                                                                            span { "Boss 挑战" }
                                                                        }
                                                                    }
                                                                }

                                                                {
                                                                    let bosses = map_clone2.mode.get_bosses();
                                                                    if !bosses.is_empty() {
                                                                        let map_id_for_boss_card = map_clone2.id;
                                                                        let total_boss_count = bosses.len();
                                                                        let bosses_vec: Vec<_> = bosses.into_iter().take(3).collect();
                                                                        let mid = map_id_for_boss_card;
                                                                        rsx! {
                                                                            div { class: "card-boss-list",
                                                                                div { class: "card-boss-title", "👑 Boss:" }
                                                                                Fragment {
                                                                                    for boss in bosses_vec {
                                                                                        div {
                                                                                            key: "boss-{boss.pokemon_type_id}",
                                                                                            class: "card-boss-item",
                                                                                            onclick: move |e| {
                                                                                                e.stop_propagation();
                                                                                                start_boss_adventure(mid, boss.pokemon_type_id);
                                                                                            },
                                                                                            img {
                                                                                                src: "{IMG_PATH_REMOTE}/pm/{boss.pokemon_type_id}.gif",
                                                                                                class: "card-boss-avatar",
                                                                                                alt: "{boss.pokemon_name}",
                                                                                            }
                                                                                            span { class: "card-boss-name", "{boss.pokemon_name}" }
                                                                                            span { class: "card-boss-level", "Lv.{boss.level}" }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                if total_boss_count > 3 {
                                                                                    div { class: "card-boss-more", "...等 {total_boss_count} 个 Boss" }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        rsx! {}
                                                                    }
                                                                }

                                                                button {
                                                                    class: "zoomed-start-btn",
                                                                    onclick: move |e| {
                                                                        e.stop_propagation();
                                                                        selected_map_for_modal.set(Some(map_clone2.clone()));
                                                                    },
                                        disabled: *loading.read() || pokemon_list_empty,
                                                                    "查看详情"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            if pokemon_list_empty {
                                                div { class: "warning-box-zoomed",
                                                    "⚠️ 你还没有宝可梦！请先去商店购买或捕捉一只宝可梦。"
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div { "区域未找到" }
                                    }
                                }
                            }
                        } else {
                            div { class: "hoenn-map-world",
                                div { class: "map-decoration",
                                    div { class: "map-land-mass" }
                                    div { class: "map-water-area" }
                                }
                                for region in regions.read().iter() {
                                    {
                                        let region_id = region.id.clone();
                                        let map_count = region.maps.len();
                                        let x = region.maps.first().map(|m| m.x).unwrap_or(50.0);
                                        let y = region.maps.first().map(|m| m.y).unwrap_or(50.0);
                                        rsx! {
                                            div {
                                                key: "{region.id}",
                                                class: "map-marker",
                                                style: format!("left: {}%; top: {}%;", x, y),
                                                onclick: move |_| {
                                                    selected_region.set(Some(region_id.clone()));
                                                },
                                                div { class: "marker-dot" }
                                                div { class: "marker-info",
                                                    div { class: "marker-label", "{region.name}" }
                                                    div { class: "marker-count", "{map_count}个地点" }
                                                }
                                            }
                                        }
                                    }
                                }
                                if pokemon_list_empty {
                                    div { class: "warning-box-map",
                                        "⚠️ 你还没有宝可梦！请先去商店购买或捕捉一只宝可梦。"
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
