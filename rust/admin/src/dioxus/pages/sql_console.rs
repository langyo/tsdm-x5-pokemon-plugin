use chrono::{Timelike, Utc};
use serde_json::Value;

use crate::dioxus::prelude::*;

use crate::dioxus::{
    components::{
        icon::{Icon, IconName},
        json_panel::JsonPanel,
    },
    state::{
        clear_notice, push_sql_history, set_busy, set_notice, set_sql_console_input,
        AdminNoticeLevel, SqlHistoryEntry, ADMIN_BUSY, ADMIN_SQL_CONSOLE,
    },
    utils::{api::run_sql, clipboard::copy_to_clipboard, code_editor::CodeEditor},
};

#[component]
pub fn SqlConsolePage() -> Element {
    let sql_state = ADMIN_SQL_CONSOLE.read().clone();
    let is_busy = *ADMIN_BUSY.read();
    let history = sql_state.history.clone();
    let mut sql_value = use_signal(|| sql_state.console.clone());

    rsx! {
        section { class: "admin-page admin-sql-page",
            div { class: "admin-page-header",
                div {
                    h2 { "SQL 控制台" }
                }
            }

            div { class: "admin-card admin-sql-editor",
                div { class: "sql-editor-header",
                    label { class: "field-label", "SQL 语句" }
                    div { class: "sql-editor-actions",
                        button {
                            class: "admin-btn admin-btn--ghost",
                            onclick: move |_| {
                                let example = "SELECT * FROM pm_config LIMIT 10;".to_string();
                                sql_value.set(example.clone());
                                set_sql_console_input(example);
                            },
                            "示例"
                        }
                    }
                }
                CodeEditor {
                    id: "sql-editor".to_string(),
                    value: sql_value(),
                    language: "sql".to_string(),
                    height: Some(180),
                    placeholder: Some("输入 SQL 语句...".to_string()),
                    onchange: move |value: String| {
                        sql_value.set(value.clone());
                        set_sql_console_input(value);
                    },
                }
                div { class: "admin-actions",
                    button {
                        class: "admin-btn admin-btn--primary",
                        disabled: is_busy,
                        onclick: move |_| {
                            let sql = sql_value.read().clone();
                            if sql.trim().is_empty() {
                                return;
                            }
                            set_busy(true);
                            clear_notice();
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
                        },
                        if is_busy {
                            "执行中..."
                        } else {
                            "执行 (Ctrl+Enter)"
                        }
                    }
                    button {
                        class: "admin-btn",
                        onclick: move |_| {
                            let sql = sql_value.read().clone();
                            if !sql.is_empty() {
                                copy_to_clipboard(&sql);
                                set_notice(AdminNoticeLevel::Info, "SQL 已复制到剪贴板");
                            }
                        },
                        "复制 SQL"
                    }
                    button {
                        class: "admin-btn",
                        onclick: move |_| {
                            sql_value.set(String::new());
                            set_sql_console_input(String::new());
                        },
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
                    p { class: "empty-hint",
                        "暂无执行记录。输入 SQL 语句并点击「执行」按钮开始。"
                    }
                } else {
                    div { class: "history-list",
                        for entry in history.into_iter().rev() {
                            SqlHistoryCard {
                                key: "sql-{entry.executed_at.timestamp_millis()}",
                                entry,
                            }
                        }
                    }
                }
            }
        }
    }
}

/// SQL 结果详情 Modal 的状态
#[derive(Clone, Debug, PartialEq)]
struct SqlResultDetail {
    title: String,
    result: String,
}

#[component]
fn SqlHistoryCard(entry: SqlHistoryEntry) -> Element {
    let timestamp = format!(
        "{:02}:{:02}:{:02}",
        entry.executed_at.hour(),
        entry.executed_at.minute(),
        entry.executed_at.second()
    );
    let is_ok = entry.result.is_ok();
    let sql_copy = entry.sql.clone();
    let mut detail_modal = use_signal(|| None::<SqlResultDetail>);

    // 解析 JSON 结果
    let (result_str, parsed_value, parse_error) = match &entry.result {
        Ok(result_str) => {
            let parse_result = serde_json::from_str::<Value>(result_str);
            let parse_error = parse_result.as_ref().err().map(|e| e.to_string());
            (result_str.clone(), parse_result.ok(), parse_error)
        }
        Err(error_str) => (error_str.clone(), None, None),
    };

    rsx! {
        details {
            class: if is_ok { "history-item is-success" } else { "history-item is-error" },
            open: true,
            summary { class: "history-summary",
                span { class: "history-status",
                    if is_ok {
                        Icon {
                            name: IconName::CircleCheck,
                            class: "admin-icon admin-icon--sm".to_string(),
                        }
                    } else {
                        Icon {
                            name: IconName::CircleX,
                            class: "admin-icon admin-icon--sm".to_string(),
                        }
                    }
                }
                span { class: "history-time", "{timestamp}" }
                span { class: "history-fulltime", "{entry.executed_at}" }
            }
            div { class: "history-body",
                div { class: "history-toolbar",
                    button {
                        class: "admin-btn admin-btn--ghost",
                        onclick: move |_| copy_to_clipboard(&sql_copy),
                        "复制 SQL"
                    }
                    if is_ok {
                        {
                            let result_copy = result_str.clone();
                            rsx! {
                                button {
                                    class: "admin-btn admin-btn--ghost",
                                    onclick: move |_| {
                                        copy_to_clipboard(&result_copy);
                                        set_notice(AdminNoticeLevel::Info, "结果 JSON 已复制到剪贴板");
                                    },
                                    "复制 JSON"
                                }
                            }
                        }
                    }
                }
                pre { class: "history-sql", "{entry.sql}" }

                // 显示 SQL 执行结果
                if is_ok {
                    // 成功执行的结果
                    if let Some(json_value) = parsed_value {
                        // 有效 JSON - 显示 JSON 面板
                        div { class: "history-result-wrapper",
                            div { class: "history-result-actions",
                                button {
                                    class: "admin-btn admin-btn--ghost admin-btn--small",
                                    onclick: move |_| {
                                        *detail_modal.write() = Some(SqlResultDetail {
                                            title: format!("SQL 结果 - {}", timestamp),
                                            result: result_str.clone(),
                                        });
                                    },
                                    "放大查看"
                                }
                            }
                            JsonPanel { value: json_value }
                        }
                    } else {
                        // 无效 JSON - 显示警告
                        div { class: "history-result history-result--warning",
                            strong { "结果不是有效的 JSON 格式" }
                            if let Some(ref err) = parse_error {
                                p { class: "parse-error-msg", "解析错误：{err}" }
                            }
                            pre { "{result_str}" }
                        }
                    }
                } else {
                    // 执行失败
                    div { class: "history-result history-result--error",
                        strong { "执行失败" }
                        p { "{result_str}" }
                    }
                }
            }
        }
        // 详情 Modal
        if let Some(detail) = detail_modal() {
            SqlResultDetailModal {
                title: detail.title,
                result: detail.result,
                on_close: move |_| *detail_modal.write() = None,
            }
        }
    }
}

/// SQL 结果详情 Modal
#[component]
fn SqlResultDetailModal(title: String, result: String, on_close: EventHandler<()>) -> Element {
    // 解析 JSON
    let parsed_value = serde_json::from_str::<Value>(&result).ok();

    rsx! {
        div { class: "admin-action-modal",
            // 背景遮罩
            button {
                class: "admin-action-modal__backdrop",
                onclick: move |_| on_close.call(()),
            }
            // Modal 面板
            div { class: "admin-action-modal__panel admin-action-modal__panel--sql-detail",
                // 头部
                div { class: "admin-action-modal__header",
                    h3 { "{title}" }
                    button {
                        class: "admin-action-modal__close-btn",
                        onclick: move |_| on_close.call(()),
                        Icon {
                            name: IconName::X,
                            class: "admin-icon".to_string(),
                        }
                    }
                }
                // 内容区域
                div { class: "admin-action-modal__body sql-detail-body",
                    if let Some(json_value) = parsed_value {
                        JsonPanel { value: json_value, in_modal: true }
                    } else {
                        pre { class: "result-raw-view result-raw-view--full", "{result}" }
                    }
                }
            }
        }
    }
}
