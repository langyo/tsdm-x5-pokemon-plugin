use serde_json::Value;

use crate::dioxus::prelude::*;

use crate::dioxus::utils::{
    clipboard::copy_to_clipboard,
    json_tree::{ExpandState, JsonTree},
};

/// JSON 面板视图模式
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum JsonPanelViewMode {
    #[default]
    Tree,
    Text,
}

/// JSON 面板组件
///
/// 用于显示 JSON 数据，支持树形视图和纯文本视图切换
#[component]
pub fn JsonPanel(
    /// JSON 数据
    value: Value,
    /// 面板标题
    #[props(default)]
    title: Option<String>,
    /// 默认视图模式
    #[props(default)]
    default_mode: JsonPanelViewMode,
    /// 是否显示工具栏
    #[props(default = true)]
    show_toolbar: bool,
    /// 最大高度（像素）
    #[props(default)]
    max_height: Option<usize>,
    /// 是否在模态框中使用（会调整样式）
    #[props(default)]
    in_modal: bool,
) -> Element {
    let mut view_mode = use_signal(|| default_mode);

    // 转换为 JSON 字符串用于文本视图
    let json_str =
        serde_json::to_string_pretty(&value).unwrap_or_else(|_| "无法序列化 JSON".to_string());
    let json_size = json_str.len();

    // 计算大小显示
    let size_display = if json_size > 1024 * 1024 {
        format!("{:.2} MB", json_size as f64 / 1024.0 / 1024.0)
    } else if json_size > 1024 {
        format!("{:.2} KB", json_size as f64 / 1024.0)
    } else {
        format!("{} B", json_size)
    };

    let max_height_style = if let Some(h) = max_height {
        format!("max-height: {}px;", h)
    } else {
        String::new()
    };

    rsx! {
        div { class: "json-panel",
            // 标题栏
            if let Some(t) = title {
                div { class: "json-panel__header",
                    h4 { class: "json-panel__title", "{t}" }
                    span { class: "json-panel__meta", "{size_display}" }
                }
            }

            // 工具栏
            if show_toolbar {
                div { class: "json-panel__toolbar",
                    div { class: "json-panel__view-switch",
                        button {
                            class: if *view_mode.read() == JsonPanelViewMode::Tree { "json-panel__view-btn json-panel__view-btn--active" } else { "json-panel__view-btn" },
                            onclick: move |_| *view_mode.write() = JsonPanelViewMode::Tree,
                            "树形"
                        }
                        button {
                            class: if *view_mode.read() == JsonPanelViewMode::Text { "json-panel__view-btn json-panel__view-btn--active" } else { "json-panel__view-btn" },
                            onclick: move |_| *view_mode.write() = JsonPanelViewMode::Text,
                            "纯文本"
                        }
                    }
                    div { class: "json-panel__actions",
                        button {
                            class: "admin-btn admin-btn--ghost admin-btn--small",
                            onclick: {
                                let json_str = json_str.clone();
                                move |_| {
                                    copy_to_clipboard(&json_str);
                                }
                            },
                            "复制 JSON"
                        }
                    }
                }
            }

            // 内容区域
            div { class: "json-panel__body", style: "{max_height_style}",
                match *view_mode.read() {
                    JsonPanelViewMode::Tree => rsx! {
                        div { class: "json-panel__tree-view",
                            JsonTree {
                                label: "ROOT".to_string(),
                                value: value.clone(),
                                show_toolbar: false,
                                default_expand: ExpandState::All,
                            }
                        }
                    },
                    JsonPanelViewMode::Text => rsx! {
                        pre { class: "json-panel__text-view",
                            code { "{json_str}" }
                        }
                    },
                }
            }
        }
    }
}
