use strum::IntoEnumIterator;

use crate::dioxus::prelude::*;

use super::form_fields::{
    Col, FieldWrapper, FormSection, NumberField, Row, SelectField, TextAreaField, TextField,
    UnsignedNumberField,
};
use crate::dioxus::{
    pages::shared::ActionModal,
    {
        components::search_box::{SearchBox, SearchResult},
        components::tag::Tag,
        state::{
            set_busy, set_notice, AdminNoticeLevel, FilterConditionType, FilterFieldState,
            LogicalOperation, ADMIN_BUSY,
        },
        utils::api::{filter_pokemon_type, get_pokemon_types_by_ids},
    },
};
use _utils::types::{
    pokemon_type::PokemonKind,
    skill_type::{SkillEffect, SkillType, StatType, StatusEffect, Target},
};

/// 可用宠物信息（简化版）
#[derive(Clone, Debug, PartialEq)]
struct AvailablePokemonInfo {
    id: u64,
    name: String,
}

/// 技能效果类型选择器（内置静态选项，避免生命周期问题）
#[component]
fn EffectTypeSelectField(
    value: &'static str,
    disabled: bool,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        FieldWrapper { label: "效果类型".to_string(), help: None,
            select {
                class: "admin-input",
                disabled,
                value: "{value}",
                onchange: move |evt| on_change.call(evt.value()),
                option {
                    value: "physical_damage",
                    selected: value == "physical_damage",
                    "物理伤害"
                }
                option {
                    value: "special_damage",
                    selected: value == "special_damage",
                    "特殊伤害"
                }
                option { value: "stat_boost", selected: value == "stat_boost", "能力变化" }
                option {
                    value: "inflict_status",
                    selected: value == "inflict_status",
                    "状态效果"
                }
                option { value: "heal", selected: value == "heal", "治疗" }
                option { value: "priority", selected: value == "priority", "先制/后制" }
                option { value: "recoil", selected: value == "recoil", "反伤" }
                option { value: "one_hit_ko", selected: value == "one_hit_ko", "一击必杀" }
                option { value: "fixed_damage", selected: value == "fixed_damage", "固定伤害" }
                option { value: "others", selected: value == "others", "其他" }
            }
        }
    }
}

/// 获取所有 PokemonKind 选项
fn pokemon_kind_options() -> Vec<(String, String)> {
    PokemonKind::iter()
        .map(|k| (k.to_string(), k.to_string()))
        .collect()
}

/// 获取所有 StatType 选项
fn stat_type_options() -> Vec<(String, String)> {
    vec![
        ("attack".to_string(), "攻击".to_string()),
        ("defense".to_string(), "防御".to_string()),
        ("special_attack".to_string(), "特攻".to_string()),
        ("special_defense".to_string(), "特防".to_string()),
        ("speed".to_string(), "速度".to_string()),
    ]
}

/// 获取所有 StatusEffect 选项
fn status_effect_options() -> Vec<(String, String)> {
    vec![
        ("burn".to_string(), "烧伤".to_string()),
        ("freeze".to_string(), "冰冻".to_string()),
        ("paralysis".to_string(), "麻痹".to_string()),
        ("poison".to_string(), "中毒".to_string()),
        ("sleep".to_string(), "睡眠".to_string()),
        ("confusion".to_string(), "混乱".to_string()),
    ]
}

/// 获取所有 Target 选项
fn target_options() -> Vec<(String, String)> {
    vec![
        ("myself".to_string(), "自己".to_string()),
        ("opponent".to_string(), "对手".to_string()),
    ]
}

/// 添加宠物到技能的可用列表
fn spawn_add_pokemon_to_skill(
    mut draft: Signal<SkillType>,
    pokemon_type_id: u64,
    pokemon_name: String,
    mut available_pokemons: Signal<Vec<AvailablePokemonInfo>>,
) {
    set_busy(true);
    spawn(async move {
        // 检查是否已存在
        let current_ids = draft.read().available_pokemons.clone();
        if current_ids.contains(&pokemon_type_id) {
            set_notice(AdminNoticeLevel::Error, "该宠物已在列表中".to_string());
            set_busy(false);
            return;
        }

        // 添加到 draft
        let mut updated = draft.read().clone();
        updated.available_pokemons.push(pokemon_type_id);
        // 排序并去重
        updated.available_pokemons.sort();
        updated.available_pokemons.dedup();
        draft.set(updated);

        // 添加到显示列表
        let mut updated_list = available_pokemons.read().clone();
        updated_list.push(AvailablePokemonInfo {
            id: pokemon_type_id,
            name: pokemon_name,
        });
        updated_list.sort_by(|a, b| a.id.cmp(&b.id));
        available_pokemons.set(updated_list);

        set_notice(AdminNoticeLevel::Success, "已添加宠物到技能".to_string());
        set_busy(false);
    });
}

/// 从技能的可用列表中移除宠物
fn spawn_remove_pokemon_from_skill(
    mut draft: Signal<SkillType>,
    pokemon_id: u64,
    mut available_pokemons: Signal<Vec<AvailablePokemonInfo>>,
) {
    set_busy(true);
    spawn(async move {
        // 从 draft 中移除
        let mut updated = draft.read().clone();
        updated.available_pokemons.retain(|&id| id != pokemon_id);
        // 重新排序
        updated.available_pokemons.sort();
        draft.set(updated);

        // 从显示列表中移除
        let mut updated_list = available_pokemons.read().clone();
        updated_list.retain(|p| p.id != pokemon_id);
        updated_list.sort_by(|a, b| a.id.cmp(&b.id));
        available_pokemons.set(updated_list);

        set_notice(AdminNoticeLevel::Success, "已移除宠物".to_string());
        set_busy(false);
    });
}

/// 技能类型精确编辑器 Modal
#[component]
pub fn SkillTypeEditorModal(
    title: String,
    data: SkillType,
    save_text: String,
    disabled: bool,
    on_save: EventHandler<SkillType>,
    on_close: EventHandler<()>,
) -> Element {
    let is_busy = *ADMIN_BUSY.read();
    let mut draft = use_signal(|| data.clone());
    let available_pokemons = use_signal(Vec::<AvailablePokemonInfo>::new);
    let mut available_pokemons_loaded = use_signal(|| false);
    let mut search_results = use_signal(Vec::<SearchResult>::new);
    let mut search_loading = use_signal(|| false);

    // 加载可用宠物列表
    use_effect(move || {
        let current_pokemon_ids = draft.read().available_pokemons.clone();
        let is_loaded = *available_pokemons_loaded.read();
        if current_pokemon_ids.is_empty() || is_loaded {
            return;
        }
        available_pokemons_loaded.set(true);
        let mut available_pokemons_clone = available_pokemons.clone();
        let mut available_pokemons_loaded_clone = available_pokemons_loaded.clone();
        spawn(async move {
            match get_pokemon_types_by_ids(current_pokemon_ids.clone()).await {
                Ok(pokemon_infos) => {
                    let pokemon_list: Vec<_> = pokemon_infos
                        .into_iter()
                        .map(|info| AvailablePokemonInfo {
                            id: info.id,
                            name: info.name,
                        })
                        .collect();
                    available_pokemons_clone.set(pokemon_list);
                }
                Err(e) => {
                    set_notice(AdminNoticeLevel::Error, format!("加载宠物列表失败: {}", e));
                    available_pokemons_clone.set(Vec::new());
                    // 加载失败，重置加载标志以便重试
                    available_pokemons_loaded_clone.set(false);
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
                                on_change: move |_| {},
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 8,
                            TextField {
                                label: "名称".to_string(),
                                value: draft.read().name.clone(),
                                placeholder: Some("技能名称".to_string()),
                                disabled,
                                on_change: move |v| draft.write().name = v,
                                help: None,
                            }
                        }
                    }
                    TextAreaField {
                        label: "描述".to_string(),
                        value: draft.read().description.clone(),
                        placeholder: Some("技能描述".to_string()),
                        rows: Some(2),
                        disabled,
                        on_change: move |v| draft.write().description = v,
                        help: None,
                    }
                }

                // 使用限制
                FormSection { title: "使用限制".to_string(),
                    Row {
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "最低等级".to_string(),
                                value: draft.read().min_level_limit,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().min_level_limit = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "充能次数".to_string(),
                                value: draft.read().use_times_limit,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().use_times_limit = v,
                                help: Some("0 表示无限使用".to_string()),
                                step: None,
                                max: None,
                            }
                        }
                    }
                }

                // 可用宠物
                FormSection { title: "可用宠物".to_string(),
                    // 加载状态
                    if !*available_pokemons_loaded.read() && !draft.read().available_pokemons.is_empty() {
                        div { class: "admin-loading",
                            div { class: "admin-loading__spinner" }
                        }
                    } else {
                        // 可用宠物列表
                        if available_pokemons.read().is_empty() {
                            if draft.read().available_pokemons.is_empty() {
                                div { class: "admin-empty-list",
                                    div { class: "admin-empty-list__content",
                                        "留空表示所有宠物都可使用"
                                    }
                                }
                            } else {
                                div { class: "admin-empty-list",
                                    div { class: "admin-empty-list__content",
                                        "正在加载宠物列表..."
                                    }
                                }
                            }
                        } else {
                            div { class: "admin-wild-pokemon-tags",
                                for pokemon in available_pokemons.read().iter() {
                                    Tag {
                                        label: format!("{} #{}", pokemon.name, pokemon.id),
                                        disabled: is_busy,
                                        on_close: {
                                            let pokemon_id = pokemon.id;
                                            let draft_clone = draft.clone();
                                            let available_pokemons_clone = available_pokemons.clone();
                                            move |_| {
                                                spawn_remove_pokemon_from_skill(
                                                    draft_clone.clone(),
                                                    pokemon_id,
                                                    available_pokemons_clone.clone(),
                                                );
                                            }
                                        },
                                        class: Some("admin-tag--closable".to_string()),
                                    }
                                }
                            }
                        }

                        // 搜索添加宠物
                        div { class: "admin-field",
                            label { class: "admin-field__label", "搜索并添加宠物" }
                            SearchBox {
                                placeholder: "输入宠物名称搜索...".to_string(),
                                disabled: is_busy || disabled,
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
                                            Err(e) => {
                                                set_notice(AdminNoticeLevel::Error, format!("搜索失败: {}", e));
                                                search_results_clone.set(Vec::new());
                                            }
                                        }
                                    });
                                },
                                on_select: move |result: SearchResult| {
                                    spawn_add_pokemon_to_skill(
                                        draft.clone(),
                                        result.id,
                                        result.name,
                                        available_pokemons.clone(),
                                    );
                                },
                            }
                        }
                    }
                }

                // 技能效果
                FormSection { title: "技能效果".to_string(),
                    SkillEffectEditor {
                        value: draft.read().effect,
                        disabled,
                        on_change: move |v| draft.write().effect = v,
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

/// 技能效果编辑器
#[component]
fn SkillEffectEditor(
    value: SkillEffect,
    disabled: bool,
    on_change: EventHandler<SkillEffect>,
) -> Element {
    rsx! {
        div { class: "admin-form-effect-editor",
            // 使用条件渲染而不是 match，避免临时值生命周期问题
            if matches!(&value, SkillEffect::PhysicalDamage { .. }) {
                PhysicalDamageEditor { value: value.clone(), disabled, on_change }
            }
            if matches!(&value, SkillEffect::SpecialDamage { .. }) {
                SpecialDamageEditor { value: value.clone(), disabled, on_change }
            }
            if matches!(&value, SkillEffect::StatBoost { .. }) {
                StatBoostEditor { value: value.clone(), disabled, on_change }
            }
            if matches!(&value, SkillEffect::InflictStatus { .. }) {
                InflictStatusEditor { value: value.clone(), disabled, on_change }
            }
            if matches!(&value, SkillEffect::Heal { .. }) {
                HealEditor { value: value.clone(), disabled, on_change }
            }
            if matches!(&value, SkillEffect::Priority { .. }) {
                PriorityEditor { value: value.clone(), disabled, on_change }
            }
            if matches!(&value, SkillEffect::Recoil { .. }) {
                RecoilEditor { value: value.clone(), disabled, on_change }
            }
            if matches!(&value, SkillEffect::OneHitKO { .. }) {
                OneHitKOEditor { value: value.clone(), disabled, on_change }
            }
            if matches!(&value, SkillEffect::FixedDamage { .. }) {
                FixedDamageEditor { value: value.clone(), disabled, on_change }
            }
            if matches!(&value, SkillEffect::Others { .. }) {
                OthersEditor { value: value.clone(), disabled, on_change }
            }
        }
    }
}

/// 物理伤害编辑器
#[component]
fn PhysicalDamageEditor(
    value: SkillEffect,
    disabled: bool,
    on_change: EventHandler<SkillEffect>,
) -> Element {
    let (kind, power) = match value {
        SkillEffect::PhysicalDamage { kind, power } => (kind, power),
        _ => unreachable!(),
    };

    rsx! {
        Row {
            Col { span: 6,
                EffectTypeSelectField {
                    value: "physical_damage",
                    disabled,
                    on_change: move |v: String| {
                        if let Ok(new_effect) = create_skill_effect(&v) {
                            on_change.call(new_effect);
                        }
                    },
                }
            }
            Col { span: 6,
                KindSelectField {
                    label: "属性",
                    value: kind,
                    disabled,
                    on_change: move |k| {
                        on_change
                            .call(SkillEffect::PhysicalDamage {
                                kind: k,
                                power,
                            })
                    },
                }
            }
        }
        Row {
            Col { span: 6,
                UnsignedNumberField {
                    label: "威力".to_string(),
                    value: power,
                    min: Some(0),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::PhysicalDamage {
                                kind,
                                power: v,
                            })
                    },
                    help: None,
                    step: None,
                    max: None,
                }
            }
        }
    }
}

/// 特殊伤害编辑器
#[component]
fn SpecialDamageEditor(
    value: SkillEffect,
    disabled: bool,
    on_change: EventHandler<SkillEffect>,
) -> Element {
    let (kind, power) = match value {
        SkillEffect::SpecialDamage { kind, power } => (kind, power),
        _ => unreachable!(),
    };

    rsx! {
        Row {
            Col { span: 6,
                EffectTypeSelectField {
                    value: "special_damage",
                    disabled,
                    on_change: move |v: String| {
                        if let Ok(new_effect) = create_skill_effect(&v) {
                            on_change.call(new_effect);
                        }
                    },
                }
            }
            Col { span: 6,
                KindSelectField {
                    label: "属性",
                    value: kind,
                    disabled,
                    on_change: move |k| {
                        on_change
                            .call(SkillEffect::SpecialDamage {
                                kind: k,
                                power,
                            })
                    },
                }
            }
        }
        Row {
            Col { span: 6,
                UnsignedNumberField {
                    label: "威力".to_string(),
                    value: power,
                    min: Some(0),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::SpecialDamage {
                                kind,
                                power: v,
                            })
                    },
                    help: None,
                    step: None,
                    max: None,
                }
            }
        }
    }
}

/// 能力变化编辑器
#[component]
fn StatBoostEditor(
    value: SkillEffect,
    disabled: bool,
    on_change: EventHandler<SkillEffect>,
) -> Element {
    let (stat, stages, target) = match value {
        SkillEffect::StatBoost {
            stat,
            stages,
            target,
        } => (stat, stages, target),
        _ => unreachable!(),
    };

    rsx! {
        Row {
            Col { span: 6,
                EffectTypeSelectField {
                    value: "stat_boost",
                    disabled,
                    on_change: move |v: String| {
                        if let Ok(new_effect) = create_skill_effect(&v) {
                            on_change.call(new_effect);
                        }
                    },
                }
            }
            Col { span: 6,
                StatTypeSelect {
                    label: "能力",
                    value: stat,
                    disabled,
                    on_change: move |s| {
                        on_change
                            .call(SkillEffect::StatBoost {
                                stat: s,
                                stages,
                                target,
                            })
                    },
                }
            }
        }
        Row {
            Col { span: 6,
                NumberField {
                    label: "变化等级".to_string(),
                    value: stages as i64,
                    min: Some(-6),
                    max: Some(6),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::StatBoost {
                                stat,
                                stages: v as i8,
                                target,
                            })
                    },
                    help: Some("-6 到 +6".to_string()),
                    step: None,
                }
            }
            Col { span: 6,
                TargetSelect {
                    label: "目标",
                    value: target,
                    disabled,
                    on_change: move |t| {
                        on_change
                            .call(SkillEffect::StatBoost {
                                stat,
                                stages,
                                target: t,
                            })
                    },
                }
            }
        }
    }
}

/// 状态效果编辑器
#[component]
fn InflictStatusEditor(
    value: SkillEffect,
    disabled: bool,
    on_change: EventHandler<SkillEffect>,
) -> Element {
    let (effect, chance) = match value {
        SkillEffect::InflictStatus { effect, chance } => (effect, chance),
        _ => unreachable!(),
    };

    rsx! {
        Row {
            Col { span: 6,
                EffectTypeSelectField {
                    value: "inflict_status",
                    disabled,
                    on_change: move |v: String| {
                        if let Ok(new_effect) = create_skill_effect(&v) {
                            on_change.call(new_effect);
                        }
                    },
                }
            }
            Col { span: 6,
                StatusEffectSelect {
                    label: "状态类型",
                    value: effect,
                    disabled,
                    on_change: move |e| {
                        on_change
                            .call(SkillEffect::InflictStatus {
                                effect: e,
                                chance,
                            })
                    },
                }
            }
        }
        Row {
            Col { span: 6,
                UnsignedNumberField {
                    label: "触发概率 (%)".to_string(),
                    value: chance as u64,
                    min: Some(0),
                    max: Some(100),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::InflictStatus {
                                effect,
                                chance: v as u8,
                            })
                    },
                    help: None,
                    step: None,
                }
            }
        }
    }
}

/// 治疗编辑器
#[component]
fn HealEditor(value: SkillEffect, disabled: bool, on_change: EventHandler<SkillEffect>) -> Element {
    let percent = match value {
        SkillEffect::Heal { percent } => percent,
        _ => unreachable!(),
    };

    rsx! {
        Row {
            Col { span: 6,
                EffectTypeSelectField {
                    value: "heal",
                    disabled,
                    on_change: move |v: String| {
                        if let Ok(new_effect) = create_skill_effect(&v) {
                            on_change.call(new_effect);
                        }
                    },
                }
            }
            Col { span: 6,
                UnsignedNumberField {
                    label: "恢复百分比".to_string(),
                    value: percent as u64,
                    min: Some(0),
                    max: Some(100),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::Heal {
                                percent: v as u8,
                            })
                    },
                    help: None,
                    step: None,
                }
            }
        }
    }
}

/// 先制效果编辑器
#[component]
fn PriorityEditor(
    value: SkillEffect,
    disabled: bool,
    on_change: EventHandler<SkillEffect>,
) -> Element {
    let (kind, power, priority) = match value {
        SkillEffect::Priority {
            kind,
            power,
            priority,
        } => (kind, power, priority),
        _ => unreachable!(),
    };

    rsx! {
        Row {
            Col { span: 6,
                EffectTypeSelectField {
                    value: "priority",
                    disabled,
                    on_change: move |v: String| {
                        if let Ok(new_effect) = create_skill_effect(&v) {
                            on_change.call(new_effect);
                        }
                    },
                }
            }
            Col { span: 6,
                KindSelectField {
                    label: "属性",
                    value: kind,
                    disabled,
                    on_change: move |k| {
                        on_change
                            .call(SkillEffect::Priority {
                                kind: k,
                                power,
                                priority,
                            })
                    },
                }
            }
        }
        Row {
            Col { span: 6,
                UnsignedNumberField {
                    label: "威力".to_string(),
                    value: power,
                    min: Some(0),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::Priority {
                                kind,
                                power: v,
                                priority,
                            })
                    },
                    help: None,
                    step: None,
                    max: None,
                }
            }
            Col { span: 6,
                NumberField {
                    label: "优先级".to_string(),
                    value: priority as i64,
                    min: Some(-7),
                    max: Some(7),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::Priority {
                                kind,
                                power,
                                priority: v as i8,
                            })
                    },
                    help: Some("正数先制，负数后制".to_string()),
                    step: None,
                }
            }
        }
    }
}

/// 反伤效果编辑器
#[component]
fn RecoilEditor(
    value: SkillEffect,
    disabled: bool,
    on_change: EventHandler<SkillEffect>,
) -> Element {
    let (kind, power, recoil_percent) = match value {
        SkillEffect::Recoil {
            kind,
            power,
            recoil_percent,
        } => (kind, power, recoil_percent),
        _ => unreachable!(),
    };

    rsx! {
        Row {
            Col { span: 6,
                EffectTypeSelectField {
                    value: "recoil",
                    disabled,
                    on_change: move |v: String| {
                        if let Ok(new_effect) = create_skill_effect(&v) {
                            on_change.call(new_effect);
                        }
                    },
                }
            }
            Col { span: 6,
                KindSelectField {
                    label: "属性",
                    value: kind,
                    disabled,
                    on_change: move |k| {
                        on_change
                            .call(SkillEffect::Recoil {
                                kind: k,
                                power,
                                recoil_percent,
                            })
                    },
                }
            }
        }
        Row {
            Col { span: 6,
                UnsignedNumberField {
                    label: "威力".to_string(),
                    value: power,
                    min: Some(0),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::Recoil {
                                kind,
                                power: v,
                                recoil_percent,
                            })
                    },
                    help: None,
                    step: None,
                    max: None,
                }
            }
            Col { span: 6,
                UnsignedNumberField {
                    label: "反伤百分比 (%)".to_string(),
                    value: recoil_percent as u64,
                    min: Some(0),
                    max: Some(100),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::Recoil {
                                kind,
                                power,
                                recoil_percent: v as u8,
                            })
                    },
                    help: None,
                    step: None,
                }
            }
        }
    }
}

/// 一击必杀编辑器
#[component]
fn OneHitKOEditor(
    value: SkillEffect,
    disabled: bool,
    on_change: EventHandler<SkillEffect>,
) -> Element {
    let kind = match value {
        SkillEffect::OneHitKO { kind } => kind,
        _ => unreachable!(),
    };

    rsx! {
        Row {
            Col { span: 6,
                EffectTypeSelectField {
                    value: "one_hit_ko",
                    disabled,
                    on_change: move |v: String| {
                        if let Ok(new_effect) = create_skill_effect(&v) {
                            on_change.call(new_effect);
                        }
                    },
                }
            }
            Col { span: 6,
                KindSelectField {
                    label: "属性",
                    value: kind,
                    disabled,
                    on_change: move |k| on_change.call(SkillEffect::OneHitKO { kind: k }),
                }
            }
        }
    }
}

/// 固定伤害编辑器
#[component]
fn FixedDamageEditor(
    value: SkillEffect,
    disabled: bool,
    on_change: EventHandler<SkillEffect>,
) -> Element {
    let damage = match value {
        SkillEffect::FixedDamage { damage } => damage,
        _ => unreachable!(),
    };

    rsx! {
        Row {
            Col { span: 6,
                EffectTypeSelectField {
                    value: "fixed_damage",
                    disabled,
                    on_change: move |v: String| {
                        if let Ok(new_effect) = create_skill_effect(&v) {
                            on_change.call(new_effect);
                        }
                    },
                }
            }
            Col { span: 6,
                UnsignedNumberField {
                    label: "固定伤害值".to_string(),
                    value: damage,
                    min: Some(0),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::FixedDamage {
                                damage: v,
                            })
                    },
                    help: None,
                    step: None,
                    max: None,
                }
            }
        }
    }
}

/// 其他效果编辑器
#[component]
fn OthersEditor(
    value: SkillEffect,
    disabled: bool,
    on_change: EventHandler<SkillEffect>,
) -> Element {
    let (kind, power) = match value {
        SkillEffect::Others { kind, power } => (kind, power),
        _ => unreachable!(),
    };

    rsx! {
        Row {
            Col { span: 6,
                EffectTypeSelectField {
                    value: "others",
                    disabled,
                    on_change: move |v: String| {
                        if let Ok(new_effect) = create_skill_effect(&v) {
                            on_change.call(new_effect);
                        }
                    },
                }
            }
            Col { span: 6,
                KindSelectField {
                    label: "属性",
                    value: kind,
                    disabled,
                    on_change: move |k| {
                        on_change
                            .call(SkillEffect::Others {
                                kind: k,
                                power,
                            })
                    },
                }
            }
        }
        Row {
            Col { span: 6,
                UnsignedNumberField {
                    label: "威力".to_string(),
                    value: power,
                    min: Some(0),
                    disabled,
                    on_change: move |v| {
                        on_change
                            .call(SkillEffect::Others {
                                kind,
                                power: v,
                            })
                    },
                    help: None,
                    step: None,
                    max: None,
                }
            }
        }
    }
}

/// 属性选择字段
#[component]
fn KindSelectField(
    label: &'static str,
    value: PokemonKind,
    disabled: bool,
    on_change: EventHandler<PokemonKind>,
) -> Element {
    let options = pokemon_kind_options();
    let current_value = value.to_string();

    rsx! {
        SelectField {
            label: label.to_string(),
            value: current_value,
            options,
            disabled,
            on_change: move |v: String| {
                if let Ok(parsed) = PokemonKind::try_from(v.as_str()) {
                    on_change.call(parsed);
                }
            },
            help: None,
        }
    }
}

/// 创建技能效果
fn create_skill_effect(effect_type: &str) -> Result<SkillEffect, ()> {
    match effect_type {
        "physical_damage" => Ok(SkillEffect::PhysicalDamage {
            kind: PokemonKind::Normal,
            power: 20,
        }),
        "special_damage" => Ok(SkillEffect::SpecialDamage {
            kind: PokemonKind::Normal,
            power: 20,
        }),
        "stat_boost" => Ok(SkillEffect::StatBoost {
            stat: StatType::Attack,
            stages: 1,
            target: Target::MySelf,
        }),
        "inflict_status" => Ok(SkillEffect::InflictStatus {
            effect: StatusEffect::Burn,
            chance: 10,
        }),
        "heal" => Ok(SkillEffect::Heal { percent: 50 }),
        "priority" => Ok(SkillEffect::Priority {
            kind: PokemonKind::Normal,
            power: 40,
            priority: 1,
        }),
        "recoil" => Ok(SkillEffect::Recoil {
            kind: PokemonKind::Normal,
            power: 120,
            recoil_percent: 25,
        }),
        "one_hit_ko" => Ok(SkillEffect::OneHitKO {
            kind: PokemonKind::Normal,
        }),
        "fixed_damage" => Ok(SkillEffect::FixedDamage { damage: 40 }),
        "others" => Ok(SkillEffect::Others {
            kind: PokemonKind::Normal,
            power: 20,
        }),
        _ => Err(()),
    }
}

/// 能力类型选择器
#[component]
fn StatTypeSelect(
    label: &'static str,
    value: StatType,
    disabled: bool,
    on_change: EventHandler<StatType>,
) -> Element {
    let stat_value = match value {
        StatType::Attack => "attack",
        StatType::Defense => "defense",
        StatType::SpecialAttack => "special_attack",
        StatType::SpecialDefense => "special_defense",
        StatType::Speed => "speed",
    };

    rsx! {
        SelectField {
            label: label.to_string(),
            value: stat_value.to_string(),
            options: stat_type_options(),
            disabled,
            on_change: move |v: String| {
                let new_stat = match v.as_str() {
                    "attack" => StatType::Attack,
                    "defense" => StatType::Defense,
                    "special_attack" => StatType::SpecialAttack,
                    "special_defense" => StatType::SpecialDefense,
                    "speed" => StatType::Speed,
                    _ => StatType::Attack,
                };
                on_change.call(new_stat);
            },
            help: None,
        }
    }
}

/// 目标选择器
#[component]
fn TargetSelect(
    label: &'static str,
    value: Target,
    disabled: bool,
    on_change: EventHandler<Target>,
) -> Element {
    let target_value = match value {
        Target::MySelf => "myself",
        Target::Opponent => "opponent",
    };

    rsx! {
        SelectField {
            label: label.to_string(),
            value: target_value.to_string(),
            options: target_options(),
            disabled,
            on_change: move |v: String| {
                let new_target = match v.as_str() {
                    "myself" => Target::MySelf,
                    "opponent" => Target::Opponent,
                    _ => Target::MySelf,
                };
                on_change.call(new_target);
            },
            help: None,
        }
    }
}

/// 状态效果选择器
#[component]
fn StatusEffectSelect(
    label: &'static str,
    value: StatusEffect,
    disabled: bool,
    on_change: EventHandler<StatusEffect>,
) -> Element {
    let effect_value = match value {
        StatusEffect::Burn => "burn",
        StatusEffect::Freeze => "freeze",
        StatusEffect::Paralysis => "paralysis",
        StatusEffect::Poison => "poison",
        StatusEffect::Sleep => "sleep",
        StatusEffect::Confusion => "confusion",
    };

    rsx! {
        SelectField {
            label: label.to_string(),
            value: effect_value.to_string(),
            options: status_effect_options(),
            disabled,
            on_change: move |v: String| {
                let new_effect = match v.as_str() {
                    "burn" => StatusEffect::Burn,
                    "freeze" => StatusEffect::Freeze,
                    "paralysis" => StatusEffect::Paralysis,
                    "poison" => StatusEffect::Poison,
                    "sleep" => StatusEffect::Sleep,
                    "confusion" => StatusEffect::Confusion,
                    _ => StatusEffect::Burn,
                };
                on_change.call(new_effect);
            },
            help: None,
        }
    }
}
