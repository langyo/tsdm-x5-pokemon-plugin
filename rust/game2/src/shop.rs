//! 商店页（tairitsu 版，1:1 移植自 rust/game/src/pages/shop.rs）。
//!
//! 外观与 Dioxus 版完全同名同类（.page-shop / .shop-tile / .buy-btn …），
//! 直接复用 plugin/wasm/game.css；行为对齐点：
//! - 分类切换/搜索重置回第 1 页，服务端返回总页数后把当前页夹回
//! - 悬浮详情显示在光标上方（视口内横向收拢），不遮挡价格与购买按钮
//! - 购买互斥（buying 守卫），成功/失败反馈用 hikari Toast
//! - 分页器窗口化页码（shop_logic::windowed_pages，同算法带测试）
//!
//! 战斗中禁用商店的遮罩随对战页移植一起接（当前 v2 无对战状态）。
//!
//! 本模块只在 wasm 下编译（依赖 fetch 回调桥）。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::MouseEvent;
use tairitsu_vdom::VNode;

use hikari_components::{
    theme::ThemeProviderProps, ThemeProvider, Toast, ToastPosition, ToastProps, ToastVariant,
};
use tairitsu_hooks::ReactiveSignal;

use crate::api;
use crate::icon::{Icon, IconName, IconProps};
use crate::shop_logic::{shop_tooltip_position, viewport_width, PagerEntry, windowed_entries};
use crate::state::{show_toast_silent, ShopItem, ShopPet, ShopState, ToastKind};

/// 图片资源基础路径（对应 Dioxus 版 IMG_PATH）。
const IMG_PATH: &str = "source/plugin/pokemon/images";
/// 宠物分类哨兵：category = None 表示宠物页（与 Dioxus 版一致）。
const PET_CATEGORY: Option<u32> = None;

/// tooltip 估算高度（px），需与 .shop-tile-tooltip 的 CSS 尺寸一致。
const ITEM_TOOLTIP_H: f64 = 130.0;
const PET_TOOLTIP_H: f64 = 190.0;

/// 名称筛选防抖间隔（毫秒），与 Dioxus 版 SEARCH_DEBOUNCE_MS 一致。
const SEARCH_DEBOUNCE_MS: i32 = 250;

fn categories() -> [(Option<u32>, &'static str, &'static str); 6] {
    [
        (Some(1), "回复药", "item/hp.gif"),
        (Some(2), "精灵球", "item/jlq.gif"),
        (Some(3), "进化石", "item/szs.gif"),
        (Some(4), "强化道具", "item/atkg.gif"),
        (Some(5), "装备道具", "item/zb01.gif"),
        (PET_CATEGORY, "宠物", "spm/25.gif"),
    ]
}

fn tooltip_anchor(evt: &MouseEvent, tooltip_h: f64) -> (f64, f64) {
    let win_w = viewport_width().unwrap_or(1280.0);
    shop_tooltip_position(evt.client_x as f64, evt.client_y as f64, tooltip_h, win_w)
}

// ---------------------------------------------------------------------------
// 数据装载
// ---------------------------------------------------------------------------

/// 按当前状态发起列表请求。回调在 promise 线程投递，只 set 信号；
/// 服务端页数收缩时把当前页夹回并重载（对齐 Dioxus 版 use_effect）。
///
/// promise 线程的调度绕行：该线程发起的异步导入（rAF 预订）会被 jco
/// 排队且在锁损坏时永不执行（UI 冻结无报错），而同步 DOM 调用可用
/// （diag 实测）——因此回调里 set 完直接同步 flush，不等帧。
pub fn reload(state: &ShopState) {
    let keyword = state.keyword.get().trim().to_string();
    let page = state.page.get();
    let category = state.category.get();
    if category == PET_CATEGORY {
        let state = state.clone();
        api::shop_pets(
            page,
            &keyword,
            Box::new(move |result| {
                match result {
                    Ok(resp) => {
                        let n = resp.pets.len();
                        state.pets.inner().set(Some(Ok(resp.pets)));
                        clamp_page_silent(&state, resp.total_pages);
                    }
                    Err(e) => {
                        state.pets.inner().set(Some(Err(e)));
                    }
                }
                promise_commit(&state);
            }),
        );
    } else {
        let state = state.clone();
        api::shop_items(
            category,
            page,
            &keyword,
            Box::new(move |result| {
                match result {
                    Ok(resp) => {
                        let n = resp.items.len();
                        state.items.inner().set(Some(Ok(resp.items)));
                        clamp_page_silent(&state, resp.total_pages);
                    }
                    Err(e) => {
                        state.items.inner().set(Some(Err(e)));
                    }
                }
                promise_commit(&state);
            }),
        );
    }
}

/// 诊断写入：#app[data-<attr>]（wasm 任何隐式线程都可靠；stderr 跨线程会抛）。
#[cfg(target_family = "wasm")]
fn diag(attr: &str, msg: &str) {
    use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
    use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::non_element_parent_node::get_element_by_id;
    if let Some(app) = get_element_by_id(0, "app") {
        let _ = set_attribute(app, attr, msg);
    }
}

#[cfg(not(target_family = "wasm"))]
fn diag(_attr: &str, _msg: &str) {}

#[cfg(target_family = "wasm")]
fn promise_commit(state: &ShopState) {
    use std::cell::Cell;
    thread_local! {
        // 递增计数：MutationObserver 靠值变化判定，恒写同一个值会让
        // 第二次及之后的数据刷新永远不派发（分页/切类全哑）。
        static TICK: Cell<u64> = const { Cell::new(0) };
    }
    let cid = state.component_id.get() as usize;
    tairitsu_vdom::runtime::mark_dirty_deferred(cid);
    use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
    TICK.with(|t| {
        let n = t.get() + 1;
        t.set(n);
        if let Some(app) = crate::element_handle("app") {
            let _ = set_attribute(app, "data-flush-pending", &n.to_string());
        }
    });
}

#[cfg(not(target_family = "wasm"))]
fn promise_commit(_state: &ShopState) {}


// ---------------------------------------------------------------------------
// 委托动作入口（delegation.rs 调用；全部走事件线程，可安全 mark_dirty）
// ---------------------------------------------------------------------------

pub fn select_category(state: &ShopState, cat: Option<u32>) {
    if state.buying.get() { return; }
    state.category.set(cat);
    state.page.set(1);
    reload(state);
}

pub fn goto_page(state: &ShopState, page: u32) {
    let last = state.total_pages.get().max(1) as u32;
    let page = page.clamp(1, last);
    if page == state.page.get() { return; }
    state.page.set(page);
    reload(state);
}

fn current_list(state: &ShopState) -> Option<(bool, Vec<crate::state::ShopItem>, Vec<crate::state::ShopPet>)> {
    let is_pet = state.category.get() == PET_CATEGORY;
    if is_pet {
        match state.pets.get() {
            Some(Ok(pets)) => Some((true, Vec::new(), pets)),
            _ => None,
        }
    } else {
        match state.items.get() {
            Some(Ok(items)) => Some((false, items, Vec::new())),
            _ => None,
        }
    }
}

pub fn buy_item_by_id(state: &ShopState, id: u64) {
    let Some((false, items, _)) = current_list(state) else { return };
    let Some(item) = items.iter().find(|i| i.id == id) else { return };
    buy_item(state, item);
}

pub fn buy_pet_by_id(state: &ShopState, id: u64) {
    let Some((true, _, pets)) = current_list(state) else { return };
    let Some(pet) = pets.iter().find(|p| p.id == id) else { return };
    buy_pet(state, pet);
}

/// 筛选提交（回车）/清除：更新关键字并重载。
pub fn reload_from_filter(state: &ShopState, value: &str) {
    state.keyword.set(value.trim().to_string());
    state.page.set(1);
    reload(state);
}

/// 纯重挂载触发（无状态变化的 UI 切换，如筛选弹层开合）。
pub fn refresh(state: &ShopState) {
    let _ = state;
    crate::request_remount();
}

pub fn viewport_width_or_default() -> f64 {
    crate::shop_logic::viewport_width().unwrap_or(1280.0)
}

fn clamp_page(state: &ShopState, total_pages: usize) {
    let last = total_pages.max(1);
    state.total_pages.set(last);
    if (state.page.get() as usize) > last {
        state.page.set(last as u32);
        reload(state);
    }
}

/// promise 线程版：全部走内部值写入，不触发任何调度。
fn clamp_page_silent(state: &ShopState, total_pages: usize) {
    let last = total_pages.max(1);
    state.total_pages.inner().set(last);
    if (state.page.get() as usize) > last {
        state.page.inner().set(last as u32);
    }
}

/// 购买互斥守卫（对齐 Dioxus 版 buying）。
fn buy_guarded(state: &ShopState) -> bool {
    if state.buying.get() {
        return false;
    }
    state.buying.set(true);
    true
}

fn buy_item(state: &ShopState, item: &ShopItem) {
    if !item.can_buy || !buy_guarded(state) {
        return;
    }
    let name = item.name.clone();
    let state = state.clone();
    api::buy_item(
        item.id,
        Box::new(move |result| {
            match result {
                Ok(resp) => show_toast_silent(
                    &state,
                    ToastKind::Success,
                    format!("成功购买 {name} ×1！余额: {}", resp.remaining_money),
                ),
                Err(e) => show_toast_silent(&state, ToastKind::Error, format!("购买失败: {e}")),
            }
            state.buying.inner().set(false);
            reload(&state);
        }),
    );
}

fn buy_pet(state: &ShopState, pet: &ShopPet) {
    if !pet.can_buy || !buy_guarded(state) {
        return;
    }
    let name = pet.name.clone();
    let state = state.clone();
    api::buy_pet(
        pet.id,
        Box::new(move |result| {
            match result {
                Ok(resp) => {
                    let location = match resp.site {
                        1 => "首位",
                        2 => "背包",
                        _ => "仓库",
                    };
                    show_toast_silent(
                        &state,
                        ToastKind::Success,
                        format!("成功购买 {name}！已放入{location}，余额: {}", resp.remaining_money),
                    );
                }
                Err(e) => show_toast_silent(&state, ToastKind::Error, format!("购买失败: {e}")),
            }
            state.buying.inner().set(false);
            reload(&state);
        }),
    );
}

/// 搜索防抖：每次输入重启计时器（对齐 Dioxus 版 SEARCH_DEBOUNCE_MS）。
/// 当前未使用：tairitsu 的 InputEvent 只带增量文本，搜索框改用
/// onchange 的 ChangeEvent（完整值，回车提交），保留此函数待上游
/// 补充 input 完整值语义后切回。（TimerOps 的 WitPlatform 实现是
/// wasm 专属，native 编不过，一并按目标门控。）
#[cfg(target_family = "wasm")]
#[allow(dead_code)]
fn debounce_reload(state: &ShopState) {
    use std::cell::Cell;
    use std::rc::Rc;
    use tairitsu_vdom::platform::TimerOps;

    thread_local! {
        static SEARCH_TIMER: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    }
    SEARCH_TIMER.with(|slot| {
        let Ok(platform) = tairitsu_web::WitPlatform::new() else {
            return;
        };
        if let Some(id) = slot.get() {
            platform.clear_timeout(id);
        }
        let state = state.clone();
        let id = platform.set_timeout(Box::new(move || reload(&state)), SEARCH_DEBOUNCE_MS);
        slot.set(Some(id));
    });
}

// ---------------------------------------------------------------------------
// 组件
// ---------------------------------------------------------------------------

#[component]
pub fn Shop(state: Box<ShopState>) -> VNode {
    rsx! {
        ThemeProvider {
            palette: "hikari".to_string(),
            primary: Some("#5cb8e6".to_string()),
            secondary: Some("#f5a623".to_string()),
            ShopCard { state: Some(state) }
        }
    }
}

#[component]
fn ShopCard(state: Box<ShopState>) -> VNode {
    // 首次装载即拉取（Dioxus 版由 use_resource 首次求值触发）。
    if state.items.get().is_none() && state.pets.get().is_none() {
        reload(&state);
    }

    let toast_state = state.clone();
    rsx! {
        div { class: "page-shop",
            div { class: "shop-card",
                div { class: "shop-card-header",
                    Icon { name: Some(IconName::Shoppingcart), size: Some(18), class: Some("header-icon".to_string()) }
                    span { "道具商店" }
                }
                div { class: "shop-card-body",
                    CategoryBar { state: Some(state.clone()) }
                    if state.category.get() == PET_CATEGORY {
                        PetsTab { state: Some(state.clone()) }
                    } else {
                        ItemsTab { state: Some(state.clone()) }
                    }
                }
            }
            if state.toast_visible.get() {
                Toast {
                    variant: toast_variant(state.toast_kind.get()),
                    message: state.toast_message.get(),
                    position: ToastPosition::BottomRight,
                    on_close: Some(tairitsu_vdom::Callback::new(move |_| {
                        toast_state.toast_visible.set(false)
                    })),
                }
            }
        }
    }
}

fn toast_variant(kind: ToastKind) -> ToastVariant {
    match kind {
        ToastKind::Success => ToastVariant::Success,
        ToastKind::Error => ToastVariant::Error,
    }
}

#[component]
fn CategoryBar(state: Box<ShopState>) -> VNode {
    let cats = categories();
    let active = state.category.get();
    let buying = state.buying.get();
    let keyword = state.keyword.get();
    let has_value = !keyword.trim().is_empty();
    let filter_open = state.filter_open.get();
    let filter_class = match (has_value, filter_open) {
        (true, true) => "shop-filter-btn has-value open",
        (true, false) => "shop-filter-btn has-value",
        (false, true) => "shop-filter-btn open",
        (false, false) => "shop-filter-btn",
    };

    rsx! {
        div { class: "shop-category-bar",
            div { class: "shop-category-group",
                for i in 0..cats.len() {
                    {
                        let state = state.clone();
                        let (cat_id, cat_name, cat_icon) = cats[i];
                        let is_pet = cat_id == PET_CATEGORY;
                        let class = if active == cat_id { "category-btn active" } else { "category-btn" };
                        let img_class = if is_pet { "cat-icon-img pet-cat-icon" } else { "cat-icon-img" };
                        let img_src = format!("{IMG_PATH}/{cat_icon}");
                        let cat_attr = match cat_id { Some(n) => n.to_string(), None => "pet".to_string() };
                        rsx! {
                            button {
                                class: class,
                                disabled: buying,
                                "data-pm-cat": cat_attr,
                                span { class: "cat-icon",
                                    img { class: img_class, src: img_src }
                                }
                                span { "{cat_name}" }
                            }
                        }
                    }
                }
            }
            FilterToggle { state: Some(state.clone()), filter_class: Some(filter_class.to_string()) }
        }
    }
}

#[component]
fn FilterToggle(state: Box<ShopState>, filter_class: String) -> VNode {
    let has_value = !state.keyword.get().trim().is_empty();
    rsx! {
        div { class: "shop-filter",
            button {
                class: filter_class,
                r#type: "button",
                title: "按名称筛选商品",
                "data-pm-filter-toggle": "1",
                if has_value {
                    span { class: "filter-icon filter-icon-filled",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            width: "16",
                            height: "16",
                            fill: "currentColor",
                            dangerous_inner_html: FUNNEL_PATH,
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
                            dangerous_inner_html: FUNNEL_PATH,
                        }
                    }
                }
            }
            if state.filter_open.get() {
                FilterPopover { state: Some(state) }
            }
        }
    }
}

const FUNNEL_PATH: &str = r#"<path d="M3 4.5h18l-6.8 7.6v7.4l-4.4-2.4v-5L3 4.5z" />"#;

#[component]
fn FilterPopover(state: Box<ShopState>) -> VNode {
    let keyword = state.keyword.get();
    let has_value = !keyword.trim().is_empty();
    rsx! {
        div {
            class: "shop-filter-backdrop",
            "data-pm-filter-close": "1",
        }
        div { class: "shop-filter-popover",
            label { class: "shop-filter-label", "按名称筛选" }
            div { class: "shop-filter-row",
                input {
                    class: "shop-filter-input",
                    r#type: "search",
                    placeholder: "输入名称后回车",
                    value: keyword,
                    autocomplete: "off",
                    "data-pm-filter-input": "1",
                }
                if has_value {
                    button {
                        class: "shop-filter-clear",
                        r#type: "button",
                        title: "清除筛选",
                        "data-pm-filter-clear": "1",
                        Icon { name: Some(IconName::X), size: Some(12), class: Some(String::new()) }
                    }
                }
            }
            p { class: "shop-filter-hint", "留空显示当前分类全部商品" }
        }
    }
}

#[component]
fn ItemsTab(state: Box<ShopState>) -> VNode {
    let data = state.items.get();
    let keyword_empty = state.keyword.get().trim().is_empty();

    match data {
        None => rsx! { div { class: "shop-loading", "正在加载商品..." } },
        Some(Err(e)) => rsx! { div { class: "error-message", "{e}" } },
        Some(Ok(items)) if items.is_empty() => {
            let msg = if keyword_empty { "暂无商品" } else { "没有找到匹配的商品，试试其他名称或分类" };
            let src = format!("{IMG_PATH}/item/box.gif");
            rsx! {
                div { class: "shop-empty",
                    img { class: "empty-icon-img", src: src }
                    p { "{msg}" }
                }
            }
        }
        Some(Ok(items)) => rsx! {
            div {
                div { class: "shop-tiles-grid",
                    for (idx , item) in items.iter().enumerate() {
                        { rsx! { ItemTile { state: Some(state.clone()), idx: Some(idx), item: Some(item.clone()) } } }
                    }
                }
                Paginator { state: Some(state) }
            }
        },
    }
}

#[component]
fn PetsTab(state: Box<ShopState>) -> VNode {
    let data = state.pets.get();
    let keyword_empty = state.keyword.get().trim().is_empty();

    match data {
        None => rsx! { div { class: "shop-loading", "正在加载宠物..." } },
        Some(Err(e)) => rsx! { div { class: "error-message", "{e}" } },
        Some(Ok(pets)) if pets.is_empty() => {
            let msg = if keyword_empty { "暂无可购买的宠物" } else { "没有找到匹配的宠物，试试其他名称" };
            let src = format!("{IMG_PATH}/item/box.gif");
            rsx! {
                div { class: "shop-empty",
                    img { class: "empty-icon-img", src: src }
                    p { "{msg}" }
                }
            }
        }
        Some(Ok(pets)) => rsx! {
            div {
                div { class: "shop-tiles-grid",
                    for (idx , pet) in pets.iter().enumerate() {
                        { rsx! { PetTile { state: Some(state.clone()), idx: Some(idx), pet: Some(pet.clone()) } } }
                    }
                }
                Paginator { state: Some(state) }
            }
        },
    }
}

/// 单个商品宫格：悬浮展示详情 tooltip（光标上方，不遮挡价格/购买按钮）。
#[component]
fn ItemTile(state: Box<ShopState>, idx: usize, item: ShopItem) -> VNode {
    let img_src = format!("{IMG_PATH}/item/{}.gif", item.image);
    let buy_class = if item.can_buy { "buy-btn" } else { "buy-btn disabled" };
    let buying = state.buying.get();
    let show_tooltip = state.hover_key.get() == Some(idx);
    let (tip_x, tip_y) = state.hover_pos.get();
    // "{ident}" 简写会移动字段，先取出字符串（i64 也无 IntoVNodeChild 实现）。
    let price = item.price.to_string();
    let name = item.name.clone();
    let title_text = item.name.clone();
    let type_name = item.type_name.clone();

    let item_id_attr = item.id.to_string();
    let tile_idx_attr = idx.to_string();
    rsx! {
        div {
            class: "shop-tile",
            "data-pm-tile": tile_idx_attr,
            "data-pm-tile-kind": "item",
            "data-pm-buy": item_id_attr,
            div { class: "shop-tile-icon",
                img { class: "tile-icon-img", src: img_src }
            }
            div { class: "shop-tile-name", title: title_text, "{name}" }
            div { class: "shop-tile-meta",
                span { class: "item-type-badge", "{type_name}" }
            }
            div { class: "shop-tile-foot",
                div { class: "item-price",
                    Icon { name: Some(IconName::Coins), size: Some(12), class: Some("price-icon".to_string()) }
                    span { class: "price-value", "{price}" }
                }
                button {
                    class: buy_class,
                    disabled: !item.can_buy || buying,
                    "购买"
                }
            }
            if show_tooltip {
                ItemTooltip { item: Some(item.clone()), x: Some(tip_x), y: Some(tip_y) }
            }
        }
    }
}

#[component]
fn ItemTooltip(item: ShopItem, x: f64, y: f64) -> VNode {
    let style = format!("left: {x}px; top: {y}px;");
    let effect = item.effect.clone();
    let price = item.price.to_string();
    let name = item.name.clone();
    let type_name = item.type_name.clone();
    let description = item.description.clone();
    let hp = format!("回复HP +{}", effect.addhp);
    let exp = format!("经验 +{}", effect.addexp);
    let lv = format!("等级 +{}", effect.addlv);
    rsx! {
        div { class: "shop-tile-tooltip",
            style: style,
            div { class: "tile-tooltip-header",
                span { class: "tile-tooltip-name", "{name}" }
                span { class: "item-type-badge", "{type_name}" }
            }
            if !description.is_empty() {
                p { class: "tile-tooltip-desc", "{description}" }
            }
            div { class: "tile-tooltip-stats",
                if effect.addhp != 0 { span { class: "tile-tooltip-stat", "{hp}" } }
                if effect.addexp != 0 { span { class: "tile-tooltip-stat", "{exp}" } }
                if effect.addlv != 0 { span { class: "tile-tooltip-stat", "{lv}" } }
            }
            div { class: "tile-tooltip-price",
                Icon { name: Some(IconName::Coins), size: Some(14), class: Some("price-icon".to_string()) }
                span { "{price}" }
            }
        }
    }
}

/// 单个宠物宫格：悬浮展示六维种族值 tooltip。
#[component]
fn PetTile(state: Box<ShopState>, idx: usize, pet: ShopPet) -> VNode {
    let img_src = format!("{IMG_PATH}/spm/{}.gif", pet.id);
    let buy_class = if pet.can_buy { "buy-btn" } else { "buy-btn disabled" };
    let buying = state.buying.get();
    let show_tooltip = state.hover_key.get() == Some(idx);
    let (tip_x, tip_y) = state.hover_pos.get();
    let has_type2 = pet.type_2.as_deref().map(|t| !t.is_empty()).unwrap_or(false);
    let price = pet.price.to_string();
    let name = pet.name.clone();
    let title_text = pet.name.clone();
    let type_1 = pet.type_1.clone();
    let type_2 = pet.type_2.clone().unwrap_or_default();

    let pet_id_attr = pet.id.to_string();
    let tile_idx_attr = idx.to_string();

    let hp = format!("HP {}", pet.hp);
    let atk = format!("攻击 {}", pet.atk);
    let def = format!("防御 {}", pet.def);
    let spatk = format!("特攻 {}", pet.spatk);
    let spdef = format!("特防 {}", pet.spdef);
    let speed = format!("速度 {}", pet.speed);

    rsx! {
        div {
            class: "shop-tile pet-tile",
            "data-pm-tile": tile_idx_attr,
            "data-pm-tile-kind": "pet",
            "data-pm-buypet": pet_id_attr,
            div { class: "shop-tile-icon",
                img { class: "tile-icon-img pet-sprite-img", src: img_src }
            }
            div { class: "shop-tile-name", title: title_text, "{name}" }
            div { class: "shop-tile-meta",
                span { class: "pet-type-1", "{type_1}" }
                if has_type2 {
                    span { class: "pet-type-2", "{type_2}" }
                }
            }
            div { class: "shop-tile-foot",
                div { class: "item-price",
                    Icon { name: Some(IconName::Coins), size: Some(12), class: Some("price-icon".to_string()) }
                    span { class: "price-value", "{price}" }
                }
                button {
                    class: buy_class,
                    disabled: !pet.can_buy || buying,
                    "购买"
                }
            }
            if show_tooltip {
                PetTooltip {
                    pet: Some(pet.clone()),
                    x: Some(tip_x),
                    y: Some(tip_y),
                    hp: Some(hp),
                    atk: Some(atk),
                    def: Some(def),
                    spatk: Some(spatk),
                    spdef: Some(spdef),
                    speed: Some(speed),
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[component]
fn PetTooltip(
    pet: ShopPet,
    x: f64,
    y: f64,
    hp: String,
    atk: String,
    def: String,
    spatk: String,
    spdef: String,
    speed: String,
) -> VNode {
    let style = format!("left: {x}px; top: {y}px;");
    let has_type2 = pet.type_2.as_deref().map(|t| !t.is_empty()).unwrap_or(false);
    let price = pet.price.to_string();
    let name = pet.name.clone();
    let title_text = pet.name.clone();
    let type_1 = pet.type_1.clone();
    let type_2 = pet.type_2.clone().unwrap_or_default();
    rsx! {
        div { class: "shop-tile-tooltip",
            style: style,
            div { class: "tile-tooltip-header",
                span { class: "tile-tooltip-name", "{name}" }
                span { class: "pet-type-1", "{type_1}" }
                if has_type2 {
                    span { class: "pet-type-2", "{type_2}" }
                }
            }
            div { class: "tile-tooltip-stats pet-stats-grid",
                span { class: "tile-tooltip-stat", "{hp}" }
                span { class: "tile-tooltip-stat", "{atk}" }
                span { class: "tile-tooltip-stat", "{def}" }
                span { class: "tile-tooltip-stat", "{spatk}" }
                span { class: "tile-tooltip-stat", "{spdef}" }
                span { class: "tile-tooltip-stat", "{speed}" }
            }
            div { class: "tile-tooltip-price",
                Icon { name: Some(IconName::Coins), size: Some(14), class: Some("price-icon".to_string()) }
                span { "{price}" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 分页器
// ---------------------------------------------------------------------------

#[component]
fn Paginator(state: Box<ShopState>) -> VNode {
    let total = state.total_pages.get().max(1);
    let current = (state.page.get() as usize).clamp(1, total);
    let entries = windowed_entries(current, total);

    rsx! {
        div { class: "shop-pagination",
            button {
                class: "pager-btn pager-nav",
                r#type: "button",
                disabled: current <= 1,
                "data-pm-page": "prev",
                Icon { name: Some(IconName::ChevronLeft), size: Some(14), class: Some(String::new()) }
            }
            for i in 0..entries.len() {
                {
                    match entries[i] {
                        PagerEntry::Page(p) => {
                            let active = p == current;
                            let class = if active { "pager-btn pager-num active" } else { "pager-btn pager-num" };
                            let label = p.to_string();
                            let page_attr = label.clone();
                            rsx! {
                                button {
                                    class: class,
                                    r#type: "button",
                                    disabled: total == 1,
                                    "data-pm-page": page_attr,
                                    "{label}"
                                }
                            }
                        }
                        PagerEntry::Ellipsis => rsx! { span { class: "pager-ellipsis", "…" } },
                    }
                }
            }
            button {
                class: "pager-btn pager-nav",
                r#type: "button",
                disabled: current >= total,
                "data-pm-page": "next",
                Icon { name: Some(IconName::ChevronRight), size: Some(14), class: Some(String::new()) }
            }
        }
    }
}
