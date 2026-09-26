use chrono::{Local, Timelike, Utc};
use std::env::consts;

use crate::dioxus::prelude::*;

use crate::{
    config::{CARGO_VERSION, UI_FRAMEWORK},
    dioxus::{
        components::{
            form_fields::NewsAnnouncementsField, icon::IconName, icon_button::IconButton,
            json_panel::JsonPanel,
        },
        pages::shared::{ActionModal, ConfirmActionModal},
        state::{
            begin_global_config_request, finish_global_config_attempt, finish_global_config_load,
            is_read_only_sql, push_sql_history, reset_global_config_to_saved, set_busy, set_notice,
            set_sql_console_input, update_global_config, AdminNoticeLevel, SqlHistoryEntry,
            ADMIN_BUSY, ADMIN_GLOBAL_CONFIG, ADMIN_SQL_CONSOLE,
        },
        utils::{
            api::{get_global_config, run_sql, set_global_config},
            clipboard::copy_to_clipboard,
            json_tree::{ExpandState, JsonTree, JsonTreeMode},
        },
    },
};
use _utils::types::global_config::GlobalConfigType;

#[component]
pub fn GlobalConfigPage() -> Element {
    let state = ADMIN_GLOBAL_CONFIG.read().clone();
    let config = state.current.clone();
    let config_for_bottom_save = config.clone();
    let is_busy = *ADMIN_BUSY.read();
    let is_dirty = state.is_dirty();

    // SQL 控制台 modal 状态
    let mut show_sql_console = use_signal(|| false);

    use_effect(move || {
        let state = ADMIN_GLOBAL_CONFIG.read().clone();
        if state.initialized || state.loading {
            return;
        }
        reload_global_config();
    });

    // 调试：监控状态变化
    use_effect(move || {
        let _state = ADMIN_GLOBAL_CONFIG.read().clone();
    });

    let debug_label = if cfg!(debug_assertions) {
        "DEBUG"
    } else {
        "RELEASE"
    };
    let platform_label = if cfg!(target_arch = "wasm32") {
        "WebAssembly".to_string()
    } else {
        format!("{}-{}", consts::OS, consts::ARCH)
    };

    rsx! {
        section { class: "admin-page admin-config-page",
            div { class: "admin-page-header",
                div {
                    h2 { "全局配置" }
                }
            }

            div { class: "admin-card config-meta-grid",
                div { class: "meta-item",
                    span { class: "meta-label", "运行模式" }
                    strong { "{debug_label}" }
                }
                div { class: "meta-item",
                    span { class: "meta-label", "插件版本" }
                    strong { "{config.version}" }
                }
                div { class: "meta-item",
                    span { class: "meta-label", "Cargo 版本" }
                    strong { "{CARGO_VERSION}" }
                }
                div { class: "meta-item",
                    span { class: "meta-label", "UI 框架" }
                    strong { "{UI_FRAMEWORK}" }
                }
                div { class: "meta-item",
                    span { class: "meta-label", "平台" }
                    strong { "{platform_label}" }
                }
                div { class: "meta-item",
                    span { class: "meta-label", "未保存更改" }
                    strong {
                        if is_dirty {
                            "是"
                        } else {
                            "否"
                        }
                    }
                }
            }

            div { class: "admin-config-grid",
                Section { title: "基础设定", open: true,
                    BoolField {
                        label: "是否开放",
                        value: config.is_open,
                        true_label: "开放",
                        false_label: "关闭",
                        hint: "控制插件全局开关。",
                        on_change: move |value| update_global_config(|item| item.is_open = value),
                    }
                    TextField {
                        label: "页眉公告标题",
                        value: config.ann_title.clone(),
                        placeholder: "输入公告标题",
                        on_change: move |value| update_global_config(|item| item.ann_title = value),
                    }
                    TextField {
                        label: "页眉公告链接",
                        value: config.ann_url.clone(),
                        placeholder: "https://...",
                        on_change: move |value| update_global_config(|item| item.ann_url = value),
                    }
                    NewsAnnouncementsField {
                        label: "首页新闻公告列表".to_string(),
                        value: config.news_announcements.clone(),
                        help: Some("配置首页显示的新闻公告，支持多条".to_string()),
                        disabled: is_busy,
                        on_change: move |value| update_global_config(|item| item.news_announcements = value),
                    }
                }

                Section { title: "亲密度设定", open: true,
                    NumberField {
                        label: "每小时亲密度增加量",
                        value: config.intimacy_increase_per_hour,
                        hint: "设置为 0 时随机增加 1-10，会影响 PK 状态及部分进化。",
                        min: 0,
                        max: u64::MAX,
                        suffix: "",
                        on_change: move |value| update_global_config(|item| item.intimacy_increase_per_hour = value),
                    }
                    NumberField {
                        label: "每次获得经验亲密度增加量",
                        value: config.intimacy_increase_per_earn_xp,
                        hint: "关闭宠物蛋模式时也不应设置为 0。",
                        min: 1,
                        max: u64::MAX,
                        suffix: "",
                        on_change: move |value: u64| update_global_config(|item| {
                            item.intimacy_increase_per_earn_xp = value.max(1);
                        }),
                    }
                    NumberField {
                        label: "亲密度增加倍率",
                        value: config.intimacy_increase_multiple,
                        hint: "若填 0，宠物蛋无法通过正常孵化。",
                        min: 0,
                        max: u64::MAX,
                        suffix: "",
                        on_change: move |value| update_global_config(|item| item.intimacy_increase_multiple = value),
                    }
                }

                Section { title: "治疗设定", open: true,
                    NumberField {
                        label: "宠物复活所需时间",
                        value: config.revive_time,
                        hint: "单位：分钟。",
                        min: 0,
                        max: u64::MAX,
                        suffix: "分钟",
                        on_change: move |value| update_global_config(|item| item.revive_time = value),
                    }
                    NumberField {
                        label: "宠物单次治疗费用",
                        value: config.medical_price,
                        hint: "",
                        min: 0,
                        max: u64::MAX,
                        suffix: "",
                        on_change: move |value| update_global_config(|item| item.medical_price = value),
                    }
                    NumberField {
                        label: "宠物复活所需回帖数量",
                        value: config.posts_count_for_wakeup,
                        hint: "",
                        min: 0,
                        max: u64::MAX,
                        suffix: "",
                        on_change: move |value| update_global_config(|item| item.posts_count_for_wakeup = value),
                    }
                }

                Section { title: "捕捉设定", open: true,
                    BoolField {
                        label: "是否开放捕捉",
                        value: config.is_enable_catch,
                        true_label: "开放",
                        false_label: "关闭",
                        hint: "",
                        on_change: move |value| update_global_config(|item| item.is_enable_catch = value),
                    }
                    NumberField {
                        label: "捕捉区域进入费用",
                        value: config.catch_price_in_catch_area,
                        hint: "",
                        min: 0,
                        max: u64::MAX,
                        suffix: "",
                        on_change: move |value| update_global_config(|item| item.catch_price_in_catch_area = value),
                    }
                    NumberField {
                        label: "捕捉经验倍率",
                        value: config.pve_catch_xp_multiple,
                        hint: "填 0 采用地图经验倍数设置。",
                        min: 0,
                        max: u64::MAX,
                        suffix: "",
                        on_change: move |value| update_global_config(|item| item.pve_catch_xp_multiple = value),
                    }
                    NumberField {
                        label: "可捕捉等级",
                        value: config.pve_catch_level,
                        hint: "填 0 采用各个宠物自己的设置。",
                        min: 0,
                        max: u64::MAX,
                        suffix: "",
                        on_change: move |value| update_global_config(|item| item.pve_catch_level = value),
                    }
                }

                Section { title: "PVE 设定", open: true,
                    BoolField {
                        label: "是否开放战斗获得经验",
                        value: config.is_enable_earn_xp_on_pve,
                        true_label: "开放",
                        false_label: "关闭",
                        hint: "关闭后无法获得战斗经验。",
                        on_change: move |value| update_global_config(|item| item.is_enable_earn_xp_on_pve = value),
                    }
                    BoolField {
                        label: "是否掉落金钱",
                        value: config.drop_money_on_pve,
                        true_label: "掉落",
                        false_label: "不掉落",
                        hint: "",
                        on_change: move |value| update_global_config(|item| item.drop_money_on_pve = value),
                    }
                    BoolField {
                        label: "金钱掉落配置来源",
                        value: config.drop_money_config_by_global,
                        true_label: "以全局配置为准",
                        false_label: "以宠物各自设置为准",
                        hint: "",
                        on_change: move |value| update_global_config(|item| item.drop_money_config_by_global = value),
                    }
                    NumberRangeField {
                        label: "金钱掉落额度范围",
                        min_value: config.drop_money_percent_min_on_pve,
                        max_value: config.drop_money_percent_max_on_pve,
                        hint: "若宠物比对方等级高于 10 级，将不掉落宠物币。",
                        on_change: move |(min_value, max_value): (u64, u64)| {
                            update_global_config(|item| {
                                item.drop_money_percent_min_on_pve = min_value.min(max_value);
                                item.drop_money_percent_max_on_pve = max_value.max(min_value);
                            });
                        },
                    }
                }

                Section { title: "宠物蛋设定", open: true,
                    BoolField {
                        label: "是否开放宠物蛋直接领取",
                        value: config.is_enable_egg,
                        true_label: "开放",
                        false_label: "关闭",
                        hint: "",
                        on_change: move |value| update_global_config(|item| item.is_enable_egg = value),
                    }
                    BoolField {
                        label: "是否允许购买宠物蛋",
                        value: config.is_enable_buy_egg,
                        true_label: "是",
                        false_label: "否",
                        hint: "",
                        on_change: move |value| update_global_config(|item| item.is_enable_buy_egg = value),
                    }
                    BoolField {
                        label: "是否允许购买宠物",
                        value: config.is_enable_buy_pokemon,
                        true_label: "是",
                        false_label: "否",
                        hint: "",
                        on_change: move |value| update_global_config(|item| item.is_enable_buy_pokemon = value),
                    }
                    BoolField {
                        label: "是否允许持有多个宠物蛋",
                        value: config.is_enable_multiple_eggs,
                        true_label: "允许",
                        false_label: "不允许",
                        hint: "仅对免费领取有效。",
                        on_change: move |value| update_global_config(|item| item.is_enable_multiple_eggs = value),
                    }
                    NumberField {
                        label: "宠物蛋购买费用",
                        value: config.egg_price,
                        hint: "",
                        min: 0,
                        max: u64::MAX,
                        suffix: "",
                        on_change: move |value| update_global_config(|item| item.egg_price = value),
                    }
                    NumberField {
                        label: "孵化宠物蛋所需最低亲密度",
                        value: config.hatch_egg_intimacy,
                        hint: "最大值 255，若小于 50 则直接孵化。",
                        min: 0,
                        max: u8::MAX as u64,
                        suffix: "",
                        on_change: move |value: u64| update_global_config(|item| {
                            item.hatch_egg_intimacy = value.min(u8::MAX as u64);
                        }),
                    }
                }

                Section { title: "高级设置", open: false,
                    p { class: "field-hint",
                        "高级页签入口仍保留在左侧导航与工具区内；具体业务页面请通过左侧导航进入。"
                    }
                    div { class: "admin-actions",
                        button {
                            class: "admin-btn admin-btn--primary",
                            onclick: move |_| show_sql_console.set(true),
                            "打开 SQL 控制台"
                        }
                    }
                }
            }

            // SQL 控制台 Modal
            SqlConsoleModal {
                show: show_sql_console(),
                on_close: move |_| show_sql_console.set(false),
            }

            div { class: "admin-list-bottom-dock",
                div { class: "admin-list-bottom-dock__status",
                    div { class: "admin-list-bottom-dock__summary",
                        span { class: "admin-list-status-bar__item",
                            span { class: "admin-list-status-bar__label", "配置状态" }
                            strong { class: "admin-list-status-bar__value",
                                if is_dirty {
                                    "有未保存修改"
                                } else {
                                    "已同步"
                                }
                            }
                        }
                    }
                }
                div { class: "admin-list-bottom-dock__actions",
                    IconButton {
                        icon: IconName::RotateCcw,
                        tooltip: "恢复已保存版本".to_string(),
                        disabled: is_busy || !is_dirty,
                        onclick: move |_| reset_global_config_to_saved(),
                    }
                    IconButton {
                        icon: IconName::Save,
                        tooltip: "保存配置".to_string(),
                        disabled: is_busy || !is_dirty,
                        onclick: move |_| save_global_config(config_for_bottom_save.clone()),
                    }
                }
            }
        }
    }
}

fn reload_global_config() {
    begin_global_config_request();
    set_busy(true);

    spawn(async move {
        match get_global_config().await {
            Ok(config) => {
                finish_global_config_load(config);
                set_notice(AdminNoticeLevel::Info, "全局配置已重新加载");
            }
            Err(error) => {
                finish_global_config_attempt();
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("读取全局配置失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn save_global_config(config: GlobalConfigType) {
    begin_global_config_request();
    set_busy(true);

    spawn(async move {
        match set_global_config(config).await {
            Ok(_saved) => {
                // 保存成功后，重新从服务器获取最新配置
                reload_global_config();
                set_notice(AdminNoticeLevel::Success, "全局配置已保存");
            }
            Err(error) => {
                finish_global_config_attempt();
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("保存全局配置失败: {}", error),
                );
                set_busy(false);
            }
        }
    });
}

#[component]
fn Section(title: &'static str, open: bool, children: Element) -> Element {
    rsx! {
        details { class: "admin-card admin-section", open,
            summary { class: "section-summary", "{title}" }
            div { class: "section-body", {children} }
        }
    }
}

#[component]
fn TextField(
    label: &'static str,
    value: String,
    placeholder: &'static str,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        label { class: "config-field",
            span { class: "field-label", "{label}" }
            input {
                class: "admin-input",
                value: "{value}",
                placeholder: "{placeholder}",
                oninput: move |event| on_change.call(event.value()),
            }
        }
    }
}

#[component]
fn TextAreaField(
    label: &'static str,
    value: String,
    rows: u32,
    hint: &'static str,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        label { class: "config-field",
            span { class: "field-label", "{label}" }
            if !hint.is_empty() {
                span { class: "field-hint", "{hint}" }
            }
            textarea {
                class: "admin-textarea",
                rows: "{rows}",
                value: "{value}",
                oninput: move |event| on_change.call(event.value()),
            }
        }
    }
}

#[component]
fn BoolField(
    label: &'static str,
    value: bool,
    true_label: &'static str,
    false_label: &'static str,
    hint: &'static str,
    on_change: EventHandler<bool>,
) -> Element {
    rsx! {
        div { class: "config-field",
            span { class: "field-label", "{label}" }
            if !hint.is_empty() {
                span { class: "field-hint", "{hint}" }
            }
            div { class: "toggle-group",
                button {
                    class: if value { "toggle-btn is-active" } else { "toggle-btn" },
                    onclick: move |_| on_change.call(true),
                    "{true_label}"
                }
                button {
                    class: if !value { "toggle-btn is-active" } else { "toggle-btn" },
                    onclick: move |_| on_change.call(false),
                    "{false_label}"
                }
            }
        }
    }
}

#[component]
fn NumberField(
    label: &'static str,
    value: u64,
    hint: &'static str,
    min: u64,
    max: u64,
    suffix: &'static str,
    on_change: EventHandler<u64>,
) -> Element {
    rsx! {
        label { class: "config-field",
            span { class: "field-label", "{label}" }
            if !hint.is_empty() {
                span { class: "field-hint", "{hint}" }
            }
            div { class: "field-input-row",
                input {
                    class: "admin-input",
                    r#type: "number",
                    min: "{min}",
                    max: "{max}",
                    value: "{value}",
                    oninput: move |event| {
                        if let Ok(parsed) = event.value().parse::<u64>() {
                            on_change.call(parsed.clamp(min, max));
                        }
                    },
                }
                if !suffix.is_empty() {
                    span { class: "field-suffix", "{suffix}" }
                }
            }
        }
    }
}

#[component]
fn NumberRangeField(
    label: &'static str,
    min_value: u64,
    max_value: u64,
    hint: &'static str,
    on_change: EventHandler<(u64, u64)>,
) -> Element {
    rsx! {
        div { class: "config-field",
            span { class: "field-label", "{label}" }
            if !hint.is_empty() {
                span { class: "field-hint", "{hint}" }
            }
            div { class: "range-row",
                input {
                    class: "admin-input",
                    r#type: "number",
                    min: "0",
                    value: "{min_value}",
                    oninput: move |event| {
                        if let Ok(next_min) = event.value().parse::<u64>() {
                            on_change.call((next_min.min(max_value), max_value.max(next_min)));
                        }
                    },
                }
                span { class: "range-separator", "~" }
                input {
                    class: "admin-input",
                    r#type: "number",
                    min: "0",
                    value: "{max_value}",
                    oninput: move |event| {
                        if let Ok(next_max) = event.value().parse::<u64>() {
                            on_change.call((min_value.min(next_max), next_max.max(min_value)));
                        }
                    },
                }
            }
        }
    }
}

// SQL 控制台 Modal 组件
#[component]
fn SqlConsoleModal(show: bool, on_close: EventHandler<()>) -> Element {
    let sql_state = ADMIN_SQL_CONSOLE.read().clone();
    let is_busy = *ADMIN_BUSY.read();
    let input = sql_state.console.clone();
    let input_for_copy = input.clone();
    let history = sql_state.history.clone();
    // 与 SQL 控制台页面保持一致的安全策略：默认只读，写语句需二次确认。
    let mut read_only = use_signal(|| true);
    let mut pending_write = use_signal(|| None::<String>);

    let execute_sql = move |sql: String| {
        set_busy(true);
        spawn(async move {
            let executed_at = Utc::now();
            match run_sql(sql.clone()).await {
                Ok(result) => {
                    push_sql_history(SqlHistoryEntry {
                        executed_at,
                        sql,
                        result: Ok(result),
                    });
                    set_notice(AdminNoticeLevel::Success, "SQL 执行完成");
                }
                Err(error) => {
                    push_sql_history(SqlHistoryEntry {
                        executed_at,
                        sql,
                        result: Err(error.to_string()),
                    });
                    set_notice(
                        AdminNoticeLevel::Error,
                        format!("执行 SQL 时出现错误: {}", error),
                    );
                }
            }
            set_busy(false);
        });
    };
    let execute_sql_confirmed = execute_sql;
    let pending_sql = pending_write();
    let pending_preview = pending_sql.as_ref().map(|sql| {
        let head: String = sql.chars().take(200).collect();
        if sql.chars().count() > 200 {
            format!("{}…", head)
        } else {
            head
        }
    });

    rsx! {
        if show {
            ActionModal { title: "SQL 控制台".to_string(), on_close,
                div { class: "admin-card admin-sql-editor",
                    label { class: "field-label", "SQL" }
                    label { class: "sql-readonly-toggle",
                        input {
                            r#type: "checkbox",
                            checked: read_only(),
                            onchange: move |event| read_only.set(event.checked()),
                        }
                        "只读模式（仅允许查询语句）"
                    }
                    textarea {
                        class: "admin-textarea admin-textarea--code",
                        rows: "6",
                        placeholder: "SELECT * FROM pm_config LIMIT 10;",
                        value: "{input}",
                        oninput: move |event| set_sql_console_input(event.value()),
                    }
                    div { class: "admin-actions",
                        button {
                            class: "admin-btn admin-btn--primary",
                            disabled: is_busy || input.trim().is_empty(),
                            onclick: move |_| {
                                let sql = ADMIN_SQL_CONSOLE.read().console.trim().to_string();
                                if sql.is_empty() {
                                    return;
                                }
                                if read_only() && !is_read_only_sql(&sql) {
                                    set_notice(
                                        AdminNoticeLevel::Error,
                                        "只读模式下仅允许 SELECT/SHOW/DESCRIBE/EXPLAIN 查询语句",
                                    );
                                    return;
                                }
                                if !read_only() {
                                    pending_write.set(Some(sql));
                                    return;
                                }
                                execute_sql(sql);
                            },
                            if is_busy {
                                "执行中..."
                            } else {
                                "执行"
                            }
                        }
                        button {
                            class: "admin-btn",
                            disabled: input.is_empty(),
                            onclick: move |_| {
                                match copy_to_clipboard(&input_for_copy) {
                                    Ok(()) => {
                                        set_notice(AdminNoticeLevel::Info, "SQL 已复制到剪贴板");
                                    }
                                    Err(error) => {
                                        set_notice(
                                            AdminNoticeLevel::Error,
                                            format!("复制失败: {}", error),
                                        );
                                    }
                                }
                            },
                            "复制"
                        }
                        button {
                            class: "admin-btn",
                            disabled: input.is_empty(),
                            onclick: move |_| set_sql_console_input(String::new()),
                            "清空"
                        }
                    }
                }

                div { class: "admin-card admin-sql-history",
                    div { class: "card-header",
                        h3 { "执行历史" }
                        span { class: "history-count", "{history.len()} 条" }
                    }
                    if history.is_empty() {
                        p { class: "empty-hint", "暂无执行记录。" }
                    } else {
                        div { class: "history-list",
                            for entry in history.into_iter().rev() {
                                SqlHistoryModalCard {
                                    key: "sql-{entry.executed_at.timestamp_millis()}",
                                    entry,
                                }
                            }
                        }
                    }
                }

                if let Some(sql) = pending_sql {
                    ConfirmActionModal {
                        title: "确认执行写操作".to_string(),
                        message: format!(
                            "只读模式已关闭，以下语句可能修改或删除数据，确认执行吗？\n\n{}",
                            pending_preview.unwrap_or_default(),
                        ),
                        confirm_text: "确认执行",
                        disabled: is_busy,
                        on_confirm: move |_| {
                            pending_write.set(None);
                            execute_sql_confirmed(sql.clone());
                        },
                        on_close: move |_| pending_write.set(None),
                    }
                }
            }
        }
    }
}

#[component]
fn SqlHistoryModalCard(entry: SqlHistoryEntry) -> Element {
    // 显示时转为浏览器本地时区（存储仍为 UTC）
    let executed_local = entry.executed_at.with_timezone(&Local);
    let timestamp = format!(
        "{:02}:{:02}:{:02}",
        executed_local.hour(),
        executed_local.minute(),
        executed_local.second()
    );
    let is_ok = entry.result.is_ok();
    let sql_copy = entry.sql.clone();

    rsx! {
        details {
            class: if is_ok { "history-item is-success" } else { "history-item is-error" },
            open: true,
            summary { class: "history-summary",
                span { class: "history-status",
                    if is_ok {
                        span {
                            class: "admin-icon admin-icon--sm",
                            dangerous_inner_html: "✓",
                        }
                    } else {
                        span {
                            class: "admin-icon admin-icon--sm",
                            dangerous_inner_html: "✕",
                        }
                    }
                }
                span { class: "history-time", "{timestamp}" }
            }
            div { class: "history-body",
                div { class: "history-toolbar",
                    button {
                        class: "admin-btn admin-btn--ghost",
                        onclick: move |_| {
                            match copy_to_clipboard(&sql_copy) {
                                Ok(()) => {
                                    set_notice(AdminNoticeLevel::Info, "SQL 已复制到剪贴板");
                                }
                                Err(error) => {
                                    set_notice(
                                        AdminNoticeLevel::Error,
                                        format!("复制失败: {}", error),
                                    );
                                }
                            }
                        },
                        "复制 SQL"
                    }
                }
                pre { class: "history-sql", "{entry.sql}" }
                match &entry.result {
                    Ok(result) => {
                        let copy_result = result.clone();
                        let parsed_value = serde_json::from_str::<serde_json::Value>(result).ok();
                        rsx! {
                            div { class: "history-toolbar",
                                button {
                                    class: "admin-btn admin-btn--ghost",
                                    onclick: move |_| {
                                        match copy_to_clipboard(&copy_result) {
                                            Ok(()) => {
                                                set_notice(
                                                    AdminNoticeLevel::Info,
                                                    "结果已复制到剪贴板",
                                                );
                                            }
                                            Err(error) => {
                                                set_notice(
                                                    AdminNoticeLevel::Error,
                                                    format!("复制失败: {}", error),
                                                );
                                            }
                                        }
                                    },
                                    "复制结果"
                                }
                            }
                            if let Some(json_value) = parsed_value {
                                div { class: "history-result-wrapper",
                                    JsonPanel { value: json_value }
                                }
                            } else {
                                pre { "{result}" }
                            }
                        }
                    }
                    Err(error) => rsx! {
                        div { class: "history-result history-result--error",
                            strong { "执行失败" }
                            p { "{error}" }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn JsonEditorField(
    label: &'static str,
    value: String,
    hint: &'static str,
    on_change: EventHandler<String>,
) -> Element {
    use serde_json::Value;

    let mut json_value = use_signal(|| {
        serde_json::from_str::<Value>(&value).unwrap_or(Value::Object(serde_json::Map::new()))
    });
    let mut parse_error = use_signal(|| None::<String>);
    let mut view_mode = use_signal(|| true);

    let parsed = serde_json::from_str::<Value>(&value);
    let is_valid = parsed.is_ok();

    rsx! {
        div { class: "config-field config-field--json",
            div { class: "config-field__header",
                span { class: "field-label", "{label}" }
                if !hint.is_empty() {
                    span { class: "field-hint", "{hint}" }
                }
                div { class: "config-field__actions",
                    if !is_valid {
                        span { class: "json-parse-error", "JSON 格式错误" }
                    }
                    button {
                        class: "admin-btn admin-btn--ghost admin-btn--small",
                        onclick: move |_| {
                            if *view_mode.read() {
                                let json_str = serde_json::to_string(&*json_value.read())
                                    .unwrap_or_default();
                                on_change.call(json_str);
                            }
                            let new_mode = !*view_mode.read();
                            view_mode.set(new_mode);
                        },
                        if *view_mode.read() {
                            "切换到文本"
                        } else {
                            "切换到编辑器"
                        }
                    }
                }
            }

            if *view_mode.read() {
                div { class: "json-editor-container",
                    JsonTree {
                        label: "ROOT".to_string(),
                        value: json_value(),
                        mode: JsonTreeMode::Editable,
                        show_toolbar: true,
                        default_expand: ExpandState::All,
                        onchange: move |new_value: Value| {
                            json_value.set(new_value);
                            let json_str = serde_json::to_string(&*json_value.read()).unwrap_or_default();
                            on_change.call(json_str);
                        },
                    }
                }
            } else {
                textarea {
                    class: "admin-textarea admin-textarea--code",
                    rows: 6,
                    value: "{value}",
                    oninput: move |event| {
                        let new_value = event.value();
                        match serde_json::from_str::<Value>(&new_value) {
                            Ok(parsed) => {
                                parse_error.set(None);
                                json_value.set(parsed);
                            }
                            Err(e) => {
                                parse_error.set(Some(e.to_string()));
                            }
                        }
                        on_change.call(new_value);
                    },
                }
                if let Some(err) = parse_error() {
                    div { class: "json-error-message", "解析错误: {err}" }
                }
            }
        }
    }
}
