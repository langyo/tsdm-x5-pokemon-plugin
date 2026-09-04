use dioxus::prelude::*;

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
            state.list.iter().filter(|p| p.site == 3).cloned().collect();
        (in_battle, ld, ld2, bag, storage)
    };

    let storage_count = storage_pokemons.len();
    let storage_pokemon_ids: Vec<u64> = storage_pokemons.iter().map(|p| p.id).collect();

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

    let mut select_all = move || {
        selected_ids.set(storage_pokemon_ids.clone());
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
            let ids = selected_ids.read().clone();
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
                                span { class: "panel-count", "{storage_count} 只" }
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
                        if *select_mode.read() && !storage_pokemons.is_empty() {
                            div { class: "select-actions-bar",
                                button {
                                    class: "select-action-btn",
                                    onclick: move |_| select_all(),
                                    "全选"
                                }
                                button {
                                    class: "select-action-btn danger",
                                    disabled: selected_ids.read().is_empty(),
                                    onclick: move |_| batch_release(),
                                    "放生 ({selected_ids.read().len()})"
                                }
                            }
                        }
                        div { class: "panel-body",
                            if loading && !loaded {
                                div { class: "panel-loading", "加载中..." }
                            } else if storage_pokemons.is_empty() {
                                div { class: "panel-empty", "仓库中没有宠物" }
                            } else {
                                div { class: "pokemon-grid storage-grid",
                                    for pokemon in storage_pokemons.iter() {
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
