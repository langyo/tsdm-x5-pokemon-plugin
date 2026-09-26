use crate::dioxus::prelude::*;

use crate::dioxus::{
    components::pokemon_editor::PokemonTypeEditorModal,
    pages::shared::{update_filter_operator, FilterCollapsiblePanel, ListBottomDock},
    state::{
        set_busy, set_notice, AdminNoticeLevel, FilterPackage, ADMIN_BUSY, ADMIN_POKEMON_DATA,
    },
    utils::api::{
        count_pokemon_type, filter_pokemon_type, insert_pokemon_type, list_pokemon_type,
        set_pokemon_type,
    },
};
use _utils::types::pokemon_type::{PokemonAttributes, PokemonType};

#[component]
pub fn PokemonDataPage() -> Element {
    let state = ADMIN_POKEMON_DATA.read().clone();
    let rows = state.items.clone();
    let filters = state.filters.clone();
    let has_more = !state.is_filtered && state.loaded_count < state.total_count;
    let is_busy = *ADMIN_BUSY.read();
    let mut editor_data = use_signal(|| None::<PokemonType>);
    let mut show_filter_modal = use_signal(|| false);

    use_effect(move || {
        let state = ADMIN_POKEMON_DATA.read().clone();
        if state.initialized || state.loading {
            return;
        }
        reload_pokemons();
    });

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = js_sys::eval(
                r#"
                (function () {
                    const container = document.getElementById('pokemon-table-scroll');
                    if (!container) return;
                    if (container.dataset.autoLoadBound === '1') return;
                    container.dataset.autoLoadBound = '1';

                    let ticking = false;
                    const tryLoad = () => {
                        const trigger = document.getElementById('pokemon-load-more-trigger');
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
                    h2 { "宠物数据" }
                }
            }

            ListBottomDock {
                title: "筛选器",
                fields: filters.clone(),
                disabled: is_busy,
                total_count: state.total_count,
                loaded_count: state.loaded_count,
                is_filtered: state.is_filtered,
                create_title: "新增宠物",
                create_disabled: false,
                on_create: move |_| {
                    editor_data.set(Some(PokemonType::default()));
                },
                reload_disabled: false,
                on_reload: move |_| reload_pokemons(),
                load_more_disabled: state.is_filtered || state.loaded_count >= state.total_count,
                on_load_more: move |_| load_more_pokemons(),
                on_operator_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    let mut state = ADMIN_POKEMON_DATA.write();
                    update_filter_operator(&mut state.filters, index, &value);
                },
                on_value_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    if let Some(field) = ADMIN_POKEMON_DATA.write().filters.get_mut(index) {
                        field.value = value;
                    }
                },
                on_apply: move |_| apply_pokemon_filters(),
                on_reset: move |_| reset_pokemon_filters(),
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
                    let mut state = ADMIN_POKEMON_DATA.write();
                    update_filter_operator(&mut state.filters, index, &value);
                },
                on_value_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    if let Some(field) = ADMIN_POKEMON_DATA.write().filters.get_mut(index) {
                        field.value = value;
                    }
                },
                on_enabled_change: move |payload: (usize, bool)| {
                    let (index, enabled) = payload;
                    if let Some(field) = ADMIN_POKEMON_DATA.write().filters.get_mut(index) {
                        field.enabled = enabled;
                    }
                },
                on_apply: move |_| {
                    apply_pokemon_filters();
                    show_filter_modal.set(false);
                },
                on_reset: move |_| reset_pokemon_filters(),
            }

            div {
                id: "pokemon-table-scroll",
                class: "admin-card data-table-card",
                if !state.initialized {
                    div { class: "admin-table-loading",
                        div { class: "admin-table-loading__spinner" }
                    }
                } else if rows.is_empty() {
                    p { class: "empty-hint", "暂无宠物数据。" }
                } else {
                    table { class: "admin-table admin-table--pokemon",
                        thead {
                            tr {
                                th { "ID" }
                                th { "名称" }
                                th { "属性" }
                                th { "售价" }
                                th { "六维概览" }
                            }
                        }
                        tbody {
                            for item in rows {
                                PokemonRow {
                                    key: "pokemon-{item.id}",
                                    item,
                                    on_edit: move |payload: PokemonType| {
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
                        id: "pokemon-load-more-trigger",
                        class: "admin-load-more-trigger",
                        "data-can-load": if has_more { "1" } else { "0" },
                        "data-loading": if state.loading { "1" } else { "0" },
                        onclick: move |_| {
                            if !ADMIN_POKEMON_DATA.read().loading {
                                load_more_pokemons();
                            }
                        },
                        "load-more"
                    }
                }
            }

            // 使用精确编辑器替代 JSON 编辑器
            if let Some(data) = editor_data() {
                PokemonTypeEditorModal {
                    title: if data.id > 0 { format!("编辑宠物 #{}", data.id) } else { "新增宠物".to_string() },
                    data: data.clone(),
                    save_text: if data.id > 0 { "保存修改" } else { "创建记录" },
                    disabled: is_busy,
                    on_save: move |payload: PokemonType| {
                        editor_data.set(None);
                        save_pokemon_payload(payload);
                    },
                    on_close: move |_| editor_data.set(None),
                }
            }

        }
    }
}

#[component]
fn PokemonRow(item: PokemonType, on_edit: EventHandler<PokemonType>) -> Element {
    let item_for_edit = item.clone();
    let mut kind_tags = vec![item.kind.0.to_string()];
    if let Some(kind2) = item.kind.1 {
        kind_tags.push(kind2.to_string());
    };

    rsx! {
        tr {
            class: "admin-table-row--clickable",
            onclick: move |_| on_edit.call(item_for_edit.clone()),
            td { "{item.id}" }
            td { "{item.name}" }
            td {
                div { class: "pokemon-kind-tags",
                    for tag in kind_tags {
                        span { class: "pokemon-kind-tag", "{tag}" }
                    }
                    if item.is_legendary {
                        span { class: "pokemon-kind-tag pokemon-kind-tag--legendary",
                            "神兽"
                        }
                    }
                }
            }
            td {
                if item.is_selling {
                    "{item.cost}"
                } else {
                    span { class: "admin-text-muted", "不可出售" }
                }
            }
            td {
                PokemonStatGrid { stats: item.initial_statistic }
            }
        }
    }
}

#[component]
fn PokemonStatGrid(stats: PokemonAttributes) -> Element {
    let entries: [(&'static str, u64, &'static str); 6] = [
        ("HP", stats.hit_points as u64, "hp"),
        ("ATK", stats.attack as u64, "atk"),
        ("DEF", stats.defense as u64, "def"),
        ("SPA", stats.special_attack as u64, "spat"),
        ("SPD", stats.special_defense as u64, "spdef"),
        ("SPE", stats.speed as u64, "spd"),
    ];

    rsx! {
        div { class: "pokemon-stat-grid",
            for (label , value , key) in entries {
                div {
                    class: format!("pokemon-stat-item pokemon-stat-item--{}", key),
                    title: format!("{}: {} / 255", label, value),
                    span { class: "pokemon-stat-item__label", "{label}" }
                    div { class: "pokemon-stat-item__track",
                        div {
                            class: format!("pokemon-stat-item__fill pokemon-stat-item__fill--{}", key),
                            style: "width: {((value.min(255) as f32 / 255.0) * 100.0):.2}%",
                        }
                    }
                }
            }
        }
    }
}

fn reload_pokemons() {
    {
        let mut state = ADMIN_POKEMON_DATA.write();
        state.loading = true;
        state.is_filtered = false;
        state.items.clear();
        state.loaded_count = 0;
    }
    set_busy(true);

    spawn(async move {
        match (count_pokemon_type().await, list_pokemon_type(0, 100).await) {
            (Ok(total), Ok(items)) => {
                let mut state = ADMIN_POKEMON_DATA.write();
                state.total_count = total;
                state.loaded_count = items.len() as u64;
                state.items = items;
                state.initialized = true;
                state.loading = false;
                state.is_filtered = false;
                set_notice(AdminNoticeLevel::Info, "宠物数据已重新加载");
            }
            (Err(error), _) | (_, Err(error)) => {
                let mut state = ADMIN_POKEMON_DATA.write();
                state.initialized = true;
                state.loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("加载宠物数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn load_more_pokemons() {
    let (from, total) = {
        let mut state = ADMIN_POKEMON_DATA.write();
        state.loading = true;
        (state.loaded_count, state.total_count)
    };
    if from >= total {
        ADMIN_POKEMON_DATA.write().loading = false;
        return;
    }
    set_busy(true);

    spawn(async move {
        match list_pokemon_type(from, 100).await {
            Ok(mut items) => {
                let count = items.len() as u64;
                let mut state = ADMIN_POKEMON_DATA.write();
                state.items.append(&mut items);
                state.loaded_count += count;
                state.initialized = true;
                state.loading = false;
                set_notice(
                    AdminNoticeLevel::Info,
                    format!("宠物数据继续加载 {} 条", count),
                );
            }
            Err(error) => {
                ADMIN_POKEMON_DATA.write().loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("继续加载宠物数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn apply_pokemon_filters() {
    let filters: Vec<FilterPackage> = ADMIN_POKEMON_DATA
        .read()
        .filters
        .iter()
        .filter_map(|field| field.to_package())
        .collect();

    if filters.is_empty() {
        reload_pokemons();
        return;
    }

    ADMIN_POKEMON_DATA.write().loading = true;
    set_busy(true);

    spawn(async move {
        match filter_pokemon_type(filters).await {
            Ok(items) => {
                let count = items.len() as u64;
                let mut state = ADMIN_POKEMON_DATA.write();
                state.items = items;
                state.total_count = count;
                state.loaded_count = count;
                state.initialized = true;
                state.loading = false;
                state.is_filtered = true;
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("筛选得到 {} 条宠物记录", count),
                );
            }
            Err(error) => {
                ADMIN_POKEMON_DATA.write().loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("筛选宠物数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn reset_pokemon_filters() {
    {
        let mut state = ADMIN_POKEMON_DATA.write();
        for field in &mut state.filters {
            field.value.clear();
            field.enabled = false;
        }
    }
    reload_pokemons();
}

fn save_pokemon_payload(payload: PokemonType) {
    let is_new = payload.id == 0;

    set_busy(true);

    spawn(async move {
        let result = if is_new {
            insert_pokemon_type(payload).await
        } else {
            set_pokemon_type(payload).await
        };

        match result {
            Ok(saved) => {
                if is_new {
                    set_notice(
                        AdminNoticeLevel::Success,
                        format!("已新增宠物 #{}", saved.id),
                    );
                } else {
                    set_notice(
                        AdminNoticeLevel::Success,
                        format!("宠物 #{} 已更新", saved.id),
                    );
                }
                let mut state = ADMIN_POKEMON_DATA.write();
                if is_new {
                    state.items.insert(0, saved);
                    state.total_count += 1;
                    state.loaded_count += 1;
                } else if let Some(pos) = state.items.iter().position(|p| p.id == saved.id) {
                    state.items[pos] = saved;
                }
            }
            Err(error) => {
                set_notice(AdminNoticeLevel::Error, format!("保存宠物失败: {}", error));
            }
        }

        set_busy(false);
    });
}
