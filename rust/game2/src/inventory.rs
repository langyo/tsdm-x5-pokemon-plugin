//! 背包页（tairitsu 版，1:1 移植自 rust/game/src/pages/inventory.rs 的
//! 核心交互）。同名同类直用 game.css。
//!
//! 覆盖：搜索/清除、六分类过滤、分页、物品卡片（图标/描述/数量/类型
//! 徽章）、使用流程（global 直用、pokemon 目标弹窗选宠、battle 提示、
//! equip 提示「装备管理随个人中心页移植后接入」）。品质重洗药的
//! 「原位前后对比面板」与个人中心页联动，随该页移植后补齐。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use hikari_components::{Toast, ToastPosition, ToastProps, ToastVariant};

use crate::app_state::AppState;
use crate::state::ToastKind;

const IMG_PATH: &str = "source/plugin/pokemon/images";

/// 物品类型 ID（对齐 Dioxus 版常量）。
const ITEM_TYPE_EQUIP: u64 = 5;

fn categories() -> [(Option<u32>, &'static str, &'static str); 6] {
    [
        (None, "全部物品", "item/box.gif"),
        (Some(1), "回复药", "item/hp.gif"),
        (Some(2), "精灵球", "item/jlq.gif"),
        (Some(3), "进化石", "item/szs.gif"),
        (Some(4), "强化道具", "item/atkg.gif"),
        (Some(5), "装备道具", "item/zb01.gif"),
    ]
}

/// 首次进入装载（对齐 Dioxus 版 use_resource 首次求值）。
pub fn ensure_loaded(state: &AppState) {
    if state.inv.items.get().is_none() {
        reload(&state.inv);
    }
}

fn reload(inv: &crate::state::InventoryState) {
    let inv = inv.clone();
    let category = inv.category.get();
    let page = inv.page.get();
    let search = inv.search_query.get();
    crate::api::inventory(
        category,
        page,
        &search,
        Box::new(move |result| {
            match result {
                Ok(resp) => {
                    inv.items.inner().set(Some(Ok(resp.items)));
                    inv.total.inner().set(resp.total);
                    let last = resp.total_pages.max(1);
                    inv.total_pages.inner().set(last);
                    if (inv.page.get() as usize) > last {
                        inv.page.inner().set(last as u32);
                        reload(&inv);
                        return;
                    }
                }
                Err(e) => inv.items.inner().set(Some(Err(e))),
            }
            crate::request_remount();
        }),
    );
}

// ---------------------------------------------------------------------------
// 委托动作入口（delegation.rs 调用）
// ---------------------------------------------------------------------------

pub fn select_category(state: &AppState, cat: Option<u32>) {
    let inv = state.inv.clone();
    inv.category.set(cat);
    inv.page.set(1);
    reload(&inv);
}

pub fn goto_page(state: &AppState, page: u32) {
    let inv = state.inv.clone();
    let last = inv.total_pages.get().max(1) as u32;
    let page = page.clamp(1, last);
    if page == inv.page.get() {
        return;
    }
    inv.page.set(page);
    reload(&inv);
}

pub fn apply_search(state: &AppState, raw: &str) {
    let inv = state.inv.clone();
    inv.search_query.set(raw.trim().to_string());
    inv.page.set(1);
    reload(&inv);
}

/// 使用物品入口（对齐 Dioxus 版 use_item）。
pub fn use_item(state: &AppState, item_id: u64, item_type: u64, use_target: Option<String>) {
    let inv = state.inv.clone();
    if inv.using.get().is_some() {
        return;
    }
    // 目标类型：服务端 use_target 优先，缺省按物品类型回退。
    let target = match use_target.as_deref() {
        Some("pokemon") => "pokemon",
        Some("battle") => "battle",
        Some("global") => "global",
        _ => match item_type {
            1 | 3 | 4 | 5 => "pokemon",
            2 => "battle",
            _ => "global",
        },
    };
    match target {
        "pokemon" => {
            if item_type == ITEM_TYPE_EQUIP {
                // 装备管理在个人中心页（未移植），先提示。
                crate::state::inv_show_toast(
                    &state.inv,
                    ToastKind::Error,
                    "装备道具请等待个人中心页完成移植后使用".to_string(),
                );
                crate::request_remount();
                return;
            }
            inv.selected_item.inner().set(Some(item_id));
            inv.show_target_modal.inner().set(true);
            inv.usable_pokemon.inner().set(None);
            crate::request_remount();
            load_usable_pokemon(state, item_id);
        }
        "battle" => {
            crate::state::inv_show_toast(
                &state.inv,
                ToastKind::Error,
                "精灵球请在战斗中使用".to_string(),
            );
            crate::request_remount();
        }
        _ => {
            inv.using.inner().set(Some(item_id));
            crate::request_remount();
            api_use_item(state, item_id, None);
        }
    }
}

fn load_usable_pokemon(state: &AppState, item_id: u64) {
    let inv = state.inv.clone();
    crate::api::usable_pokemon(
        item_id,
        Box::new(move |result| {
            inv.usable_pokemon.inner().set(Some(result));
            crate::request_remount();
        }),
    );
}

pub fn close_target_modal(state: &AppState) {
    let inv = state.inv.clone();
    inv.show_target_modal.set(false);
    inv.selected_item.set(None);
    inv.usable_pokemon.set(None);
    crate::request_remount();
}

/// 在弹窗中选定宝可梦，执行使用。
pub fn confirm_use_on_pokemon(state: &AppState, pokemon_id: u64) {
    let inv = state.inv.clone();
    if inv.using.get().is_some() {
        return;
    }
    let Some(item_id) = inv.selected_item.get() else {
        return;
    };
    inv.using.inner().set(Some(item_id));
    crate::request_remount();
    api_use_item(state, item_id, Some(pokemon_id));
}

fn api_use_item(state: &AppState, item_id: u64, pokemon_id: Option<u64>) {
    let inv = state.inv.clone();
    // 回调须 'static：捕获 AppState 的克隆（而非借用）。
    let state = state.clone();
    crate::api::use_item(
        item_id,
        pokemon_id,
        Box::new(move |result| {
            match result {
                Ok(out) => {
                    crate::state::inv_show_toast_silent(&inv, ToastKind::Success, out.message);
                    if pokemon_id.is_none() {
                        inv.show_target_modal.inner().set(false);
                        inv.selected_item.inner().set(None);
                        inv.usable_pokemon.inner().set(None);
                    } else if out.item_remaining.unwrap_or(0) > 0 {
                        // 弹窗保留、刷新目标列表供「继续使用」（对齐 Dioxus 版
                        // 品质/怠惰道具行为；普通道具也统一保留，行为差异随
                        // 品质对比面板补齐时再分）。
                        load_usable_pokemon(&state, item_id);
                    } else {
                        inv.show_target_modal.inner().set(false);
                        inv.selected_item.inner().set(None);
                        inv.usable_pokemon.inner().set(None);
                    }
                    reload(&inv);
                }
                Err(e) => {
                    crate::state::inv_show_toast_silent(
                        &inv,
                        ToastKind::Error,
                        format!("使用失败: {e}"),
                    );
                }
            }
            inv.using.inner().set(None);
            crate::request_remount();
        }),
    );
}

// ---------------------------------------------------------------------------
// 组件
// ---------------------------------------------------------------------------

#[component]
pub fn Inventory(state: Box<AppState>) -> VNode {
    let inv = state.inv.clone();
    let data = inv.items.get();
    let category = inv.category.get();
    let search_input = inv.search_input.get();
    let cats = categories();

    let body = match &data {
        None => rsx! { div { class: "shop-loading", "正在加载背包..." } },
        Some(Err(message)) => {
            let m = message.clone();
            rsx! { div { class: "shop-empty", "{m}" } }
        }
        Some(Ok(items)) if items.is_empty() => {
            let has_query = !inv.search_query.get().is_empty();
            let msg = if has_query {
                "没有找到匹配的物品，试试其他名称或分类"
            } else {
                "此分类没有物品"
            };
            let src = format!("{IMG_PATH}/item/box.gif");
            rsx! {
                div { class: "shop-empty",
                    img { class: "empty-icon-img", src: src }
                    p { "{msg}" }
                }
            }
        }
        Some(Ok(items)) => {
            let total = inv.total.get();
            let count_text = format!("当前分类共 {total} 项");
            let total_pages = inv.total_pages.get();
            let page = inv.page.get();
            let page_info = format!("第 {page} / {total_pages} 页");
            let prev_disabled = page <= 1;
            let next_disabled = page >= total_pages as u32;
            rsx! {
                p { class: "inventory-result-count", "{count_text}" }
                div { class: "shop-items-scroll",
                    div { class: "shop-items-grid",
                        for idx in 0..items.len() {
                            { rsx! { InventoryTile { inv: Some(Box::new(inv.clone())), item: Some(items[idx].clone()) } } }
                        }
                    }
                }
                if total_pages > 1 {
                    div { class: "shop-pagination",
                        button {
                            class: "page-btn",
                            disabled: prev_disabled,
                            "data-pm-inv-page": "prev",
                            "上一页"
                        }
                        span { class: "page-info", "{page_info}" }
                        button {
                            class: "page-btn",
                            disabled: next_disabled,
                            "data-pm-inv-page": "next",
                            "下一页"
                        }
                    }
                }
            }
        }
    };

    let modal = if inv.show_target_modal.get() {
        let pokemon_body = match inv.usable_pokemon.get() {
            None => plain("正在加载可用宠物列表...", "loading-pokemon-list"),
            Some(Err(_)) => plain("加载宠物列表失败，请关闭后重试", "error-pokemon-list"),
            Some(Ok(list)) if list.is_empty() => {
                plain("没有可以使用该物品的宠物", "empty-pokemon-list")
            }
            Some(Ok(list)) => {
                let rows: Vec<VNode> = list
                    .iter()
                    .map(|p| {
                        let pid = p.id.to_string();
                        let name = p.nickname.clone();
                        let level = format!("Lv.{}", p.level);
                        let hp = p.hp;
                        let max_hp = p.max_hp;
                        let hp_text = format!("{hp}/{max_hp}");
                        let hp_percent = if max_hp > 0 {
                            (hp as f64 / max_hp as f64 * 100.0) as u32
                        } else {
                            0
                        };
                        let hp_color = if hp_percent < 30 {
                            "#e74c3c"
                        } else {
                            "#2ecc71"
                        };
                        let hp_style = format!("width: {hp_percent}%; background: {hp_color};");
                        rsx! {
                            div {
                                class: "pokemon-select-item",
                                "data-pm-inv-pick": pid,
                                div { class: "pokemon-select-body",
                                    div { class: "pokemon-select-info",
                                        span { class: "pokemon-select-name", "{name}" }
                                        span { class: "pokemon-select-level", "{level}" }
                                    }
                                    div { class: "hp-bar-container",
                                        div { class: "hp-bar", style: hp_style }
                                        span { class: "hp-text", "{hp_text}" }
                                    }
                                }
                            }
                        }
                    })
                    .collect();
                rsx! {
                    div { class: "pokemon-select-list custom-scrollbar",
                        for i in 0..rows.len() {
                            ..vec![rows[i].clone()]
                        }
                    }
                }
            }
        };
        let item_name = inv
            .selected_item
            .get()
            .map(|id| {
                inv.items
                    .get()
                    .and_then(|r| r.ok())
                    .and_then(|items| {
                        items
                            .iter()
                            .find(|i| i.type_id == id)
                            .map(|i| i.name.clone())
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let title = format!("选择宝可梦 - {item_name}");
        rsx! {
            div { class: "modal-overlay", "data-pm-inv-modal-close": "1",
                div { class: "modal-content",
                    div { class: "modal-header",
                        h3 { "{title}" }
                        button { class: "modal-close", "data-pm-inv-modal-close": "1", "×" }
                    }
                    div { class: "modal-body", ..vec![pokemon_body] }
                }
            }
        }
    } else {
        tairitsu_vdom::VNode::empty()
    };

    let toast_state = inv.clone();
    rsx! {
        div { class: "page-inventory",
            div { class: "inventory-card",
                div { class: "inventory-card-body",
                    div { class: "inventory-search-bar",
                        input {
                            class: "inventory-search-input",
                            r#type: "search",
                            placeholder: "按物品名称搜索",
                            value: search_input,
                            "data-pm-inv-search": "1"
                        }
                        button { class: "inventory-search-btn", "data-pm-inv-search-submit": "1", "搜索" }
                        button { class: "inventory-search-clear", "data-pm-inv-search-clear": "1", "清除" }
                    }
                    div { class: "shop-category-bar",
                        div { class: "shop-category-group",
                            for i in 0..cats.len() {
                                {
                                    let (cat_id, cat_name, cat_icon) = cats[i];
                                    let cat_attr = cat_id.map(|n| n.to_string()).unwrap_or_else(|| "all".to_string());
                                    let active = category == cat_id;
                                    let cls = if active { "category-btn active" } else { "category-btn" };
                                    let img_src = format!("{IMG_PATH}/{cat_icon}");
                                    rsx! {
                                        button {
                                            class: cls,
                                            "data-pm-inv-cat": cat_attr,
                                            span { class: "cat-icon",
                                                img { class: "cat-icon-img", src: img_src }
                                            }
                                            span { "{cat_name}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ..vec![body]
                }
            }
            ..vec![modal]
            if inv.toast_visible.get() {
                Toast {
                    variant: inv_toast_variant(inv.toast_kind.get()),
                    message: inv.toast_message.get(),
                    position: ToastPosition::BottomRight,
                    on_close: Some(tairitsu_vdom::Callback::new(move |_| {
                        toast_state.toast_visible.set(false)
                    })),
                }
            }
        }
    }
}

fn inv_toast_variant(kind: ToastKind) -> ToastVariant {
    match kind {
        ToastKind::Success => ToastVariant::Success,
        ToastKind::Error => ToastVariant::Error,
    }
}

fn plain(text: &str, class: &str) -> VNode {
    let t = text.to_string();
    rsx! { div { class: "pokemon-select-modal", div { class: class, "{t}" } } }
}

#[component]
fn InventoryTile(
    inv: Box<crate::state::InventoryState>,
    item: crate::state::InventoryItem,
) -> VNode {
    let using = inv.using.get();
    let img_src = format!("{IMG_PATH}/item/{}.gif", item.image);
    let qty = format!("×{}", item.quantity);
    let is_using = using == Some(item.type_id);
    let any_using = using.is_some();
    // 精灵球（battle）不显示使用按钮（对齐 Dioxus 版）。
    let target = match item.use_target.as_deref() {
        Some(t) => t.to_string(),
        None => match item.item_type {
            1 | 3 | 4 | 5 => "pokemon".to_string(),
            2 => "battle".to_string(),
            _ => "global".to_string(),
        },
    };
    let show_button = target != "battle";
    let usable = !is_using && !any_using;
    let button_text = if is_using { "使用中..." } else { "使用" };
    let button_class = if usable {
        "buy-btn"
    } else {
        "buy-btn disabled"
    }
    .to_string();
    let use_attr = item.type_id.to_string();

    rsx! {
        div { class: "shop-item-card",
            div { class: "item-icon-area",
                img { class: "item-icon-img", src: img_src }
            }
            div { class: "item-info",
                div { class: "item-name", "{item.name}" }
                div { class: "item-desc", "{item.description}" }
                div { class: "item-meta",
                    span { class: "item-type-badge", "{item.type_name}" }
                    span { class: "item-type-badge", "{qty}" }
                }
            }
            div { class: "item-action",
                if show_button {
                    button {
                        class: button_class,
                        disabled: !usable,
                        "data-pm-inv-use": use_attr,
                        "{button_text}"
                    }
                }
            }
        }
    }
}
