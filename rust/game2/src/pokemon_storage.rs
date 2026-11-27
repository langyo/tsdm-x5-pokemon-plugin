//! 仓库页（tairitsu 版，1:1 移植自 rust/game/src/pages/pokemon_storage.rs）。
//! 同名同类直用 game.css（.page-pokemon-storage / .storage-panel /
//! .pokemon-slot / .select-actions-bar …）。
//!
//! 覆盖：背包宠物 6 槽、仓库筛选（名字/属性/等级区间/闪光/排序）、
//! 多选批量放生、设为首位、移入仓库/背包、放生（带濒危/战斗中/背包满
//! 的门禁提示）。筛选与排序逻辑在 storage_logic.rs（带单测）。
//!
//! 与 Dioxus 版的两处有意差异：
//! - 点击卡片展开内联操作行（Dioxus 是浮层菜单；移植时委托架构下内联更稳，沿用）。
//! - 战斗中遮罩随战斗页移植接入（当前无战斗态，全部按非战斗渲染）。
//!
//! 事件用 tairitsu 原生绑定：搜索框逐字过滤（oninput，与 Dioxus 版一致）；
//! 属性 / 等级区间 / 闪光 / 排序走 change（Dioxus 版等级区间同样是 oninput）。

use hikari_components::{Button, ButtonProps};
use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use crate::app_state::AppState;
use crate::state::{ToastKind, STORAGE_PAGE_SIZE};
use crate::storage_logic::{
    hp_class_storage, hp_health_status, is_weak_state, matching_storage_ids, parse_level_range,
    selected_matching_ids, StorageEntry, StorageFilters, StorageSort,
};
use crate::toast;

const IMG_PATH: &str = "source/plugin/pokemon/images";

/// 18 种属性（对齐 Dioxus 版 pokemon_types）。
const POKEMON_TYPES: [&str; 18] = [
    "普通", "火", "水", "草", "电", "冰", "格斗", "毒", "地面", "飞行", "超能", "虫", "岩石",
    "幽灵", "龙", "恶", "钢", "妖精",
];

/// 首次进入装载：仓库与宠物中心共用整表列表。
pub fn ensure_loaded(state: &AppState) {
    if state.center.pokemons.get().is_none() {
        let st = state.clone();
        crate::api::pokemon_list(Box::new(move |result| {
            st.center.pokemons.inner().set(Some(result));
            crate::request_remount();
        }));
    }
}

fn reload_list(state: &AppState) {
    let st = state.clone();
    crate::api::pokemon_list(Box::new(move |result| {
        st.center.pokemons.inner().set(Some(result));
        crate::request_remount();
    }));
}

// ---------------------------------------------------------------------------
// 动作入口（本页各元素的原生 onclick / onchange / oninput 回调调用）
// ---------------------------------------------------------------------------

pub fn set_filter(state: &AppState, field: &str, value: &str) {
    let st = state.storage.clone();
    match field {
        "type" => st.type_filter.inner().set(value.to_string()),
        "shiny" => st.shiny_filter.inner().set(value.to_string()),
        "sort" => st.sort_value.inner().set(value.to_string()),
        "min" => st.min_level.inner().set(value.to_string()),
        "max" => st.max_level.inner().set(value.to_string()),
        "search" => st.search_input.inner().set(value.to_string()),
        _ => {}
    }
    st.reset_view();
    crate::request_remount();
}

pub fn clear_filters(state: &AppState) {
    let st = state.storage.clone();
    st.search_input.inner().set(String::new());
    st.type_filter.inner().set(String::new());
    st.min_level.inner().set(String::new());
    st.max_level.inner().set(String::new());
    st.shiny_filter.inner().set(String::new());
    st.sort_value.inner().set("oldest".to_string());
    st.reset_view();
    crate::request_remount();
}

pub fn enter_select_mode(state: &AppState) {
    let st = state.storage.clone();
    st.select_mode.inner().set(true);
    crate::request_remount();
}

pub fn exit_select_mode(state: &AppState) {
    let st = state.storage.clone();
    st.select_mode.inner().set(false);
    st.selected_ids.inner().set(Vec::new());
    crate::request_remount();
}

/// 点卡片：多选模式下切换勾选，否则展开/收起操作行。
pub fn click_card(state: &AppState, id: u64) {
    let st = state.storage.clone();
    if st.action_loading.get() {
        return;
    }
    if st.select_mode.get() {
        let mut ids = st.selected_ids.get();
        if let Some(pos) = ids.iter().position(|&x| x == id) {
            ids.remove(pos);
        } else {
            ids.push(id);
        }
        st.selected_ids.inner().set(ids);
        crate::request_remount();
        return;
    }
    let next = if st.open_menu.get() == Some(id) {
        None
    } else {
        Some(id)
    };
    st.open_menu.inner().set(next);
    crate::request_remount();
}

/// 全选当前筛选结果（执行时以最新筛选重核，对齐 Dioxus 版注释）。
pub fn select_all(state: &AppState) {
    let st = state.storage.clone();
    let matching = current_matching_ids(state);
    st.selected_ids.inner().set(matching);
    crate::request_remount();
}

/// 批量放生：串行执行，完成后统一反馈。
pub fn batch_release(state: &AppState) {
    let st = state.storage.clone();
    if st.action_loading.get() {
        return;
    }
    let matching = current_matching_ids(state);
    let ids = selected_matching_ids(&st.selected_ids.get(), &matching);
    if ids.is_empty() {
        return;
    }
    st.action_loading.inner().set(true);
    crate::request_remount();
    release_sequence(state.clone(), ids, 0, 0, 0);
}

fn release_sequence(state: AppState, ids: Vec<u64>, idx: usize, ok: u32, fail: u32) {
    if idx >= ids.len() {
        let st = state.storage.clone();
        st.action_loading.inner().set(false);
        st.select_mode.inner().set(false);
        st.selected_ids.inner().set(Vec::new());
        let (kind, msg) = if fail == 0 {
            (ToastKind::Success, format!("成功放生 {ok} 只宠物"))
        } else {
            (
                ToastKind::Error,
                format!("放生完成：成功 {ok} 只，失败 {fail} 只"),
            )
        };
        toast::show(kind, msg);
        crate::request_remount();
        reload_list(&state);
        return;
    }
    let id = ids[idx];
    let next_state = state.clone();
    crate::api::release_pokemon(
        id,
        Box::new(move |result| {
            let (ok, fail) = if result.is_ok() {
                (ok + 1, fail)
            } else {
                (ok, fail + 1)
            };
            release_sequence(next_state, ids, idx + 1, ok, fail);
        }),
    );
}

/// 写操作收尾（单只路径）。
fn finish_action(state: &AppState, message: Option<(ToastKind, String)>) {
    let st = state.storage.clone();
    if let Some((kind, msg)) = message {
        toast::show(kind, msg);
    }
    st.action_loading.inner().set(false);
    st.open_menu.inner().set(None);
    crate::request_remount();
    reload_list(state);
}

pub fn set_first(state: &AppState, id: u64) {
    let st = state.storage.clone();
    if st.action_loading.get() {
        return;
    }
    st.action_loading.inner().set(true);
    crate::request_remount();
    let st2 = state.clone();
    crate::api::set_first_pokemon(
        id,
        Box::new(move |result| {
            finish_action(
                &st2,
                match result {
                    Ok(()) => None,
                    Err(e) => Some((ToastKind::Error, format!("设为首位失败: {e}"))),
                },
            )
        }),
    );
}

pub fn move_to_site(state: &AppState, id: u64, site: u8, err_prefix: &str) {
    let st = state.storage.clone();
    if st.action_loading.get() {
        return;
    }
    st.action_loading.inner().set(true);
    crate::request_remount();
    let st2 = state.clone();
    let prefix = err_prefix.to_string();
    crate::api::move_pokemon_to_site(
        id,
        site,
        Box::new(move |result| {
            finish_action(
                &st2,
                match result {
                    Ok(()) => None,
                    Err(e) => Some((ToastKind::Error, format!("{prefix}失败: {e}"))),
                },
            )
        }),
    );
}

pub fn release_one(state: &AppState, id: u64) {
    let st = state.storage.clone();
    if st.action_loading.get() {
        return;
    }
    st.action_loading.inner().set(true);
    crate::request_remount();
    let st2 = state.clone();
    crate::api::release_pokemon(
        id,
        Box::new(move |result| {
            finish_action(
                &st2,
                match result {
                    Ok(()) => Some((ToastKind::Success, "放生成功".to_string())),
                    Err(e) => Some((ToastKind::Error, format!("放生失败: {e}"))),
                },
            )
        }),
    );
}

pub fn load_more(state: &AppState) {
    let st = state.storage.clone();
    let next = st.visible_count.get() + STORAGE_PAGE_SIZE;
    st.visible_count.inner().set(next);
    crate::request_remount();
}

/// 当前筛选结果（id 列表，按排序）。等级区间非法时回退为不过滤
/// （Dioxus 版此时显示错误文案并保持空集——这里保持同语义：Err → 空）。
fn current_matching_ids(state: &AppState) -> Vec<u64> {
    let Some(list) = state.center.pokemons.get().and_then(|r| r.ok()) else {
        return Vec::new();
    };
    let (min, max) = match parse_level_range(
        &state.storage.min_level.get(),
        &state.storage.max_level.get(),
    ) {
        Ok(range) => range,
        Err(_) => return Vec::new(),
    };
    let filters = StorageFilters {
        name: &state.storage.search_input.get(),
        pokemon_type: &state.storage.type_filter.get(),
        min_level: min,
        max_level: max,
        shiny: match state.storage.shiny_filter.get().as_str() {
            "shiny" => Some(true),
            "normal" => Some(false),
            _ => None,
        },
        sort: StorageSort::from_value(&state.storage.sort_value.get()),
    };
    let entries: Vec<StorageEntry> = list
        .iter()
        .filter(|p| p.site >= 3)
        .map(|p| StorageEntry {
            id: p.id,
            name: &p.name,
            nickname: p.nickname.as_deref(),
            type_1: &p.type_1,
            type_2: p.type_2.as_deref(),
            level: p.level,
            shiny: p.is_shiny,
        })
        .collect();
    matching_storage_ids(&entries, &filters)
}

// ---------------------------------------------------------------------------
// 组件
// ---------------------------------------------------------------------------

#[component]
pub fn PokemonStorage(state: Box<AppState>) -> VNode {
    let st = state;
    let list_state = st.center.pokemons.get();
    let loading = list_state.is_none();
    let list = list_state
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();

    let bag: Vec<crate::state::PokemonBasic> = list
        .iter()
        .filter(|p| p.site == 1 || p.site == 2)
        .cloned()
        .collect();
    let storage: Vec<crate::state::PokemonBasic> =
        list.iter().filter(|p| p.site >= 3).cloned().collect();

    let matching = current_matching_ids(&st);
    let by_id: std::collections::HashMap<u64, &crate::state::PokemonBasic> =
        storage.iter().map(|p| (p.id, p)).collect();
    let filtered: Vec<crate::state::PokemonBasic> = matching
        .iter()
        .filter_map(|id| by_id.get(id).map(|p| (*p).clone()))
        .collect();
    let visible_count = st.storage.visible_count.get();
    let shown: Vec<crate::state::PokemonBasic> =
        filtered.iter().take(visible_count).cloned().collect();

    let select_mode = st.storage.select_mode.get();
    let selected_ids = st.storage.selected_ids.get();
    let selected_count = selected_matching_ids(&selected_ids, &matching).len();
    let storage_count = filtered.len();
    let count_text = format!("{storage_count} / {} 只", storage.len());
    let level_err = parse_level_range(&st.storage.min_level.get(), &st.storage.max_level.get())
        .err()
        .map(|m| m.to_string());

    // 背包 6 槽（含空槽）。
    let mut bag_cells: Vec<VNode> = Vec::with_capacity(6);
    for i in 0..6usize {
        if i < bag.len() {
            bag_cells.push(slot_card(
                &bag[i],
                false,
                false,
                &selected_ids,
                select_mode,
                &st,
            ));
        } else {
            bag_cells.push(rsx! { div { class: "pokemon-slot empty-slot" } });
        }
    }

    // 仓库网格。
    let storage_body = if loading {
        rsx! { div { class: "panel-loading", "加载中..." } }
    } else if filtered.is_empty() {
        let text = if storage.is_empty() {
            "仓库中没有宠物"
        } else {
            "没有匹配的宠物"
        };
        rsx! { div { class: "panel-empty", "{text}" } }
    } else {
        let mut cells: Vec<VNode> = shown
            .iter()
            .map(|p| slot_card(p, true, false, &selected_ids, select_mode, &st))
            .collect();
        let more = if filtered.len() > shown.len() {
            let label = format!("加载更多（已显示 {}/{}）", shown.len(), filtered.len());
            let more_state = st.clone();
            rsx! {
                div { class: "storage-load-more",
                    Button {
                        class: ("select-action-btn").to_string(),
                        onclick: crate::button::click(move || load_more(&more_state)),
                        "{label}"
                                        }
                }
            }
        } else {
            VNode::empty()
        };
        cells.push(more);
        rsx! {
            div { class: "pokemon-grid storage-grid",
                for i in 0..cells.len() {
                    ..vec![cells[i].clone()]
                }
            }
        }
    };

    // 多选操作条。
    let select_bar = if select_mode && !filtered.is_empty() {
        let all_label = format!("全选筛选结果 ({storage_count})");
        let release_label = format!("放生 ({selected_count})");
        let release_cls = if selected_count == 0 {
            "select-action-btn danger"
        } else {
            "select-action-btn danger"
        };
        let all_state = st.clone();
        let release_state = st.clone();
        rsx! {
            div { class: "select-actions-bar",
                Button {
                    class: ("select-action-btn").to_string(),
                    onclick: crate::button::click(move || select_all(&all_state)),
                    "{all_label}"
                                }
                Button {
                    class: (release_cls).to_string(),
                    disabled: selected_count == 0,
                    onclick: crate::button::click(move || batch_release(&release_state)),
                    "{release_label}"
                                }
            }
        }
    } else {
        VNode::empty()
    };

    // 筛选条。
    let search_value = st.storage.search_input.get();
    let type_value = st.storage.type_filter.get();
    let shiny_value = st.storage.shiny_filter.get();
    let sort_value = st.storage.sort_value.get();
    let min_value = st.storage.min_level.get();
    let max_value = st.storage.max_level.get();
    // 各筛选控件的原生回调各持一份状态（闭包要求 'static）。
    let search_state = st.clone();
    let type_state = st.clone();
    let min_state = st.clone();
    let max_state = st.clone();
    let shiny_state = st.clone();
    let sort_state = st.clone();
    let clear_state = st.clone();
    let err_block = match &level_err {
        Some(msg) => {
            let m = msg.clone();
            rsx! { p { class: "storage-filter-error", role: "alert", "{m}" } }
        }
        None => VNode::empty(),
    };
    let select_btn = if select_mode {
        let exit_state = st.clone();
        rsx! {
            Button {
                class: ("select-mode-btn exit").to_string(),
                onclick: crate::button::click(move || exit_select_mode(&exit_state)),
                "取消选择"
                        }
        }
    } else if storage_count > 0 {
        let enter_state = st.clone();
        rsx! {
            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                class: ("select-mode-btn").to_string(),
                onclick: crate::button::click(move || enter_select_mode(&enter_state)),
                "多选"
                        } }, [("data-test-st-select-mode", ("1").to_string())]) }
        }
    } else {
        VNode::empty()
    };

    let bag_body = if loading {
        rsx! { div { class: "panel-loading", "加载中..." } }
    } else {
        rsx! {
            div { class: "pokemon-grid",
                for i in 0..bag_cells.len() {
                    ..vec![bag_cells[i].clone()]
                }
            }
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
                        div { class: "panel-body", ..vec![bag_body] }
                    }
                    div { class: "storage-panel storage-panel-bottom",
                        div { class: "panel-header",
                            div { class: "panel-header-left",
                                span { class: "panel-title", "仓库宠物" }
                                span { class: "panel-count", "{count_text}" }
                            }
                            ..vec![select_btn]
                        }
                        div { class: "storage-search-bar",
                            // 搜索逐字过滤（oninput，对齐 Dioxus 版）：事件带整字段值，
                            // 不再挂 change，失焦时不会重复触发一次。
                            input {
                                class: "storage-search-input",
                                r#type: "text",
                                placeholder: "搜索名字 / 昵称",
                                value: search_value,
                                "data-test-st-search": "1",
                                oninput: move |e: tairitsu_vdom::InputEvent| {
                                    set_filter(&search_state, "search", &e.value)
                                }
                            }
                            div { class: "storage-filter-fields",
                                label {
                                    "属性"
                                    select {
                                        value: type_value,
                                        "data-test-st-type": "1",
                                        onchange: move |e: tairitsu_vdom::ChangeEvent| {
                                            set_filter(&type_state, "type", &e.value)
                                        },
                                        option { value: "", "全部属性" }
                                        for i in 0..POKEMON_TYPES.len() {
                                            {
                                                let t = POKEMON_TYPES[i];
                                                let v = t;
                                                rsx! { option { value: v, "{t}" } }
                                            }
                                        }
                                    }
                                }
                                label {
                                    "最低等级"
                                    input {
                                        r#type: "number", min: "1", step: "1",
                                        placeholder: "不限",
                                        value: min_value,
                                        "data-test-st-min": "1",
                                        onchange: move |e: tairitsu_vdom::ChangeEvent| {
                                            set_filter(&min_state, "min", &e.value)
                                        }
                                    }
                                }
                                label {
                                    "最高等级"
                                    input {
                                        r#type: "number", min: "1", step: "1",
                                        placeholder: "不限",
                                        value: max_value,
                                        onchange: move |e: tairitsu_vdom::ChangeEvent| {
                                            set_filter(&max_state, "max", &e.value)
                                        }
                                    }
                                }
                                label {
                                    "闪光"
                                    select {
                                        value: shiny_value,
                                        onchange: move |e: tairitsu_vdom::ChangeEvent| {
                                            set_filter(&shiny_state, "shiny", &e.value)
                                        },
                                        option { value: "", "全部" }
                                        option { value: "shiny", "仅闪光" }
                                        option { value: "normal", "非闪光" }
                                    }
                                }
                                label {
                                    "排序"
                                    select {
                                        value: sort_value,
                                        "data-test-st-sort": "1",
                                        onchange: move |e: tairitsu_vdom::ChangeEvent| {
                                            set_filter(&sort_state, "sort", &e.value)
                                        },
                                        option { value: "oldest", "取得较早优先" }
                                        option { value: "newest", "最近取得优先" }
                                        option { value: "level_asc", "等级从低到高" }
                                        option { value: "level_desc", "等级从高到低" }
                                    }
                                }
                                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                                    class: ("select-action-btn").to_string(),
                                    onclick: crate::button::click(move || clear_filters(&clear_state)),
                                    "清除筛选"
                                                                } }, [("data-test-st-clear", ("1").to_string())]) }
                            }
                            ..vec![err_block]
                        }
                        ..vec![select_bar]
                        div { class: "panel-body", ..vec![storage_body] }
                    }
                }
            }
        }
    }
}

/// 单张宠物卡（背包/仓库通用）。操作行内联展开（见模块注释）。
fn slot_card(
    pm: &crate::state::PokemonBasic,
    in_storage: bool,
    _is_in_battle: bool,
    selected_ids: &[u64],
    select_mode: bool,
    st: &AppState,
) -> VNode {
    let display_name = pm.display_name();
    let alt_name = display_name.clone();
    let name_text = display_name.clone();
    let hp_pct = if pm.max_hp > 0 {
        ((pm.hp.max(0) as f64) / (pm.max_hp as f64) * 100.0) as u32
    } else {
        0
    };
    let hp_cls = hp_class_storage(pm.hp, pm.max_hp);
    let status = hp_health_status(pm.hp, pm.max_hp);
    let critical = status == "critical";
    let weak = is_weak_state(pm.state);
    let is_selected = selected_ids.contains(&pm.id);
    let menu_open = st.storage.open_menu.get() == Some(pm.id) && !select_mode;

    let slot_class = if critical {
        "pokemon-slot critical".to_string()
    } else if weak {
        "pokemon-slot weak".to_string()
    } else if is_selected {
        "pokemon-slot selected".to_string()
    } else {
        // 健康状态落在类上（`.pokemon-slot.healthy` 之类），底色由它给。
        format!("pokemon-slot {status}")
    };

    let id = pm.id;
    let sprite = format!("{IMG_PATH}/spm/{}.gif", pm.type_id);
    let title = format!("{display_name} · Lv.{}", pm.level);
    let level = format!("Lv.{}", pm.level);
    let hp_fill_style = if critical {
        String::new()
    } else {
        format!("width: {hp_pct}%")
    };
    let hp_text = if critical {
        rsx! { div { class: "critical-hp-text", "濒危" } }
    } else if weak {
        rsx! { div { class: "weak-hp-text", "虚弱" } }
    } else if hp_pct < 100 {
        // 旧版这里是直接的文本节点，不包 span。
        VNode::Text(tairitsu_vdom::VText::new(&format!(
            "{}/{}",
            pm.hp, pm.max_hp
        )))
    } else {
        VNode::empty()
    };

    let checkbox = if select_mode {
        let checked = if is_selected {
            "checkbox-icon checked"
        } else {
            "checkbox-icon"
        };
        rsx! {
            div { class: "pokemon-select-checkbox",
                span { class: checked, "✓" }
            }
        }
    } else {
        VNode::empty()
    };

    // 操作行（菜单展开时）：携带区 → 设为首位/放入仓库；仓库区 → 放入背包/放生。
    // 操作按钮在卡片内部，而卡片自己监听 click（展开/收起、勾选）：按钮的
    // 回调必须 stop_propagation，等价于委托时代「操作按钮先于卡片命中」。
    let menu = if menu_open {
        let bag_full = st
            .center
            .pokemons
            .get()
            .and_then(|r| r.ok())
            .map(|list| list.iter().filter(|p| p.site == 1 || p.site == 2).count() >= 6)
            .unwrap_or(false);
        let cannot_release = pm.state == 0 || pm.hp <= 0;
        let mut actions: Vec<VNode> = Vec::new();
        if in_storage {
            let bag_btn = if bag_full {
                rsx! {
                    Button { class: ("popup-menu-item disabled").to_string(), disabled: true, "背包已满（6/6）"                     }
                }
            } else {
                let to_bag_state = st.clone();
                let to_bag_key = id.to_string();
                rsx! {
                    { crate::button::hooked(tairitsu_macros::rsx! { Button {
                        class: ("popup-menu-item").to_string(),
                        onclick: Some(tairitsu_vdom::EventHandler::new(move |e: tairitsu_vdom::MouseEvent| {
                            e.stop_propagation();
                            move_to_site(&to_bag_state, id, 2, "放入背包");
                        })),
                        "放入背包"
                                        } }, [("data-test-st-to-bag", (to_bag_key).to_string())]) }
                }
            };
            actions.push(bag_btn);
            let release_btn = if cannot_release {
                rsx! {
                    Button { class: ("popup-menu-item danger disabled").to_string(), disabled: true, "放生"                     }
                }
            } else {
                let release_state = st.clone();
                let release_key = id.to_string();
                rsx! {
                    { crate::button::hooked(tairitsu_macros::rsx! { Button {
                        class: ("popup-menu-item danger").to_string(),
                        onclick: Some(tairitsu_vdom::EventHandler::new(move |e: tairitsu_vdom::MouseEvent| {
                            e.stop_propagation();
                            release_one(&release_state, id);
                        })),
                        "放生"
                                        } }, [("data-test-st-release", (release_key).to_string())]) }
                }
            };
            actions.push(release_btn);
        } else {
            if pm.site != 1 {
                let first_state = st.clone();
                let first_btn = rsx! {
                    Button {
                        class: ("popup-menu-item").to_string(),
                        onclick: Some(tairitsu_vdom::EventHandler::new(move |e: tairitsu_vdom::MouseEvent| {
                            e.stop_propagation();
                            set_first(&first_state, id);
                        })),
                        "设为首位"
                                        }
                };
                actions.push(first_btn);
            }
            let to_storage_state = st.clone();
            let store_btn = rsx! {
                Button {
                    class: ("popup-menu-item").to_string(),
                    onclick: Some(tairitsu_vdom::EventHandler::new(move |e: tairitsu_vdom::MouseEvent| {
                        e.stop_propagation();
                        move_to_site(&to_storage_state, id, 3, "放入仓库");
                    })),
                    "放入仓库"
                                }
            };
            actions.push(store_btn);
        }
        rsx! {
            div { class: "popup-menu slot-actions",
                for i in 0..actions.len() {
                    ..vec![actions[i].clone()]
                }
            }
        }
    } else {
        VNode::empty()
    };

    // 卡片自身的点击：多选模式切换勾选，否则展开/收起操作行。
    let card_state = st.clone();
    let card_key = id.to_string();
    rsx! {
        div { class: slot_class,
            "data-test-st-card": card_key,
            "data-pokemon-id": id.to_string(),
            "data-shiny": if pm.is_shiny { "true" } else { "false" },
            onclick: move |_: tairitsu_vdom::MouseEvent| click_card(&card_state, id),
            title: title,
            div { class: "pokemon-sprite",
                img { src: sprite, alt: alt_name }
            }
            ..vec![checkbox]
            div { class: "pokemon-slot-name",
                span { class: "pokemon-slot-name-text", "{name_text}" }
                if pm.is_shiny {
                    span { class: "pet-mark-compact", "★" }
                }
                if pm.is_legendary {
                    span { class: "pet-mark-compact", "◆" }
                }
            }
            div { class: "pokemon-level", "{level}" }
            div { class: "pokemon-hp-bar",
                if !critical {
                    // 高度和渐变在 `.hp-bar-fill` 上，`hp_cls` 只给颜色档。
                    div { class: "hp-bar-fill {hp_cls}", style: hp_fill_style }
                }
            }
            div { class: "pokemon-hp-text", ..vec![hp_text] }
            ..vec![menu]
        }
    }
}
