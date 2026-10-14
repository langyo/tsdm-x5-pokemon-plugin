use crate::prelude::*;

use crate::{
    components::layout::IMG_PATH,
    hooks::{use_shop, use_shop_pets},
    state::{
        refresh_inventory_state, refresh_pokemon_list, refresh_user_profile_state, show_error,
        show_success, use_battle_state,
    },
    utils::api_client::NewApiClient,
};

const PET_CATEGORY: Option<u32> = None;

#[component]
pub fn Shop() -> Element {
    let mut current_page = use_signal(|| 1u32);
    let mut current_category = use_signal(|| Some(1u32));
    let buying = use_signal(|| false);
    let search_input = use_signal(String::new);
    let filter_open = use_signal(|| false);

    let shop_data = use_shop(current_category, current_page, search_input);
    let pet_shop_data = use_shop_pets(current_page, search_input);

    let categories = [
        (Some(1u32), "回复药", "item/hp.gif"),
        (Some(2), "精灵球", "item/jlq.gif"),
        (Some(3), "进化石", "item/szs.gif"),
        (Some(4), "强化道具", "item/atkg.gif"),
        (Some(5), "装备道具", "item/zb01.gif"),
        (PET_CATEGORY, "宠物", "spm/25.gif"),
    ];

    let is_battle_active = use_battle_state();

    // 服务端是分页总数的最终来源：筛选/切类后页数收缩时把当前页夹回去。
    use_effect(move || {
        let last_page = if *current_category.read() == PET_CATEGORY {
            pet_shop_data
                .read()
                .as_ref()
                .and_then(|r| r.as_ref().ok())
                .map(|d| d.total_pages.max(1))
                .unwrap_or(1)
        } else {
            shop_data
                .read()
                .as_ref()
                .and_then(|r| r.as_ref().ok())
                .map(|d| d.total_pages.max(1))
                .unwrap_or(1)
        };
        if ((*current_page.peek()) as usize) > last_page {
            current_page.set(last_page as u32);
        }
    });

    rsx! {
        div { class: "page-shop",
            div { class: "shop-card",
                if is_battle_active {
                    div { class: "battle-disabled-overlay",
                        div { class: "battle-disabled-message",
                            span { class: "battle-icon", "⚔️" }
                            p { "战斗中无法使用商店" }
                            p { class: "battle-hint", "请先结束当前战斗" }
                        }
                    }
                }

                div { class: "shop-card-header",
                    span { class: "header-icon-emoji", "🛒" }
                    span { "道具商店" }
                }

                div { class: "shop-card-body",
                    div { class: "shop-category-bar",
                        div { class: "shop-category-group",
                            for (cat_id , cat_name , cat_icon) in &categories {
                                {
                                    let cid = *cat_id;
                                    let active = *current_category.read() == cid;
                                    let is_pet = cid == PET_CATEGORY;
                                    rsx! {
                                        button {
                                            class: if active { "category-btn active" } else { "category-btn" },
                                            disabled: *buying.read(),
                                            onclick: move |_| {
                                                if *buying.read() { return; }
                                                current_category.set(cid);
                                                current_page.set(1);
                                            },
                                            span { class: "cat-icon",
                                                if is_pet {
                                                    img {
                                                        class: "cat-icon-img pet-cat-icon",
                                                        src: "{IMG_PATH}/{cat_icon}",
                                                        alt: "{cat_name}",
                                                    }
                                                } else {
                                                    img {
                                                        class: "cat-icon-img",
                                                        src: "{IMG_PATH}/{cat_icon}",
                                                        alt: "{cat_name}",
                                                    }
                                                }
                                            }
                                            span { "{cat_name}" }
                                        }
                                    }
                                }
                            }
                        }

                        ShopNameFilter {
                            filter_open,
                            search_input,
                            on_keyword_change: move |_| {
                                current_page.set(1);
                            },
                        }
                    }

                    if *current_category.read() == PET_CATEGORY {
                        ShopPetsTab {
                            buying,
                            current_page,
                            pet_shop_data,
                            search_input,
                        }
                    } else {
                        ShopItemsTab {
                            buying,
                            current_page,
                            shop_data,
                            search_input,
                        }
                    }
                }
            }
        }
    }
}

/// 分类栏右侧的名称筛选器：点击展开带编辑框的悬浮面板，
/// 输入内容即时（防抖后）触发明细模糊匹配，按钮图标随筛选状态变化。
#[component]
fn ShopNameFilter(
    mut filter_open: Signal<bool>,
    mut search_input: Signal<String>,
    on_keyword_change: EventHandler<()>,
) -> Element {
    let keyword = search_input.read().clone();
    // 与服务端一致：纯空格不算有效筛选，图标状态跟随 trim 后的判定。
    let has_value = !keyword.trim().is_empty();

    rsx! {
        div { class: "shop-filter",
            button {
                // has-value 只由“有输入内容”决定（图标随之切换）；open 仅表示面板展开。
                class: match (has_value, *filter_open.read()) {
                    (true, true) => "shop-filter-btn has-value open",
                    (true, false) => "shop-filter-btn has-value",
                    (false, true) => "shop-filter-btn open",
                    (false, false) => "shop-filter-btn",
                },
                r#type: "button",
                title: "按名称筛选商品",
                aria_label: "按名称筛选商品",
                aria_haspopup: "dialog",
                aria_expanded: "{*filter_open.read()}",
                onclick: move |_| {
                    let next = !*filter_open.read();
                    filter_open.set(next);
                },
                if has_value {
                    // 有输入内容：实心漏斗 + 角标圆点。
                    span { class: "filter-icon filter-icon-filled",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            width: "16",
                            height: "16",
                            fill: "currentColor",
                            path { d: "M3 4.5h18l-6.8 7.6v7.4l-4.4-2.4v-5L3 4.5z" }
                        }
                    }
                    span { class: "filter-value-dot" }
                } else {
                    span { class: "filter-icon filter-icon-outline",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            width: "16",
                            height: "16",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linejoin: "round",
                            path { d: "M3 4.5h18l-6.8 7.6v7.4l-4.4-2.4v-5L3 4.5z" }
                        }
                    }
                }
            }

            if *filter_open.read() {
                div { class: "shop-filter-backdrop", onclick: move |_| filter_open.set(false) }
                div { class: "shop-filter-popover", role: "dialog", aria_label: "名称筛选",
                    label { class: "shop-filter-label", r#for: "shop-filter-input", "按名称筛选" }
                    div { class: "shop-filter-row",
                        input {
                            id: "shop-filter-input",
                            class: "shop-filter-input",
                            r#type: "search",
                            placeholder: "输入名称，动态匹配",
                            value: "{keyword}",
                            autocomplete: "off",
                            autofocus: true,
                            oninput: move |evt| {
                                search_input.set(evt.value());
                                on_keyword_change.call(());
                            },
                            onkeydown: move |evt| {
                                if evt.key() == Key::Escape {
                                    filter_open.set(false);
                                }
                            },
                        }
                        if has_value {
                            button {
                                id: "shop-filter-clear",
                                class: "shop-filter-clear",
                                r#type: "button",
                                title: "清除筛选",
                                onclick: move |_| {
                                    search_input.set(String::new());
                                    on_keyword_change.call(());
                                },
                                "✕"
                            }
                        }
                    }
                    p { class: "shop-filter-hint", "留空显示当前分类全部商品" }
                }
            }
        }
    }
}

/// 居中换页器：数字页码 + 首尾省略，替代原先的页内滚动。
#[component]
fn ShopPaginator(mut current_page: Signal<u32>, total_pages: usize) -> Element {
    let current = (*current_page.read()).max(1) as usize;
    let pages = windowed_pages(current, total_pages);
    let total = total_pages.max(1);

    rsx! {
        div { class: "shop-pagination",
            button {
                class: "pager-btn pager-nav",
                r#type: "button",
                disabled: current <= 1,
                aria_label: "上一页",
                onclick: move |_| {
                    let p = (*current_page.read()).saturating_sub(1).max(1);
                    current_page.set(p);
                },
                "‹"
            }
            for entry in pages {
                if let Some(p) = entry {
                    {
                        let active = p == current;
                        rsx! {
                            button {
                                key: "page-{p}",
                                class: if active { "pager-btn pager-num active" } else { "pager-btn pager-num" },
                                r#type: "button",
                                disabled: total == 1,
                                aria_current: if active { "page" } else { "" },
                                onclick: move |_| current_page.set(p as u32),
                                "{p}"
                            }
                        }
                    }
                } else {
                    span { class: "pager-ellipsis", "…" }
                }
            }
            button {
                class: "pager-btn pager-nav",
                r#type: "button",
                disabled: current >= total,
                aria_label: "下一页",
                onclick: move |_| {
                    let p = (*current_page.read() + 1).min(total as u32);
                    current_page.set(p);
                },
                "›"
            }
        }
    }
}

/// 页码窗口：总页数不多时全部展示，否则保留首尾页并给出省略号。
fn windowed_pages(current: usize, total: usize) -> Vec<Option<usize>> {
    let total = total.max(1);
    let current = current.clamp(1, total);
    if total <= 7 {
        return (1..=total).map(Some).collect();
    }
    let window_start = current.saturating_sub(1).max(2);
    let window_end = (current + 1).min(total - 1);
    let mut pages: Vec<Option<usize>> = vec![Some(1)];
    if window_start > 2 {
        pages.push(None);
    }
    pages.extend((window_start..=window_end).map(Some));
    if window_end < total - 1 {
        pages.push(None);
    }
    pages.push(Some(total));
    pages
}

#[component]
fn ShopItemsTab(
    mut buying: Signal<bool>,
    current_page: Signal<u32>,
    shop_data: Resource<Result<_utils::types::api_shop::ShopListResponse, String>>,
    search_input: Signal<String>,
) -> Element {
    let data_to_display = shop_data
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();

    rsx! {
        if shop_data.read().is_none() {
            div { class: "shop-loading", "正在加载商品..." }
        } else if let Some(data) = data_to_display {
            if data.items.is_empty() {
                div { class: "shop-empty",
                    img {
                        class: "empty-icon-img",
                        src: "{IMG_PATH}/item/box.gif",
                        alt: "",
                    }
                    if search_input.read().trim().is_empty() {
                        p { "暂无商品" }
                    } else {
                        p { "没有找到匹配的商品，试试其他名称或分类" }
                    }
                }
            } else {
                div { class: "shop-tiles-grid",
                    for (idx, item) in data.items.into_iter().enumerate() {
                        ShopItemTile {
                            key: "{idx}",
                            item,
                            buying,
                        }
                    }
                }

                ShopPaginator { current_page, total_pages: data.total_pages }
            }
        } else if let Some(err) = shop_data.read().as_ref().and_then(|r| r.as_ref().err()) {
            div { class: "error-message", "加载失败: {err}" }
        }
    }
}

/// 单个商品宫格：悬浮展示详情 tooltip。
#[component]
fn ShopItemTile(item: _utils::types::api_shop::ShopItem, mut buying: Signal<bool>) -> Element {
    let mut show_tooltip = use_signal(|| false);

    rsx! {
        div {
            class: "shop-tile",
            onmouseenter: move |_| show_tooltip.set(true),
            onmouseleave: move |_| show_tooltip.set(false),
            div { class: "shop-tile-icon",
                img {
                    class: "tile-icon-img",
                    src: "{IMG_PATH}/item/{item.image}.gif",
                    alt: "{item.name}",
                }
            }
            div { class: "shop-tile-name", title: "{item.name}", "{item.name}" }
            div { class: "shop-tile-meta",
                span { class: "item-type-badge", "{item.type_name}" }
            }
            div { class: "shop-tile-foot",
                div { class: "item-price",
                    span { class: "price-icon-emoji", "💰" }
                    span { class: "price-value", "{item.price}" }
                }
                button {
                    class: if item.can_buy { "buy-btn" } else { "buy-btn disabled" },
                    disabled: !item.can_buy || *buying.read(),
                    onclick: move |_| {
                        if !item.can_buy || *buying.read() || use_battle_state() { return; }
                        buying.set(true);
                        let name_clone = item.name.clone();
                        let api = NewApiClient::new();
                        let do_refresh = || {
                            refresh_user_profile_state();
                            refresh_inventory_state();
                        };
                        spawn(async move {
                            match api.buy_item(item.id, 1).await {
                                Ok(resp) => {
                                    show_success(
                                        format!(
                                            "成功购买 {} ×1！余额: {}",
                                            name_clone,
                                            resp.remaining_money,
                                        ),
                                    );
                                }
                                Err(e) => {
                                    show_error(format!("购买失败: {}", e));
                                }
                            }
                            do_refresh();
                            buying.set(false);
                        });
                    },
                    "购买"
                }
            }
            if *show_tooltip.read() {
                ShopItemTooltip { item: item.clone() }
            }
        }
    }
}

/// 商品详情 Tooltip（fixed 定位，不拦截鼠标事件）。
#[component]
fn ShopItemTooltip(item: _utils::types::api_shop::ShopItem) -> Element {
    let effect = &item.effect;
    rsx! {
        div { class: "shop-tile-tooltip",
            div { class: "tile-tooltip-header",
                span { class: "tile-tooltip-name", "{item.name}" }
                span { class: "item-type-badge", "{item.type_name}" }
            }
            if !item.description.is_empty() {
                p { class: "tile-tooltip-desc", "{item.description}" }
            }
            div { class: "tile-tooltip-stats",
                if effect.addhp != 0 {
                    span { class: "tile-tooltip-stat", "回复HP +{effect.addhp}" }
                }
                if effect.addexp != 0 {
                    span { class: "tile-tooltip-stat", "经验 +{effect.addexp}" }
                }
                if effect.addlv != 0 {
                    span { class: "tile-tooltip-stat", "等级 +{effect.addlv}" }
                }
            }
            div { class: "tile-tooltip-price",
                span { class: "price-icon-emoji", "💰" }
                span { "{item.price}" }
            }
        }
    }
}

#[component]
fn ShopPetsTab(
    mut buying: Signal<bool>,
    current_page: Signal<u32>,
    pet_shop_data: Resource<Result<_utils::types::api_shop::ShopPetListResponse, String>>,
    search_input: Signal<String>,
) -> Element {
    let data_to_display = pet_shop_data
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();

    rsx! {
        if pet_shop_data.read().is_none() {
            div { class: "shop-loading", "正在加载宠物..." }
        } else if let Some(data) = data_to_display {
            if data.pets.is_empty() {
                div { class: "shop-empty",
                    img {
                        class: "empty-icon-img",
                        src: "{IMG_PATH}/item/box.gif",
                        alt: "",
                    }
                    if search_input.read().trim().is_empty() {
                        p { "暂无可购买的宠物" }
                    } else {
                        p { "没有找到匹配的宠物，试试其他名称" }
                    }
                }
            } else {
                div { class: "shop-tiles-grid",
                    for (idx, pet) in data.pets.into_iter().enumerate() {
                        ShopPetTile {
                            key: "{idx}",
                            pet,
                            buying,
                        }
                    }
                }

                ShopPaginator { current_page, total_pages: data.total_pages }
            }
        } else if let Some(err) = pet_shop_data.read().as_ref().and_then(|r| r.as_ref().err()) {
            div { class: "error-message", "加载失败: {err}" }
        }
    }
}

/// 单个宠物宫格：悬浮展示种族值 tooltip。
#[component]
fn ShopPetTile(pet: _utils::types::api_shop::ShopPet, mut buying: Signal<bool>) -> Element {
    let mut show_tooltip = use_signal(|| false);

    rsx! {
        div {
            class: "shop-tile pet-tile",
            onmouseenter: move |_| show_tooltip.set(true),
            onmouseleave: move |_| show_tooltip.set(false),
            div { class: "shop-tile-icon",
                img {
                    class: "tile-icon-img pet-sprite-img",
                    src: "{IMG_PATH}/spm/{pet.id}.gif",
                    alt: "{pet.name}",
                }
            }
            div { class: "shop-tile-name", title: "{pet.name}", "{pet.name}" }
            div { class: "shop-tile-meta",
                span { class: "badge badge-type-{pet.type_1}", "{pet.type_1}" }
                if let Some(t2) = &pet.type_2 {
                    if !t2.is_empty() {
                        span { class: "badge badge-type-{t2}", "{t2}" }
                    }
                }
            }
            div { class: "shop-tile-foot",
                div { class: "item-price",
                    span { class: "price-icon-emoji", "💰" }
                    span { class: "price-value", "{pet.price}" }
                }
                button {
                    class: if pet.can_buy { "buy-btn" } else { "buy-btn disabled" },
                    disabled: !pet.can_buy || *buying.read(),
                    onclick: move |_| {
                        if !pet.can_buy || *buying.read() || use_battle_state() { return; }
                        buying.set(true);
                        let name_clone = pet.name.clone();
                        let api = NewApiClient::new();
                        spawn(async move {
                            match api.buy_pet(pet.id).await {
                                Ok(resp) => {
                                    let location = match resp.site {
                                        1 => "首位",
                                        2 => "背包",
                                        _ => "仓库",
                                    };
                                    show_success(
                                        format!(
                                            "成功购买 {}！已放入{}，余额: {}",
                                            name_clone,
                                            location,
                                            resp.remaining_money,
                                        ),
                                    );
                                    refresh_pokemon_list();
                                    refresh_user_profile_state();
                                }
                                Err(e) => {
                                    show_error(format!("购买失败: {}", e));
                                }
                            }
                            buying.set(false);
                        });
                    },
                    "购买"
                }
            }
            if *show_tooltip.read() {
                ShopPetTooltip { pet: pet.clone() }
            }
        }
    }
}

/// 宠物详情 Tooltip：六维种族值 + 属性 + 价格。
#[component]
fn ShopPetTooltip(pet: _utils::types::api_shop::ShopPet) -> Element {
    rsx! {
        div { class: "shop-tile-tooltip",
            div { class: "tile-tooltip-header",
                span { class: "tile-tooltip-name", "{pet.name}" }
                span { class: "badge badge-type-{pet.type_1}", "{pet.type_1}" }
                if let Some(t2) = &pet.type_2 {
                    if !t2.is_empty() {
                        span { class: "badge badge-type-{t2}", "{t2}" }
                    }
                }
            }
            div { class: "tile-tooltip-stats pet-stats-grid",
                span { class: "tile-tooltip-stat", "HP {pet.hp}" }
                span { class: "tile-tooltip-stat", "攻击 {pet.atk}" }
                span { class: "tile-tooltip-stat", "防御 {pet.def}" }
                span { class: "tile-tooltip-stat", "特攻 {pet.spatk}" }
                span { class: "tile-tooltip-stat", "特防 {pet.spdef}" }
                span { class: "tile-tooltip-stat", "速度 {pet.speed}" }
            }
            div { class: "tile-tooltip-price",
                span { class: "price-icon-emoji", "💰" }
                span { "{pet.price}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::windowed_pages;

    fn compact(pages: &[Option<usize>]) -> Vec<String> {
        pages
            .iter()
            .map(|p| p.map(|n| n.to_string()).unwrap_or_else(|| "…".to_string()))
            .collect()
    }

    #[test]
    fn few_pages_are_all_listed() {
        assert_eq!(compact(&windowed_pages(1, 1)), ["1"]);
        assert_eq!(
            compact(&windowed_pages(3, 7)),
            ["1", "2", "3", "4", "5", "6", "7"]
        );
    }

    #[test]
    fn long_lists_keep_edges_with_ellipsis() {
        assert_eq!(compact(&windowed_pages(1, 12)), ["1", "2", "…", "12"]);
        assert_eq!(
            compact(&windowed_pages(5, 12)),
            ["1", "…", "4", "5", "6", "…", "12"]
        );
        assert_eq!(compact(&windowed_pages(12, 12)), ["1", "…", "11", "12"]);
    }

    #[test]
    fn zero_total_falls_back_to_single_page() {
        assert_eq!(compact(&windowed_pages(3, 0)), ["1"]);
    }
}
