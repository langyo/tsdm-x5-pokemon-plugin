use crate::dioxus::prelude::*;

use crate::dioxus::{
    components::item_editor::ItemTypeEditorModal,
    pages::shared::{update_filter_operator, FilterCollapsiblePanel, ListBottomDock},
    state::{set_busy, set_notice, AdminNoticeLevel, FilterPackage, ADMIN_BUSY, ADMIN_ITEM_DATA},
    utils::api::{
        count_item_type, filter_item_type, insert_item_type, list_item_type, set_item_type,
    },
};
use _utils::types::item_type::{ItemTag, ItemType};

#[component]
pub fn ItemDataPage() -> Element {
    let state = ADMIN_ITEM_DATA.read().clone();
    let rows = state.items.clone();
    let filters = state.filters.clone();
    let has_more = !state.is_filtered && state.loaded_count < state.total_count;
    let is_busy = *ADMIN_BUSY.read();
    let mut editor_data = use_signal(|| None::<ItemType>);
    let mut show_filter_modal = use_signal(|| false);

    use_effect(move || {
        let state = ADMIN_ITEM_DATA.read().clone();
        if state.initialized || state.loading {
            return;
        }
        reload_items();
    });

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = js_sys::eval(
                r#"
                (function () {
                    const container = document.getElementById('item-table-scroll');
                    if (!container) return;
                    if (container.dataset.autoLoadBound === '1') return;
                    container.dataset.autoLoadBound = '1';
                    let ticking = false;
                    const tryLoad = () => {
                        const trigger = document.getElementById('item-load-more-trigger');
                        if (!trigger) return;
                        if (trigger.dataset.canLoad !== '1' || trigger.dataset.loading === '1') return;
                        const nearBottom = container.scrollTop + container.clientHeight >= container.scrollHeight - 48;
                        if (nearBottom) {
                            trigger.click();
                            setTimeout(tryLoad, 120);
                        }
                    };
                    container.addEventListener('scroll', () => {
                        if (ticking) return;
                        ticking = true;
                        requestAnimationFrame(() => {
                            ticking = false;
                            tryLoad();
                        });
                    });
                    setTimeout(tryLoad, 0);
                })();
                "#,
            );
        }
    });

    rsx! {
        section { class: "admin-page admin-data-page",
            div { class: "admin-page-header",
                div {
                    h2 { "道具数据" }
                }
            }

            ListBottomDock {
                title: "筛选器",
                fields: filters.clone(),
                disabled: is_busy,
                total_count: state.total_count,
                loaded_count: state.loaded_count,
                is_filtered: state.is_filtered,
                create_title: "新增道具",
                create_disabled: false,
                on_create: move |_| {
                    editor_data.set(Some(ItemType::default()));
                },
                reload_disabled: false,
                on_reload: move |_| reload_items(),
                load_more_disabled: state.is_filtered || state.loaded_count >= state.total_count,
                on_load_more: move |_| load_more_items(),
                on_operator_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    let mut state = ADMIN_ITEM_DATA.write();
                    update_filter_operator(&mut state.filters, index, &value);
                },
                on_value_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    if let Some(field) = ADMIN_ITEM_DATA.write().filters.get_mut(index) {
                        field.value = value;
                    }
                },
                on_apply: move |_| apply_item_filters(),
                on_reset: move |_| reset_item_filters(),
                on_filter_toggle: move |_| {
                    show_filter_modal.toggle();
                },
            }

            FilterCollapsiblePanel {
                fields: filters.clone(),
                disabled: is_busy,
                visible: show_filter_modal(),
                on_close: move |_| show_filter_modal.set(false),
                on_operator_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    let mut state = ADMIN_ITEM_DATA.write();
                    update_filter_operator(&mut state.filters, index, &value);
                },
                on_value_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    if let Some(field) = ADMIN_ITEM_DATA.write().filters.get_mut(index) {
                        field.value = value;
                    }
                },
                on_enabled_change: move |payload: (usize, bool)| {
                    let (index, enabled) = payload;
                    if let Some(field) = ADMIN_ITEM_DATA.write().filters.get_mut(index) {
                        field.enabled = enabled;
                    }
                },
                on_apply: move |_| {
                    apply_item_filters();
                    show_filter_modal.set(false);
                },
                on_reset: move |_| reset_item_filters(),
            }

            div { id: "item-table-scroll", class: "admin-card data-table-card",
                if !state.initialized {
                    div { class: "admin-table-loading",
                        div { class: "admin-table-loading__spinner" }
                    }
                } else if rows.is_empty() {
                    p { class: "empty-hint", "暂无道具数据。" }
                } else {
                    table { class: "admin-table admin-table--item",
                        thead {
                            tr {
                                th { "ID" }
                                th { "名称" }
                                th { "类型" }
                                th { "价格" }
                                th { "出售" }
                                th { "描述" }
                            }
                        }
                        tbody {
                            for item in rows {
                                ItemRow {
                                    key: "item-{item.id}",
                                    item,
                                    on_edit: move |payload: ItemType| {
                                        editor_data.set(Some(payload));
                                    },
                                }
                            }
                        }
                    }
                }

                if has_more {
                    div { class: "admin-load-more-indicator",
                        div { class: "admin-table-loading__spinner" }
                    }
                    button {
                        id: "item-load-more-trigger",
                        class: "admin-load-more-trigger",
                        "data-can-load": if has_more { "1" } else { "0" },
                        "data-loading": if state.loading { "1" } else { "0" },
                        onclick: move |_| {
                            if !ADMIN_ITEM_DATA.read().loading {
                                load_more_items();
                            }
                        },
                        "load-more"
                    }
                }
            }

            // 使用精确编辑器
            if let Some(data) = editor_data() {
                ItemTypeEditorModal {
                    title: if data.id > 0 { format!("编辑道具 #{}", data.id) } else { "新增道具".to_string() },
                    data: data.clone(),
                    save_text: if data.id > 0 { "保存修改" } else { "创建记录" },
                    disabled: is_busy,
                    on_save: move |payload: ItemType| {
                        editor_data.set(None);
                        save_item_payload(payload);
                    },
                    on_close: move |_| editor_data.set(None),
                }
            }

        }
    }
}

#[component]
fn ItemRow(item: ItemType, on_edit: EventHandler<ItemType>) -> Element {
    let item_for_edit = item.clone();

    rsx! {
        tr {
            class: "admin-table-row--clickable",
            onclick: move |_| on_edit.call(item_for_edit.clone()),
            td { "{item.id}" }
            td { "{item.name}" }
            td { "{item_tag_label(&item.tag)}" }
            td { "{item.price}" }
            td {
                if item.is_selling {
                    "是"
                } else {
                    "否"
                }
            }
            td { "{item.description}" }
        }
    }
}

fn reload_items() {
    {
        let mut state = ADMIN_ITEM_DATA.write();
        state.loading = true;
        state.is_filtered = false;
        state.items.clear();
        state.loaded_count = 0;
    }
    set_busy(true);

    spawn(async move {
        match (count_item_type().await, list_item_type(0, 100).await) {
            (Ok(total), Ok(items)) => {
                let mut state = ADMIN_ITEM_DATA.write();
                state.total_count = total;
                state.loaded_count = items.len() as u64;
                state.items = items;
                state.initialized = true;
                state.loading = false;
                state.is_filtered = false;
                set_notice(AdminNoticeLevel::Info, "道具数据已重新加载");
            }
            (Err(error), _) | (_, Err(error)) => {
                let mut state = ADMIN_ITEM_DATA.write();
                state.initialized = true;
                state.loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("加载道具数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn load_more_items() {
    let (from, total) = {
        let mut state = ADMIN_ITEM_DATA.write();
        state.loading = true;
        (state.loaded_count, state.total_count)
    };
    if from >= total {
        ADMIN_ITEM_DATA.write().loading = false;
        return;
    }
    set_busy(true);

    spawn(async move {
        match list_item_type(from, 100).await {
            Ok(mut items) => {
                let count = items.len() as u64;
                let mut state = ADMIN_ITEM_DATA.write();
                state.items.append(&mut items);
                state.loaded_count += count;
                state.initialized = true;
                state.loading = false;
                set_notice(
                    AdminNoticeLevel::Info,
                    format!("道具数据继续加载 {} 条", count),
                );
            }
            Err(error) => {
                ADMIN_ITEM_DATA.write().loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("继续加载道具数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn apply_item_filters() {
    let filters: Vec<FilterPackage> = ADMIN_ITEM_DATA
        .read()
        .filters
        .iter()
        .filter_map(|field| field.to_package())
        .collect();

    if filters.is_empty() {
        reload_items();
        return;
    }

    ADMIN_ITEM_DATA.write().loading = true;
    set_busy(true);

    spawn(async move {
        match filter_item_type(filters).await {
            Ok(items) => {
                let count = items.len() as u64;
                let mut state = ADMIN_ITEM_DATA.write();
                state.items = items;
                state.total_count = count;
                state.loaded_count = count;
                state.initialized = true;
                state.loading = false;
                state.is_filtered = true;
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("筛选得到 {} 条道具记录", count),
                );
            }
            Err(error) => {
                ADMIN_ITEM_DATA.write().loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("筛选道具数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn reset_item_filters() {
    {
        let mut state = ADMIN_ITEM_DATA.write();
        for field in &mut state.filters {
            field.value.clear();
            field.enabled = false;
        }
    }
    reload_items();
}

fn save_item_payload(payload: ItemType) {
    let is_new = payload.id == 0;

    set_busy(true);

    spawn(async move {
        let result = if is_new {
            insert_item_type(payload).await
        } else {
            set_item_type(payload).await
        };

        match result {
            Ok(saved) => {
                if is_new {
                    set_notice(
                        AdminNoticeLevel::Success,
                        format!("已新增道具 #{}", saved.id),
                    );
                } else {
                    set_notice(
                        AdminNoticeLevel::Success,
                        format!("道具 #{} 已更新", saved.id),
                    );
                }
                let mut state = ADMIN_ITEM_DATA.write();
                if is_new {
                    state.items.insert(0, saved);
                    state.total_count += 1;
                    state.loaded_count += 1;
                } else if let Some(pos) = state.items.iter().position(|i| i.id == saved.id) {
                    state.items[pos] = saved;
                }
            }
            Err(error) => {
                set_notice(AdminNoticeLevel::Error, format!("保存道具失败: {}", error));
            }
        }

        set_busy(false);
    });
}

fn item_tag_label(tag: &ItemTag) -> String {
    match tag {
        ItemTag::Drug => "药品".to_string(),
        ItemTag::Ball(id) => format!("宠物球 #{}", id),
        ItemTag::Evolution(id) => format!("升级素材 #{}", id),
        ItemTag::Enhance => "强化道具".to_string(),
        ItemTag::Armor(site) => format!("装备 {}", site),
        ItemTag::Special(text) => format!("特殊物品 {}", text),
    }
}
