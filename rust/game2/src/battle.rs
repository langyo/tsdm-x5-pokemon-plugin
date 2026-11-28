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

use hikari_components::{Button, ButtonProps};
use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use crate::app_state::AppState;
use crate::battle_types::{
    difficulty_class, quality_gate_notice, recommendation, BattleStatus, MapInfo,
};
use crate::icon::{Icon, IconName, IconProps};
use crate::state::{ItemCardKey, PendingBattleAction, PokemonBasic, ToastKind};
use crate::toast;

const IMG_PATH: &str = "source/plugin/pokemon/images";
const IMG_PATH_REMOTE: &str = "https://img.tsdm39.com/Pokemon";

// ---------------------------------------------------------------------------
// 装载与动作（页面各元素的原生 onclick 调用）
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
                // 战斗前的自动治疗不看 PP（旧版 `get_injured_pokemons`）。
                .filter(|p| (p.site == 1 || p.site == 2) && p.is_injured())
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
                toast::show(
                    ToastKind::Error,
                    "治疗结果尚未确认，请检查宠物状态后再继续。",
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

// ---- 战斗动作入口（按钮原生 onclick 直调） ----

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

/// 切换战斗页标签。对齐 Dioxus 版：切到道具 / 捕捉时拉一次清单（标签变化的
/// use_effect）；已经停在这两个标签上再点一次也重拉（`!loading` 时）。卡片上的
/// 悬浮提示与详情弹窗是组件内状态，换标签即卸载，所以一并收起。
pub fn switch_tab(state: &AppState, tab: u8) {
    let bt = state.battle.clone();
    let changed = bt.tab.get() != tab;
    let reload = (tab == 1 || tab == 2) && (changed || !bt.loading.get());
    bt.tab.inner().set(tab);
    bt.item_hover.inner().set(None);
    bt.item_details.inner().set(None);
    crate::request_remount();
    if reload {
        refresh_battle_inventory(&bt);
    }
}

/// 用户此刻是否在战斗里（旧版 `USER_STATE.is_in_battle()`，取资料的 `npcid > 0`，
/// 战斗一结束服务端就清零）。这边从场景派生：有场景且战斗没结束。只看「有没有
/// 场景」的话，结算弹窗开着的时候侧栏迷你格还顶着「战斗中」的角标。
pub fn user_in_battle(state: &AppState) -> bool {
    matches!(state.battle.scene.get(), Some(Some(ref scene)) if !scene.effective_battle_over())
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
    bt.log_data.inner().set(None);
    bt.log_copy_status.inner().set(String::new());
    crate::request_remount();
    crate::api::get_battle_log(
        battle_id,
        Box::new(move |result| {
            bt.log_data.inner().set(Some(result));
            crate::request_remount();
        }),
    );
}

pub fn close_log(state: &AppState) {
    state.battle.show_log.inner().set(false);
    crate::request_remount();
}

/// 战报弹窗的「重新载入」（旧版 `report.restart()`）：清掉复制提示，重拉战报。
pub fn reload_log(state: &AppState) {
    open_log(state);
}

// ---------------------------------------------------------------------------
// 道具 / 精灵球卡（旧版 BattleItemCard 的组件内状态）
// ---------------------------------------------------------------------------

/// 指针进入卡片：记下光标位置（提示框落点用）并显示。
fn item_card_enter(state: &AppState, key: ItemCardKey, client_x: i32, client_y: i32) {
    let bt = state.battle.clone();
    bt.item_hover_pos
        .inner()
        .set((client_x as f64, client_y as f64));
    bt.item_hover.inner().set(Some(key));
    crate::request_remount();
}

/// 指针离开卡片：收起提示框。
fn item_card_leave(state: &AppState, key: ItemCardKey) {
    let bt = state.battle.clone();
    if bt.item_hover.get() == Some(key) {
        bt.item_hover.inner().set(None);
        crate::request_remount();
    }
}

/// 点卡片：开详情弹窗，收起提示框。
fn open_item_details(state: &AppState, key: ItemCardKey) {
    let bt = state.battle.clone();
    if bt.loading.get() {
        return;
    }
    bt.item_hover.inner().set(None);
    bt.item_details.inner().set(Some(key));
    crate::request_remount();
}

fn close_item_details(state: &AppState) {
    state.battle.item_details.inner().set(None);
    crate::request_remount();
}

/// 详情弹窗里的「使用道具 / 使用精灵球」：关弹窗，再按卡的类型派发。
/// `disabled` 是旧版那一条：卡当前用不了 / 动作进行中 / 数量为 0。
fn use_item_card(state: &AppState, is_ball: bool, use_id: u64, disabled: bool) {
    if disabled {
        return;
    }
    state.battle.item_details.inner().set(None);
    if is_ball {
        capture(state, use_id);
    } else {
        use_item_in_battle(state, use_id);
    }
}

fn show_player_tooltip(state: &AppState) {
    state.battle.player_hover.inner().set(true);
    crate::request_remount();
}

fn hide_player_tooltip(state: &AppState) {
    state.battle.player_hover.inner().set(false);
    crate::request_remount();
}

/// 复制战报 BBCode 到剪贴板（对齐 Dioxus 版 log.rs 的「复制战报」）。
pub fn copy_log(state: &AppState) {
    let bbcode = state
        .battle
        .log_data
        .get()
        .and_then(|r| r.ok())
        .map(|d| d.bbcode)
        .unwrap_or_default();
    if bbcode.is_empty() {
        return;
    }
    // tairitsu 的 platform-helpers 提供同步剪贴板写入；失败时提示手动选取。
    let ok = tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::platform_helpers::copy_to_clipboard(&bbcode);
    state.battle.log_copy_status.inner().set(
        if ok {
            "已复制战报，可贴到论坛。"
        } else {
            "无法自动复制，请选取上方文字后复制。"
        }
        .to_string(),
    );
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

/// 性别类名后缀（旧版 `gender_class`；未知为空）。
fn gender_class(gender: u8) -> &'static str {
    match gender {
        0 => "male",
        1 => "female",
        _ => "",
    }
}

// ---------------------------------------------------------------------------
// 道具 / 精灵球卡（旧版 BattleItemCard）
// ---------------------------------------------------------------------------

/// 一张道具 / 精灵球卡的显示数据（旧版 `battle_item_for_card` 之后的 `InventoryItem`
/// 加上 `BattleItemCard` 的几个 prop）。
struct ItemCard {
    key: ItemCardKey,
    /// 使用键：道具传记录 id，精灵球传种类 id（旧版 `on_click` 的两种实参）。
    use_id: u64,
    name: String,
    description: String,
    image_src: String,
    qty: i64,
    type_name: String,
    /// 当前用不了（详情里的「使用」禁用；卡面变灰但照样能点开看详情）。
    disabled: bool,
    /// 动作进行中（卡面按钮本身禁用）。
    busy: bool,
    condition: String,
}

/// 物品图地址：带扩展名的原样用，否则补 `.gif`（旧版 card / tooltip 同一规则）。
fn item_image_src(image: &str) -> String {
    if image.contains('.') {
        format!("{IMG_PATH}/item/{image}")
    } else {
        format!("{IMG_PATH}/item/{image}.gif")
    }
}

/// 道具（`is_ball = false`）/ 精灵球（`is_ball = true`）页签的卡片数据。
fn item_cards(
    st: &AppState,
    my: &crate::battle_types::BattlePokemon,
    loading: bool,
    is_ball: bool,
) -> Vec<ItemCard> {
    let bt = st.battle.clone();
    if is_ball {
        bt.battle_balls
            .get()
            .into_iter()
            .enumerate()
            .map(|(idx, ball)| ItemCard {
                key: (true, idx),
                use_id: ball.type_id,
                name: ball.name,
                description: ball.description,
                image_src: item_image_src(&ball.image),
                qty: ball.quantity,
                type_name: ball.type_name,
                disabled: loading,
                busy: loading,
                condition: "用于捕捉当前野生宠物；捕捉失败时对方可能反击。".to_string(),
            })
            .collect()
    } else {
        bt.battle_items
            .get()
            .into_iter()
            .enumerate()
            .map(|(idx, item)| {
                let usable = item.can_use_on(my);
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
                ItemCard {
                    key: (false, idx),
                    use_id: item.id,
                    name: item.name.clone(),
                    description: item.display_description(),
                    image_src: item_image_src(&item.img),
                    qty: item.nums,
                    type_name: item.type_label().to_string(),
                    disabled: loading || !usable,
                    busy: loading,
                    condition,
                }
            })
            .collect()
    }
}

/// 按标识找回一张卡（清单刷新后下标可能失效，找不到就不画）。
fn find_card(
    st: &AppState,
    my: &crate::battle_types::BattlePokemon,
    loading: bool,
    key: ItemCardKey,
) -> Option<ItemCard> {
    item_cards(st, my, loading, key.0).into_iter().nth(key.1)
}

/// 道具 / 精灵球卡：48px 图标格 + 数量角标。悬浮出提示、点击开详情弹窗——两者
/// 渲染在页面层（[`battle_item_tooltip`] / [`battle_item_details_modal`]）。
///
/// 有意是原生 `button` 而不是 hikari `Button`：卡片要 `mouseenter` / `mouseleave`
/// （hikari `Button` 只接 `onclick`），与迷你格的格子同一处理。
fn battle_item_card(st: &AppState, card: &ItemCard) -> VNode {
    let card_cls = if card.disabled {
        "battle-item-card disabled"
    } else if card.key.0 {
        "battle-item-card capture-ball-card"
    } else {
        "battle-item-card"
    };
    let alt = card.name.clone();
    let image_src = card.image_src.clone();
    let aria_label = format!("查看 {}", card.name);
    let aria_disabled = if card.busy { "true" } else { "false" };
    let hook = if card.key.0 {
        format!("ball-{}", card.use_id)
    } else {
        format!("item-{}", card.use_id)
    };
    let qty = card.qty.to_string();
    let busy = card.busy;
    let key = card.key;
    let click_state = st.clone();
    let enter_state = st.clone();
    let leave_state = st.clone();
    rsx! {
        div { class: "battle-item-card-wrapper",
            button {
                r#type: "button",
                class: card_cls,
                "aria-label": aria_label,
                "aria-disabled": aria_disabled,
                "data-test-bt-item-card": hook,
                disabled: busy,
                onclick: move |_| open_item_details(&click_state, key),
                onmouseenter: move |e: tairitsu_vdom::MouseEvent| {
                    item_card_enter(&enter_state, key, e.client_x, e.client_y)
                },
                onmouseleave: move |_| item_card_leave(&leave_state, key),
                div { class: "battle-item-icon",
                    img { src: image_src, alt: alt }
                }
                span { class: "battle-item-qty", "{qty}" }
            }
        }
    }
}

/// 卡片的悬浮提示（旧版 BattleItemTooltip）：落在光标下方 30px、水平居中。
fn battle_item_tooltip(card: &ItemCard, pos: (f64, f64)) -> VNode {
    const TOOLTIP_WIDTH: f64 = 180.0;
    let tooltip_left = pos.0 - TOOLTIP_WIDTH / 2.0;
    let tooltip_top = pos.1 + 30.0;
    let style = format!("left: {tooltip_left}px; top: {tooltip_top}px;");
    let alt = card.name.clone();
    let image_src = card.image_src.clone();
    let name = card.name.clone();
    let description = card.description.clone();
    let show_desc = !description.is_empty();
    let type_line = format!("类型: {}", card.type_name);
    let qty_line = format!("数量: {}", card.qty);
    rsx! {
        div { class: "battle-item-tooltip", style: style,
            div { class: "tooltip-header",
                img { class: "tooltip-icon", src: image_src, alt: alt }
                span { class: "tooltip-name", "{name}" }
            }
            if show_desc {
                div { class: "tooltip-desc", "{description}" }
            }
            div { class: "tooltip-info",
                div { class: "tooltip-type", "{type_line}" }
                div { class: "tooltip-qty", "{qty_line}" }
            }
        }
    }
}

/// 点开卡片后的详情弹窗（旧版 BattleItemCard 里内嵌的 Modal）：效果 / 持有数量 /
/// 适用条件 + 「使用道具」或「使用精灵球」。
fn battle_item_details_modal(st: &AppState, card: &ItemCard) -> VNode {
    let close_state = st.clone();
    let use_state = st.clone();
    let effect = format!("效果：{}", card.description);
    let owned = format!("持有数量：{}", card.qty);
    let condition = format!("适用条件：{}", card.condition);
    let use_label = if card.key.0 {
        "使用精灵球"
    } else {
        "使用道具"
    };
    let use_disabled = card.disabled || card.busy || card.qty <= 0;
    // 使用键与种类随卡走：闭包要 'static，拷一份。
    let use_id = card.use_id;
    let is_ball = card.key.0;
    let body: Vec<VNode> = vec![rsx! {
        div { class: "battle-item-details",
            p { "{effect}" }
            p { "{owned}" }
            p { "{condition}" }
            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                class: ("btn btn-primary").to_string(),
                disabled: use_disabled,
                onclick: crate::button::click(move || {
                    use_item_card(&use_state, is_ball, use_id, use_disabled)
                }),
                "{use_label}"
                                            } }, [("data-test-bt-item-use", ("1").to_string())]) }
        }
    }];
    crate::modal::modal_with(
        crate::modal::ModalSpec::new(&card.name, false),
        move || close_item_details(&close_state),
        body,
    )
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
    let first_pm = list.first();
    let pokemon_empty = list.is_empty();

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
            let conn_state = st.clone();
            // 外壳是共用的 hikari Modal（见 crate::modal）。旧版这一框照样带关闭键，
            // 只是遮罩不接点击、关闭回调是空的（只能由正文里的按钮离开）。旧正文
            // 的类 `battle-connection` 落在 body_class 上，子节点与旧版相同。
            let body: Vec<VNode> = vec![
                rsx! { p { role: "alert", "{err}" } },
                rsx! { p { "{hint}" } },
                rsx! {
                    Button {
                        class: ("btn btn-primary").to_string(),
                        disabled: loading,
                        onclick: crate::button::click(move || {
                            // 有无未确认的动作在点击时判断（与原委托分支一致）。
                            if conn_state.battle.pending.get().is_some() {
                                retry(&conn_state);
                            } else {
                                reconnect(&conn_state);
                            }
                        }),
                        "{btn_label}"
                                        }
                },
            ];
            crate::modal::modal_with(
                crate::modal::ModalSpec {
                    body_class: "battle-connection",
                    ..crate::modal::ModalSpec::new("确认战斗状态", false)
                },
                || {},
                body,
            )
        }
        None => VNode::empty(),
    };

    let body = match &scene {
        Some(scene) if !scene.event_scene_hidden() => {
            battle_scene(&st, scene, loading, pokemon_empty)
        }
        _ => adventure_map_view(&st, &maps_state, first_pm, pokemon_empty, loading),
    };

    rsx! {
        div { class: "page-adventure",
            ..vec![body, conn_modal]
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
    first_pm: Option<&PokemonBasic>,
    pokemon_empty: bool,
    loading: bool,
) -> VNode {
    let bt = st.battle.clone();
    // 对齐 Dioxus 版 actions_blocked：加载中 / 有未确认动作 / 连接异常都禁用。
    let actions_blocked = loading || bt.pending.get().is_some() || bt.conn_error.get().is_some();
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
            let color_cls = format!("area-type-badge badge-area-{}", map.area_color());
            let level_text = map.level_range_text();
            let mode_label = map.mode_label();
            let rec = recommendation(&map, first_pm);
            let quality_blocked = quality_gate_notice(&map, first_pm).is_some();
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
                            " 出战宠物品质需达到 {quality_name}（由六项个体值总和判定）"
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
                let boss_map_id = map.id;
                let boss_type_id = boss.pokemon_type_id;
                let boss_index = boss.boss_index.unwrap_or(0);
                let boss_state = st.clone();
                boss_rows.push(rsx! {
                    Button {
                        class: ("boss-row-btn").to_string(),
                        disabled: actions_blocked || pokemon_empty || quality_blocked,
                        onclick: crate::button::click(move || {
                            start_boss(&boss_state, boss_map_id, boss_type_id, Some(boss_index))
                        }),
                        div { class: "boss-row-content",
                            img { class: "boss-row-avatar", src: sprite, alt: alt_name }
                            div { class: "boss-row-info",
                                span { class: "boss-row-name", "{name}" }
                                span { class: "boss-row-details", "{detail}" }
                            }
                            span { class: "boss-row-action",
                                {Icon(IconProps {
                                    name: Some(IconName::Swords),
                                    size: Some(12),
                                    class: Some("lucide-inline".to_string()),
                                })}
                                "挑战"
                            }
                        }
                                        }
                });
            }
            let boss_section = if !boss_rows.is_empty() {
                rsx! {
                    div { class: "map-detail-section boss-challenge-section",
                        h4 {
                            {Icon(IconProps {
                                name: Some(IconName::Crown),
                                size: Some(14),
                                class: Some("lucide-inline".to_string()),
                            })}
                            "Boss 挑战"
                        }
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
                let start_map_id = map.id;
                let start_state = st.clone();
                rsx! {
                    Button {
                        class: ("modal-action-btn").to_string(),
                        disabled: actions_blocked || pokemon_empty || quality_blocked,
                        onclick: crate::button::click(move || start_adventure(&start_state, start_map_id)),
                        "开始冒险"
                                        }
                }
            } else {
                VNode::empty()
            };
            let map_overlay_state = st.clone();
            // 外壳是共用的 hikari Modal（见 crate::modal）：点弹窗内部不关（hikari
            // 只把遮罩上的点击当关闭请求）。旧正文的类 `modal-map-details` 落在
            // body_class 上，子节点与旧版相同、样式不动。
            let body: Vec<VNode> = vec![
                rsx! {
                    div { class: "map-detail-header",
                        span { class: "map-icon", "{icon}" }
                        h2 { "{map.name}" }
                        span { class: color_cls, "{area_name}" }
                    }
                },
                rsx! {
                    div { class: "map-detail-section",
                        h4 { "挑战建议" }
                        p { class: "recommendation-text", "{rec}" }
                    }
                },
                quality_row,
                rsx! {
                    div { class: "map-detail-section",
                        h4 { "基本信息" }
                        div { class: "detail-grid",
                            if has_wild {
                                div { class: "detail-item",
                                    span { class: "detail-label", "等级范围" }
                                    span { class: "detail-value", "{level_text}" }
                                }
                            }
                            div { class: "detail-item",
                                span { class: "detail-label", "地形类型" }
                                span { class: "detail-value", "{area_name2}" }
                            }
                            div { class: "detail-item",
                                span { class: "detail-label", "地图模式" }
                                span { class: "detail-value", "{mode_label}" }
                            }
                        }
                    }
                },
                wild_section,
                boss_section,
                start_btn,
            ];
            crate::modal::modal_with(
                crate::modal::ModalSpec {
                    body_class: "modal-map-details",
                    ..crate::modal::ModalSpec::new(&title, false)
                },
                move || close_map_detail(&map_overlay_state),
                body,
            )
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
                        let region_name = region.name.clone();
                        let count_text = format!("共 {} 个冒险地点", region.maps.len());
                        let x = region.x;
                        let y = region.y;
                        let bg_style = format!("background-position: {x}% {y}%;");
                        let mut cards: Vec<VNode> = Vec::new();
                        for map in &region.maps {
                            let map_id = map.id;
                            let icon = map.area_icon();
                            let name = map.name.clone();
                            let area_name = map.area_type_name.clone();
                            let color_cls =
                                format!("area-type-badge badge-area-{}", map.area_color());
                            let rec = recommendation(map, first_pm);
                            let diff = difficulty_class(map, first_pm);
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
                                    let sprite = format!("{IMG_PATH_REMOTE}/pm/{}.gif", boss.pokemon_type_id);
                                    let name = boss.pokemon_name.clone();
                                    let alt_name = name.clone();
                                    let level = format!("Lv.{}", boss.level);
                                    let chip_map_id = map.id;
                                    let chip_type_id = boss.pokemon_type_id;
                                    let chip_index = boss.boss_index.unwrap_or(0);
                                    let chip_state = st.clone();
                                    rsx! {
                                        div { class: "card-boss-item",
                                            // 对齐 Dioxus 版：点 Boss 直接开战，拦住冒泡，
                                            // 免得再触发地图卡自身的「打开详情」。
                                            onclick: move |e: tairitsu_vdom::MouseEvent| {
                                                e.stop_propagation();
                                                start_boss(&chip_state, chip_map_id, chip_type_id, Some(chip_index));
                                            },
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
                            let card_state = st.clone();
                            let detail_btn_state = st.clone();
                            cards.push(rsx! {
                                div { class: card_cls,
                                    "data-test-bt-map-open": map_id.to_string(),
                                    onclick: move |_| open_map_detail(&card_state, map_id),
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
                                    Button {
                                        class: ("zoomed-start-btn").to_string(),
                                        disabled: loading || pokemon_empty,
                                        // 卡片自身也开详情：按钮上拦住冒泡，一次点击只打开一次（对齐 Dioxus 版）。
                                        onclick: Some(tairitsu_vdom::EventHandler::new(move |e: tairitsu_vdom::MouseEvent| {
                                            e.stop_propagation();
                                            open_map_detail(&detail_btn_state, map_id);
                                        })),
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
                        let back_state = st.clone();
                        rsx! {
                            div { class: "hoenn-map-zoomed",
                                Button {
                                    class: ("back-to-world-btn").to_string(),
                                    onclick: crate::button::click(move || select_region(&back_state, None)),
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
                    // 标签是显示名（「山脉地区」），不是 id（「山脉」）——旧版用 region.name。
                    let name = region.name.clone();
                    let region_id = region.id.clone();
                    let marker_state = st.clone();
                    markers.push(rsx! {
                        div {
                            class: "map-marker",
                            style: style,
                            "data-test-bt-region": name.clone(),
                            onclick: move |_| select_region(&marker_state, Some(region_id.clone())),
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
    let switch_open_state = st.clone();
    // 我方出场位的悬浮详情（旧版 MyPokemonSection 的 show_tooltip）。
    let hover_enter_state = st.clone();
    let hover_leave_state = st.clone();
    let player_hover = bt.player_hover.get();

    // —— 两侧精灵 ——
    let wild = &scene.wild_pokemon;
    let (wild_pct, wild_cls) = hp_pct_class(wild.hp, wild.max_hp);
    let wild_style = format!("width: {wild_pct}%;");
    let wild_hp_text = format!("{}/{}", wild.hp, wild.max_hp);
    let wild_sprite = crate::state::front_sprite_url(wild.id, wild.is_shiny);
    let wild_name = wild.name.clone();
    let wild_alt = wild_name.clone();
    let wild_level = format!("Lv.{}", wild.level);
    let wild_gender = gender_symbol(wild.gender);
    let wild_gender_cls = format!("enemy-gender {}", gender_class(wild.gender));
    let wild_shiny = wild.is_shiny;

    let my = &scene.my_pokemon;
    let (my_pct, my_cls) = hp_pct_class(my.hp, my.max_hp);
    let my_style = format!("width: {my_pct}%;");
    let my_hp_text = format!("{}/{}", my.hp, my.max_hp);
    // 我方出场位是背面图（目录 `pmb` / `pmsgb`，与正面图不同）。
    let my_sprite = crate::state::back_sprite_url(my.id, my.is_shiny);
    let my_name = my.name.clone();
    let my_alt = my_name.clone();
    let my_level = format!("Lv.{}", my.level);
    let my_shiny = my.is_shiny;
    let tooltip_level = format!("Lv.{}", my.level);
    let tooltip_hp = format!("{}/{}", my.hp, my.max_hp);
    let tooltip_name = my_name.clone();
    let tooltip_skills: Vec<VNode> = my
        .skills
        .iter()
        .map(|skill| {
            let name = skill.name.clone();
            let pp = format!("PP: {}/{}", skill.pp, skill.max_pp);
            let stype = skill.skill_type.clone();
            let show_type = !stype.is_empty();
            rsx! {
                div { class: "tooltip-skill-row",
                    span { class: "skill-name", "{name}" }
                    span { class: "skill-pp", "{pp}" }
                    if show_type {
                        ..vec![crate::type_badge::type_badge(&stype, "", "type-badge-sm")]
                    }
                }
            }
        })
        .collect();
    let player_tooltip = if player_hover {
        rsx! {
            div { class: "pokemon-tooltip",
                div { class: "pokemon-tooltip-header",
                    div { class: "tooltip-name-row",
                        span { class: "tooltip-name", "{tooltip_name}" }
                        span { class: "tooltip-level", "{tooltip_level}" }
                    }
                }
                div { class: "pokemon-tooltip-stats",
                    div { class: "tooltip-stat",
                        span { class: "stat-label", "HP" }
                        span { class: "stat-value", "{tooltip_hp}" }
                    }
                }
                div { class: "pokemon-tooltip-skills",
                    for i in 0..tooltip_skills.len() {
                        ..vec![tooltip_skills[i].clone()]
                    }
                }
            }
        }
    } else {
        VNode::empty()
    };

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
                let sid = skill.id;
                let skill_state = st.clone();
                let is_disabled = loading || skill.pp == 0;
                let cls = if is_disabled {
                    "skill-btn disabled"
                } else {
                    "skill-btn"
                };
                let name = skill.name.clone();
                // 徽章是旧版 TypeBadge：属性、分类各一枚，并排在 .skill-type-badges 里。
                let stype = skill.skill_type.clone();
                let scat = skill.category.clone();
                let badges = crate::type_badge::type_badge(&stype, &scat, "");
                let power_label = match skill.category.as_str() {
                    "特攻" | "特殊" => "特攻：",
                    "物攻" | "物理" => "物攻：",
                    _ => "威力：",
                };
                let power_str = skill.power.to_string();
                // `PP：<strong>15/15</strong>`，上限在 strong 里面；没有上限时只有当前值。
                let pp_str = if skill.max_pp > 0 {
                    format!("{}/{}", skill.pp, skill.max_pp)
                } else {
                    skill.pp.to_string()
                };
                let pp_cls = if skill.pp == 0 {
                    "skill-pp low"
                } else if skill.max_pp > 0 && skill.pp <= skill.max_pp / 3 {
                    "skill-pp warn"
                } else {
                    "skill-pp"
                };
                skill_btns.push(rsx! {
                    Button {
                        class: (cls).to_string(),
                        disabled: is_disabled,
                        onclick: crate::button::click(move || use_skill(&skill_state, sid)),
                        div { class: "skill-btn-content",
                            div { class: "skill-name", "{name}" }
                            ..vec![badges]
                            div { class: "skill-stats-row",
                                span { class: "skill-power",
                                    "{power_label}"
                                    strong { "{power_str}" }
                                }
                                span { class: pp_cls,
                                    "PP："
                                    strong { "{pp_str}" }
                                }
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
            let attack_state = st.clone();
            rsx! {
                div { class: "skills-panel",
                    div { class: "attack-button-section",
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: (if loading { "normal-attack-btn disabled" } else { "normal-attack-btn" }).to_string(),
                            disabled: loading,
                            onclick: crate::button::click(move || normal_attack(&attack_state)),
                            {Icon(IconProps {
                                name: Some(IconName::Swords),
                                size: Some(22),
                                class: Some("attack-btn-icon".to_string()),
                            })}
                            span { class: "attack-btn-label", "普通攻击" }
                                                } }, [("data-test-bt-attack", ("1").to_string())]) }
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
            let cards = item_cards(st, my, loading, false);
            if cards.is_empty() {
                rsx! { div { class: "items-panel", div { class: "empty-items-hint", "暂无道具" } } }
            } else {
                let nodes: Vec<VNode> = cards.iter().map(|c| battle_item_card(st, c)).collect();
                rsx! {
                    div { class: "items-panel",
                        div { class: "battle-items-grid",
                            for i in 0..nodes.len() {
                                ..vec![nodes[i].clone()]
                            }
                        }
                    }
                }
            }
        }
        2 => {
            let cards = item_cards(st, my, loading, true);
            if cards.is_empty() {
                rsx! { div { class: "capture-panel", div { class: "empty-items-hint", "暂无精灵球" } } }
            } else {
                let nodes: Vec<VNode> = cards.iter().map(|c| battle_item_card(st, c)).collect();
                rsx! {
                    div { class: "capture-panel",
                        div { class: "battle-items-grid",
                            for i in 0..nodes.len() {
                                ..vec![nodes[i].clone()]
                            }
                        }
                    }
                }
            }
        }
        _ => {
            let flee_state = st.clone();
            rsx! {
                div { class: "flee-panel",
                    { crate::button::hooked(tairitsu_macros::rsx! { Button {
                        class: (if loading { "flee-button disabled" } else { "flee-button" }).to_string(),
                        disabled: loading,
                        onclick: crate::button::click(move || flee(&flee_state)),
                        {Icon(IconProps {
                            name: Some(IconName::CircleX),
                            size: Some(16),
                            class: Some("flee-icon".to_string()),
                        })}
                        span { "退出战斗" }
                                                } }, [("data-test-bt-flee-tab", ("1").to_string())]) }
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
                let item_id = sel.item_id;
                let skill_id = skill.id;
                let row_state = st.clone();
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
                    Button {
                        class: ("skill-selection-item").to_string(),
                        disabled: loading,
                        onclick: crate::button::click(move || use_item_on_skill(&row_state, item_id, skill_id)),
                        div { class: "skill-selection-info",
                            span { class: "skill-selection-name", "{name}" }
                            span { class: pp_cls, "{pp_text}" }
                        }
                        span { class: "skill-selection-action", "选择" }
                                        }
                });
            }
            let sel_overlay_state = st.clone();
            // 外壳是共用的 hikari Modal（见 crate::modal）。旧正文的类
            // `skill-selection-modal` 落在 body_class 上，子节点与旧版相同。
            let body: Vec<VNode> = vec![
                rsx! { p { class: "skill-selection-hint", "请选择要恢复PP的技能：" } },
                rsx! {
                    div { class: "skill-selection-list",
                        for i in 0..rows.len() {
                            ..vec![rows[i].clone()]
                        }
                    }
                },
            ];
            crate::modal::modal_with(
                crate::modal::ModalSpec {
                    body_class: "skill-selection-modal",
                    ..crate::modal::ModalSpec::new(&title, false)
                },
                move || cancel_selection(&sel_overlay_state),
                body,
            )
        }
        None => VNode::empty(),
    };

    let log_modal = if bt.show_log.get() {
        // 对齐 Dioxus 版 log.rs：「重新载入」+ 逐回合分组 + 「可贴到论坛的战报」
        // 文本框 + 复制按钮，整列排在 `.battle-log-panel`（flex 列、12px 间距）里。
        let log = bt.log_data.get();
        let copy_status = bt.log_copy_status.get();
        let reload_state = st.clone();
        let reload_btn: VNode = crate::button::hooked(
            tairitsu_macros::rsx! { Button {
            class: ("btn btn-secondary").to_string(),
            onclick: crate::button::click(move || reload_log(&reload_state)),
            "重新载入"
                        } },
            [("data-test-bt-log-refresh", ("1").to_string())],
        );
        let report = match log {
            None => rsx! { p { role: "status", "正在载入战报..." } },
            Some(Err(e)) => {
                let msg = format!("战报载入失败：{e}。请重新载入。");
                rsx! { p { role: "alert", "data-test-bt-log-error": "1", "{msg}" } }
            }
            Some(Ok(data)) => {
                let bbcode = data.bbcode.clone();
                let turns: Vec<VNode> = data
                    .turns
                    .iter()
                    .map(|t| {
                        let title = format!("第 {} 回合", t.turn);
                        let rows: Vec<VNode> = t
                            .lines
                            .iter()
                            .map(|l| {
                                let text = l.clone();
                                rsx! { p { "{text}" } }
                            })
                            .collect();
                        rsx! {
                            section { class: "battle-log-turn",
                                h4 { "{title}" }
                                for i in 0..rows.len() {
                                    ..vec![rows[i].clone()]
                                }
                            }
                        }
                    })
                    .collect();
                let flat: Vec<VNode> = data
                    .lines
                    .iter()
                    .map(|l| {
                        let text = l.clone();
                        rsx! { p { "{text}" } }
                    })
                    .collect();
                let empty_notice = rsx! { p { "尚无战斗事件。" } };
                let copy_state = st.clone();
                rsx! {
                    div { class: "battle-log-events",
                        if data.turns.is_empty() {
                            if data.lines.is_empty() {
                                ..vec![empty_notice]
                            }
                            for i in 0..flat.len() {
                                ..vec![flat[i].clone()]
                            }
                        } else {
                            for i in 0..turns.len() {
                                ..vec![turns[i].clone()]
                            }
                        }
                    }
                    label { "for": "battle-share-text", "可贴到论坛的战报" }
                    textarea {
                        id: "battle-share-text",
                        "data-test-bt-log-text": "1",
                        readonly: true,
                        rows: "5",
                        "{bbcode}"
                    }
                    { crate::button::hooked(tairitsu_macros::rsx! { Button {
                        class: ("btn btn-primary").to_string(),
                        onclick: crate::button::click(move || copy_log(&copy_state)),
                        "复制战报"
                                                    } }, [("data-test-bt-log-copy", ("1").to_string())]) }
                    p { role: "status", class: "battle-copy-status", "{copy_status}" }
                }
            }
        };
        let log_overlay_state = st.clone();
        let log_close_state = st.clone();
        // 外壳是共用的 hikari Modal（见 crate::modal）。旧正文的类
        // `battle-log-panel` 落在 body_class 上，子节点与旧版相同。
        crate::modal::modal_hooked(
            crate::modal::ModalSpec {
                body_class: "battle-log-panel",
                ..crate::modal::ModalSpec::new("逐回合战报", false)
            },
            crate::modal::hook_close_button("data-test-bt-log-close", move || {
                close_log(&log_close_state)
            }),
            move || close_log(&log_overlay_state),
            vec![reload_btn, report],
        )
    } else {
        VNode::empty()
    };

    // 道具卡的悬浮提示与详情弹窗（旧版渲染在卡片组件里；这里放在页面层，
    // 免得被战斗卡片的变换/动画当成包含块）。
    let item_tooltip = match bt.item_hover.get() {
        Some(key) => match find_card(st, my, loading, key) {
            Some(card) => battle_item_tooltip(&card, bt.item_hover_pos.get()),
            None => VNode::empty(),
        },
        None => VNode::empty(),
    };
    let item_details = match bt.item_details.get() {
        Some(key) => match find_card(st, my, loading, key) {
            Some(card) => battle_item_details_modal(st, &card),
            None => VNode::empty(),
        },
        None => VNode::empty(),
    };

    let _ = pokemon_empty;
    let log_open_state = st.clone();
    let skills_tab_state = st.clone();
    let items_tab_state = st.clone();
    let capture_tab_state = st.clone();
    let flee_tab_state = st.clone();

    rsx! {
        div { class: "page-battle",
            div { class: "battle-tools",
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    variant: hikari_components::ButtonVariant::Secondary,,                    class: ("btn btn-secondary").to_string(),
                    disabled: scene.engine_battle_id == 0,
                    onclick: crate::button::click(move || open_log(&log_open_state)),
                    "查看战报 / 分享"
                                } }, [("data-test-bt-log-open", ("1").to_string())]) }
            }
            div { class: "battle-card",
                div { class: "battle-card-body",
                    div { class: "battle-content-area",
                        // 页签：激活的那一枚前后各一枚箭头，文字包在 span 里（旧版如此）。
                        div { class: "battle-tab-menu",
                            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                                class: (skills_cls).to_string(),
                                onclick: crate::button::click(move || switch_tab(&skills_tab_state, 0)),
                                if tab == 0 {
                                    span { class: "tab-arrow left", ">" }
                                }
                                span { "技能" }
                                if tab == 0 {
                                    span { class: "tab-arrow right", "<" }
                                }
                                                        } }, [("data-test-bt-tab", ("0").to_string())]) }
                            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                                class: (items_cls).to_string(),
                                onclick: crate::button::click(move || switch_tab(&items_tab_state, 1)),
                                if tab == 1 {
                                    span { class: "tab-arrow left", ">" }
                                }
                                span { "道具" }
                                if tab == 1 {
                                    span { class: "tab-arrow right", "<" }
                                }
                                                        } }, [("data-test-bt-tab", ("1").to_string())]) }
                            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                                class: (capture_cls).to_string(),
                                onclick: crate::button::click(move || switch_tab(&capture_tab_state, 2)),
                                if tab == 2 {
                                    span { class: "tab-arrow left", ">" }
                                }
                                span { "捕捉" }
                                if tab == 2 {
                                    span { class: "tab-arrow right", "<" }
                                }
                                                        } }, [("data-test-bt-tab", ("2").to_string())]) }
                            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                                class: (flee_cls).to_string(),
                                onclick: crate::button::click(move || switch_tab(&flee_tab_state, 3)),
                                if tab == 3 {
                                    span { class: "tab-arrow left", ">" }
                                }
                                span { "退出战斗" }
                                if tab == 3 {
                                    span { class: "tab-arrow right", "<" }
                                }
                                                        } }, [("data-test-bt-tab", ("3").to_string())]) }
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
                                                        span { class: "pet-mark pet-mark-shiny", title: "异色（闪光）宠物", "闪" }
                                                    }
                                                    span { class: wild_gender_cls, "{wild_gender}" }
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
                                        "data-test-bt-switch-open": "1",
                                        onclick: move |_| open_switch_modal(&switch_open_state),
                                        onmouseenter: move |_| show_player_tooltip(&hover_enter_state),
                                        onmouseleave: move |_| hide_player_tooltip(&hover_leave_state),
                                        div { class: "player-info-box",
                                            div { class: "player-header",
                                                div { class: "player-name-row",
                                                    span { class: "player-name", "{my_name}" }
                                                    if my_shiny {
                                                        span { class: "pet-mark pet-mark-shiny", title: "异色（闪光）宠物", "闪" }
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
                                        ..vec![player_tooltip]
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
            ..vec![modal, selection_modal, log_modal, item_details, item_tooltip]
        }
    }
}

/// 结算弹窗（胜利/失败/逃脱/捕捉 + 奖励 + 结束/继续）。
fn result_modal(st: &AppState, scene: &crate::battle_types::BattleScene, loading: bool) -> VNode {
    let result_log_state = st.clone();
    let end_state = st.clone();
    let continue_state = st.clone();
    let title = scene.status.result_title();
    // 结果旁的图标按状态取（旧版 BattleResultSection 的 result_icon）。
    let result_icon = match scene.status {
        BattleStatus::Victory | BattleStatus::Captured => Some(IconName::PartyPopper),
        BattleStatus::Defeat => Some(IconName::HeartCrack),
        BattleStatus::Fled => Some(IconName::Footprints),
        BattleStatus::Active => None,
    };
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
                        div { class: "reward-item",
                            {Icon(IconProps {
                                name: Some(IconName::Star),
                                size: Some(14),
                                class: Some("reward-icon".to_string()),
                            })}
                            span { class: "reward-value", "{exp_text}" }
                        }
                        div { class: "reward-item",
                            {Icon(IconProps {
                                name: Some(IconName::Coins),
                                size: Some(14),
                                class: Some("reward-icon".to_string()),
                            })}
                            span { class: "reward-value", "{money_text}" }
                        }
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
            let text = format!("{my_name} 升级了！");
            let levels = format!("Lv.{} → Lv.{}", lu.old_level, lu.new_level);
            rsx! {
                div { class: "level-up-notice",
                    {Icon(IconProps {
                        name: Some(IconName::ArrowUp),
                        size: Some(14),
                        class: Some("level-up-icon".to_string()),
                    })}
                    span { class: "level-up-text", "{text}" }
                    span { class: "level-up-levels", "{levels}" }
                }
            }
        }
        _ => VNode::empty(),
    };

    // 胜利且宠物存活时可继续（对齐 Dioxus 版 can_continue 判定）。
    let can_continue = scene.status == BattleStatus::Victory && scene.my_pokemon.hp > 0;
    let continue_btn = if can_continue {
        rsx! {
            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                class: ("battle-continue-btn").to_string(),
                disabled: loading,
                onclick: crate::button::click(move || continue_battle(&continue_state)),
                "继续战斗"
                        } }, [("data-test-bt-continue", ("1").to_string())]) }
        }
    } else {
        VNode::empty()
    };

    // 外壳是共用的 hikari Modal（见 crate::modal）。旧正文 `battle-result-section`
    // 落在 body_class 上（窄屏下它的 `padding: 0` 与 `.modal-body` 的 `20px`
    // 同权重，同一个元素上才是旧版的结果），子节点与旧版相同。
    let body: Vec<VNode> = vec![
        rsx! {
            div { class: result_cls,
                h2 {
                    if let Some(icon) = result_icon {
                        {Icon(IconProps {
                            name: Some(icon),
                            size: Some(20),
                            class: Some("lucide-inline".to_string()),
                        })}
                    }
                    "{title}"
                }
                ..vec![rewards_block, level_up_block]
            }
        },
        rsx! {
            div { class: "battle-end-buttons",
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: ("battle-end-btn").to_string(),
                    onclick: crate::button::click(move || open_log(&result_log_state)),
                    "查看 / 分享战报"
                                } }, [("data-test-bt-end-log", ("1").to_string())]) }
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: ("battle-end-btn").to_string(),
                    disabled: loading,
                    onclick: crate::button::click(move || end_battle(&end_state)),
                    "返回冒险地图"
                                } }, [("data-test-bt-end", ("1").to_string())]) },
                ..vec![continue_btn]
            }
        },
    ];
    // 旧版这个弹窗有关闭键、遮罩也点得动（`close_on_overlay: !loading &&
    // !pokemon_fainted`），两者的回调在战斗已结束时就等于「返回冒险地图」。
    let close_state = st.clone();
    let battle_over = scene.effective_battle_over();
    let needs_replacement = scene.needs_replacement();
    crate::modal::modal_with(
        crate::modal::ModalSpec {
            body_class: "battle-result-section",
            ..crate::modal::ModalSpec::new(&title, false)
        },
        move || {
            if loading || needs_replacement {
                return;
            }
            if battle_over {
                end_battle(&close_state);
            }
        },
        body,
    )
}

/// 我方倒下但有替补：选择替换宠物（或无惩罚逃跑）。
fn replace_modal(st: &AppState, scene: &crate::battle_types::BattleScene, loading: bool) -> VNode {
    let fainted_name = scene.my_pokemon.name.clone();
    let message = format!("{fainted_name} 倒下了！请选择一只宠物继续战斗：");
    let list_state = st.center.pokemons.get();
    let list_loading = list_state.is_none();
    let list = list_state.and_then(|r| r.ok()).unwrap_or_default();
    let fainted_id = scene.my_pokemon.instance_id;
    let replace_flee_state = st.clone();

    let mut rows: Vec<VNode> = Vec::new();
    for pm in list.iter().filter(|p| p.site == 1 || p.site == 2) {
        let is_the_fainted = pm.id == fainted_id;
        let is_fainted = is_the_fainted || pm.hp <= 0;
        let attr = pm.id.to_string();
        let pm_id = pm.id;
        let replace_state = st.clone();
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
        let hp_cls = if is_fainted {
            "hp-text fainted"
        } else {
            "hp-text"
        };
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
            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                class: (cls).to_string(),
                disabled: loading || is_fainted,
                onclick: crate::button::click(move || replace_pokemon(&replace_state, pm_id)),
                img { class: "replacement-pokemon-sprite", src: sprite, alt: alt_name }
                div { class: "replacement-pokemon-info",
                    div { class: "replacement-pokemon-name",
                        "{name}"
                        ..vec![first_badge, fainted_badge]
                    }
                    div { class: "replacement-pokemon-level", "{level}" }
                }
                div { class: "replacement-pokemon-hp",
                    span { class: hp_cls, "{hp_text}" }
                }
                        } }, [("data-test-bt-replace", (attr).to_string())]) }
        });
    }

    // 替换宠物的选择框：外壳是共用的 hikari Modal。旧版它照样带关闭键、遮罩
    // 也接了点击，只是回调在宠物倒下时直接返回——所以这里关闭键在、点了没反应，
    // 只能选一只或逃跑。旧正文的类 `faint-replace-modal` 落在 body_class 上，
    // 子节点与旧版相同。
    let body: Vec<VNode> = vec![
        rsx! { p { class: "faint-message", "{message}" } },
        if list_loading {
            rsx! { p { "正在加载可替换的宠物..." } }
        } else {
            VNode::empty()
        },
        rsx! {
            div { class: "replacement-pokemon-list",
                for i in 0..rows.len() {
                    ..vec![rows[i].clone()]
                }
            }
        },
        rsx! {
            div { class: "faint-replace-actions",
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: ("btn btn-warning").to_string(),
                    disabled: loading,
                    onclick: crate::button::click(move || flee(&replace_flee_state)),
                    {Icon(IconProps {
                        name: Some(IconName::Footprints),
                        size: Some(14),
                        class: Some("lucide-inline".to_string()),
                    })}
                    "逃跑（无惩罚）"
                                } }, [("data-test-bt-flee", ("1").to_string())]) }
            }
        },
    ];
    crate::modal::modal_with(
        crate::modal::ModalSpec {
            body_class: "faint-replace-modal",
            ..crate::modal::ModalSpec::new("选择替换宠物", false)
        },
        || {},
        body,
    )
}

/// 手动换宠确认。
fn switch_modal(st: &AppState, loading: bool) -> VNode {
    let overlay_state = st.clone();
    let close_state = st.clone();
    let cancel_state = st.clone();
    let confirm_state = st.clone();
    // 外壳是共用的 hikari Modal（见 crate::modal）。旧正文的类
    // `switch-pokemon-modal` 落在 body_class 上（它的 `padding` 规则与
    // `.modal-body` 的同权重，同一个元素上才是旧版的结果），子节点与旧版相同。
    let body: Vec<VNode> = vec![
        rsx! {
            p { class: "switch-pokemon-warning",
                {Icon(IconProps {
                    name: Some(IconName::TriangleAlert),
                    size: Some(14),
                    class: Some("lucide-inline".to_string()),
                })}
                "切换宠物可能会让当前上场宠物再受到一次攻击！"
            }
        },
        rsx! {
            p { class: "switch-pokemon-hint", "确定要切换吗？" }
        },
        rsx! {
            div { class: "switch-pokemon-actions",
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    variant: hikari_components::ButtonVariant::Secondary,,                    class: ("btn btn-secondary").to_string(),
                    onclick: crate::button::click(move || close_switch_modal(&cancel_state)),
                    "取消"
                                } }, [("data-test-bt-switch-cancel", ("1").to_string())]) }
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: ("btn btn-primary").to_string(),
                    disabled: loading,
                    onclick: crate::button::click(move || switch_pokemon(&confirm_state)),
                    "确定切换"
                                } }, [("data-test-bt-switch-confirm", ("1").to_string())]) }
            }
        },
    ];
    crate::modal::modal_hooked(
        crate::modal::ModalSpec {
            body_class: "switch-pokemon-modal",
            ..crate::modal::ModalSpec::new("切换上场宠物", false)
        },
        crate::modal::hook_close_button("data-test-bt-switch-cancel", move || {
            close_switch_modal(&close_state)
        }),
        // 旧版关闭回调：动作进行中不关（`if loading || pokemon_fainted { return }`）。
        move || {
            if !loading {
                close_switch_modal(&overlay_state);
            }
        },
        body,
    )
}
