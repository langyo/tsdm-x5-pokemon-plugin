use crate::dioxus::prelude::*;

use crate::dioxus::{
    components::skill_editor::SkillTypeEditorModal,
    pages::shared::{update_filter_operator, FilterCollapsiblePanel, ListBottomDock},
    state::{set_busy, set_notice, AdminNoticeLevel, FilterPackage, ADMIN_BUSY, ADMIN_SKILL_DATA},
    utils::api::{
        count_skill_type, filter_skill_type, insert_skill_type, list_skill_type, set_skill_type,
    },
};
use _utils::types::skill_type::SkillType;

#[component]
pub fn SkillDataPage() -> Element {
    let state = ADMIN_SKILL_DATA.read().clone();
    let rows = state.items.clone();
    let filters = state.filters.clone();
    let has_more = !state.is_filtered && state.loaded_count < state.total_count;
    let is_busy = *ADMIN_BUSY.read();
    let mut editor_data = use_signal(|| None::<SkillType>);
    let mut show_filter_modal = use_signal(|| false);

    use_effect(move || {
        let state = ADMIN_SKILL_DATA.read().clone();
        if state.initialized || state.loading {
            return;
        }
        reload_skills();
    });

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = js_sys::eval(
                r#"
                (function () {
                    const container = document.getElementById('skill-table-scroll');
                    if (!container) return;
                    if (container.dataset.autoLoadBound === '1') return;
                    container.dataset.autoLoadBound = '1';
                    let ticking = false;
                    const tryLoad = () => {
                        const trigger = document.getElementById('skill-load-more-trigger');
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
                    h2 { "技能数据" }
                }
            }

            ListBottomDock {
                title: "筛选器",
                fields: filters.clone(),
                disabled: is_busy,
                total_count: state.total_count,
                loaded_count: state.loaded_count,
                is_filtered: state.is_filtered,
                create_title: "新增技能",
                create_disabled: false,
                on_create: move |_| {
                    editor_data.set(Some(SkillType::default()));
                },
                reload_disabled: false,
                on_reload: move |_| reload_skills(),
                load_more_disabled: state.is_filtered || state.loaded_count >= state.total_count,
                on_load_more: move |_| load_more_skills(),
                on_operator_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    let mut state = ADMIN_SKILL_DATA.write();
                    update_filter_operator(&mut state.filters, index, &value);
                },
                on_value_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    if let Some(field) = ADMIN_SKILL_DATA.write().filters.get_mut(index) {
                        field.value = value;
                    }
                },
                on_apply: move |_| apply_skill_filters(),
                on_reset: move |_| reset_skill_filters(),
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
                    let mut state = ADMIN_SKILL_DATA.write();
                    update_filter_operator(&mut state.filters, index, &value);
                },
                on_value_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    if let Some(field) = ADMIN_SKILL_DATA.write().filters.get_mut(index) {
                        field.value = value;
                    }
                },
                on_enabled_change: move |payload: (usize, bool)| {
                    let (index, enabled) = payload;
                    if let Some(field) = ADMIN_SKILL_DATA.write().filters.get_mut(index) {
                        field.enabled = enabled;
                    }
                },
                on_apply: move |_| {
                    apply_skill_filters();
                    show_filter_modal.set(false);
                },
                on_reset: move |_| reset_skill_filters(),
            }

            div { id: "skill-table-scroll", class: "admin-card data-table-card",
                if !state.initialized {
                    div { class: "admin-table-loading",
                        div { class: "admin-table-loading__spinner" }
                    }
                } else if rows.is_empty() {
                    p { class: "empty-hint", "暂无技能数据。" }
                } else {
                    table { class: "admin-table admin-table--skill",
                        thead {
                            tr {
                                th { "ID" }
                                th { "名称" }
                                th { "最低等级" }
                                th { "充能次数" }
                                th { "可用种族数" }
                            }
                        }
                        tbody {
                            for item in rows {
                                SkillRow {
                                    key: "skill-{item.id}",
                                    item,
                                    on_edit: move |payload: SkillType| {
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
                        id: "skill-load-more-trigger",
                        class: "admin-load-more-trigger",
                        "data-can-load": if has_more { "1" } else { "0" },
                        "data-loading": if state.loading { "1" } else { "0" },
                        onclick: move |_| {
                            if !ADMIN_SKILL_DATA.read().loading {
                                load_more_skills();
                            }
                        },
                        "load-more"
                    }
                }
            }

            // 使用精确编辑器
            if let Some(data) = editor_data() {
                SkillTypeEditorModal {
                    title: if data.id > 0 { format!("编辑技能 #{}", data.id) } else { "新增技能".to_string() },
                    data: data.clone(),
                    save_text: if data.id > 0 { "保存修改" } else { "创建记录" },
                    disabled: is_busy,
                    on_save: move |payload: SkillType| {
                        editor_data.set(None);
                        save_skill_payload(payload);
                    },
                    on_close: move |_| editor_data.set(None),
                }
            }

        }
    }
}

#[component]
fn SkillRow(item: SkillType, on_edit: EventHandler<SkillType>) -> Element {
    let item_for_edit = item.clone();

    rsx! {
        tr {
            class: "admin-table-row--clickable",
            onclick: move |_| on_edit.call(item_for_edit.clone()),
            td { "{item.id}" }
            td { "{item.name}" }
            td { "{item.min_level_limit}" }
            td { "{item.use_times_limit}" }
            td { "{item.available_pokemons.len()}" }
        }
    }
}

fn reload_skills() {
    {
        let mut state = ADMIN_SKILL_DATA.write();
        state.loading = true;
        state.is_filtered = false;
        state.items.clear();
        state.loaded_count = 0;
    }
    set_busy(true);

    spawn(async move {
        match (count_skill_type().await, list_skill_type(0, 100).await) {
            (Ok(total), Ok(items)) => {
                let mut state = ADMIN_SKILL_DATA.write();
                state.total_count = total;
                state.loaded_count = items.len() as u64;
                state.items = items;
                state.initialized = true;
                state.loading = false;
                state.is_filtered = false;
                set_notice(AdminNoticeLevel::Info, "技能数据已重新加载");
            }
            (Err(error), _) | (_, Err(error)) => {
                let mut state = ADMIN_SKILL_DATA.write();
                state.initialized = true;
                state.loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("加载技能数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn load_more_skills() {
    let (from, total) = {
        let mut state = ADMIN_SKILL_DATA.write();
        state.loading = true;
        (state.loaded_count, state.total_count)
    };
    if from >= total {
        ADMIN_SKILL_DATA.write().loading = false;
        return;
    }
    set_busy(true);

    spawn(async move {
        match list_skill_type(from, 100).await {
            Ok(mut items) => {
                let count = items.len() as u64;
                let mut state = ADMIN_SKILL_DATA.write();
                state.items.append(&mut items);
                state.loaded_count += count;
                state.initialized = true;
                state.loading = false;
                set_notice(
                    AdminNoticeLevel::Info,
                    format!("技能数据继续加载 {} 条", count),
                );
            }
            Err(error) => {
                ADMIN_SKILL_DATA.write().loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("继续加载技能数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn apply_skill_filters() {
    let filters: Vec<FilterPackage> = ADMIN_SKILL_DATA
        .read()
        .filters
        .iter()
        .filter_map(|field| field.to_package())
        .collect();

    if filters.is_empty() {
        reload_skills();
        return;
    }

    ADMIN_SKILL_DATA.write().loading = true;
    set_busy(true);

    spawn(async move {
        match filter_skill_type(filters).await {
            Ok(items) => {
                let count = items.len() as u64;
                let mut state = ADMIN_SKILL_DATA.write();
                state.items = items;
                state.total_count = count;
                state.loaded_count = count;
                state.initialized = true;
                state.loading = false;
                state.is_filtered = true;
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("筛选得到 {} 条技能记录", count),
                );
            }
            Err(error) => {
                ADMIN_SKILL_DATA.write().loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("筛选技能数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn reset_skill_filters() {
    {
        let mut state = ADMIN_SKILL_DATA.write();
        for field in &mut state.filters {
            field.value.clear();
            field.enabled = false;
        }
    }
    reload_skills();
}

fn save_skill_payload(payload: SkillType) {
    let is_new = payload.id == 0;

    set_busy(true);
    spawn(async move {
        let result = if is_new {
            insert_skill_type(payload).await
        } else {
            set_skill_type(payload).await
        };

        match result {
            Ok(saved) => {
                if is_new {
                    set_notice(
                        AdminNoticeLevel::Success,
                        format!("技能 #{} 已创建", saved.id),
                    );
                } else {
                    set_notice(
                        AdminNoticeLevel::Success,
                        format!("技能 #{} 已更新", saved.id),
                    );
                }
                let mut state = ADMIN_SKILL_DATA.write();
                if is_new {
                    state.items.insert(0, saved);
                    state.total_count += 1;
                    state.loaded_count += 1;
                } else if let Some(pos) = state.items.iter().position(|s| s.id == saved.id) {
                    state.items[pos] = saved;
                }
            }
            Err(error) => {
                set_notice(AdminNoticeLevel::Error, format!("保存技能失败: {}", error));
                set_busy(false);
            }
        }
    });
}
