//! 冒险 + 战斗页（tairitsu 版，移植自 rust/game/src/pages/adventure.rs 与
//! battle.rs 的合并）。同名同类直用 game.css（.page-adventure/.page-battle/
//! .battle-scene-container/.skills-panel …）。
//!
//! 与 Dioxus 版的结构差异（行为对齐）：
//! - adventure 与 battle 合并成一个路由组件——Dioxus 里 Adventure 包着
//!   BattlePage，这里同样按「有场景则渲染战斗、否则渲染地图」分支。
//! - 弹窗态从 scene 派生（整树重挂不做边沿检测）：
//!   needs_replacement → 换宠弹窗；effective_battle_over → 结算弹窗。
//! - pending 动作只存内存（跨页刷新不保留；Dioxus 存 localStorage，
//!   随 wasi 存储接入再补）。
//! - 幂等 request_id 用时间戳+计数器（见 api::battle_request_id）。
//! - 战报只展示文本行（分享/复制依赖剪贴板桥，后续补）。
//! - 品质门槛不做客户端预判（list 载荷无个体值；服务端仍会拦截），
//!   地图卡仍显示门槛要求文案。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use hikari_components::{Toast, ToastPosition, ToastProps, ToastVariant};

use crate::app_state::AppState;
use crate::battle_types::{BattleStatus, MapInfo};
use crate::state::{PendingBattleAction, ToastKind};

const IMG_PATH: &str = "source/plugin/pokemon/images";
const IMG_PATH_REMOTE: &str = "https://img.tsdm39.com/Pokemon";

// ---------------------------------------------------------------------------
// 装载与动作（delegation.rs 调用）
// ---------------------------------------------------------------------------

/// 首次进入：恢复战斗 + 拉地图（战斗中不拉也行，但拉了切换更快）。
pub fn ensure_loaded(state: &AppState) {
    if state.battle.scene.get().is_none() && !state.battle.loading.get() {
        recover(state);
    }
    if state.battle.maps.get().is_none() {
        load_maps(state);
    }
}

fn recover(state: &AppState) {
    let bt = state.battle.clone();
    // loading 立即置位：render_route 每次重挂都会调 ensure_loaded，
    // 场景信号要等 fetch 回调才写——没有守卫会无限重复发起。
    bt.loading.inner().set(true);
    crate::api::recover_battle(Box::new(move |result| {
        match result {
            Ok(scene) => {
                let has_scene = scene.is_some();
                bt.scene.inner().set(Some(scene));
                bt.conn_error.inner().set(None);
                if has_scene {
                    refresh_battle_inventory(&bt);
                }
            }
            Err(e) => {
                bt.scene.inner().set(Some(None));
                bt.conn_error
                    .inner()
                    .set(Some(format!("战斗同步失败：{e}")));
            }
        }
        // 恢复结束必须复位守卫——否则 loading 永真，一切动作被互斥挡住。
        bt.loading.inner().set(false);
        crate::request_remount();
    }));
    let st = state.clone();
    crate::api::pokemon_list(Box::new(move |result| {
        st.center.pokemons.inner().set(Some(result));
        crate::request_remount();
    }));
}

const MAPS_LOADING: &str = "__maps_loading__";

fn load_maps(state: &AppState) {
    let bt = state.battle.clone();
    // 同步写哨兵：fetch 回调前重挂不再重复发起（否则与 recover 同型活锁）。
    bt.maps.inner().set(Some(Err(MAPS_LOADING.to_string())));
    crate::api::get_maps(Box::new(move |result| {
        bt.maps.inner().set(Some(result));
        crate::request_remount();
    }));
}

fn refresh_battle_inventory(bt: &crate::state::BattleState) {
    let bt2 = bt.clone();
    crate::api::get_battle_items(Box::new(move |result| {
        if let Ok(items) = result {
            bt2.battle_items.inner().set(items);
            crate::request_remount();
        }
    }));
    let bt3 = bt.clone();
    crate::api::get_battle_balls(Box::new(move |result| {
        if let Ok(balls) = result {
            bt3.battle_balls.inner().set(balls);
            crate::request_remount();
        }
    }));
}

/// 动作入口：互斥守卫 + 乐观锁快照 + pending 记录（失败可重试）。
pub fn begin(state: &AppState, action: &str, fields: serde_json::Value) {
    let bt = state.battle.clone();
    if bt.loading.get() || bt.pending.get().is_some() || bt.conn_error.get().is_some() {
        return;
    }
    let scene = bt.scene.get().flatten();
    if action != "start" && scene.is_none() {
        return;
    }
    let (engine_id, revision) = scene
        .as_ref()
        .map(|s| (s.engine_battle_id, s.revision))
        .unwrap_or((0, 0));
    let pending = PendingBattleAction {
        action: action.to_string(),
        engine_battle_id: engine_id,
        expected_revision: revision,
        fields,
    };
    submit(state, pending, false);
}

/// 重试上次未确认的动作。
pub fn retry(state: &AppState) {
    let bt = state.battle.clone();
    if let Some(pending) = bt.pending.get() {
        submit(state, pending, true);
    }
}

/// 重新同步战斗（清除连接错误）。
pub fn reconnect(state: &AppState) {
    let bt = state.battle.clone();
    bt.conn_error.inner().set(None);
    bt.scene.inner().set(None);
    crate::request_remount();
    recover(state);
}

fn submit(state: &AppState, pending: PendingBattleAction, is_retry: bool) {
    let bt = state.battle.clone();
    bt.loading.inner().set(true);
    bt.pending.inner().set(Some(pending.clone()));
    crate::request_remount();
    let body = crate::api::battle_action_body(
        pending.engine_battle_id,
        pending.expected_revision,
        pending.fields.clone(),
    );
    let st = state.clone();
    crate::api::perform_battle_action(
        &pending.action,
        body,
        Box::new(move |result| {
            let bt = st.battle.clone();
            match result {
                Ok(crate::api::BattleMutation::Scene(scene)) => {
                    bt.pending.inner().set(None);
                    bt.conn_error.inner().set(None);
                    bt.selection.inner().set(None);
                    bt.last_map_id.inner().set(scene.map_id);
                    bt.scene.inner().set(Some(Some(scene)));
                    // 战斗会改宠物 HP/PP 与道具数量。
                    refresh_battle_inventory(&bt);
                    let st2 = st.clone();
                    crate::api::pokemon_list(Box::new(move |r| {
                        st2.center.pokemons.inner().set(Some(r));
                        crate::request_remount();
                    }));
                }
                Ok(crate::api::BattleMutation::Selection(selection)) => {
                    bt.pending.inner().set(None);
                    bt.conn_error.inner().set(None);
                    bt.selection.inner().set(Some(selection));
                }
                Err(e) => {
                    // 网络类失败保留 pending 供重试；确定失败直接清。
                    if !e.contains("网络") {
                        bt.pending.inner().set(None);
                    }
                    bt.conn_error.inner().set(Some(e));
                }
            }
            let _ = is_retry;
            bt.loading.inner().set(false);
            crate::request_remount();
        }),
    );
}

/// 结束战斗返回地图：清场景并全量刷新（列表/资料/背包）。
pub fn end_battle(state: &AppState) {
    let bt = state.battle.clone();
    bt.scene.inner().set(Some(None));
    bt.selection.inner().set(None);
    bt.show_switch_modal.inner().set(false);
    crate::request_remount();
    let st = state.clone();
    crate::api::pokemon_list(Box::new(move |r| {
        st.center.pokemons.inner().set(Some(r));
        crate::request_remount();
    }));
    crate::api::inventory(None, 1, "", {
        let st = state.clone();
        Box::new(move |r| {
            if let Ok(list) = r {
                st.inv.items.inner().set(Some(Ok(list.items)));
                crate::request_remount();
            }
        })
    });
}

/// 结算「继续战斗」：先治疗全部受伤宠物再开同图。
pub fn continue_battle(state: &AppState) {
    let bt = state.battle.clone();
    let map_id = bt.last_map_id.get();
    if map_id == 0 || bt.loading.get() {
        return;
    }
    let injured: Vec<u64> = state
        .center
        .pokemons
        .get()
        .and_then(|r| r.ok())
        .map(|list| {
            list.iter()
                .filter(|p| (p.site == 1 || p.site == 2) && p.needs_healing())
                .map(|p| p.id)
                .collect()
        })
        .unwrap_or_default();
    bt.loading.inner().set(true);
    crate::request_remount();
    heal_then_start(state, injured, 0, map_id);
}

fn heal_then_start(state: &AppState, ids: Vec<u64>, idx: usize, map_id: u64) {
    if idx >= ids.len() {
        let bt = state.battle.clone();
        bt.loading.inner().set(false);
        crate::request_remount();
        begin(state, "start", serde_json::json!({ "map_id": map_id }));
        return;
    }
    let st = state.clone();
    crate::api::heal_pokemon(
        ids[idx],
        false,
        Box::new(move |result| {
            if result.is_err() {
                let bt = st.battle.clone();
                bt.loading.inner().set(false);
                crate::state::battle_show_toast_silent(
                    &bt,
                    ToastKind::Error,
                    "治疗结果尚未确认，请检查宠物状态后再继续。".to_string(),
                );
                crate::request_remount();
                return;
            }
            let st2 = st.clone();
            crate::api::pokemon_list(Box::new(move |r| {
                st2.center.pokemons.inner().set(Some(r));
                crate::request_remount();
            }));
            heal_then_start(&st, ids, idx + 1, map_id);
        }),
    );
}

// ---- 战斗动作入口（delegation 直调） ----

pub fn use_skill(state: &AppState, skill_id: u64) {
    begin(state, "turn", serde_json::json!({ "skill_id": skill_id }));
}

pub fn normal_attack(state: &AppState) {
    begin(state, "turn", serde_json::json!({ "skill_id": 0 }));
}

pub fn use_item_in_battle(state: &AppState, item_id: u64) {
    begin(state, "use_item", serde_json::json!({ "item_id": item_id }));
}

pub fn capture(state: &AppState, ball_id: u64) {
    begin(state, "capture", serde_json::json!({ "ball_id": ball_id }));
}

pub fn flee(state: &AppState) {
    begin(state, "flee", serde_json::json!({}));
}

pub fn switch_pokemon(state: &AppState) {
    state.battle.show_switch_modal.inner().set(false);
    crate::request_remount();
    begin(state, "switch_pokemon", serde_json::json!({}));
}

pub fn replace_pokemon(state: &AppState, pokemon_id: u64) {
    begin(
        state,
        "replace_pokemon",
        serde_json::json!({ "pokemon_id": pokemon_id }),
    );
}

pub fn use_item_on_skill(state: &AppState, item_id: u64, skill_record_id: u64) {
    begin(
        state,
        "use_item_on_skill",
        serde_json::json!({ "item_id": item_id, "skill_record_id": skill_record_id }),
    );
}

pub fn start_adventure(state: &AppState, map_id: u64) {
    state.battle.selected_map.inner().set(None);
    state.battle.last_map_id.inner().set(map_id);
    crate::request_remount();
    begin(state, "start", serde_json::json!({ "map_id": map_id }));
}

pub fn start_boss(state: &AppState, map_id: u64, boss_type_id: u64, boss_index: Option<u64>) {
    state.battle.selected_map.inner().set(None);
    state.battle.last_map_id.inner().set(map_id);
    crate::request_remount();
    let mut fields = serde_json::json!({
        "map_id": map_id,
        "boss_pokemon_type_id": boss_type_id,
    });
    if let (Some(obj), Some(idx)) = (fields.as_object_mut(), boss_index) {
        obj.insert("boss_index".to_string(), serde_json::json!(idx));
    }
    begin(state, "start", fields);
}

// ---- 弹窗/标签页状态 ----

pub fn open_map_detail(state: &AppState, map_id: u64) {
    state.battle.selected_map.inner().set(Some(map_id));
    crate::request_remount();
}

pub fn close_map_detail(state: &AppState) {
    state.battle.selected_map.inner().set(None);
    crate::request_remount();
}

pub fn select_region(state: &AppState, region: Option<String>) {
    state.battle.selected_region.inner().set(region);
    crate::request_remount();
}

pub fn switch_tab(state: &AppState, tab: u8) {
    let bt = state.battle.clone();
    let reload = tab == 1 || tab == 2;
    bt.tab.inner().set(tab);
    crate::request_remount();
    if reload {
        refresh_battle_inventory(&bt);
    }
}

pub fn open_switch_modal(state: &AppState) {
    state.battle.show_switch_modal.inner().set(true);
    crate::request_remount();
}

pub fn close_switch_modal(state: &AppState) {
    state.battle.show_switch_modal.inner().set(false);
    crate::request_remount();
}

pub fn cancel_selection(state: &AppState) {
    state.battle.selection.inner().set(None);
    crate::request_remount();
}

pub fn open_log(state: &AppState) {
    let bt = state.battle.clone();
    let battle_id = bt
        .scene
        .get()
        .flatten()
        .map(|s| s.engine_battle_id)
        .unwrap_or(0);
    if battle_id == 0 {
        return;
    }
    bt.show_log.inner().set(true);
    bt.log_lines.inner().set(Vec::new());
    crate::request_remount();
    crate::api::get_battle_log(
        battle_id,
        Box::new(move |result| {
            bt.log_lines
                .inner()
                .set(result.unwrap_or_else(|e| vec![format!("战报加载失败：{e}")]));
            crate::request_remount();
        }),
    );
}

pub fn close_log(state: &AppState) {
    state.battle.show_log.inner().set(false);
    crate::request_remount();
}

// ---------------------------------------------------------------------------
// 组件
// ---------------------------------------------------------------------------

fn gender_symbol(gender: u8) -> &'static str {
    match gender {
        0 => "♂",
        1 => "♀",
        _ => "",
    }
}

fn hp_pct_class(hp: i64, max_hp: i64) -> (u32, &'static str) {
    if max_hp <= 0 || hp <= 0 {
        return (0, "hp-bar-fg hp-low");
    }
    let pct = ((hp as f64 / max_hp as f64) * 100.0) as u32;
    let cls = if pct > 50 {
        "hp-bar-fg hp-high"
    } else if pct > 20 {
        "hp-bar-fg hp-mid"
    } else {
        "hp-bar-fg hp-low"
    };
    (pct, cls)
}

fn recommendation(map: &MapInfo, first_level: Option<u64>) -> String {
    match first_level {
        None => "请先获得一只宝可梦".to_string(),
        Some(level) => {
            if level < map.min_level {
                format!("等级不足，建议等级 Lv.{} 以上", map.min_level)
            } else if level > map.max_level + 10 {
                "等级过高，经验收益较低".to_string()
            } else {
                "等级适中，推荐挑战".to_string()
            }
        }
    }
}

fn difficulty_class(map: &MapInfo, first_level: Option<u64>) -> &'static str {
    match first_level {
        None => "difficulty-unknown",
        Some(level) => {
            if level < map.min_level {
                "difficulty-hard"
            } else if level > map.max_level + 10 {
                "difficulty-easy"
            } else if level >= map.min_level && level <= map.max_level {
                "difficulty-medium"
            } else {
                "difficulty-normal"
            }
        }
    }
}

#[component]
pub fn Adventure(state: Box<AppState>) -> VNode {
    let st = state;
    let bt = st.battle.clone();

    let maps_state = bt.maps.get();
    let scene = bt.scene.get().flatten();
    let loading = bt.loading.get();
    let conn_error = bt.conn_error.get();

    let list = st
        .center
        .pokemons
        .get()
        .and_then(|r| r.ok())
        .unwrap_or_default();
    let first_level = list.first().map(|p| p.level);
    let pokemon_empty = list.is_empty();

    let toast = if bt.toast_visible.get() {
        let toast_state = bt.clone();
        let variant = match bt.toast_kind.get() {
            ToastKind::Success => ToastVariant::Success,
            ToastKind::Error => ToastVariant::Error,
        };
        let message = bt.toast_message.get();
        rsx! {
            Toast {
                variant: variant,
                message: message,
                position: ToastPosition::BottomRight,
                on_close: Some(tairitsu_vdom::Callback::new(move |_| {
                    toast_state.toast_visible.set(false)
                })),
            }
        }
    } else {
        VNode::empty()
    };

    // 连接错误弹窗（上次动作结果未确认）。
    let conn_modal = match conn_error {
        Some(err) => {
            let has_pending = bt.pending.get().is_some();
            let hint = if has_pending {
                "将确认刚才那次操作，已执行的操作不会重复扣除道具或推进回合。"
            } else {
                ""
            };
            let btn_label = if has_pending {
                "重试并确认结果"
            } else {
                "重新同步战斗"
            };
            rsx! {
                div { class: "modal-overlay",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "确认战斗状态" }
                        }
                        div { class: "modal-body battle-connection",
                            p { role: "alert", "{err}" }
                            p { "{hint}" }
                            button {
                                class: "btn btn-primary",
                                disabled: loading,
                                "data-pm-bt-conn-action": "1",
                                "{btn_label}"
                            }
                        }
                    }
                }
            }
        }
        None => VNode::empty(),
    };

    let body = match &scene {
        Some(scene) if !scene.event_scene_hidden() => {
            battle_scene(&st, scene, loading, pokemon_empty)
        }
        _ => adventure_map_view(&st, &maps_state, first_level, pokemon_empty, loading),
    };

    rsx! {
        div { class: "page-adventure",
            ..vec![body, conn_modal, toast]
        }
    }
}

/// 占位谓词：场景渲染分支判断（保留给未来的 phase 过滤）。
trait SceneExt {
    fn event_scene_hidden(&self) -> bool;
}

impl SceneExt for crate::battle_types::BattleScene {
    fn event_scene_hidden(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// 冒险地图（世界 → 区域 → 详情弹窗）
// ---------------------------------------------------------------------------

fn adventure_map_view(
    st: &AppState,
    maps_state: &Option<Result<Vec<MapInfo>, String>>,
    first_level: Option<u64>,
    pokemon_empty: bool,
    loading: bool,
) -> VNode {
    let bt = st.battle.clone();
    let maps_err = maps_state.as_ref().and_then(|r| r.as_ref().err()).cloned();
    let maps_loading = maps_state.is_none() || maps_err.as_deref() == Some(MAPS_LOADING);
    let maps = maps_state
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();
    let maps_err = if maps_err.as_deref() == Some(MAPS_LOADING) {
        None
    } else {
        maps_err
    };

    // 详情弹窗。
    let selected_map = bt
        .selected_map
        .get()
        .and_then(|id| maps.iter().find(|m| m.id == id).cloned());
    let detail_modal = match selected_map {
        Some(map) => {
            let title = format!("{} - 详细信息", map.name);
            let area_name2 = map.area_type_name.clone();
            let icon = map.area_icon();
            let area_name = map.area_type_name.clone();
            let color_cls = format!("badge-area-{}", map.area_color());
            let level_text = map.level_range_text();
            let mode_label = map.mode_label();
            let rec = recommendation(&map, first_level);
            let has_wild = map.has_wild();
            let min_quality = map.min_quality;
            let quality_name = map.min_quality_name.clone();
            let quality_row = if min_quality > 0 {
                let badge_cls = format!("quality-badge quality-rank-{min_quality}");
                rsx! {
                    div { class: "map-detail-section quality-gate-section",
                        h4 { "品质门槛" }
                        p { class: "quality-gate-text",
                            span { class: badge_cls, "{quality_name}" }
                            " 出战宠物品质需达到该档位（由六项个体值总和判定）"
                        }
                    }
                }
            } else {
                VNode::empty()
            };
            let wild_tags: Vec<VNode> = map
                .wild_pokemons
                .iter()
                .map(|p| {
                    let name = p.name.clone();
                    rsx! { span { class: "pokemon-tag", "{name}" } }
                })
                .collect();
            let wild_section = if !wild_tags.is_empty() && has_wild {
                rsx! {
                    div { class: "map-detail-section",
                        h4 { "可能出现" }
                        div { class: "pokemon-list",
                            for i in 0..wild_tags.len() {
                                ..vec![wild_tags[i].clone()]
                            }
                        }
                    }
                }
            } else {
                VNode::empty()
            };
            let mut boss_rows: Vec<VNode> = Vec::new();
            for boss in &map.bosses {
                let sprite = format!("{IMG_PATH_REMOTE}/pm/{}.gif", boss.pokemon_type_id);
                let name = boss.pokemon_name.clone();
                let alt_name = name.clone();
                let detail = format!("Lv.{} · 倍率 x{}", boss.level, boss.boss_multiplier);
                let attr = format!(
                    "{}|{}|{}",
                    map.id,
                    boss.pokemon_type_id,
                    boss.boss_index.unwrap_or(0)
                );
                boss_rows.push(rsx! {
                    button {
                        class: "boss-row-btn",
                        disabled: loading || pokemon_empty,
                        "data-pm-bt-boss": attr,
                        div { class: "boss-row-content",
                            img { class: "boss-row-avatar", src: sprite, alt: alt_name }
                            div { class: "boss-row-info",
                                span { class: "boss-row-name", "{name}" }
                                span { class: "boss-row-details", "{detail}" }
                            }
                            span { class: "boss-row-action", "挑战" }
                        }
                    }
                });
            }
            let boss_section = if !boss_rows.is_empty() {
                rsx! {
                    div { class: "map-detail-section boss-challenge-section",
                        h4 { "Boss 挑战" }
                        div { class: "boss-list-simple",
                            for i in 0..boss_rows.len() {
                                ..vec![boss_rows[i].clone()]
                            }
                        }
                    }
                }
            } else {
                VNode::empty()
            };
            let start_btn = if has_wild {
                let attr = map.id.to_string();
                rsx! {
                    button {
                        class: "modal-action-btn",
                        disabled: loading || pokemon_empty,
                        "data-pm-bt-start": attr,
                        "开始冒险"
                    }
                }
            } else {
                VNode::empty()
            };
            rsx! {
                div { class: "modal-overlay", "data-pm-bt-map-close": "1",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{title}" }
                            button { class: "modal-close", "data-pm-bt-map-close": "1", "×" }
                        }
                        div { class: "modal-body modal-map-details",
                            div { class: "map-detail-header",
                                span { class: "map-icon", "{icon}" }
                                h2 { "{map.name}" }
                                span { class: color_cls, "{area_name}" }
                            }
                            div { class: "map-detail-section",
                                h4 { "挑战建议" }
                                p { class: "recommendation-text", "{rec}" }
                            }
                            ..vec![quality_row]
                            div { class: "map-detail-section",
                                h4 { "基本信息" }
                                div { class: "detail-grid",
                                    div { class: "detail-item",
                                        span { class: "detail-label", "地形类型" }
                                        span { class: "detail-value", "{area_name2}" }
                                    }
                                    div { class: "detail-item",
                                        span { class: "detail-label", "地图模式" }
                                        span { class: "detail-value", "{mode_label}" }
                                    }
                                    if has_wild {
                                        div { class: "detail-item",
                                            span { class: "detail-label", "等级范围" }
                                            span { class: "detail-value", "{level_text}" }
                                        }
                                    }
                                }
                            }
                            ..vec![wild_section, boss_section, start_btn]
                        }
                    }
                }
            }
        }
        None => VNode::empty(),
    };

    let empty_warning = if pokemon_empty {
        rsx! {
            div { class: "warning-box-map", "你还没有宝可梦！请先去商店购买或捕捉一只宝可梦。" }
        }
    } else {
        VNode::empty()
    };

    let view = if maps_loading {
        rsx! { div { class: "loading", "加载地图数据..." } }
    } else if let Some(err) = maps_err {
        let msg = format!("加载地图失败：{err}");
        rsx! { div { class: "error-message", "{msg}" } }
    } else {
        match bt.selected_region.get() {
            Some(region_id) => {
                let regions = crate::battle_types::regions_from_maps(&maps);
                let region = regions.iter().find(|r| r.id == region_id);
                match region {
                    Some(region) => {
                        let region_name = region.id.clone();
                        let count_text = format!("共 {} 个冒险地点", region.maps.len());
                        let x = region.x;
                        let y = region.y;
                        let bg_style = format!("background-position: {x}% {y}%;");
                        let mut cards: Vec<VNode> = Vec::new();
                        for map in &region.maps {
                            let id_attr = map.id.to_string();
                            let id_attr_btn = map.id.to_string();
                            let icon = map.area_icon();
                            let name = map.name.clone();
                            let area_name = map.area_type_name.clone();
                            let color_cls = format!("badge-area-{}", map.area_color());
                            let rec = recommendation(map, first_level);
                            let diff = difficulty_class(map, first_level);
                            let card_cls = format!("zoomed-map-card {diff}");
                            let level_text = map.level_range_text();
                            let has_wild = map.has_wild();
                            let boss_count = map.bosses.len();
                            let info_line = if has_wild {
                                format!("等级：{level_text}")
                            } else {
                                format!("Boss 数量：{boss_count}")
                            };
                            let quality_line = if map.min_quality > 0 {
                                format!("品质：{}+", map.min_quality_name)
                            } else {
                                String::new()
                            };
                            let mut boss_chips: Vec<VNode> = map
                                .bosses
                                .iter()
                                .take(3)
                                .map(|boss| {
                                    let attr = format!(
                                        "{}|{}|{}",
                                        map.id,
                                        boss.pokemon_type_id,
                                        boss.boss_index.unwrap_or(0)
                                    );
                                    let sprite = format!("{IMG_PATH_REMOTE}/pm/{}.gif", boss.pokemon_type_id);
                                    let name = boss.pokemon_name.clone();
                                    let alt_name = name.clone();
                                    let level = format!("Lv.{}", boss.level);
                                    rsx! {
                                        div { class: "card-boss-item",
                                            "data-pm-bt-boss": attr,
                                            img { class: "card-boss-avatar", src: sprite, alt: alt_name }
                                            span { class: "card-boss-name", "{name}" }
                                            span { class: "card-boss-level", "{level}" }
                                        }
                                    }
                                })
                                .collect();
                            let boss_block = if map.bosses.is_empty() {
                                VNode::empty()
                            } else {
                                if map.bosses.len() > 3 {
                                    let more = format!("...等 {} 个 Boss", map.bosses.len());
                                    boss_chips.push(rsx! {
                                        div { class: "card-boss-more", "{more}" }
                                    });
                                }
                                rsx! {
                                    div { class: "card-boss-list",
                                        div { class: "card-boss-title", "Boss:" }
                                        for i in 0..boss_chips.len() {
                                            ..vec![boss_chips[i].clone()]
                                        }
                                    }
                                }
                            };
                            cards.push(rsx! {
                                div { class: card_cls,
                                    "data-pm-bt-map-open": id_attr,
                                    div { class: "zoomed-map-header",
                                        span { class: "map-icon", "{icon}" }
                                        h4 { "{name}" }
                                        span { class: color_cls, "{area_name}" }
                                    }
                                    p { class: "map-desc", "{rec}" }
                                    div { class: "zoomed-map-info",
                                        div { class: "info-row",
                                            if !quality_line.is_empty() {
                                                span { class: "map-quality-requirement", "{quality_line}" }
                                            }
                                            span { "{info_line}" }
                                        }
                                    }
                                    ..vec![boss_block]
                                    button {
                                        class: "zoomed-start-btn",
                                        disabled: loading || pokemon_empty,
                                        "data-pm-bt-map-open": id_attr_btn,
                                        "查看详情"
                                    }
                                }
                            });
                        }
                        let zoomed_warning = if pokemon_empty {
                            rsx! {
                                div { class: "warning-box-zoomed", "你还没有宝可梦！请先去商店购买或捕捉一只宝可梦。" }
                            }
                        } else {
                            VNode::empty()
                        };
                        rsx! {
                            div { class: "hoenn-map-zoomed",
                                button {
                                    class: "back-to-world-btn",
                                    "data-pm-bt-region-back": "1",
                                    "← 返回世界地图"
                                }
                                div { class: "zoomed-region-title",
                                    h2 { "{region_name}" }
                                    p { "{count_text}" }
                                }
                                div { class: "zoomed-map-bg", style: bg_style }
                                div { class: "zoomed-map-list",
                                    for i in 0..cards.len() {
                                        ..vec![cards[i].clone()]
                                    }
                                }
                                ..vec![zoomed_warning]
                            }
                        }
                    }
                    None => rsx! { div { "区域未找到" } },
                }
            }
            None => {
                let regions = crate::battle_types::regions_from_maps(&maps);
                let mut markers: Vec<VNode> = Vec::new();
                for region in &regions {
                    let count_text = format!("{}个地点", region.maps.len());
                    let style = format!("left: {}%; top: {}%;", region.x, region.y);
                    let name = region.id.clone();
                    let region_attr = region.id.clone();
                    markers.push(rsx! {
                        div {
                            class: "map-marker",
                            style: style,
                            "data-pm-bt-region": region_attr,
                            div { class: "marker-dot" }
                            div { class: "marker-info",
                                div { class: "marker-label", "{name}" }
                                div { class: "marker-count", "{count_text}" }
                            }
                        }
                    });
                }
                rsx! {
                    div { class: "hoenn-map-world",
                        div { class: "map-decoration",
                            div { class: "map-land-mass" }
                            div { class: "map-water-area" }
                        }
                        for i in 0..markers.len() {
                            ..vec![markers[i].clone()]
                        }
                        ..vec![empty_warning]
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "adventure-map-wrapper",
            ..vec![view, detail_modal]
        }
    }
}

// ---------------------------------------------------------------------------
// 战斗场景（核心循环 UI）
// ---------------------------------------------------------------------------

fn battle_scene(
    st: &AppState,
    scene: &crate::battle_types::BattleScene,
    loading: bool,
    pokemon_empty: bool,
) -> VNode {
    let bt = st.battle.clone();
    let tab = bt.tab.get();
    let battle_over = scene.effective_battle_over();
    let needs_replacement = scene.needs_replacement();
    let turn = scene.turn.to_string();

    // —— 两侧精灵 ——
    let wild = &scene.wild_pokemon;
    let (wild_pct, wild_cls) = hp_pct_class(wild.hp, wild.max_hp);
    let wild_style = format!("width: {wild_pct}%;");
    let wild_hp_text = format!("{}/{}", wild.hp, wild.max_hp);
    let wild_sprite = format!("{IMG_PATH_REMOTE}/pm/{}.gif", wild.id);
    let wild_name = wild.name.clone();
    let wild_alt = wild_name.clone();
    let wild_level = format!("Lv.{}", wild.level);
    let wild_gender = gender_symbol(wild.gender);
    let wild_shiny = wild.is_shiny;

    let my = &scene.my_pokemon;
    let (my_pct, my_cls) = hp_pct_class(my.hp, my.max_hp);
    let my_style = format!("width: {my_pct}%;");
    let my_hp_text = format!("{}/{}", my.hp, my.max_hp);
    let my_sprite = format!("{IMG_PATH_REMOTE}/back/{}.gif", my.id);
    let my_name = my.name.clone();
    let my_alt = my_name.clone();
    let my_level = format!("Lv.{}", my.level);
    let my_shiny = my.is_shiny;

    // —— 标签页 ——
    let skills_cls = if tab == 0 {
        "battle-tab active"
    } else {
        "battle-tab"
    };
    let items_cls = if tab == 1 {
        "battle-tab active"
    } else {
        "battle-tab"
    };
    let capture_cls = if tab == 2 {
        "battle-tab active"
    } else {
        "battle-tab"
    };
    let flee_cls = if tab == 3 {
        "battle-tab active"
    } else {
        "battle-tab"
    };

    let tab_content = match tab {
        0 => {
            let mut skill_btns: Vec<VNode> = Vec::new();
            for skill in &my.skills {
                let sid = skill.id.to_string();
                let disabled = loading || skill.pp == 0;
                let cls = if disabled {
                    "skill-btn disabled"
                } else {
                    "skill-btn"
                };
                let name = skill.name.clone();
                let type_badge = format!("badge badge-type-{}", skill.skill_type);
                let cat_badge = format!("badge badge-cat-{}", skill.category);
                let power_label = match skill.category.as_str() {
                    "特攻" | "特殊" => "特攻",
                    "物攻" | "物理" => "物攻",
                    _ => "威力",
                };
                let power_text = format!("{}：{}", power_label, skill.power);
                let pp_text = format!("PP：{}/{}", skill.pp, skill.max_pp);
                let pp_cls = if skill.pp == 0 {
                    "skill-pp low"
                } else if skill.max_pp > 0 && skill.pp <= skill.max_pp / 3 {
                    "skill-pp warn"
                } else {
                    "skill-pp"
                };
                let stype = skill.skill_type.clone();
                let scat = skill.category.clone();
                skill_btns.push(rsx! {
                    button {
                        class: cls,
                        r#type: "button",
                        disabled: disabled,
                        "data-pm-bt-skill": sid,
                        div { class: "skill-btn-content",
                            div { class: "skill-name", "{name}" }
                            span { class: type_badge, "{stype}" }
                            span { class: cat_badge, "{scat}" }
                            div { class: "skill-stats-row",
                                span { class: "skill-power", "{power_text}" }
                                span { class: pp_cls, "{pp_text}" }
                            }
                        }
                    }
                });
            }
            for _ in my.skills.len()..4 {
                skill_btns.push(rsx! {
                    div { class: "empty-skill-slot", span { class: "empty-slot-text", "空" } }
                });
            }
            rsx! {
                div { class: "skills-panel",
                    div { class: "attack-button-section",
                        button {
                            class: if loading { "normal-attack-btn disabled" } else { "normal-attack-btn" },
                            r#type: "button",
                            disabled: loading,
                            "data-pm-bt-attack": "1",
                            span { class: "attack-btn-label", "普通攻击" }
                        }
                    }
                    div { class: "skills-grid-section",
                        div { class: "skills-grid",
                            for i in 0..skill_btns.len() {
                                ..vec![skill_btns[i].clone()]
                            }
                        }
                    }
                }
            }
        }
        1 => {
            let items = bt.battle_items.get();
            if items.is_empty() {
                rsx! { div { class: "items-panel", div { class: "empty-items-hint", "暂无道具" } } }
            } else {
                let mut cards: Vec<VNode> = Vec::new();
                for item in items.iter() {
                    let id_attr = item.id.to_string();
                    let usable = item.can_use_on(my);
                    let disabled = loading || !usable;
                    let cls = if disabled {
                        "battle-item-card disabled"
                    } else {
                        "battle-item-card"
                    };
                    let img_src = format!("{IMG_PATH}/item/{}.gif", item.img);
                    let name = item.name.clone();
                    let alt_name = name.clone();
                    let qty = format!("×{}", item.nums);
                    let label = item.type_label();
                    let desc = item.display_description();
                    let condition = if my.hp <= 0 {
                        "宠物已倒下，请先替换宠物。".to_string()
                    } else if item.is_pp_restore() {
                        if usable {
                            "选择一个PP未满的技能，恢复后对方会反击。".to_string()
                        } else {
                            "所有技能PP已满，暂时不需要恢复。".to_string()
                        }
                    } else if usable {
                        "恢复当前宠物的HP，使用后对方会反击。".to_string()
                    } else {
                        "当前HP已满，暂时不需要恢复。".to_string()
                    };
                    cards.push(rsx! {
                        button {
                            class: cls,
                            r#type: "button",
                            disabled: disabled,
                            "data-pm-bt-item": id_attr,
                            title: condition,
                            div { class: "battle-item-icon",
                                img { src: img_src, alt: alt_name }
                            }
                            div { class: "battle-item-info",
                                div { class: "battle-item-name", "{name}" }
                                div { class: "battle-item-desc", "{desc}" }
                                span { class: "battle-item-type", "{label}" }
                            }
                            span { class: "battle-item-qty", "{qty}" }
                        }
                    });
                }
                rsx! {
                    div { class: "items-panel",
                        div { class: "battle-items-grid",
                            for i in 0..cards.len() {
                                ..vec![cards[i].clone()]
                            }
                        }
                    }
                }
            }
        }
        2 => {
            let balls = bt.battle_balls.get();
            if balls.is_empty() {
                rsx! { div { class: "capture-panel", div { class: "empty-items-hint", "没有精灵球" } } }
            } else {
                let mut cards: Vec<VNode> = Vec::new();
                for ball in balls.iter() {
                    let id_attr = ball.type_id.to_string();
                    let disabled = loading || ball.quantity <= 0;
                    let cls = if disabled {
                        "battle-item-card disabled"
                    } else {
                        "battle-item-card"
                    };
                    let img_src = format!("{IMG_PATH}/item/{}.gif", ball.image);
                    let name = ball.name.clone();
                    let alt_name = name.clone();
                    let qty = format!("×{}", ball.quantity);
                    let desc = if ball.quantity <= 0 {
                        "数量不足".to_string()
                    } else {
                        "投掷精灵球尝试捕捉野生的宝可梦。".to_string()
                    };
                    cards.push(rsx! {
                        button {
                            class: cls,
                            r#type: "button",
                            disabled: disabled,
                            "data-pm-bt-ball": id_attr,
                            div { class: "battle-item-icon",
                                img { src: img_src, alt: alt_name }
                            }
                            div { class: "battle-item-info",
                                div { class: "battle-item-name", "{name}" }
                                div { class: "battle-item-desc", "{desc}" }
                            }
                            span { class: "battle-item-qty", "{qty}" }
                        }
                    });
                }
                rsx! {
                    div { class: "capture-panel",
                        div { class: "battle-items-grid",
                            for i in 0..cards.len() {
                                ..vec![cards[i].clone()]
                            }
                        }
                    }
                }
            }
        }
        _ => {
            rsx! {
                div { class: "flee-panel",
                    p { class: "flee-hint", "确定要离开这场战斗吗？离开后将无法获得本场奖励。" }
                    button {
                        class: "btn btn-warning",
                        r#type: "button",
                        disabled: loading,
                        "data-pm-bt-flee": "1",
                        "退出战斗"
                    }
                }
            }
        }
    };

    // —— 结算 / 换宠 / 技能选择 / 战报 弹窗 ——
    let modal = if battle_over {
        result_modal(st, scene, loading)
    } else if needs_replacement {
        replace_modal(st, scene, loading)
    } else if bt.show_switch_modal.get() {
        switch_modal(st, loading)
    } else {
        VNode::empty()
    };

    let selection_modal = match bt.selection.get() {
        Some(sel) => {
            let title = format!("选择技能 - {}", sel.item_name);
            let mut rows: Vec<VNode> = Vec::new();
            for skill in &sel.available_skills {
                let attr = format!("{}|{}", sel.item_id, skill.id);
                let name = skill.name.clone();
                let pp_text = format!("PP: {}/{}", skill.current_pp, skill.max_pp);
                let pp_cls = if skill.current_pp == 0 {
                    "skill-selection-pp low"
                } else if skill.max_pp > 0 && skill.current_pp <= skill.max_pp / 3 {
                    "skill-selection-pp warn"
                } else {
                    "skill-selection-pp"
                };
                rows.push(rsx! {
                    button {
                        class: "skill-selection-item",
                        r#type: "button",
                        disabled: loading,
                        "data-pm-bt-select-skill": attr,
                        div { class: "skill-selection-info",
                            span { class: "skill-selection-name", "{name}" }
                            span { class: pp_cls, "{pp_text}" }
                        }
                        span { class: "skill-selection-action", "选择" }
                    }
                });
            }
            rsx! {
                div { class: "modal-overlay", "data-pm-bt-select-cancel": "1",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{title}" }
                            button { class: "modal-close", "data-pm-bt-select-cancel": "1", "×" }
                        }
                        div { class: "modal-body skill-selection-modal",
                            p { class: "skill-selection-hint", "请选择要恢复PP的技能：" }
                            div { class: "skill-selection-list",
                                for i in 0..rows.len() {
                                    ..vec![rows[i].clone()]
                                }
                            }
                        }
                    }
                }
            }
        }
        None => VNode::empty(),
    };

    let log_modal = if bt.show_log.get() {
        let lines = bt.log_lines.get();
        let body = if lines.is_empty() {
            rsx! { p { "正在加载战报..." } }
        } else {
            let mut rows: Vec<VNode> = lines
                .iter()
                .map(|l| {
                    let text = l.clone();
                    rsx! { p { "{text}" } }
                })
                .collect();
            rsx! {
                for i in 0..rows.len() {
                    ..vec![rows[i].clone()]
                }
            }
        };
        rsx! {
            div { class: "modal-overlay", "data-pm-bt-log-close": "1",
                div { class: "modal-content",
                    div { class: "modal-header",
                        h3 { "战报" }
                        button { class: "modal-close", "data-pm-bt-log-close": "1", "×" }
                    }
                    div { class: "modal-body battle-log-body", ..vec![body] }
                }
            }
        }
    } else {
        VNode::empty()
    };

    let _ = pokemon_empty;

    rsx! {
        div { class: "page-battle",
            div { class: "battle-tools",
                button {
                    class: "btn btn-secondary",
                    disabled: scene.engine_battle_id == 0,
                    "data-pm-bt-log-open": "1",
                    "查看战报"
                }
            }
            div { class: "battle-card",
                div { class: "battle-card-body",
                    div { class: "battle-content-area",
                        div { class: "battle-tab-menu",
                            button { class: skills_cls, "data-pm-bt-tab": "0", "技能" }
                            button { class: items_cls, "data-pm-bt-tab": "1", "道具" }
                            button { class: capture_cls, "data-pm-bt-tab": "2", "捕捉" }
                            button { class: flee_cls, "data-pm-bt-tab": "3", "退出战斗" }
                        }
                        div { class: "battle-divider" }
                        div { class: "battle-main-content",
                            div { class: "battle-scene-container",
                                div { class: "battle-scene-upper",
                                    div { class: "battle-field",
                                        div { class: "enemy-pokemon-block",
                                            div { class: "enemy-sprite-container",
                                                img { class: "enemy-sprite", src: wild_sprite, alt: wild_alt }
                                            }
                                            div { class: "enemy-info-box",
                                                div { class: "enemy-header",
                                                    div { class: "enemy-name-row",
                                                        span { class: "enemy-name", "{wild_name}" }
                                                        if wild_shiny {
                                                            span { class: "pet-mark", "★" }
                                                        }
                                                        span { class: "enemy-gender", "{wild_gender}" }
                                                    }
                                                    span { class: "enemy-level-text", "{wild_level}" }
                                                }
                                                div { class: "enemy-hp-row",
                                                    div { class: "hp-bar-bg",
                                                        div { class: wild_cls, style: wild_style }
                                                    }
                                                    span { class: "hp-text", "{wild_hp_text}" }
                                                }
                                            }
                                        }
                                        div { class: "player-pokemon-block",
                                            "data-pm-bt-switch-open": "1",
                                            div { class: "player-info-box",
                                                div { class: "player-header",
                                                    div { class: "player-name-row",
                                                        span { class: "player-name", "{my_name}" }
                                                        if my_shiny {
                                                            span { class: "pet-mark", "★" }
                                                        }
                                                        span { class: "player-level", "{my_level}" }
                                                    }
                                                }
                                                div { class: "player-hp-row",
                                                    div { class: "hp-bar-bg",
                                                        div { class: my_cls, style: my_style }
                                                    }
                                                    span { class: "hp-text", "{my_hp_text}" }
                                                }
                                            }
                                            div { class: "player-sprite-container",
                                                img { class: "player-sprite", src: my_sprite, alt: my_alt }
                                            }
                                        }
                                    }
                                    div { class: "battle-turn-display",
                                        span { class: "turn-label", "回合" }
                                        span { class: "turn-number", "{turn}" }
                                    }
                                }
                            }
                            div { class: "battle-tab-content", ..vec![tab_content] }
                        }
                    }
                }
            }
            ..vec![modal, selection_modal, log_modal]
        }
    }
}

/// 结算弹窗（胜利/失败/逃脱/捕捉 + 奖励 + 结束/继续）。
fn result_modal(st: &AppState, scene: &crate::battle_types::BattleScene, loading: bool) -> VNode {
    let bt = st.battle.clone();
    let title = scene.status.result_title();
    let result_cls = match scene.status {
        BattleStatus::Victory | BattleStatus::Captured => "battle-result victory",
        BattleStatus::Defeat => "battle-result defeat",
        _ => "battle-result fled",
    };
    let my_name = scene.my_pokemon.name.clone();

    let rewards_block = match &scene.rewards {
        Some(r) => {
            let exp_text = format!("{} 经验", r.exp);
            let money_text = format!("{} 金币", r.money);
            let today_row = if r.money_daily_limit > 0 {
                let text = format!(
                    "今日已获得 {} / {} 宠物币",
                    r.money_today, r.money_daily_limit
                );
                rsx! { p { class: "reward-money-today", "{text}" } }
            } else {
                VNode::empty()
            };
            let notes: Vec<VNode> = r
                .money_notes
                .iter()
                .map(|n| {
                    let text = n.clone();
                    rsx! { p { class: "reward-money-note", "{text}" } }
                })
                .collect();
            rsx! {
                div { class: "battle-rewards",
                    h4 { "获得奖励" }
                    div { class: "reward-items",
                        div { class: "reward-item", span { class: "reward-value", "{exp_text}" } }
                        div { class: "reward-item", span { class: "reward-value", "{money_text}" } }
                    }
                    ..vec![today_row]
                    for i in 0..notes.len() {
                        ..vec![notes[i].clone()]
                    }
                }
            }
        }
        None => VNode::empty(),
    };

    let level_up_block = match &scene.level_up {
        Some(lu) if lu.level_up => {
            let text = format!(
                "{my_name} 升级了！Lv.{} → Lv.{}",
                lu.old_level, lu.new_level
            );
            rsx! {
                div { class: "level-up-notice",
                    span { class: "level-up-text", "{text}" }
                }
            }
        }
        _ => VNode::empty(),
    };

    // 胜利且宠物存活时可继续（对齐 Dioxus 版 can_continue 判定）。
    let can_continue = scene.status == BattleStatus::Victory && scene.my_pokemon.hp > 0;
    let continue_btn = if can_continue {
        rsx! {
            button {
                class: "battle-continue-btn",
                disabled: loading,
                "data-pm-bt-continue": "1",
                "继续战斗"
            }
        }
    } else {
        VNode::empty()
    };
    let _ = bt;

    rsx! {
        div { class: "modal-overlay",
            div { class: "modal-content",
                div { class: "modal-header", h3 { "{title}" } }
                div { class: "modal-body battle-result-section",
                    div { class: result_cls,
                        h2 { "{title}" }
                        ..vec![rewards_block, level_up_block]
                    }
                    div { class: "battle-end-buttons",
                        button { class: "battle-end-btn", "data-pm-bt-log-open": "1", "查看战报" }
                        button {
                            class: "battle-end-btn",
                            disabled: loading,
                            "data-pm-bt-end": "1",
                            "返回冒险地图"
                        }
                        ..vec![continue_btn]
                    }
                }
            }
        }
    }
}

/// 我方倒下但有替补：选择替换宠物（或无惩罚逃跑）。
fn replace_modal(st: &AppState, scene: &crate::battle_types::BattleScene, loading: bool) -> VNode {
    let fainted_name = scene.my_pokemon.name.clone();
    let message = format!("{fainted_name} 倒下了！请选择一只宠物继续战斗：");
    let list = st
        .center
        .pokemons
        .get()
        .and_then(|r| r.ok())
        .unwrap_or_default();
    let fainted_id = scene.my_pokemon.instance_id;

    let mut rows: Vec<VNode> = Vec::new();
    for pm in list.iter().filter(|p| p.site == 1 || p.site == 2) {
        let is_the_fainted = pm.id == fainted_id;
        let is_fainted = is_the_fainted || pm.hp <= 0;
        let attr = pm.id.to_string();
        let cls = if is_fainted {
            "replacement-pokemon-card disabled"
        } else {
            "replacement-pokemon-card"
        };
        let sprite = format!("{IMG_PATH}/spm/{}.gif", pm.type_id);
        let name = pm.display_name();
        let alt_name = name.clone();
        let level = format!("Lv.{}", pm.level);
        let hp_text = format!(
            "HP: {}/{}",
            if is_the_fainted { 0 } else { pm.hp },
            pm.max_hp
        );
        let first_badge = if pm.site == 1 {
            rsx! { span { class: "first-badge", "首位" } }
        } else {
            VNode::empty()
        };
        let fainted_badge = if is_the_fainted {
            rsx! { span { class: "fainted-badge", "已倒下" } }
        } else {
            VNode::empty()
        };
        rows.push(rsx! {
            button {
                class: cls,
                r#type: "button",
                disabled: loading || is_fainted,
                "data-pm-bt-replace": attr,
                img { class: "replacement-pokemon-sprite", src: sprite, alt: alt_name }
                div { class: "replacement-pokemon-info",
                    div { class: "replacement-pokemon-name",
                        "{name}"
                        ..vec![first_badge, fainted_badge]
                    }
                    div { class: "replacement-pokemon-level", "{level}" }
                }
                div { class: "replacement-pokemon-hp",
                    span { class: "hp-text", "{hp_text}" }
                }
            }
        });
    }

    rsx! {
        div { class: "modal-overlay",
            div { class: "modal-content",
                div { class: "modal-header", h3 { "选择替换宠物" } }
                div { class: "modal-body faint-replace-modal",
                    p { class: "faint-message", "{message}" }
                    div { class: "replacement-pokemon-list",
                        for i in 0..rows.len() {
                            ..vec![rows[i].clone()]
                        }
                    }
                    div { class: "faint-replace-actions",
                        button {
                            class: "btn btn-warning",
                            disabled: loading,
                            "data-pm-bt-flee": "1",
                            "逃跑（无惩罚）"
                        }
                    }
                }
            }
        }
    }
}

/// 手动换宠确认。
fn switch_modal(st: &AppState, loading: bool) -> VNode {
    let _ = st;
    rsx! {
        div { class: "modal-overlay", "data-pm-bt-switch-cancel": "1",
            div { class: "modal-content",
                div { class: "modal-header",
                    h3 { "切换上场宠物" }
                    button { class: "modal-close", "data-pm-bt-switch-cancel": "1", "×" }
                }
                div { class: "modal-body switch-pokemon-modal",
                    p { class: "switch-pokemon-warning", "切换宠物可能会让当前上场宠物再受到一次攻击！" }
                    p { class: "switch-pokemon-hint", "确定要切换吗？（将轮换背包中的下一只宠物）" }
                    div { class: "switch-pokemon-actions",
                        button { class: "btn btn-secondary", "data-pm-bt-switch-cancel": "1", "取消" }
                        button {
                            class: "btn btn-primary",
                            disabled: loading,
                            "data-pm-bt-switch-confirm": "1",
                            "确定切换"
                        }
                    }
                }
            }
        }
    }
}
