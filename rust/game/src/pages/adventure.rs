use crate::prelude::*;

use crate::{
    components::common::Modal,
    components::layout::IMG_PATH_REMOTE,
    pages::BattlePage,
    state::{
        clear_battle_scene, refresh_inventory_state, refresh_pokemon_list,
        refresh_user_profile_state, set_battle_scene, show_error, show_warning, use_pokemon_state,
        BATTLE_STATE, POKEMON_STATE,
    },
    utils::api_client::{battle_timeout, NewApiClient},
};
use _utils::functions::{quality_grade, quality_rank};
use _utils::types::{
    api_battle::{
        BattleItem, BattleMutationResponse, PendingBattleAction, PpRestoreSkill, StartBattleRequest,
    },
    api_map::MapInfo,
    api_map_region::MapRegion,
    api_pokemon::PokemonBasic,
    api_user::InventoryItem,
};

#[component]
pub fn Adventure() -> Element {
    use_pokemon_state();
    let mut loading = use_signal(|| true);
    let pending = use_signal(crate::state::load_pending_battle);
    let connection_error = use_signal(|| {
        if pending.read().is_some() {
            Some("上次操作的结果尚未确认，请重试确认结果。".to_string())
        } else {
            None
        }
    });
    let mut selected_region = use_signal(|| None::<String>);
    let mut selected_map_for_modal = use_signal(|| None::<MapInfo>);
    let min_level_filter = use_signal(|| None::<u32>);
    let max_level_filter = use_signal(|| None::<u32>);

    let mut maps: Signal<Vec<MapInfo>> = use_signal(Vec::new);
    let mut maps_loading = use_signal(|| true);

    let mut battle_items = use_signal(Vec::<BattleItem>::new);
    let mut battle_balls = use_signal(Vec::<InventoryItem>::new);

    let mut skill_selection_mode = use_signal(|| None::<(u64, String, Vec<PpRestoreSkill>)>);

    let actions = BattleActions {
        loading,
        pending,
        error: connection_error,
        items: battle_items,
        balls: battle_balls,
        selection: skill_selection_mode,
    };

    let _resource = use_resource(move || async move {
        let api = NewApiClient::new();

        match api.recover_current_battle().await {
            Ok(Some(scene)) => {
                set_battle_scene(Some(scene.clone()));

                actions.refresh_inventory().await;
            }
            Ok(None) => clear_battle_scene(),
            Err(error) => {
                let mut signal = connection_error;
                signal.set(Some(format!("战斗同步失败：{error}")));
            }
        }
        loading.set(false);
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

    let mut start_boss_adventure =
        move |map_id: u64, boss_type_id: u64, boss_index: Option<u64>| {
            selected_map_for_modal.set(None);
            actions.begin(
                "start",
                serde_json::json!(StartBattleRequest {
                    map_id,
                    boss_pokemon_type_id: Some(boss_type_id),
                    boss_index,
                }),
            );
        };
    let regions = use_memo(move || MapRegion::from_maps(&maps.read()));
    let mut start_adventure = move |map_id: u64| {
        selected_map_for_modal.set(None);
        actions.begin("start", serde_json::json!({"map_id":map_id}));
    };
    let use_skill =
        move |skill_id: u64| actions.begin("turn", serde_json::json!({"skill_id":skill_id}));
    let use_item_on_skill = move |item_id: u64, skill_record_id: u64| {
        actions.begin(
            "use_item_on_skill",
            serde_json::json!({"item_id":item_id,"skill_record_id":skill_record_id}),
        );
    };
    let use_item_in_battle =
        move |item_id: u64| actions.begin("use_item", serde_json::json!({"item_id":item_id}));
    let attack = move || actions.begin("turn", serde_json::json!({"skill_id":0}));
    let capture =
        move |ball_id: u64| actions.begin("capture", serde_json::json!({"ball_id":ball_id}));
    let flee_battle = move |_| actions.begin("flee", serde_json::json!({}));

    let refresh_battle_items = move |_| {
        let api = NewApiClient::new();
        spawn(async move {
            match api.get_battle_items().await {
                Ok(data) => battle_items.set(data.items),
                Err(error) => {
                    show_error(format!("道具载入失败：{error}。请重新点击道具页签重试。"))
                }
            }
        });
    };

    let refresh_battle_balls = move |_| {
        let api = NewApiClient::new();
        spawn(async move {
            match api.get_battle_balls().await {
                Ok(data) => battle_balls.set(data.items),
                Err(error) => {
                    show_error(format!("精灵球载入失败：{error}。请重新点击捕捉页签重试。"))
                }
            }
        });
    };

    let end_battle_with_refresh = move |_| {
        clear_battle_scene();
        refresh_pokemon_list();
        refresh_user_profile_state();
        refresh_inventory_state();
    };

    let switch_pokemon = move || actions.begin("switch_pokemon", serde_json::json!({}));
    let replace_pokemon = move |pokemon_id: u64| {
        actions.begin(
            "replace_pokemon",
            serde_json::json!({"pokemon_id":pokemon_id}),
        )
    };

    // 活动地图品质门槛：出战宠物的品质档位是否低于地图要求。
    // 返回 None 表示无门槛或已达要求；旧端点未返回个体值时不做客户端预判（服务端仍会拦截）。
    let quality_gate_notice = |map: &MapInfo, pokemons: &[PokemonBasic]| -> Option<String> {
        if map.min_quality == 0 {
            return None;
        }
        let rank = pokemons
            .first()
            .and_then(|p| p.iv_total)
            .map(quality_rank)?;
        if (rank as u32) < (map.min_quality as u32) {
            Some(format!(
                "需要出战宠物品质达到 {}（当前 {}）",
                map.min_quality_name,
                quality_grade(rank)
            ))
        } else {
            None
        }
    };

    let get_recommendation = |map: &MapInfo, pokemons: &[PokemonBasic]| -> String {
        if pokemons.is_empty() {
            return "请先获得一只宝可梦".to_string();
        }

        if let Some(notice) = quality_gate_notice(map, pokemons) {
            return notice;
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

    let (
        pokemon_list_empty,
        pokemon_list_for_recommendation,
        current_battle_scene,
        can_continue_battle,
    ) = {
        let ps = POKEMON_STATE.read();
        let bs = BATTLE_STATE.read();
        let scene = bs.scene.as_ref();
        let is_victory = scene
            .map(|s| s.status == _utils::types::api_battle::BattleStatus::Victory)
            .unwrap_or(false);
        let my_pokemon_alive = scene.map(|s| s.my_pokemon.hp > 0).unwrap_or(false);
        (
            ps.list.is_empty(),
            ps.list.clone(),
            bs.scene.clone(),
            is_victory && my_pokemon_alive,
        )
    };

    let mut continue_battle = move |_| {
        let map_id = crate::state::get_last_map_id();
        if map_id == 0
            || *loading.read()
            || pending.read().is_some()
            || connection_error.read().is_some()
        {
            return;
        }
        let injured = POKEMON_STATE.read().get_injured_pokemons();
        loading.set(true);
        spawn(async move {
            let api = NewApiClient::new();
            for pokemon in injured {
                if let Err(error) = battle_timeout(api.heal_pokemon(pokemon.id))
                    .await
                    .and_then(|result| result)
                {
                    refresh_pokemon_list();
                    refresh_user_profile_state();
                    loading.set(false);
                    show_error(format!(
                        "治疗结果尚未确认：{error}。请检查宠物状态后再继续。"
                    ));
                    return;
                }
            }
            refresh_pokemon_list();
            refresh_user_profile_state();
            loading.set(false);
            actions.begin("start", serde_json::json!({"map_id":map_id}));
        });
    };
    let actions_blocked =
        *loading.read() || pending.read().is_some() || connection_error.read().is_some();

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

                        if map.min_quality > 0 {
                            div { class: "map-detail-section quality-gate-section",
                                h4 { "品质门槛" }
                                p { class: "quality-gate-text",
                                    span { class: "quality-badge quality-rank-{map.min_quality}", "{map.min_quality_name}" }
                                    " 出战宠物品质需达到 {map.min_quality_name}（由六项个体值总和判定）"
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
                                let bosses_vec: Vec<_> = bosses.into_iter().enumerate().collect();
                                let mid = map_id_for_boss;
                                let quality_blocked = quality_gate_notice(
                                    &map,
                                    &pokemon_list_for_recommendation,
                                )
                                .is_some();
                                rsx! {
                                    div { class: "map-detail-section boss-challenge-section",
                                        h4 { "👑 Boss 挑战" }
                                        div { class: "boss-list-simple",
                                            Fragment {
                                                for (position, boss) in bosses_vec {
                                                    button {
                                                        key: "boss-{mid}-{position}",
                                                        class: "boss-row-btn",
                                                         disabled: actions_blocked || pokemon_list_empty || quality_blocked,
                                                         onclick: move |_| start_boss_adventure(mid, boss.pokemon_type_id, boss.boss_index),
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
                            let quality_blocked = quality_gate_notice(
                                &map,
                                &pokemon_list_for_recommendation,
                            )
                            .is_some();
                            rsx! {
                                if has_wild {
                                    button {
                                        class: "modal-action-btn",
                                        onclick: move |_| start_adventure(map_id_for_adventure),
                                    disabled: actions_blocked || pokemon_list_empty || quality_blocked,
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
                    loading: actions_blocked,
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
                                                                        {
                                                                            if map_clone2.min_quality > 0 {
                                                                                rsx! {
                                                                                    span { class: "map-quality-requirement",
                                                                                        "品质：{map_clone2.min_quality_name}+"
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                rsx! {}
                                                                            }
                                                                        }
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
                                                                        let bosses_vec: Vec<_> = bosses.into_iter().enumerate().take(3).collect();
                                                                        let mid = map_id_for_boss_card;
                                                                        rsx! {
                                                                            div { class: "card-boss-list",
                                                                                div { class: "card-boss-title", "👑 Boss:" }
                                                                                Fragment {
                                                                                    for (position, boss) in bosses_vec {
                                                                                        div {
                                                                                            key: "boss-{mid}-{position}",
                                                                                            class: "card-boss-item",
                                                                                            onclick: move |e| {
                                                                                                e.stop_propagation();
                                                                                                start_boss_adventure(mid, boss.pokemon_type_id, boss.boss_index);
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
                                        disabled: actions_blocked || pokemon_list_empty,
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
            if let Some(error) = connection_error.read().clone() {
                Modal { is_open: true, close_on_overlay: false,
                    title: "确认战斗状态".to_string(), on_close: move |_| {},
                    div { class: "battle-connection", "data-testid": "battle-connection",
                        p { role: "alert", "{error}" }
                        if pending.read().is_some() {
                            p { "将确认刚才那次操作，已执行的操作不会重复扣除道具或推进回合。" }
                            button { class: "btn btn-primary", "data-testid": "battle-action-retry",
                                disabled: *loading.read(), onclick: move |_| actions.retry(), "重试并确认结果" }
                        } else {
                            button { class: "btn btn-primary", "data-testid": "battle-reconnect",
                                disabled: *loading.read(), onclick: move |_| actions.recover(), "重新同步战斗" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct BattleActions {
    loading: Signal<bool>,
    pending: Signal<Option<PendingBattleAction>>,
    error: Signal<Option<String>>,
    items: Signal<Vec<BattleItem>>,
    balls: Signal<Vec<InventoryItem>>,
    selection: Signal<Option<(u64, String, Vec<PpRestoreSkill>)>>,
}

impl BattleActions {
    fn begin(self, action: &str, fields: serde_json::Value) {
        if *self.loading.read() || self.pending.read().is_some() || self.error.read().is_some() {
            return;
        }
        let scene = BATTLE_STATE.read().scene.clone();
        if action != "start" && scene.is_none() {
            return;
        }
        let action = PendingBattleAction::new(
            action,
            if action == "start" {
                None
            } else {
                scene.as_ref()
            },
            fields,
        );
        self.submit(action);
    }

    fn retry(self) {
        let pending = self.pending.read().clone();
        if let Some(action) = pending {
            self.submit(action);
        }
    }

    fn submit(mut self, action: PendingBattleAction) {
        if *self.loading.read() {
            return;
        }
        let is_retry = self.pending.read().is_some();
        self.loading.set(true);
        self.pending.set(Some(action.clone()));
        crate::state::save_pending_battle(Some(&action));
        spawn(async move {
            let api = NewApiClient::new();
            match api.perform_battle_action(&action).await {
                Ok(response) => {
                    self.pending.set(None);
                    crate::state::save_pending_battle(None);
                    self.error.set(None);
                    let verify_current_state = is_retry
                        && match &response {
                            BattleMutationResponse::Scene(scene) => !scene.battle_over,
                            BattleMutationResponse::SkillSelection(_) => true,
                        };
                    // A receipt confirms the original action, but another tab may have
                    // advanced or ended its battle while this tab was disconnected.
                    if verify_current_state {
                        match api.recover_current_battle().await {
                            Ok(Some(current)) => set_battle_scene(Some(current)),
                            Ok(None) => {
                                clear_battle_scene();
                                self.selection.set(None);
                                refresh_pokemon_list();
                                refresh_user_profile_state();
                                refresh_inventory_state();
                                self.loading.set(false);
                                show_warning("刚才的操作已确认，该场战斗已结束。");
                                return;
                            }
                            Err(error) => {
                                self.selection.set(None);
                                self.error
                                    .set(Some(format!("刚才的操作已确认，战斗同步失败：{error}")));
                                self.loading.set(false);
                                return;
                            }
                        }
                    }
                    match response {
                        BattleMutationResponse::Scene(scene) => {
                            self.selection.set(None);
                            let stale = BATTLE_STATE.read().scene.as_ref().is_some_and(|current| {
                                current.engine_battle_id > 0
                                    && scene.engine_battle_id > 0
                                    && (current.engine_battle_id > scene.engine_battle_id
                                        || (current.engine_battle_id == scene.engine_battle_id
                                            && current.revision > scene.revision)
                                        || (current.engine_battle_id != scene.engine_battle_id
                                            && !current.battle_over
                                            && action.action != "start"))
                            });
                            if stale {
                                self.error.set(Some(
                                    "刚才的操作已确认，但战斗已有更新，请重新同步。".to_string(),
                                ));
                            } else if !verify_current_state {
                                set_battle_scene(Some(*scene));
                            }
                            refresh_pokemon_list();
                            refresh_user_profile_state();
                            refresh_inventory_state();
                            self.refresh_inventory().await;
                        }
                        BattleMutationResponse::SkillSelection(selection) => {
                            let stale = selection.engine_battle_id > 0
                                && BATTLE_STATE.read().scene.as_ref().is_some_and(|current| {
                                    current.engine_battle_id > 0
                                        && (current.engine_battle_id != selection.engine_battle_id
                                            || current.revision != selection.revision)
                                });
                            if stale {
                                self.selection.set(None);
                                self.error
                                    .set(Some("战斗已更新，请重新同步后选择道具。".to_string()));
                            } else {
                                self.selection.set(Some((
                                    selection.item_id,
                                    selection.item_name,
                                    selection.available_skills,
                                )));
                            }
                        }
                    }
                }
                Err(error) => {
                    if !error.uncertain {
                        self.pending.set(None);
                        crate::state::save_pending_battle(None);
                    }
                    self.error.set(Some(error.message));
                }
            }
            self.loading.set(false);
        });
    }

    fn recover(mut self) {
        if *self.loading.read() || self.pending.read().is_some() {
            return;
        }
        self.loading.set(true);
        spawn(async move {
            let api = NewApiClient::new();
            match api.recover_current_battle().await {
                Ok(scene) => {
                    set_battle_scene(scene);
                    self.selection.set(None);
                    self.error.set(None);
                    refresh_pokemon_list();
                    refresh_user_profile_state();
                    self.refresh_inventory().await;
                }
                Err(error) => self.error.set(Some(format!("战斗同步失败：{error}"))),
            }
            self.loading.set(false);
        });
    }

    async fn refresh_inventory(mut self) {
        let api = NewApiClient::new();
        match api.get_battle_items().await {
            Ok(data) => self.items.set(data.items),
            Err(error) => show_error(format!("道具载入失败：{error}。请重新点击道具页签重试。")),
        }
        match api.get_battle_balls().await {
            Ok(data) => self.balls.set(data.items),
            Err(error) => show_error(format!("精灵球载入失败：{error}。请重新点击捕捉页签重试。")),
        }
    }
}
