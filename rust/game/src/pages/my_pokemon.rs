use crate::prelude::*;

use crate::{
    components::{
        common::{use_popup, Modal, ModalLg, PopupMenu, PopupMenuItem, TypeBadge},
        layout::{IMG_PATH, IMG_PATH_REMOTE},
    },
    state::{
        refresh_pokemon_list, show_error, show_success, start_global_loading, stop_global_loading,
        update_pokemon_hp, use_battle_state, use_pokemon_state, MyPokemonTab, SelectedItem,
        EQUIPMENT_BONUSES, MY_POKEMON_TAB, POKEMON_STATE, SELECTED_ITEM, SELECTED_POKEMON_INDEX,
    },
    utils::{
        api_client::NewApiClient,
        pokemon::{gender_class, gender_symbol, hp_class, hp_percent},
    },
};
use _utils::types::api_pokemon::{EquipmentItem, EquipmentSlot, LearnableSkill, PokemonSkillSlot};

#[component]
fn BonusTagsLeft() -> Element {
    let bonuses = EQUIPMENT_BONUSES.read();
    let left_bonuses: Vec<_> = bonuses
        .iter()
        .filter(|(name, _)| matches!(name.as_str(), "HP" | "攻击" | "防御"))
        .collect();

    if left_bonuses.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "side-tags left",
            for (stat_name , value) in left_bonuses {
                if *value != 0 {
                    {
                        let class_name = match stat_name.as_str() {
                            "HP" => "bonus-tag-small hp",
                            "攻击" => "bonus-tag-small atk",
                            "防御" => "bonus-tag-small def",
                            _ => "bonus-tag-small",
                        };
                        let display_text = if *value > 0 {
                            format!("+{} {}", value, stat_name)
                        } else {
                            format!("{} {}", value, stat_name)
                        };
                        rsx! {
                            span { class: "{class_name}", "{display_text}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BonusTagsRight() -> Element {
    let bonuses = EQUIPMENT_BONUSES.read();
    let right_bonuses: Vec<_> = bonuses
        .iter()
        .filter(|(name, _)| matches!(name.as_str(), "特攻" | "特防" | "速度"))
        .collect();

    if right_bonuses.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "side-tags right",
            for (stat_name , value) in right_bonuses {
                if *value != 0 {
                    {
                        let class_name = match stat_name.as_str() {
                            "特攻" => "bonus-tag-small spat",
                            "特防" => "bonus-tag-small spdef",
                            "速度" => "bonus-tag-small spd",
                            _ => "bonus-tag-small",
                        };
                        let display_text = if *value > 0 {
                            format!("+{} {}", value, stat_name)
                        } else {
                            format!("{} {}", value, stat_name)
                        };
                        rsx! {
                            span { class: "{class_name}", "{display_text}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn MyPokemon() -> Element {
    use_pokemon_state();
    let refresh_trigger = use_signal(|| 0u64);
    let mut show_ev_modal = use_signal(|| false);
    let mut show_evo_path_modal = use_signal(|| false);
    let mut selected_pokemon_type_id = use_signal(|| 0u64);

    // 每次打开个人中心时都刷新宠物列表，确保数据是最新的
    let _resource = use_resource(move || async move {
        refresh_pokemon_list();
    });

    use_effect(move || {
        let trigger = *refresh_trigger.read();
        if trigger > 0 {
            refresh_pokemon_list();
        }
    });

    let (loading, error, current_pm) = {
        let state = POKEMON_STATE.read();
        let selected_id = *SELECTED_POKEMON_INDEX.read();
        let pm = selected_id
            .and_then(|id| state.list.iter().find(|p| p.id == id).cloned())
            .or_else(|| state.get_first_pokemon().cloned());
        (state.loading, state.error.clone(), pm)
    };

    rsx! {
        div { class: "page-my-pokemon",
            div { class: "mypm-card",
                div { class: "mypm-card-header",
                    img {
                        class: "header-icon-img",
                        src: "{IMG_PATH}/item/jlq.gif",
                        alt: "",
                    }
                    span { "我的宠物" }
                }

                div { class: "mypm-card-body",
                    if loading {
                        div { class: "loading", "正在加载宠物数据..." }
                    } else if let Some(err) = &error {
                        div { class: "error-message", "加载失败：{err}" }
                    } else if let Some(pm) = &current_pm {
                            div { class: "mypm-content-area",
                            div { class: "mypm-tab-menu",
                                button {
                                    class: if *MY_POKEMON_TAB.read() == MyPokemonTab::Stats { "mypm-tab active" } else { "mypm-tab" },
                                    onclick: move |_| *MY_POKEMON_TAB.write() = MyPokemonTab::Stats,
                                    if *MY_POKEMON_TAB.read() == MyPokemonTab::Stats {
                                        span { class: "tab-arrow left", ">" }
                                    }
                                    span { "属性" }
                                    if *MY_POKEMON_TAB.read() == MyPokemonTab::Stats {
                                        span { class: "tab-arrow right", "<" }
                                    }
                                }
                                button {
                                    class: if *MY_POKEMON_TAB.read() == MyPokemonTab::Equipment { "mypm-tab active" } else { "mypm-tab" },
                                    onclick: move |_| *MY_POKEMON_TAB.write() = MyPokemonTab::Equipment,
                                    if *MY_POKEMON_TAB.read() == MyPokemonTab::Equipment {
                                        span { class: "tab-arrow left", ">" }
                                    }
                                    span { "装备" }
                                    if *MY_POKEMON_TAB.read() == MyPokemonTab::Equipment {
                                        span { class: "tab-arrow right", "<" }
                                    }
                                }
                                button {
                                    class: if *MY_POKEMON_TAB.read() == MyPokemonTab::Skills { "mypm-tab active" } else { "mypm-tab" },
                                    onclick: move |_| *MY_POKEMON_TAB.write() = MyPokemonTab::Skills,
                                    if *MY_POKEMON_TAB.read() == MyPokemonTab::Skills {
                                        span { class: "tab-arrow left", ">" }
                                    }
                                    span { "技能" }
                                    if *MY_POKEMON_TAB.read() == MyPokemonTab::Skills {
                                        span { class: "tab-arrow right", "<" }
                                    }
                                }
                            }

                            div { class: "mypm-divider" }

                            div { class: "mypm-main-content",
                                div { class: "pokemon-sprite-area",
                                    div { class: "pokemon-display-with-tags",
                                        if *MY_POKEMON_TAB.read() == MyPokemonTab::Equipment {
                                            BonusTagsLeft {}
                                        }

                                        img {
                                            class: "pokemon-sprite-img",
                                            src: "{IMG_PATH_REMOTE}/pm/{pm.type_id}.gif",
                                            alt: "{pm.name}",
                                        }

                                        if *MY_POKEMON_TAB.read() == MyPokemonTab::Equipment {
                                            BonusTagsRight {}
                                        }
                                    }
                                }

                                div { class: "mypm-tab-content",
                                    match *MY_POKEMON_TAB.read() {
                                        MyPokemonTab::Stats => rsx! {
                                            StatsTabContent {
                                                pokemon_id: pm.id,
                                                pm_name: pm.name.clone(),
                                                pm_type_id: pm.type_id,
                                                pm_gender: pm.gender,
                                                pm_is_shiny: pm.is_shiny,
                                                pm_level: pm.level as u32,
                                                pm_hp: pm.hp,
                                                pm_max_hp: pm.max_hp,
                                                pm_exp: pm.exp,
                                                pm_exp_for_current_level: pm.exp_for_current_level,
                                                pm_exp_for_next_level: pm.exp_for_next_level,
                                                pm_type1: pm.base_info.type_1.clone(),
                                                pm_type2: pm.base_info.type_2.clone(),
                                                pm_state: pm.state,
                                                pm_state_text: pm.state_text.clone(),
                                                pm_state_class: pm.state_class.clone(),
                                                pm_affection: pm.affection,
                                                refresh_trigger,
                                                on_calculator_click: move |_| show_ev_modal.set(true),
                                                on_evolution_path_click: {
                                                    let type_id = pm.type_id;
                                                    move |_| {
                                                        selected_pokemon_type_id.set(type_id);
                                                        show_evo_path_modal.set(true);
                                                    }
                                                },
                                            }
                                        },
                                        MyPokemonTab::Equipment => rsx! {
                                            EquipmentTabContent { pokemon_id: pm.id, refresh_trigger }
                                        },
                                        MyPokemonTab::Skills => rsx! {
                                            SkillsTabContent { pokemon_id: pm.id, pokemon_level: pm.level as u32, refresh_trigger }
                                        },
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "empty-hint", "你还没有宠物，快去冒险捕捉吧！" }
                    }
                }
            }

            // 努力值计算器 Modal
            if *show_ev_modal.read() {
                EffortValueCalculatorModal { on_close: move |_| show_ev_modal.set(false) }
            }

            // 进化路径 Modal
            if *show_evo_path_modal.read() {
                EvolutionPathModal {
                    pokemon_type_id: *selected_pokemon_type_id.read(),
                    on_close: move |_| show_evo_path_modal.set(false),
                }
            }
        }
    }
}

#[component]
fn StatsTabContent(
    pokemon_id: u64,
    pm_name: String,
    pm_type_id: u64,
    pm_gender: u8,
    pm_is_shiny: bool,
    pm_level: u32,
    pm_hp: i64,
    pm_max_hp: i64,
    pm_exp: u64,
    pm_exp_for_current_level: u64,
    pm_exp_for_next_level: u64,
    pm_type1: String,
    pm_type2: Option<String>,
    pm_state: u8,
    pm_state_text: String,
    pm_state_class: String,
    pm_affection: u32,
    refresh_trigger: Signal<u64>,
    on_calculator_click: EventHandler<()>,
    on_evolution_path_click: EventHandler<()>,
) -> Element {
    let _is_shiny = pm_is_shiny;
    let detail = use_resource(move || async move {
        let api = NewApiClient::new();
        api.get_pokemon_detail(pokemon_id).await
    });

    let detail_data = detail
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();

    let (original_name, current_nickname) = detail_data
        .as_ref()
        .map(|d| {
            let original = d.name.clone();
            let nickname = d.nickname.clone();
            (original, nickname)
        })
        .unwrap_or((pm_name.clone(), None));

    let display_name = current_nickname
        .clone()
        .unwrap_or_else(|| original_name.clone());

    let (atk, def, spa, spd, spe) = detail_data
        .as_ref()
        .map(|d| {
            (
                d.stats.attack,
                d.stats.defense,
                d.stats.sp_attack,
                d.stats.sp_defense,
                d.stats.speed,
            )
        })
        .unwrap_or((0, 0, 0, 0, 0));

    let exp_in_current_level = pm_exp.saturating_sub(pm_exp_for_current_level);
    let exp_range = pm_exp_for_next_level.saturating_sub(pm_exp_for_current_level);
    let exp_percent = if exp_range > 0 {
        (exp_in_current_level as f64 / exp_range as f64 * 100.0).min(100.0) as u32
    } else {
        100
    };

    let exp_needed = pm_exp_for_next_level.saturating_sub(pm_exp_for_current_level);
    let exp_current_in_level = pm_exp.saturating_sub(pm_exp_for_current_level);

    let mut is_editing = use_signal(|| false);
    let mut edit_value = use_signal(|| display_name.clone());
    let mut is_saving = use_signal(|| false);
    let mut save_error = use_signal(|| Option::<String>::None);
    let mut save_success = use_signal(|| false);

    let handle_key_down = move |e: KeyboardEvent| {
        if e.key() == Key::Enter {
            let new_name = edit_value.read().clone();
            if new_name.is_empty() || new_name.trim().is_empty() {
                save_error.set(Some("昵称不能为空".to_string()));
                return;
            }

            is_saving.set(true);
            save_error.set(None);
            save_success.set(false);

            let pokemon_id = pokemon_id;
            let mut is_editing = is_editing;
            let mut is_saving = is_saving;
            let mut save_error = save_error;
            let mut save_success = save_success;
            let mut refresh_trigger = refresh_trigger;

            spawn(async move {
                let api = NewApiClient::new();
                match api.rename_pokemon(pokemon_id, &new_name).await {
                    Ok(_) => {
                        save_success.set(true);
                        is_editing.set(false);
                        refresh_trigger += 1;
                    }
                    Err(e) => {
                        save_error.set(Some(format!("保存失败：{}", e)));
                        is_saving.set(false);
                    }
                }
            });
        } else if e.key() == Key::Escape {
            is_editing.set(false);
            save_error.set(None);
            save_success.set(false);
        }
    };

    rsx! {
        div { class: "stats-tab-layout",
            div { class: "stats-header", style: "margin-bottom: 8px;",
                div { class: "info-tags",
                    span { class: "info-tag", "No {pm_type_id}" }
                    span { class: "info-tag", "ID {pokemon_id}" }
                }

                div { class: "tags-group",
                    div { class: "type-badges",
                        span { class: "badge badge-type-{pm_type1}", "{pm_type1}" }
                        if let Some(ref t2) = pm_type2 {
                            span { class: "badge badge-type-{t2}", "{t2}" }
                        }
                    }

                    if pm_state != 1 {
                        StateTagWithTooltip {
                            state: pm_state,
                            state_text: pm_state_text.clone(),
                            state_class: pm_state_class.clone(),
                        }
                    }
                }
            }

            div { class: "pokemon-name-row",
                if *is_editing.read() {
                    div { class: "pokemon-name-edit-wrapper",
                        input {
                            class: "pokemon-name-input",
                            value: "{edit_value}",
                            oninput: move |e| {
                                let value = e.value();
                                if value.chars().count() <= 14 {
                                    edit_value.set(value);
                                }
                            },
                            onkeydown: handle_key_down,
                            placeholder: "输入昵称（最多 14 字）",
                            disabled: *is_saving.read(),
                            maxlength: 14,
                        }
                        div { class: "edit-actions-inline",
                            button {
                                class: "edit-action-btn save",
                                onclick: move |_event| {
                                    let new_name = edit_value.read().clone();
                                    if new_name.is_empty() || new_name.trim().is_empty() {
                                        save_error.set(Some("昵称不能为空".to_string()));
                                        return;
                                    }

                                    is_saving.set(true);
                                    save_error.set(None);
                                    save_success.set(false);

                                    let pokemon_id = pokemon_id;
                                    let mut is_editing = is_editing;
                                    let mut is_saving = is_saving;
                                    let mut save_error = save_error;
                                    let mut save_success = save_success;
                                    let mut refresh_trigger = refresh_trigger;

                                    spawn(async move {
                                        let api = NewApiClient::new();
                                        match api.rename_pokemon(pokemon_id, &new_name).await {
                                            Ok(_) => {
                                                save_success.set(true);
                                                is_editing.set(false);
                                                refresh_trigger += 1;
                                            }
                                            Err(e) => {
                                                save_error.set(Some(format!("保存失败：{}", e)));
                                                is_saving.set(false);
                                            }
                                        }
                                    });
                                },
                                r#type: "button",
                                title: "保存",
                                "✓"
                            }
                            button {
                                class: "edit-action-btn cancel",
                                onclick: move |_| {
                                    is_editing.set(false);
                                    save_error.set(None);
                                    save_success.set(false);
                                },
                                r#type: "button",
                                title: "取消",
                                "✕"
                            }
                        }
                    }
                } else {
                    if let Some(ref nickname) = current_nickname {
                        if nickname != &original_name {
                            span { class: "pokemon-name-with-original",
                                span { class: "pokemon-name", "{nickname}" }
                                span { class: "original-name", " ({original_name})" }
                            }
                        } else {
                            span { class: "pokemon-name", "{nickname}" }
                        }
                    } else {
                        span { class: "pokemon-name", "{original_name}" }
                    }
                    button {
                        class: "edit-icon-btn",
                        onclick: move |_event| {
                            edit_value.set(display_name.clone());
                            is_editing.set(true);
                            save_error.set(None);
                            save_success.set(false);
                        },
                        r#type: "button",
                        "✏️"
                    }
                    span { class: "pokemon-gender {gender_class(pm_gender)}",
                        "{gender_symbol(pm_gender)}"
                    }
                }
            }

            div { class: "stats-bottom-row",
                div { class: "stats-bars",
                    div { class: "stat-bar-row",
                        span { class: "bar-label", "HP" }
                        div { class: "bar-track hp-bar",
                            div {
                                class: "bar-fill {hp_class(pm_hp, pm_max_hp)}",
                                style: "width: {hp_percent(pm_hp, pm_max_hp)}%",
                            }
                            span { class: "bar-text", "{pm_hp} / {pm_max_hp}" }
                        }
                    }

                    div { class: "stat-bar-row with-sub-info",
                        span { class: "bar-label", "EXP" }
                        div { class: "bar-track exp-bar",
                            div {
                                class: "bar-fill exp-fill",
                                style: "width: {exp_percent}%",
                            }
                            span { class: "bar-text", "{exp_current_in_level} / {exp_needed}" }
                        }
                    }
                    div { class: "affection-row",
                        span { class: "affection-label", "好感度" }
                        span { class: "affection-value", "{pm_affection}" }
                    }
                    div { class: "tool-buttons-col",
                        button {
                            class: "ev-calculator-btn",
                            onclick: move |_event| on_calculator_click(()),
                            "努力值计算器"
                        }
                        button {
                            class: "ev-calculator-btn",
                            onclick: move |_event| on_evolution_path_click(()),
                            "进化路径"
                        }
                    }
                }

                div { class: "stats-radar-chart",
                    RadarChart {
                        level: pm_level,
                        attack: atk as u32,
                        defense: def as u32,
                        sp_attack: spa as u32,
                        sp_defense: spd as u32,
                        speed: spe as u32,
                    }
                }
            }
        }
    }
}

#[component]
fn RadarChart(
    level: u32,
    attack: u32,
    defense: u32,
    sp_attack: u32,
    sp_defense: u32,
    speed: u32,
) -> Element {
    let cx = 140.0f64;
    let cy = 140.0f64;
    let max_r = 70.0f64;

    let angles: Vec<f64> = (0..5)
        .map(|i| (-90.0 + i as f64 * 72.0).to_radians())
        .collect();

    let labels = ["攻击", "防御", "特攻", "特防", "速度"];
    let values = [attack, defense, sp_attack, sp_defense, speed];
    let colors = ["#E74C3C", "#F5A623", "#9B59B6", "#1ABC9C", "#3498DB"];

    let min_r = max_r * 0.2;
    let grid_layers = [1.0, 0.6, 0.3];
    let grid_paths: Vec<String> = grid_layers
        .iter()
        .map(|&scale| {
            let r = min_r + scale * (max_r - min_r);
            let points: Vec<String> = angles
                .iter()
                .map(|&a| format!("{:.1},{:.1}", cx + r * a.cos(), cy + r * a.sin()))
                .collect();
            format!("M {} Z", points.join(" L "))
        })
        .collect();

    let max_stat = 300.0f64;
    let data_points: Vec<(f64, f64)> = values
        .iter()
        .zip(angles.iter())
        .map(|(&v, &a)| {
            let ratio = (v as f64 / max_stat).min(1.0);
            let r = min_r + ratio * (max_r - min_r);
            (cx + r * a.cos(), cy + r * a.sin())
        })
        .collect();

    let data_path = format!(
        "M {} Z",
        data_points
            .iter()
            .map(|(x, y)| format!("{:.1},{:.1}", x, y))
            .collect::<Vec<_>>()
            .join(" L ")
    );

    let label_positions: Vec<(f64, f64)> = angles
        .iter()
        .map(|&a| {
            let r = max_r + 15.0;
            (cx + r * a.cos(), cy + r * a.sin())
        })
        .collect();

    rsx! {
        svg {
            class: "radar-svg responsive",
            view_box: "0 0 280 280",
            width: "200px",
            height: "200px",

            for path in &grid_paths {
                path {
                    d: "{path}",
                    fill: "none",
                    stroke: "#e0e0e0",
                    stroke_width: "1",
                }
            }

            for & a in &angles {
                line {
                    x1: "{cx}",
                    y1: "{cy}",
                    x2: "{cx + max_r * a.cos()}",
                    y2: "{cy + max_r * a.sin()}",
                    stroke: "#e0e0e0",
                    stroke_width: "0.5",
                }
            }

            path {
                d: "{data_path}",
                fill: "rgba(52, 152, 219, 0.2)",
                stroke: "#3498db",
                stroke_width: "2",
            }

            for (x , y) in &data_points {
                circle {
                    cx: "{x}",
                    cy: "{y}",
                    r: "4",
                    fill: "#3498db",
                    stroke: "white",
                    stroke_width: "2",
                }
            }

            text {
                x: "{cx}",
                y: "{cy + 4.0}",
                text_anchor: "middle",
                font_size: "16",
                font_weight: "bold",
                fill: "#333",
                "Lv {level}"
            }

            for (i , (lx , ly)) in label_positions.iter().enumerate() {
                text {
                    x: "{lx}",
                    y: "{ly - 8.0}",
                    text_anchor: "middle",
                    font_size: "12",
                    fill: "{colors[i]}",
                    font_weight: "bold",
                    "{values[i]}"
                }
                text {
                    x: "{lx}",
                    y: "{ly + 4.0}",
                    text_anchor: "middle",
                    font_size: "12",
                    fill: "{colors[i]}",
                    "● {labels[i]}"
                }
            }
        }
    }
}

#[component]
fn EquipmentTabContent(pokemon_id: u64, refresh_trigger: Signal<u64>) -> Element {
    let mut action_loading = use_signal(|| false);

    let equipment_resource = use_resource(move || async move {
        let api = NewApiClient::new();
        let result = api.get_equipment(pokemon_id).await;
        result
    });

    let equipment_data = equipment_resource.read();

    let slots = equipment_data
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .map(|d| d.equipment_slots.clone())
        .unwrap_or_default();

    let owned_items = equipment_data
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .map(|d| d.owned_items.clone())
        .unwrap_or_default();

    let total_bonuses = calculate_total_bonuses(&slots);

    let bonuses_clone = total_bonuses.clone();
    let current = EQUIPMENT_BONUSES.peek();
    if *current != bonuses_clone {
        drop(current);
        *EQUIPMENT_BONUSES.write() = bonuses_clone;
    }

    let slots: Vec<EquipmentSlot> = if slots.len() >= 4 {
        slots
    } else {
        let mut result = slots.clone();
        for i in result.len()..4 {
            result.push(EquipmentSlot {
                slot_index: i as u32,
                equipment_id: 0,
                item: None,
            });
        }
        result
    };

    rsx! {
        div { class: "equipment-tab",
            // 战斗中的遮罩层
            if use_battle_state() {
                div { class: "battle-disabled-overlay",
                    div { class: "battle-disabled-message",
                        span { class: "battle-icon", "⚔️" }
                        p { "战斗中无法调整装备" }
                        p { class: "battle-hint", "请先结束当前战斗" }
                    }
                }
            }

            if *action_loading.read() {
                div { class: "equipment-loading-overlay", "处理中..." }
            }

            div { class: "equipment-slots-row",
                for (idx , slot) in slots.iter().enumerate() {
                    EquipmentSlotDraggable {
                        pokemon_id,
                        slot: slot.clone(),
                        slot_index: idx as u32,
                        action_loading,
                        refresh_trigger,
                    }
                }
            }

            {
                let has_equipment_selected = SELECTED_ITEM
                    .read()
                    .as_ref()
                    .map(|item| item.source_type == "equipment")
                    .unwrap_or(false);

                let section_class = if has_equipment_selected {
                    "equipment-inventory-section can-unequip"
                } else {
                    "equipment-inventory-section"
                };

                rsx! {
                    div {
                        class: section_class,
                        onclick: move |_| {
                            let selected = SELECTED_ITEM.read().clone();
                            if let Some(ref item) = selected {
                                if item.source_type == "equipment" {
                                    if let Some(from_slot) = item.slot_index {
                                        action_loading.set(true);
                                        let pid = pokemon_id;
                                        let mut refresh = refresh_trigger;
                                        spawn(async move {
                                            let api = NewApiClient::new();
                                            match api.unequip_item(pid, from_slot).await {
                                                Ok(resp) => {
                                                    show_success("卸下装备成功");
                                                    if resp.new_maxhp > 0 {
                                                        update_pokemon_hp(
                                                            pid,
                                                            resp.new_hp as i64,
                                                            resp.new_maxhp as i64,
                                                        );
                                                    }
                                                    refresh += 1;
                                                }
                                                Err(e) => {
                                                    show_error(format!("卸下装备失败: {}", e));
                                                }
                                            }
                                            action_loading.set(false);
                                        });
                                        *SELECTED_ITEM.write() = None;
                                    }
                                }
                            }
                        },
                        div { class: "inventory-header",
                            span { class: "inventory-title", "物品栏" }
                            if SELECTED_ITEM.read().as_ref().map(|s| s.source_type == "inventory").unwrap_or(false) {
                                span { class: "equip-hint", "点击装备槽以装载" }
                            }
                        }
                        div { class: "equipment-inventory-grid",
                            InventoryGridContent {
                                pokemon_id,
                                owned_items: owned_items.clone(),
                                action_loading,
                                refresh_trigger,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InventoryGridContent(
    owned_items: Vec<EquipmentItem>,
    action_loading: Signal<bool>,
    pokemon_id: u64,
    refresh_trigger: Signal<u64>,
) -> Element {
    let _action_loading = action_loading;
    let _pokemon_id = pokemon_id;
    let _refresh_trigger = refresh_trigger;

    let available_items: Vec<EquipmentItem> = owned_items
        .into_iter()
        .filter(|item| item.available_count > 0)
        .collect();

    if available_items.is_empty() {
        return rsx! {
            div { class: "inventory-empty", "暂无可用装备物品" }
        };
    }

    rsx! {
        for item in available_items {
            InventoryItemCard { item }
        }
    }
}

#[component]
fn EquipmentSlotDraggable(
    slot: EquipmentSlot,
    slot_index: u32,
    action_loading: Signal<bool>,
    pokemon_id: u64,
    refresh_trigger: Signal<u64>,
) -> Element {
    let has_equipment = slot.item.is_some();
    let slot_item = slot.item.clone();

    let is_selected = SELECTED_ITEM
        .read()
        .as_ref()
        .map(|item| item.source_type == "equipment" && item.slot_index == Some(slot_index))
        .unwrap_or(false);

    let has_inventory_selected = SELECTED_ITEM
        .read()
        .as_ref()
        .map(|item| item.source_type == "inventory")
        .unwrap_or(false);

    let mut show_tooltip = use_signal(|| false);
    let mut tooltip_position = use_signal(|| (0.0f64, 0.0f64, 0.0f64));

    let handle_click = move |_| {
        let selected = SELECTED_ITEM.read().clone();

        if let Some(ref item) = selected {
            if item.source_type == "inventory" {
                let need_unequip_first = has_equipment;
                action_loading.set(true);
                let pid = pokemon_id;
                let myitem_id = item.myitem_id;
                let mut refresh = refresh_trigger;
                spawn(async move {
                    let api = NewApiClient::new();
                    if need_unequip_first {
                        if let Err(e) = api.unequip_item(pid, slot_index).await {
                            show_error(format!("卸下装备失败: {}", e));
                            action_loading.set(false);
                            return;
                        }
                    }
                    match api
                        .equip_item(pid, myitem_id, Some(slot_index as i32))
                        .await
                    {
                        Ok(resp) => {
                            show_success("装备成功");
                            if resp.new_maxhp > 0 {
                                update_pokemon_hp(pid, resp.new_hp as i64, resp.new_maxhp as i64);
                            }
                            refresh += 1;
                        }
                        Err(e) => {
                            show_error(format!("装备失败: {}", e));
                        }
                    }
                    action_loading.set(false);
                });
                *SELECTED_ITEM.write() = None;
            } else if item.source_type == "equipment" {
                if let Some(from_slot) = item.slot_index {
                    if from_slot != slot_index {
                        let need_unequip_first = has_equipment;
                        action_loading.set(true);
                        let pid = pokemon_id;
                        let myitem_id = item.myitem_id;
                        let mut refresh = refresh_trigger;
                        spawn(async move {
                            let api = NewApiClient::new();
                            if need_unequip_first {
                                if let Err(e) = api.unequip_item(pid, slot_index).await {
                                    show_error(format!("卸下装备失败: {}", e));
                                    action_loading.set(false);
                                    return;
                                }
                            }
                            if let Err(e) = api.unequip_item(pid, from_slot).await {
                                show_error(format!("卸下装备失败: {}", e));
                                action_loading.set(false);
                                return;
                            }
                            match api
                                .equip_item(pid, myitem_id, Some(slot_index as i32))
                                .await
                            {
                                Ok(resp) => {
                                    show_success("装备移动成功");
                                    if resp.new_maxhp > 0 {
                                        update_pokemon_hp(
                                            pid,
                                            resp.new_hp as i64,
                                            resp.new_maxhp as i64,
                                        );
                                    }
                                    refresh += 1;
                                }
                                Err(e) => {
                                    show_error(format!("装备失败: {}", e));
                                }
                            }
                            action_loading.set(false);
                        });
                    }
                }
                *SELECTED_ITEM.write() = None;
            }
        } else if has_equipment {
            if let Some(ref item) = slot_item {
                *SELECTED_ITEM.write() = Some(SelectedItem {
                    source_type: "equipment".to_string(),
                    myitem_id: item.myitem_id,
                    name: item.name.clone(),
                    image: item.image.clone(),
                    slot_index: Some(slot_index),
                });
            }
        }
    };

    let overlay_class = if is_selected {
        "equip-slot-overlay selected"
    } else if has_inventory_selected {
        "equip-slot-overlay can-equip"
    } else {
        "equip-slot-overlay"
    };

    rsx! {
        div {
            class: "equip-slot",
            onclick: handle_click,
            onmouseenter: move |evt: Event<MouseData>| {
                if slot.item.is_some() {
                    let coords = evt.data().client_coordinates();
                    tooltip_position.set((coords.x, coords.y, 48.0));
                    show_tooltip.set(true);
                }
            },
            onmouseleave: move |_| {
                show_tooltip.set(false);
            },
            div { class: overlay_class }
            div { class: if has_equipment { "equip-slot-inner filled" } else { "equip-slot-inner empty" },
                if let Some(ref item) = slot.item {
                    {
                        let img_src = if item.image.contains('.') {
                            item.image.clone()
                        } else {
                            format!("{}.gif", item.image)
                        };
                        rsx! {
                            img {
                                class: "equip-icon",
                                src: "{IMG_PATH}/item/{img_src}",
                                alt: "{item.name}",
                            }
                        }
                    }
                } else {
                    span { class: "equip-plus", "＋" }
                }
            }
            if *show_tooltip.read() {
                if let Some(ref item) = slot.item {
                    EquipmentTooltipInline {
                        item: item.clone(),
                        left: tooltip_position.read().0,
                        bottom: tooltip_position.read().1,
                        width: tooltip_position.read().2,
                    }
                }
            }
        }
    }
}

#[component]
fn InventoryItemCard(item: EquipmentItem) -> Element {
    let is_equipped = item.is_equipped;
    let available_count = item.available_count;

    let item_for_click = item.clone();
    let item_for_display = item.clone();
    let item_for_tooltip = item.clone();

    let mut show_tooltip = use_signal(|| false);
    let mut tooltip_position = use_signal(|| (0.0f64, 0.0f64, 0.0f64));

    let handle_click = move |_| {
        if !is_equipped {
            *SELECTED_ITEM.write() = Some(SelectedItem {
                source_type: "inventory".to_string(),
                myitem_id: item_for_click.myitem_id,
                name: item_for_click.name.clone(),
                image: item_for_click.image.clone(),
                slot_index: None,
            });
        }
    };

    let image_src = if item_for_display.image.contains('.') {
        item_for_display.image.clone()
    } else {
        format!("{}.gif", item_for_display.image)
    };

    let item_class = if is_equipped {
        "inventory-item equipped"
    } else {
        "inventory-item"
    };

    rsx! {
        div { class: if is_equipped { "inventory-item-wrapper equipped" } else { "inventory-item-wrapper" },
            div {
                class: item_class,
                onclick: handle_click,
                onmouseenter: move |evt: Event<MouseData>| {
                    let coords = evt.data().client_coordinates();
                    tooltip_position.set((coords.x, coords.y, 48.0));
                    show_tooltip.set(true);
                },
                onmouseleave: move |_| {
                    show_tooltip.set(false);
                },
                div { class: "inventory-item-icon",
                    img {
                        src: "{IMG_PATH}/item/{image_src}",
                        alt: "{item_for_display.name}",
                    }
                }
                if is_equipped {
                    div { class: "equipped-badge", "已装备" }
                }
                if available_count > 1 {
                    div { class: "item-count-badge", "{available_count}" }
                }
            }
            if *show_tooltip.read() {
                EquipmentTooltipInline {
                    item: item_for_tooltip.clone(),
                    left: tooltip_position.read().0,
                    bottom: tooltip_position.read().1,
                    width: tooltip_position.read().2,
                }
            }
        }
    }
}

#[component]
fn EquipmentTooltipInline(item: EquipmentItem, left: f64, bottom: f64, width: f64) -> Element {
    let img_src = if item.image.contains('.') {
        item.image.clone()
    } else {
        format!("{}.gif", item.image)
    };

    let tooltip_width = 180.0;
    let tooltip_left = left + (width - tooltip_width) / 2.0;
    let tooltip_top = bottom + 8.0;

    rsx! {
        div {
            class: "equipment-tooltip-inline",
            style: "left: {tooltip_left}px; top: {tooltip_top}px;",
            div { class: "tooltip-header",
                img {
                    class: "tooltip-icon",
                    src: "{IMG_PATH}/item/{img_src}",
                    alt: "{item.name}",
                }
                span { class: "tooltip-name", "{item.name}" }
            }
            if !item.description.is_empty() {
                div { class: "tooltip-desc", "{item.description}" }
            }
            div { class: "tooltip-stats",
                if item.equipment_hp > 0 {
                    div { class: "tooltip-stat hp", "HP +{item.equipment_hp}" }
                }
                if item.equipment_atk > 0 {
                    div { class: "tooltip-stat atk", "攻击 +{item.equipment_atk}" }
                }
                if item.equipment_def > 0 {
                    div { class: "tooltip-stat def", "防御 +{item.equipment_def}" }
                }
                if item.equipment_spatk > 0 {
                    div { class: "tooltip-stat spat", "特攻 +{item.equipment_spatk}" }
                }
                if item.equipment_spdef > 0 {
                    div { class: "tooltip-stat spdef", "特防 +{item.equipment_spdef}" }
                }
                if item.equipment_sd > 0 {
                    div { class: "tooltip-stat spd", "速度 +{item.equipment_sd}" }
                }
            }
            if item.is_equipped {
                div { class: "tooltip-equipped-hint", "⚠ 此装备已被装备" }
            }
        }
    }
}

fn calculate_total_bonuses(slots: &[EquipmentSlot]) -> Vec<(String, i64)> {
    let mut hp = 0i64;
    let mut atk = 0i64;
    let mut def = 0i64;
    let mut spat = 0i64;
    let mut spdef = 0i64;
    let mut spd = 0i64;

    for slot in slots {
        if let Some(ref item) = slot.item {
            hp += item.equipment_hp as i64;
            atk += item.equipment_atk as i64;
            def += item.equipment_def as i64;
            spat += item.equipment_spatk as i64;
            spdef += item.equipment_spdef as i64;
            spd += item.equipment_sd as i64;
        }
    }

    let bonuses = vec![
        ("HP".to_string(), hp),
        ("攻击".to_string(), atk),
        ("防御".to_string(), def),
        ("特攻".to_string(), spat),
        ("特防".to_string(), spdef),
        ("速度".to_string(), spd),
    ];

    bonuses
}

#[component]
fn StateTagWithTooltip(state: u8, state_text: String, state_class: String) -> Element {
    let mut show_tooltip = use_signal(|| false);
    let mut tooltip_position = use_signal(|| (0.0f64, 0.0f64, 0.0f64));

    let state_info = get_state_info(state);

    rsx! {
        span {
            class: "state-tag state-{state_class}",
            onmouseenter: move |event| {
                let coords = event.client_coordinates();
                tooltip_position.set((coords.x, coords.y, 60.0));
                show_tooltip.set(true);
            },
            onmouseleave: move |_| {
                show_tooltip.set(false);
            },
            "{state_text}"
        }

        if *show_tooltip.read() {
            StateTooltip {
                state_info: state_info.clone(),
                left: tooltip_position.read().0,
                bottom: tooltip_position.read().1,
                width: tooltip_position.read().2,
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct StateInfo {
    name: String,
    description: String,
    hp_mult: f64,
    atk_mult: f64,
    def_mult: f64,
    spatk_mult: f64,
    spdef_mult: f64,
    speed_mult: f64,
}

fn get_state_info(state: u8) -> StateInfo {
    match state {
        0 => StateInfo {
            name: "濒危".to_string(),
            description: "宠物生命垂危，无法战斗".to_string(),
            hp_mult: 0.0,
            atk_mult: 0.0,
            def_mult: 0.0,
            spatk_mult: 0.0,
            spdef_mult: 0.0,
            speed_mult: 0.0,
        },
        2 => StateInfo {
            name: "生病".to_string(),
            description: "宠物身体不适，全属性下降".to_string(),
            hp_mult: 0.9,
            atk_mult: 0.9,
            def_mult: 0.9,
            spatk_mult: 0.9,
            spdef_mult: 0.9,
            speed_mult: 0.9,
        },
        3 => StateInfo {
            name: "生病".to_string(),
            description: "病情加重，全属性大幅下降".to_string(),
            hp_mult: 0.8,
            atk_mult: 0.8,
            def_mult: 0.8,
            spatk_mult: 0.8,
            spdef_mult: 0.8,
            speed_mult: 0.8,
        },
        4 => StateInfo {
            name: "生病".to_string(),
            description: "病情严重，全属性骤降".to_string(),
            hp_mult: 0.5,
            atk_mult: 0.5,
            def_mult: 0.5,
            spatk_mult: 0.5,
            spdef_mult: 0.5,
            speed_mult: 0.5,
        },
        5 => StateInfo {
            name: "饥饿".to_string(),
            description: "宠物饿了，全属性轻微下降".to_string(),
            hp_mult: 0.9,
            atk_mult: 0.9,
            def_mult: 0.9,
            spatk_mult: 0.9,
            spdef_mult: 0.9,
            speed_mult: 0.9,
        },
        6 => StateInfo {
            name: "饥饿".to_string(),
            description: "极度饥饿，可能死亡或生病".to_string(),
            hp_mult: 0.8,
            atk_mult: 0.8,
            def_mult: 0.8,
            spatk_mult: 0.8,
            spdef_mult: 0.8,
            speed_mult: 0.8,
        },
        7 => StateInfo {
            name: "疲惫".to_string(),
            description: "过度劳累，全属性骤降".to_string(),
            hp_mult: 0.5,
            atk_mult: 0.5,
            def_mult: 0.5,
            spatk_mult: 0.5,
            spdef_mult: 0.5,
            speed_mult: 0.5,
        },
        8 => StateInfo {
            name: "兴奋".to_string(),
            description: "斗志昂扬，攻击提升".to_string(),
            hp_mult: 1.1,
            atk_mult: 1.2,
            def_mult: 1.0,
            spatk_mult: 1.2,
            spdef_mult: 1.0,
            speed_mult: 1.0,
        },
        9 => StateInfo {
            name: "兴奋".to_string(),
            description: "极度亢奋，攻击大增但防御下降".to_string(),
            hp_mult: 1.2,
            atk_mult: 1.5,
            def_mult: 0.5,
            spatk_mult: 1.5,
            spdef_mult: 0.5,
            speed_mult: 1.2,
        },
        10 => StateInfo {
            name: "兴奋".to_string(),
            description: "爆发状态，全属性翻倍！".to_string(),
            hp_mult: 2.0,
            atk_mult: 2.0,
            def_mult: 2.0,
            spatk_mult: 2.0,
            spdef_mult: 2.0,
            speed_mult: 2.0,
        },
        11 => StateInfo {
            name: "受伤".to_string(),
            description: "战斗受伤，HP上限下降".to_string(),
            hp_mult: 0.8,
            atk_mult: 1.0,
            def_mult: 1.0,
            spatk_mult: 1.0,
            spdef_mult: 1.0,
            speed_mult: 1.0,
        },
        12 => StateInfo {
            name: "开心".to_string(),
            description: "心情愉悦，HP回复加快".to_string(),
            hp_mult: 1.3,
            atk_mult: 1.0,
            def_mult: 1.0,
            spatk_mult: 1.0,
            spdef_mult: 1.0,
            speed_mult: 1.0,
        },
        13 => StateInfo {
            name: "开心".to_string(),
            description: "非常开心，HP和速度提升".to_string(),
            hp_mult: 1.1,
            atk_mult: 1.0,
            def_mult: 1.0,
            spatk_mult: 1.0,
            spdef_mult: 1.0,
            speed_mult: 1.2,
        },
        14 => StateInfo {
            name: "开心".to_string(),
            description: "极度快乐，HP、防御、速度大增".to_string(),
            hp_mult: 1.3,
            atk_mult: 1.0,
            def_mult: 1.2,
            spatk_mult: 1.0,
            spdef_mult: 1.2,
            speed_mult: 1.6,
        },
        15 => StateInfo {
            name: "惊慌".to_string(),
            description: "受到惊吓，全属性下降".to_string(),
            hp_mult: 0.7,
            atk_mult: 0.7,
            def_mult: 0.7,
            spatk_mult: 0.7,
            spdef_mult: 0.7,
            speed_mult: 0.7,
        },
        16 => StateInfo {
            name: "自恋".to_string(),
            description: "自我欣赏，防御提升但攻击下降".to_string(),
            hp_mult: 1.0,
            atk_mult: 0.8,
            def_mult: 1.2,
            spatk_mult: 0.8,
            spdef_mult: 1.2,
            speed_mult: 1.0,
        },
        17 => StateInfo {
            name: "自恋".to_string(),
            description: "极度自恋，防御大增但攻击骤降".to_string(),
            hp_mult: 1.5,
            atk_mult: 0.6,
            def_mult: 1.5,
            spatk_mult: 0.6,
            spdef_mult: 1.5,
            speed_mult: 1.0,
        },
        18 => StateInfo {
            name: "愤怒".to_string(),
            description: "怒火中烧，攻击提升但防御下降".to_string(),
            hp_mult: 1.0,
            atk_mult: 1.5,
            def_mult: 0.8,
            spatk_mult: 1.5,
            spdef_mult: 0.8,
            speed_mult: 1.0,
        },
        19 => StateInfo {
            name: "愤怒".to_string(),
            description: "暴怒状态，攻击翻倍但防御骤降".to_string(),
            hp_mult: 1.0,
            atk_mult: 2.5,
            def_mult: 0.3,
            spatk_mult: 2.5,
            spdef_mult: 0.3,
            speed_mult: 1.0,
        },
        _ => StateInfo {
            name: "正常".to_string(),
            description: "状态良好".to_string(),
            hp_mult: 1.0,
            atk_mult: 1.0,
            def_mult: 1.0,
            spatk_mult: 1.0,
            spdef_mult: 1.0,
            speed_mult: 1.0,
        },
    }
}

#[component]
fn StateTooltip(state_info: StateInfo, left: f64, bottom: f64, width: f64) -> Element {
    let tooltip_width = 200.0;
    let tooltip_left = left + (width - tooltip_width) / 2.0;
    let tooltip_top = bottom + 8.0;

    let format_mult = |mult: f64, name: &str| -> Option<String> {
        if (mult - 1.0).abs() < 0.01 {
            return None;
        }
        if mult > 1.0 {
            Some(format!("{} +{:.0}%", name, (mult - 1.0) * 100.0))
        } else {
            Some(format!("{} -{:.0}%", name, (1.0 - mult) * 100.0))
        }
    };

    rsx! {
        div {
            class: "state-tooltip-inline",
            style: "left: {tooltip_left}px; top: {tooltip_top}px;",
            div { class: "tooltip-header",
                span { class: "tooltip-state-name", "{state_info.name}" }
            }
            div { class: "tooltip-desc", "{state_info.description}" }
            div { class: "tooltip-stats",
                if let Some(text) = format_mult(state_info.hp_mult, "HP") {
                    div { class: "tooltip-stat hp", "{text}" }
                }
                if let Some(text) = format_mult(state_info.atk_mult, "攻击") {
                    div { class: "tooltip-stat atk", "{text}" }
                }
                if let Some(text) = format_mult(state_info.def_mult, "防御") {
                    div { class: "tooltip-stat def", "{text}" }
                }
                if let Some(text) = format_mult(state_info.spatk_mult, "特攻") {
                    div { class: "tooltip-stat spat", "{text}" }
                }
                if let Some(text) = format_mult(state_info.spdef_mult, "特防") {
                    div { class: "tooltip-stat spdef", "{text}" }
                }
                if let Some(text) = format_mult(state_info.speed_mult, "速度") {
                    div { class: "tooltip-stat spd", "{text}" }
                }
            }
        }
    }
}

#[component]
pub fn SkillsTabContent(
    pokemon_id: u64,
    pokemon_level: u32,
    refresh_trigger: Signal<u64>,
) -> Element {
    let mut show_learn_modal = use_signal(|| false);
    let popup = use_popup();

    let _ = *refresh_trigger.read();

    let (_current_pm, skills) = {
        let state = POKEMON_STATE.read();
        let pm = state.list.iter().find(|pm| pm.id == pokemon_id).cloned();
        let sk = pm.as_ref().map(|pm| pm.skills.clone()).unwrap_or_default();
        (pm, sk)
    };

    let valid_skills: Vec<PokemonSkillSlot> = skills
        .clone()
        .into_iter()
        .filter(|s| s.type_id > 0)
        .collect();

    let empty_slots_count = 4 - valid_skills.len();

    let handle_learn_click = move |_: MouseEvent| {
        show_learn_modal.set(true);
    };

    let handle_learn_skill = move |skill: LearnableSkill| {
        if !skill.is_available {
            return;
        }

        let pokemon_id = pokemon_id;
        let skill_id = skill.id;
        let mut refresh_trigger = refresh_trigger;

        spawn(async move {
            start_global_loading("学习技能中...");
            let api = NewApiClient::new();
            match api.learn_skill(pokemon_id, skill_id, None).await {
                Ok(_) => {
                    show_success("技能学习成功");
                    refresh_trigger += 1;
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("battle") {
                        show_error("战斗状态下无法学习技能");
                    } else if error_msg.contains("Level requirement") {
                        show_error("等级不足，无法学习该技能");
                    } else if error_msg.contains("Skill slots are full") {
                        show_error("技能槽已满，请先遗忘一个技能");
                    } else {
                        show_error(format!("学习技能失败: {}", e));
                    }
                }
            }
            stop_global_loading();
            show_learn_modal.set(false);
        });
    };

    rsx! {
        div { class: "skills-grid-container",
            // 战斗中的遮罩层
            if use_battle_state() {
                div { class: "battle-disabled-overlay",
                    div { class: "battle-disabled-message",
                        span { class: "battle-icon", "⚔️" }
                        p { "战斗中无法调整技能" }
                        p { class: "battle-hint", "请先结束当前战斗" }
                    }
                }
            }

            div { class: "skills-grid",
            for skill in valid_skills.iter() {
                {
                    let skill_clone = skill.clone();
                    let pokemon_id_for_forget = pokemon_id;
                    let mut refresh_trigger_for_forget = refresh_trigger;
                    let mut popup_for_close = popup;

                    rsx! {
                        div {
                            class: "skill-card-v2 skill-slot-clickable",
                            onclick: move |evt: Event<MouseData>| {
                                evt.stop_propagation();
                                let skill_for_menu = skill_clone.clone();
                                let content = rsx! {
                                    PopupMenu {
                                        PopupMenuItem {
                                            danger: true,
                                            onclick: move |_| {
                                                popup_for_close.close();
                                                if skill_for_menu.type_id > 0 {
                                                    let pokemon_id = pokemon_id_for_forget;
                                                    let skill_id = skill_for_menu.type_id;
                                                    spawn(async move {
                                                        start_global_loading("遗忘技能中...");
                                                        let api = NewApiClient::new();
                                                        match api.forget_skill(pokemon_id, skill_id).await {
                                                            Ok(_) => {
                                                                show_success("技能遗忘成功");
                                                                refresh_trigger_for_forget += 1;
                                                            }
                                                            Err(e) => {
                                                                let error_msg = e.to_string();
                                                                if error_msg.contains("battle") {
                                                                    show_error("战斗状态下无法遗忘技能");
                                                                } else if error_msg.contains("PP not full") {
                                                                    show_error("PP 值未满，无法遗忘技能");
                                                                } else {
                                                                    show_error(format!("遗忘技能失败: {}", e));
                                                                }
                                                            }
                                                        }
                                                        stop_global_loading();
                                                    });
                                                }
                                            },
                                            "遗忘此技能"
                                        }
                                    }
                                };
                                popup_for_close
                                    .open_at_mouse(
                                        evt.data().client_coordinates().x,
                                        evt.data().client_coordinates().y + 10.0,
                                        content,
                                    );
                            },
                            div { class: "skill-card-top",
                                span { class: "skill-name",
                                    if skill.name.is_empty() {
                                        "技能 {skill.type_id}"
                                    } else {
                                        "{skill.name}"
                                    }
                                }
                                span { class: "skill-pp",
                                    em { "PP " }
                                    strong { "{skill.pp}" }
                                    if skill.max_pp > 0 {
                                        " /{skill.max_pp}"
                                    } else {
                                        " /{skill.pp}"
                                    }
                                }
                            }
                            div { class: "skill-card-bottom",
                                TypeBadge {
                                    skill_type: skill.skill_type.clone(),
                                    category: skill.category.clone(),
                                }
                                div { class: "skill-stats",
                                    if skill.category == "特攻" || skill.category == "特殊" {
                                        span { class: "skill-power",
                                            "特攻："
                                            strong { "{skill.power}" }
                                        }
                                    } else if skill.category == "物攻" || skill.category == "物理" {
                                        span { class: "skill-power",
                                            "物攻："
                                            strong { "{skill.power}" }
                                        }
                                    } else {
                                        span { class: "skill-power",
                                            "威力："
                                            strong { "{skill.power}" }
                                        }
                                    }
                                    span { class: "skill-pp",
                                        "PP："
                                        strong {
                                            "{skill.pp}"
                                            if skill.max_pp > 0 {
                                                "/{skill.max_pp}"
                                            }
                                        }
                                    }
                                }
                                if skill.level > 0 {
                                    span { class: "skill-level", "Lv {skill.level}" }
                                }
                            }
                        }
                    }
                }
            }

            for _ in 0..empty_slots_count {
                div { class: "skill-slot-empty", onclick: handle_learn_click,
                    div { class: "skill-slot-plus", "+" }
                    div { class: "skill-slot-hint", "点击学习新技能" }
                }
            }
        }
        }

        if *show_learn_modal.read() {
            SkillLearnModal {
                pokemon_id,
                on_close: move |_| {
                    show_learn_modal.set(false);
                },
                on_learn: handle_learn_skill,
                title: format!("学习新技能 (Lv.{})", pokemon_level),
            }
        }
    }
}

#[component]
fn SkillLearnModal(
    pokemon_id: u64,
    on_close: EventHandler<()>,
    on_learn: EventHandler<LearnableSkill>,
    title: String,
) -> Element {
    let resource = use_resource(move || async move {
        let api = NewApiClient::new();
        api.get_learnable_skills(pokemon_id).await
    });

    let loading = resource.read().is_none();
    let resource_data = resource.read();
    let available = resource_data
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .map(|d| d.available_skills.clone())
        .unwrap_or_default();
    let unavailable = resource_data
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .map(|d| d.unlocked_skills.clone())
        .unwrap_or_default();

    rsx! {
        Modal { is_open: true, on_close, title,
            div { class: "skill-learn-content",
                if loading {
                    div { class: "skill-learn-loading", "加载技能列表中..." }
                } else if available.is_empty() && unavailable.is_empty() {
                    div { class: "skill-learn-empty", "暂无可学习的技能" }
                } else {
                    if !available.is_empty() {
                        div { class: "skill-section-title", "可学习" }
                        for skill in available.iter() {
                            SkillLearnItem {
                                skill: skill.clone(),
                                on_learn,
                            }
                        }
                    }

                    if !unavailable.is_empty() {
                        div { class: "skill-section-title", "等级不足" }
                        for skill in unavailable.iter() {
                            SkillLearnItem {
                                skill: skill.clone(),
                                on_learn,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillLearnItem(skill: LearnableSkill, on_learn: EventHandler<LearnableSkill>) -> Element {
    let is_available = skill.is_available;
    let skill_clone = skill.clone();

    let handle_click = move |_: MouseEvent| {
        if !is_available {
            return;
        }
        on_learn.call(skill_clone.clone());
    };

    rsx! {
        div {
            class: if is_available { "skill-learn-item" } else { "skill-learn-item disabled" },
            onclick: handle_click,
            div { class: "skill-learn-item-header",
                span { class: "skill-learn-name", "{skill.name}" }
                if is_available {
                    span { class: "skill-learn-learn-btn", "学习" }
                } else {
                    span { class: "skill-learn-level-hint", "需要 Lv.{skill.required_level}" }
                }
            }
            div { class: "skill-learn-item-body",
                span { class: "badge badge-type-{skill.skill_type}", "{skill.skill_type}" }
                if !skill.category.is_empty() {
                    span { class: "badge badge-cat-{skill.category}", "{skill.category}" }
                }
                if skill.power > 0 {
                    span { class: "skill-power", "威力：{skill.power}" }
                }
                span { class: "skill-pp", "PP: {skill.max_pp}" }
            }
            if !skill.description.is_empty() {
                div { class: "skill-description", "{skill.description}" }
            }
        }
    }
}

#[component]
fn EffortValueCalculatorModal(on_close: EventHandler<()>) -> Element {
    // 努力值状态：HP, 攻击, 防御, 特攻, 特防, 速度
    let current_hp = use_signal(|| 0u32);
    let current_atk = use_signal(|| 0u32);
    let current_def = use_signal(|| 0u32);
    let current_spa = use_signal(|| 0u32);
    let current_spd = use_signal(|| 0u32);
    let current_spe = use_signal(|| 0u32);

    let added_hp = use_signal(|| 0u32);
    let added_atk = use_signal(|| 0u32);
    let added_def = use_signal(|| 0u32);
    let added_spa = use_signal(|| 0u32);
    let added_spd = use_signal(|| 0u32);
    let added_spe = use_signal(|| 0u32);

    // 计算总计
    let total_hp = *current_hp.read() + *added_hp.read();
    let total_atk = *current_atk.read() + *added_atk.read();
    let total_def = *current_def.read() + *added_def.read();
    let total_spa = *current_spa.read() + *added_spa.read();
    let total_spd = *current_spd.read() + *added_spd.read();
    let total_spe = *current_spe.read() + *added_spe.read();

    let grand_total = total_hp + total_atk + total_def + total_spa + total_spd + total_spe;

    // 验证状态
    let stats_valid = total_hp <= 255
        && total_atk <= 255
        && total_def <= 255
        && total_spa <= 255
        && total_spd <= 255
        && total_spe <= 255
        && grand_total <= 510;

    // 属性名称
    let stat_names = ["HP", "攻击", "防御", "特攻", "特防", "速度"];
    let current_values = [
        *current_hp.read(),
        *current_atk.read(),
        *current_def.read(),
        *current_spa.read(),
        *current_spd.read(),
        *current_spe.read(),
    ];
    let added_values = [
        *added_hp.read(),
        *added_atk.read(),
        *added_def.read(),
        *added_spa.read(),
        *added_spd.read(),
        *added_spe.read(),
    ];
    let total_values = [
        total_hp, total_atk, total_def, total_spa, total_spd, total_spe,
    ];

    // 创建输入处理函数的宏
    let create_handler = |mut signal: Signal<u32>, max: u32| {
        move |e: Event<FormData>| {
            if let Ok(val) = e.value().parse::<u32>() {
                if val <= max {
                    signal.set(val);
                }
            }
        }
    };

    rsx! {
        ModalLg {
            is_open: true,
            on_close,
            title: "努力值分配计算器".to_string(),
            div { class: "ev-calculator",
                div { class: "ev-notice",
                    p { "注意事项：" }
                    ul {
                        li { "每项努力值最高 255 点" }
                        li { "6 项努力值总计最高 510 点" }
                        li { "本工具仅用于计算，不会保存数据" }
                    }
                }

                div { class: "ev-table",
                    // 表头
                    div { class: "ev-table-header",
                        div { class: "ev-cell ev-label", "努力值" }
                        for i in 0..6 {
                            div { class: "ev-cell", "{stat_names[i]}" }
                        }
                        div { class: "ev-cell ev-total", "总计" }
                    }

                    // 总分配行
                    div { class: "ev-table-row",
                        div { class: "ev-cell ev-label", "总分配" }
                        for i in 0..6 {
                            div { class: "ev-cell", "{total_values[i]}" }
                        }
                        div {
                            class: "ev-cell ev-total grand-total",
                            class: if grand_total > 510 { "invalid" } else { "" },
                            "{grand_total}"
                        }
                    }

                    // 已分配行
                    div { class: "ev-table-row",
                        div { class: "ev-cell ev-label", "已分配" }
                        for i in 0..6 {
                            div { class: "ev-cell", "{current_values[i]}" }
                        }
                        div { class: "ev-cell ev-total", "{current_values.iter().sum::<u32>()}" }
                    }

                    // 待分配行
                    div { class: "ev-table-row input-row",
                        div { class: "ev-cell ev-label", "待分配" }
                        // HP 输入
                        div { class: "ev-cell",
                            input {
                                class: "ev-input",
                                r#type: "number",
                                min: 0,
                                max: 255,
                                value: "{added_hp}",
                                oninput: create_handler(added_hp, 255),
                            }
                        }
                        // 攻击输入
                        div { class: "ev-cell",
                            input {
                                class: "ev-input",
                                r#type: "number",
                                min: 0,
                                max: 255,
                                value: "{added_atk}",
                                oninput: create_handler(added_atk, 255),
                            }
                        }
                        // 防御输入
                        div { class: "ev-cell",
                            input {
                                class: "ev-input",
                                r#type: "number",
                                min: 0,
                                max: 255,
                                value: "{added_def}",
                                oninput: create_handler(added_def, 255),
                            }
                        }
                        // 特攻输入
                        div { class: "ev-cell",
                            input {
                                class: "ev-input",
                                r#type: "number",
                                min: 0,
                                max: 255,
                                value: "{added_spa}",
                                oninput: create_handler(added_spa, 255),
                            }
                        }
                        // 特防输入
                        div { class: "ev-cell",
                            input {
                                class: "ev-input",
                                r#type: "number",
                                min: 0,
                                max: 255,
                                value: "{added_spd}",
                                oninput: create_handler(added_spd, 255),
                            }
                        }
                        // 速度输入
                        div { class: "ev-cell",
                            input {
                                class: "ev-input",
                                r#type: "number",
                                min: 0,
                                max: 255,
                                value: "{added_spe}",
                                oninput: create_handler(added_spe, 255),
                            }
                        }
                        // 总计待分配
                        div { class: "ev-cell ev-total", "{added_values.iter().sum::<u32>()}" }
                    }
                }

                div { class: "ev-status",
                    if stats_valid {
                        span { class: "ev-status-valid", "✓ 配置有效" }
                    } else {
                        span { class: "ev-status-invalid", "✕ 配置无效（超过限制）" }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum EvoPathTab {
    Forward,
    Backward,
}

#[component]
fn EvolutionPathModal(pokemon_type_id: u64, on_close: EventHandler<()>) -> Element {
    let mut active_evo_tab = use_signal(|| EvoPathTab::Forward);

    let evo_data = use_resource(move || async move {
        let api = NewApiClient::new();
        api.get_evolution_path(pokemon_type_id)
            .await
            .map_err(|e| format!("查询失败: {}", e))
    });

    let loaded = evo_data
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();

    rsx! {
        Modal {
            is_open: true,
            on_close,
            title: "进化路径".to_string(),
            if evo_data.read().is_none() {
                div { class: "evo-path-loading", "正在查询进化数据..." }
            } else if let Some(data) = loaded {
                div { class: "evo-path-content",
                    div { class: "evo-path-current",
                        div { class: "evo-path-current-header",
                            img {
                                class: "evo-path-sprite",
                                src: "{IMG_PATH}/spm/{data.pokemon_id}.gif",
                                alt: "{data.pokemon_name}",
                            }
                            div { class: "evo-path-current-info",
                                span { class: "evo-path-name", "{data.pokemon_name}" }
                                div { class: "evo-path-types",
                                    span { class: "badge badge-type-{data.pokemon_type1}", "{data.pokemon_type1}" }
                                    if let Some(t2) = &data.pokemon_type2 {
                                        if !t2.is_empty() {
                                            span { class: "badge badge-type-{t2}", "{t2}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "evo-path-tab-bar",
                        button {
                            class: if active_evo_tab() == EvoPathTab::Forward { "evo-path-tab active" } else { "evo-path-tab" },
                            onclick: move |_| active_evo_tab.set(EvoPathTab::Forward),
                            "进化目标"
                        }
                        button {
                            class: if active_evo_tab() == EvoPathTab::Backward { "evo-path-tab active" } else { "evo-path-tab" },
                            onclick: move |_| active_evo_tab.set(EvoPathTab::Backward),
                            "进化来源"
                        }
                    }

                    match active_evo_tab() {
                        EvoPathTab::Forward => {
                            if !data.forward.is_empty() {
                                rsx! {
                                    div { class: "evo-path-section",
                                        for node in &data.forward {
                                            div { class: "evo-path-node",
                                                img {
                                                    class: "evo-path-sprite",
                                                    src: "{IMG_PATH}/spm/{node.id}.gif",
                                                    alt: "{node.name}",
                                                }
                                                div { class: "evo-path-node-info",
                                                    span { class: "evo-path-name", "{node.name}" }
                                                    div { class: "evo-path-types",
                                                        span { class: "badge badge-type-{node.type_1}", "{node.type_1}" }
                                                        if let Some(t2) = &node.type_2 {
                                                            if !t2.is_empty() {
                                                                span { class: "badge badge-type-{t2}", "{t2}" }
                                                            }
                                                        }
                                                    }
                                                    div { class: "evo-path-condition",
                                                        span { "📦 {node.condition_display}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "evo-path-empty",
                                        span { "该宝可梦没有已知的进化形态" }
                                    }
                                }
                            }
                        }
                        EvoPathTab::Backward => {
                            if !data.backward.is_empty() {
                                rsx! {
                                    div { class: "evo-path-section",
                                        for node in &data.backward {
                                            div { class: "evo-path-node",
                                                img {
                                                    class: "evo-path-sprite",
                                                    src: "{IMG_PATH}/spm/{node.id}.gif",
                                                    alt: "{node.name}",
                                                }
                                                div { class: "evo-path-node-info",
                                                    span { class: "evo-path-name", "{node.name}" }
                                                    div { class: "evo-path-types",
                                                        span { class: "badge badge-type-{node.type_1}", "{node.type_1}" }
                                                        if let Some(t2) = &node.type_2 {
                                                            if !t2.is_empty() {
                                                                span { class: "badge badge-type-{t2}", "{t2}" }
                                                            }
                                                        }
                                                    }
                                                    div { class: "evo-path-condition",
                                                        span { "📦 {node.condition_display}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "evo-path-empty",
                                        span { "该宝可梦没有已知的进化来源" }
                                    }
                                }
                            }
                        }
                    }
                }
            } else if let Some(err) = evo_data.read().as_ref().and_then(|r| r.as_ref().err()) {
                div { class: "evo-path-error", "查询失败: {err}" }
            }
        }
    }
}
