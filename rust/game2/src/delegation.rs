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

use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::get_attribute;
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::node::get_parent_element;
use tairitsu_web::wit_platform::WitElement;
use tairitsu_web::WitPlatform;

use crate::app_state::AppState;
use crate::shop_logic::shop_tooltip_position;

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
pub fn install(app: &WitElement, state: &AppState) {
    use tairitsu_vdom::platform::DomOps;
    let platform = WitPlatform::new().expect("platform");

    // ---- click：页面导航 / 欢迎页初始化 / 分类 / 分页 / 购买 / 筛选 ----
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
                dispatch_click(&st, target);
            }),
        );
    }

    // ---- change：搜索输入（回车提交；背包搜索同样走 change） ----
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
                    crate::shop::reload_from_filter(&st.shop, &ev.value);
                } else if attr_of(target, "data-pm-inv-search").is_some() {
                    st.inv.search_input.set(ev.value.clone());
                    crate::inventory::apply_search(&st, &ev.value);
                } else if attr_of(target, "data-pm-my-pm-nick-input").is_some() {
                    st.my_pm.edit_value.inner().set(ev.value.clone());
                } else if let Some((_, key)) =
                    find_with_attr(target, "data-pm-ev-input").map(|(h, v)| (h, v))
                {
                    // 努力值计算器输入："cur-N" / "add-N"
                    if let Some((added, idx)) = key.split_once('-') {
                        let is_add = added == "add";
                        if let Ok(i) = idx.parse::<usize>() {
                            crate::my_pokemon::set_ev_value(&st, i, is_add, &ev.value);
                        }
                    }
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
                let Ok(idx) = idx_str.parse::<usize>() else {
                    return;
                };
                let kind = find_with_attr(target, "data-pm-tile-kind")
                    .map(|(_, k)| k)
                    .unwrap_or_default();
                let h = if kind == "pet" {
                    PET_TOOLTIP_H
                } else {
                    ITEM_TOOLTIP_H
                };
                let win_w = crate::shop::viewport_width_or_default();
                let (x, y) =
                    shop_tooltip_position(ev.client_x as f64, ev.client_y as f64, h, win_w);
                // 内部值写入 + 请求重挂载：ReactiveSignal::set 会走
                // mark_dirty → rAF → flush_render → apply，而 apply 路径
                // 在 jco 下挂起（§10.1.3）；tooltip 是纯派生 UI，直接
                // 重建整树最稳。
                st.shop.hover_pos.inner().set((x, y));
                st.shop.hover_key.inner().set(Some(idx));
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
                    st.shop.hover_key.inner().set(None);
                    crate::request_remount();
                }
            }),
        );
    }
}

fn dispatch_click(state: &AppState, target: u64) {
    // 顶部导航：页面切换
    if let Some((_, v)) = find_with_attr(target, "data-pm-nav") {
        let route = match v.as_str() {
            "shop" => crate::app_state::Route::Shop,
            "inventory" => crate::app_state::Route::Inventory,
            "center" => crate::app_state::Route::PokemonCenter,
            "my-pokemon" => crate::app_state::Route::MyPokemon,
            _ => crate::app_state::Route::Home,
        };
        state.route.set(route);
        crate::request_remount();
        return;
    }
    // 宠物中心：治疗单只 / 全部治疗
    if let Some((_, v)) = find_with_attr(target, "data-pm-center-heal") {
        if let Ok(pid) = v.parse::<u64>() {
            let flee = find_with_attr(target, "data-pm-center-flee")
                .map(|(_, f)| f == "1")
                .unwrap_or(false);
            crate::pokemon_center::heal_one(state, pid, flee);
        }
        return;
    }
    if find_with_attr(target, "data-pm-center-heal-all").is_some() {
        crate::pokemon_center::heal_all(state);
        return;
    }
    // 我的宝可梦：标签页 / 宠物选择 / 昵称编辑 / 装备
    if let Some((_, v)) = find_with_attr(target, "data-pm-my-pm-tab") {
        let tab = match v.as_str() {
            "equipment" => crate::state::MyPokemonTab::Equipment,
            "skills" => crate::state::MyPokemonTab::Skills,
            _ => crate::state::MyPokemonTab::Stats,
        };
        crate::my_pokemon::switch_tab(state, tab);
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-my-pm-select") {
        if let Ok(pid) = v.parse::<u64>() {
            crate::my_pokemon::select_pokemon(state, pid);
        }
        return;
    }
    // 装备：注意槽位与物品卡的匹配顺序——物品卡在物品栏区块内，
    // 区块自身的 data-pm-eq-inv-section 不能先命中。
    if let Some((_, v)) = find_with_attr(target, "data-pm-eq-inv-item") {
        if let Ok(mid) = v.parse::<u64>() {
            crate::my_pokemon::select_inventory_item(state, mid);
        }
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-eq-slot") {
        if let Ok(idx) = v.parse::<u32>() {
            crate::my_pokemon::click_slot(state, idx);
        }
        return;
    }
    if find_with_attr(target, "data-pm-eq-inv-section").is_some() {
        crate::my_pokemon::unequip_selected_from_section(state);
        return;
    }
    if find_with_attr(target, "data-pm-eq-reload").is_some() {
        state.my_pm.equipment.inner().set(None);
        crate::request_remount();
        crate::my_pokemon::ensure_equipment(state);
        return;
    }
    // 技能：卡片选中 / 遗忘 / 空槽学习 / 弹窗内学习与关闭
    if let Some((_, v)) = find_with_attr(target, "data-pm-skill-forget") {
        if let Ok(tid) = v.parse::<u64>() {
            crate::my_pokemon::forget_skill(state, tid);
        }
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-skill-card") {
        if let Ok(tid) = v.parse::<u64>() {
            crate::my_pokemon::toggle_skill_card(state, tid);
        }
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-learn-item") {
        if let Ok(sid) = v.parse::<u64>() {
            crate::my_pokemon::learn_skill(state, sid);
        }
        return;
    }
    if find_with_attr(target, "data-pm-skill-learn-slot").is_some() {
        crate::my_pokemon::open_learn_modal(state);
        return;
    }
    if find_with_attr(target, "data-pm-learn-reload").is_some() {
        state.my_pm.learnable.inner().set(None);
        crate::request_remount();
        crate::my_pokemon::open_learn_modal(state);
        return;
    }
    if find_with_attr(target, "data-pm-learn-close").is_some() {
        crate::my_pokemon::close_learn_modal(state);
        return;
    }
    // 属性页弹窗：努力值计算器 / 进化路径 / 进化
    if find_with_attr(target, "data-pm-my-pm-ev-open").is_some() {
        crate::my_pokemon::open_ev_modal(state);
        return;
    }
    if find_with_attr(target, "data-pm-ev-close").is_some() {
        crate::my_pokemon::close_ev_modal(state);
        return;
    }
    if find_with_attr(target, "data-pm-my-pm-evo-path-open").is_some() {
        crate::my_pokemon::open_evo_path_modal(state);
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-evo-path-tab") {
        crate::my_pokemon::switch_evo_path_tab(state, v == "forward");
        return;
    }
    if find_with_attr(target, "data-pm-evo-path-close").is_some() {
        crate::my_pokemon::close_evo_path_modal(state);
        return;
    }
    if find_with_attr(target, "data-pm-my-pm-evo-open").is_some() {
        crate::my_pokemon::open_evolution_modal(state);
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-evo-do") {
        let name = find_with_attr(target, "data-pm-evo-do-name")
            .map(|(_, n)| n)
            .unwrap_or_default();
        if let Ok(to_id) = v.parse::<u64>() {
            crate::my_pokemon::evolve_to(state, to_id, &name);
        }
        return;
    }
    if find_with_attr(target, "data-pm-evo-modal-close").is_some() {
        crate::my_pokemon::close_evolution_modal(state);
        return;
    }
    if find_with_attr(target, "data-pm-my-pm-nick-edit").is_some() {
        let current = state
            .my_pm
            .detail
            .get()
            .and_then(|r| r.ok())
            .map(|d| {
                d.nickname
                    .clone()
                    .filter(|n| !n.is_empty())
                    .unwrap_or_else(|| d.name.clone())
            })
            .unwrap_or_default();
        crate::my_pokemon::begin_edit(state, &current);
        return;
    }
    if find_with_attr(target, "data-pm-my-pm-nick-save").is_some() {
        crate::my_pokemon::submit_rename(state);
        return;
    }
    if find_with_attr(target, "data-pm-my-pm-nick-cancel").is_some() {
        crate::my_pokemon::cancel_edit(state);
        return;
    }
    // 背包：分类 / 分页 / 使用 / 弹窗
    if let Some((_, v)) = find_with_attr(target, "data-pm-inv-cat") {
        let cat = if v == "all" {
            None
        } else {
            v.parse::<u32>().ok()
        };
        crate::inventory::select_category(state, cat);
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-inv-page") {
        let page = match v.as_str() {
            "prev" => state.inv.page.get().saturating_sub(1).max(1),
            "next" => state.inv.page.get() + 1,
            n => n.parse::<u32>().unwrap_or(1),
        };
        crate::inventory::goto_page(state, page);
        return;
    }
    if find_with_attr(target, "data-pm-inv-search-submit").is_some() {
        crate::inventory::apply_search(state, &state.inv.search_input.get());
        return;
    }
    if find_with_attr(target, "data-pm-inv-search-clear").is_some() {
        state.inv.search_input.set(String::new());
        crate::inventory::apply_search(state, "");
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-inv-use") {
        // data 属性带的是物品类型 ID（type_id）——use_item/get_usable_pokemon
        // 都以它为键（对齐 Dioxus 版），背包行 id 不外发。
        let item_id = v.parse::<u64>().unwrap_or(0);
        let meta = state
            .inv
            .items
            .get()
            .and_then(|r| r.ok())
            .and_then(|items| {
                items
                    .iter()
                    .find(|i| i.type_id == item_id)
                    .map(|i| (i.item_type, i.use_target.clone()))
            });
        if let Some((item_type, use_target)) = meta {
            crate::inventory::use_item(state, item_id, item_type, use_target);
        }
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-inv-pick") {
        if let Ok(pid) = v.parse::<u64>() {
            crate::inventory::confirm_use_on_pokemon(state, pid);
        }
        return;
    }
    if find_with_attr(target, "data-pm-inv-modal-close").is_some() {
        crate::inventory::close_target_modal(state);
        return;
    }
    // 欢迎页：开始冒险（新玩家初始化）
    if find_with_attr(target, "data-pm-welcome-init").is_some() {
        crate::welcome::initialize(state);
        return;
    }
    let shop = state.shop.clone();
    // 分类
    if let Some((_, v)) = find_with_attr(target, "data-pm-cat") {
        let cat = if v == "pet" {
            None
        } else {
            v.parse::<u32>().ok()
        };
        crate::shop::select_category(&shop, cat);
        return;
    }
    // 分页
    if let Some((_, v)) = find_with_attr(target, "data-pm-page") {
        let page = match v.as_str() {
            "prev" => shop.page.get().saturating_sub(1).max(1),
            "next" => shop.page.get() + 1,
            n => n.parse::<u32>().unwrap_or(1),
        };
        crate::shop::goto_page(&shop, page);
        return;
    }
    // 购买物品
    if let Some((_, v)) = find_with_attr(target, "data-pm-buy") {
        if let Ok(id) = v.parse::<u64>() {
            crate::shop::buy_item_by_id(&shop, id);
        }
        return;
    }
    // 购买宠物
    if let Some((_, v)) = find_with_attr(target, "data-pm-buypet") {
        if let Ok(id) = v.parse::<u64>() {
            crate::shop::buy_pet_by_id(&shop, id);
        }
        return;
    }
    // 筛选
    if find_with_attr(target, "data-pm-filter-toggle").is_some() {
        let next = !shop.filter_open.get();
        shop.filter_open.set(next);
        crate::shop::refresh(&shop);
        return;
    }
    if find_with_attr(target, "data-pm-filter-close").is_some() {
        shop.filter_open.set(false);
        crate::shop::refresh(&shop);
        return;
    }
    if find_with_attr(target, "data-pm-filter-clear").is_some() {
        crate::shop::reload_from_filter(&shop, "");
        return;
    }
}
