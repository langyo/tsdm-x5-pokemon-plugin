use strum::IntoEnumIterator;

use crate::dioxus::prelude::*;

use crate::dioxus::{
    components::evolution_editor::EvolutionInfoEditorModal,
    components::search_box::{SearchBox, SearchResult},
    components::tag::Tag,
    pages::shared::ActionModal,
    state::{
        set_notice, AdminNoticeLevel, FilterConditionType, FilterFieldState, FilterPackage,
        LogicalOperation,
    },
    utils::api::{filter_skill_type, list_evolution_info, list_map_info, set_evolution_info},
};
use _utils::types::{
    evolution_info::EvolutionInfo,
    map_info::MapInfo,
    pokemon_type::{PokemonAttributes, PokemonKind, PokemonType},
    skill_type::SkillType,
};

/// 宠物图片路径前缀
const IMG_PATH: &str = "source/plugin/pokemon/pokemon_system/images";

use super::form_fields::{
    BoolField, Col, FieldWrapper, FloatField, FormSection, Row, SelectField, TextAreaField,
    TextField, UnsignedNumberField,
};

/// 获取所有 PokemonKind 选项
fn pokemon_kind_options() -> Vec<(String, String)> {
    PokemonKind::iter()
        .map(|k| (k.to_string(), k.to_string()))
        .collect()
}

/// 格式化进化条件标签
fn format_evolution_condition(evo: &EvolutionInfo) -> String {
    match &evo.condition {
        _utils::types::evolution_info::EvolutionLimitType::MinLevel(v) => {
            format!(" (Lv.{})", v.min_level)
        }
        _utils::types::evolution_info::EvolutionLimitType::UseItem(v) => {
            if v.item_name.is_empty() {
                " (使用道具)".to_string()
            } else {
                format!(" (使用: {})", v.item_name)
            }
        }
        _utils::types::evolution_info::EvolutionLimitType::MinIntimacy(v) => {
            format!(" (亲密度 {})", v.min_intimacy)
        }
        _utils::types::evolution_info::EvolutionLimitType::CompareAttackAndDefense(_) => {
            " (攻防比较)".to_string()
        }
        _utils::types::evolution_info::EvolutionLimitType::Random(_) => " (随机)".to_string(),
        _utils::types::evolution_info::EvolutionLimitType::Sex(_) => " (性别)".to_string(),
        _utils::types::evolution_info::EvolutionLimitType::IsBagHaveChairs(_) => {
            " (背包空位)".to_string()
        }
    }
}

/// 宠物属性编辑器组件（两行三列布局）
#[component]
pub fn PokemonAttributesEditor(
    value: PokemonAttributes,
    disabled: bool,
    on_change: EventHandler<PokemonAttributes>,
) -> Element {
    let mut attrs = value;

    rsx! {
        Row { gap: "8px".to_string(),
            Col { span: 4,
                AttributeInput {
                    label: "HP".to_string(),
                    value: attrs.hit_points,
                    disabled,
                    on_change: move |v| {
                        attrs.hit_points = v;
                        on_change.call(attrs);
                    },
                }
            }
            Col { span: 4,
                AttributeInput {
                    label: "攻击".to_string(),
                    value: attrs.attack,
                    disabled,
                    on_change: move |v| {
                        attrs.attack = v;
                        on_change.call(attrs);
                    },
                }
            }
            Col { span: 4,
                AttributeInput {
                    label: "防御".to_string(),
                    value: attrs.defense,
                    disabled,
                    on_change: move |v| {
                        attrs.defense = v;
                        on_change.call(attrs);
                    },
                }
            }
        }
        Row { gap: "8px".to_string(),
            Col { span: 4,
                AttributeInput {
                    label: "特攻".to_string(),
                    value: attrs.special_attack,
                    disabled,
                    on_change: move |v| {
                        attrs.special_attack = v;
                        on_change.call(attrs);
                    },
                }
            }
            Col { span: 4,
                AttributeInput {
                    label: "特防".to_string(),
                    value: attrs.special_defense,
                    disabled,
                    on_change: move |v| {
                        attrs.special_defense = v;
                        on_change.call(attrs);
                    },
                }
            }
            Col { span: 4,
                AttributeInput {
                    label: "速度".to_string(),
                    value: attrs.speed,
                    disabled,
                    on_change: move |v| {
                        attrs.speed = v;
                        on_change.call(attrs);
                    },
                }
            }
        }
    }
}

#[component]
fn AttributeInput(
    label: String,
    value: u64,
    disabled: bool,
    on_change: EventHandler<u64>,
) -> Element {
    rsx! {
        FieldWrapper { label, help: None,
            input {
                class: "admin-input admin-input--small",
                r#type: "number",
                min: "0",
                max: "255",
                value: "{value}",
                disabled,
                oninput: move |evt| {
                    if let Ok(num) = evt.value().parse::<u64>() {
                        on_change.call(num.min(255));
                    }
                },
            }
        }
    }
}

/// 宠物类型精确编辑器 Modal
#[component]
pub fn PokemonTypeEditorModal(
    title: String,
    data: PokemonType,
    save_text: String,
    disabled: bool,
    on_save: EventHandler<PokemonType>,
    on_close: EventHandler<()>,
) -> Element {
    let mut draft = use_signal(|| data.clone());

    // 关联数据
    let mut available_skills = use_signal(|| Vec::<SkillType>::new());
    let mut evolution_routes = use_signal(|| Vec::<EvolutionInfo>::new());
    let mut maps = use_signal(|| Vec::<MapInfo>::new());

    // 搜索状态
    let mut skill_search_results = use_signal(Vec::<SearchResult>::new);
    let mut skill_search_loading = use_signal(|| false);
    let mut evolution_search_results = use_signal(Vec::<SearchResult>::new);
    let mut evolution_search_loading = use_signal(|| false);
    let mut map_search_results = use_signal(Vec::<SearchResult>::new);
    let mut map_search_loading = use_signal(|| false);

    // 加载关联数据
    let load_related_data = move || {
        let draft_clone = draft.clone();
        spawn(async move {
            let pokemon_id = draft_clone.read().id;
            let evolution_ids = draft_clone.read().evolution_info_ids.clone();
            let map_ids = draft_clone.read().map_ids.clone();

            if pokemon_id == 0 {
                return;
            }

            // 加载可学习技能（查询可用此的种族包含该宠物 ID 的技能）
            let filters = vec![FilterFieldState {
                tag: "可用此的种族".to_string(),
                operator: FilterConditionType::Text(LogicalOperation::Contains),
                value: pokemon_id.to_string(),
                enum_options: Vec::new(),
                enabled: true,
            }];
            if let Ok(skills) =
                filter_skill_type(filters.iter().filter_map(|f| f.to_package()).collect()).await
            {
                available_skills.set(skills);
            }

            // 加载进化路线 - 加载所有以当前宠物为源或目标的进化路线
            if let Ok(all_evolutions) = list_evolution_info(0, 1000).await {
                let filtered: Vec<_> = all_evolutions
                    .into_iter()
                    .filter(|e| e.source_id == pokemon_id || e.target_id == pokemon_id)
                    .collect();
                evolution_routes.set(filtered);
            }

            // 加载所在地图 - 使用 list API 然后客户端过滤
            if !map_ids.is_empty() {
                if let Ok(all_maps) = list_map_info(0, 1000).await {
                    let filtered: Vec<_> = all_maps
                        .into_iter()
                        .filter(|m| map_ids.contains(&m.id))
                        .collect();
                    maps.set(filtered);
                }
            } else {
            }
        });
    };

    // 进化路线编辑
    let mut editing_evolution = use_signal(|| None::<EvolutionInfo>);

    // 初次加载关联数据
    use_effect(move || {
        load_related_data();
    });

    // 属性选择相关
    let kind_options = pokemon_kind_options();
    let mut kind_options_with_none = vec![("无".to_string(), "无".to_string())];
    kind_options_with_none.extend(kind_options.clone());

    let kind1 = draft.read().kind.0.to_string();
    let kind2 = draft
        .read()
        .kind
        .1
        .map(|k| k.to_string())
        .unwrap_or_default();
    let kind2_selected = if draft.read().kind.1.is_some() {
        &kind2
    } else {
        "无"
    };

    // 性别权重相关
    let sex_weight_enabled = draft.read().sex_weight.is_some();
    let sex_weight_value = draft.read().sex_weight.unwrap_or(0.0);

    rsx! {
        ActionModal { title, on_close,
            div { class: "admin-form-editor",
                // 基本信息
                FormSection { title: "基本信息".to_string(),
                    // 图片预览
                    div { class: "admin-form-pokemon-preview",
                        img {
                            class: "admin-form-pokemon-sprite",
                            src: "{IMG_PATH}/spm/{draft.read().id}.gif",
                            alt: "{draft.read().name}",
                        }
                    }
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
                                placeholder: Some("宠物名称".to_string()),
                                disabled,
                                on_change: move |v| draft.write().name = v,
                                help: None,
                            }
                        }
                    }
                    TextAreaField {
                        label: "描述".to_string(),
                        value: draft.read().description.clone(),
                        placeholder: Some("宠物描述".to_string()),
                        rows: Some(2),
                        disabled,
                        on_change: move |v| draft.write().description = v,
                        help: None,
                    }
                }

                // 商店相关
                FormSection { title: "商店设置".to_string(),
                    Row {
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "售价".to_string(),
                                value: draft.read().cost,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().cost = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 6,
                            BoolField {
                                label: "可购买".to_string(),
                                value: draft.read().is_selling,
                                disabled,
                                on_change: move |v| draft.write().is_selling = v,
                                help: None,
                            }
                        }
                    }
                }

                // 属性设置
                FormSection { title: "属性设置".to_string(),
                    Row {
                        Col { span: 6,
                            SelectField {
                                label: "主属性".to_string(),
                                value: kind1.clone(),
                                options: kind_options.clone(),
                                disabled,
                                on_change: move |v: String| {
                                    if let Ok(parsed) = PokemonKind::try_from(v.as_str()) {
                                        draft.write().kind.0 = parsed;
                                    }
                                },
                                help: None,
                            }
                        }
                        Col { span: 6,
                            SelectField {
                                label: "副属性".to_string(),
                                value: kind2_selected.to_string(),
                                options: kind_options_with_none,
                                disabled,
                                on_change: move |v: String| {
                                    if v == "无" {
                                        draft.write().kind.1 = None;
                                    } else if let Ok(parsed) = PokemonKind::try_from(v.as_str()) {
                                        draft.write().kind.1 = Some(parsed);
                                    }
                                },
                                help: None,
                            }
                        }
                    }
                    Row {
                        Col { span: 6,
                            BoolField {
                                label: "是否为神兽".to_string(),
                                value: draft.read().is_legendary,
                                disabled,
                                on_change: move |v| draft.write().is_legendary = v,
                                help: None,
                            }
                        }
                    }
                    // 性别权重：拆分为两个字段
                    Row {
                        Col { span: 4,
                            BoolField {
                                label: "启用性别".to_string(),
                                value: sex_weight_enabled,
                                disabled,
                                on_change: move |v| {
                                    if v {
                                        draft.write().sex_weight = Some(0.5);
                                    } else {
                                        draft.write().sex_weight = None;
                                    }
                                },
                                help: None,
                            }
                        }
                        Col { span: 8,
                            FloatField {
                                label: "性别权重".to_string(),
                                value: sex_weight_value,
                                min: Some(0.0),
                                max: Some(1.0),
                                step: Some(0.01),
                                precision: Some(2),
                                disabled: disabled || !sex_weight_enabled,
                                on_change: move |v| {
                                    if sex_weight_enabled {
                                        draft.write().sex_weight = Some(v);
                                    }
                                },
                                help: Some("0.0-1.0，值越大雌性概率越高".to_string()),
                            }
                        }
                    }
                }

                // 六维属性
                FormSection { title: "初始个体值 (0-255)".to_string(),
                    PokemonAttributesEditor {
                        value: draft.read().initial_statistic,
                        disabled,
                        on_change: move |v| draft.write().initial_statistic = v,
                    }
                }

                FormSection { title: "初始努力值 (0-255)".to_string(),
                    PokemonAttributesEditor {
                        value: draft.read().initial_base_points,
                        disabled,
                        on_change: move |v| draft.write().initial_base_points = v,
                    }
                }

                // 权重设置
                FormSection { title: "权重设置".to_string(),
                    Row {
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "捕获权重".to_string(),
                                value: draft.read().capture_weight,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().capture_weight = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "遭遇权重".to_string(),
                                value: draft.read().meet_weight,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().meet_weight = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                    }
                    Row {
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "孵化排序".to_string(),
                                value: draft.read().birth_order,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().birth_order = v,
                                help: Some("从宠物蛋孵化到该宠物的几率".to_string()),
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "强度权重".to_string(),
                                value: draft.read().strength_weight,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().strength_weight = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                    }
                }

                // 金钱掉落
                FormSection { title: "金钱掉落范围".to_string(),
                    Row {
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "最小值".to_string(),
                                value: draft.read().drop_money_range.0,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().drop_money_range.0 = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "最大值".to_string(),
                                value: draft.read().drop_money_range.1,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().drop_money_range.1 = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                    }
                }

                // 可学习技能
                FormSection { title: "可学习技能".to_string(),
                    if available_skills.read().is_empty() {
                        div { class: "admin-empty-list",
                            div { class: "admin-empty-list__content", "暂无可学习技能" }
                        }
                    } else {
                        div { class: "admin-wild-pokemon-tags",
                            for skill in available_skills.read().iter() {
                                Tag {
                                    label: format!("{} #{}", skill.name, skill.id),
                                    disabled,
                                    on_close: {
                                        let skill_id = skill.id;
                                        let mut available_skills_clone = available_skills.clone();
                                        move |_| {
                                            available_skills_clone.write().retain(|s| s.id != skill_id);
                                        }
                                    },
                                    class: Some("admin-tag--closable".to_string()),
                                }
                            }
                        }
                    }

                    // 搜索添加技能
                    div { class: "admin-field",
                        label { class: "admin-field__label", "搜索并添加技能" }
                        SearchBox {
                            placeholder: "输入技能名称搜索...".to_string(),
                            disabled,
                            results: skill_search_results(),
                            loading: *skill_search_loading.read(),
                            on_search: move |query: String| {
                                let query = query.trim().to_string();
                                if query.is_empty() {
                                    skill_search_results.set(Vec::new());
                                    return;
                                }
                                skill_search_loading.set(true);
                                let mut skill_search_results_clone = skill_search_results.clone();
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
                                    let result = filter_skill_type(
                                            filters.iter().filter_map(|f| f.to_package()).collect(),
                                        )
                                        .await;
                                    skill_search_results_clone.write().clear();
                                    match result {
                                        Ok(skills) => {
                                            let results: Vec<_> = skills
                                                .into_iter()
                                                .map(|s| SearchResult {
                                                    id: s.id,
                                                    name: s.name.clone(),
                                                })
                                                .collect();
                                            *skill_search_results_clone.write() = results;
                                        }
                                        Err(e) => {
                                            set_notice(AdminNoticeLevel::Error, format!("搜索失败: {}", e));
                                        }
                                    }
                                    skill_search_loading.set(false);
                                });
                            },
                            on_select: move |result: SearchResult| {
                                let mut available_skills_clone = available_skills.clone();
                                spawn(async move {
                                    if let Ok(skills) = filter_skill_type(
                                            vec![
                                                FilterPackage {
                                                    tag: "ID".to_string(),
                                                    operator: FilterConditionType::Id,
                                                    value: result.id.to_string(),
                                                },
                                            ],
                                        )
                                        .await
                                    {
                                        if let Some(skill) = skills.first() {
                                            available_skills_clone.write().push(skill.clone());
                                        }
                                    }
                                });
                                skill_search_results.set(Vec::new());
                            },
                        }
                    }
                }

                // 进化方向
                FormSection { title: "进化方向".to_string(),
                    if evolution_routes.read().is_empty() {
                        div { class: "admin-empty-list",
                            div { class: "admin-empty-list__content", "暂无进化路线" }
                        }
                    } else {
                        div { class: "admin-list",
                            for evo in evolution_routes.read().iter() {
                                div {
                                    class: "admin-list-item",
                                    onclick: {
                                        let evo_clone = evo.clone();
                                        move |_| {
                                            editing_evolution.set(Some(evo_clone.clone()));
                                        }
                                    },
                                    div { class: "admin-list-item__content",
                                        span { class: "admin-list-item__name", "{evo.source_name}" }
                                        span { class: "admin-list-item__arrow", "→" }
                                        span { class: "admin-list-item__name", "{evo.target_name}" }
                                        span { class: "admin-list-item__condition", "{format_evolution_condition(&evo)}" }
                                    }
                                    span { class: "admin-list-item__id", "#{evo.id}" }
                                }
                            }
                        }
                    }

                    // 搜索添加进化路线
                    div { class: "admin-field",
                        label { class: "admin-field__label", "搜索并添加进化路线" }
                        SearchBox {
                            placeholder: "输入进化路线搜索...".to_string(),
                            disabled,
                            results: evolution_search_results(),
                            loading: *evolution_search_loading.read(),
                            on_search: move |query: String| {
                                let query = query.trim().to_string();
                                if query.is_empty() {
                                    evolution_search_results.set(Vec::new());
                                    return;
                                }
                                evolution_search_loading.set(true);
                                let mut evolution_search_results_clone = evolution_search_results.clone();
                                spawn(async move {
                                    if let Ok(all_evolutions) = list_evolution_info(0, 1000).await {
                                        let results: Vec<_> = all_evolutions
                                            .into_iter()
                                            .filter(|e| {
                                                e.source_name.contains(&query) || e.target_name.contains(&query)
                                            })
                                            .map(|e| SearchResult {
                                                id: e.id,
                                                name: format!("{} → {}", e.source_name, e.target_name),
                                            })
                                            .collect();
                                        *evolution_search_results_clone.write() = results;
                                    }
                                    evolution_search_loading.set(false);
                                });
                            },
                            on_select: move |result: SearchResult| {
                                let mut evolution_routes_clone = evolution_routes.clone();
                                spawn(async move {
                                    if let Ok(all_evolutions) = list_evolution_info(0, 1000).await {
                                        if let Some(evolution) = all_evolutions
                                            .iter()
                                            .find(|e| e.id == result.id)
                                        {
                                            evolution_routes_clone.write().push(evolution.clone());
                                        }
                                    }
                                });
                                evolution_search_results.set(Vec::new());
                            },
                        }
                    }
                }

                // 所在地图
                FormSection { title: "所在地图".to_string(),
                    if maps.read().is_empty() {
                        div { class: "admin-empty-list",
                            div { class: "admin-empty-list__content", "暂无所在地图" }
                        }
                    } else {
                        div { class: "admin-wild-pokemon-tags",
                            for map in maps.read().iter() {
                                Tag {
                                    label: format!("{} #{}", map.name, map.id),
                                    disabled,
                                    on_close: {
                                        let map_id = map.id;
                                        let mut maps_clone = maps.clone();
                                        move |_| {
                                            maps_clone.write().retain(|m| m.id != map_id);
                                        }
                                    },
                                    class: Some("admin-tag--closable".to_string()),
                                }
                            }
                        }
                    }

                    // 搜索添加地图
                    div { class: "admin-field",
                        label { class: "admin-field__label", "搜索并添加地图" }
                        SearchBox {
                            placeholder: "输入地图名称搜索...".to_string(),
                            disabled,
                            results: map_search_results(),
                            loading: *map_search_loading.read(),
                            on_search: move |query: String| {
                                let query = query.trim().to_string();
                                if query.is_empty() {
                                    map_search_results.set(Vec::new());
                                    return;
                                }
                                map_search_loading.set(true);
                                let mut map_search_results_clone = map_search_results.clone();
                                spawn(async move {
                                    if let Ok(all_maps) = list_map_info(0, 1000).await {
                                        let results: Vec<_> = all_maps
                                            .into_iter()
                                            .filter(|m| m.name.contains(&query))
                                            .map(|m| SearchResult {
                                                id: m.id,
                                                name: m.name.clone(),
                                            })
                                            .collect();
                                        *map_search_results_clone.write() = results;
                                    }
                                    map_search_loading.set(false);
                                });
                            },
                            on_select: move |result: SearchResult| {
                                let mut maps_clone = maps.clone();
                                spawn(async move {
                                    if let Ok(all_maps) = list_map_info(0, 1000).await {
                                        if let Some(map) = all_maps.iter().find(|m| m.id == result.id) {
                                            maps_clone.write().push(map.clone());
                                        }
                                    }
                                });
                                map_search_results.set(Vec::new());
                            },
                        }
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
                        onclick: {
                            let maps_clone = maps.clone();
                            let evolution_routes_clone = evolution_routes.clone();
                            move |_| {
                                let mut data = draft.read().clone();
                                data.map_ids = maps_clone.read().iter().map(|m| m.id).collect();
                                data.evolution_info_ids = evolution_routes_clone
                                    .read()
                                    .iter()
                                    .map(|e| e.id)
                                    .collect();
                                on_save.call(data);
                            }
                        },
                        "{save_text}"
                    }
                }

                // 进化路线编辑 modal
                if let Some(evolution) = editing_evolution.read().clone() {
                    EvolutionInfoEditorModal {
                        title: "编辑进化路线".to_string(),
                        data: evolution.clone(),
                        save_text: "保存".to_string(),
                        disabled: false,
                        on_save: move |updated: EvolutionInfo| {
                            let mut evolution_routes_clone = evolution_routes.clone();
                            spawn(async move {
                                match set_evolution_info(updated.clone()).await {
                                    Ok(_) => {
                                        evolution_routes_clone.write().retain(|e| e.id != updated.id);
                                        evolution_routes_clone.write().push(updated);
                                        set_notice(
                                            AdminNoticeLevel::Success,
                                            "已保存进化路线".to_string(),
                                        );
                                        editing_evolution.set(None);
                                    }
                                    Err(e) => {
                                        set_notice(AdminNoticeLevel::Error, format!("保存失败: {}", e));
                                    }
                                }
                            });
                        },
                        on_close: move |_| {
                            editing_evolution.set(None);
                        },
                    }
                }
            }
        }
    }
}
