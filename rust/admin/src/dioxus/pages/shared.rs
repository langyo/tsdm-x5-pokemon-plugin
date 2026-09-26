use crate::dioxus::prelude::*;

use crate::dioxus::{
    components::{
        icon::{Icon, IconName},
        icon_button::IconButton,
    },
    state::{FilterConditionType, FilterFieldState, LogicalOperation, NumberOperation},
};

#[component]
pub fn FilterCollapsiblePanel(
    fields: Vec<FilterFieldState>,
    disabled: bool,
    visible: bool,
    on_close: EventHandler<()>,
    on_operator_change: EventHandler<(usize, String)>,
    on_value_change: EventHandler<(usize, String)>,
    on_enabled_change: EventHandler<(usize, bool)>,
    on_apply: EventHandler<()>,
    on_reset: EventHandler<()>,
) -> Element {
    let class = if visible {
        "admin-inline-filter is-open"
    } else {
        "admin-inline-filter"
    };

    rsx! {
        div { class: "{class}",
            div {
                class: "admin-inline-filter__backdrop",
                onclick: move |_| on_close.call(()),
            }
            div { class: "admin-inline-filter__body",
                for (index , field) in fields.into_iter().enumerate() {
                    FilterFieldRow {
                        key: "filter-{field.tag}-{index}",
                        index,
                        field,
                        disabled,
                        on_operator_change,
                        on_value_change,
                        on_enabled_change,
                    }
                }
            }
            div { class: "admin-inline-filter__footer",
                button {
                    class: "admin-btn admin-btn--primary",
                    disabled,
                    onclick: move |_| on_apply.call(()),
                    if disabled {
                        "查询中..."
                    } else {
                        "应用筛选"
                    }
                }
                button {
                    class: "admin-btn",
                    disabled,
                    onclick: move |_| on_reset.call(()),
                    "重置筛选"
                }
            }
        }
    }
}

#[component]
pub fn FloatingActionBar(children: Element) -> Element {
    rsx! {
        div { class: "admin-floating-action-bar",
            div { class: "admin-actions", {children} }
        }
    }
}

#[component]
pub fn FilterPanel(
    title: &'static str,
    fields: Vec<FilterFieldState>,
    disabled: bool,
    show_title: bool,
    show_actions: bool,
    on_operator_change: EventHandler<(usize, String)>,
    on_value_change: EventHandler<(usize, String)>,
    on_enabled_change: EventHandler<(usize, bool)>,
    on_apply: EventHandler<()>,
    on_reset: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "admin-card filter-panel",
            if show_title || show_actions {
                div { class: "card-header",
                    if show_title {
                        h3 { "{title}" }
                    }
                    if show_actions {
                        div { class: "admin-actions",
                            button {
                                class: "admin-btn admin-btn--primary",
                                disabled,
                                onclick: move |_| on_apply.call(()),
                                if disabled {
                                    "查询中..."
                                } else {
                                    "应用筛选"
                                }
                            }
                            button {
                                class: "admin-btn",
                                disabled,
                                onclick: move |_| on_reset.call(()),
                                "重置筛选"
                            }
                        }
                    }
                }
            }
            div { class: "filter-grid",
                for (index , field) in fields.into_iter().enumerate() {
                    FilterFieldRow {
                        key: "filter-{field.tag}-{index}",
                        index,
                        field,
                        disabled,
                        on_operator_change,
                        on_value_change,
                        on_enabled_change,
                    }
                }
            }
        }
    }
}

#[component]
pub fn ListBottomDock(
    title: &'static str,
    fields: Vec<FilterFieldState>,
    disabled: bool,
    total_count: u64,
    loaded_count: u64,
    is_filtered: bool,
    #[props(default = true)] show_create: bool,
    create_title: &'static str,
    create_disabled: bool,
    on_create: EventHandler<()>,
    reload_disabled: bool,
    on_reload: EventHandler<()>,
    load_more_disabled: bool,
    on_load_more: EventHandler<()>,
    on_operator_change: EventHandler<(usize, String)>,
    on_value_change: EventHandler<(usize, String)>,
    on_apply: EventHandler<()>,
    on_reset: EventHandler<()>,
    #[props(default = true)] show_filter: bool,
    on_filter_toggle: EventHandler<()>,
) -> Element {
    let _ = (
        title,
        fields,
        is_filtered,
        load_more_disabled,
        on_load_more,
        on_operator_change,
        on_value_change,
        on_apply,
        on_reset,
    );

    rsx! {
        div { class: "admin-list-bottom-dock",
            div { class: "admin-list-bottom-dock__status",
                div { class: "admin-list-bottom-dock__summary",
                    span { class: "admin-list-status-bar__item",
                        span { class: "admin-list-status-bar__label", "总数" }
                        strong { class: "admin-list-status-bar__value", "{total_count}" }
                    }
                    span { class: "admin-list-status-bar__item",
                        span { class: "admin-list-status-bar__label", "已加载" }
                        strong { class: "admin-list-status-bar__value", "{loaded_count}" }
                    }
                }
            }
            div { class: "admin-list-bottom-dock__actions",
                if show_create {
                    IconButton {
                        icon: IconName::Plus,
                        tooltip: create_title.to_string(),
                        disabled: disabled || create_disabled,
                        onclick: move |_| on_create.call(()),
                    }
                }
                IconButton {
                    icon: IconName::RotateCcw,
                    tooltip: "重新加载".to_string(),
                    disabled: disabled || reload_disabled,
                    onclick: move |_| on_reload.call(()),
                }
                if show_filter {
                    IconButton {
                        icon: IconName::Search,
                        tooltip: "筛选".to_string(),
                        disabled,
                        class: if is_filtered { Some("admin-icon-button--active".to_string()) } else { None },
                        onclick: move |_| on_filter_toggle.call(()),
                    }
                }
            }
        }
    }
}

#[component]
pub fn ActionModal(
    title: String,
    on_close: EventHandler<()>,
    show_close: Option<bool>,
    wide: Option<bool>,
    children: Element,
) -> Element {
    let show_close = show_close.unwrap_or(true);
    let is_wide = wide.unwrap_or(false);
    let modal_class = if is_wide {
        "admin-action-modal admin-action-modal--wide"
    } else {
        "admin-action-modal"
    };

    rsx! {
        div { class: "{modal_class}",
            button {
                class: "admin-action-modal__backdrop",
                onclick: move |_| on_close.call(()),
                aria_label: "关闭弹窗",
            }
            div { class: "admin-action-modal__panel",
                div { class: "admin-action-modal__header",
                    h3 { "{title}" }
                    if show_close {
                        button {
                            class: "admin-action-modal__close-btn",
                            onclick: move |_| on_close.call(()),
                            aria_label: "关闭",
                            Icon { name: IconName::X, class: None }
                        }
                    }
                }
                div { class: "admin-action-modal__body", {children} }
            }
        }
    }
}

#[component]
pub fn JsonEditorModal(
    title: String,
    value: String,
    save_text: &'static str,
    disabled: bool,
    on_change: EventHandler<String>,
    on_copy: EventHandler<()>,
    on_save: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    rsx! {
        ActionModal { title, on_close,
            div { class: "admin-actions",
                button {
                    class: "admin-btn admin-btn--ghost",
                    onclick: move |_| on_copy.call(()),
                    "复制 JSON"
                }
            }
            textarea {
                class: "admin-textarea",
                rows: "16",
                value: "{value}",
                oninput: move |evt| on_change.call(evt.value()),
            }
            div { class: "admin-actions",
                button {
                    class: "admin-btn admin-btn--primary",
                    disabled,
                    onclick: move |_| on_save.call(()),
                    "{save_text}"
                }
            }
        }
    }
}

#[component]
pub fn JsonDetailModal(
    title: String,
    json: String,
    on_copy: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    rsx! {
        ActionModal { title, on_close,
            div { class: "admin-actions",
                button {
                    class: "admin-btn admin-btn--ghost",
                    onclick: move |_| on_copy.call(()),
                    "复制 JSON"
                }
            }
            pre { class: "history-sql", "{json}" }
        }
    }
}

#[component]
pub fn ConfirmActionModal(
    title: String,
    message: String,
    confirm_text: &'static str,
    disabled: bool,
    on_confirm: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    rsx! {
        ActionModal { title, on_close,
            p { class: "admin-form-help", "{message}" }
            div { class: "admin-actions",
                button { class: "admin-btn", onclick: move |_| on_close.call(()), "取消" }
                button {
                    class: "admin-btn admin-btn--primary",
                    disabled,
                    onclick: move |_| on_confirm.call(()),
                    "{confirm_text}"
                }
            }
        }
    }
}

#[component]
fn FilterFieldRow(
    index: usize,
    field: FilterFieldState,
    disabled: bool,
    on_operator_change: EventHandler<(usize, String)>,
    on_value_change: EventHandler<(usize, String)>,
    on_enabled_change: EventHandler<(usize, bool)>,
) -> Element {
    let value = field.value.clone();
    let current_operator = operator_key(&field.operator);
    let enum_options = field.enum_options.clone();
    let field_enabled = field.enabled;

    // 控件的禁用状态：全局禁用或字段未启用
    let control_disabled = disabled || !field_enabled;

    rsx! {
        div { class: "filter-field",
            // 第一行：标题（左）和开关（右）
            div { class: "filter-field-header",
                span { class: "field-label", "{field.tag}" }
                button {
                    class: if field_enabled { "admin-switch is-active" } else { "admin-switch" },
                    r#type: "button",
                    disabled,
                    onclick: move |_| on_enabled_change.call((index, !field_enabled)),
                    span { class: "admin-switch__track",
                        span { class: "admin-switch__thumb" }
                    }
                }
            }
            // 第二行：控件
            match field.operator {
                FilterConditionType::Boolean => rsx! {
                    select {
                        class: "admin-input",
                        disabled: control_disabled,
                        value: "{value}",
                        onchange: move |event| on_value_change.call((index, event.value())),
                        option { value: "", "-- 请选择 --" }
                        option { value: "是", "是" }
                        option { value: "否", "否" }
                    }
                },
                _ => {
                    // 如果有枚举选项，显示下拉框；否则显示文本/数字输入框
                    if !enum_options.is_empty() {
                        rsx! {
                            select {
                                class: "admin-input",
                                disabled: control_disabled,
                                value: "{value}",
                                onchange: move |event| on_value_change.call((index, event.value())),
                                option { value: "", "-- 请选择 --" }
                                for (_db_value, label) in enum_options {
                                    option { value: "{label}", "{label}" }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "filter-control-row filter-control-row--inline",
                                select {
                                    class: "admin-input admin-input--operator admin-input--compact",
                                    disabled: control_disabled,
                                    value: "{current_operator}",
                                    onchange: move |event| on_operator_change.call((index, event.value())),
                                    for (key , label) in operator_options(&field.operator) {
                                        option { value: "{key}", "{label}" }
                                    }
                                }
                                input {
                                    class: "admin-input",
                                    r#type: if matches!(field.operator, FilterConditionType::Number(_)) { "number" } else { "text" },
                                    disabled: control_disabled,
                                    value: "{value}",
                                    oninput: move |event| on_value_change.call((index, event.value())),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn update_filter_operator(fields: &mut [FilterFieldState], index: usize, raw: &str) {
    if let Some(field) = fields.get_mut(index) {
        field.operator = match field.operator {
            FilterConditionType::Id => FilterConditionType::Id,
            FilterConditionType::Text(_) => FilterConditionType::Text(match raw {
                "not_equal" => LogicalOperation::NotEqual,
                "contains" => LogicalOperation::Contains,
                _ => LogicalOperation::Equal,
            }),
            FilterConditionType::Number(_) => FilterConditionType::Number(match raw {
                "equal" => NumberOperation::Equal,
                "not_equal" => NumberOperation::NotEqual,
                "greater" => NumberOperation::Greater,
                "greater_or_equal" => NumberOperation::GreaterOrEqual,
                "less" => NumberOperation::Less,
                "less_or_equal" => NumberOperation::LessOrEqual,
                _ => NumberOperation::GreaterOrEqual,
            }),
            FilterConditionType::Boolean => FilterConditionType::Boolean,
        };
    }
}

fn operator_key(operator: &FilterConditionType) -> &'static str {
    match operator {
        FilterConditionType::Id => "equal",
        FilterConditionType::Text(LogicalOperation::Equal) => "equal",
        FilterConditionType::Text(LogicalOperation::NotEqual) => "not_equal",
        FilterConditionType::Text(LogicalOperation::Contains) => "contains",
        FilterConditionType::Number(NumberOperation::Equal) => "equal",
        FilterConditionType::Number(NumberOperation::NotEqual) => "not_equal",
        FilterConditionType::Number(NumberOperation::Greater) => "greater",
        FilterConditionType::Number(NumberOperation::GreaterOrEqual) => "greater_or_equal",
        FilterConditionType::Number(NumberOperation::Less) => "less",
        FilterConditionType::Number(NumberOperation::LessOrEqual) => "less_or_equal",
        FilterConditionType::Boolean => "boolean",
    }
}

fn operator_options(operator: &FilterConditionType) -> Vec<(&'static str, &'static str)> {
    match operator {
        FilterConditionType::Id => vec![("equal", "=")],
        FilterConditionType::Text(_) => vec![("equal", "包含"), ("not_equal", "不包含")],
        FilterConditionType::Number(_) => vec![
            ("equal", "="),
            ("not_equal", "≠"),
            ("greater", ">"),
            ("greater_or_equal", ">="),
            ("less", "<"),
            ("less_or_equal", "<="),
        ],
        FilterConditionType::Boolean => vec![("boolean", "布尔")],
    }
}
