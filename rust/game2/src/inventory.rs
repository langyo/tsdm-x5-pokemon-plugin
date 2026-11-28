//! 背包页（tairitsu 版，1:1 移植自 rust/game/src/pages/inventory.rs 的
//! 核心交互）。同名同类直用 game.css。
//!
//! 覆盖：搜索/清除、六分类过滤、分页、物品卡片（图标/描述/数量/类型
//! 徽章）、使用流程（global 直用、pokemon 目标弹窗选宠、battle 提示、
//! equip 提示「装备管理随个人中心页移植后接入」）。品质重洗药的
//! 「原位前后对比面板」与个人中心页联动，随该页移植后补齐。

use hikari_components::{Button, ButtonProps};
use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use crate::app_state::AppState;
use crate::icon::{Icon, IconName, IconProps};
use crate::state::ToastKind;
use crate::toast;

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

/// 进入背包页时把页状态重置成旧版挂载时的样子。旧版的分类、页码、搜索词都是
/// 组件内的信号，离开页面就销毁，回来是一张新页：分类是
/// `INITIAL_INVENTORY_CATEGORY.take().or(Some(1))`——侧栏小格留下的那一类，没有就是
/// 「回复药」——页码回到 1，搜索词清空，列表重新拉取（期间是加载态）。这边各页
/// 状态是持久的，所以由 `navigate` 在路由切换时显式调这里。
pub fn on_enter(state: &AppState) {
    let inv = &state.inv;
    let initial = inv.entry_category.replace(None);
    inv.category.set(Some(initial.unwrap_or(1)));
    inv.page.set(1);
    inv.search_input.set(String::new());
    inv.search_query.set(String::new());
    // 清掉上次的列表；下一次渲染时 ensure_loaded 会按新分类重新拉取。
    inv.items.inner().set(None);
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
// 动作入口（本页各元素的原生 onclick / onchange 回调调用）
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
                // 装备道具跳转我的宠物装备页（对齐 Dioxus 版：切
                // MY_POKEMON_TAB=Equipment + CURRENT_PAGE=MyPokemon）。
                state
                    .my_pm
                    .tab
                    .inner()
                    .set(crate::state::MyPokemonTab::Equipment);
                state.route.inner().set(crate::app_state::Route::MyPokemon);
                crate::request_remount();
                crate::my_pokemon::ensure_loaded(state);
                return;
            }
            inv.selected_item.inner().set(Some(item_id));
            inv.show_target_modal.inner().set(true);
            inv.usable_pokemon.inner().set(None);
            // 上一次的品质对比面板与失败提示不带到新的一次使用里（旧版
            // `quality_result.set(None)` + `pokemon_list_error_shown`）。
            inv.quality_result.inner().set(None);
            inv.list_error_shown.inner().set(false);
            crate::request_remount();
            load_usable_pokemon(state, item_id);
        }
        "battle" => {
            toast::show(ToastKind::Warning, "精灵球请在战斗中使用");
            crate::request_remount();
        }
        _ => {
            inv.using.inner().set(Some(item_id));
            crate::request_remount();
            api_use_item(state, item_id, None, None, false);
        }
    }
}

fn load_usable_pokemon(state: &AppState, item_id: u64) {
    let inv = state.inv.clone();
    crate::api::usable_pokemon(
        item_id,
        Box::new(move |result| {
            // 目标列表拉取失败弹一次提示（旧版 use_effect + pokemon_list_error_shown）。
            if let Err(e) = &result {
                if !inv.list_error_shown.get() {
                    inv.list_error_shown.inner().set(true);
                    toast::show(ToastKind::Error, format!("加载宠物列表失败: {e}"));
                }
            }
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
    inv.quality_result.set(None);
    inv.list_error_shown.set(false);
    crate::request_remount();
}

/// 品质重洗的目标：弹窗里那一行的名字、物种、等级、是否异色。
type QualityTarget = (String, u64, u64, bool);

/// 在弹窗中选定宝可梦，执行使用。品质重洗药与怠惰药片成功后保留弹窗（前者
/// 原位换成品质对比面板，后者刷新目标列表）；其余道具选中即关弹窗（旧版如此）。
pub fn confirm_use_on_pokemon(state: &AppState, pokemon_id: u64) {
    let inv = state.inv.clone();
    if inv.using.get().is_some() {
        return;
    }
    let Some(item_id) = inv.selected_item.get() else {
        return;
    };
    let (quality_target, is_ev_reset): (Option<QualityTarget>, bool) =
        match inv.usable_pokemon.get() {
            Some(Ok(list)) if list.item_module == "quality" => (
                list.usable_pokemon
                    .iter()
                    .find(|p| p.id == pokemon_id)
                    .map(|p| (p.nickname.clone(), p.type_id, p.level, p.is_shiny)),
                false,
            ),
            Some(Ok(list)) if list.item_module == "clean_equipment" => (None, true),
            _ => (None, false),
        };
    let keep_modal = quality_target.is_some() || is_ev_reset;
    inv.using.inner().set(Some(item_id));
    if !keep_modal {
        inv.show_target_modal.inner().set(false);
    }
    crate::request_remount();
    api_use_item(
        state,
        item_id,
        Some(pokemon_id),
        quality_target,
        is_ev_reset,
    );
}

/// use_item 的回调分流（旧版 confirm_use_on_pokemon 的 spawn 体）。
fn api_use_item(
    state: &AppState,
    item_id: u64,
    pokemon_id: Option<u64>,
    quality_target: Option<QualityTarget>,
    is_ev_reset: bool,
) {
    let inv = state.inv.clone();
    // 回调须 'static：捕获 AppState 的克隆（而非借用）。
    let state = state.clone();
    crate::api::use_item(
        item_id,
        pokemon_id,
        Box::new(move |result| {
            match result {
                Ok(out) => {
                    // 载荷带回前后品质 → 弹窗原位换成对比面板。
                    let reroll = match (quality_target, out.pokemon_updated.as_ref()) {
                        (Some((name, type_id, level, is_shiny)), Some(update))
                            if update.old_quality_grade.is_some()
                                || update.new_quality_grade.is_some() =>
                        {
                            Some(crate::state::QualityRerollResult {
                                pokemon_id: pokemon_id.unwrap_or(0),
                                name,
                                type_id,
                                level,
                                is_shiny,
                                old_grade: update.old_quality_grade.clone().unwrap_or_default(),
                                new_grade: update.new_quality_grade.clone().unwrap_or_default(),
                                old_iv_total: update.old_iv_total.unwrap_or(0),
                                new_iv_total: update.new_iv_total.unwrap_or(0),
                                item_remaining: out.item_remaining.unwrap_or(0),
                            })
                        }
                        _ => None,
                    };
                    let remaining = out.item_remaining.unwrap_or(0);
                    let on_pokemon = pokemon_id.is_some();
                    toast::show(ToastKind::Success, out.message);
                    match reroll {
                        Some(reroll) => {
                            // 还有剩余药品时刷新目标列表供「继续使用」；用尽则不再请求
                            // （服务端会因无库存报错）。
                            let can_continue = reroll.item_remaining > 0;
                            inv.quality_result.inner().set(Some(reroll));
                            reload(&inv);
                            crate::refresh::bag_counts(&state);
                            crate::refresh::pokemon_list(&state);
                            if can_continue {
                                load_usable_pokemon(&state, item_id);
                            }
                        }
                        None if is_ev_reset => {
                            // 怠惰药片：保留弹窗并刷新目标列表，努力值已清零的宠物会
                            // 从列表消失；用尽则关闭。
                            reload(&inv);
                            crate::refresh::bag_counts(&state);
                            crate::refresh::pokemon_list(&state);
                            if remaining > 0 {
                                load_usable_pokemon(&state, item_id);
                            } else {
                                close_target_modal(&state);
                            }
                        }
                        None => {
                            if on_pokemon {
                                close_target_modal(&state);
                                crate::refresh::pokemon_list(&state);
                            }
                            reload(&inv);
                            crate::refresh::bag_counts(&state);
                        }
                    }
                }
                Err(e) => {
                    toast::show(ToastKind::Error, format!("使用失败: {e}"));
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
            rsx! { div { class: "shop-empty", role: "alert", "{m}" } }
        }
        Some(Ok(items)) => {
            let total = inv.total.get();
            let count_text = format!("当前分类共 {total} 项");
            // 旧版：数据到了就先出「当前分类共 N 项」这一行（空分类也有），再按有没有
            // 物品给空态或网格。
            let empty = if items.is_empty() {
                let has_query = !inv.search_query.get().is_empty();
                let msg = if has_query {
                    "没有找到匹配的物品，试试其他名称或分类"
                } else {
                    "此分类没有物品"
                };
                let src = format!("{IMG_PATH}/item/box.gif");
                Some(rsx! {
                    div { class: "shop-empty",
                        img { class: "empty-icon-img", src: src, alt: "" }
                        p { "{msg}" }
                    }
                })
            } else {
                None
            };
            let total_pages = inv.total_pages.get();
            let page = inv.page.get();
            let page_info = format!("第 {page} / {total_pages} 页");
            let prev_disabled = page <= 1;
            let next_disabled = page >= total_pages as u32;
            // 翻页按钮在点击时才读当前页码（与委托版一致），不取渲染时的 page。
            let prev_state = state.clone();
            let next_state = state.clone();
            let mut nodes: Vec<VNode> = Vec::with_capacity(3);
            nodes.push(rsx! {
                p { id: "inventory-result-count", class: "inventory-result-count", "{count_text}" }
            });
            if let Some(empty) = empty {
                nodes.push(empty);
            } else {
                nodes.push(rsx! {
                    div { class: "shop-items-scroll",
                        div { class: "shop-items-grid",
                            for idx in 0..items.len() {
                                { rsx! { InventoryTile { state: Some(state.clone()), item: Some(items[idx].clone()) } } }
                            }
                        }
                    }
                });
            }
            if total_pages > 1 {
                nodes.push(rsx! {
                    div { class: "shop-pagination",
                        Button {
                            class: ("page-btn").to_string(),
                            disabled: prev_disabled,
                            onclick: crate::button::click(move || {
                                let page = prev_state.inv.page.get().saturating_sub(1).max(1);
                                goto_page(&prev_state, page);
                            }),
                            {Icon(IconProps {
                                name: Some(IconName::ChevronLeft),
                                size: Some(12),
                                class: Some("lucide-inline".to_string()),
                            })}
                            "上一页"
                                                }
                        span { class: "page-info", "{page_info}" }
                        Button {
                            class: ("page-btn").to_string(),
                            disabled: next_disabled,
                            onclick: crate::button::click(move || goto_page(&next_state, next_state.inv.page.get() + 1)),
                            "下一页"
                            {Icon(IconProps {
                                name: Some(IconName::ChevronRight),
                                size: Some(12),
                                class: Some("lucide-inline-end".to_string()),
                            })}
                                                }
                    }
                });
            }
            VNode::Fragment(nodes)
        }
    };

    let modal = if inv.show_target_modal.get() {
        // 品质重洗成功：弹窗原位换成前后对比面板（旧版 quality_result）。
        let quality_panel = inv
            .quality_result
            .get()
            .map(|result| quality_compare_panel(&state, &result));
        let pokemon_body = match quality_panel {
            Some(panel) => panel,
            None => match inv.usable_pokemon.get() {
                None => plain("正在加载可用宠物列表...", "loading-pokemon-list"),
                Some(Err(_)) => plain("加载宠物列表失败，请关闭后重试", "error-pokemon-list"),
                Some(Ok(list)) if list.usable_pokemon.is_empty() => {
                    let carried_only = is_carried_only_module(&list.item_module);
                    let text = if carried_only {
                        "没有携带中的宠物可以使用该物品"
                    } else {
                        "没有可以使用该物品的宝可梦"
                    };
                    plain(text, "empty-pokemon-list")
                }
                Some(Ok(list)) => {
                    // 品质重洗药（quality）与怠惰药片（clean_equipment）只作用于
                    // 携带中的宠物：服务端只返回携带中的目标，选择器展示精灵图
                    // 与品质/努力值摘要（对齐 Dioxus 版的目标选择器）。
                    let is_quality = list.item_module == "quality";
                    let is_ev_reset = list.item_module == "clean_equipment";
                    let carried_only = is_carried_only_module(&list.item_module);
                    let item_name = list.item_name.clone();
                    let unusable_count = list.unusable_count;
                    let rows: Vec<VNode> = list
                    .usable_pokemon
                    .iter()
                    .map(|p| {
                        let id = p.id;
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
                        let show_quality = is_quality;
                        let show_sprite = carried_only;
                        let sprite_src =
                            crate::state::front_sprite_url(p.type_id, p.is_shiny);
                        let sprite_alt = name.clone();
                        let quality_block = if show_quality {
                            match &p.quality_grade {
                                Some(grade) if !grade.is_empty() => {
                                    let rank = p.quality_rank.unwrap_or(0);
                                    let badge_cls =
                                        format!("quality-badge quality-rank-{rank}");
                                    let grade_text = grade.clone();
                                    rsx! { span { class: badge_cls, "{grade_text}" } }
                                }
                                _ => VNode::empty(),
                            }
                        } else {
                            VNode::empty()
                        };
                        let iv_block = if show_quality {
                            match p.iv_total {
                                Some(total) => {
                                    let t = format!("总个体值 {total} / 186");
                                    rsx! { div { class: "pokemon-select-stat-total", "{t}" } }
                                }
                                None => VNode::empty(),
                            }
                        } else {
                            VNode::empty()
                        };
                        let ev_block = if is_ev_reset {
                            match p.ev_total {
                                Some(total) => {
                                    let t = format!("努力值总和 {total} / 510");
                                    rsx! { div { class: "pokemon-select-stat-total", "{t}" } }
                                }
                                None => VNode::empty(),
                            }
                        } else {
                            VNode::empty()
                        };
                        let row_cls = if show_quality {
                            "pokemon-select-item is-detailed is-quality"
                        } else if show_sprite {
                            "pokemon-select-item is-detailed is-ev-reset"
                        } else {
                            "pokemon-select-item"
                        };
                        let shiny_block = if p.is_shiny && show_sprite {
                            rsx! { span { class: "pet-mark pet-mark-shiny pet-mark-compact", title: "异色（闪光）宠物", "闪" } }
                        } else {
                            VNode::empty()
                        };
                        // 回调须 'static：每行各持一份状态克隆。
                        let pick_state = state.clone();
                        let pick_key = id.to_string();
                        rsx! {
                            div {
                                class: row_cls,
                                "data-test-inv-pick": pick_key,
                                onclick: move |_| confirm_use_on_pokemon(&pick_state, id),
                                if show_sprite {
                                    div { class: "pokemon-select-sprite",
                                        img { class: "pokemon-select-sprite-img", src: sprite_src, alt: sprite_alt }
                                        ..vec![shiny_block]
                                    }
                                }
                                div { class: "pokemon-select-body",
                                    div { class: "pokemon-select-info",
                                        span { class: "pokemon-select-name", "{name}" }
                                        span { class: "pokemon-select-level", "{level}" }
                                        ..vec![quality_block]
                                    }
                                    ..vec![iv_block, ev_block]
                                    div { class: "hp-bar-container",
                                        div { class: "hp-bar", style: hp_style }
                                        span { class: "hp-text", "{hp_text}" }
                                    }
                                }
                            }
                        }
                    })
                    .collect();
                    let unusable_block = if !carried_only && unusable_count > 0 {
                        let t = format!("剩余 {unusable_count} 个宠物无法使用该物品");
                        rsx! { div { class: "unusable-pokemon-hint", "{t}" } }
                    } else {
                        VNode::empty()
                    };
                    let picker_hint = if carried_only {
                        let t = format!("{item_name}只显示携带中的宠物（最多 6 只）");
                        rsx! { div { class: "quality-picker-hint", "{t}" } }
                    } else {
                        VNode::empty()
                    };
                    rsx! {
                        div { class: "pokemon-select-modal",
                            ..vec![picker_hint]
                            div { class: "pokemon-select-list custom-scrollbar",
                                for i in 0..rows.len() {
                                    ..vec![rows[i].clone()]
                                }
                            }
                            ..vec![unusable_block]
                        }
                    }
                }
            },
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
        // 外壳是 hikari 的 Modal（见 crate::modal）：点遮罩与关闭键都走
        // on_close，点弹窗本体不关（hikari 只把遮罩上的点击当关闭请求）。
        // 关闭键带 `data-test-inv-modal-close`：回归脚本按它找关闭键。
        let close_for_button = state.clone();
        let close_state = state.clone();
        crate::modal::modal_hooked(
            crate::modal::ModalSpec::new(&title, false),
            crate::modal::hook_close_button("data-test-inv-modal-close", move || {
                close_target_modal(&close_for_button)
            }),
            move || close_target_modal(&close_state),
            vec![pokemon_body],
        )
    } else {
        tairitsu_vdom::VNode::empty()
    };

    // 回调须 'static：各按钮各持一份状态克隆。
    let search_state = state.clone();
    let submit_state = state.clone();
    let clear_state = state.clone();

    rsx! {
        div { class: "page-inventory",
            div { class: "inventory-card",
                // 战斗中整页盖一层（旧版 battle-disabled-overlay）：战斗里不能用背包。
                if crate::battle::user_in_battle(&state) {
                    ..vec![crate::battle::battle_disabled_overlay("战斗中无法使用背包")]
                }
                div { class: "inventory-card-body",
                    div { class: "inventory-search-bar",
                        // 回车/失焦提交（change）：先记下输入值，再按它搜索。
                        input {
                            class: "inventory-search-input",
                            r#type: "search",
                            placeholder: "按物品名称搜索",
                            value: search_input,
                            "data-test-inv-search": "1",
                            onchange: move |e: tairitsu_vdom::ChangeEvent| {
                                let value = e.value;
                                search_state.inv.search_input.set(value.clone());
                                apply_search(&search_state, &value);
                            }
                        }
                        Button {
                            class: ("inventory-search-btn").to_string(),
                            onclick: crate::button::click(move || apply_search(&submit_state, &submit_state.inv.search_input.get())),
                            "搜索"
                                                }
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: ("inventory-search-clear").to_string(),
                            onclick: crate::button::click(move || {
                                clear_state.inv.search_input.set(String::new());
                                apply_search(&clear_state, "");
                            }),
                            "清除"
                                                } }, [("data-test-inv-search-clear", ("1").to_string())]) }
                    }
                    // 旧版背包页的分类栏里按钮直接挂在 `.shop-category-bar` 下（商店页
                    // 才有一层 `.shop-category-group`）。
                    div { class: "shop-category-bar",
                            for i in 0..cats.len() {
                                {
                                    let (cat_id, cat_name, cat_icon) = cats[i];
                                    let active = category == cat_id;
                                    let cls = if active { "category-btn active" } else { "category-btn" };
                                    let img_src = format!("{IMG_PATH}/{cat_icon}");
                                    let cat_state = state.clone();
                                    // 测试钩子：回归脚本按它定位分类按钮（all / 分类号）。
                                    let cat_key = cat_id.map(|n| n.to_string()).unwrap_or_else(|| "all".to_string());
                                    rsx! {
                                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                                            class: (cls).to_string(),
                                            onclick: crate::button::click(move || select_category(&cat_state, cat_id)),
                                            span { class: "cat-icon",
                                                img { class: "cat-icon-img", src: img_src, alt: cat_name }
                                            }
                                            span { "{cat_name}" }
                                                                                } }, [("data-test-inv-cat", (cat_key).to_string())]) }
                                    }
                                }
                            }
                    }
                    ..vec![body]
                }
            }
            ..vec![modal]
        }
    }
}

fn plain(text: &str, class: &str) -> VNode {
    let t = text.to_string();
    rsx! { div { class: "pokemon-select-modal", div { class: class, "{t}" } } }
}

/// 品质重洗药（quality）与怠惰药片（clean_equipment）只作用于携带中的宠物，
/// 服务端只返回携带中的目标（对齐 Dioxus 版 is_carried_only）。
fn is_carried_only_module(module: &str) -> bool {
    matches!(module, "quality" | "clean_equipment")
}

/// 品质重洗成功后的前后对比面板（旧版 quality-compare-panel）：精灵图、品质徽章
/// 前后对比、个体值总和、剩余数量，以及「继续使用」/「关闭」。
fn quality_compare_panel(state: &AppState, result: &crate::state::QualityRerollResult) -> VNode {
    let continue_state = state.clone();
    let close_state = state.clone();
    let sprite = crate::state::front_sprite_url(result.type_id, result.is_shiny);
    let pet_attr = result.pokemon_id.to_string();
    let name = result.name.clone();
    let alt = name.clone();
    let level = format!("Lv.{}", result.level);
    let shiny = result.is_shiny;
    let old_badge = format!(
        "quality-badge quality-rank-{}",
        crate::quality::quality_rank(result.old_iv_total)
    );
    let new_badge = format!(
        "quality-badge quality-rank-{}",
        crate::quality::quality_rank(result.new_iv_total)
    );
    let old_grade = result.old_grade.clone();
    let new_grade = result.new_grade.clone();
    let ivs = format!("总个体值 {} → {}", result.old_iv_total, result.new_iv_total);
    let remaining = format!("品质重洗药剩余 {} 个", result.item_remaining);
    let can_continue = result.item_remaining > 0;
    rsx! {
        div { class: "pokemon-select-modal",
            div { class: "quality-compare-panel", "data-pokemon-id": pet_attr,
                div { class: "quality-compare-pet",
                    div { class: "quality-compare-sprite",
                        img { class: "quality-compare-sprite-img", src: sprite, alt: alt }
                        if shiny {
                            span { class: "pet-mark pet-mark-shiny pet-mark-compact", title: "异色（闪光）宠物", "闪" }
                        }
                    }
                    div { class: "quality-compare-pet-info",
                        span { class: "quality-compare-pet-name", "{name}" }
                        span { class: "quality-compare-pet-level", "{level}" }
                    }
                }
                div { class: "quality-compare-change",
                    span { class: old_badge, "{old_grade}" }
                    span { class: "quality-compare-arrow", "→" }
                    span { class: new_badge, "{new_grade}" }
                }
                div { class: "quality-compare-ivs", "{ivs}" }
                div { class: "quality-compare-remaining", "{remaining}" }
                div { class: "quality-compare-actions",
                    if can_continue {
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: ("quality-compare-btn primary").to_string(),
                            onclick: crate::button::click(move || {
                                continue_quality_reroll(&continue_state)
                            }),
                            "继续使用"
                                                    } }, [("data-test-inv-reroll-continue", ("1").to_string())]) }
                    }
                    { crate::button::hooked(tairitsu_macros::rsx! { Button {
                        class: ("quality-compare-btn").to_string(),
                        onclick: crate::button::click(move || close_target_modal(&close_state)),
                        "关闭"
                                                } }, [("data-test-inv-reroll-close", ("1").to_string())]) }
                }
            }
        }
    }
}

/// 对比面板的「继续使用」：收起面板，回到（已刷新的）目标列表。
pub fn continue_quality_reroll(state: &AppState) {
    state.inv.quality_result.inner().set(None);
    crate::request_remount();
}

/// 单个物品卡片。use_item 需要整个 AppState（装备道具会切到个人中心页），
/// 所以这里收 state，而不是只收背包子状态。
#[component]
fn InventoryTile(state: Box<AppState>, item: crate::state::InventoryItem) -> VNode {
    let using = state.inv.using.get();
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
    // 能用：显示按钮、目标是宠物或全体（旧版的 `ItemTargetType::Pokemon | Global`）。
    let usable = show_button && matches!(target.as_str(), "pokemon" | "global");
    // 再加上「当前没有别的物品在用」才是可点。
    let enabled = usable && !is_using && !any_using;
    let button_text = if is_using { "使用中..." } else { "使用" };
    let button_class = if enabled {
        "buy-btn"
    } else {
        "buy-btn disabled"
    }
    .to_string();
    // 使用键是物品类型 ID（type_id）——use_item/get_usable_pokemon 都以它为键
    // （对齐 Dioxus 版），背包行 id 不外发。
    let item_id = item.type_id;
    let item_type = item.item_type;
    let use_target = item.use_target.clone();
    // 测试钩子：回归脚本按物品类型 ID 定位使用按钮。
    let use_key = item_id.to_string();
    let alt_name = item.name.clone();

    rsx! {
        div { class: "shop-item-card",
            "data-item-id": use_key.clone(),
            div { class: "item-icon-area",
                // alt 是物品名：图片加载失败时浏览器用它画占位框（旧版如此）。
                img { class: "item-icon-img", src: img_src, alt: alt_name }
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
                    { crate::button::hooked(tairitsu_macros::rsx! { Button {
                        class: (button_class).to_string(),
                        disabled: !enabled,
                        onclick: crate::button::click(move || use_item(&state, item_id, item_type, use_target.clone())),
                        "{button_text}"
                                        } }, [("data-test-inv-use", (use_key).to_string())]) }
                }
            }
        }
    }
}
