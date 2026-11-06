//! 我的宝可梦页（tairitsu 版，移植自 rust/game/src/pages/my_pokemon.rs 的
//! 「属性」标签页）。同名同类直用 game.css（.page-my-pokemon / .mypm-card /
//! .stats-tab-layout / .radar-svg …）。
//!
//! 本阶段范围：宠物选择、属性面板（编号/属性/状态/性别/闪光/品质）、HP 与
//! EXP 条、好感度、总个体值与六项个体值、五维雷达图、昵称编辑。
//!
//! 未接入：装备 / 技能标签页、努力值计算器、进化路径与进化弹窗——这些依赖
//! 各自端点的写流程，随后续阶段补齐；对应按钮不渲染，避免给出无法完成的入口。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use hikari_components::{Toast, ToastPosition, ToastProps, ToastVariant};

use crate::app_state::AppState;
use crate::quality::{quality_grade, quality_rank, UNKNOWN_RANK};

const IMG_PATH: &str = "source/plugin/pokemon/images";
/// 昵称上限（对齐 Dioxus 版输入框）。
const NICKNAME_MAX: usize = 14;

/// 首次进入装载：拉宠物列表，并按当前标签页拉详情/装备。
pub fn ensure_loaded(state: &AppState) {
    if state.center.pokemons.get().is_none() {
        load_list(state);
    } else {
        ensure_detail(state);
    }
    if state.my_pm.tab.get() == crate::state::MyPokemonTab::Equipment {
        ensure_equipment(state);
    }
}

fn load_list(state: &AppState) {
    let st = state.clone();
    crate::api::pokemon_list(Box::new(move |result| {
        st.center.pokemons.inner().set(Some(result));
        crate::request_remount();
        ensure_detail(&st);
    }));
}

/// 选中宠物变化或详情缺失时拉详情。
fn ensure_detail(state: &AppState) {
    let Some(list) = state.center.pokemons.get().and_then(|r| r.ok()) else {
        return;
    };
    let Some(pm) = pick_target(&list, state.my_pm.selected_id.get()) else {
        state.my_pm.detail.inner().set(Some(Ok(Default::default())));
        crate::request_remount();
        return;
    };
    let detail = state.my_pm.detail.get();
    let already = detail
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .map(|d| d.id == pm.id && !d.name.is_empty())
        .unwrap_or(false);
    if already {
        return;
    }
    let my = state.my_pm.clone();
    crate::api::pokemon_detail(
        pm.id,
        Box::new(move |result| {
            my.detail.inner().set(Some(result));
            crate::request_remount();
        }),
    );
}

/// 选中目标：指定 ID 若仍存在则用之，否则退回第一只（对齐 Dioxus 版
/// `or_else(get_first_pokemon)`）。
fn pick_target(
    list: &[crate::state::PokemonBasic],
    selected: Option<u64>,
) -> Option<crate::state::PokemonBasic> {
    selected
        .and_then(|id| list.iter().find(|p| p.id == id))
        .or_else(|| list.first())
        .cloned()
}

/// 切换选中的宠物。
pub fn select_pokemon(state: &AppState, id: u64) {
    if state.my_pm.selected_id.get() == Some(id) {
        return;
    }
    let my = state.my_pm.clone();
    my.selected_id.inner().set(Some(id));
    my.detail.inner().set(None);
    my.equipment.inner().set(None);
    my.equip_selected.inner().set(None);
    // 切换宠物时退出编辑态，避免把上一只的输入带过去。
    my.editing.inner().set(false);
    my.save_error.inner().set(None);
    my.save_success.inner().set(false);
    crate::request_remount();
    ensure_detail(state);
    ensure_equipment(state);
}

/// 开始编辑昵称。
pub fn begin_edit(state: &AppState, current: &str) {
    let my = state.my_pm.clone();
    my.edit_value.inner().set(current.to_string());
    my.editing.inner().set(true);
    my.save_error.inner().set(None);
    my.save_success.inner().set(false);
    crate::request_remount();
}

/// 取消编辑。
pub fn cancel_edit(state: &AppState) {
    let my = state.my_pm.clone();
    my.editing.inner().set(false);
    my.save_error.inner().set(None);
    my.save_success.inner().set(false);
    crate::request_remount();
}

/// 提交改名。空昵称在本地拦下（对齐 Dioxus 版）。
pub fn submit_rename(state: &AppState) {
    let my = state.my_pm.clone();
    if my.saving.get() {
        return;
    }
    let name = my.edit_value.get();
    let trimmed = name.trim();
    if trimmed.is_empty() {
        my.save_error.inner().set(Some("昵称不能为空".to_string()));
        crate::request_remount();
        return;
    }
    if trimmed.chars().count() > NICKNAME_MAX {
        my.save_error
            .inner()
            .set(Some(format!("昵称最多 {NICKNAME_MAX} 字")));
        crate::request_remount();
        return;
    }
    let Some(detail) = my.detail.get().and_then(|r| r.ok()) else {
        return;
    };
    let pokemon_id = detail.id;
    let new_name = trimmed.to_string();
    my.saving.inner().set(true);
    my.save_error.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    crate::api::rename_pokemon(
        pokemon_id,
        &new_name,
        Box::new(move |result| {
            let my = st.my_pm.clone();
            match result {
                Ok(()) => {
                    my.save_success.inner().set(true);
                    my.editing.inner().set(false);
                    // 详情与列表都要刷（页面显示与选择器都用昵称）。
                    my.detail.inner().set(None);
                    st.center.pokemons.inner().set(None);
                }
                Err(e) => {
                    my.save_error.inner().set(Some(format!("保存失败：{e}")));
                }
            }
            my.saving.inner().set(false);
            crate::request_remount();
            if my.save_error.get().is_none() {
                crate::api::pokemon_list({
                    let st = st.clone();
                    Box::new(move |result| {
                        st.center.pokemons.inner().set(Some(result));
                        crate::request_remount();
                        ensure_detail(&st);
                    })
                });
            }
        }),
    );
}

// ---------------------------------------------------------------------------
// 装备标签页：动作入口（delegation.rs 调用）
// ---------------------------------------------------------------------------

/// 切换标签页（进入装备页时装载装备数据）。
pub fn switch_tab(state: &AppState, tab: crate::state::MyPokemonTab) {
    let my = state.my_pm.clone();
    if my.tab.get() == tab {
        return;
    }
    my.tab.inner().set(tab);
    // 切标签页时清掉装备选择，避免残留高亮。
    my.equip_selected.inner().set(None);
    crate::request_remount();
    if tab == crate::state::MyPokemonTab::Equipment {
        ensure_equipment(state);
    }
}

/// 装备数据缺失时拉取（当前选中宠物）。
pub fn ensure_equipment(state: &AppState) {
    let already = state
        .my_pm
        .equipment
        .get()
        .map(|r| r.is_ok())
        .unwrap_or(false);
    if already {
        return;
    }
    let Some(list) = state.center.pokemons.get().and_then(|r| r.ok()) else {
        return;
    };
    let Some(pm) = pick_target(&list, state.my_pm.selected_id.get()) else {
        return;
    };
    let my = state.my_pm.clone();
    crate::api::equipment(
        pm.id,
        Box::new(move |result| {
            my.equipment.inner().set(Some(result));
            crate::request_remount();
        }),
    );
}

fn reload_equipment(state: &AppState) {
    let Some(list) = state.center.pokemons.get().and_then(|r| r.ok()) else {
        return;
    };
    let Some(pm) = pick_target(&list, state.my_pm.selected_id.get()) else {
        return;
    };
    let st = state.clone();
    crate::api::equipment(
        pm.id,
        Box::new(move |result| {
            st.my_pm.equipment.inner().set(Some(result));
            crate::request_remount();
        }),
    );
    // 装备会改 HP 上限，宠物列表（与属性页详情）一并失效重拉。
    state.my_pm.detail.inner().set(None);
    crate::api::pokemon_list({
        let st2 = state.clone();
        Box::new(move |result| {
            st2.center.pokemons.inner().set(Some(result));
            crate::request_remount();
        })
    });
}

/// 动作收尾：清 loading、清选择、刷数据（失败也刷——服务端可能完成了
/// 部分步骤，对齐 Dioxus 版 finish_equipment_action）。
fn finish_equip_action(state: &AppState, message: Option<(crate::state::ToastKind, String)>) {
    let my = state.my_pm.clone();
    if let Some((kind, msg)) = message {
        crate::state::mypm_show_toast_silent(&my, kind, msg);
    }
    my.equip_loading.inner().set(false);
    my.equip_selected.inner().set(None);
    crate::request_remount();
    reload_equipment(state);
}

/// 点背包装备卡：未装备的记录选中/再点取消；已占用的不可选。
pub fn select_inventory_item(state: &AppState, myitem_id: u64) {
    let my = state.my_pm.clone();
    if my.equip_loading.get() {
        return;
    }
    let Some(Ok(eq)) = my.equipment.get() else {
        return;
    };
    let Some(item) = eq.owned.iter().find(|i| i.myitem_id == myitem_id) else {
        return;
    };
    if item.is_equipped || item.equipped_count > 0 {
        return;
    }
    let selected = my.equip_selected.get();
    if let Some(sel) = selected {
        if sel.from_inventory && sel.myitem_id == myitem_id {
            my.equip_selected.inner().set(None);
            crate::request_remount();
            return;
        }
    }
    my.equip_selected
        .inner()
        .set(Some(crate::state::SelectedEquip {
            from_inventory: true,
            myitem_id,
            slot_index: None,
        }));
    crate::request_remount();
}

/// 点装备槽：按当前选择分派（装载 / 移动 / 选中已装备）。
pub fn click_slot(state: &AppState, slot_index: u32) {
    let my = state.my_pm.clone();
    if my.equip_loading.get() {
        return;
    }
    let Some(Ok(eq)) = my.equipment.get() else {
        return;
    };
    let occupied = eq
        .slots
        .iter()
        .find(|s| s.slot_index == slot_index)
        .and_then(|s| s.item.as_ref())
        .is_some();

    match my.equip_selected.get() {
        Some(sel) if sel.from_inventory => {
            // 装载：目标槽被占时先卸下。
            my.equip_loading.inner().set(true);
            crate::request_remount();
            if occupied {
                unequip_then(
                    state,
                    slot_index,
                    Box::new(move |st| {
                        equip_to(&st, sel.myitem_id, slot_index, "装备成功");
                    }),
                );
            } else {
                equip_to(state, sel.myitem_id, slot_index, "装备成功");
            }
        }
        Some(sel) => {
            let Some(from_slot) = sel.slot_index else {
                return;
            };
            if from_slot == slot_index {
                // 点原槽：取消选择。
                my.equip_selected.inner().set(None);
                crate::request_remount();
                return;
            }
            // 移动：目标槽被占时先卸下，再卸原槽，最后装到目标槽。
            my.equip_loading.inner().set(true);
            crate::request_remount();
            if occupied {
                unequip_then(
                    state,
                    slot_index,
                    Box::new(move |st| {
                        unequip_then(
                            &st,
                            from_slot,
                            Box::new(move |st2| {
                                equip_to(&st2, sel.myitem_id, slot_index, "装备移动成功");
                            }),
                        );
                    }),
                );
            } else {
                unequip_then(
                    state,
                    from_slot,
                    Box::new(move |st| {
                        equip_to(&st, sel.myitem_id, slot_index, "装备移动成功");
                    }),
                );
            }
        }
        None if occupied => {
            // 选中槽内已装备的物品（等待卸下/移动指令）。
            let slot_item = eq
                .slots
                .iter()
                .find(|s| s.slot_index == slot_index)
                .and_then(|s| s.item.clone());
            if let Some(item) = slot_item {
                my.equip_selected
                    .inner()
                    .set(Some(crate::state::SelectedEquip {
                        from_inventory: false,
                        myitem_id: item.myitem_id,
                        slot_index: Some(slot_index),
                    }));
                crate::request_remount();
            }
        }
        None => {}
    }
}

/// 点物品栏空白处：当前选中的是已装备物品时执行卸下。
pub fn unequip_selected_from_section(state: &AppState) {
    let my = state.my_pm.clone();
    if my.equip_loading.get() {
        return;
    }
    let Some(sel) = my.equip_selected.get() else {
        return;
    };
    if sel.from_inventory {
        // 背包物品选中时点空白：取消选择。
        my.equip_selected.inner().set(None);
        crate::request_remount();
        return;
    }
    let Some(slot_index) = sel.slot_index else {
        return;
    };
    my.equip_loading.inner().set(true);
    crate::request_remount();
    unequip_slot(state, slot_index);
}

fn current_pokemon_id(state: &AppState) -> Option<u64> {
    let list = state.center.pokemons.get().and_then(|r| r.ok())?;
    pick_target(&list, state.my_pm.selected_id.get()).map(|pm| pm.id)
}

fn unequip_slot(state: &AppState, slot_index: u32) {
    let Some(pid) = current_pokemon_id(state) else {
        finish_equip_action(state, None);
        return;
    };
    let st = state.clone();
    crate::api::unequip_item(
        pid,
        slot_index,
        Box::new(move |result| match result {
            Ok(()) => {
                finish_equip_action(
                    &st,
                    Some((crate::state::ToastKind::Success, "卸下装备成功".to_string())),
                );
            }
            Err(e) => {
                finish_equip_action(
                    &st,
                    Some((crate::state::ToastKind::Error, format!("卸下装备失败: {e}"))),
                );
            }
        }),
    );
}

fn unequip_then(state: &AppState, slot_index: u32, next: Box<dyn FnOnce(AppState)>) {
    let Some(pid) = current_pokemon_id(state) else {
        finish_equip_action(state, None);
        return;
    };
    let st = state.clone();
    crate::api::unequip_item(
        pid,
        slot_index,
        Box::new(move |result| match result {
            Ok(()) => next(st),
            Err(e) => finish_equip_action(
                &st,
                Some((crate::state::ToastKind::Error, format!("卸下装备失败: {e}"))),
            ),
        }),
    );
}

fn equip_to(state: &AppState, myitem_id: u64, slot_index: u32, success_msg: &str) {
    let Some(pid) = current_pokemon_id(state) else {
        finish_equip_action(state, None);
        return;
    };
    let st = state.clone();
    let msg = success_msg.to_string();
    crate::api::equip_item(
        pid,
        myitem_id,
        slot_index,
        Box::new(move |result| match result {
            Ok(()) => {
                finish_equip_action(&st, Some((crate::state::ToastKind::Success, msg)));
            }
            Err(e) => {
                finish_equip_action(
                    &st,
                    Some((crate::state::ToastKind::Error, format!("装备失败: {e}"))),
                );
            }
        }),
    );
}

// ---------------------------------------------------------------------------
// 技能标签页：动作入口（delegation.rs 调用）
// ---------------------------------------------------------------------------

/// 打开学习弹窗（拉可学习列表）。
pub fn open_learn_modal(state: &AppState) {
    let my = state.my_pm.clone();
    if my.skill_loading.get() {
        return;
    }
    my.learn_modal_open.inner().set(true);
    my.learnable.inner().set(None);
    crate::request_remount();
    load_learnable(state);
}

pub fn close_learn_modal(state: &AppState) {
    let my = state.my_pm.clone();
    if my.skill_loading.get() {
        return;
    }
    my.learn_modal_open.inner().set(false);
    crate::request_remount();
}

fn load_learnable(state: &AppState) {
    let Some(pid) = current_pokemon_id(state) else {
        return;
    };
    let my = state.my_pm.clone();
    crate::api::learnable_skills(
        pid,
        Box::new(move |result| {
            my.learnable.inner().set(Some(result));
            crate::request_remount();
        }),
    );
}

/// 技能动作收尾：清守卫、刷列表（技能槽来自 list 载荷）。
fn finish_skill_action(
    state: &AppState,
    message: Option<(crate::state::ToastKind, String)>,
    close_modal: bool,
) {
    let my = state.my_pm.clone();
    if let Some((kind, msg)) = message {
        crate::state::mypm_show_toast_silent(&my, kind, msg);
    }
    my.skill_loading.inner().set(false);
    my.skill_selected.inner().set(None);
    if close_modal {
        my.learn_modal_open.inner().set(false);
    }
    crate::request_remount();
    // 技能槽在 list 载荷里：重拉列表即可（详情不含技能，无需失效）。
    let st = state.clone();
    crate::api::pokemon_list(Box::new(move |result| {
        st.center.pokemons.inner().set(Some(result));
        crate::request_remount();
    }));
}

/// 学习技能（弹窗内）。
pub fn learn_skill(state: &AppState, skill_id: u64) {
    let my = state.my_pm.clone();
    if my.skill_loading.get() {
        return;
    }
    let Some(Ok((available, _))) = my.learnable.get() else {
        return;
    };
    if !available.iter().any(|s| s.id == skill_id && s.is_available) {
        return;
    }
    let Some(pid) = current_pokemon_id(state) else {
        return;
    };
    my.skill_loading.inner().set(true);
    crate::request_remount();
    let st = state.clone();
    crate::api::learn_skill(
        pid,
        skill_id,
        Box::new(move |result| match result {
            Ok(()) => finish_skill_action(
                &st,
                Some((crate::state::ToastKind::Success, "技能学习成功".to_string())),
                true,
            ),
            Err(e) => finish_skill_action(
                &st,
                Some((crate::state::ToastKind::Error, format!("学习技能失败：{e}"))),
                false,
            ),
        }),
    );
}

/// 选中/取消选中技能卡（展开遗忘操作）。
pub fn toggle_skill_card(state: &AppState, skill_type_id: u64) {
    let my = state.my_pm.clone();
    if my.skill_loading.get() {
        return;
    }
    let next = if my.skill_selected.get() == Some(skill_type_id) {
        None
    } else {
        Some(skill_type_id)
    };
    my.skill_selected.inner().set(next);
    crate::request_remount();
}

/// 遗忘技能（客户端先行校验 can_forget，对齐 Dioxus 版）。
pub fn forget_skill(state: &AppState, skill_type_id: u64) {
    let my = state.my_pm.clone();
    if my.skill_loading.get() {
        return;
    }
    let Some(list) = state.center.pokemons.get().and_then(|r| r.ok()) else {
        return;
    };
    let Some(pm) = pick_target(&list, state.my_pm.selected_id.get()) else {
        return;
    };
    let Some(skill) = pm.skills.iter().find(|s| s.type_id == skill_type_id) else {
        return;
    };
    if !skill.can_forget {
        crate::state::mypm_show_toast_silent(
            &my,
            crate::state::ToastKind::Error,
            format!(
                "该技能PP未满（{}/{}），请先在宠物中心治疗，或使用PP恢复道具补满后再遗忘",
                skill.pp, skill.max_pp
            ),
        );
        crate::request_remount();
        return;
    }
    my.skill_loading.inner().set(true);
    crate::request_remount();
    let st = state.clone();
    crate::api::forget_skill(
        pm.id,
        skill_type_id,
        Box::new(move |result| match result {
            Ok(()) => finish_skill_action(
                &st,
                Some((crate::state::ToastKind::Success, "技能遗忘成功".to_string())),
                false,
            ),
            Err(e) => finish_skill_action(
                &st,
                Some((crate::state::ToastKind::Error, format!("遗忘技能失败：{e}"))),
                false,
            ),
        }),
    );
}

// ---------------------------------------------------------------------------
// 属性页弹窗：努力值计算器 / 进化路径 / 进化
// ---------------------------------------------------------------------------

pub fn open_ev_modal(state: &AppState) {
    state.my_pm.show_ev_modal.inner().set(true);
    crate::request_remount();
}

pub fn close_ev_modal(state: &AppState) {
    state.my_pm.show_ev_modal.inner().set(false);
    crate::request_remount();
}

/// 努力值输入变更：idx 0-5，`added` 区分当前值/追加值，夹在 0-255。
pub fn set_ev_value(state: &AppState, idx: usize, added: bool, raw: &str) {
    if idx >= 6 {
        return;
    }
    let value: u32 = raw.trim().parse::<u32>().map(|v| v.min(255)).unwrap_or(0);
    let my = state.my_pm.clone();
    if added {
        let mut arr = my.ev_added.get();
        arr[idx] = value;
        my.ev_added.inner().set(arr);
    } else {
        let mut arr = my.ev_current.get();
        arr[idx] = value;
        my.ev_current.inner().set(arr);
    }
    crate::request_remount();
}

pub fn open_evo_path_modal(state: &AppState) {
    let my = state.my_pm.clone();
    let Some(list) = state.center.pokemons.get().and_then(|r| r.ok()) else {
        return;
    };
    let Some(pm) = pick_target(&list, state.my_pm.selected_id.get()) else {
        return;
    };
    my.show_evo_path_modal.inner().set(true);
    my.evo_path_forward.inner().set(true);
    my.evo_path.inner().set(None);
    crate::request_remount();
    crate::api::evolution_path(
        pm.type_id,
        Box::new(move |result| {
            my.evo_path.inner().set(Some(result));
            crate::request_remount();
        }),
    );
}

pub fn close_evo_path_modal(state: &AppState) {
    state.my_pm.show_evo_path_modal.inner().set(false);
    crate::request_remount();
}

pub fn switch_evo_path_tab(state: &AppState, forward: bool) {
    state.my_pm.evo_path_forward.inner().set(forward);
    crate::request_remount();
}

pub fn open_evolution_modal(state: &AppState) {
    let my = state.my_pm.clone();
    let Some(pid) = current_pokemon_id(state) else {
        return;
    };
    my.show_evolution_modal.inner().set(true);
    my.evo_check.inner().set(None);
    crate::request_remount();
    crate::api::evolution_check(
        pid,
        Box::new(move |result| {
            my.evo_check.inner().set(Some(result));
            crate::request_remount();
        }),
    );
}

pub fn close_evolution_modal(state: &AppState) {
    let my = state.my_pm.clone();
    if my.evolving.get() {
        return;
    }
    my.show_evolution_modal.inner().set(false);
    crate::request_remount();
}

/// 执行进化：成功后关闭弹窗、刷列表与详情（形态会变）。
pub fn evolve_to(state: &AppState, to_id: u64, to_name: &str) {
    let my = state.my_pm.clone();
    if my.evolving.get() {
        return;
    }
    let Some(pid) = current_pokemon_id(state) else {
        return;
    };
    my.evolving.inner().set(true);
    crate::request_remount();
    let st = state.clone();
    let name = to_name.to_string();
    crate::api::evolve(
        pid,
        to_id,
        Box::new(move |result| {
            let my = st.my_pm.clone();
            match result {
                Ok(new_name) => {
                    let shown = if new_name.is_empty() { name } else { new_name };
                    crate::state::mypm_show_toast_silent(
                        &my,
                        crate::state::ToastKind::Success,
                        format!("🎉 进化成功！进化为了 {shown}"),
                    );
                    my.show_evolution_modal.inner().set(false);
                    // 形态/名字/属性全变：列表与详情都失效重拉。
                    my.detail.inner().set(None);
                    let st2 = st.clone();
                    crate::api::pokemon_list(Box::new(move |result| {
                        st2.center.pokemons.inner().set(Some(result));
                        crate::request_remount();
                    }));
                }
                Err(e) => {
                    crate::state::mypm_show_toast_silent(
                        &my,
                        crate::state::ToastKind::Error,
                        format!("进化失败：{e}"),
                    );
                }
            }
            my.evolving.inner().set(false);
            crate::request_remount();
        }),
    );
}

// ---------------------------------------------------------------------------
// 组件
// ---------------------------------------------------------------------------

#[component]
pub fn MyPokemon(state: Box<AppState>) -> VNode {
    let st = state;
    let list_state = st.center.pokemons.get();
    let detail_state = st.my_pm.detail.get();
    let editing = st.my_pm.editing.get();
    let edit_value = st.my_pm.edit_value.get();
    let saving = st.my_pm.saving.get();
    let save_error = st.my_pm.save_error.get();
    let save_success = st.my_pm.save_success.get();
    let tab = st.my_pm.tab.get();

    let loading = list_state.is_none();
    let list = list_state
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();
    let list_err = list_state.as_ref().and_then(|r| r.as_ref().err()).cloned();

    let selected = st.my_pm.selected_id.get();
    let current = pick_target(&list, selected);
    let detail = detail_state.as_ref().and_then(|r| r.as_ref().ok()).cloned();
    let detail_err = detail_state
        .as_ref()
        .and_then(|r| r.as_ref().err())
        .cloned();

    let body = if loading {
        rsx! { div { class: "loading", "正在加载宠物数据..." } }
    } else if let Some(err) = list_err {
        let msg = format!("加载失败：{err}");
        rsx! { div { class: "error-message", "{msg}" } }
    } else if let Some(pm) = current.clone() {
        let selector = pet_selector(&list, pm.id, &st);

        // 标签页菜单。
        let stats_cls = if tab == crate::state::MyPokemonTab::Stats {
            "mypm-tab active"
        } else {
            "mypm-tab"
        };
        let equip_cls = if tab == crate::state::MyPokemonTab::Equipment {
            "mypm-tab active"
        } else {
            "mypm-tab"
        };
        let skills_cls = if tab == crate::state::MyPokemonTab::Skills {
            "mypm-tab active"
        } else {
            "mypm-tab"
        };
        let tab_menu = rsx! {
            div { class: "mypm-tab-menu",
                button { class: stats_cls, "data-pm-my-pm-tab": "stats", "属性" }
                button { class: equip_cls, "data-pm-my-pm-tab": "equipment", "装备" }
                button { class: skills_cls, "data-pm-my-pm-tab": "skills", "技能" }
            }
        };

        // 精灵图区（装备页时两侧展示六维加成合计）。
        let bonus_tags = if tab == crate::state::MyPokemonTab::Equipment {
            st.my_pm
                .equipment
                .get()
                .and_then(|r| r.ok())
                .map(|eq| bonus_tags_vnode(&eq.total_bonuses()))
                .unwrap_or_else(VNode::empty)
        } else {
            VNode::empty()
        };
        let sprite = format!("{IMG_PATH}/spm/{}.gif", pm.type_id);
        let sprite_alt = pm.display_name();
        let sprite_area = rsx! {
            div { class: "mypm-main-content",
                div { class: "pokemon-sprite-area",
                    div { class: "pokemon-display-with-tags",
                        ..vec![bonus_tags]
                        img { class: "pokemon-sprite-img", src: sprite, alt: sprite_alt }
                    }
                }
                div { class: "mypm-tab-content",
                    ..vec![match tab {
                        crate::state::MyPokemonTab::Stats => match &detail {
                            Some(d) if !d.name.is_empty() => stats_panel(
                                &st,
                                d,
                                editing,
                                &edit_value,
                                saving,
                                &save_error,
                                save_success,
                            ),
                            _ => match detail_err {
                                Some(err) => {
                                    let msg = format!("详情加载失败：{err}");
                                    rsx! { div { class: "error-message", "{msg}" } }
                                }
                                None => {
                                    let name = pm.display_name();
                                    let hint = format!("正在加载 {name} 的属性...");
                                    rsx! { div { class: "loading", "{hint}" } }
                                }
                            },
                        },
                        crate::state::MyPokemonTab::Equipment => equipment_panel(&st, pm.id),
                        crate::state::MyPokemonTab::Skills => skills_panel(&st, &pm),
                    }]
                }
            }
        };

        rsx! {
            div { class: "mypm-content-area",
                ..vec![selector, tab_menu]
                div { class: "mypm-divider" }
                ..vec![sprite_area]
            }
        }
    } else {
        rsx! { div { class: "empty-hint", "你还没有宠物，快去冒险捕捉吧！" } }
    };

    // 学习弹窗（技能页打开时）。
    let learn_modal = if st.my_pm.learn_modal_open.get() {
        learn_modal_vnode(&st)
    } else {
        VNode::empty()
    };
    // 属性页三弹窗。
    let ev_modal = if st.my_pm.show_ev_modal.get() {
        ev_calculator_modal(&st)
    } else {
        VNode::empty()
    };
    let evo_path_modal = if st.my_pm.show_evo_path_modal.get() {
        evo_path_modal(&st)
    } else {
        VNode::empty()
    };
    let evolution_modal = if st.my_pm.show_evolution_modal.get() {
        evolution_modal(&st)
    } else {
        VNode::empty()
    };
    let modals: Vec<VNode> = vec![ev_modal, evo_path_modal, evolution_modal];

    let toast = if st.my_pm.toast_visible.get() {
        let toast_state = st.my_pm.clone();
        let variant = match st.my_pm.toast_kind.get() {
            crate::state::ToastKind::Success => ToastVariant::Success,
            crate::state::ToastKind::Error => ToastVariant::Error,
        };
        let message = st.my_pm.toast_message.get();
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

    let icon = format!("{IMG_PATH}/item/jlq.gif");
    rsx! {
        div { class: "page-my-pokemon",
            div { class: "mypm-card",
                div { class: "mypm-card-header",
                    img { class: "header-icon-img", src: icon, alt: "" }
                    span { "我的宠物" }
                }
                div { class: "mypm-card-body", ..vec![body] }
            }
            ..{
                let mut all = vec![learn_modal];
                all.extend(modals.iter().cloned());
                all.push(toast);
                all
            }
        }
    }
}

/// 装备加成合计标签（左 HP/攻击/防御，右 特攻/特防/速度；对齐 Dioxus
/// BonusTagsLeft/Right，非零才显示）。
fn bonus_tags_vnode(total: &[i64; 6]) -> VNode {
    let names = ["HP", "攻击", "防御", "特攻", "特防", "速度"];
    let classes = [
        "bonus-tag-small hp",
        "bonus-tag-small atk",
        "bonus-tag-small def",
        "bonus-tag-small spat",
        "bonus-tag-small spdef",
        "bonus-tag-small spd",
    ];
    let chips: Vec<VNode> = names
        .iter()
        .zip(classes.iter())
        .enumerate()
        .filter(|(i, _)| total[*i] != 0)
        .map(|(i, (name, cls))| {
            let text = if total[i] > 0 {
                format!("+{} {}", total[i], name)
            } else {
                format!("{} {}", total[i], name)
            };
            let c = cls.to_string();
            rsx! { span { class: c, "{text}" } }
        })
        .collect();
    rsx! {
        div { class: "side-tags",
            for i in 0..chips.len() {
                ..vec![chips[i].clone()]
            }
        }
    }
}

/// 装备标签页面板：4 槽 + 物品栏。
fn equipment_panel(st: &AppState, pokemon_id: u64) -> VNode {
    let _ = pokemon_id;
    let equip_state = st.my_pm.equipment.get();
    let loading = st.my_pm.equip_loading.get();
    let selected = st.my_pm.equip_selected.get();

    match &equip_state {
        None => rsx! { div { class: "loading", "正在加载装备..." } },
        Some(Err(error)) => {
            let msg = format!("装备加载失败：{error}");
            rsx! {
                div { class: "equipment-tab",
                    div { role: "alert", class: "error-message", "{msg}" }
                    button {
                        class: "btn btn-primary",
                        "data-pm-eq-reload": "1",
                        "重新加载装备"
                    }
                }
            }
        }
        Some(Ok(eq)) => {
            let has_equip_selected = selected
                .as_ref()
                .map(|s| !s.from_inventory)
                .unwrap_or(false);
            let has_inv_selected = selected.as_ref().map(|s| s.from_inventory).unwrap_or(false);
            let section_cls = if has_equip_selected {
                "equipment-inventory-section can-unequip"
            } else {
                "equipment-inventory-section"
            };
            let hint = if has_inv_selected {
                "点击装备槽以装载"
            } else if has_equip_selected {
                "点击空白处卸下 / 点击槽位移动"
            } else {
                ""
            };

            let slot_nodes: Vec<VNode> = eq
                .slots
                .iter()
                .map(|slot| {
                    let idx = slot.slot_index.to_string();
                    let filled = slot.item.is_some();
                    let inner_cls = if filled {
                        "equip-slot-inner filled"
                    } else {
                        "equip-slot-inner empty"
                    };
                    let is_selected_slot = selected
                        .as_ref()
                        .map(|s| !s.from_inventory && s.slot_index == Some(slot.slot_index))
                        .unwrap_or(false);
                    let overlay_cls = if is_selected_slot {
                        "equip-slot-overlay selected"
                    } else if has_inv_selected {
                        "equip-slot-overlay can-equip"
                    } else {
                        "equip-slot-overlay"
                    };
                    let inner = match &slot.item {
                        Some(item) => {
                            let img_src = if item.image.contains('.') {
                                item.image.clone()
                            } else {
                                format!("{}.gif", item.image)
                            };
                            let src = format!("{IMG_PATH}/item/{img_src}");
                            let alt = item.name.clone();
                            let title = item.name.clone();
                            rsx! {
                                img { class: "equip-icon", src: src, alt: alt, title: title }
                            }
                        }
                        None => rsx! { span { class: "equip-plus", "＋" } },
                    };
                    rsx! {
                        div { class: "equip-slot",
                            "data-pm-eq-slot": idx,
                            div { class: overlay_cls }
                            div { class: inner_cls, ..vec![inner] }
                        }
                    }
                })
                .collect();

            let inv_nodes: Vec<VNode> = eq
                .owned
                .iter()
                .filter(|item| item.available_count > 0 || item.equipped_count > 0)
                .map(|item| {
                    let mid = item.myitem_id.to_string();
                    let is_equipped = item.is_equipped || item.equipped_count > 0;
                    let is_selected_item = selected
                        .as_ref()
                        .map(|s| s.from_inventory && s.myitem_id == item.myitem_id)
                        .unwrap_or(false);
                    let wrapper_cls = if is_equipped {
                        "inventory-item-wrapper equipped"
                    } else if is_selected_item {
                        "inventory-item-wrapper selected"
                    } else {
                        "inventory-item-wrapper"
                    };
                    let img_src = if item.image.contains('.') {
                        item.image.clone()
                    } else {
                        format!("{}.gif", item.image)
                    };
                    let src = format!("{IMG_PATH}/item/{img_src}");
                    let alt = item.name.clone();
                    let badge = if item.is_equipped {
                        rsx! { div { class: "equipped-badge", "已装备" } }
                    } else if item.equipped_count > 0 {
                        rsx! { div { class: "equipped-badge", "其他宠物使用中" } }
                    } else {
                        VNode::empty()
                    };
                    let count = if item.available_count > 1 {
                        let c = item.available_count.to_string();
                        rsx! { div { class: "item-count-badge", "{c}" } }
                    } else {
                        VNode::empty()
                    };
                    let title = item.bonus_summary();
                    rsx! {
                        div { class: wrapper_cls,
                            div {
                                class: "inventory-item",
                                title: title,
                                "data-pm-eq-inv-item": mid,
                                div { class: "inventory-item-icon",
                                    img { src: src, alt: alt }
                                }
                                ..vec![badge, count]
                            }
                        }
                    }
                })
                .collect();

            let inv_body = if inv_nodes.is_empty() {
                rsx! { div { class: "inventory-empty", "暂无可用装备物品" } }
            } else {
                rsx! {
                    for i in 0..inv_nodes.len() {
                        ..vec![inv_nodes[i].clone()]
                    }
                }
            };

            let loading_overlay = if loading {
                rsx! { div { class: "equipment-loading-overlay", "处理中..." } }
            } else {
                VNode::empty()
            };

            rsx! {
                div { class: "equipment-tab",
                    ..vec![loading_overlay]
                    div { class: "equipment-slots-row",
                        for i in 0..slot_nodes.len() {
                            ..vec![slot_nodes[i].clone()]
                        }
                    }
                    div {
                        class: section_cls,
                        "data-pm-eq-inv-section": "1",
                        div { class: "inventory-header",
                            span { class: "inventory-title", "物品栏" }
                            span { class: "equip-hint", "{hint}" }
                        }
                        div { class: "equipment-inventory-grid",
                            ..vec![inv_body]
                        }
                    }
                }
            }
        }
    }
}

/// 宠物选择条（列表 ≤ 1 时不渲染，避免无意义交互）。
fn pet_selector(list: &[crate::state::PokemonBasic], selected_id: u64, st: &AppState) -> VNode {
    if list.len() <= 1 {
        return VNode::empty();
    }
    let _ = st;
    let chips: Vec<VNode> = list
        .iter()
        .map(|p| {
            let id = p.id.to_string();
            let active = p.id == selected_id;
            let cls = if active {
                "pet-chip active"
            } else {
                "pet-chip"
            };
            let label = p.display_name();
            let level = format!("Lv.{}", p.level);
            rsx! {
                button {
                    class: cls,
                    "data-pm-my-pm-select": id,
                    span { class: "pet-chip-name", "{label}" }
                    span { class: "pet-chip-level", "{level}" }
                }
            }
        })
        .collect();
    rsx! {
        div { class: "mypm-tab-menu",
            for i in 0..chips.len() {
                ..vec![chips[i].clone()]
            }
        }
    }
}

fn stats_panel(
    st: &AppState,
    d: &crate::state::PokemonDetail,
    editing: bool,
    edit_value: &str,
    saving: bool,
    save_error: &Option<String>,
    save_success: bool,
) -> VNode {
    let _ = st;
    let display_name = d
        .nickname
        .clone()
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| d.name.clone());
    let has_distinct_nickname = d
        .nickname
        .as_ref()
        .map(|n| !n.is_empty() && n != &d.name)
        .unwrap_or(false);

    let no = format!("No {}", d.type_id);
    let pid = format!("ID {}", d.id);
    let level = format!("Lv.{}", d.level);
    let gender_cls = match d.gender {
        1 => "pokemon-gender gender-male",
        2 => "pokemon-gender gender-female",
        _ => "pokemon-gender gender-none",
    };
    let gender_sym = match d.gender {
        1 => "♂",
        2 => "♀",
        _ => "—",
    };
    let type1_cls = format!("badge badge-type-{}", d.type_1);
    let type1 = d.type_1.clone();
    let type2 = d.type_2.clone();
    let show_state = d.state != 1;
    let state_cls = format!("state-label state-{}", d.state_class);
    let state_text = d.state_text.clone();

    let hp_pct = if d.max_hp > 0 {
        ((d.hp.max(0) as f64) / (d.max_hp as f64) * 100.0) as u32
    } else {
        0
    };
    let hp_fill = if d.max_hp <= 0 || d.hp <= 0 {
        "bar-fill hp-low"
    } else {
        let pct = d.hp as f64 / d.max_hp as f64;
        if pct > 0.5 {
            "bar-fill hp-high"
        } else if pct > 0.2 {
            "bar-fill hp-mid"
        } else {
            "bar-fill hp-low"
        }
    };
    let hp_text = format!("{} / {}", d.hp, d.max_hp);
    let hp_style = format!("width: {hp_pct}%");

    let exp_in_level = d.exp.saturating_sub(d.exp_for_current_level);
    let exp_range = d.exp_for_next_level.saturating_sub(d.exp_for_current_level);
    let exp_pct = if exp_range > 0 {
        ((exp_in_level as f64 / exp_range as f64) * 100.0).min(100.0) as u32
    } else {
        100
    };
    let exp_text = format!("{exp_in_level} / {exp_range}");
    let exp_style = format!("width: {exp_pct}%");
    let affection = d.affection.to_string();

    // 品质：优先用载荷字段，缺失时按总个体值判定。
    let iv_block = d.ivs.map(|ivs| {
        let total: u64 = ivs.iter().sum();
        let rank = d.quality_rank.unwrap_or_else(|| quality_rank(total));
        let grade = d
            .quality_grade
            .clone()
            .unwrap_or_else(|| quality_grade(rank).to_string());
        (ivs, total, rank, grade)
    });
    let badge_grade = iv_block
        .as_ref()
        .map(|(_, _, _, g)| g.clone())
        .filter(|g| !g.is_empty());
    let badge_rank = iv_block
        .as_ref()
        .map(|(_, _, r, _)| *r)
        .unwrap_or_else(|| quality_rank(0));

    let stats = d.stats.unwrap_or([0; 5]);
    let radar = radar_chart(stats[0], stats[1], stats[2], stats[3], stats[4]);

    // 昵称区：编辑态 / 展示态
    let name_area = if editing {
        let err_block = save_error
            .as_ref()
            .map(|e| {
                let msg = e.clone();
                rsx! { p { class: "error-message", role: "alert", "{msg}" } }
            })
            .unwrap_or_else(|| VNode::empty());
        rsx! {
            div { class: "pokemon-name-edit-wrapper",
                input {
                    class: "pokemon-name-input",
                    "aria-label": "宠物昵称",
                    value: edit_value,
                    maxlength: NICKNAME_MAX,
                    placeholder: "输入昵称（最多 14 字）",
                    disabled: saving,
                    "data-pm-my-pm-nick-input": "1"
                }
                div { class: "edit-actions-inline",
                    button {
                        class: "edit-action-btn save",
                        r#type: "button",
                        title: "保存",
                        disabled: saving,
                        "data-pm-my-pm-nick-save": "1",
                        "✓"
                    }
                    button {
                        class: "edit-action-btn cancel",
                        r#type: "button",
                        title: "取消",
                        disabled: saving,
                        "data-pm-my-pm-nick-cancel": "1",
                        "×"
                    }
                }
                ..vec![err_block]
            }
        }
    } else {
        let name_node = if has_distinct_nickname {
            let original_text = format!(" ({})", d.name);
            let nick = display_name.clone();
            rsx! {
                span { class: "pokemon-name-with-original",
                    span { class: "pokemon-name", "{nick}" }
                    span { class: "original-name", "{original_text}" }
                }
            }
        } else {
            let n = display_name.clone();
            rsx! { span { class: "pokemon-name", "{n}" } }
        };
        let edit_seed = display_name.clone();
        let _ = edit_seed;
        let mut nodes: Vec<VNode> = Vec::with_capacity(2);
        nodes.push(name_node);
        nodes.push(rsx! {
            button {
                class: "edit-icon-btn",
                r#type: "button",
                "aria-label": "修改昵称",
                "data-pm-my-pm-nick-edit": "1",
                "✎"
            }
        });
        let container: Vec<VNode> = nodes;
        rsx! {
            for i in 0..container.len() {
                ..vec![container[i].clone()]
            }
        }
    };

    let state_block = if show_state {
        rsx! { span { class: state_cls, "{state_text}" } }
    } else {
        VNode::empty()
    };
    let type2_block = match type2 {
        Some(t2) => {
            let cls = format!("badge badge-type-{t2}");
            rsx! { span { class: cls, "{t2}" } }
        }
        None => VNode::empty(),
    };
    let shiny_block = if d.is_shiny {
        rsx! { span { class: "pet-mark", "★" } }
    } else {
        VNode::empty()
    };
    let god_block = if d.is_legendary {
        rsx! { span { class: "pet-mark", "◆" } }
    } else {
        VNode::empty()
    };
    let quality_badge = match badge_grade {
        Some(g) => {
            let cls = format!("quality-badge quality-rank-{badge_rank}");
            rsx! { span { class: cls, title: "品质由六项个体值总和判定", "{g}" } }
        }
        None => VNode::empty(),
    };
    let name_row: Vec<VNode> = vec![
        name_area,
        rsx! { span { class: gender_cls, "{gender_sym}" } },
        shiny_block,
        god_block,
        quality_badge,
        rsx! { span { class: "heal-pm-level", "{level}" } },
    ];
    let name_row = rsx! {
        div { class: "pokemon-name-row",
            for i in 0..name_row.len() {
                ..vec![name_row[i].clone()]
            }
        }
    };
    let success_block = if save_success {
        rsx! { p { class: "save-success", "昵称已更新" } }
    } else {
        VNode::empty()
    };

    let iv_rows = match iv_block {
        Some((ivs, total, rank, _)) => {
            let total_text = if rank == UNKNOWN_RANK {
                format!("{total}")
            } else {
                format!("{total} / 186")
            };
            let overflow = if rank == UNKNOWN_RANK {
                rsx! { span { class: "iv-overflow-hint", "超出 186 上限（历史个体值）" } }
            } else {
                VNode::empty()
            };
            let chips: Vec<VNode> = ["HP", "攻击", "防御", "特攻", "特防", "速度"]
                .iter()
                .enumerate()
                .map(|(i, label)| {
                    let text = format!("{label} {}", ivs[i]);
                    rsx! { span { class: "iv-chip", "{text}" } }
                })
                .collect();
            rsx! {
                div { class: "iv-summary-row",
                    span { class: "affection-label", "总个体值" }
                    span { class: "affection-value", "{total_text}" }
                    ..vec![overflow]
                }
                div { class: "iv-detail-row",
                    for i in 0..chips.len() {
                        ..vec![chips[i].clone()]
                    }
                }
            }
        }
        None => VNode::empty(),
    };

    rsx! {
        div { class: "stats-tab-layout",
            div { class: "stats-header",
                div { class: "info-tags",
                    span { class: "info-tag", "{no}" }
                    span { class: "info-tag", "{pid}" }
                }
                div { class: "tags-group",
                    div { class: "type-badges",
                        span { class: type1_cls, "{type1}" }
                        ..vec![type2_block]
                    }
                    ..vec![state_block]
                }
            }
            ..vec![name_row, success_block]
            div { class: "stats-bottom-row",
                div { class: "stats-bars",
                    div { class: "stat-bar-row",
                        span { class: "bar-label", "HP" }
                        div { class: "bar-track hp-bar",
                            div { class: hp_fill, style: hp_style }
                            span { class: "bar-text", "{hp_text}" }
                        }
                    }
                    div { class: "stat-bar-row with-sub-info",
                        span { class: "bar-label", "EXP" }
                        div { class: "bar-track exp-bar",
                            div { class: "bar-fill exp-fill", style: exp_style }
                            span { class: "bar-text", "{exp_text}" }
                        }
                    }
                    div { class: "affection-row",
                        span { class: "affection-label", "好感度" }
                        span { class: "affection-value", "{affection}" }
                    }
                    ..vec![iv_rows]
                    div { class: "tool-buttons-col",
                        button { class: "ev-calculator-btn", "data-pm-my-pm-ev-open": "1", "努力值计算器" }
                        button { class: "ev-calculator-btn", "data-pm-my-pm-evo-path-open": "1", "进化路径" }
                        button { class: "ev-calculator-btn", "data-pm-my-pm-evo-open": "1", "进化" }
                    }
                }
                ..vec![radar]
            }
        }
    }
}

/// 五维雷达图（SVG，对齐 Dioxus 版 RadarChart 的几何：五边形网格、
/// 最小半径 20%、上限 300 点）。
fn radar_chart(attack: i64, defense: i64, sp_attack: i64, sp_defense: i64, speed: i64) -> VNode {
    let cx = 140.0f64;
    let cy = 140.0f64;
    let max_r = 70.0f64;
    let min_r = max_r * 0.2;
    let max_stat = 300.0f64;

    let angles: Vec<f64> = (0..5)
        .map(|i| (-90.0 + i as f64 * 72.0).to_f64_radians())
        .collect();

    let grid_paths: Vec<String> = [1.0, 0.6, 0.3]
        .iter()
        .map(|scale| {
            let r = min_r + scale * (max_r - min_r);
            let points: Vec<String> = angles
                .iter()
                .map(|a| format!("{:.1},{:.1}", cx + r * a.cos(), cy + r * a.sin()))
                .collect();
            format!("M {} Z", points.join(" L "))
        })
        .collect();

    let values = [attack, defense, sp_attack, sp_defense, speed];
    let data_path = {
        let points: Vec<String> = values
            .iter()
            .zip(angles.iter())
            .map(|(v, a)| {
                let ratio = (*v as f64 / max_stat).min(1.0).max(0.0);
                let r = min_r + ratio * (max_r - min_r);
                format!("{:.1},{:.1}", cx + r * a.cos(), cy + r * a.sin())
            })
            .collect();
        format!("M {} Z", points.join(" L "))
    };

    let labels = ["攻击", "防御", "特攻", "特防", "速度"];
    let label_nodes: Vec<VNode> = angles
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let r = max_r + 15.0;
            let x = cx + r * a.cos();
            let y = cy + r * a.sin();
            let text = labels[i];
            let x_s = format!("{x:.1}");
            let y_s = format!("{y:.1}");
            rsx! {
                text {
                    x: x_s,
                    y: y_s,
                    text_anchor: "middle",
                    dominant_baseline: "middle",
                    font_size: "10",
                    fill: "#666",
                    "{text}"
                }
            }
        })
        .collect();

    let axis_nodes: Vec<VNode> = angles
        .iter()
        .map(|a| {
            let x2 = format!("{:.1}", cx + max_r * a.cos());
            let y2 = format!("{:.1}", cy + max_r * a.sin());
            rsx! {
                line {
                    x1: "140",
                    y1: "140",
                    x2: x2,
                    y2: y2,
                    stroke: "#e0e0e0",
                    stroke_width: "0.5"
                }
            }
        })
        .collect();

    rsx! {
        div { class: "stats-radar-chart",
            svg {
                class: "radar-svg responsive",
                view_box: "0 0 280 280",
                width: "200px",
                height: "200px",
                for i in 0..grid_paths.len() {
                    path {
                        d: grid_paths[i].clone(),
                        fill: "none",
                        stroke: "#e0e0e0",
                        stroke_width: "1"
                    }
                }
                for i in 0..axis_nodes.len() {
                    ..vec![axis_nodes[i].clone()]
                }
                path {
                    d: data_path,
                    fill: "rgba(52, 152, 219, 0.25)",
                    stroke: "#3498db",
                    stroke_width: "2"
                }
                for i in 0..label_nodes.len() {
                    ..vec![label_nodes[i].clone()]
                }
            }
        }
    }
}

/// 角度转弧度（`f64` 无内置方法，避免额外依赖）。
trait ToRadians {
    fn to_f64_radians(self) -> f64;
}

impl ToRadians for f64 {
    fn to_f64_radians(self) -> f64 {
        self * std::f64::consts::PI / 180.0
    }
}

// ---------------------------------------------------------------------------
// 技能标签页面板与学习弹窗
// ---------------------------------------------------------------------------

/// 技能标签页面板：技能卡（点击展开遗忘）+ 空槽（点击学习）。
fn skills_panel(st: &AppState, pm: &crate::state::PokemonBasic) -> VNode {
    let loading = st.my_pm.skill_loading.get();
    let selected = st.my_pm.skill_selected.get();

    let valid: Vec<&crate::state::PokemonSkill> =
        pm.skills.iter().filter(|s| s.type_id > 0).collect();
    let empty_count = 4usize.saturating_sub(valid.len());

    let mut cards: Vec<VNode> = Vec::new();
    for skill in &valid {
        let tid = skill.type_id.to_string();
        let is_selected = selected == Some(skill.type_id);
        let card_cls = if is_selected {
            "skill-card-v2 skill-slot-clickable selected"
        } else {
            "skill-card-v2 skill-slot-clickable"
        };
        let name = if skill.name.is_empty() {
            format!("技能 {}", skill.type_id)
        } else {
            skill.name.clone()
        };
        let pp_head = format!("PP {}", skill.pp);
        let pp_tail = if skill.max_pp > 0 {
            format!("/{}", skill.max_pp)
        } else {
            format!("/{}", skill.pp)
        };
        let type_badge_cls = format!("badge badge-type-{}", skill.skill_type);
        let cat_badge_cls = format!("badge badge-cat-{}", skill.category);
        let power_label = match skill.category.as_str() {
            "特攻" | "特殊" => "特攻",
            "物攻" | "物理" => "物攻",
            _ => "威力",
        };
        let power_text = format!("{}：{}", power_label, skill.power);
        let pp_text = format!("PP：{}/{}", skill.pp, skill.max_pp);
        let level_text = if skill.level > 0 {
            Some(format!("Lv {}", skill.level))
        } else {
            None
        };
        let stype = skill.skill_type.clone();
        let scat = skill.category.clone();

        let forget_block = if is_selected {
            let btn_cls = if skill.can_forget {
                "skill-forget-btn"
            } else {
                "skill-forget-btn disabled"
            };
            rsx! {
                button {
                    class: btn_cls,
                    r#type: "button",
                    disabled: loading,
                    "data-pm-skill-forget": tid.clone(),
                    "遗忘此技能"
                }
            }
        } else {
            VNode::empty()
        };

        cards.push(rsx! {
            div { class: card_cls,
                "data-pm-skill-card": tid.clone(),
                div { class: "skill-card-top",
                    span { class: "skill-name", "{name}" }
                    span { class: "skill-pp",
                        "{pp_head}"
                        strong { "{pp_tail}" }
                    }
                }
                div { class: "skill-card-bottom",
                    span { class: type_badge_cls, "{stype}" }
                    span { class: cat_badge_cls, "{scat}" }
                    span { class: "skill-power", "{power_text}" }
                    span { class: "skill-pp", "{pp_text}" }
                    if let Some(_lv) = level_text {
                        span { class: "skill-level", "{_lv}" }
                    }
                }
                ..vec![forget_block]
            }
        });
    }
    for _ in 0..empty_count {
        cards.push(rsx! {
            button {
                r#type: "button",
                class: "skill-slot-empty",
                disabled: loading,
                "data-pm-skill-learn-slot": "1",
                div { class: "skill-slot-plus", "+" }
                div { class: "skill-slot-hint", "点击学习新技能" }
            }
        });
    }

    let loading_hint = if loading {
        rsx! { div { role: "status", class: "skill-loading-hint", "正在更新技能..." } }
    } else {
        VNode::empty()
    };

    rsx! {
        div { class: "skills-grid-container",
            ..vec![loading_hint]
            div { class: "skills-grid",
                for i in 0..cards.len() {
                    ..vec![cards[i].clone()]
                }
            }
        }
    }
}

/// 学习弹窗：可学习 + 等级不足两段列表。
fn learn_modal_vnode(st: &AppState) -> VNode {
    let level = st
        .center
        .pokemons
        .get()
        .and_then(|r| r.ok())
        .and_then(|list| {
            list.iter()
                .find(|p| Some(p.id) == st.my_pm.selected_id.get())
                .or_else(|| list.first())
                .map(|p| p.level)
        })
        .unwrap_or(0);
    let title = format!("学习新技能 (Lv.{level})");

    let body = match st.my_pm.learnable.get() {
        None => rsx! { div { class: "skill-learn-loading", "加载技能列表中..." } },
        Some(Err(error)) => {
            let msg = format!("技能加载失败：{error}");
            rsx! {
                div { role: "alert", class: "error-message", "{msg}" }
                button { class: "btn btn-primary", "data-pm-learn-reload": "1", "重新加载技能" }
            }
        }
        Some(Ok((available, locked))) if available.is_empty() && locked.is_empty() => {
            rsx! { div { class: "skill-learn-empty", "暂无可学习的技能" } }
        }
        Some(Ok((available, locked))) => {
            let mut rows: Vec<VNode> = Vec::new();
            if !available.is_empty() {
                rows.push(rsx! { div { class: "skill-section-title", "可学习" } });
                for skill in &available {
                    rows.push(learn_item(skill, st.my_pm.skill_loading.get()));
                }
            }
            if !locked.is_empty() {
                rows.push(rsx! { div { class: "skill-section-title", "等级不足" } });
                for skill in &locked {
                    rows.push(learn_item(skill, true));
                }
            }
            rsx! {
                for i in 0..rows.len() {
                    ..vec![rows[i].clone()]
                }
            }
        }
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-learn-close": "1",
            div { class: "modal-content",
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close", "data-pm-learn-close": "1", "×" }
                }
                div { class: "modal-body",
                    div { class: "skill-learn-content", ..vec![body] }
                }
            }
        }
    }
}

fn learn_item(skill: &crate::state::LearnableSkill, loading: bool) -> VNode {
    let sid = skill.id.to_string();
    let cls = if skill.is_available {
        "skill-learn-item"
    } else {
        "skill-learn-item disabled"
    };
    let type_cls = format!("badge badge-type-{}", skill.skill_type);
    let cat_cls = format!("badge badge-cat-{}", skill.category);
    let power_text = if skill.power > 0 {
        Some(format!("威力：{}", skill.power))
    } else {
        None
    };
    let pp_text = format!("PP: {}", skill.max_pp);
    let hint = if skill.is_available {
        "学习".to_string()
    } else {
        format!("需要 Lv.{}", skill.required_level)
    };
    let sname = skill.name.clone();
    let stype = skill.skill_type.clone();
    let scat = skill.category.clone();
    let sdesc = skill.description.clone();

    rsx! {
        button {
            r#type: "button",
            class: cls,
            disabled: !skill.is_available || loading,
            "data-pm-learn-item": sid,
            div { class: "skill-learn-item-header",
                span { class: "skill-learn-name", "{sname}" }
                span { class: "skill-learn-hint", "{hint}" }
            }
            div { class: "skill-learn-item-body",
                span { class: type_cls, "{stype}" }
                span { class: cat_cls, "{scat}" }
                if let Some(_p) = power_text {
                    span { class: "skill-power", "{_p}" }
                }
                span { class: "skill-pp", "{pp_text}" }
            }
            if !skill.description.is_empty() {
                div { class: "skill-description", "{sdesc}" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 属性页弹窗：努力值计算器 / 进化路径 / 进化
// ---------------------------------------------------------------------------

const EV_STAT_NAMES: [&str; 6] = ["HP", "攻击", "防御", "特攻", "特防", "速度"];

/// 努力值计算器（纯客户端）：当前值 + 计划追加，单项 ≤255、总计 ≤510。
fn ev_calculator_modal(st: &AppState) -> VNode {
    let current = st.my_pm.ev_current.get();
    let added = st.my_pm.ev_added.get();
    let totals: Vec<u32> = current
        .iter()
        .zip(added.iter())
        .map(|(c, a)| c + a)
        .collect();
    let grand: u32 = totals.iter().sum();
    let valid = totals.iter().all(|t| *t <= 255) && grand <= 510;
    let (status_cls, status_text) = if valid {
        ("ev-status-valid", "配置有效")
    } else {
        ("ev-status-invalid", "配置无效（超过限制）")
    };

    let header_cells: Vec<VNode> = EV_STAT_NAMES
        .iter()
        .map(|n| {
            let label = *n;
            rsx! { div { class: "ev-cell ev-header", "{label}" } }
        })
        .collect();

    let cur_cells: Vec<VNode> = (0..6)
        .map(|i| {
            let attr = format!("cur-{i}");
            let value = current[i].to_string();
            rsx! {
                div { class: "ev-cell",
                    input {
                        class: "ev-input",
                        r#type: "number",
                        min: "0",
                        max: "255",
                        value: value,
                        "data-pm-ev-input": attr
                    }
                }
            }
        })
        .collect();
    let add_cells: Vec<VNode> = (0..6)
        .map(|i| {
            let attr = format!("add-{i}");
            let value = added[i].to_string();
            rsx! {
                div { class: "ev-cell",
                    input {
                        class: "ev-input",
                        r#type: "number",
                        min: "0",
                        max: "255",
                        value: value,
                        "data-pm-ev-input": attr
                    }
                }
            }
        })
        .collect();
    let total_cells: Vec<VNode> = (0..6)
        .map(|i| {
            let t = totals[i].to_string();
            rsx! { div { class: "ev-cell", "{t}" } }
        })
        .collect();
    let grand_text = grand.to_string();
    let added_sum: u32 = added.iter().sum();
    let added_sum_text = added_sum.to_string();

    rsx! {
        div { class: "modal-overlay", "data-pm-ev-close": "1",
            div { class: "modal-content",
                div { class: "modal-header",
                    h3 { "努力值计算器" }
                    button { class: "modal-close", "data-pm-ev-close": "1", "×" }
                }
                div { class: "modal-body",
                    div { class: "ev-calculator",
                        div { class: "ev-row",
                            div { class: "ev-cell ev-row-label", "属性" }
                            for i in 0..header_cells.len() {
                                ..vec![header_cells[i].clone()]
                            }
                        }
                        div { class: "ev-row",
                            div { class: "ev-cell ev-row-label", "当前" }
                            for i in 0..cur_cells.len() {
                                ..vec![cur_cells[i].clone()]
                            }
                        }
                        div { class: "ev-row",
                            div { class: "ev-cell ev-row-label", "追加" }
                            for i in 0..add_cells.len() {
                                ..vec![add_cells[i].clone()]
                            }
                            div { class: "ev-cell ev-total", "{added_sum_text}" }
                        }
                        div { class: "ev-row",
                            div { class: "ev-cell ev-row-label", "合计" }
                            for i in 0..total_cells.len() {
                                ..vec![total_cells[i].clone()]
                            }
                            div { class: "ev-cell ev-total", "{grand_text}" }
                        }
                        div { class: "ev-status",
                            span { class: status_cls, "{status_text}" }
                        }
                    }
                }
            }
        }
    }
}

/// 进化路径弹窗：进化 / 退化两个 tab。
fn evo_path_modal(st: &AppState) -> VNode {
    let forward = st.my_pm.evo_path_forward.get();
    let fwd_cls = if forward {
        "evo-path-tab active"
    } else {
        "evo-path-tab"
    };
    let bwd_cls = if forward {
        "evo-path-tab"
    } else {
        "evo-path-tab active"
    };

    let body = match st.my_pm.evo_path.get() {
        None => rsx! { div { class: "evo-path-loading", "正在加载进化路径..." } },
        Some(Err(error)) => {
            let msg = format!("加载失败：{error}");
            rsx! { div { class: "evo-path-error", "{msg}" } }
        }
        Some(Ok(data)) => {
            let nodes = if forward {
                &data.forward
            } else {
                &data.backward
            };
            if nodes.is_empty() {
                let hint = if forward {
                    "没有进化路线"
                } else {
                    "没有退化路线"
                };
                rsx! { div { class: "evo-path-empty", "{hint}" } }
            } else {
                let rows: Vec<VNode> = nodes
                    .iter()
                    .map(|n| {
                        let sprite = format!("{IMG_PATH}/spm/{}.gif", n.id);
                        let name = n.name.clone();
                        let alt_name = name.clone();
                        let cond = if n.condition_display.is_empty() {
                            "—".to_string()
                        } else {
                            n.condition_display.clone()
                        };
                        rsx! {
                            div { class: "evo-path-node",
                                img { class: "evo-path-sprite", src: sprite, alt: alt_name }
                                div { class: "evo-path-node-info",
                                    span { class: "evo-path-name", "{name}" }
                                    div { class: "evo-path-condition", "{cond}" }
                                }
                            }
                        }
                    })
                    .collect();
                rsx! {
                    div { class: "evo-path-content",
                        for i in 0..rows.len() {
                            ..vec![rows[i].clone()]
                        }
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-evo-path-close": "1",
            div { class: "modal-content",
                div { class: "modal-header",
                    h3 { "进化路径" }
                    button { class: "modal-close", "data-pm-evo-path-close": "1", "×" }
                }
                div { class: "modal-body",
                    div { class: "evo-path-tabs",
                        button { class: fwd_cls, "data-pm-evo-path-tab": "forward", "进化路线" }
                        button { class: bwd_cls, "data-pm-evo-path-tab": "backward", "退化路线" }
                    }
                    ..vec![body]
                }
            }
        }
    }
}

/// 进化弹窗：可进化路线（条件 + 状态 + 执行按钮）。
fn evolution_modal(st: &AppState) -> VNode {
    let evolving = st.my_pm.evolving.get();

    let body = match st.my_pm.evo_check.get() {
        None => rsx! { div { class: "evo-path-loading", "正在查询进化条件..." } },
        Some(Err(error)) => {
            let msg = format!("查询失败: {error}");
            rsx! { div { class: "evo-path-error", "{msg}" } }
        }
        Some(Ok(list)) if list.is_empty() => {
            rsx! { div { class: "evo-path-empty", "该宝可梦没有可用的进化路线" } }
        }
        Some(Ok(list)) => {
            let rows: Vec<VNode> = list
                .iter()
                .map(|evo| {
                    let sprite = format!("{IMG_PATH}/spm/{}.gif", evo.to_id);
                    let name = evo.to_name.clone();
                    let alt_name = name.clone();
                    let condition = evo
                        .condition_display
                        .clone()
                        .unwrap_or_else(|| "—".to_string());
                    let status = if evo.can_evolve {
                        "满足进化条件".to_string()
                    } else if evo.reason.is_empty() {
                        "条件未满足".to_string()
                    } else {
                        evo.reason.clone()
                    };
                    let btn = if evo.can_evolve {
                        let to_id = evo.to_id.to_string();
                        let to_name = evo.to_name.clone();
                        rsx! {
                            button {
                                class: "ev-calculator-btn",
                                disabled: evolving,
                                "data-pm-evo-do": to_id,
                                "data-pm-evo-do-name": to_name,
                                "进化"
                            }
                        }
                    } else {
                        VNode::empty()
                    };
                    rsx! {
                        div { class: "evo-path-node",
                            img { class: "evo-path-sprite", src: sprite, alt: alt_name }
                            div { class: "evo-path-node-info",
                                span { class: "evo-path-name", "{name}" }
                                div { class: "evo-path-condition", "{condition}" }
                                div { class: "evo-path-condition", "{status}" }
                            }
                            ..vec![btn]
                        }
                    }
                })
                .collect();
            rsx! {
                div { class: "evo-path-content",
                    for i in 0..rows.len() {
                        ..vec![rows[i].clone()]
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-evo-modal-close": "1",
            div { class: "modal-content",
                div { class: "modal-header",
                    h3 { "进化" }
                    button { class: "modal-close", "data-pm-evo-modal-close": "1", "×" }
                }
                div { class: "modal-body", ..vec![body] }
            }
        }
    }
}
