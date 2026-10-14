use crate::prelude::*;

use crate::{
    components::{
        common::{use_popup, PopupContext, PopupMenu, PopupMenuItem},
        layout::IMG_PATH_REMOTE,
    },
    state::{
        show_error, show_success, use_pokemon_state, use_user_profile_state, POKEMON_STATE,
        USER_STATE,
    },
    utils::{
        api_client::NewApiClient,
        pokemon::{hp_class_storage, hp_health_status, hp_percent, is_weak_state},
        storage_filters::{
            matching_storage_ids, parse_level_range, selected_matching_ids, StorageEntry,
            StorageFilters, StorageSort,
        },
    },
};
use _utils::types::api_pokemon::PokemonBasic;

#[component]
pub fn PokemonStorage() -> Element {
    use_pokemon_state();
    use_user_profile_state();

    let popup = use_popup();
    let mut select_mode = use_signal::<bool>(|| false);
    let mut selected_ids = use_signal::<Vec<u64>>(Vec::new);
    let mut search_text = use_signal(String::new);
    let mut type_filter = use_signal(String::new);
    let mut min_level = use_signal(String::new);
    let mut max_level = use_signal(String::new);
    let mut shiny_filter = use_signal(String::new);
    let mut sort_order = use_signal(StorageSort::default);
    // 仓库可能有多箱数百只宠物（旧数据 site 3..N），默认只渲染一部分
    const STORAGE_PAGE_SIZE: usize = 60;
    let mut visible_count = use_signal(|| STORAGE_PAGE_SIZE);

    let (is_user_in_battle, loading, loaded, bag_pokemons, storage_pokemons) = {
        let state = POKEMON_STATE.read();
        let user_state = USER_STATE.read();
        let in_battle = user_state.is_in_battle();
        let ld = state.loading;
        let ld2 = state.loaded;
        let bag: Vec<PokemonBasic> = state
            .list
            .iter()
            .filter(|p| p.site == 1 || p.site == 2)
            .cloned()
            .collect();
        let storage: Vec<PokemonBasic> =
            state.list.iter().filter(|p| p.site >= 3).cloned().collect();
        (in_battle, ld, ld2, bag, storage)
    };

    let keyword = search_text.read().trim().to_lowercase();
    let level_range = parse_level_range(&min_level.read(), &max_level.read());
    let type_value = type_filter.read().clone();
    let (min, max) = level_range.unwrap_or_default();
    let filters = StorageFilters {
        name: &keyword,
        pokemon_type: &type_value,
        min_level: min,
        max_level: max,
        shiny: match shiny_filter.read().as_str() {
            "shiny" => Some(true),
            "normal" => Some(false),
            _ => None,
        },
        sort: *sort_order.read(),
    };
    let entries: Vec<_> = storage_pokemons
        .iter()
        .map(|p| StorageEntry {
            id: p.id,
            name: &p.name,
            nickname: p.nickname.as_deref(),
            type_1: &p.base_info.type_1,
            type_2: p.base_info.type_2.as_deref(),
            level: p.level,
            shiny: p.is_shiny,
        })
        .collect();
    let storage_pokemon_ids = if level_range.is_ok() {
        matching_storage_ids(&entries, &filters)
    } else {
        Vec::new()
    };
    let by_id: std::collections::HashMap<_, _> =
        storage_pokemons.iter().map(|p| (p.id, p)).collect();
    let filtered_storage: Vec<PokemonBasic> = storage_pokemon_ids
        .iter()
        .filter_map(|id| by_id.get(id).map(|pokemon| (**pokemon).clone()))
        .collect();
    let shown_storage: Vec<PokemonBasic> = filtered_storage
        .iter()
        .take(*visible_count.read())
        .cloned()
        .collect();

    let storage_count = filtered_storage.len();
    let selected_count = selected_matching_ids(&selected_ids.read(), &storage_pokemon_ids).len();
    let pokemon_types = [
        "普通", "火", "水", "草", "电", "冰", "格斗", "毒", "地面", "飞行", "超能", "虫", "岩石",
        "幽灵", "龙", "恶", "钢", "妖精",
    ];

    let mut toggle_select = {
        let mut selected_ids = selected_ids;
        move |id: u64| {
            let mut ids = selected_ids.read().clone();
            if let Some(pos) = ids.iter().position(|&x| x == id) {
                ids.remove(pos);
            } else {
                ids.push(id);
            }
            selected_ids.set(ids);
        }
    };

    let mut select_all = {
        let matching_ids = storage_pokemon_ids.clone();
        move || selected_ids.set(matching_ids.clone())
    };

    let mut exit_select_mode = {
        let mut select_mode = select_mode;
        let mut selected_ids = selected_ids;
        move || {
            select_mode.set(false);
            selected_ids.set(vec![]);
        }
    };

    let mut batch_release = {
        let mut select_mode = select_mode;
        let mut selected_ids = selected_ids;
        move || {
            let ids = selected_matching_ids(&selected_ids.read(), &storage_pokemon_ids);
            if ids.is_empty() {
                return;
            }
            spawn(async move {
                let api = NewApiClient::new();
                let mut success_count = 0;
                let mut fail_count = 0;
                for id in ids.iter() {
                    match api.release_pokemon(*id).await {
                        Ok(_) => success_count += 1,
                        Err(_) => fail_count += 1,
                    }
                }
                crate::state::refresh_pokemon_list();
                if fail_count == 0 {
                    show_success(format!("成功放生 {} 只宠物", success_count));
                } else {
                    show_error(format!(
                        "放生完成：成功 {} 只，失败 {} 只",
                        success_count, fail_count
                    ));
                }
            });
            select_mode.set(false);
            selected_ids.set(vec![]);
        }
    };

    rsx! {
        div { class: "page-pokemon-storage",

            div { class: "storage-container",
                div { class: "storage-left",
                    div { class: "storage-panel bag-panel",
                        div { class: "panel-header",
                            span { class: "panel-title", "背包宠物" }
                        }
                        div { class: "panel-body",
                            if loading && !loaded {
                                div { class: "panel-loading", "加载中..." }
                            } else {
                                div { class: "pokemon-grid",
                                    for i in 0..6usize {
                                        {
                                            if i < bag_pokemons.len() {
                                                let pokemon = bag_pokemons[i].clone();
                                                let display_name = pokemon
                                                    .nickname
                                                    .clone()
                                                    .unwrap_or_else(|| pokemon.name.clone());
                                                let hp_pct = hp_percent(pokemon.hp, pokemon.max_hp);
                                                let hp_cls = hp_class_storage(pokemon.hp, pokemon.max_hp);
                                                let health_status = hp_health_status(pokemon.hp, pokemon.max_hp);
                                                let pokemon_is_in_battle = is_user_in_battle && pokemon.site == 1;
                                                rsx! {
                                                    PokemonSlot {
                                                        key: "{i}",
                                                        pokemon,
                                                        display_name,
                                                        hp_percent: hp_pct,
                                                        hp_class: hp_cls,
                                                        health_status,
                                                        popup,
                                                        is_in_battle: pokemon_is_in_battle,
                                                        is_select_mode: false,
                                                        is_selected: false,
                                                        on_toggle_select: None,
                                                    }
                                                }
                                            } else {
                                                rsx! {
                                                    div { key: "{i}", class: "pokemon-slot empty-slot" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "storage-panel storage-panel-bottom",
                        div { class: "panel-header",
                            div { class: "panel-header-left",
                                span { class: "panel-title", "仓库宠物" }
                                span { class: "panel-count", id: "storage-result-count", "{storage_count} / {storage_pokemons.len()} 只" }
                            }
                            if *select_mode.read() {
                                button {
                                    class: "select-mode-btn exit",
                                    onclick: move |_| exit_select_mode(),
                                    "取消选择"
                                }
                            } else if storage_count > 0 {
                                button {
                                    class: "select-mode-btn",
                                    onclick: move |_| select_mode.set(true),
                                    "多选"
                                }
                            }
                        }
                        div { class: "storage-search-bar",
                            input {
                                id: "storage-search",
                                class: "storage-search-input",
                                r#type: "text",
                                placeholder: "搜索名字 / 昵称",
                                value: "{search_text.read()}",
                                oninput: move |evt| {
                                    search_text.set(evt.value());
                                    selected_ids.set(Vec::new());
                                    visible_count.set(STORAGE_PAGE_SIZE);
                                }
                            }
                            div { class: "storage-filter-fields",
                                label {
                                    "属性"
                                    select {
                                        id: "storage-type-filter",
                                        value: "{type_filter.read()}",
                                        onchange: move |evt| {
                                            type_filter.set(evt.value());
                                            selected_ids.set(Vec::new());
                                            visible_count.set(STORAGE_PAGE_SIZE);
                                        },
                                        option { value: "", "全部属性" }
                                        for pokemon_type in pokemon_types {
                                            option { value: "{pokemon_type}", "{pokemon_type}" }
                                        }
                                    }
                                }
                                label {
                                    "最低等级"
                                    input {
                                        id: "storage-min-level", r#type: "number", min: "1", step: "1",
                                        placeholder: "不限", value: "{min_level.read()}",
                                        oninput: move |evt| {
                                            min_level.set(evt.value());
                                            selected_ids.set(Vec::new());
                                            visible_count.set(STORAGE_PAGE_SIZE);
                                        },
                                    }
                                }
                                label {
                                    "最高等级"
                                    input {
                                        id: "storage-max-level", r#type: "number", min: "1", step: "1",
                                        placeholder: "不限", value: "{max_level.read()}",
                                        oninput: move |evt| {
                                            max_level.set(evt.value());
                                            selected_ids.set(Vec::new());
                                            visible_count.set(STORAGE_PAGE_SIZE);
                                        },
                                    }
                                }
                                label {
                                    "闪光"
                                    select {
                                        id: "storage-shiny-filter", value: "{shiny_filter.read()}",
                                        onchange: move |evt| {
                                            shiny_filter.set(evt.value());
                                            selected_ids.set(Vec::new());
                                            visible_count.set(STORAGE_PAGE_SIZE);
                                        },
                                        option { value: "", "全部" }
                                        option { value: "shiny", "仅闪光" }
                                        option { value: "normal", "非闪光" }
                                    }
                                }
                                label {
                                    "排序"
                                    select {
                                        id: "storage-sort", value: sort_order.read().value(),
                                        onchange: move |evt| {
                                            sort_order.set(StorageSort::from_value(&evt.value()));
                                            selected_ids.set(Vec::new());
                                            visible_count.set(STORAGE_PAGE_SIZE);
                                        },
                                        option { value: "oldest", "取得较早优先" }
                                        option { value: "newest", "最近取得优先" }
                                        option { value: "level_asc", "等级从低到高" }
                                        option { value: "level_desc", "等级从高到低" }
                                    }
                                }
                                button {
                                    id: "storage-clear-filters", class: "select-action-btn",
                                    onclick: move |_| {
                                        search_text.set(String::new());
                                        type_filter.set(String::new());
                                        min_level.set(String::new());
                                        max_level.set(String::new());
                                        shiny_filter.set(String::new());
                                        sort_order.set(StorageSort::default());
                                        selected_ids.set(Vec::new());
                                        visible_count.set(STORAGE_PAGE_SIZE);
                                    },
                                    "清除筛选"
                                }
                            }
                            if let Err(message) = level_range {
                                p { class: "storage-filter-error", role: "alert", "{message}" }
                            }
                        }
                        if *select_mode.read() && !filtered_storage.is_empty() {
                            div { class: "select-actions-bar",
                                button {
                                    id: "storage-select-all",
                                    class: "select-action-btn",
                                    onclick: move |_| select_all(),
                                    "全选筛选结果 ({storage_count})"
                                }
                                button {
                                    id: "storage-batch-release",
                                    class: "select-action-btn danger",
                                    disabled: selected_count == 0,
                                    onclick: move |_| batch_release(),
                                    "放生 ({selected_count})"
                                }
                            }
                        }
                        div { class: "panel-body",
                            if loading && !loaded {
                                div { class: "panel-loading", "加载中..." }
                            } else if filtered_storage.is_empty() {
                                div { class: "panel-empty",
                                    if storage_pokemons.is_empty() {
                                        "仓库中没有宠物"
                                    } else {
                                        "没有匹配的宠物"
                                    }
                                }
                            } else {
                                div { class: "pokemon-grid storage-grid",
                                    for pokemon in shown_storage.iter() {
                                        {
                                            let pokemon_clone = pokemon.clone();
                                            let display_name = pokemon
                                                .nickname
                                                .clone()
                                                .unwrap_or_else(|| pokemon.name.clone());
                                            let hp_percent = hp_percent(pokemon.hp, pokemon.max_hp);
                                            let hp_class = hp_class_storage(pokemon.hp, pokemon.max_hp);
                                            let health_status = hp_health_status(pokemon.hp, pokemon.max_hp);
                                            let pokemon_id = pokemon.id;
                                            let is_selected = selected_ids.read().contains(&pokemon_id);
                                            rsx! {
                                                PokemonSlot {
                                                    key: "{pokemon_id}",
                                                    pokemon: pokemon_clone,
                                                    display_name,
                                                    hp_percent,
                                                    hp_class,
                                                    health_status,
                                                    popup,
                                                    is_in_battle: false,
                                                    is_select_mode: *select_mode.read(),
                                                    is_selected,
                                                    on_toggle_select: move |_| toggle_select(pokemon_id),
                                                }
                                            }
                                        }
                                    }
                                }
                                if filtered_storage.len() > shown_storage.len() {
                                    div { class: "storage-load-more",
                                        button {
                                            class: "select-action-btn",
                                            id: "storage-load-more",
                                            onclick: move |_| {
                                                let next = *visible_count.read() + STORAGE_PAGE_SIZE;
                                                visible_count.set(next);
                                            },
                                            "加载更多（已显示 {shown_storage.len()}/{filtered_storage.len()}）"
                                        }
                                    }
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
fn PokemonSlot(
    pokemon: PokemonBasic,
    display_name: String,
    hp_percent: u32,
    hp_class: &'static str,
    health_status: &'static str,
    popup: PopupContext,
    is_in_battle: bool,
    is_select_mode: bool,
    is_selected: bool,
    #[props(default)] on_toggle_select: Option<EventHandler<()>>,
) -> Element {
    let pokemon_for_click = pokemon.clone();
    let mut popup_for_close = popup;
    let is_critical = health_status == "critical";
    let is_weak = is_weak_state(pokemon.state);

    let slot_class = if is_critical {
        "pokemon-slot critical"
    } else if is_weak {
        "pokemon-slot weak"
    } else if is_selected {
        "pokemon-slot selected"
    } else {
        &format!("pokemon-slot {health_status}")
    };

    rsx! {
        div {
            class: "{slot_class}",
            "data-pokemon-id": "{pokemon.id}",
            "data-shiny": "{pokemon.is_shiny}",
            title: "{display_name} · Lv.{pokemon.level}",
            onclick: move |evt: Event<MouseData>| {
                if is_select_mode {
                    if let Some(handler) = &on_toggle_select {
                        handler.call(());
                    }
                    return;
                }

                evt.stop_propagation();
                let is_first = pokemon_for_click.site == 1;
                let is_bag = pokemon_for_click.site == 1 || pokemon_for_click.site == 2;
                let selected_id = pokemon_for_click.id;
                let pokemon_state_is_critical = pokemon_for_click.state == 0;
                let pokemon_hp_is_zero = pokemon_for_click.hp <= 0;
                let pokemon_is_in_battle = pokemon_for_click.site == 1 && is_in_battle;
                let cannot_release = pokemon_state_is_critical || pokemon_hp_is_zero
                    || pokemon_is_in_battle;
                let bag_full = !is_bag && POKEMON_STATE.read().get_bag_pokemons().len() >= 6;
                let content = rsx! {
                    PopupMenu {
                        if is_bag {
                            if !is_first {
                                PopupMenuItem {
                                    primary: true,
                                    disabled: pokemon_is_in_battle,
                                    onclick: move |_| {
                                        if pokemon_is_in_battle {
                                            return;
                                        }
                                        popup_for_close.close();
                                        let id = selected_id;
                                        spawn(async move {
                                            let api = NewApiClient::new();
                                            match api.set_first_pokemon(id).await {
                                                Ok(_) => {
                                                    crate::state::refresh_pokemon_list();
                                                }
                                                Err(e) => show_error(format!("设为首位失败: {}", e)),
                                            }
                                        });
                                    },
                                    "设为首位"
                                }
                            }
                            PopupMenuItem {
                                disabled: pokemon_is_in_battle,
                                onclick: move |_| {
                                    if pokemon_is_in_battle {
                                        return;
                                    }
                                    popup_for_close.close();
                                    let id = selected_id;
                                    spawn(async move {
                                        let api = NewApiClient::new();
                                        match api.move_pokemon_to_site(id, 3).await {
                                            Ok(_) => {
                                                crate::state::refresh_pokemon_list();
                                            }
                                            Err(e) => show_error(format!("放入仓库失败: {}", e)),
                                        }
                                    });
                                },
                                "放入仓库"
                            }
                        } else {
                            PopupMenuItem {
                                disabled: bag_full,
                                onclick: move |_| {
                                    if bag_full {
                                        return;
                                    }
                                    popup_for_close.close();
                                    let id = selected_id;
                                    spawn(async move {
                                        let api = NewApiClient::new();
                                        match api.move_pokemon_to_site(id, 2).await {
                                            Ok(_) => {
                                                crate::state::refresh_pokemon_list();
                                            }
                                            Err(e) => show_error(format!("放入背包失败: {}", e)),
                                        }
                                    });
                                },
                                if bag_full {
                                    "背包已满（6/6）"
                                } else {
                                    "放入背包"
                                }
                            }
                            PopupMenuItem {
                                danger: true,
                                disabled: cannot_release,
                                onclick: move |_| {
                                    if cannot_release {
                                        return;
                                    }
                                    popup_for_close.close();
                                    let id = selected_id;
                                    spawn(async move {
                                        let api = NewApiClient::new();
                                        match api.release_pokemon(id).await {
                                            Ok(_) => {
                                                crate::state::refresh_pokemon_list();
                                                show_success("放生成功".to_string());
                                            }
                                            Err(e) => show_error(format!("放生失败: {}", e)),
                                        }
                                    });
                                },
                                "放生"
                            }
                        }
                    }
                };
                popup_for_close
                    .open_at_mouse(
                        evt.data().client_coordinates().x,
                        evt.data().client_coordinates().y + 10.0,
                        content,
                    );
            },
            div { class: "pokemon-sprite",
                img {
                    src: "{IMG_PATH_REMOTE}/pm/{pokemon.type_id}.gif",
                    alt: "{display_name}",
                }
            }
            if is_in_battle {
                span { class: "pokemon-battle-icon", "⚔️" }
            }
            if is_select_mode {
                div { class: "pokemon-select-checkbox",
                    if is_selected {
                        span { class: "checkbox-icon checked", "✓" }
                    } else {
                        span { class: "checkbox-icon", "" }
                    }
                }
            }
            div { class: "pokemon-slot-name", "{display_name}" }
            div { class: "pokemon-level", "Lv.{pokemon.level}" }
            div { class: "pokemon-hp-bar",
                if !is_critical {
                    div {
                        class: "hp-bar-fill {hp_class}",
                        style: "width: {hp_percent}%",
                    }
                }
            }
            div { class: "pokemon-hp-text",
                if is_critical {
                    div { class: "critical-hp-text", "濒危" }
                } else if is_weak {
                    div { class: "weak-hp-text", "虚弱" }
                } else if hp_percent < 100 {
                    "{pokemon.hp}/{pokemon.max_hp}"
                } else {
                    ""
                }
            }
        }
    }
}
