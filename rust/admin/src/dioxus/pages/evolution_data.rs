use crate::dioxus::prelude::*;

use crate::dioxus::{
    components::evolution_editor::EvolutionInfoEditorModal,
    pages::shared::{update_filter_operator, FilterCollapsiblePanel, ListBottomDock},
    state::{
        set_busy, set_notice, AdminNoticeLevel, FilterPackage, ADMIN_BUSY, ADMIN_EVOLUTION_DATA,
    },
    utils::api::{
        count_evolution_info, filter_evolution_info, insert_evolution_info, list_evolution_info,
        set_evolution_info,
    },
};
use _utils::types::{
    evolution_info::{EvolutionCompareType, EvolutionInfo, EvolutionLimitType},
    pokemon_type::PokemonSex,
};

#[component]
pub fn EvolutionDataPage() -> Element {
    let state = ADMIN_EVOLUTION_DATA.read().clone();
    let rows = state.items.clone();
    let filters = state.filters.clone();
    let has_more = !state.is_filtered && state.loaded_count < state.total_count;
    let is_busy = *ADMIN_BUSY.read();
    let mut editor_data = use_signal(|| None::<EvolutionInfo>);
    let mut show_filter_modal = use_signal(|| false);

    use_effect(move || {
        let state = ADMIN_EVOLUTION_DATA.read().clone();

        if state.initialized || state.loading {
            return;
        }

        reload_evolution();
    });

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = js_sys::eval(
                r#"
                (function () {
                    const container = document.getElementById('evolution-table-scroll');
                    if (!container) return;
                    if (container.dataset.autoLoadBound === '1') return;
                    container.dataset.autoLoadBound = '1';
                    let ticking = false;
                    const tryLoad = () => {
                        const trigger = document.getElementById('evolution-load-more-trigger');
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
                    h2 { "进化路线" }
                }
            }

            ListBottomDock {

                title: "筛选器",
                fields: filters.clone(),
                disabled: is_busy,
                total_count: state.total_count,
                loaded_count: state.loaded_count,
                is_filtered: state.is_filtered,
                create_title: "新增规则",
                create_disabled: false,
                on_create: move |_| {
                    editor_data.set(Some(EvolutionInfo::default()));
                },
                reload_disabled: false,
                on_reload: move |_| reload_evolution(),
                load_more_disabled: state.is_filtered || state.loaded_count >= state.total_count,
                on_load_more: move |_| load_more_evolution(),
                on_operator_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    let mut state = ADMIN_EVOLUTION_DATA.write();
                    update_filter_operator(&mut state.filters, index, &value);
                },
                on_value_change: move |payload: (usize, String)| {
                    let (index, value) = payload;

                    if let Some(field) = ADMIN_EVOLUTION_DATA.write().filters.get_mut(index) {
                        field.value = value;
                    }
                },
                on_apply: move |_| apply_evolution_filters(),
                on_reset: move |_| reset_evolution_filters(),
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
                    let mut state = ADMIN_EVOLUTION_DATA.write();
                    update_filter_operator(&mut state.filters, index, &value);
                },
                on_value_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    if let Some(field) = ADMIN_EVOLUTION_DATA.write().filters.get_mut(index) {
                        field.value = value;
                    }
                },
                on_enabled_change: move |payload: (usize, bool)| {
                    let (index, enabled) = payload;
                    if let Some(field) = ADMIN_EVOLUTION_DATA.write().filters.get_mut(index) {
                        field.enabled = enabled;
                    }
                },
                on_apply: move |_| {
                    apply_evolution_filters();
                    show_filter_modal.set(false);
                },
                on_reset: move |_| reset_evolution_filters(),
            }

            div {
                id: "evolution-table-scroll",
                class: "admin-card data-table-card",
                if !state.initialized {
                    div { class: "admin-table-loading",
                        div { class: "admin-table-loading__spinner" }
                    }
                } else if rows.is_empty() {
                    p { class: "empty-hint", "暂无进化规则数据。" }
                } else {
                    table { class: "admin-table admin-table--evolution",
                        thead {
                            tr {
                                th { "ID" }
                                th { "来源" }
                                th { "目标" }
                                th { "规则" }
                                th { "优先级" }
                            }
                        }

                        tbody {
                            for item in rows {
                                EvolutionRow {
                                    key: "evo-{item.id}",
                                    item,
                                    on_edit: move |payload: EvolutionInfo| {
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
                        id: "evolution-load-more-trigger",
                        class: "admin-load-more-trigger",
                        "data-can-load": if has_more { "1" } else { "0" },
                        "data-loading": if state.loading { "1" } else { "0" },
                        onclick: move |_| {
                            if !ADMIN_EVOLUTION_DATA.read().loading {
                                load_more_evolution();
                            }
                        },
                        "load-more"
                    }
                }
            }

            // 使用精确编辑器
            if let Some(data) = editor_data() {
                EvolutionInfoEditorModal {
                    title: if data.id > 0 { format!("编辑进化规则 #{}", data.id) } else { "新增进化规则".to_string() },
                    data: data.clone(),
                    save_text: if data.id > 0 { "保存修改" } else { "创建记录" },
                    disabled: is_busy,
                    on_save: move |payload: EvolutionInfo| {
                        editor_data.set(None);
                        save_evolution_payload(payload);
                    },
                    on_close: move |_| editor_data.set(None),
                }
            }

        }
    }
}

#[component]
fn EvolutionRow(item: EvolutionInfo, on_edit: EventHandler<EvolutionInfo>) -> Element {
    let item_for_edit = item.clone();

    let source_display = if item.source_name.is_empty() {
        format!("#{}", item.source_id)
    } else {
        format!("{} (#{})", item.source_name, item.source_id)
    };

    let target_display = if item.target_name.is_empty() {
        format!("#{}", item.target_id)
    } else {
        format!("{} (#{})", item.target_name, item.target_id)
    };

    rsx! {
        tr {
            class: "admin-table-row--clickable",
            onclick: move |_| on_edit.call(item_for_edit.clone()),
            td { "{item.id}" }
            td { "{source_display}" }
            td { "{target_display}" }
            td { "{evolution_rule_label(&item.condition)}" }
            td { "{item.priority}" }
        }
    }
}

fn save_evolution_payload(payload: EvolutionInfo) {
    let is_new = payload.id == 0;

    set_busy(true);

    spawn(async move {
        let result = if is_new {
            insert_evolution_info(payload).await
        } else {
            set_evolution_info(payload).await
        };

        match result {
            Ok(saved) => {
                if is_new {
                    set_notice(
                        AdminNoticeLevel::Success,
                        format!("进化规则 #{} 已创建", saved.id),
                    );
                } else {
                    set_notice(
                        AdminNoticeLevel::Success,
                        format!("进化规则 #{} 已更新", saved.id),
                    );
                }

                let mut state = ADMIN_EVOLUTION_DATA.write();
                if is_new {
                    state.items.insert(0, saved);
                    state.total_count += 1;
                    state.loaded_count += 1;
                } else if let Some(pos) = state.items.iter().position(|e| e.id == saved.id) {
                    state.items[pos] = saved;
                }
            }

            Err(error) => {
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("保存进化规则失败: {}", error),
                );
                set_busy(false);
            }
        }
    });
}

fn reload_evolution() {
    {
        let mut state = ADMIN_EVOLUTION_DATA.write();
        state.loading = true;
        state.is_filtered = false;
        state.items.clear();
        state.loaded_count = 0;
    }

    set_busy(true);

    spawn(async move {
        match (
            count_evolution_info().await,
            list_evolution_info(0, 100).await,
        ) {
            (Ok(total), Ok(items)) => {
                let mut state = ADMIN_EVOLUTION_DATA.write();
                state.total_count = total;
                state.loaded_count = items.len() as u64;
                state.items = items;
                state.initialized = true;
                state.loading = false;
                state.is_filtered = false;
                set_notice(AdminNoticeLevel::Info, "进化规则数据已重新加载");
            }

            (Err(error), _) | (_, Err(error)) => {
                let mut state = ADMIN_EVOLUTION_DATA.write();
                state.initialized = true;
                state.loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("加载进化规则失败: {}", error),
                );
            }
        }

        set_busy(false);
    });
}

fn load_more_evolution() {
    let (from, total) = {
        let mut state = ADMIN_EVOLUTION_DATA.write();
        state.loading = true;
        (state.loaded_count, state.total_count)
    };

    if from >= total {
        ADMIN_EVOLUTION_DATA.write().loading = false;
        return;
    }

    set_busy(true);

    spawn(async move {
        match list_evolution_info(from, 100).await {
            Ok(mut items) => {
                let count = items.len() as u64;
                let mut state = ADMIN_EVOLUTION_DATA.write();
                state.items.append(&mut items);
                state.loaded_count += count;
                state.initialized = true;
                state.loading = false;
                set_notice(
                    AdminNoticeLevel::Info,
                    format!("进化规则继续加载 {} 条", count),
                );
            }

            Err(error) => {
                ADMIN_EVOLUTION_DATA.write().loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("继续加载进化规则失败: {}", error),
                );
            }
        }

        set_busy(false);
    });
}

fn apply_evolution_filters() {
    let filters: Vec<FilterPackage> = ADMIN_EVOLUTION_DATA
        .read()
        .filters
        .iter()
        .filter_map(|field| field.to_package())
        .collect();

    if filters.is_empty() {
        reload_evolution();
        return;
    }

    ADMIN_EVOLUTION_DATA.write().loading = true;
    set_busy(true);

    spawn(async move {
        match filter_evolution_info(filters).await {
            Ok(items) => {
                let count = items.len() as u64;
                let mut state = ADMIN_EVOLUTION_DATA.write();
                state.items = items;
                state.total_count = count;
                state.loaded_count = count;
                state.initialized = true;
                state.loading = false;
                state.is_filtered = true;
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("筛选得到 {} 条进化记录", count),
                );
            }

            Err(error) => {
                ADMIN_EVOLUTION_DATA.write().loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("筛选进化规则失败: {}", error),
                );
            }
        }

        set_busy(false);
    });
}

fn reset_evolution_filters() {
    {
        let mut state = ADMIN_EVOLUTION_DATA.write();

        for field in &mut state.filters {
            field.value.clear();
            field.enabled = false;
        }
    }

    reload_evolution();
}

fn evolution_rule_label(rule: &EvolutionLimitType) -> String {
    match rule {
        EvolutionLimitType::MinLevel(v) => format!("最低等级 Lv.{}", v.min_level),
        EvolutionLimitType::UseItem(v) => {
            if v.item_name.is_empty() {
                format!("使用道具 #{}", v.use_item)
            } else {
                format!("使用道具: {}", v.item_name)
            }
        }
        EvolutionLimitType::MinIntimacy(v) => format!("亲密度达到 {}", v.min_intimacy),
        EvolutionLimitType::CompareAttackAndDefense(v) => match v.compare_attack_and_defense {
            EvolutionCompareType::Equal => "攻击和防御相等".to_string(),
            EvolutionCompareType::Greater => "攻击大于防御".to_string(),
            EvolutionCompareType::Less => "攻击小于防御".to_string(),
        },

        EvolutionLimitType::Random(v) => format!("随机概率 {:.0}%", v.random * 100.0),
        EvolutionLimitType::Sex(v) => match v.sex {
            PokemonSex::Male => "雄性".to_string(),
            PokemonSex::Female => "雌性".to_string(),
            PokemonSex::Unknown => "未知性别".to_string(),
        },

        EvolutionLimitType::IsBagHaveChairs(v) => if v.is_bag_have_chairs {
            "背包有空位"
        } else {
            "背包没有空位"
        }
        .to_string(),
    }
}
