//! 事件委托层：所有交互监听器只在 #app 上注册一次（启动时），
//! 树重挂载（remount_latest）零监听器开销。
//!
//! 背景：逐元素监听在每次重挂载时要重新注册 ~30 个监听器（异步导入），
//! jco 的任务/锁账本在数轮后退化（第 5 次重挂载后注册失效，点击全哑）。
//! 委托把注册次数降到常量 4（click/change/mouseover/mouseout），
//! 命中判定靠瓦片上的 data-pm-* 属性 + 自 #app 向下的子树匹配。
//!
//! jco 下事件目标只有原始句柄，closest 没有对应绑定——这里从
//! event.target 沿 parent 链向上最多走 12 层读属性判定。

use tairitsu_web::WitPlatform;
use tairitsu_web::wit_platform::WitElement;
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::get_attribute;
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::node::get_parent_element;

use crate::shop_logic::shop_tooltip_position;
use crate::state::ShopState;

const ITEM_TOOLTIP_H: f64 = 130.0;
const PET_TOOLTIP_H: f64 = 190.0;

fn attr_of(handle: u64, name: &str) -> Option<String> {
    get_attribute(handle, name)
}

/// 从 target 沿 parent 链向上找第一个带指定属性的元素句柄。
fn find_with_attr(target: u64, name: &str) -> Option<(u64, String)> {
    let mut cur = target;
    for _ in 0..12 {
        if let Some(v) = attr_of(cur, name) {
            return Some((cur, v));
        }
        match get_parent_element(cur) {
            Some(parent) => cur = parent,
            None => return None,
        }
    }
    None
}

/// 启动时在 #app 上注册全部委托监听器。
pub fn install(app: &WitElement, state: &ShopState) {
    use tairitsu_vdom::platform::DomOps;
    let platform = WitPlatform::new().expect("platform");

    // ---- click：分类 / 分页 / 购买 / 筛选 ----
    {
        let st = state.clone();
        platform.add_event_listener(
            app,
            "click",
            Box::new(move |e| {
                let Some(target) = e.as_any().downcast_ref::<tairitsu_vdom::MouseEvent>() else {
                    return;
                };
                let Some(target) = target.target else { return };
                crate::delegation::dispatch_click(&st, target);
            }),
        );
    }

    // ---- change：搜索输入（回车提交） ----
    {
        let st = state.clone();
        platform.add_event_listener(
            app,
            "change",
            Box::new(move |e| {
                let Some(ev) = e.as_any().downcast_ref::<tairitsu_vdom::ChangeEvent>() else {
                    return;
                };
                let Some(target) = ev.target else { return };
                if attr_of(target, "data-pm-filter-input").is_some() {
                    crate::shop::reload_from_filter(&st, &ev.value);
                }
            }),
        );
    }

    // ---- mouseover/mouseout：tooltip（mouseenter 不冒泡，用 over/out 判定瓦片进出） ----
    {
        let st = state.clone();
        platform.add_event_listener(
            app,
            "mouseover",
            Box::new(move |e| {
                let Some(ev) = e.as_any().downcast_ref::<tairitsu_vdom::MouseEvent>() else {
                    return;
                };
                let Some(target) = ev.target else { return };
                let Some((_, idx_str)) = find_with_attr(target, "data-pm-tile") else {
                    return;
                };
                let Ok(idx) = idx_str.parse::<usize>() else { return };
                let kind = find_with_attr(target, "data-pm-tile-kind")
                    .map(|(_, k)| k)
                    .unwrap_or_default();
                let h = if kind == "pet" { PET_TOOLTIP_H } else { ITEM_TOOLTIP_H };
                let win_w = crate::shop::viewport_width_or_default();
                let (x, y) = shop_tooltip_position(ev.client_x as f64, ev.client_y as f64, h, win_w);
                // 内部值写入 + 请求重挂载：ReactiveSignal::set 会走
                // mark_dirty → rAF → flush_render → apply，而 apply 路径
                // 在 jco 下挂起（§10.1.3）；tooltip 是纯派生 UI，直接
                // 重建整树最稳。
                st.hover_pos.inner().set((x, y));
                st.hover_key.inner().set(Some(idx));
                crate::request_remount();
            }),
        );
    }
    {
        let st = state.clone();
        platform.add_event_listener(
            app,
            "mouseout",
            Box::new(move |e| {
                let Some(ev) = e.as_any().downcast_ref::<tairitsu_vdom::MouseEvent>() else {
                    return;
                };
                let Some(target) = ev.target else { return };
                // 离开瓦片或其内部任一节点都收起 tooltip（瓦片内移动会再次触发 over）。
                if find_with_attr(target, "data-pm-tile").is_some() {
                    st.hover_key.inner().set(None);
                    crate::request_remount();
                }
            }),
        );
    }
}

fn dispatch_click(state: &ShopState, target: u64) {
    // 分类
    if let Some((_, v)) = find_with_attr(target, "data-pm-cat") {
        let cat = if v == "pet" { None } else { v.parse::<u32>().ok() };
        crate::shop::select_category(state, cat);
        return;
    }
    // 分页
    if let Some((_, v)) = find_with_attr(target, "data-pm-page") {
        let page = match v.as_str() {
            "prev" => state.page.get().saturating_sub(1).max(1),
            "next" => state.page.get() + 1,
            n => n.parse::<u32>().unwrap_or(1),
        };
        crate::shop::goto_page(state, page);
        return;
    }
    // 购买物品
    if let Some((_, v)) = find_with_attr(target, "data-pm-buy") {
        if let Ok(id) = v.parse::<u64>() {
            crate::shop::buy_item_by_id(state, id);
        }
        return;
    }
    // 购买宠物
    if let Some((_, v)) = find_with_attr(target, "data-pm-buypet") {
        if let Ok(id) = v.parse::<u64>() {
            crate::shop::buy_pet_by_id(state, id);
        }
        return;
    }
    // 筛选
    if find_with_attr(target, "data-pm-filter-toggle").is_some() {
        let next = !state.filter_open.get();
        state.filter_open.set(next);
        crate::shop::refresh(state);
        return;
    }
    if find_with_attr(target, "data-pm-filter-close").is_some() {
        state.filter_open.set(false);
        crate::shop::refresh(state);
        return;
    }
    if find_with_attr(target, "data-pm-filter-clear").is_some() {
        crate::shop::reload_from_filter(state, "");
        return;
    }
}
