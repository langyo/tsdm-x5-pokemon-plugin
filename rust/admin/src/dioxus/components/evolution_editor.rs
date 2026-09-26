use crate::dioxus::prelude::*;

use super::form_fields::{Col, FloatField, FormSection, Row, SelectField, UnsignedNumberField};
use crate::dioxus::{
    components::search_box::{IdSearchBox, SearchResult},
    pages::shared::ActionModal,
    state::{set_notice, AdminNoticeLevel, FilterConditionType, FilterFieldState},
    utils::api::{filter_item_type, get_pokemon_types_by_ids},
};
use _utils::types::{
    evolution_info::{
        CompareAttackAndDefenseVariant, EvolutionCompareType, EvolutionInfo, EvolutionLimitType,
        IsBagHaveChairsVariant, MinIntimacyVariant, MinLevelVariant, RandomVariant, SexVariant,
        UseItemVariant,
    },
    pokemon_type::PokemonSex,
};

/// 格式化源宠物标签
fn format_source_label(evolution: &EvolutionInfo) -> String {
    if evolution.source_name.is_empty() {
        "源宠物".to_string()
    } else {
        format!("源宠物 ({}#{})", evolution.source_name, evolution.source_id)
    }
}

/// 格式化目标宠物标签
fn format_target_label(evolution: &EvolutionInfo) -> String {
    if evolution.target_name.is_empty() {
        "目标宠物".to_string()
    } else {
        format!(
            "目标宠物 ({}#{})",
            evolution.target_name, evolution.target_id
        )
    }
}

/// 格式化物品标签
fn format_item_label(item_id: u64) -> String {
    if item_id == 0 {
        "物品".to_string()
    } else {
        format!("物品 ID #{}", item_id)
    }
}

/// 带选中显示的宠物搜索框（按 ID 搜索）
#[component]
fn PokemonSearchBox(
    label: String,
    placeholder: String,
    disabled: bool,
    selected_id: u64,
    selected_name: String,
    original_id: u64,
    on_clear: EventHandler<()>,
    on_select: EventHandler<SearchResult>,
) -> Element {
    let mut search_results = use_signal(Vec::<SearchResult>::new);
    let mut search_loading = use_signal(|| false);

    let has_selection = selected_id > 0;
    let is_modified = has_selection && selected_id != original_id;

    rsx! {
        div { class: "admin-field",
            label { class: "admin-field__label", "{label}" }
            // ID 搜索框 - 只接受数字输入
            IdSearchBox {
                placeholder,
                disabled,
                results: search_results(),
                loading: *search_loading.read(),
                on_search: move |id: u64| {
                    search_loading.set(true);
                    let mut search_results_clone = search_results.clone();
                    spawn(async move {
                        let result = get_pokemon_types_by_ids(vec![id]).await;
                        search_results_clone.write().clear();
                        match result {
                            Ok(types) => {
                                let results: Vec<_> = types.into_iter()
                                    .map(|t| SearchResult {
                                        id: t.id,
                                        name: t.name.clone(),
                                    })
                                    .collect();
                                *search_results_clone.write() = results;
                            }
                            Err(e) => {
                                set_notice(AdminNoticeLevel::Error, format!("搜索失败: {}", e));
                            }
                        }
                        search_loading.set(false);
                    });
                },
                on_select: move |result: SearchResult| {
                    on_select.call(result);
                    search_results.set(Vec::new());
                },
                selected: if has_selection {
                    Some(SearchResult {
                        id: selected_id,
                        name: selected_name.clone(),
                    })
                } else {
                    None
                },
                on_clear: Some(on_clear.clone()),
            }
            // 选中的宠物卡片显示在下方
            if has_selection {
                div {
                    class: if is_modified {
                        "admin-search-box__selected admin-search-box__selected--modified"
                    } else {
                        "admin-search-box__selected"
                    },
                    span { class: "admin-search-box__selected-name", "{selected_name}" }
                    span { class: "admin-search-box__selected-id", "#{selected_id}" }
                    if is_modified {
                        span { class: "admin-search-box__selected-badge", "已修改" }
                    }
                }
            }
        }
    }
}

/// 带选中显示的物品搜索框（按 ID 搜索）
#[component]
fn ItemSearchBox(
    label: String,
    placeholder: String,
    disabled: bool,
    selected_id: u64,
    selected_name: String,
    original_id: u64,
    on_clear: EventHandler<()>,
    on_select: EventHandler<SearchResult>,
) -> Element {
    let mut search_results = use_signal(Vec::<SearchResult>::new);
    let mut search_loading = use_signal(|| false);

    let has_selection = selected_id > 0;
    let is_modified = has_selection && selected_id != original_id;

    rsx! {
        div { class: "admin-field",
            label { class: "admin-field__label", "{label}" }
            // ID 搜索框 - 只接受数字输入
            IdSearchBox {
                placeholder,
                disabled,
                results: search_results(),
                loading: *search_loading.read(),
                on_search: move |id: u64| {
                    search_loading.set(true);
                    let mut search_results_clone = search_results.clone();
                    spawn(async move {
                        let filters = vec![FilterFieldState {
                            tag: "ID".to_string(),
                            operator: FilterConditionType::Id,
                            value: id.to_string(),
                            enum_options: Vec::new(),
                            enabled: true,
                        }];
                        let result = filter_item_type(
                            filters.iter().filter_map(|f| f.to_package()).collect()
                        ).await;
                        search_results_clone.write().clear();
                        match result {
                            Ok(items) => {
                                let results: Vec<_> = items.into_iter()
                                    .map(|t| SearchResult {
                                        id: t.id,
                                        name: t.name.clone(),
                                    })
                                    .collect();
                                *search_results_clone.write() = results;
                            }
                            Err(e) => {
                                set_notice(AdminNoticeLevel::Error, format!("搜索失败: {}", e));
                            }
                        }
                        search_loading.set(false);
                    });
                },
                on_select: move |result: SearchResult| {
                    on_select.call(result);
                    search_results.set(Vec::new());
                },
                selected: if has_selection {
                    Some(SearchResult {
                        id: selected_id,
                        name: selected_name.clone(),
                    })
                } else {
                    None
                },
                on_clear: Some(on_clear.clone()),
            }
            // 选中的物品卡片显示在下方
            if has_selection {
                div {
                    class: if is_modified {
                        "admin-search-box__selected admin-search-box__selected--modified"
                    } else {
                        "admin-search-box__selected"
                    },
                    span { class: "admin-search-box__selected-name", "{selected_name}" }
                    span { class: "admin-search-box__selected-id", "#{selected_id}" }
                    if is_modified {
                        span { class: "admin-search-box__selected-badge", "已修改" }
                    }
                }
            }
        }
    }
}

/// 进化路线精确编辑器 Modal
#[component]
pub fn EvolutionInfoEditorModal(
    title: String,
    data: EvolutionInfo,
    save_text: String,
    disabled: bool,
    on_save: EventHandler<EvolutionInfo>,
    on_close: EventHandler<()>,
) -> Element {
    let mut draft = use_signal(|| data.clone());
    let original_source_id = data.source_id;
    let original_target_id = data.target_id;
    let original_item_id = match &data.condition {
        EvolutionLimitType::UseItem(v) => v.use_item,
        _ => 0,
    };

    rsx! {
        ActionModal { title, on_close,
            div { class: "admin-form-editor admin-form-editor--compact",
                // 基本信息
                FormSection { title: "基本信息".to_string(),
                    Row {
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "ID".to_string(),
                                value: draft.read().id,
                                min: Some(0),
                                disabled: true,
                                on_change: move |_| {},
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "优先级".to_string(),
                                value: draft.read().priority,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().priority = v,
                                help: Some("数值越小优先级越高".to_string()),
                                step: None,
                                max: None,
                            }
                        }
                    }
                }

                // 进化关系
                FormSection { title: "进化关系".to_string(),
                    Row {
                        Col { span: 6,
                            PokemonSearchBox {
                                label: "源宠物".to_string(),
                                placeholder: "输入宠物ID搜索...".to_string(),
                                disabled,
                                selected_id: draft.read().source_id,
                                selected_name: draft.read().source_name.clone(),
                                original_id: original_source_id,
                                on_clear: move |_| {
                                    draft.write().source_id = 0;
                                    draft.write().source_name = String::new();
                                },
                                on_select: move |result: SearchResult| {
                                    draft.write().source_id = result.id;
                                    draft.write().source_name = result.name;
                                },
                            }
                        }
                        Col { span: 6,
                            PokemonSearchBox {
                                label: "目标宠物".to_string(),
                                placeholder: "输入宠物ID搜索...".to_string(),
                                disabled,
                                selected_id: draft.read().target_id,
                                selected_name: draft.read().target_name.clone(),
                                original_id: original_target_id,
                                on_clear: move |_| {
                                    draft.write().target_id = 0;
                                    draft.write().target_name = String::new();
                                },
                                on_select: move |result: SearchResult| {
                                    draft.write().target_id = result.id;
                                    draft.write().target_name = result.name;
                                },
                            }
                        }
                    }
                }

                // 进化条件
                FormSection { title: "进化条件".to_string(),
                    EvolutionConditionEditor {
                        value: draft.read().condition.clone(),
                        original_item_id: original_item_id,
                        disabled,
                        on_change: move |v| draft.write().condition = v,
                    }
                }

                // 操作按钮
                div { class: "admin-form-actions",
                    button {
                        class: "admin-btn",
                        disabled,
                        onclick: move |_| on_close.call(()),
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn--primary",
                        disabled,
                        onclick: move |_| on_save.call(draft.read().clone()),
                        "{save_text}"
                    }
                }
            }
        }
    }
}

/// 进化条件编辑器
#[component]
fn EvolutionConditionEditor(
    value: EvolutionLimitType,
    original_item_id: u64,
    disabled: bool,
    on_change: EventHandler<EvolutionLimitType>,
) -> Element {
    let condition_type = match &value {
        EvolutionLimitType::MinLevel(_) => "min_level",
        EvolutionLimitType::UseItem(_) => "use_item",
        EvolutionLimitType::MinIntimacy(_) => "min_intimacy",
        EvolutionLimitType::CompareAttackAndDefense(_) => "compare_atk_def",
        EvolutionLimitType::Random(_) => "random",
        EvolutionLimitType::Sex(_) => "sex",
        EvolutionLimitType::IsBagHaveChairs(_) => "bag_space",
    };

    let condition_options: Vec<(String, String)> = vec![
        ("min_level".to_string(), "最低等级".to_string()),
        ("use_item".to_string(), "使用物品".to_string()),
        ("min_intimacy".to_string(), "最低好感度".to_string()),
        (
            "compare_atk_def".to_string(),
            "攻击力与防御力比较".to_string(),
        ),
        ("random".to_string(), "随机".to_string()),
        ("sex".to_string(), "性别".to_string()),
        ("bag_space".to_string(), "背包有空位".to_string()),
    ];

    rsx! {
        div { class: "admin-form-condition-editor",
            // 根据条件类型显示参数（两列一行）
            match &value {
                EvolutionLimitType::MinLevel(v) => rsx! {
                    Row {
                        Col { span: 6,
                            SelectField {
                                label: "条件类型".to_string(),
                                value: condition_type.to_string(),
                                options: condition_options.clone(),
                                disabled,
                                on_change: move |v: String| {
                                    let new_condition = match v.as_str() {
                                        "min_level" => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                        "use_item" => EvolutionLimitType::UseItem(UseItemVariant { use_item: 0, item_name: String::new() }),
                                        "min_intimacy" => EvolutionLimitType::MinIntimacy(MinIntimacyVariant { min_intimacy: 0 }),
                                        "compare_atk_def" => EvolutionLimitType::CompareAttackAndDefense(CompareAttackAndDefenseVariant { compare_attack_and_defense: EvolutionCompareType::Equal }),
                                        "random" => EvolutionLimitType::Random(RandomVariant { random: 0.0 }),
                                        "sex" => EvolutionLimitType::Sex(SexVariant { sex: PokemonSex::Unknown }),
                                        "bag_space" => EvolutionLimitType::IsBagHaveChairs(IsBagHaveChairsVariant { is_bag_have_chairs: true }),
                                        _ => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                    };
                                    on_change.call(new_condition);
                                },
                                help: None,
                            }
                        }
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "最低等级".to_string(),
                                value: v.min_level,
                                min: Some(0),
                                disabled,
                                on_change: move |v| on_change.call(EvolutionLimitType::MinLevel(MinLevelVariant { min_level: v })),
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                    }
                },
                EvolutionLimitType::UseItem(v) => rsx! {
                    Row {
                        Col { span: 6,
                            SelectField {
                                label: "条件类型".to_string(),
                                value: condition_type.to_string(),
                                options: condition_options.clone(),
                                disabled,
                                on_change: move |v: String| {
                                    let new_condition = match v.as_str() {
                                        "min_level" => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                        "use_item" => EvolutionLimitType::UseItem(UseItemVariant { use_item: 0, item_name: String::new() }),
                                        "min_intimacy" => EvolutionLimitType::MinIntimacy(MinIntimacyVariant { min_intimacy: 0 }),
                                        "compare_atk_def" => EvolutionLimitType::CompareAttackAndDefense(CompareAttackAndDefenseVariant { compare_attack_and_defense: EvolutionCompareType::Equal }),
                                        "random" => EvolutionLimitType::Random(RandomVariant { random: 0.0 }),
                                        "sex" => EvolutionLimitType::Sex(SexVariant { sex: PokemonSex::Unknown }),
                                        "bag_space" => EvolutionLimitType::IsBagHaveChairs(IsBagHaveChairsVariant { is_bag_have_chairs: true }),
                                        _ => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                    };
                                    on_change.call(new_condition);
                                },
                                help: None,
                            }
                        }
                        Col { span: 6,
                            ItemSearchBox {
                                label: "物品".to_string(),
                                placeholder: "输入物品ID搜索...".to_string(),
                                disabled,
                                selected_id: v.use_item,
                                selected_name: v.item_name.clone(),
                                original_id: original_item_id,
                                on_clear: move |_| {
                                    on_change.call(EvolutionLimitType::UseItem(UseItemVariant { use_item: 0, item_name: String::new() }));
                                },
                                on_select: move |result: SearchResult| {
                                    on_change.call(EvolutionLimitType::UseItem(UseItemVariant { use_item: result.id, item_name: result.name }));
                                },
                            }
                        }
                    }
                },
                EvolutionLimitType::MinIntimacy(v) => rsx! {
                    Row {
                        Col { span: 6,
                            SelectField {
                                label: "条件类型".to_string(),
                                value: condition_type.to_string(),
                                options: condition_options.clone(),
                                disabled,
                                on_change: move |v: String| {
                                    let new_condition = match v.as_str() {
                                        "min_level" => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                        "use_item" => EvolutionLimitType::UseItem(UseItemVariant { use_item: 0, item_name: String::new() }),
                                        "min_intimacy" => EvolutionLimitType::MinIntimacy(MinIntimacyVariant { min_intimacy: 0 }),
                                        "compare_atk_def" => EvolutionLimitType::CompareAttackAndDefense(CompareAttackAndDefenseVariant { compare_attack_and_defense: EvolutionCompareType::Equal }),
                                        "random" => EvolutionLimitType::Random(RandomVariant { random: 0.0 }),
                                        "sex" => EvolutionLimitType::Sex(SexVariant { sex: PokemonSex::Unknown }),
                                        "bag_space" => EvolutionLimitType::IsBagHaveChairs(IsBagHaveChairsVariant { is_bag_have_chairs: true }),
                                        _ => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                    };
                                    on_change.call(new_condition);
                                },
                                help: None,
                            }
                        }
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "最低好感度".to_string(),
                                value: v.min_intimacy,
                                min: Some(0),
                                disabled,
                                on_change: move |v| on_change.call(EvolutionLimitType::MinIntimacy(MinIntimacyVariant { min_intimacy: v })),
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                    }
                },
                EvolutionLimitType::CompareAttackAndDefense(v) => rsx! {
                    Row {
                        Col { span: 6,
                            SelectField {
                                label: "条件类型".to_string(),
                                value: condition_type.to_string(),
                                options: condition_options.clone(),
                                disabled,
                                on_change: move |v: String| {
                                    let new_condition = match v.as_str() {
                                        "min_level" => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                        "use_item" => EvolutionLimitType::UseItem(UseItemVariant { use_item: 0, item_name: String::new() }),
                                        "min_intimacy" => EvolutionLimitType::MinIntimacy(MinIntimacyVariant { min_intimacy: 0 }),
                                        "compare_atk_def" => EvolutionLimitType::CompareAttackAndDefense(CompareAttackAndDefenseVariant { compare_attack_and_defense: EvolutionCompareType::Equal }),
                                        "random" => EvolutionLimitType::Random(RandomVariant { random: 0.0 }),
                                        "sex" => EvolutionLimitType::Sex(SexVariant { sex: PokemonSex::Unknown }),
                                        "bag_space" => EvolutionLimitType::IsBagHaveChairs(IsBagHaveChairsVariant { is_bag_have_chairs: true }),
                                        _ => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                    };
                                    on_change.call(new_condition);
                                },
                                help: None,
                            }
                        }
                        Col { span: 6,
                            CompareTypeSelect {
                                value: v.compare_attack_and_defense,
                                disabled,
                                on_change: move |v| on_change.call(EvolutionLimitType::CompareAttackAndDefense(CompareAttackAndDefenseVariant { compare_attack_and_defense: v })),
                            }
                        }
                    }
                },
                EvolutionLimitType::Random(v) => rsx! {
                    Row {
                        Col { span: 6,
                            SelectField {
                                label: "条件类型".to_string(),
                                value: condition_type.to_string(),
                                options: condition_options.clone(),
                                disabled,
                                on_change: move |v: String| {
                                    let new_condition = match v.as_str() {
                                        "min_level" => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                        "use_item" => EvolutionLimitType::UseItem(UseItemVariant { use_item: 0, item_name: String::new() }),
                                        "min_intimacy" => EvolutionLimitType::MinIntimacy(MinIntimacyVariant { min_intimacy: 0 }),
                                        "compare_atk_def" => EvolutionLimitType::CompareAttackAndDefense(CompareAttackAndDefenseVariant { compare_attack_and_defense: EvolutionCompareType::Equal }),
                                        "random" => EvolutionLimitType::Random(RandomVariant { random: 0.0 }),
                                        "sex" => EvolutionLimitType::Sex(SexVariant { sex: PokemonSex::Unknown }),
                                        "bag_space" => EvolutionLimitType::IsBagHaveChairs(IsBagHaveChairsVariant { is_bag_have_chairs: true }),
                                        _ => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                    };
                                    on_change.call(new_condition);
                                },
                                help: None,
                            }
                        }
                        Col { span: 6,
                            FloatField {
                                label: "进化概率".to_string(),
                                value: v.random,
                                min: Some(0.0),
                                max: Some(1.0),
                                step: Some(0.01),
                                precision: Some(2),
                                disabled,
                                on_change: move |v| on_change.call(EvolutionLimitType::Random(RandomVariant { random: v })),
                                help: Some("0.0 - 1.0".to_string()),
                            }
                        }
                    }
                },
                EvolutionLimitType::Sex(v) => rsx! {
                    Row {
                        Col { span: 6,
                            SelectField {
                                label: "条件类型".to_string(),
                                value: condition_type.to_string(),
                                options: condition_options.clone(),
                                disabled,
                                on_change: move |v: String| {
                                    let new_condition = match v.as_str() {
                                        "min_level" => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                        "use_item" => EvolutionLimitType::UseItem(UseItemVariant { use_item: 0, item_name: String::new() }),
                                        "min_intimacy" => EvolutionLimitType::MinIntimacy(MinIntimacyVariant { min_intimacy: 0 }),
                                        "compare_atk_def" => EvolutionLimitType::CompareAttackAndDefense(CompareAttackAndDefenseVariant { compare_attack_and_defense: EvolutionCompareType::Equal }),
                                        "random" => EvolutionLimitType::Random(RandomVariant { random: 0.0 }),
                                        "sex" => EvolutionLimitType::Sex(SexVariant { sex: PokemonSex::Unknown }),
                                        "bag_space" => EvolutionLimitType::IsBagHaveChairs(IsBagHaveChairsVariant { is_bag_have_chairs: true }),
                                        _ => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                    };
                                    on_change.call(new_condition);
                                },
                                help: None,
                            }
                        }
                        Col { span: 6,
                            SexSelect {
                                value: v.sex,
                                disabled,
                                on_change: move |v| on_change.call(EvolutionLimitType::Sex(SexVariant { sex: v })),
                            }
                        }
                    }
                },
                EvolutionLimitType::IsBagHaveChairs(v) => rsx! {
                    Row {
                        Col { span: 6,
                            SelectField {
                                label: "条件类型".to_string(),
                                value: condition_type.to_string(),
                                options: condition_options.clone(),
                                disabled,
                                on_change: move |v: String| {
                                    let new_condition = match v.as_str() {
                                        "min_level" => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                        "use_item" => EvolutionLimitType::UseItem(UseItemVariant { use_item: 0, item_name: String::new() }),
                                        "min_intimacy" => EvolutionLimitType::MinIntimacy(MinIntimacyVariant { min_intimacy: 0 }),
                                        "compare_atk_def" => EvolutionLimitType::CompareAttackAndDefense(CompareAttackAndDefenseVariant { compare_attack_and_defense: EvolutionCompareType::Equal }),
                                        "random" => EvolutionLimitType::Random(RandomVariant { random: 0.0 }),
                                        "sex" => EvolutionLimitType::Sex(SexVariant { sex: PokemonSex::Unknown }),
                                        "bag_space" => EvolutionLimitType::IsBagHaveChairs(IsBagHaveChairsVariant { is_bag_have_chairs: true }),
                                        _ => EvolutionLimitType::MinLevel(MinLevelVariant { min_level: 0 }),
                                    };
                                    on_change.call(new_condition);
                                },
                                help: None,
                            }
                        }
                        Col { span: 6,
                            SelectField {
                                label: "背包要求".to_string(),
                                value: if v.is_bag_have_chairs { "true".to_string() } else { "false".to_string() },
                                options: vec![
                                    ("true".to_string(), "需要空位".to_string()),
                                    ("false".to_string(), "无需空位".to_string()),
                                ],
                                disabled,
                                on_change: move |v: String| {
                                    on_change.call(EvolutionLimitType::IsBagHaveChairs(IsBagHaveChairsVariant { is_bag_have_chairs: v == "true" }));
                                },
                                help: None,
                            }
                        }
                    }
                },
            }
        }
    }
}

/// 比较类型选择器
#[component]
fn CompareTypeSelect(
    value: EvolutionCompareType,
    disabled: bool,
    on_change: EventHandler<EvolutionCompareType>,
) -> Element {
    let options: Vec<(String, String)> = vec![
        ("equal".to_string(), "攻击力 = 防御力".to_string()),
        ("greater".to_string(), "攻击力 > 防御力".to_string()),
        ("less".to_string(), "攻击力 < 防御力".to_string()),
    ];

    let current = match value {
        EvolutionCompareType::Equal => "equal",
        EvolutionCompareType::Greater => "greater",
        EvolutionCompareType::Less => "less",
    };

    rsx! {
        SelectField {
            label: "比较类型".to_string(),
            value: current.to_string(),
            options,
            disabled,
            on_change: move |v: String| {
                let new_type = match v.as_str() {
                    "equal" => EvolutionCompareType::Equal,
                    "greater" => EvolutionCompareType::Greater,
                    "less" => EvolutionCompareType::Less,
                    _ => EvolutionCompareType::Equal,
                };
                on_change.call(new_type);
            },
            help: None,
        }
    }
}

/// 性别选择器
#[component]
fn SexSelect(value: PokemonSex, disabled: bool, on_change: EventHandler<PokemonSex>) -> Element {
    let options: Vec<(String, String)> = vec![
        ("male".to_string(), "雄性".to_string()),
        ("female".to_string(), "雌性".to_string()),
        ("unknown".to_string(), "未知性别".to_string()),
    ];

    rsx! {
        SelectField {
            label: "性别要求".to_string(),
            value: match value {
                PokemonSex::Male => "male".to_string(),
                PokemonSex::Female => "female".to_string(),
                PokemonSex::Unknown => "unknown".to_string(),
            },
            options,
            disabled,
            on_change: move |v: String| {
                let parsed = match v.as_str() {
                    "male" => Ok(PokemonSex::Male),
                    "female" => Ok(PokemonSex::Female),
                    "unknown" => Ok(PokemonSex::Unknown),
                    _ => Err(()),
                };
                if let Ok(parsed) = parsed {
                    on_change.call(parsed);
                }
            },
            help: None,
        }
    }
}
