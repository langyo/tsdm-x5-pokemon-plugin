use crate::dioxus::prelude::*;
use strum::IntoEnumIterator;

use super::form_fields::{
    BoolField, Col, FloatField, FormSection, Row, SelectField, TextField, UnsignedNumberField,
};
use super::icon::IconName;
use super::icon_button::{ButtonSize, IconButton};
use super::pokemon_editor::PokemonAttributesEditor;
use super::tabs::Tabs;
use super::two_step_wizard::TwoStepWizardModal;
use crate::dioxus::{
    pages::shared::ActionModal,
    {
        components::search_box::{SearchBox, SearchResult},
        components::tag::Tag,
        state::{
            set_busy, set_notice, AdminNoticeLevel, FilterConditionType, FilterFieldState,
            LogicalOperation, ADMIN_BUSY,
        },
        utils::api::{
            add_pokemon_to_map, filter_pokemon_type, get_wild_pokemons_for_map,
            remove_pokemon_from_map,
        },
    },
};
use _utils::types::{
    api_map::WildPokemonInfo,
    map_boss::MapBoss,
    map_info::{MapAreaType, MapInfo},
};

/// 地图宠物标签页选项
#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum MapPetTab {
    #[default]
    Wild,
    Boss,
}

/// 获取当前区域类型的代码
fn map_area_type_code(map_area_type: &MapAreaType) -> String {
    match map_area_type {
        MapAreaType::Plain => "l",
        MapAreaType::Grass => "g",
        MapAreaType::Water => "p",
        MapAreaType::Sea => "s",
        // TODO: change `api_map.rs` context
        MapAreaType::SeaBottom => "b",
        MapAreaType::Mountain => "m",
        MapAreaType::Cave => "c",
        MapAreaType::Sand => "d",
        MapAreaType::Factory => "f",
        MapAreaType::Base => "t",
        MapAreaType::Town => "v",
        MapAreaType::Gym => "n",
        MapAreaType::Sky => "h",
        MapAreaType::DeepSea => "o",
        MapAreaType::Lava => "k",
    }
    .to_string()
}

/// 获取所有 MapAreaType 选项
fn map_area_type_options() -> Vec<(String, String)> {
    MapAreaType::iter()
        .map(|t| {
            let code = map_area_type_code(&t);
            (code, t.to_string())
        })
        .collect()
}

/// 移除宠物的辅助函数
fn spawn_remove_pokemon(
    map_id: u64,
    pokemon_id: u64,
    mut wild_pokemons: Signal<Vec<WildPokemonInfo>>,
) {
    set_busy(true);
    spawn(async move {
        match remove_pokemon_from_map(map_id, pokemon_id).await {
            Ok(()) => {
                let mut updated = wild_pokemons.read().clone();
                updated.retain(|p| p.id != pokemon_id);
                // 保持按 ID 排序（虽然 retain 不改变顺序，但确保一致性）
                updated.sort_by(|a, b| a.id.cmp(&b.id));
                wild_pokemons.set(updated);
                set_notice(AdminNoticeLevel::Success, "已移除宠物".to_string());
            }
            Err(e) => {
                set_notice(AdminNoticeLevel::Error, format!("移除失败: {}", e));
            }
        }
        set_busy(false);
    });
}

/// 添加宠物到地图的辅助函数
fn spawn_add_pokemon_to_map(
    map_id: u64,
    pokemon_type_id: u64,
    pokemon_name: String,
    mut wild_pokemons: Signal<Vec<WildPokemonInfo>>,
) {
    set_busy(true);
    spawn(async move {
        match add_pokemon_to_map(map_id, pokemon_type_id).await {
            Ok(()) => {
                let mut updated = wild_pokemons.read().clone();
                updated.push(WildPokemonInfo {
                    id: pokemon_type_id,
                    name: pokemon_name,
                });
                // 按 ID 排序
                updated.sort_by(|a, b| a.id.cmp(&b.id));
                wild_pokemons.set(updated);
                set_notice(AdminNoticeLevel::Success, "已添加宠物到地图".to_string());
            }
            Err(e) => {
                set_notice(AdminNoticeLevel::Error, format!("添加失败: {}", e));
            }
        }
        set_busy(false);
    });
}

/// 地图编辑器 Modal
#[component]
pub fn MapInfoEditorModal(
    title: String,
    data: MapInfo,
    save_text: String,
    disabled: bool,
    on_save: EventHandler<MapInfo>,
    on_close: EventHandler<()>,
) -> Element {
    let is_busy = *ADMIN_BUSY.read();
    let mut draft = use_signal(|| data.clone());
    let map_id = draft.read().id;
    let wild_pokemons = use_signal(Vec::<WildPokemonInfo>::new);
    let mut wild_pokemons_loaded = use_signal(|| false);
    let mut editing_boss_index = use_signal(|| None::<usize>);

    // 地图宠物标签页：根据是否有 Boss 配置来决定默认标签页
    let mut map_pet_tab = use_signal(|| {
        if draft
            .read()
            .mode
            .get_boss_config()
            .map_or(true, |config| config.bosses.is_empty())
        {
            MapPetTab::Wild
        } else {
            MapPetTab::Boss
        }
    });

    // 添加 Boss 的两步流程状态：0=未开始, 1=选择宠物, 2=配置属性
    let mut boss_add_step = use_signal(|| 0u8);
    let mut boss_add_draft = use_signal(MapBoss::default);
    let mut boss_add_search_results = use_signal(Vec::<SearchResult>::new);
    let mut boss_add_search_loading = use_signal(|| false);

    // 添加野生宠物的 modal 状态
    let mut show_add_pokemon_modal = use_signal(|| false);
    let mut add_pokemon_search_results = use_signal(Vec::<SearchResult>::new);
    let mut add_pokemon_search_loading = use_signal(|| false);
    let mut add_pokemon_selected = use_signal(|| None::<SearchResult>);

    let area_type_options = map_area_type_options();
    let current_area_type_code = map_area_type_code(&draft.read().area_type);

    // 编辑模式时加载野生宠物列表
    use_effect(move || {
        let current_map_id = draft.read().id;
        if current_map_id == 0 || *wild_pokemons_loaded.read() {
            return;
        }
        wild_pokemons_loaded.set(true);
        let mut wild_pokemons_clone = wild_pokemons.clone();
        spawn(async move {
            match get_wild_pokemons_for_map(current_map_id).await {
                Ok(mut pokemons) => {
                    // 按 ID 排序
                    pokemons.sort_by(|a, b| a.id.cmp(&b.id));
                    wild_pokemons_clone.set(pokemons);
                }
                Err(e) => {
                    set_notice(AdminNoticeLevel::Error, format!("加载野生宠物失败: {}", e));
                    wild_pokemons_clone.set(Vec::new());
                }
            }
        });
    });

    rsx! {
        ActionModal { title, on_close,
            div { class: "admin-form-editor",
                // 基本信息
                FormSection { title: "基本信息".to_string(),
                    Row {
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "ID".to_string(),
                                value: draft.read().id,
                                min: Some(0),
                                disabled: true,
                                on_change: move |_v| (),
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 8,
                            TextField {
                                label: "名称".to_string(),
                                value: draft.read().name.clone(),
                                placeholder: Some("地图名称".to_string()),
                                disabled,
                                on_change: move |v| draft.write().name = v,
                                help: None,
                            }
                        }
                    }
                    Row {
                        Col { span: 6,
                            SelectField {
                                label: "地形类型".to_string(),
                                value: current_area_type_code.clone(),
                                options: area_type_options.clone(),
                                disabled,
                                on_change: move |v: String| {
                                    for t in MapAreaType::iter() {
                                        if map_area_type_code(&t) == v {
                                            draft.write().area_type = t;
                                            break;
                                        }
                                    }
                                },
                                help: None,
                            }
                        }
                        Col { span: 6,
                            BoolField {
                                label: "启用".to_string(),
                                value: draft.read().is_enabled,
                                disabled,
                                on_change: move |v| draft.write().is_enabled = v,
                                help: None,
                            }
                        }
                    }
                }

                // 宠物管理 - 独立的导航区域
                Tabs {
                    active: *map_pet_tab.read(),
                    options: vec![
                        (MapPetTab::Wild, "野生宠物".to_string()),
                        (MapPetTab::Boss, "Boss 挑战".to_string()),
                    ],
                    disabled,
                    on_change: move |tab| map_pet_tab.set(tab),
                }

                // 野生宠物标签页内容
                if matches!(*map_pet_tab.read(), MapPetTab::Wild) {
                    // 野怪等级
                    FormSection { title: "野怪等级".to_string(),
                        Row {
                            Col { span: 6,
                                UnsignedNumberField {
                                    label: "最低等级".to_string(),
                                    value: draft.read().min_level,
                                    min: Some(1),
                                    max: Some(255),
                                    disabled,
                                    on_change: move |v| draft.write().min_level = v,
                                    help: None,
                                    step: None,
                                }
                            }
                            Col { span: 6,
                                UnsignedNumberField {
                                    label: "最高等级".to_string(),
                                    value: draft.read().max_level,
                                    min: Some(1),
                                    max: Some(255),
                                    disabled,
                                    on_change: move |v| draft.write().max_level = v,
                                    help: None,
                                    step: None,
                                }
                            }
                        }
                    }

                    // 经验设置
                    FormSection { title: "经验设置".to_string(),
                        Row {
                            Col { span: 6,
                                UnsignedNumberField {
                                    label: "经验值".to_string(),
                                    value: draft.read().mode.get_experience().unwrap_or(0),
                                    min: Some(0),
                                    disabled,
                                    on_change: move |v| {
                                        let current_mode = draft.read().mode.clone();
                                        match current_mode {
                                            _utils::types::map_info::MapMode::Wild { experience_increase_times, .. } => {
                                                draft.write().mode = _utils::types::map_info::MapMode::Wild {
                                                    experience: v,
                                                    experience_increase_times,
                                                };
                                            }
                                            _utils::types::map_info::MapMode::Hybrid { bosses, experience_increase_times, .. } => {
                                                draft.write().mode = _utils::types::map_info::MapMode::Hybrid {
                                                    experience: v,
                                                    experience_increase_times,
                                                    bosses,
                                                };
                                            }
                                            _ => {
                                                // Boss mode doesn't have experience, convert to Wild
                                                draft.write().mode = _utils::types::map_info::MapMode::Wild {
                                                    experience: v,
                                                    experience_increase_times: 0,
                                                };
                                            }
                                        }
                                    },
                                    help: None,
                                    step: None,
                                    max: None,
                                }
                            }
                            Col { span: 6,
                                UnsignedNumberField {
                                    label: "经验倍率".to_string(),
                                    value: draft.read().mode.get_experience_increase_times().unwrap_or(0),
                                    min: Some(0),
                                    disabled,
                                    on_change: move |v| {
                                        let current_mode = draft.read().mode.clone();
                                        match current_mode {
                                            _utils::types::map_info::MapMode::Wild { experience, .. } => {
                                                draft.write().mode = _utils::types::map_info::MapMode::Wild {
                                                    experience,
                                                    experience_increase_times: v,
                                                };
                                            }
                                            _utils::types::map_info::MapMode::Hybrid { experience, bosses, .. } => {
                                                draft.write().mode = _utils::types::map_info::MapMode::Hybrid {
                                                    experience,
                                                    experience_increase_times: v,
                                                    bosses,
                                                };
                                            }
                                            _ => {
                                                // Boss mode doesn't have experience_increase_times, convert to Wild
                                                draft.write().mode = _utils::types::map_info::MapMode::Wild {
                                                    experience: 0,
                                                    experience_increase_times: v,
                                                };
                                            }
                                        }
                                    },
                                    help: None,
                                    step: None,
                                    max: None,
                                }
                            }
                        }
                    }

                    // 野生宠物管理
                    if map_id > 0 {
                        FormSection {
                            title: "野生宠物列表".to_string(),
                            actions: Some(rsx! {
                                IconButton {
                                    icon: IconName::Plus,
                                    tooltip: "添加宠物".to_string(),
                                    disabled: is_busy || disabled,
                                    size: Some(ButtonSize::ExtraSmall),
                                    onclick: move |_| {
                                        show_add_pokemon_modal.set(true);
                                    },
                                    class: None,
                                }
                            }),

                            if !*wild_pokemons_loaded.read() {
                                div { class: "admin-loading",
                                    div { class: "admin-loading__spinner" }
                                }
                            } else {
                                if wild_pokemons.read().is_empty() {
                                    div { class: "admin-empty-list",
                                        div { class: "admin-empty-list__content", "暂无野生宠物" }
                                    }
                                } else {
                                    div { class: "admin-wild-pokemon-tags",
                                        for pokemon in wild_pokemons.read().iter() {
                                            Tag {
                                                label: format!("{} #{}", pokemon.name, pokemon.id),
                                                disabled: is_busy,
                                                on_close: {
                                                    let pokemon_id = pokemon.id;
                                                    let map_id = draft.read().id;
                                                    let wild_pokemons_clone = wild_pokemons.clone();
                                                    move |_| {
                                                        spawn_remove_pokemon(map_id, pokemon_id, wild_pokemons_clone.clone());
                                                    }
                                                },
                                                class: Some("admin-tag--closable".to_string()),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        FormSection { title: "野生宠物列表".to_string(),
                            div { class: "admin-empty-list",
                                div { class: "admin-empty-list__content",
                                    "请先保存地图后再添加野生宠物"
                                }
                            }
                        }
                    }
                }

                // Boss 挑战标签页内容
                if matches!(*map_pet_tab.read(), MapPetTab::Boss) {
                    FormSection {
                        title: "Boss 挑战".to_string(),
                        actions: Some(rsx! {
                            if map_id > 0 {
                                IconButton {
                                    icon: IconName::Plus,
                                    tooltip: "添加 Boss".to_string(),
                                    disabled: is_busy || disabled,
                                    size: Some(ButtonSize::ExtraSmall),
                                    onclick: move |_| {
                                        boss_add_draft.set(MapBoss::default());
                                        boss_add_search_results.set(Vec::new());
                                        boss_add_step.set(1);
                                    },
                                    class: None,
                                }
                            }
                        }),

                        div { class: "admin-boss-config",
                            if draft.read().mode.get_boss_config().map_or(true, |config| config.bosses.is_empty()) {
                                // 紧凑的空状态提示
                                div { class: "admin-empty-hint",
                                    span { class: "admin-empty-hint__text", "暂无 Boss，点击右上角 + 添加" }
                                }
                            } else {
                                // 紧凑的 Boss 列表（一行一个）
                                div { class: "admin-boss-list-compact",
                                    for (index , boss) in draft.read().mode.get_boss_config().iter().flat_map(|config| config.bosses.iter()).enumerate() {
                                        div { class: "admin-boss-row",
                                            span { class: "admin-boss-row__name",
                                                "{boss.pokemon_name}"
                                            }
                                            span { class: "admin-boss-row__level",
                                                "Lv.{boss.level}"
                                            }
                                            span { class: "admin-boss-row__multiplier",
                                                "{boss.boss_multiplier}x"
                                            }
                                            div { class: "admin-boss-row__actions",
                                                button {
                                                    class: "admin-btn-link admin-btn-link--small",
                                                    disabled: is_busy || disabled,
                                                    r#type: "button",
                                                    onclick: move |_| editing_boss_index.set(Some(index)),
                                                    "编辑"
                                                }
                                                span { class: "admin-boss-row__divider", "·" }
                                                button {
                                                    class: "admin-btn-link admin-btn-link--small admin-btn-link--danger",
                                                    disabled: is_busy || disabled,
                                                    r#type: "button",
                                                    onclick: move |_| {
                                                        let current_mode = draft.read().mode.clone();
                                                        match current_mode {
                                                            _utils::types::map_info::MapMode::Boss { mut bosses } => {
                                                                bosses.bosses.remove(index);
                                                                draft.write().mode = _utils::types::map_info::MapMode::Boss { bosses };
                                                            }
                                                            _utils::types::map_info::MapMode::Hybrid { experience, experience_increase_times, mut bosses } => {
                                                                bosses.bosses.remove(index);
                                                                draft.write().mode = _utils::types::map_info::MapMode::Hybrid {
                                                                    experience,
                                                                    experience_increase_times,
                                                                    bosses,
                                                                };
                                                            }
                                                            _ => {}
                                                        }
                                                    },
                                                    "删除"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 操作按钮
                div { class: "admin-form-actions",
                    button {
                        class: "admin-btn",
                        disabled: is_busy || disabled,
                        r#type: "button",
                        onclick: move |_| on_close.call(()),
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn--primary",
                        disabled: is_busy || disabled,
                        r#type: "button",
                        onclick: move |_| on_save.call(draft.read().clone()),
                        "{save_text}"
                    }
                }

                // 编辑已有 Boss 的 Modal
                if let Some(editing_idx) = *editing_boss_index.read() {
                    BossEditModal {
                        title: "编辑 Boss".to_string(),
                        boss: draft.read().mode.get_boss_config().and_then(|config| config.bosses.get(editing_idx)).cloned().unwrap_or_default(),
                        on_save: move |updated_boss| {
                            if let Some(idx) = *editing_boss_index.read() {
                                let current_mode = draft.read().mode.clone();
                                match current_mode {
                                    _utils::types::map_info::MapMode::Boss { mut bosses } => {
                                        bosses.bosses[idx] = updated_boss;
                                        draft.write().mode = _utils::types::map_info::MapMode::Boss { bosses };
                                    }
                                    _utils::types::map_info::MapMode::Hybrid { experience, experience_increase_times, mut bosses } => {
                                        bosses.bosses[idx] = updated_boss;
                                        draft.write().mode = _utils::types::map_info::MapMode::Hybrid {
                                            experience,
                                            experience_increase_times,
                                            bosses,
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            editing_boss_index.set(None);
                        },
                        on_close: move |_| editing_boss_index.set(None),
                    }
                }

                // 添加新 Boss 的两步 Modal
                if *boss_add_step.read() >= 1 {
                    AddBossModal {
                        step: *boss_add_step.read(),
                        draft: boss_add_draft.read().clone(),
                        search_results: boss_add_search_results.read().clone(),
                        search_loading: *boss_add_search_loading.read(),
                        on_search: move |query: String| {
                            let query: String = query.trim().to_string();
                            if query.is_empty() {
                                boss_add_search_results.set(Vec::new());
                                return;
                            }
                            boss_add_search_loading.set(true);
                            let mut sr = boss_add_search_results.clone();
                            let mut sl = boss_add_search_loading.clone();
                            spawn(async move {
                                let filters = vec![
                                    FilterFieldState {
                                        tag: "名称".to_string(),
                                        operator: FilterConditionType::Text(LogicalOperation::Equal),
                                        value: query,
                                        enum_options: Vec::new(),
                                        enabled: true,
                                    },
                                ];
                                let result = filter_pokemon_type(
                                        filters.iter().filter_map(|f| f.to_package()).collect(),
                                    )
                                    .await;
                                sl.set(false);
                                match result {
                                    Ok(types) => {
                                        sr.set(
                                            types
                                                .into_iter()
                                                .map(|t| SearchResult {
                                                    id: t.id,
                                                    name: t.name.clone(),
                                                })
                                                .collect(),
                                        );
                                    }
                                    Err(_) => {
                                        sr.set(Vec::new());
                                    }
                                }
                            });
                        },
                        on_select_pokemon: move |result: SearchResult| {
                            boss_add_draft.write().pokemon_type_id = result.id;
                            boss_add_draft.write().pokemon_name = result.name;
                            boss_add_search_results.set(Vec::new());
                        },
                        on_next: move |_| boss_add_step.set(2),
                        on_update_draft: move |updated: MapBoss| {
                            boss_add_draft.set(updated);
                        },
                        on_back: move |_| boss_add_step.set(1),
                        on_confirm: move |_| {
                            let new_boss = boss_add_draft.read().clone();
                            if new_boss.pokemon_type_id > 0 {
                                let current_mode = draft.read().mode.clone();
                                let new_mode = match current_mode {
                                    _utils::types::map_info::MapMode::Boss { mut bosses } => {
                                        bosses.bosses.push(new_boss);
                                        _utils::types::map_info::MapMode::Boss { bosses }
                                    }
                                    _utils::types::map_info::MapMode::Hybrid { experience, experience_increase_times, mut bosses } => {
                                        bosses.bosses.push(new_boss);
                                        _utils::types::map_info::MapMode::Hybrid {
                                            experience,
                                            experience_increase_times,
                                            bosses,
                                        }
                                    }
                                    _ => {
                                        // Convert from Wild to Boss mode
                                        _utils::types::map_info::MapMode::Boss {
                                            bosses: _utils::types::map_boss::MapBossConfig {
                                                bosses: vec![new_boss],
                                            },
                                        }
                                    }
                                };
                                draft.write().mode = new_mode;
                            }
                            boss_add_step.set(0);
                        },
                        on_close: move |_| boss_add_step.set(0),
                    }
                }
            }
        }

        // 添加野生宠物 Modal
        if show_add_pokemon_modal() {
            ActionModal {
                title: "添加野生宠物".to_string(),
                on_close: move |_| {
                    show_add_pokemon_modal.set(false);
                    add_pokemon_search_results.set(Vec::new());
                    add_pokemon_selected.set(None);
                },
                div { class: "admin-form-editor",
                    FormSection { title: "搜索并选择宠物".to_string(),
                        div { class: "admin-field",
                            label { class: "admin-field__label", "搜索宠物" }
                            SearchBox {
                                placeholder: "输入宠物名称搜索...".to_string(),
                                disabled: is_busy,
                                results: add_pokemon_search_results(),
                                loading: *add_pokemon_search_loading.read(),
                                on_search: move |query: String| {
                                    let query = query.trim().to_string();
                                    if query.is_empty() {
                                        add_pokemon_search_results.set(Vec::new());
                                        return;
                                    }
                                    add_pokemon_search_loading.set(true);
                                    let mut results_clone = add_pokemon_search_results.clone();
                                    let mut loading_clone = add_pokemon_search_loading.clone();
                                    spawn(async move {
                                        let filters = vec![
                                            FilterFieldState {
                                                tag: "名称".to_string(),
                                                operator: FilterConditionType::Text(LogicalOperation::Equal),
                                                value: query,
                                                enum_options: Vec::new(),
                                                enabled: true,
                                            },
                                        ];
                                        let result = filter_pokemon_type(
                                                filters.iter().filter_map(|f| f.to_package()).collect(),
                                            )
                                            .await;
                                        loading_clone.set(false);
                                        match result {
                                            Ok(types) => {
                                                let results: Vec<_> = types
                                                    .into_iter()
                                                    .map(|t| SearchResult {
                                                        id: t.id,
                                                        name: t.name.clone(),
                                                    })
                                                    .collect();
                                                results_clone.set(results);
                                            }
                                            Err(e) => {
                                                set_notice(AdminNoticeLevel::Error, format!("搜索失败: {}", e));
                                                results_clone.set(Vec::new());
                                            }
                                        }
                                    });
                                },
                                on_select: move |result: SearchResult| {
                                    add_pokemon_selected.set(Some(result.clone()));
                                    add_pokemon_search_results.set(Vec::new());
                                },
                                selected: add_pokemon_selected().clone(),
                                on_clear: move |_: ()| {
                                    add_pokemon_selected.set(None);
                                },
                            }
                        }
                    }

                    // 确认按钮
                    div { class: "admin-modal-actions",
                        button {
                            class: "admin-btn",
                            r#type: "button",
                            onclick: move |_| {
                                show_add_pokemon_modal.set(false);
                                add_pokemon_search_results.set(Vec::new());
                                add_pokemon_selected.set(None);
                            },
                            "取消"
                        }
                        button {
                            class: "admin-btn admin-btn--primary",
                            r#type: "button",
                            disabled: add_pokemon_selected().is_none(),
                            onclick: move |_| {
                                if let Some(s) = add_pokemon_selected() {
                                    spawn_add_pokemon_to_map(
                                        draft.read().id,
                                        s.id,
                                        s.name.clone(),
                                        wild_pokemons.clone(),
                                    );
                                }
                                show_add_pokemon_modal.set(false);
                                add_pokemon_search_results.set(Vec::new());
                                add_pokemon_selected.set(None);
                            },
                            "确认添加"
                        }
                    }
                }
            }
        }
    }
}

/// 两步走 Boss 添加 Modal（步骤 1：选择宠物；步骤 2：配置属性）
#[component]
fn AddBossModal(
    step: u8,
    draft: MapBoss,
    search_results: Vec<SearchResult>,
    search_loading: bool,
    on_search: EventHandler<String>,
    on_select_pokemon: EventHandler<SearchResult>,
    on_next: EventHandler<()>,
    on_update_draft: EventHandler<MapBoss>,
    on_back: EventHandler<()>,
    on_confirm: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    let mut local_draft = use_signal(|| draft.clone());
    // 步骤 1 中已选中的孠猥（未提交到步骤 2）
    let mut pending_selection = use_signal(|| None::<SearchResult>);

    // 当外部 draft 变化时同步（pokemon 被选择后，step 从 1->2，draft 更新了）
    use_effect(move || {
        *local_draft.write() = draft.clone();
    });

    // 将 u8 (1,2) 转换为 usize (0,1)
    let current_step = (step as usize).saturating_sub(1);

    rsx! {
        TwoStepWizardModal {
            title: "添加 Boss".to_string(),
            step: current_step,
            steps: vec!["选择宠物".to_string(), "配置属性".to_string()],
            can_proceed: pending_selection.read().is_some(),
            disabled: false,
            on_close: move |_| {
                on_close.call(());
            },
            on_confirm: move |_| {
                on_confirm.call(());
            },
            on_step_change: move |new_step| {
                // new_step 是 0 或 1，需要处理步骤转换
                if new_step == 0 {
                    on_back.call(()); // 回到步骤 1
                } else {
                    // 进入步骤 2，先更新 draft 再调用 next
                    if let Some(sel) = pending_selection.read().clone() {
                        local_draft.write().pokemon_type_id = sel.id;
                        local_draft.write().pokemon_name = sel.name.clone();
                        on_select_pokemon.call(sel);
                        on_next.call(());
                    }
                }
            },

            // 根据步骤显示不同内容
            if step == 1 {
                // 步骤 1：搜索并选择宠物
                FormSection { title: "选择宠物".to_string(),
                    Row { gap: "12px".to_string(),
                        Col { span: 12,
                            div { class: "admin-field",
                                label { class: "admin-field__label", "搜索宠物" }
                                SearchBox {
                                    placeholder: "输入宠物名称搜索...".to_string(),
                                    disabled: false,
                                    results: search_results.clone(),
                                    loading: search_loading,
                                    on_search: move |q| on_search.call(q),
                                    on_select: move |r: SearchResult| {
                                        pending_selection.set(Some(r));
                                    },
                                    selected: pending_selection.read().clone(),
                                    on_clear: move |_: ()| pending_selection.set(None),
                                }
                            }
                        }
                    }
                }
            } else {
                // 步骤 2：已选宠物只读展示
                div { class: "admin-grant-selected admin-grant-selected--readonly",
                    span { class: "admin-grant-selected__label", "宠物:" }
                    span { class: "admin-grant-selected__name",
                        "#{local_draft.read().pokemon_type_id} {local_draft.read().pokemon_name}"
                    }
                }

                // 步骤 2：配置 Boss 属性
                FormSection { title: "等级设置".to_string(),
                    Row {
                        Col { span: 12,
                            UnsignedNumberField {
                                label: "等级".to_string(),
                                value: local_draft.read().level,
                                min: Some(1),
                                max: None,
                                disabled: false,
                                on_change: move |v| {
                                    local_draft.write().level = v;
                                    on_update_draft.call(local_draft.read().clone());
                                },
                                help: Some("Boss 的固定等级（可超过 100）".to_string()),
                                step: None,
                            }
                        }
                    }
                }
                FormSection { title: "个体值".to_string(),
                    PokemonAttributesEditor {
                        value: local_draft.read().attributes,
                        disabled: false,
                        on_change: move |v| {
                            local_draft.write().attributes = v;
                            on_update_draft.call(local_draft.read().clone());
                        },
                    }
                }
                FormSection { title: "属性倍率".to_string(),
                    Row {
                        Col { span: 12,
                            FloatField {
                                label: "属性倍率".to_string(),
                                value: local_draft.read().boss_multiplier,
                                min: Some(1.0),
                                max: Some(5.0),
                                step: Some(0.1),
                                precision: Some(1),
                                disabled: false,
                                on_change: move |v| {
                                    local_draft.write().boss_multiplier = v;
                                    on_update_draft.call(local_draft.read().clone());
                                },
                                help: Some("Boss 属性倍率（1.0 = 无加成）".to_string()),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Boss 详情编辑 Modal
#[component]
fn BossEditModal(
    title: String,
    boss: MapBoss,
    on_save: EventHandler<MapBoss>,
    on_close: EventHandler<()>,
) -> Element {
    let mut draft = use_signal(|| boss.clone());
    let mut search_results = use_signal(Vec::<SearchResult>::new);
    let mut search_loading = use_signal(|| false);

    rsx! {
        ActionModal { title: title.clone(), on_close,
            div { class: "admin-form-editor admin-form-editor--compact",
                FormSection { title: "宠物选择".to_string(),
                    div { class: "admin-field",
                        label { class: "admin-field__label", "搜索宠物" }
                        SearchBox {
                            placeholder: "输入宠物名称搜索...".to_string(),
                            disabled: false,
                            results: search_results(),
                            loading: *search_loading.read(),
                            on_search: move |query: String| {
                                let query = query.trim().to_string();
                                if query.is_empty() {
                                    search_results.set(Vec::new());
                                    return;
                                }
                                search_loading.set(true);
                                let mut search_results_clone = search_results.clone();
                                let mut search_loading_clone = search_loading.clone();
                                spawn(async move {
                                    let filters = vec![
                                        FilterFieldState {
                                            tag: "名称".to_string(),
                                            operator: FilterConditionType::Text(LogicalOperation::Equal),
                                            value: query,
                                            enum_options: Vec::new(),
                                            enabled: true,
                                        },
                                    ];
                                    let result = filter_pokemon_type(
                                            filters.iter().filter_map(|f| f.to_package()).collect(),
                                        )
                                        .await;
                                    search_loading_clone.set(false);
                                    match result {
                                        Ok(types) => {
                                            let results: Vec<_> = types
                                                .into_iter()
                                                .map(|t| SearchResult {
                                                    id: t.id,
                                                    name: t.name.clone(),
                                                })
                                                .collect();
                                            search_results_clone.set(results);
                                        }
                                        Err(_) => {
                                            search_results_clone.set(Vec::new());
                                        }
                                    }
                                });
                            },
                            on_select: move |result: SearchResult| {
                                draft.write().pokemon_type_id = result.id;
                                draft.write().pokemon_name = result.name.clone();
                                search_results.set(Vec::new());
                            },
                            selected: {
                                let dr = draft.read();
                                if dr.pokemon_type_id > 0 {
                                    Some(SearchResult {
                                        id: dr.pokemon_type_id,
                                        name: dr.pokemon_name.clone(),
                                    })
                                } else {
                                    None
                                }
                            },
                            on_clear: move |_: ()| {
                                draft.write().pokemon_type_id = 0;
                                draft.write().pokemon_name = String::new();
                            },
                        }
                    }
                }

                FormSection { title: "等级设置".to_string(),
                    Row {
                        Col { span: 12,
                            UnsignedNumberField {
                                label: "等级".to_string(),
                                value: draft.read().level,
                                min: Some(1),
                                max: None,
                                disabled: false,
                                on_change: move |v| draft.write().level = v,
                                help: Some("Boss 的固定等级（可超过 100）".to_string()),
                                step: None,
                            }
                        }
                    }
                }

                FormSection { title: "个体值".to_string(),
                    PokemonAttributesEditor {
                        value: draft.read().attributes,
                        disabled: false,
                        on_change: move |v| draft.write().attributes = v,
                    }
                }

                FormSection { title: "属性倍率".to_string(),
                    Row {
                        Col { span: 12,
                            FloatField {
                                label: "属性倍率".to_string(),
                                value: draft.read().boss_multiplier,
                                min: Some(1.0),
                                max: Some(5.0),
                                step: Some(0.1),
                                precision: Some(1),
                                disabled: false,
                                on_change: move |v| draft.write().boss_multiplier = v,
                                help: Some("Boss 属性倍率（1.0 = 无加成）".to_string()),
                            }
                        }
                    }
                }

                // 操作按钮
                div { class: "admin-form-actions",
                    button {
                        class: "admin-btn",
                        r#type: "button",
                        onclick: move |_| on_close.call(()),
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn--primary",
                        r#type: "button",
                        onclick: move |_| on_save.call(draft.read().clone()),
                        "保存"
                    }
                }
            }
        }
    }
}
