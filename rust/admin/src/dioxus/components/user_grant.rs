use crate::dioxus::prelude::*;

use crate::dioxus::{
    components::{
        form_fields::{
            BoolField, Col, FormSection, Row, SelectField, TextField, UnsignedNumberField,
        },
        search_box::{SearchBox, SearchResult},
        two_step_wizard::TwoStepWizardModal,
    },
    state::{
        set_busy, set_notice, AdminNoticeLevel, FilterConditionType, FilterFieldState,
        LogicalOperation, ADMIN_BUSY,
    },
    utils::api::{filter_item_type, filter_pokemon_type, insert_item_info, insert_pokemon_info},
};
use _utils::types::{
    item_info::ItemInfo,
    pokemon_info::{PokemonInfo, PokemonSite, PokemonStatus},
    pokemon_type::PokemonSex,
};

// =============================================================================
// 给予宠物模态框
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum GrantPokemonStep {
    #[default]
    Search, // 第一步：搜索并选择宠物
    Configure, // 第二步：配置宠物属性
}

#[derive(Clone, Debug, PartialEq)]
pub struct GrantPokemonDraft {
    pub type_id: Option<u64>,
    pub type_name: String,
    pub name: String,
    pub level: Option<u64>,
    pub site: PokemonSite,
    pub sex: PokemonSex,
    pub is_shiny: bool,
    pub intimacy: Option<u64>,
}

impl Default for GrantPokemonDraft {
    fn default() -> Self {
        Self {
            type_id: None,
            type_name: String::new(),
            name: String::new(),
            level: None,
            site: PokemonSite::Store,
            sex: PokemonSex::Unknown,
            is_shiny: false,
            intimacy: None,
        }
    }
}

#[component]
pub fn GrantPokemonModal(
    owner_uid: u64,
    on_close: EventHandler<()>,
    on_success: EventHandler<PokemonInfo>,
) -> Element {
    let is_busy = *ADMIN_BUSY.read();
    let mut draft = use_signal(GrantPokemonDraft::default);
    let mut step = use_signal(GrantPokemonStep::default);
    let mut search_results = use_signal(Vec::<SearchResult>::new);
    let mut search_loading = use_signal(|| false);

    let current_step = match step() {
        GrantPokemonStep::Search => 0,
        GrantPokemonStep::Configure => 1,
    };

    rsx! {
        TwoStepWizardModal {
            title: format!("给予宠物给用户 #{}", owner_uid),
            step: current_step,
            steps: vec!["搜索宠物".to_string(), "设置属性".to_string()],
            can_proceed: draft().type_id.is_some(),
            disabled: is_busy,
            on_close: move |_| {
                step.set(GrantPokemonStep::Search);
                draft.set(GrantPokemonDraft::default());
                search_results.set(Vec::new());
                on_close.call(());
            },
            on_confirm: move |_| {
                let d = draft();
                if let Some(type_id) = d.type_id {
                    spawn_grant_pokemon(owner_uid, type_id, d.clone(), on_success);
                }
            },
            on_step_change: move |new_step| {
                step.set(
                    if new_step == 0 {
                        GrantPokemonStep::Search
                    } else {
                        GrantPokemonStep::Configure
                    },
                );
            },

            // 第一步：搜索并选择宠物
            if matches!(step(), GrantPokemonStep::Search) {
                FormSection { title: "选择宠物".to_string(),
                    Row { gap: "12px".to_string(),
                        Col { span: 12,
                            div { class: "admin-field",
                                label { class: "admin-field__label", "搜索宠物" }
                                SearchBox {
                                    placeholder: "输入宠物名称搜索...".to_string(),
                                    disabled: is_busy,
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
                                        let mut d = draft();
                                        d.type_id = Some(result.id);
                                        d.type_name = result.name.clone();
                                        draft.set(d);
                                    },
                                    selected: draft()
                                        .type_id
                                        .map(|id| SearchResult {
                                            id,
                                            name: draft().type_name.clone(),
                                        }),
                                    on_clear: move |_: ()| {
                                        let mut d = draft();
                                        d.type_id = None;
                                        d.type_name.clear();
                                        draft.set(d);
                                    },
                                }
                            }
                        }
                    }
                }
            }

            // 第二步：配置宠物属性
            if matches!(step(), GrantPokemonStep::Configure) {
                // 显示已选择的宠物（只读）
                div { class: "admin-grant-selected admin-grant-selected--readonly",
                    span { class: "admin-grant-selected__label", "宠物:" }
                    span { class: "admin-grant-selected__name",
                        "#{draft().type_id.unwrap()} {draft().type_name}"
                    }
                    button {
                        class: "admin-btn-link",
                        r#type: "button",
                        onclick: move |_| {
                            step.set(GrantPokemonStep::Search);
                        },
                        "重新选择"
                    }
                }

                FormSection { title: "属性设置（留空使用随机值）".to_string(),
                    Row { gap: "12px".to_string(),
                        Col { span: 6,
                            TextField {
                                label: "昵称".to_string(),
                                value: draft().name.clone(),
                                placeholder: Some("留空使用默认名称".to_string()),
                                help: None,
                                disabled: is_busy,
                                on_change: move |v: String| {
                                    let mut d = draft();
                                    d.name = v;
                                    draft.set(d);
                                },
                            }
                        }
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "等级".to_string(),
                                value: draft().level.unwrap_or(1),
                                min: Some(1),
                                max: Some(100),
                                help: Some("留空默认为1级".to_string()),
                                disabled: is_busy,
                                on_change: move |v: u64| {
                                    let mut d = draft();
                                    d.level = Some(v);
                                    draft.set(d);
                                },
                            }
                        }
                    }
                    Row { gap: "12px".to_string(),
                        Col { span: 4,
                            SelectField {
                                label: "位置".to_string(),
                                value: site_to_string(&draft().site),
                                options: vec![
                                    ("跟随".to_string(), "跟随玩家".to_string()),
                                    ("背包".to_string(), "背包".to_string()),
                                    ("仓库".to_string(), "仓库".to_string()),
                                ],
                                help: None,
                                disabled: is_busy,
                                on_change: move |v: String| {
                                    let mut d = draft();
                                    d.site = string_to_site(&v);
                                    draft.set(d);
                                },
                            }
                        }
                        Col { span: 4,
                            SelectField {
                                label: "性别".to_string(),
                                value: sex_to_string(&draft().sex),
                                options: vec![
                                    ("未知".to_string(), "未知".to_string()),
                                    ("雄性".to_string(), "雄性".to_string()),
                                    ("雌性".to_string(), "雌性".to_string()),
                                ],
                                help: None,
                                disabled: is_busy,
                                on_change: move |v: String| {
                                    let mut d = draft();
                                    d.sex = string_to_sex(&v);
                                    draft.set(d);
                                },
                            }
                        }
                        Col { span: 4,
                            BoolField {
                                label: "闪光".to_string(),
                                value: draft().is_shiny,
                                help: Some("是否为闪光宠物".to_string()),
                                disabled: is_busy,
                                on_change: move |v: bool| {
                                    let mut d = draft();
                                    d.is_shiny = v;
                                    draft.set(d);
                                },
                            }
                        }
                    }
                    Row { gap: "12px".to_string(),
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "亲密度".to_string(),
                                value: draft().intimacy.unwrap_or(50),
                                min: Some(0),
                                max: Some(100),
                                help: Some("留空默认为50".to_string()),
                                disabled: is_busy,
                                on_change: move |v: u64| {
                                    let mut d = draft();
                                    d.intimacy = Some(v);
                                    draft.set(d);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

fn spawn_grant_pokemon(
    owner_uid: u64,
    type_id: u64,
    draft: GrantPokemonDraft,
    on_success: EventHandler<PokemonInfo>,
) {
    set_busy(true);
    spawn(async move {
        let info = PokemonInfo {
            id: 0,
            type_id,
            owner: owner_uid,
            name: if draft.name.is_empty() {
                draft.type_name.clone()
            } else {
                draft.name.clone()
            },
            site: draft.site,
            level: draft.level.unwrap_or(1),
            experience: 0,
            intimacy: draft.intimacy.unwrap_or(50),
            using_ball_id: 1,
            is_shiny: draft.is_shiny,
            status: PokemonStatus::Normal,
            sex: draft.sex,
            statistic: _utils::types::pokemon_type::PokemonAttributes::from_0_to_31_random_numbers(
            ),
            base_points: Default::default(),
            skills: vec![],
            armor_slots_id: (None, None, None, None),
        };

        match insert_pokemon_info(info).await {
            Ok(saved) => {
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("已给予宠物 #{}", saved.id),
                );
                on_success.call(saved);
            }
            Err(e) => {
                set_notice(AdminNoticeLevel::Error, format!("给予宠物失败: {}", e));
            }
        }
        set_busy(false);
    });
}

// =============================================================================
// 给予物品模态框
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum GrantItemStep {
    #[default]
    Search, // 第一步：搜索并选择物品
    Configure, // 第二步：设置数量
}

#[derive(Clone, Debug, PartialEq)]
pub struct GrantItemDraft {
    pub type_id: Option<u64>,
    pub type_name: String,
    pub count: u64,
}

impl Default for GrantItemDraft {
    fn default() -> Self {
        Self {
            type_id: None,
            type_name: String::new(),
            count: 1,
        }
    }
}

#[component]
pub fn GrantItemModal(
    owner_uid: u64,
    on_close: EventHandler<()>,
    on_success: EventHandler<ItemInfo>,
) -> Element {
    let is_busy = *ADMIN_BUSY.read();
    let mut draft = use_signal(GrantItemDraft::default);
    let mut step = use_signal(GrantItemStep::default);
    let mut search_results = use_signal(Vec::<SearchResult>::new);
    let mut search_loading = use_signal(|| false);

    let current_step = match step() {
        GrantItemStep::Search => 0,
        GrantItemStep::Configure => 1,
    };

    rsx! {
        TwoStepWizardModal {
            title: format!("给予物品给用户 #{}", owner_uid),
            step: current_step,
            steps: vec!["选择物品".to_string(), "设置数量".to_string()],
            can_proceed: draft().type_id.is_some(),
            disabled: is_busy,
            on_close: move |_| {
                step.set(GrantItemStep::Search);
                draft.set(GrantItemDraft::default());
                search_results.set(Vec::new());
                on_close.call(());
            },
            on_confirm: move |_| {
                let d = draft();
                if let Some(type_id) = d.type_id {
                    spawn_grant_item(owner_uid, type_id, d.count, on_success);
                }
            },
            on_step_change: move |new_step| {
                step.set(
                    if new_step == 0 { GrantItemStep::Search } else { GrantItemStep::Configure },
                );
            },

            // 第一步：搜索并选择物品
            if matches!(step(), GrantItemStep::Search) {
                FormSection { title: "选择物品".to_string(),
                    Row { gap: "12px".to_string(),
                        Col { span: 12,
                            div { class: "admin-field",
                                label { class: "admin-field__label", "搜索物品" }
                                SearchBox {
                                    placeholder: "输入物品名称搜索...".to_string(),
                                    disabled: is_busy,
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
                                            let result = filter_item_type(
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
                                        let mut d = draft();
                                        d.type_id = Some(result.id);
                                        d.type_name = result.name.clone();
                                        draft.set(d);
                                    },
                                    selected: draft()
                                        .type_id
                                        .map(|id| SearchResult {
                                            id,
                                            name: draft().type_name.clone(),
                                        }),
                                    on_clear: move |_: ()| {
                                        let mut d = draft();
                                        d.type_id = None;
                                        d.type_name.clear();
                                        draft.set(d);
                                    },
                                }
                            }
                        }
                    }
                }
            }

            // 第二步：设置数量
            if matches!(step(), GrantItemStep::Configure) {
                // 显示已选择的物品（只读）
                div { class: "admin-grant-selected admin-grant-selected--readonly",
                    span { class: "admin-grant-selected__label", "物品:" }
                    span { class: "admin-grant-selected__name",
                        "#{draft().type_id.unwrap()} {draft().type_name}"
                    }
                    button {
                        class: "admin-btn-link",
                        r#type: "button",
                        onclick: move |_| {
                            step.set(GrantItemStep::Search);
                        },
                        "重新选择"
                    }
                }

                // 数量设置
                FormSection { title: "数量设置".to_string(),
                    Row { gap: "12px".to_string(),
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "数量".to_string(),
                                value: draft().count,
                                min: Some(1),
                                max: Some(9999),
                                help: Some("给予的物品数量".to_string()),
                                disabled: is_busy,
                                on_change: move |v: u64| {
                                    let mut d = draft();
                                    d.count = v;
                                    draft.set(d);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

fn spawn_grant_item(owner_uid: u64, type_id: u64, count: u64, on_success: EventHandler<ItemInfo>) {
    set_busy(true);
    spawn(async move {
        let info = ItemInfo {
            id: 0,
            owner: owner_uid,
            type_id,
            count,
        };

        match insert_item_info(info).await {
            Ok(saved) => {
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("已给予物品 #{} x{}", saved.type_id, saved.count),
                );
                on_success.call(saved);
            }
            Err(e) => {
                set_notice(AdminNoticeLevel::Error, format!("给予物品失败: {}", e));
            }
        }
        set_busy(false);
    });
}

// =============================================================================
// 步骤指示器组件
// =============================================================================

#[derive(Props, Clone, PartialEq)]
pub struct StepIndicatorProps {
    pub steps: Vec<String>,
    pub current_step: usize,
}

#[component]
pub fn StepIndicator(props: StepIndicatorProps) -> Element {
    rsx! {
        div { class: "admin-grant-steps",
            for (index , label) in props.steps.iter().enumerate() {
                div { class: if index == props.current_step { "admin-grant-step admin-grant-step--active" } else if index < props.current_step { "admin-grant-step admin-grant-step--completed" } else { "admin-grant-step" },
                    span { class: "admin-grant-step__number", "{index + 1}" }
                    span { class: "admin-grant-step__label", "{label}" }
                }
                // 连接线（最后一个步骤后面不需要）
                if index < props.steps.len() - 1 {
                    div { class: if index < props.current_step { "admin-grant-step__connector admin-grant-step__connector--completed" } else { "admin-grant-step__connector" } }
                }
            }
        }
    }
}

// =============================================================================
// 辅助函数
// =============================================================================

fn site_to_string(site: &PokemonSite) -> String {
    match site {
        PokemonSite::Header => "跟随".to_string(),
        PokemonSite::Bag => "背包".to_string(),
        PokemonSite::Store => "仓库".to_string(),
        PokemonSite::Hospital => "医院".to_string(),
    }
}

fn string_to_site(s: &str) -> PokemonSite {
    match s {
        "跟随" => PokemonSite::Header,
        "背包" => PokemonSite::Bag,
        "仓库" => PokemonSite::Store,
        "医院" => PokemonSite::Hospital,
        _ => PokemonSite::Store,
    }
}

fn sex_to_string(sex: &PokemonSex) -> String {
    match sex {
        PokemonSex::Male => "雄性".to_string(),
        PokemonSex::Female => "雌性".to_string(),
        PokemonSex::Unknown => "未知".to_string(),
    }
}

fn string_to_sex(s: &str) -> PokemonSex {
    match s {
        "雄性" => PokemonSex::Male,
        "雌性" => PokemonSex::Female,
        _ => PokemonSex::Unknown,
    }
}
