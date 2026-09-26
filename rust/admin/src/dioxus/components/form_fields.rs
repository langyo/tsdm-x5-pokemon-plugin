use crate::dioxus::prelude::*;

// =============================================================================
// 12 列网格布局系统
// =============================================================================

/// 行容器（12 列网格）
#[component]
pub fn Row(children: Element, #[props(default)] gap: Option<String>) -> Element {
    let gap_style = gap.unwrap_or_else(|| "12px".to_string());
    rsx! {
        div { class: "admin-row", style: "gap: {gap_style};", {children} }
    }
}

/// 列容器（基于 12 列网格）
#[component]
pub fn Col(
    /// 占用的列数 (1-12)
    #[props(default = 12)]
    span: u32,
    children: Element,
) -> Element {
    rsx! {
        div { class: "admin-col admin-col--{span}", {children} }
    }
}

// =============================================================================
// 表单分组
// =============================================================================

/// 表单分组标题
#[component]
pub fn FormSection(
    title: String,
    children: Element,
    /// 标题栏右侧的操作按钮（可选）
    #[props(default)]
    actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: "admin-form-section",
            div { class: "admin-form-section__header",
                h4 { class: "admin-form-section__title", "{title}" }
                if let Some(actions_elem) = actions {
                    div { class: "admin-form-section__actions", {actions_elem} }
                }
            }
            div { class: "admin-form-section__content", {children} }
        }
    }
}

// =============================================================================
// 基础表单字段包装器
// =============================================================================

/// 表单字段包装器 - label 在左上角，内容垂直居中
#[component]
pub fn FieldWrapper(label: String, help: Option<String>, children: Element) -> Element {
    rsx! {
        div { class: "admin-field",
            label { class: "admin-field__label", "{label}" }
            div { class: "admin-field__body", {children} }
            if let Some(help_text) = help {
                p { class: "admin-field__help", "{help_text}" }
            }
        }
    }
}

// =============================================================================
// 具体表单字段组件
// =============================================================================

/// 文本输入字段
#[component]
pub fn TextField(
    label: String,
    value: String,
    placeholder: Option<String>,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        FieldWrapper { label, help,
            input {
                class: "admin-input",
                r#type: "text",
                value: "{value}",
                placeholder: placeholder.unwrap_or_default(),
                disabled,
                oninput: move |evt| on_change.call(evt.value()),
            }
        }
    }
}

/// 多行文本输入字段
#[component]
pub fn TextAreaField(
    label: String,
    value: String,
    placeholder: Option<String>,
    help: Option<String>,
    rows: Option<usize>,
    disabled: bool,
    on_change: EventHandler<String>,
) -> Element {
    let rows_val = rows.unwrap_or(2);
    rsx! {
        FieldWrapper { label, help,
            textarea {
                class: "admin-textarea",
                rows: "{rows_val}",
                value: "{value}",
                placeholder: placeholder.unwrap_or_default(),
                disabled,
                oninput: move |evt| on_change.call(evt.value()),
            }
        }
    }
}

/// 无符号数字输入字段
#[component]
pub fn UnsignedNumberField(
    label: String,
    value: u64,
    min: Option<u64>,
    max: Option<u64>,
    step: Option<u64>,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<u64>,
) -> Element {
    let min_attr = min.map(|v| v.to_string()).unwrap_or_default();
    let max_attr = max.map(|v| v.to_string()).unwrap_or_default();
    let step_attr = step
        .map(|v| v.to_string())
        .unwrap_or_else(|| "1".to_string());

    rsx! {
        FieldWrapper { label, help,
            input {
                class: "admin-input",
                r#type: "number",
                value: "{value}",
                min: "{min_attr}",
                max: "{max_attr}",
                step: "{step_attr}",
                disabled,
                oninput: move |evt| {
                    if let Ok(num) = evt.value().parse::<u64>() {
                        on_change.call(num);
                    }
                },
            }
        }
    }
}

/// 布尔值滑块开关字段
#[component]
pub fn BoolField(
    label: String,
    value: bool,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<bool>,
) -> Element {
    rsx! {
        FieldWrapper { label, help,
            button {
                class: if value { "admin-switch is-active" } else { "admin-switch" },
                r#type: "button",
                disabled,
                onclick: move |_| on_change.call(!value),
                span { class: "admin-switch__track",
                    span { class: "admin-switch__thumb" }
                }
            }
        }
    }
}

/// 下拉选择字段
#[component]
pub fn SelectField(
    label: String,
    value: String,
    options: Vec<(String, String)>,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        FieldWrapper { label, help,
            select {
                class: "admin-input",
                disabled,
                value: "{value}",
                onchange: move |evt| on_change.call(evt.value()),
                for (opt_value , opt_label) in options {
                    option { value: "{opt_value}", selected: opt_value == value, "{opt_label}" }
                }
            }
        }
    }
}

/// 浮点数输入字段
#[component]
pub fn FloatField(
    label: String,
    value: f32,
    min: Option<f32>,
    max: Option<f32>,
    step: Option<f32>,
    precision: Option<usize>,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<f32>,
) -> Element {
    let min_attr = min.map(|v| v.to_string()).unwrap_or_default();
    let max_attr = max.map(|v| v.to_string()).unwrap_or_default();
    let step_attr = step
        .map(|v| v.to_string())
        .unwrap_or_else(|| "0.01".to_string());
    let display_value = if let Some(p) = precision {
        format!("{:.1$}", value, p)
    } else {
        format!("{}", value)
    };

    rsx! {
        FieldWrapper { label, help,
            input {
                class: "admin-input",
                r#type: "number",
                value: "{display_value}",
                min: "{min_attr}",
                max: "{max_attr}",
                step: "{step_attr}",
                disabled,
                oninput: move |evt| {
                    if let Ok(num) = evt.value().parse::<f32>() {
                        on_change.call(num);
                    }
                },
            }
        }
    }
}

/// ID 数组输入字段
#[component]
pub fn IdArrayField(
    label: String,
    value: Vec<u64>,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<Vec<u64>>,
) -> Element {
    let display_value = value
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    rsx! {
        FieldWrapper { label, help,
            input {
                class: "admin-input",
                r#type: "text",
                value: "{display_value}",
                placeholder: "用逗号分隔的ID列表，如: 1, 2, 3",
                disabled,
                oninput: move |evt| {
                    let parsed: Vec<u64> = evt
                        .value()
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u64>().ok())
                        .collect();
                    on_change.call(parsed);
                },
            }
        }
    }
}

// =============================================================================
// 兼容旧接口的组件（保持向后兼容）
// =============================================================================

/// 旧的 FormRow 组件（保持兼容）
#[component]
pub fn FormRow(children: Element) -> Element {
    rsx! {
        div { class: "admin-form-row", {children} }
    }
}

/// 旧的 NumberField 组件（保持兼容）
#[component]
pub fn NumberField(
    label: String,
    value: i64,
    min: Option<i64>,
    max: Option<i64>,
    step: Option<i64>,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<i64>,
) -> Element {
    let min_attr = min.map(|v| v.to_string()).unwrap_or_default();
    let max_attr = max.map(|v| v.to_string()).unwrap_or_default();
    let step_attr = step
        .map(|v| v.to_string())
        .unwrap_or_else(|| "1".to_string());

    rsx! {
        FieldWrapper { label, help,
            input {
                class: "admin-input",
                r#type: "number",
                value: "{value}",
                min: "{min_attr}",
                max: "{max_attr}",
                step: "{step_attr}",
                disabled,
                oninput: move |evt| {
                    if let Ok(num) = evt.value().parse::<i64>() {
                        on_change.call(num);
                    }
                },
            }
        }
    }
}

/// 旧的 OptionalNumberField 组件（保持兼容）
#[component]
pub fn OptionalNumberField(
    label: String,
    value: Option<i64>,
    min: Option<i64>,
    max: Option<i64>,
    step: Option<i64>,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<Option<i64>>,
) -> Element {
    let min_attr = min.map(|v| v.to_string()).unwrap_or_default();
    let max_attr = max.map(|v| v.to_string()).unwrap_or_default();
    let step_attr = step
        .map(|v| v.to_string())
        .unwrap_or_else(|| "1".to_string());
    let (is_none, display_value) = match value {
        Some(v) => (false, v.to_string()),
        None => (true, String::new()),
    };

    rsx! {
        FieldWrapper { label, help,
            div { class: "admin-field__inline",
                label { class: "admin-field__checkbox",
                    input {
                        r#type: "checkbox",
                        checked: !is_none,
                        disabled,
                        onchange: move |evt| {
                            if evt.checked() {
                                on_change.call(Some(0));
                            } else {
                                on_change.call(None);
                            }
                        },
                    }
                    span { "启用" }
                }
                if !is_none {
                    input {
                        class: "admin-input",
                        r#type: "number",
                        value: "{display_value}",
                        min: "{min_attr}",
                        max: "{max_attr}",
                        step: "{step_attr}",
                        disabled,
                        oninput: move |evt| {
                            if let Ok(num) = evt.value().parse::<i64>() {
                                on_change.call(Some(num));
                            }
                        },
                    }
                }
            }
        }
    }
}

/// 旧的 OptionalFloatField 组件（保持兼容）
#[component]
pub fn OptionalFloatField(
    label: String,
    value: Option<f32>,
    min: Option<f32>,
    max: Option<f32>,
    step: Option<f32>,
    precision: Option<usize>,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<Option<f32>>,
) -> Element {
    let min_attr = min.map(|v| v.to_string()).unwrap_or_default();
    let max_attr = max.map(|v| v.to_string()).unwrap_or_default();
    let step_attr = step
        .map(|v| v.to_string())
        .unwrap_or_else(|| "0.01".to_string());
    let (is_none, display_value) = match value {
        Some(v) => (
            false,
            if let Some(p) = precision {
                format!("{:.1$}", v, p)
            } else {
                format!("{}", v)
            },
        ),
        None => (true, String::new()),
    };

    rsx! {
        FieldWrapper { label, help,
            div { class: "admin-field__inline",
                button {
                    class: if !is_none { "admin-switch is-active" } else { "admin-switch" },
                    r#type: "button",
                    disabled,
                    onclick: move |_| {
                        if is_none {
                            on_change.call(Some(0.0));
                        } else {
                            on_change.call(None);
                        }
                    },
                    span { class: "admin-switch__track",
                        span { class: "admin-switch__thumb" }
                    }
                }
                span { class: "admin-field__inline-label", "启用" }
                if !is_none {
                    input {
                        class: "admin-input admin-input--small",
                        r#type: "number",
                        value: "{display_value}",
                        min: "{min_attr}",
                        max: "{max_attr}",
                        step: "{step_attr}",
                        disabled,
                        oninput: move |evt| {
                            if let Ok(num) = evt.value().parse::<f32>() {
                                on_change.call(Some(num));
                            }
                        },
                    }
                }
            }
        }
    }
}

/// 旧的 SwitchGroupField 组件（保持兼容）
#[component]
pub fn SwitchGroupField(
    label: String,
    value: String,
    options: Vec<(String, String)>,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        FieldWrapper { label, help,
            div { class: "admin-switch-group",
                for (opt_value , opt_label) in options {
                    button {
                        class: if opt_value == value { "admin-switch-btn is-active" } else { "admin-switch-btn" },
                        disabled,
                        onclick: move |_| on_change.call(opt_value.clone()),
                        "{opt_label}"
                    }
                }
            }
        }
    }
}

// =============================================================================
// 新闻公告列表编辑组件
// =============================================================================

/// 新闻公告列表编辑组件
#[component]
pub fn NewsAnnouncementsField(
    label: String,
    /// 公告列表
    value: Vec<_utils::types::api_config::NewsAnnouncement>,
    help: Option<String>,
    disabled: bool,
    on_change: EventHandler<Vec<_utils::types::api_config::NewsAnnouncement>>,
) -> Element {
    use crate::dioxus::pages::shared::ActionModal;
    use _utils::types::api_config::NewsAnnouncement;

    // 内部可变状态 - 用于编辑操作
    let mut announcements = use_signal(|| value.clone());

    // 当 prop 变化时同步到内部状态（使用 JSON 比较检测变化）
    {
        let value_json = serde_json::to_string(&value).unwrap_or_default();
        let current_json = serde_json::to_string(&*announcements.read()).unwrap_or_default();
        if value_json != current_json {
            *announcements.write() = value.clone();
        }
    }

    // Modal 显示状态
    let mut show_modal = use_signal(|| false);

    // 编辑中的索引 (None 表示新建，Some(idx) 表示编辑)
    let mut editing_index = use_signal(|| None::<usize>);

    // 临时编辑值
    let mut edit_title = use_signal(String::new);
    let mut edit_url = use_signal(String::new);

    let has_announcements = !announcements.read().is_empty();

    // Clone announcements list to avoid lifetime issues in the loop
    let items_list = announcements.read().clone();

    // 检查是否可以添加（最多 6 个）
    let can_add = announcements.read().len() < 6;

    // 打开新建/编辑 modal
    let open_modal = {
        let mut show_modal = show_modal.clone();
        let mut editing_index = editing_index.clone();
        let mut edit_title = edit_title.clone();
        let mut edit_url = edit_url.clone();
        let announcements = announcements.clone();
        move |idx: Option<usize>| {
            editing_index.set(idx);
            let current_list = announcements.read().clone();
            if let Some(i) = idx {
                if let Some(item) = current_list.get(i) {
                    edit_title.set(item.title.clone());
                    edit_url.set(item.url.clone());
                }
            } else {
                edit_title.set(String::new());
                edit_url.set(String::new());
            }
            show_modal.set(true);
        }
    };

    // 保存公告（最多 6 个）
    let save_announcement = {
        let mut announcements = announcements.clone();
        let mut edit_title = edit_title.clone();
        let mut edit_url = edit_url.clone();
        let mut editing_index = editing_index.clone();
        let mut show_modal = show_modal.clone();
        let on_change = on_change.clone();
        move |_| {
            let title = edit_title.read().trim().to_string();
            let url = edit_url.read().trim().to_string();

            if title.is_empty() {
                return;
            }

            let mut anns = announcements.read().clone();
            let new_announcement = NewsAnnouncement { title, url };

            if let Some(idx) = *editing_index.read() {
                if idx < anns.len() {
                    anns[idx] = new_announcement;
                }
            } else {
                // 新增时检查数量限制
                if anns.len() >= 6 {
                    return; // 超过限制，不添加
                }
                anns.push(new_announcement);
            }

            *announcements.write() = anns.clone();
            on_change.call(anns);

            // 关闭 modal 并重置
            show_modal.set(false);
            edit_title.set(String::new());
            edit_url.set(String::new());
            editing_index.set(None);
        }
    };

    // 删除公告
    let delete_announcement = {
        let mut announcements = announcements.clone();
        let on_change = on_change.clone();
        move |idx: usize| {
            let mut anns = announcements.read().clone();
            if idx < anns.len() {
                anns.remove(idx);
                *announcements.write() = anns.clone();
                on_change.call(anns);
            }
        }
    };

    rsx! {
        FieldWrapper { label, help,
            div { class: "admin-news-announcements",
                // 公告列表
                if has_announcements {
                    div { class: "admin-news-announcements__list",
                        for (idx , item) in items_list.iter().enumerate() {
                            div { key: "announcement-{idx}", class: "admin-news-announcements__item",
                                div { class: "admin-news-announcements__item-content",
                                    div { class: "admin-news-announcements__item-title", "{item.title}" }
                                    div { class: "admin-news-announcements__item-url", "{item.url}" }
                                }
                                div { class: "admin-news-announcements__item-actions",
                                    button {
                                        class: "admin-btn admin-btn--small",
                                        r#type: "button",
                                        disabled,
                                        onclick: {
                                            let mut open_modal = open_modal.clone();
                                            move |_| open_modal(Some(idx))
                                        },
                                        "编辑"
                                    }
                                    button {
                                        class: "admin-btn admin-btn--small admin-btn--danger",
                                        r#type: "button",
                                        disabled,
                                        onclick: {
                                            let mut delete_announcement = delete_announcement.clone();
                                            move |_| delete_announcement(idx)
                                        },
                                        "删除"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "admin-news-announcements__empty", "暂无公告，点击下方按钮添加" }
                }

                // 添加按钮（达到 6 个时禁用）
                button {
                    class: "admin-btn admin-btn--primary",
                    r#type: "button",
                    disabled: disabled || !can_add,
                    onclick: {
                        let mut open_modal = open_modal.clone();
                        move |_| open_modal(None)
                    },
                    if can_add { "添加公告" } else { "已达上限 (6)" }
                }
            }
        }

        // 编辑/新建 Modal
        if *show_modal.read() {
            ActionModal {
                title: if editing_index.read().is_some() {
                    "编辑公告".to_string()
                } else {
                    "添加新公告".to_string()
                },
                on_close: move |_| {
                    show_modal.set(false);
                    edit_title.set(String::new());
                    edit_url.set(String::new());
                    editing_index.set(None);
                },
                div { class: "admin-news-announcements__modal-form",
                    div { class: "admin-field",
                        label { class: "admin-field__label", "公告标题" }
                        input {
                            class: "admin-input",
                            r#type: "text",
                            placeholder: "输入公告标题",
                            value: "{edit_title}",
                            oninput: move |evt| edit_title.set(evt.value()),
                        }
                    }
                    div { class: "admin-field",
                        label { class: "admin-field__label", "链接地址（可选）" }
                        input {
                            class: "admin-input",
                            r#type: "text",
                            placeholder: "https://...",
                            value: "{edit_url}",
                            oninput: move |evt| edit_url.set(evt.value()),
                        }
                    }
                }
                div { class: "admin-actions",
                    button {
                        class: "admin-btn",
                        onclick: move |_| {
                            show_modal.set(false);
                            edit_title.set(String::new());
                            edit_url.set(String::new());
                            editing_index.set(None);
                        },
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn--primary",
                        onclick: save_announcement,
                        if editing_index.read().is_some() {
                            "保存修改"
                        } else {
                            "添加公告"
                        }
                    }
                }
            }
        }
    }
}
