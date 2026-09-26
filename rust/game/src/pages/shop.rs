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
    let scroll_container: Signal<Option<web_sys::Element>> = use_signal(|| None);

    let shop_data = use_shop(current_category, current_page);
    let pet_shop_data = use_shop_pets(current_page);

    let categories = [
        (Some(1u32), "回复药", "item/hp.gif"),
        (Some(2), "精灵球", "item/jlq.gif"),
        (Some(3), "进化石", "item/szs.gif"),
        (Some(4), "强化道具", "item/atkg.gif"),
        (Some(5), "装备道具", "item/zb01.gif"),
        (PET_CATEGORY, "宠物", "spm/25.gif"),
    ];

    let is_battle_active = use_battle_state();

    let scroll_to_top = move |_| {
        if let Some(elem) = scroll_container.read().as_ref() {
            elem.set_scroll_top(0);
        }
    };

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
                        for (cat_id , cat_name , cat_icon) in &categories {
                            {
                                let cid = *cat_id;
                                let active = *current_category.read() == cid;
                                let is_pet = cid == PET_CATEGORY;
                                rsx! {
                                    button {
                                        class: if active { "category-btn active" } else { "category-btn" },
                                        onclick: move |_| {
                                            current_category.set(cid);
                                            current_page.set(1);
                                            scroll_to_top(());
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

                    if *current_category.read() == PET_CATEGORY {
                        ShopPetsTab {
                            current_page,
                            pet_shop_data,
                            scroll_container,
                            on_scroll_to_top: scroll_to_top,
                        }
                    } else {
                        ShopItemsTab {
                            current_category,
                            current_page,
                            shop_data,
                            scroll_container,
                            on_scroll_to_top: scroll_to_top,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ShopItemsTab(
    current_category: Signal<Option<u32>>,
    current_page: Signal<u32>,
    shop_data: Resource<Result<_utils::types::api_shop::ShopListResponse, String>>,
    scroll_container: Signal<Option<web_sys::Element>>,
    on_scroll_to_top: EventHandler<()>,
) -> Element {
    let data_to_display = shop_data
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();

    let do_scroll = move |_| on_scroll_to_top.call(());

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
                    p { "暂无商品" }
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
                                    let item_id = item.id;
                                    let item_name = item.name.clone();
                                    let item_desc = item.description.clone();
                                    let item_price = item.price;
                                    let item_type_name = item.type_name.clone();
                                    let item_image = item.image.clone();
                                    let can_buy = item.can_buy;

                                    rsx! {
                                        div { key: "{idx}", class: "shop-item-card",
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
                                                }
                                            }
                                            div { class: "item-action",
                                                div { class: "item-price",
                                                    span { class: "price-icon-emoji", "💰" }
                                                    span { class: "price-value", "{item_price}" }
                                                }
                                                button {
                                                    class: if can_buy { "buy-btn" } else { "buy-btn disabled" },
                                                    disabled: !can_buy,
                                                    onclick: move |_| {
                                                        let name_clone = item_name.clone();
                                                        let api = NewApiClient::new();
                                                        let do_refresh = || {
                                                            refresh_user_profile_state();
                                                            refresh_inventory_state();
                                                        };
                                                        spawn(async move {
                                                            match api.buy_item(item_id, 1).await {
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
                                                        });
                                                    },
                                                    "购买"
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
                                do_scroll(());
                            },
                            "← 上一页"
                        }
                        span { class: "page-info",
                            "第 {data.page} / {data.total_pages} 页"
                        }
                        button {
                            class: "page-btn",
                            disabled: *current_page.read() as usize >= data.total_pages,
                            onclick: move |_| {
                                let p = *current_page.read() + 1;
                                current_page.set(p);
                                do_scroll(());
                            },
                            "下一页 →"
                        }
                    }
                }
            }
        } else if let Some(err) = shop_data.read().as_ref().and_then(|r| r.as_ref().err()) {
            div { class: "error-message", "加载失败: {err}" }
        }
    }
}

#[component]
fn ShopPetsTab(
    current_page: Signal<u32>,
    pet_shop_data: Resource<Result<_utils::types::api_shop::ShopPetListResponse, String>>,
    scroll_container: Signal<Option<web_sys::Element>>,
    on_scroll_to_top: EventHandler<()>,
) -> Element {
    let data_to_display = pet_shop_data
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();

    let do_scroll = move |_| on_scroll_to_top.call(());

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
                    p { "暂无可购买的宠物" }
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
                            data.pets
                                .into_iter()
                                .enumerate()
                                .map(|(idx, pet)| {
                                    let pet_id = pet.id;
                                    let pet_name = pet.name.clone();
                                    let pet_type1 = pet.type_1.clone();
                                    let pet_type2 = pet.type_2.clone();
                                    let pet_price = pet.price;
                                    let can_buy = pet.can_buy;
                                    let stats_text = format!(
                                        "HP:{} ATK:{} DEF:{} SPA:{} SPD:{} SPE:{}",
                                        pet.hp, pet.atk, pet.def, pet.spatk, pet.spdef, pet.speed
                                    );

                                    rsx! {
                                        div { key: "pet-{idx}", class: "shop-item-card",
                                            div { class: "item-icon-area",
                                                img {
                                                    class: "item-icon-img",
                                                    src: "{IMG_PATH}/spm/{pet_id}.gif",
                                                    alt: "{pet_name}",
                                                }
                                            }
                                            div { class: "item-info",
                                                div { class: "item-name", "{pet_name}" }
                                                div { class: "item-desc", "{stats_text}" }
                                                div { class: "item-meta",
                                                    span { class: "badge badge-type-{pet_type1}", "{pet_type1}" }
                                                    if let Some(t2) = &pet_type2 {
                                                        if !t2.is_empty() {
                                                            span { class: "badge badge-type-{t2}", "{t2}" }
                                                        }
                                                    }
                                                }
                                            }
                                            div { class: "item-action",
                                                div { class: "item-price",
                                                    span { class: "price-icon-emoji", "💰" }
                                                    span { class: "price-value", "{pet_price}" }
                                                }
                                                button {
                                                    class: if can_buy { "buy-btn" } else { "buy-btn disabled" },
                                                    disabled: !can_buy,
                                                    onclick: move |_| {
                                                        let name_clone = pet_name.clone();
                                                        let api = NewApiClient::new();
                                                        spawn(async move {
                                                            match api.buy_pet(pet_id).await {
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
                                                        });
                                                    },
                                                    "购买"
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
                                do_scroll(());
                            },
                            "← 上一页"
                        }
                        span { class: "page-info",
                            "第 {data.page} / {data.total_pages} 页"
                        }
                        button {
                            class: "page-btn",
                            disabled: *current_page.read() as usize >= data.total_pages,
                            onclick: move |_| {
                                let p = *current_page.read() + 1;
                                current_page.set(p);
                                do_scroll(());
                            },
                            "下一页 →"
                        }
                    }
                }
            }
        } else if let Some(err) = pet_shop_data.read().as_ref().and_then(|r| r.as_ref().err()) {
            div { class: "error-message", "加载失败: {err}" }
        }
    }
}
