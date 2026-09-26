use crate::dioxus::prelude::*;

use crate::dioxus::{ state::{ begin_file_explorer_request, finish_directory_load, finish_file_explorer_attempt, finish_file_load, set_busy, set_notice, AdminNoticeLevel, ExplorerContent, ExplorerItemType, ADMIN_BUSY, ADMIN_FILE_EXPLORER, }, utils::{ api::{get_file, list_file}, clipboard::copy_to_clipboard, }, };

#[component]
pub fn FileExplorerPage() -> Element {
    let state = ADMIN_FILE_EXPLORER.read().clone();
    let is_busy = *ADMIN_BUSY.read();

    use_effect(move || {
        let state = ADMIN_FILE_EXPLORER.read().clone();
        if state.initialized || state.loading {
            return;
        }
        load_directory(Vec::new());
    });

    let breadcrumb_dirs = state.path.clone();
    let breadcrumb_items: Vec<(usize, String, Vec<String>)> = breadcrumb_dirs
        .iter()
        .enumerate()
        .map(|(index, item)| (index, item.clone(), breadcrumb_dirs[..=index].to_vec()))
        .collect();
    let selected_file = state.selected_file.clone();
    let content = state.content.clone();

    rsx! {
        section { class: "admin-page admin-file-page",
            div { class: "admin-page-header",
                div {
                    h2 { "文件管理器" }
                }
            }

            div { class: "admin-card admin-file-toolbar",
                div { class: "admin-actions",
                    button {
                        class: "admin-btn admin-btn--primary",
                        disabled: is_busy,
                        onclick: move |_| refresh_current(),
                        if is_busy {
                            "刷新中..."
                        } else {
                            "刷新"
                        }
                    }
                    button {
                        class: "admin-btn",
                        disabled: is_busy,
                        onclick: move |_| load_directory(Vec::new()),
                        "返回根目录"
                    }
                }
                div { class: "breadcrumb-bar",
                    button {
                        class: "breadcrumb-item",
                        disabled: is_busy,
                        onclick: move |_| load_directory(Vec::new()),
                        "/"
                    }
                    for (index , item , path_slice) in breadcrumb_items {
                        button {
                            key: "dir-{index}",
                            class: "breadcrumb-item",
                            disabled: is_busy,
                            onclick: move |_| load_directory(path_slice.clone()),
                            "{item}"
                        }
                    }
                    if let Some(file_name) = selected_file {
                        span { class: "breadcrumb-current", "{file_name}" }
                    }
                }
            }

            div { class: "admin-card admin-file-content",
                match content {
                    ExplorerContent::Directory(entries) => rsx! {
                        if entries.is_empty() {
                            p { class: "empty-hint", "当前目录为空。" }
                        } else {
                            div { class: "file-grid",
                                for (name , item_type) in entries {
                                    FileEntryCard {
                                        key: "entry-{name}",
                                        name,
                                        item_type,
                                        current_path: state.path.clone(),
                                        disabled: is_busy,
                                    }
                                }
                            }
                        }
                    },
                    ExplorerContent::File(text) => {
                        let copied_text = text.clone();
                        rsx! {
                            div { class: "file-viewer",
                                div { class: "history-toolbar",
                                    button {
                                        class: "admin-btn admin-btn--ghost",
                                        onclick: move |_| {
                                            copy_to_clipboard(&copied_text);
                                            set_notice(AdminNoticeLevel::Success, "文件内容已复制到剪贴板");
                                        },
                                        "复制文件内容"
                                    }
                                }
                                textarea {
                                    class: "admin-textarea admin-textarea--file",
                                    rows: "24",
                                    readonly: true,
                                    value: "{text}",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FileEntryCard(
    name: String,
    item_type: ExplorerItemType,
    current_path: Vec<String>,
    disabled: bool,
) -> Element {
    let icon = match item_type {
        ExplorerItemType::Directory => "DIR",
        ExplorerItemType::File => "FILE",
    };
    let class_name = match item_type {
        ExplorerItemType::Directory => "file-entry file-entry--dir",
        ExplorerItemType::File => "file-entry file-entry--file",
    };
    let target_name = name.clone();

    rsx! {
        button {
            class: class_name,
            disabled,
            onclick: move |_| {
                let mut next_path = current_path.clone();
                next_path.push(target_name.clone());
                match item_type {
                    ExplorerItemType::Directory => load_directory(next_path),
                    ExplorerItemType::File => {
                        open_file(current_path.clone(), target_name.clone())
                    }
                }
            },
            span { class: "file-entry-icon", "{icon}" }
            span { class: "file-entry-name", "{name}" }
        }
    }
}

fn refresh_current() {
    let state = ADMIN_FILE_EXPLORER.read().clone();
    if let Some(file_name) = state.selected_file {
        open_file(state.path, file_name);
    } else {
        load_directory(state.path);
    }
}

fn load_directory(path: Vec<String>) {
    begin_file_explorer_request();
    set_busy(true);

    spawn(async move {
        match list_file(path.clone()).await {
            Ok(entries) => {
                finish_directory_load(path, entries);
                set_notice(AdminNoticeLevel::Info, "目录已刷新");
            }
            Err(error) => {
                finish_file_explorer_attempt();
                set_notice(AdminNoticeLevel::Error, format!("读取目录失败: {}", error));
            }
        }
        set_busy(false);
    });
}

fn open_file(directory_path: Vec<String>, file_name: String) {
    begin_file_explorer_request();
    set_busy(true);

    let mut file_path = directory_path.clone();
    file_path.push(file_name.clone());

    spawn(async move {
        match get_file(file_path).await {
            Ok(content) => {
                finish_file_load(directory_path, file_name, content);
            }
            Err(error) => {
                finish_file_explorer_attempt();
                set_notice(AdminNoticeLevel::Error, format!("读取文件失败: {}", error));
            }
        }
        set_busy(false);
    });
}
