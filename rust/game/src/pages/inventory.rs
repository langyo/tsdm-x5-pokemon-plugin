use crate::prelude::*;

use crate::{
    components::{
        common::Modal,
        layout::{Page, CURRENT_PAGE, IMG_PATH, INITIAL_INVENTORY_CATEGORY},
    },
    state::{
        refresh_inventory_state, refresh_pokemon_list, show_error, show_success, show_warning,
        use_battle_state,
    },
    utils::api_client::NewApiClient,
};

const ITEM_TYPE_DRUG: u64 = 1;
const ITEM_TYPE_BALL: u64 = 2;
const ITEM_TYPE_EVOLUTION: u64 = 3;
const ITEM_TYPE_ENHANCE: u64 = 4;
const ITEM_TYPE_EQUIP: u64 = 5;

#[derive(Clone, PartialEq)]
enum ItemTargetType {
    #[allow(dead_code)]
    None,
    Pokemon,
    Battle,
    Global,
}

#[component]
pub fn Inventory() -> Element {
    let mut current_page = use_signal(|| 1usize);
    let mut search_input = use_signal(String::new);
    let mut search_query = use_signal(String::new);
    let mut current_category = use_signal(|| {
        let initial = *INITIAL_INVENTORY_CATEGORY.read();
        *INITIAL_INVENTORY_CATEGORY.write() = None;
        initial.or(Some(1))
    });
    let mut using_item = use_signal(|| None::<u64>);
    let mut scroll_container: Signal<Option<web_sys::Element>> = use_signal(|| None);

    let mut selected_item = use_signal(|| None::<(u64, String, u64)>);
    let mut show_target_modal = use_signal(|| false);
    let mut pokemon_list_error_shown = use_signal(|| false);

    let mut usable_pokemon_list = use_resource(move || {
        let selection = selected_item.read().clone();
        async move {
            if let Some((item_id, _, _)) = selection {
                let api = NewApiClient::new();
                Some(api.get_usable_pokemon(item_id).await)
            } else {
                None
            }
        }
    });

    // 监听资源状态变化，显示错误 toast
    use_effect(move || {
        if let Some(Some(Err(e))) = usable_pokemon_list.read().as_ref() {
            if !*pokemon_list_error_shown.read() {
                pokemon_list_error_shown.set(true);
                show_error(format!("加载宠物列表失败: {}", e));
            }
        }
    });

    // 优先使用服务端给出的 use_target（容量箱子等训练家级道具是 global，不需要选宠）；
    // 旧服务端没有该字段时回退到按物品类型判断。
    let get_item_target_type = |type_id: u64, use_target: Option<&str>| -> ItemTargetType {
        match use_target {
            Some("pokemon") => return ItemTargetType::Pokemon,
            Some("battle") => return ItemTargetType::Battle,
            Some("global") => return ItemTargetType::Global,
            _ => {}
        }
        match type_id {
            ITEM_TYPE_DRUG => ItemTargetType::Pokemon,
            ITEM_TYPE_BALL => ItemTargetType::Battle,
            ITEM_TYPE_EVOLUTION => ItemTargetType::Pokemon,
            ITEM_TYPE_ENHANCE => ItemTargetType::Pokemon,
            ITEM_TYPE_EQUIP => ItemTargetType::Pokemon,
            _ => ItemTargetType::Global,
        }
    };

    let mut inventory_data = use_resource(move || async move {
        let api = NewApiClient::new();
        let category = *current_category.read();
        let page = *current_page.read();
        let search = search_query.read().clone();
        let result = api
            .get_user_inventory_search(category, page as u32, &search)
            .await
            .map_err(|e| format!("加载失败: {}", e));
        result
    });

    let inventory_state = || {
        let state = inventory_data.read();
        state.clone()
    };

    let scroll_to_top = move || {
        if let Some(elem) = scroll_container.read().as_ref() {
            elem.set_scroll_top(0);
        }
    };

    let mut apply_search = move || {
        search_query.set(search_input.read().trim().to_string());
        current_page.set(1);
        scroll_to_top();
    };

    use_effect(move || {
        if let Some(Ok(data)) = inventory_data.read().as_ref() {
            let last_page = data.total_pages.max(1);
            if *current_page.peek() > last_page {
                current_page.set(last_page);
            }
        }
    });

    let mut use_item =
        move |item_id: u64, item_type_id: u64, item_name: String, use_target: Option<String>| {
            if using_item.read().is_some() {
                return;
            }
            let target_type = get_item_target_type(item_type_id, use_target.as_deref());

            // 装备道具特殊处理：跳转到个人中心装备页面
            if item_type_id == ITEM_TYPE_EQUIP {
                *crate::state::MY_POKEMON_TAB.write() = crate::state::MyPokemonTab::Equipment;
                *CURRENT_PAGE.write() = Page::MyPokemon;
                return;
            }

            match target_type {
                ItemTargetType::Pokemon => {
                    selected_item.set(Some((item_id, item_name, item_type_id)));
                    show_target_modal.set(true);
                    // 重启资源以加载可用宠物列表
                    usable_pokemon_list.restart();
                }
                ItemTargetType::Global => {
                    using_item.set(Some(item_id));
                    spawn(async move {
                        let api = NewApiClient::new();
                        match api.use_item(item_id, None).await {
                            Ok(result) => {
                                show_success(result.message);
                                inventory_data.restart();
                                refresh_inventory_state();
                            }
                            Err(e) => {
                                show_error(format!("使用失败: {}", e));
                            }
                        }
                        using_item.set(None);
                    });
                }
                ItemTargetType::Battle => {
                    show_warning("精灵球请在战斗中使用");
                }
                ItemTargetType::None => {
                    show_warning("该物品无法使用");
                }
            }
        };

    let mut confirm_use_on_pokemon = move |pokemon_id: u64| {
        if using_item.read().is_some() {
            return;
        }
        if let Some((item_id, _item_name, _)) = selected_item.read().as_ref() {
            let target_item_id = *item_id;
            using_item.set(Some(target_item_id));
            show_target_modal.set(false);

            spawn(async move {
                let api = NewApiClient::new();
                match api.use_item(target_item_id, Some(pokemon_id)).await {
                    Ok(result) => {
                        show_success(result.message);
                        selected_item.set(None);
                        inventory_data.restart();
                        refresh_pokemon_list();
                        refresh_inventory_state();
                    }
                    Err(e) => {
                        show_error(format!("使用失败: {}", e));
                    }
                }
                using_item.set(None);
            });
        }
    };

    let categories = [
        (None, "全部物品", "item/box.gif"),
        (Some(1u32), "回复药", "item/hp.gif"),
        (Some(2), "精灵球", "item/jlq.gif"),
        (Some(3), "进化石", "item/szs.gif"),
        (Some(4), "强化道具", "item/atkg.gif"),
        (Some(5), "装备道具", "item/zb01.gif"),
    ];

    let items_to_display = inventory_data
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();

    rsx! {
        div { class: "page-inventory",
            div { class: "inventory-card",
                // 战斗中的遮罩层
                if use_battle_state() {
                    div { class: "battle-disabled-overlay",
                        div { class: "battle-disabled-message",
                            span { class: "battle-icon", "⚔️" }
                            p { "战斗中无法使用背包" }
                            p { class: "battle-hint", "请先结束当前战斗" }
                        }
                    }
                }

                div { class: "inventory-card-body",
                    div { class: "inventory-search-bar",
                        input {
                            id: "inventory-search", r#type: "search",
                            placeholder: "按物品名称搜索", "aria-label": "搜索背包物品名称",
                            value: "{search_input.read()}",
                            oninput: move |evt| search_input.set(evt.value()),
                            onkeydown: move |evt| {
                                if evt.key() == Key::Enter { apply_search(); }
                            },
                        }
                        button { id: "inventory-search-submit", onclick: move |_| apply_search(), "搜索" }
                        button {
                            id: "inventory-search-clear",
                            onclick: move |_| {
                                search_input.set(String::new());
                                search_query.set(String::new());
                                current_page.set(1);
                                scroll_to_top();
                            },
                            "清除"
                        }
                    }
                    div { class: "shop-category-bar",
                        for (cat_id , cat_name , cat_icon) in &categories {
                            {
                                let cid = *cat_id;
                                let active = *current_category.read() == cid;
                                rsx! {
                                    button {
                                        class: if active { "category-btn active" } else { "category-btn" },
                                        onclick: move |_| {
                                            current_category.set(cid);
                                            current_page.set(1);
                                            scroll_to_top();
                                        },
                                        span { class: "cat-icon",
                                            img {
                                                class: "cat-icon-img",
                                                src: "{IMG_PATH}/{cat_icon}",
                                                alt: "{cat_name}",
                                            }
                                        }
                                        span { "{cat_name}" }
                                    }
                                }
                            }
                        }
                    }

                    if inventory_state().is_none() {
                        div { class: "shop-loading", "正在加载背包..." }
                    } else if let Some(Err(message)) = inventory_state() {
                        div { class: "shop-empty", role: "alert", "{message}" }
                    } else if let Some(data) = items_to_display {
                        p { id: "inventory-result-count", class: "inventory-result-count", "当前分类共 {data.total} 项" }
                        if data.items.is_empty() {
                            div { class: "shop-empty",
                                img {
                                    class: "empty-icon-img",
                                    src: "{IMG_PATH}/item/box.gif",
                                    alt: "",
                                }
                                if search_query.read().is_empty() {
                                    p { "此分类没有物品" }
                                } else {
                                    p { "没有找到匹配的物品，试试其他名称或分类" }
                                }
                            }
                        } else {
                            div {
                                class: "shop-items-scroll",
                                onmounted: move |cx| {
                                    if let Some(elem) = cx.data.downcast::<web_sys::Element>() {
                                        scroll_container.set(Some(elem.clone()));
                                    }
                                },
                                div { class: "shop-items-grid",
                                    {
                                        data.items
                                            .into_iter()
                                            .enumerate()
                                            .map(|(idx, item)| {
                                                // type_id 是物品类型 ID（pm_itemdata.id），用于 API 调用
                                                let item_type_id = item.type_id;
                                                // item_type 是物品类型（1=回复药，2=精灵球，3=进化石，4=强化道具，5=装备道具）
                                                let item_category = item.item_type;
                                                let item_name = item.name.clone();
                                                let item_name2 = item.name.clone();
                                                let item_desc = item.description.clone();
                                                let item_type_name = item.type_name.clone();
                                                let item_image = item.image.clone();
                                                let is_using = *using_item.read() == Some(item_type_id);
                                                let any_using = using_item.read().is_some();
                                                let item_use_target = item.use_target.clone();
                                                let target_type = get_item_target_type(item_category, item_use_target.as_deref());

                                                // 精灵球类型不显示使用按钮
                                                let show_button = target_type != ItemTargetType::Battle;
                                                let usable = show_button
                                                    && matches!(
                                                        target_type,
                                                        ItemTargetType::Pokemon | ItemTargetType::Global
                                                    );
                                                let button_text = if is_using { "使用中..." } else { "使用" };
                                                let button_class = if usable && !is_using && !any_using {
                                                    "buy-btn"
                                                } else {
                                                    "buy-btn disabled"
                                                };
                                                rsx! {
                                                    div { key: "{idx}", class: "shop-item-card", "data-item-id": "{item_type_id}",
                                                        div { class: "item-icon-area",
                                                            img {
                                                                class: "item-icon-img",
                                                                src: "{IMG_PATH}/item/{item_image}.gif",
                                                                alt: "{item_name}",
                                                            }
                                                        }
                                                        div { class: "item-info",
                                                            div { class: "item-name", "{item_name}" }
                                                            div { class: "item-desc", "{item_desc}" }
                                                            div { class: "item-meta",
                                                                span { class: "item-type-badge", "{item_type_name}" }
                                                                span { class: "item-type-badge", "×{item.quantity}" }
                                                            }
                                                        }
                                                        div { class: "item-action",
                                                            if show_button {
                                                                button {
                                                                    class: "{button_class}",
                                                                    disabled: !usable || is_using || any_using,
                                                                    onclick: move |_| use_item(item_type_id, item_category, item_name2.clone(), item_use_target.clone()),
                                                                    "{button_text}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            })
                                    }
                                }
                            }

                            if data.total_pages > 1 {
                                div { class: "shop-pagination",
                                    button {
                                        class: "page-btn",
                                        disabled: *current_page.read() <= 1,
                                        onclick: move |_| {
                                            let p = (*current_page.read()).saturating_sub(1).max(1);
                                            current_page.set(p);
                                            scroll_to_top();
                                        },
                                        "← 上一页"
                                    }
                                    span { class: "page-info",
                                        "第 {data.page} / {data.total_pages} 页"
                                    }
                                    button {
                                        class: "page-btn",
                                        disabled: *current_page.read() >= data.total_pages,
                                        onclick: move |_| {
                                            let p = *current_page.read() + 1;
                                            current_page.set(p);
                                            scroll_to_top();
                                        },
                                        "下一页 →"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Modal {
                is_open: *show_target_modal.read(),
                on_close: move |_| {
                    show_target_modal.set(false);
                    selected_item.set(None);
                    pokemon_list_error_shown.set(false);
                },
                title: {
                    if let Some((_, item_name, _)) = selected_item.read().as_ref() {
                        format!("选择宝可梦 - {}", item_name)
                    } else {
                        "选择宝可梦".to_string()
                    }
                },
                {
                    let resource_value = usable_pokemon_list.read();
                    let modal_content = resource_value.as_ref().and_then(|r| r.as_ref());
                    match modal_content {
                        Some(Ok(data)) => {
                            if data.usable_pokemon.is_empty() {
                                rsx! {
                                    div { class: "pokemon-select-modal",
                                        div { class: "empty-pokemon-list",
                                            "没有可以使用该物品的宝可梦"
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "pokemon-select-modal",
                                        div { class: "pokemon-select-list custom-scrollbar",
                                            for p in &data.usable_pokemon {
                                                {
                                                    let pokemon_id = p.id;
                                                    let pokemon_level = p.level;
                                                    let pokemon_hp = p.hp;
                                                    let pokemon_max_hp = p.max_hp;
                                                    let hp_percent = if pokemon_max_hp > 0 {
                                                        ((pokemon_hp as f64 / pokemon_max_hp as f64) * 100.0) as u32
                                                    } else {
                                                        0
                                                    };
                                                    let hp_color = if hp_percent < 30 { "#e74c3c" } else { "#2ecc71" };

                                                    let display_name = p.nickname.clone();

                                                    rsx! {
                                                        div {
                                                            class: "pokemon-select-item",
                                                            onclick: move |_| confirm_use_on_pokemon(pokemon_id),
                                                            div { class: "pokemon-select-info",
                                                                span { class: "pokemon-select-name", "{display_name}" }
                                                                span { class: "pokemon-select-level", "Lv.{pokemon_level}" }
                                                            }
                                                            div { class: "hp-bar-container",
                                                                div {
                                                                    class: "hp-bar",
                                                                    style: "width: {hp_percent}%; background: {hp_color};",
                                                                }
                                                                span { class: "hp-text", "{pokemon_hp}/{pokemon_max_hp}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            // 显示无法使用的宠物数量
                                            if data.unusable_count > 0 {
                                                div { class: "unusable-pokemon-hint",
                                                    "剩余 {data.unusable_count} 个宠物无法使用该物品"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(Err(_)) => {
                            rsx! {
                                div { class: "pokemon-select-modal",
                                    div { class: "error-pokemon-list",
                                        "加载宠物列表失败，请关闭后重试"
                                    }
                                }
                            }
                        }
                        None => {
                            rsx! {
                                div { class: "pokemon-select-modal",
                                    div { class: "loading-pokemon-list", "正在加载可用宠物列表..." }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
