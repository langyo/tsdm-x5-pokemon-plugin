use dioxus::prelude::*;

use crate::dioxus::{
    components::{
        form_fields::{
            BoolField, Col, FieldWrapper, FormSection, Row, SelectField, TextField,
            UnsignedNumberField,
        },
        icon::IconName,
        icon_button::{ButtonSize, IconButton},
        tabs::Tabs,
        user_grant::{GrantItemModal, GrantPokemonModal},
    },
    pages::shared::{update_filter_operator, ActionModal, FilterCollapsiblePanel, ListBottomDock},
    state::{
        set_busy, set_notice, AdminNoticeLevel, FilterPackage, ADMIN_BUSY, ADMIN_ITEM_DATA,
        ADMIN_POKEMON_DATA, ADMIN_USER_DATA,
    },
    utils::api::{
        count_user_info, delete_item_info, delete_pokemon_info, filter_user_info, list_item_info,
        list_item_type, list_pokemon_info, list_pokemon_type, list_user_info, set_item_info,
        set_pokemon_info, set_user_info,
    },
};
use _utils::types::{
    item_info::ItemInfo,
    item_type::ItemTag,
    pokemon_info::{PokemonInfo, PokemonSite},
    pokemon_type::PokemonAttributes,
    user_info::UserInfo,
};

/// 宠物标签页选项
#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum PokemonTab {
    #[default]
    All,
    Bag,
    Store,
}

impl PokemonTab {
    fn label(self) -> &'static str {
        match self {
            PokemonTab::All => "全部",
            PokemonTab::Bag => "背包",
            PokemonTab::Store => "仓库",
        }
    }

    fn filter(&self, site: PokemonSite) -> bool {
        match self {
            PokemonTab::All => true,
            PokemonTab::Bag => site == PokemonSite::Bag || site == PokemonSite::Header,
            PokemonTab::Store => site == PokemonSite::Store,
        }
    }
}

/// 物品标签页选项
#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum ItemTab {
    #[default]
    All,
    Drug,
    Ball,
    Evolution,
    Enhance,
    Armor,
    Special,
}

impl ItemTab {
    fn label(self) -> &'static str {
        match self {
            ItemTab::All => "全部",
            ItemTab::Drug => "药品",
            ItemTab::Ball => "宠物球",
            ItemTab::Evolution => "升级素材",
            ItemTab::Enhance => "强化道具",
            ItemTab::Armor => "装备",
            ItemTab::Special => "特殊物品",
        }
    }

    fn matches_tag(&self, tag: &ItemTag) -> bool {
        match self {
            ItemTab::All => true,
            ItemTab::Drug => matches!(tag, ItemTag::Drug),
            ItemTab::Ball => matches!(tag, ItemTag::Ball(_)),
            ItemTab::Evolution => matches!(tag, ItemTag::Evolution(_)),
            ItemTab::Enhance => matches!(tag, ItemTag::Enhance),
            ItemTab::Armor => matches!(tag, ItemTag::Armor(_)),
            ItemTab::Special => matches!(tag, ItemTag::Special(_)),
        }
    }
}

#[component]
pub fn UserDataPage() -> Element {
    let state = ADMIN_USER_DATA.read().clone();
    let rows = state.items.clone();
    let filters = state.filters.clone();
    let has_more = !state.is_filtered && state.loaded_count < state.total_count;
    let is_busy = *ADMIN_BUSY.read();

    let mut selected_user = use_signal(|| None::<u64>);
    let mut user_editor_data = use_signal(|| None::<UserInfo>);
    let mut pokemon_infos = use_signal(Vec::<PokemonInfo>::new);
    let mut item_infos = use_signal(Vec::<ItemInfo>::new);

    // 编辑现有宠物/物品的表单数据
    let mut pokemon_editor_data = use_signal(|| None::<PokemonInfo>);
    let mut item_editor_data = use_signal(|| None::<ItemInfo>);

    // 放生宠物/删除物品的确认模态框控制
    let mut release_pokemon_data = use_signal(|| None::<PokemonInfo>);
    let mut delete_item_data = use_signal(|| None::<ItemInfo>);

    // 给予宠物/物品的模态框控制
    let mut show_grant_pokemon = use_signal(|| false);
    let mut show_grant_item = use_signal(|| false);

    // 标签页选择状态
    let mut pokemon_tab = use_signal(|| PokemonTab::default());
    let mut item_tab = use_signal(|| ItemTab::default());
    let mut show_filter_modal = use_signal(|| false);

    use_effect(move || {
        let state = ADMIN_USER_DATA.read().clone();
        if state.initialized || state.loading {
            return;
        }
        reload_users();
    });

    // 预加载宠物类型和物品类型数据（用于显示名称）
    use_effect(move || {
        let pokemon_state = ADMIN_POKEMON_DATA.read().clone();
        if !pokemon_state.initialized && !pokemon_state.loading {
            spawn(async move {
                if let Ok(items) = list_pokemon_type(0, 500).await {
                    let mut state = ADMIN_POKEMON_DATA.write();
                    state.items = items;
                    state.initialized = true;
                }
            });
        }
    });

    use_effect(move || {
        let item_state = ADMIN_ITEM_DATA.read().clone();
        if !item_state.initialized && !item_state.loading {
            spawn(async move {
                if let Ok(items) = list_item_type(0, 1000).await {
                    let mut state = ADMIN_ITEM_DATA.write();
                    state.items = items;
                    state.initialized = true;
                }
            });
        }
    });

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = js_sys::eval(
                r#"
                (function () {
                    const container = document.getElementById('user-table-scroll');
                    if (!container) return;
                    if (container.dataset.autoLoadBound === '1') return;
                    container.dataset.autoLoadBound = '1';
                    let ticking = false;
                    const tryLoad = () => {
                        const trigger = document.getElementById('user-load-more-trigger');
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
                    h2 { "用户数据" }
                }
            }

            ListBottomDock {
                title: "筛选器",
                fields: filters.clone(),
                disabled: is_busy,
                total_count: state.total_count,
                loaded_count: state.loaded_count,
                is_filtered: state.is_filtered,
                show_create: false,
                create_title: "新增用户",
                create_disabled: true,
                on_create: move |_| {},
                reload_disabled: false,
                on_reload: move |_| reload_users(),
                load_more_disabled: state.is_filtered || state.loaded_count >= state.total_count,
                on_load_more: move |_| load_more_users(),
                on_operator_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    let mut state = ADMIN_USER_DATA.write();
                    update_filter_operator(&mut state.filters, index, &value);
                },
                on_value_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    if let Some(field) = ADMIN_USER_DATA.write().filters.get_mut(index) {
                        field.value = value;
                    }
                },
                on_apply: move |_| apply_user_filters(),
                on_reset: move |_| reset_user_filters(),
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
                    let mut state = ADMIN_USER_DATA.write();
                    update_filter_operator(&mut state.filters, index, &value);
                },
                on_value_change: move |payload: (usize, String)| {
                    let (index, value) = payload;
                    if let Some(field) = ADMIN_USER_DATA.write().filters.get_mut(index) {
                        field.value = value;
                    }
                },
                on_enabled_change: move |payload: (usize, bool)| {
                    let (index, enabled) = payload;
                    if let Some(field) = ADMIN_USER_DATA.write().filters.get_mut(index) {
                        field.enabled = enabled;
                    }
                },
                on_apply: move |_| {
                    apply_user_filters();
                    show_filter_modal.set(false);
                },
                on_reset: move |_| reset_user_filters(),
            }

            div { id: "user-table-scroll", class: "admin-card data-table-card",
                if !state.initialized {
                    div { class: "admin-table-loading",
                        div { class: "admin-table-loading__spinner" }
                    }
                } else if rows.is_empty() {
                    p { class: "empty-hint", "暂无用户数据。" }
                } else {
                    table { class: "admin-table admin-table--user",
                        thead {
                            tr {
                                th { "UID" }
                                th { "昵称" }
                                th { "胜 / 负" }
                                th { "金钱" }
                                th { "经验值" }
                                th { "宠物" }
                                th { "道具" }
                            }                        }
                        tbody {
                            for item in rows {
                                UserRow {
                                    key: "user-{item.id}",
                                    item,
                                    on_manage_related: move |uid| {
                                        selected_user.set(Some(uid));
                                        let user_info = ADMIN_USER_DATA
                                            .read()
                                            .items
                                            .iter()
                                            .find(|u| u.id == uid)
                                            .cloned();
                                        user_editor_data.set(user_info);
                                        load_user_related(uid, pokemon_infos, item_infos);
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
                        id: "user-load-more-trigger",
                        class: "admin-load-more-trigger",
                        "data-can-load": if has_more { "1" } else { "0" },
                        "data-loading": if state.loading { "1" } else { "0" },
                        onclick: move |_| {
                            if !ADMIN_USER_DATA.read().loading {
                                load_more_users();
                            }
                        },
                        "load-more"
                    }
                }
            }

            if let Some(uid) = selected_user() {
                ActionModal {
                    title: format!("用户 #{} 的附属数据", uid),
                    wide: true,
                    on_close: move |_| {
                        selected_user.set(None);
                        user_editor_data.set(None);
                        pokemon_infos.set(Vec::new());
                        item_infos.set(Vec::new());
                        pokemon_editor_data.set(None);
                        item_editor_data.set(None);
                    },
                    div { class: "admin-modal-tables-grid",
                        // 用户元数据编辑区域
                        if let Some(initial) = user_editor_data() {
                            UserMetaEditor {
                                key: "user-meta-editor-{uid}",
                                initial_data: initial,
                                disabled: is_busy,
                                on_save: move |updated: UserInfo| {
                                    save_user_meta(uid, updated);
                                },
                            }
                        }

                        // 宠物列表区域
                        div { class: "admin-modal-table-section",
                            div { class: "admin-modal-section-header",
                                Tabs {
                                    active: pokemon_tab(),
                                    options: vec![
                                        (PokemonTab::All, "全部".to_string()),
                                        (PokemonTab::Bag, "背包".to_string()),
                                        (PokemonTab::Store, "仓库".to_string()),
                                    ],
                                    disabled: is_busy,
                                    on_change: move |tab| pokemon_tab.set(tab),
                                }
                                div { class: "admin-modal-actions",
                                    IconButton {
                                        icon: IconName::RotateCcw,
                                        tooltip: "刷新宠物列表".to_string(),
                                        disabled: is_busy,
                                        size: Some(ButtonSize::ExtraSmall),
                                        onclick: move |_| {
                                            set_busy(true);
                                            spawn(async move {
                                                if let Ok(items) = list_pokemon_info(uid, 0, 1000).await {
                                                    pokemon_infos.set(items);
                                                    set_notice(AdminNoticeLevel::Info, "宠物列表已刷新");
                                                }
                                                set_busy(false);
                                            });
                                        },
                                        class: None,
                                    }
                                    IconButton {
                                        icon: IconName::Plus,
                                        tooltip: "给予宠物".to_string(),
                                        disabled: is_busy,
                                        size: Some(ButtonSize::ExtraSmall),
                                        onclick: move |_| show_grant_pokemon.set(true),
                                        class: None,
                                    }
                                }
                            }
                            div { class: "admin-table-wrapper admin-table-wrapper--limited",
                                if pokemon_infos().is_empty() {
                                    p { class: "empty-hint", "该用户暂无宠物。" }
                                } else {
                                    table { class: "admin-table admin-table--user-pokemon-info",
                                        thead {
                                            tr {
                                                th { "ID" }
                                                th { "种族" }
                                                th { "昵称" }
                                                th { "等级" }
                                                th { "位置" }
                                            }
                                        }
                                        tbody {
                                            for info in pokemon_infos() {
                                                if pokemon_tab().filter(info.site) {
                                                    UserPokemonRow {
                                                        key: "user-pm-{info.id}",
                                                        info,
                                                        on_edit: move |payload: PokemonInfo| {
                                                            pokemon_editor_data.set(Some(payload));
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // 物品列表区域
                        div { class: "admin-modal-table-section",
                            div { class: "admin-modal-section-header",
                                Tabs {
                                    active: item_tab(),
                                    options: vec![
                                        (ItemTab::All, "全部".to_string()),
                                        (ItemTab::Drug, "药品".to_string()),
                                        (ItemTab::Ball, "宠物球".to_string()),
                                        (ItemTab::Evolution, "升级素材".to_string()),
                                        (ItemTab::Enhance, "强化道具".to_string()),
                                        (ItemTab::Armor, "装备".to_string()),
                                        (ItemTab::Special, "特殊物品".to_string()),
                                    ],
                                    disabled: is_busy,
                                    on_change: move |tab| item_tab.set(tab),
                                }
                                div { class: "admin-modal-actions",
                                    IconButton {
                                        icon: IconName::RotateCcw,
                                        tooltip: "刷新物品列表".to_string(),
                                        disabled: is_busy,
                                        size: Some(ButtonSize::ExtraSmall),
                                        onclick: move |_| {
                                            set_busy(true);
                                            spawn(async move {
                                                if let Ok(items) = list_item_info(uid, 0, 1000).await {
                                                    item_infos.set(items);
                                                    set_notice(AdminNoticeLevel::Info, "物品列表已刷新");
                                                }
                                                set_busy(false);
                                            });
                                        },
                                        class: None,
                                    }
                                    IconButton {
                                        icon: IconName::Plus,
                                        tooltip: "给予物品".to_string(),
                                        disabled: is_busy,
                                        size: Some(ButtonSize::ExtraSmall),
                                        onclick: move |_| show_grant_item.set(true),
                                        class: None,
                                    }
                                }
                            }
                            div { class: "admin-table-wrapper admin-table-wrapper--limited",
                                if item_infos().is_empty() {
                                    p { class: "empty-hint", "该用户暂无物品。" }
                                } else {
                                    table { class: "admin-table admin-table--user-item-info",
                                        thead {
                                            tr {
                                                th { "ID" }
                                                th { "类型" }
                                                th { "数量" }
                                            }
                                        }
                                        tbody {
                                            for info in item_infos() {
                                                if let Some(item_type) = ADMIN_ITEM_DATA
                                                    .read()
                                                    .items
                                                    .iter()
                                                    .find(|t| t.id == info.type_id)
                                                {
                                                    if item_tab().matches_tag(&item_type.tag) {
                                                        UserItemRow {
                                                            key: "user-item-{info.id}",
                                                            info,
                                                            on_edit: move |payload: ItemInfo| {
                                                                item_editor_data.set(Some(payload));
                                                            },
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

            // 编辑宠物的可视化表单模态框
            if let Some(uid) = selected_user() {
                if let Some(info) = pokemon_editor_data() {
                    EditPokemonInfoModal {
                        initial_data: info,
                        disabled: is_busy,
                        on_save: move |updated: PokemonInfo| {
                            save_user_pokemon_info(uid, updated, pokemon_infos, item_infos);
                            pokemon_editor_data.set(None);
                        },
                        on_release: move |data: PokemonInfo| {
                            pokemon_editor_data.set(None);
                            release_pokemon_data.set(Some(data));
                        },
                        on_close: move |_| {
                            pokemon_editor_data.set(None);
                        },
                    }
                }

                if let Some(info) = item_editor_data() {
                    EditItemInfoModal {
                        initial_data: info,
                        disabled: is_busy,
                        on_save: move |updated: ItemInfo| {
                            save_user_item_info(uid, updated, pokemon_infos, item_infos);
                            item_editor_data.set(None);
                        },
                        on_delete: move |data: ItemInfo| {
                            item_editor_data.set(None);
                            delete_item_data.set(Some(data));
                        },
                        on_close: move |_| {
                            item_editor_data.set(None);
                        },
                    }
                }
            }

            // 放生宠物确认模态框
            if let Some(uid) = selected_user() {
                if let Some(info) = release_pokemon_data() {
                    ReleasePokemonConfirmModal {
                        pokemon_info: info.clone(),
                        disabled: is_busy,
                        on_confirm: move |_| {
                            release_user_pokemon(uid, info.clone(), pokemon_infos, item_infos);
                            release_pokemon_data.set(None);
                        },
                        on_close: move |_| {
                            release_pokemon_data.set(None);
                        },
                    }
                }
            }

            // 删除物品模态框
            if let Some(uid) = selected_user() {
                if let Some(info) = delete_item_data() {
                    DeleteItemModal {
                        item_info: info,
                        disabled: is_busy,
                        on_confirm: move |count: u64| {
                            delete_user_item(uid, info.clone(), count, pokemon_infos, item_infos);
                            delete_item_data.set(None);
                        },
                        on_close: move |_| {
                            delete_item_data.set(None);
                        },
                    }
                }
            }

            // 给予宠物模态框
            if let Some(uid) = selected_user() {
                if *show_grant_pokemon.read() {
                    GrantPokemonModal {
                        owner_uid: uid,
                        on_close: move |_| show_grant_pokemon.set(false),
                        on_success: move |_saved: PokemonInfo| {
                            show_grant_pokemon.set(false);
                            load_user_related(uid, pokemon_infos, item_infos);
                        },
                    }
                }
            }

            // 给予物品模态框
            if let Some(uid) = selected_user() {
                if *show_grant_item.read() {
                    GrantItemModal {
                        owner_uid: uid,
                        on_close: move |_| show_grant_item.set(false),
                        on_success: move |_saved: ItemInfo| {
                            show_grant_item.set(false);
                            load_user_related(uid, pokemon_infos, item_infos);
                        },
                    }
                }
            }

        }
    }
}

#[component]
fn UserRow(item: UserInfo, on_manage_related: EventHandler<u64>) -> Element {
    let item_id = item.id;
    let record_label = format!("{} / {}", item.win_count, item.lose_count);
    let pokemon_label = format!("{} 只", item.pokemon_list.len());
    let item_label = format!("{} 件", item.item_list.len());

    rsx! {
        tr {
            class: "admin-table-row--clickable",
            onclick: move |_| on_manage_related.call(item_id),
            td { "{item.id}" }
            td { "{item.name}" }
            td { "{record_label}" }
            td { "{item.money}" }
            td { "{item.experience}" }
            td { "{pokemon_label}" }
            td { "{item_label}" }
        }
    }
}

#[component]
fn UserPokemonRow(info: PokemonInfo, on_edit: EventHandler<PokemonInfo>) -> Element {
    let info_for_edit = info.clone();

    // 查找种族名称
    let species_name = ADMIN_POKEMON_DATA
        .read()
        .items
        .iter()
        .find(|t| t.id == info.type_id)
        .map(|t| t.name.clone())
        .unwrap_or_else(|| "未知".to_string());

    let species_label = format!("{} #{}", species_name, info.type_id);

    // 昵称：如果为空则显示灰色的"未设定"
    let nickname_display = if info.name.is_empty() {
        rsx! {
            em { class: "admin-text-muted", "未设定" }
        }
    } else {
        rsx! { "{info.name}" }
    };

    rsx! {
        tr {
            class: "admin-table-row--clickable",
            onclick: move |_| on_edit.call(info_for_edit.clone()),
            td { "{info.id}" }
            td { "{species_label}" }
            td { {nickname_display} }
            td { "{info.level}" }
            td { "{info.site}" }
        }
    }
}

#[component]
fn UserItemRow(info: ItemInfo, on_edit: EventHandler<ItemInfo>) -> Element {
    let info_for_edit = info;

    // 查找物品类型名称
    let type_name = ADMIN_ITEM_DATA
        .read()
        .items
        .iter()
        .find(|t| t.id == info.type_id)
        .map(|t| t.name.clone())
        .unwrap_or_else(|| "未知".to_string());

    let type_label = format!("{} #{}", type_name, info.type_id);

    rsx! {
        tr {
            class: "admin-table-row--clickable",
            onclick: move |_| on_edit.call(info_for_edit),
            td { "{info.id}" }
            td { "{type_label}" }
            td { "{info.count}" }
        }
    }
}

fn reload_users() {
    {
        let mut state = ADMIN_USER_DATA.write();
        state.loading = true;
        state.is_filtered = false;
        state.items.clear();
        state.loaded_count = 0;
    }
    set_busy(true);

    spawn(async move {
        match (count_user_info().await, list_user_info(0, 100).await) {
            (Ok(total), Ok(items)) => {
                let mut state = ADMIN_USER_DATA.write();
                state.total_count = total;
                state.loaded_count = items.len() as u64;
                state.items = items;
                state.initialized = true;
                state.loading = false;
                state.is_filtered = false;
                set_notice(AdminNoticeLevel::Info, "用户数据已重新加载");
            }
            (Err(error), _) | (_, Err(error)) => {
                let mut state = ADMIN_USER_DATA.write();
                state.initialized = true;
                state.loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("加载用户数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn load_more_users() {
    let (from, total) = {
        let mut state = ADMIN_USER_DATA.write();
        state.loading = true;
        (state.loaded_count, state.total_count)
    };
    if from >= total {
        ADMIN_USER_DATA.write().loading = false;
        return;
    }
    set_busy(true);

    spawn(async move {
        match list_user_info(from, 100).await {
            Ok(mut items) => {
                let count = items.len() as u64;
                let mut state = ADMIN_USER_DATA.write();
                state.items.append(&mut items);
                state.loaded_count += count;
                state.initialized = true;
                state.loading = false;
                set_notice(
                    AdminNoticeLevel::Info,
                    format!("用户数据继续加载 {} 条", count),
                );
            }
            Err(error) => {
                ADMIN_USER_DATA.write().loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("继续加载用户数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn apply_user_filters() {
    let filters: Vec<FilterPackage> = ADMIN_USER_DATA
        .read()
        .filters
        .iter()
        .filter_map(|field| field.to_package())
        .collect();

    if filters.is_empty() {
        reload_users();
        return;
    }

    ADMIN_USER_DATA.write().loading = true;
    set_busy(true);

    spawn(async move {
        match filter_user_info(filters).await {
            Ok(items) => {
                let count = items.len() as u64;
                let mut state = ADMIN_USER_DATA.write();
                state.items = items;
                state.total_count = count;
                state.loaded_count = count;
                state.initialized = true;
                state.loading = false;
                state.is_filtered = true;
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("筛选得到 {} 条用户记录", count),
                );
            }
            Err(error) => {
                ADMIN_USER_DATA.write().loading = false;
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("筛选用户数据失败: {}", error),
                );
            }
        }
        set_busy(false);
    });
}

fn reset_user_filters() {
    {
        let mut state = ADMIN_USER_DATA.write();
        for field in &mut state.filters {
            field.value.clear();
            field.enabled = false;
        }
    }
    reload_users();
}

fn load_user_related(
    uid: u64,
    mut pokemon_infos: Signal<Vec<PokemonInfo>>,
    mut item_infos: Signal<Vec<ItemInfo>>,
) {
    set_busy(true);

    spawn(async move {
        let pokemon_result = list_pokemon_info(uid, 0, 200).await;
        let item_result = list_item_info(uid, 0, 200).await;

        match (pokemon_result, item_result) {
            (Ok(pokemons), Ok(items)) => {
                pokemon_infos.set(pokemons);
                item_infos.set(items);
                set_notice(
                    AdminNoticeLevel::Info,
                    format!("用户 #{} 的附属数据已加载", uid),
                );
            }
            (Err(error), _) => {
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("加载 PokemonInfo 失败: {}", error),
                );
            }
            (_, Err(error)) => {
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("加载 ItemInfo 失败: {}", error),
                );
            }
        }

        set_busy(false);
    });
}

fn save_user_meta(uid: u64, mut data: UserInfo) {
    data.id = uid;

    set_busy(true);

    spawn(async move {
        match set_user_info(data).await {
            Ok(saved) => {
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("用户 #{} 信息已更新", saved.id),
                );
                reload_users();
            }
            Err(error) => {
                set_notice(
                    AdminNoticeLevel::Error,
                    format!("保存用户信息失败: {}", error),
                );
            }
        }

        set_busy(false);
    });
}

fn save_user_pokemon_info(
    uid: u64,
    mut data: PokemonInfo,
    pokemon_infos: Signal<Vec<PokemonInfo>>,
    item_infos: Signal<Vec<ItemInfo>>,
) {
    data.owner = uid;

    set_busy(true);

    spawn(async move {
        match set_pokemon_info(data).await {
            Ok(saved) => {
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("宠物 #{} 已更新", saved.id),
                );
                load_user_related(uid, pokemon_infos, item_infos);
            }
            Err(error) => {
                set_notice(AdminNoticeLevel::Error, format!("保存宠物失败: {}", error));
            }
        }

        set_busy(false);
    });
}

fn save_user_item_info(
    uid: u64,
    mut data: ItemInfo,
    pokemon_infos: Signal<Vec<PokemonInfo>>,
    item_infos: Signal<Vec<ItemInfo>>,
) {
    data.owner = uid;

    set_busy(true);

    spawn(async move {
        match set_item_info(data).await {
            Ok(saved) => {
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("物品 #{} 已更新", saved.id),
                );
                load_user_related(uid, pokemon_infos, item_infos);
            }
            Err(error) => {
                set_notice(AdminNoticeLevel::Error, format!("保存物品失败: {}", error));
            }
        }

        set_busy(false);
    });
}

fn release_user_pokemon(
    uid: u64,
    data: PokemonInfo,
    pokemon_infos: Signal<Vec<PokemonInfo>>,
    item_infos: Signal<Vec<ItemInfo>>,
) {
    set_busy(true);

    spawn(async move {
        match delete_pokemon_info(data.id).await {
            Ok(()) => {
                set_notice(
                    AdminNoticeLevel::Success,
                    format!("宠物 #{} 已放生", data.id),
                );
                load_user_related(uid, pokemon_infos, item_infos);
            }
            Err(error) => {
                set_notice(AdminNoticeLevel::Error, format!("放生宠物失败: {}", error));
            }
        }

        set_busy(false);
    });
}

fn delete_user_item(
    uid: u64,
    data: ItemInfo,
    delete_count: u64,
    pokemon_infos: Signal<Vec<PokemonInfo>>,
    item_infos: Signal<Vec<ItemInfo>>,
) {
    set_busy(true);

    spawn(async move {
        if delete_count >= data.count {
            // 删除整个条目
            match delete_item_info(data.id).await {
                Ok(()) => {
                    set_notice(
                        AdminNoticeLevel::Success,
                        format!("物品 #{} 已完全删除", data.id),
                    );
                    load_user_related(uid, pokemon_infos, item_infos);
                }
                Err(error) => {
                    set_notice(AdminNoticeLevel::Error, format!("删除物品失败: {}", error));
                }
            }
        } else {
            // 减少数量
            let mut updated = data.clone();
            updated.count = data.count - delete_count;
            match set_item_info(updated).await {
                Ok(saved) => {
                    set_notice(
                        AdminNoticeLevel::Success,
                        format!(
                            "物品 #{} 数量已减少 {}，剩余 {}",
                            saved.id, delete_count, saved.count
                        ),
                    );
                    load_user_related(uid, pokemon_infos, item_infos);
                }
                Err(error) => {
                    set_notice(AdminNoticeLevel::Error, format!("更新物品失败: {}", error));
                }
            }
        }

        set_busy(false);
    });
}

// ============ 可视化表单模态框组件 ============

/// 性别选项（使用 serde 名称）
fn sex_options() -> Vec<(String, String)> {
    vec![
        ("male".to_string(), "雄性".to_string()),
        ("female".to_string(), "雌性".to_string()),
        ("unknown".to_string(), "未知性别".to_string()),
    ]
}

/// 位置选项（使用 serde 名称）
fn site_options() -> Vec<(String, String)> {
    vec![
        ("header".to_string(), "正在直接跟随玩家".to_string()),
        ("bag".to_string(), "位于背包".to_string()),
        ("store".to_string(), "位于仓库".to_string()),
        ("hospital".to_string(), "正在医院接受治疗".to_string()),
    ]
}

/// 状态选项（使用 serde 名称）
fn status_options() -> Vec<(String, String)> {
    vec![
        ("normal".to_string(), "正常".to_string()),
        ("sick1".to_string(), "生病阶段一".to_string()),
        ("sick2".to_string(), "生病阶段二".to_string()),
        ("sick3".to_string(), "生病阶段三".to_string()),
        ("hungry1".to_string(), "饥饿阶段一".to_string()),
        ("hungry2".to_string(), "饥饿阶段二".to_string()),
        ("tired".to_string(), "疲惫".to_string()),
        ("excited1".to_string(), "兴奋阶段一".to_string()),
        ("excited2".to_string(), "兴奋阶段二".to_string()),
        ("excited3".to_string(), "兴奋阶段三".to_string()),
        ("hurt".to_string(), "受伤".to_string()),
        ("happy1".to_string(), "快乐阶段一".to_string()),
        ("happy2".to_string(), "快乐阶段二".to_string()),
        ("happy3".to_string(), "快乐阶段三".to_string()),
        ("shock".to_string(), "惊慌".to_string()),
        ("self_love1".to_string(), "自恋阶段一".to_string()),
        ("self_love2".to_string(), "自恋阶段二".to_string()),
        ("angry1".to_string(), "愤怒阶段一".to_string()),
        ("angry2".to_string(), "愤怒阶段二".to_string()),
        ("dead".to_string(), "死亡".to_string()),
    ]
}

/// 编辑宠物信息的可视化表单模态框
#[component]
fn EditPokemonInfoModal(
    initial_data: PokemonInfo,
    disabled: bool,
    on_save: EventHandler<PokemonInfo>,
    on_release: EventHandler<PokemonInfo>,
    on_close: EventHandler<()>,
) -> Element {
    let mut draft = use_signal(|| initial_data.clone());
    let pokemon_id = initial_data.id;
    let type_id = initial_data.type_id;

    // 查找种族名称
    let species_name = ADMIN_POKEMON_DATA
        .read()
        .items
        .iter()
        .find(|t| t.id == type_id)
        .map(|t| t.name.clone())
        .unwrap_or_else(|| "未知".to_string());

    // 获取当前枚举值的字符串表示（使用 serde 的 snake_case）
    let sex_value = serde_json::to_string(&draft.read().sex)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string();
    let site_value = serde_json::to_string(&draft.read().site)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string();
    let status_value = serde_json::to_string(&draft.read().status)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string();

    let handle_save = move |_| {
        on_save.call(draft.read().clone());
    };

    let handle_release = move |_| {
        on_release.call(draft.read().clone());
    };

    rsx! {
        ActionModal {
            title: format!("编辑宠物 #{} - {} #{}", pokemon_id, species_name, type_id),
            on_close: move |_| on_close.call(()),
            div { class: "admin-form-editor",
                // 基本信息
                FormSection { title: "基本信息".to_string(),
                    Row {
                        Col { span: 4,
                            TextField {
                                label: "昵称".to_string(),
                                value: draft.read().name.clone(),
                                placeholder: Some("宠物昵称".to_string()),
                                disabled,
                                on_change: move |v| draft.write().name = v,
                                help: None,
                            }
                        }
                        Col { span: 4,
                            SelectField {
                                label: "性别".to_string(),
                                value: sex_value.clone(),
                                options: sex_options(),
                                disabled,
                                on_change: move |v: String| {
                                    if let Ok(parsed) = serde_json::from_str(&format!("\"{}\"", v)) {
                                        draft.write().sex = parsed;
                                    }
                                },
                                help: None,
                            }
                        }
                        Col { span: 4,
                            BoolField {
                                label: "闪光".to_string(),
                                value: draft.read().is_shiny,
                                disabled,
                                on_change: move |v| draft.write().is_shiny = v,
                                help: None,
                            }
                        }
                    }
                    Row {
                        Col { span: 6,
                            SelectField {
                                label: "位置".to_string(),
                                value: site_value.clone(),
                                options: site_options(),
                                disabled,
                                on_change: move |v: String| {
                                    if let Ok(parsed) = serde_json::from_str(&format!("\"{}\"", v)) {
                                        draft.write().site = parsed;
                                    }
                                },
                                help: None,
                            }
                        }
                        Col { span: 6,
                            SelectField {
                                label: "状态".to_string(),
                                value: status_value.clone(),
                                options: status_options(),
                                disabled,
                                on_change: move |v: String| {
                                    if let Ok(parsed) = serde_json::from_str(&format!("\"{}\"", v)) {
                                        draft.write().status = parsed;
                                    }
                                },
                                help: None,
                            }
                        }
                    }
                }

                // 数值属性
                FormSection { title: "数值属性".to_string(),
                    Row {
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "等级".to_string(),
                                value: draft.read().level,
                                min: Some(1),
                                disabled,
                                on_change: move |v| draft.write().level = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "经验值".to_string(),
                                value: draft.read().experience,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().experience = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "亲密度".to_string(),
                                value: draft.read().intimacy,
                                min: Some(0),
                                max: Some(100),
                                disabled,
                                on_change: move |v| draft.write().intimacy = v,
                                help: None,
                                step: None,
                            }
                        }
                    }
                }

                // 个体值 (0-31)
                FormSection { title: "个体值 (0-31)".to_string(),
                    PokemonAttributesEditor {
                        value: draft.read().statistic,
                        disabled,
                        on_change: move |v| draft.write().statistic = v,
                    }
                }

                // 努力值 (0-255)
                FormSection { title: "努力值 (0-255)".to_string(),
                    PokemonAttributesEditor {
                        value: draft.read().base_points,
                        disabled,
                        on_change: move |v| draft.write().base_points = v,
                        max_value: 255,
                    }
                }

                // 操作按钮
                div { class: "admin-form-actions",
                    div { class: "admin-form-actions__left",
                        button {
                            class: "admin-btn admin-btn--danger",
                            disabled,
                            onclick: handle_release,
                            "放生宠物"
                        }
                    }
                    div { class: "admin-form-actions__right",
                        button {
                            class: "admin-btn",
                            disabled,
                            onclick: move |_| on_close.call(()),
                            "取消"
                        }
                        button {
                            class: "admin-btn admin-btn--primary",
                            disabled,
                            onclick: handle_save,
                            "保存修改"
                        }
                    }
                }
            }
        }
    }
}

/// 宠物属性编辑器组件（两行三列布局）
#[component]
fn PokemonAttributesEditor(
    value: PokemonAttributes,
    disabled: bool,
    #[props(default = 31)] max_value: u64,
    on_change: EventHandler<PokemonAttributes>,
) -> Element {
    let mut attrs = value;

    rsx! {
        Row { gap: "8px".to_string(),
            Col { span: 4,
                AttributeInput {
                    label: "HP".to_string(),
                    value: attrs.hit_points,
                    disabled,
                    max_value,
                    on_change: move |v| {
                        attrs.hit_points = v;
                        on_change.call(attrs);
                    },
                }
            }
            Col { span: 4,
                AttributeInput {
                    label: "攻击".to_string(),
                    value: attrs.attack,
                    disabled,
                    max_value,
                    on_change: move |v| {
                        attrs.attack = v;
                        on_change.call(attrs);
                    },
                }
            }
            Col { span: 4,
                AttributeInput {
                    label: "防御".to_string(),
                    value: attrs.defense,
                    disabled,
                    max_value,
                    on_change: move |v| {
                        attrs.defense = v;
                        on_change.call(attrs);
                    },
                }
            }
        }
        Row { gap: "8px".to_string(),
            Col { span: 4,
                AttributeInput {
                    label: "特攻".to_string(),
                    value: attrs.special_attack,
                    disabled,
                    max_value,
                    on_change: move |v| {
                        attrs.special_attack = v;
                        on_change.call(attrs);
                    },
                }
            }
            Col { span: 4,
                AttributeInput {
                    label: "特防".to_string(),
                    value: attrs.special_defense,
                    disabled,
                    max_value,
                    on_change: move |v| {
                        attrs.special_defense = v;
                        on_change.call(attrs);
                    },
                }
            }
            Col { span: 4,
                AttributeInput {
                    label: "速度".to_string(),
                    value: attrs.speed,
                    disabled,
                    max_value,
                    on_change: move |v| {
                        attrs.speed = v;
                        on_change.call(attrs);
                    },
                }
            }
        }
    }
}

#[component]
fn AttributeInput(
    label: String,
    value: u64,
    disabled: bool,
    max_value: u64,
    on_change: EventHandler<u64>,
) -> Element {
    rsx! {
        FieldWrapper { label, help: None,
            input {
                class: "admin-input admin-input--small",
                r#type: "number",
                min: "0",
                max: "{max_value}",
                value: "{value}",
                disabled,
                oninput: move |evt| {
                    if let Ok(num) = evt.value().parse::<u64>() {
                        on_change.call(num.min(max_value));
                    }
                },
            }
        }
    }
}

/// 编辑物品信息的可视化表单模态框
#[component]
fn EditItemInfoModal(
    initial_data: ItemInfo,
    disabled: bool,
    on_save: EventHandler<ItemInfo>,
    on_delete: EventHandler<ItemInfo>,
    on_close: EventHandler<()>,
) -> Element {
    let mut draft = use_signal(|| initial_data.clone());
    let item_id = initial_data.id;
    let type_id = initial_data.type_id;

    // 查找物品类型名称
    let type_name = ADMIN_ITEM_DATA
        .read()
        .items
        .iter()
        .find(|t| t.id == type_id)
        .map(|t| t.name.clone())
        .unwrap_or_else(|| "未知".to_string());

    let handle_save = move |_| {
        on_save.call(draft.read().clone());
    };

    let handle_delete = move |_| {
        on_delete.call(draft.read().clone());
    };

    rsx! {
        ActionModal {
            title: format!("编辑物品 #{} - {} #{}", item_id, type_name, type_id),
            on_close: move |_| on_close.call(()),
            div { class: "admin-form-editor",
                FormSection { title: "物品信息".to_string(),
                    Row {
                        Col { span: 6,
                            TextField {
                                label: "物品类型".to_string(),
                                value: format!("{} #{}", type_name, type_id),
                                placeholder: None,
                                disabled: true,
                                on_change: move |_| {},
                                help: None,
                            }
                        }
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "数量".to_string(),
                                value: draft.read().count,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().count = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                    }
                }

                div { class: "admin-form-actions",
                    div { class: "admin-form-actions__left",
                        button {
                            class: "admin-btn admin-btn--danger",
                            disabled,
                            onclick: handle_delete,
                            "删除物品"
                        }
                    }
                    div { class: "admin-form-actions__right",
                        button {
                            class: "admin-btn",
                            disabled,
                            onclick: move |_| on_close.call(()),
                            "取消"
                        }
                        button {
                            class: "admin-btn admin-btn--primary",
                            disabled,
                            onclick: handle_save,
                            "保存修改"
                        }
                    }
                }
            }
        }
    }
}

// ============ 确认模态框组件 ============

/// 放生宠物确认模态框
#[component]
fn ReleasePokemonConfirmModal(
    pokemon_info: PokemonInfo,
    disabled: bool,
    on_confirm: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    let pokemon_id = pokemon_info.id;
    let type_id = pokemon_info.type_id;

    // 查找种族名称
    let species_name = ADMIN_POKEMON_DATA
        .read()
        .items
        .iter()
        .find(|t| t.id == type_id)
        .map(|t| t.name.clone())
        .unwrap_or_else(|| "未知".to_string());

    let nickname = if pokemon_info.name.is_empty() {
        "（未命名）".to_string()
    } else {
        pokemon_info.name.clone()
    };

    rsx! {
        ActionModal {
            title: "确认放生宠物".to_string(),
            on_close: move |_| on_close.call(()),
            div { class: "admin-confirm-dialog",
                div { class: "admin-confirm-dialog__warning",
                    div { class: "admin-confirm-dialog__icon", "!" }
                    div { class: "admin-confirm-dialog__content",
                        p { class: "admin-confirm-dialog__title", "确定要放生这只宠物吗？" }
                        p { class: "admin-confirm-dialog__info",
                            "宠物 #{pokemon_id} {nickname} - {species_name} #{type_id} 等级 {pokemon_info.level}"
                        }
                        p { class: "admin-confirm-dialog__hint", "此操作不可撤销！" }
                    }
                }

                div { class: "admin-form-actions",
                    button {
                        class: "admin-btn",
                        disabled,
                        onclick: move |_| on_close.call(()),
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn--danger",
                        disabled,
                        onclick: move |_| on_confirm.call(()),
                        "确认放生"
                    }
                }
            }
        }
    }
}

/// 删除物品模态框
#[component]
fn DeleteItemModal(
    item_info: ItemInfo,
    disabled: bool,
    on_confirm: EventHandler<u64>,
    on_close: EventHandler<()>,
) -> Element {
    let type_id = item_info.type_id;
    let max_count = item_info.count;

    // 查找物品类型名称
    let type_name = ADMIN_ITEM_DATA
        .read()
        .items
        .iter()
        .find(|t| t.id == type_id)
        .map(|t| t.name.clone())
        .unwrap_or_else(|| "未知".to_string());

    let mut delete_count = use_signal(|| max_count);
    let current_count = delete_count();
    let is_valid = current_count >= 1 && current_count <= max_count;

    rsx! {
        ActionModal {
            title: format!("删除物品 - {} (持有 {} 个)", type_name, max_count),
            on_close: move |_| on_close.call(()),
            div { class: "admin-confirm-dialog",
                FormSection { title: "删除数量".to_string(),
                    Row {
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "删除数量".to_string(),
                                value: current_count,
                                min: Some(1),
                                max: Some(max_count),
                                step: None,
                                help: Some(format!("最大可删除 {} 个", max_count)),
                                disabled,
                                on_change: move |v: u64| delete_count.set(v.min(max_count).max(1)),
                            }
                        }
                        Col { span: 6,
                            div { class: "admin-field",
                                label { class: "admin-field__label", "快捷操作" }
                                div { class: "admin-quick-actions",
                                    button {
                                        class: "admin-btn admin-btn--small",
                                        disabled: disabled || current_count == 1,
                                        onclick: move |_| delete_count.set(1),
                                        "删除1个"
                                    }
                                    button {
                                        class: "admin-btn admin-btn--small",
                                        disabled: disabled || max_count <= 10 || current_count == 10.min(max_count),
                                        onclick: move |_| delete_count.set(10.min(max_count)),
                                        "删除10个"
                                    }
                                    button {
                                        class: "admin-btn admin-btn--small",
                                        disabled: disabled || current_count == max_count,
                                        onclick: move |_| delete_count.set(max_count),
                                        "全部删除"
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "admin-form-actions",
                    button {
                        class: "admin-btn",
                        disabled,
                        onclick: move |_| on_close.call(()),
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn--danger",
                        disabled: disabled || !is_valid,
                        onclick: move |_| {
                            if is_valid {
                                on_confirm.call(current_count);
                            }
                        },
                        if current_count >= max_count {
                            "全部删除"
                        } else {
                            "确认删除"
                        }
                    }
                }
            }
        }
    }
}

/// 用户元数据编辑器（嵌入附属数据 modal 内，无独立 modal 外壳）
#[component]
fn UserMetaEditor(
    initial_data: UserInfo,
    disabled: bool,
    on_save: EventHandler<UserInfo>,
) -> Element {
    let mut draft = use_signal(|| initial_data.clone());
    let user_id = initial_data.id;
    let username = initial_data.name.clone();

    let handle_save = move |_| {
        on_save.call(draft.read().clone());
    };

    rsx! {
        div { class: "admin-modal-section admin-modal-section--user-meta",
            div { class: "admin-form-editor",
                FormSection { title: "基本信息".to_string(),
                    Row {
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "金钱".to_string(),
                                value: draft.read().money,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().money = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "经验值".to_string(),
                                value: draft.read().experience,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().experience = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "战力等级".to_string(),
                                value: draft.read().strength,
                                min: Some(1),
                                disabled,
                                on_change: move |v| draft.write().strength = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                    }
                }

                FormSection { title: "战绩".to_string(),
                    Row {
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "胜场".to_string(),
                                value: draft.read().win_count,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().win_count = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "负场".to_string(),
                                value: draft.read().lose_count,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().lose_count = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "总场次".to_string(),
                                value: draft.read().total_battles,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().total_battles = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                    }
                }

                FormSection { title: "高级属性".to_string(),
                    Row {
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "战力值".to_string(),
                                value: draft.read().str,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().str = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "仓库容量".to_string(),
                                value: draft.read().boxnum,
                                min: Some(1),
                                disabled,
                                on_change: move |v| draft.write().boxnum = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "魅力值".to_string(),
                                value: draft.read().allure,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().allure = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                    }
                }

                div { class: "admin-form-actions",
                    div { class: "admin-form-actions__left" }
                    div { class: "admin-form-actions__right",
                        button {
                            class: "admin-btn admin-btn--primary",
                            disabled,
                            onclick: handle_save,
                            "保存用户信息"
                        }
                    }
                }
            }
        }
    }
}
