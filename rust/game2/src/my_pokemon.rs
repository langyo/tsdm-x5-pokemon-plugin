//! 我的宝可梦页（tairitsu 版，移植自 rust/game/src/pages/my_pokemon.rs 的
//! 「属性」标签页）。同名同类直用 game.css（.page-my-pokemon / .mypm-card /
//! .stats-tab-layout / .radar-svg …）。
//!
//! 本阶段范围：宠物选择、属性面板（编号/属性/状态/性别/闪光/品质）、HP 与
//! EXP 条、好感度、总个体值与六项个体值、五维雷达图、昵称编辑。
//!
//! 未接入：装备 / 技能标签页、努力值计算器、进化路径与进化弹窗——这些依赖
//! 各自端点的写流程，随后续阶段补齐；对应按钮不渲染，避免给出无法完成的入口。

use hikari_components::{Button, ButtonProps};
use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use crate::app_state::AppState;
use crate::icon::{Icon, IconName, IconProps};
use crate::quality::{quality_grade, quality_rank, UNKNOWN_RANK};
use crate::toast;

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
// 装备标签页：动作入口（本页元素的 onclick 调用）
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

/// 「重新加载装备」：清掉失败态后重拉（对齐 Dioxus 版
/// `equipment_resource.restart()`）。
fn retry_equipment_load(state: &AppState) {
    state.my_pm.equipment.inner().set(None);
    crate::request_remount();
    ensure_equipment(state);
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
        toast::show(kind, msg);
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
    // 对齐 Dioxus 版：section 的点击只处理「已装备物品被选中」的卸下，
    // 背包物品的选择不在此取消（点了物品的事件会冒泡到这里，取消逻辑会
    // 把刚选中的立刻清掉）。背包选择用再点一次同一件物品来取消。
    let Some(sel) = my.equip_selected.get() else {
        return;
    };
    if sel.from_inventory {
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
// 技能标签页：动作入口（本页元素的 onclick 调用）
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

/// 「重新加载技能」：清掉失败态后重开学习弹窗（对齐 Dioxus 版
/// `resource.restart()`）。
fn retry_learnable_load(state: &AppState) {
    state.my_pm.learnable.inner().set(None);
    crate::request_remount();
    open_learn_modal(state);
}

/// 技能动作收尾：清守卫、刷列表（技能槽来自 list 载荷）。
fn finish_skill_action(
    state: &AppState,
    message: Option<(crate::state::ToastKind, String)>,
    close_modal: bool,
) {
    let my = state.my_pm.clone();
    if let Some((kind, msg)) = message {
        toast::show(kind, msg);
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
        toast::show(
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
/// 输入一个待分配值。对齐 Dioxus 版 create_handler：解析成 u32 且 ≤255 才
/// 接受（超限/解析失败都保持旧值）；`added` 参数保留给未来可能恢复的
/// 「已分配」输入位。
pub fn set_ev_value(state: &AppState, idx: usize, added: bool, raw: &str) {
    if idx >= 6 {
        return;
    }
    let Ok(value) = raw.trim().parse::<u32>() else {
        return;
    };
    if value > 255 {
        return;
    }
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
                    toast::show(
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
                    toast::show(crate::state::ToastKind::Error, format!("进化失败：{e}"));
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
        // 标签页菜单。旧版（pages/my_pokemon.rs）：选中的页签两侧各有一个箭头，
        // 文字包在 span 里。
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
        // 标签页按钮各绑自己的处理函数（状态句柄克隆进闭包）。
        let st_tab_stats = st.clone();
        let st_tab_equip = st.clone();
        let st_tab_skills = st.clone();
        let tab_menu = rsx! {
            div { class: "mypm-tab-menu",
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: (stats_cls).to_string(),
                    onclick: crate::button::click(move || switch_tab(&st_tab_stats, crate::state::MyPokemonTab::Stats)),
                    if tab == crate::state::MyPokemonTab::Stats {
                        span { class: "tab-arrow left", ">" }
                    }
                    span { "属性" }
                    if tab == crate::state::MyPokemonTab::Stats {
                        span { class: "tab-arrow right", "<" }
                    }
                                } }, [("data-test-my-pm-tab", ("stats").to_string())]) }
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: (equip_cls).to_string(),
                    onclick: crate::button::click(move || switch_tab(&st_tab_equip, crate::state::MyPokemonTab::Equipment)),
                    if tab == crate::state::MyPokemonTab::Equipment {
                        span { class: "tab-arrow left", ">" }
                    }
                    span { "装备" }
                    if tab == crate::state::MyPokemonTab::Equipment {
                        span { class: "tab-arrow right", "<" }
                    }
                                } }, [("data-test-my-pm-tab", ("equipment").to_string())]) }
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: (skills_cls).to_string(),
                    onclick: crate::button::click(move || switch_tab(&st_tab_skills, crate::state::MyPokemonTab::Skills)),
                    if tab == crate::state::MyPokemonTab::Skills {
                        span { class: "tab-arrow left", ">" }
                    }
                    span { "技能" }
                    if tab == crate::state::MyPokemonTab::Skills {
                        span { class: "tab-arrow right", "<" }
                    }
                                } }, [("data-test-my-pm-tab", ("skills").to_string())]) }
            }
        };

        // 精灵图区（装备页时两侧展示六维加成合计：左 HP/攻击/防御，右 特攻/特防/速度，
        // 两个容器在装备页总是渲染——旧版 BonusTagsLeft / BonusTagsRight）。
        let (bonus_left, bonus_right) = if tab == crate::state::MyPokemonTab::Equipment {
            match st.my_pm.equipment.get().and_then(|r| r.ok()) {
                Some(eq) => {
                    let total = eq.total_bonuses();
                    (bonus_tags(&total, true), bonus_tags(&total, false))
                }
                None => (VNode::empty(), VNode::empty()),
            }
        } else {
            (VNode::empty(), VNode::empty())
        };
        // 正面精灵图走远程 CDN 的动画大图（旧版 `front_sprite_url`）；alt 是物种名。
        let sprite = crate::state::front_sprite_url(pm.type_id, pm.is_shiny);
        let sprite_alt = pm.name.clone();
        let sprite_area = rsx! {
            div { class: "mypm-main-content",
                div { class: "pokemon-sprite-area",
                    div { class: "pokemon-display-with-tags",
                        ..vec![bonus_left]
                        img { class: "pokemon-sprite-img", src: sprite, alt: sprite_alt }
                        ..vec![bonus_right]
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
                ..vec![tab_menu]
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
                all
            }
        }
    }
}

/// 装备加成合计标签的一侧（旧版 BonusTagsLeft / BonusTagsRight）：容器总是渲染，
/// 里面只列非零的项；`left` 取 HP/攻击/防御，否则特攻/特防/速度。
fn bonus_tags(total: &[i64; 6], left: bool) -> VNode {
    let names = ["HP", "攻击", "防御", "特攻", "特防", "速度"];
    let classes = [
        "bonus-tag-small hp",
        "bonus-tag-small atk",
        "bonus-tag-small def",
        "bonus-tag-small spat",
        "bonus-tag-small spdef",
        "bonus-tag-small spd",
    ];
    let range = if left { 0..3 } else { 3..6 };
    let chips: Vec<VNode> = range
        .filter(|&i| total[i] != 0)
        .map(|i| {
            let text = if total[i] > 0 {
                format!("+{} {}", total[i], names[i])
            } else {
                format!("{} {}", total[i], names[i])
            };
            let c = classes[i].to_string();
            rsx! { span { class: c, "{text}" } }
        })
        .collect();
    let side = if left {
        "side-tags left"
    } else {
        "side-tags right"
    };
    rsx! {
        div { class: side,
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
            let retry_state = st.clone();
            rsx! {
                div { class: "equipment-tab",
                    div { role: "alert", class: "error-message", "{msg}" }
                    { crate::button::hooked(tairitsu_macros::rsx! { Button {
                        class: ("btn btn-primary").to_string(),
                        onclick: crate::button::click(move || retry_equipment_load(&retry_state)),
                        "重新加载装备"
                                        } }, [("data-test-eq-reload", ("1").to_string())]) }
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

            let slot_nodes: Vec<VNode> = eq
                .slots
                .iter()
                .map(|slot| {
                    let idx = slot.slot_index.to_string();
                    let slot_state = st.clone();
                    let slot_index = slot.slot_index;
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
                            "data-test-eq-slot": idx,
                            onclick: move |_| click_slot(&slot_state, slot_index),
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
                    let myitem_id = item.myitem_id;
                    let item_state = st.clone();
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
                                "data-test-eq-inv-item": mid,
                                onclick: move |_| select_inventory_item(&item_state, myitem_id),
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
            let section_state = st.clone();

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
                        "data-test-eq-inv-section": "1",
                        onclick: move |_| unequip_selected_from_section(&section_state),
                        div { class: "inventory-header",
                            span { class: "inventory-title", "物品栏" }
                            if has_inv_selected {
                                span { class: "equip-hint", "点击装备槽以装载" }
                            }
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

fn stats_panel(
    st: &AppState,
    d: &crate::state::PokemonDetail,
    editing: bool,
    edit_value: &str,
    saving: bool,
    save_error: &Option<String>,
    save_success: bool,
) -> VNode {
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
    // 性别：类是 `pokemon-gender male|female`（未知时只有前一个，旧版留了个尾随空格），
    // 符号 ♂ / ♀，未知为空——不是「—」。
    let gender_cls = match d.gender {
        1 => "pokemon-gender male",
        2 => "pokemon-gender female",
        _ => "pokemon-gender ",
    };
    let gender_sym = match d.gender {
        1 => "♂",
        2 => "♀",
        _ => "",
    };
    let type1_cls = format!("badge badge-type-{}", d.type_1);
    let type1 = d.type_1.clone();
    let type2 = d.type_2.clone();
    let show_state = d.state != 1;
    let state_cls = format!("state-label state-{}", d.state_class);
    let state_text = d.state_text.clone();
    // 状态影响说明（旧版是 hover tooltip，这里用 title 呈现同一份数据）。
    let state_detail = crate::pet_state::describe(d.state);

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
    // 名字旁的评级徽章只在载荷带 quality_grade 时出现（旧版如此）；等级取载荷的
    // quality_rank，没有则按个体值总和判定。
    let badge_grade = d.quality_grade.clone().filter(|g| !g.is_empty());
    let badge_rank = d
        .quality_rank
        .unwrap_or_else(|| quality_rank(d.ivs.map(|ivs| ivs.iter().sum()).unwrap_or(0)));

    let stats = d.stats.unwrap_or([0; 5]);
    let radar = radar_chart(d.level, stats[0], stats[1], stats[2], stats[3], stats[4]);
    let ev_state = st.clone();
    let evo_path_state = st.clone();
    let evo_state = st.clone();

    // 昵称区：编辑态 / 展示态
    let name_area = if editing {
        let err_block = save_error
            .as_ref()
            .map(|e| {
                let msg = e.clone();
                rsx! { p { class: "error-message", role: "alert", "{msg}" } }
            })
            .unwrap_or_else(|| VNode::empty());
        let input_state = st.clone();
        let key_state = st.clone();
        let save_state = st.clone();
        let cancel_state = st.clone();
        rsx! {
            div { class: "pokemon-name-edit-wrapper",
                input {
                    class: "pokemon-name-input",
                    "aria-label": "宠物昵称",
                    value: edit_value,
                    maxlength: NICKNAME_MAX,
                    placeholder: "输入昵称（最多 14 字）",
                    disabled: saving,
                    "data-test-my-pm-nick-input": "1",
                    oninput: move |e: tairitsu_vdom::InputEvent| {
                        // 只记草稿值，不重渲染（输入框是受控的，重渲染会把
                        // 光标顶回末尾）。
                        input_state.my_pm.edit_value.inner().set(e.value.clone());
                    },
                    onkeydown: move |e: tairitsu_vdom::KeyboardEvent| {
                        // 对齐 Dioxus 版 handle_key_down：回车提交（空昵称在
                        // submit_rename 里拦下）、Esc 取消；保存中忽略按键。
                        if saving {
                            return;
                        }
                        if e.key == "Enter" {
                            submit_rename(&key_state);
                        } else if e.key == "Escape" {
                            cancel_edit(&key_state);
                        }
                    }
                }
                div { class: "edit-actions-inline",
                    { crate::button::hooked(tairitsu_macros::rsx! { Button {
                        class: ("edit-action-btn save").to_string(),
                        disabled: saving,
                        onclick: crate::button::click(move || submit_rename(&save_state)),
                        {Icon(IconProps {
                            name: Some(IconName::Check),
                            size: Some(14),
                            class: Some("lucide-icon".to_string()),
                        })}
                                        } }, [("title", ("保存").to_string()), ("data-test-my-pm-nick-save", ("1").to_string())]) }
                    { crate::button::hooked(tairitsu_macros::rsx! { Button {
                        class: ("edit-action-btn cancel").to_string(),
                        disabled: saving,
                        onclick: crate::button::click(move || cancel_edit(&cancel_state)),
                        {Icon(IconProps {
                            name: Some(IconName::X),
                            size: Some(14),
                            class: Some("lucide-icon".to_string()),
                        })}
                                        } }, [("title", ("取消").to_string()), ("data-test-my-pm-nick-cancel", ("1").to_string())]) }
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
        let edit_state = st.clone();
        // 旧版是铅笔图标（lucide Pencil，12px），不是文字。
        let edit_button = crate::button::hooked(
            tairitsu_macros::rsx! { Button {
                class: ("edit-icon-btn").to_string(),
                onclick: crate::button::click(move || begin_edit(&edit_state, &edit_seed)),
                {Icon(IconProps {
                    name: Some(IconName::Pencil),
                    size: Some(12),
                    class: Some("lucide-icon".to_string()),
                })}
            } },
            [
                ("aria-label", ("修改昵称").to_string()),
                ("data-test-my-pm-nick-edit", ("1").to_string()),
            ],
        );
        let container: Vec<VNode> = vec![name_node, edit_button];
        rsx! {
            for i in 0..container.len() {
                ..vec![container[i].clone()]
            }
        }
    };

    let state_block = if show_state {
        rsx! { span { class: state_cls, title: state_detail, "{state_text}" } }
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
    // 闪光 / 神兽标记（旧版 ShinyMark / GodMark）：「闪」「神」字徽标，带说明。
    let shiny_block = if d.is_shiny {
        rsx! { span { class: "pet-mark pet-mark-shiny", title: "异色（闪光）宠物", "闪" } }
    } else {
        VNode::empty()
    };
    let god_block = if d.is_legendary {
        rsx! { span { class: "pet-mark pet-mark-god", title: "神兽 / 活动定制宠物", "神" } }
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
    // 名字行：名字、性别、标记、徽章。旧版这一行没有等级；编辑态整行只有
    // 编辑框（Dioxus 把性别/标记/徽章放在 else 分支里）。
    let name_row: Vec<VNode> = if editing {
        vec![name_area]
    } else {
        vec![
            name_area,
            rsx! { span { class: gender_cls, "{gender_sym}" } },
            shiny_block,
            god_block,
            quality_badge,
        ]
    };
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
        Some((ivs, total, rank, grade)) => {
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
            // 这一行总带着评级徽章：载荷有 quality_grade 就用它，没有按总和判定。
            let badge_cls = format!("quality-badge quality-rank-{rank}");
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
                    span { class: badge_cls, "{grade}" }
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
            // 旧版这里带着内联的 8px 下边距（game.css 里是 0）。
            div { class: "stats-header", style: "margin-bottom: 8px;",
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
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: ("ev-calculator-btn").to_string(),
                            onclick: crate::button::click(move || open_ev_modal(&ev_state)),
                            "努力值计算器"
                                                } }, [("data-test-my-pm-ev-open", ("1").to_string())]) }
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: ("ev-calculator-btn").to_string(),
                            onclick: crate::button::click(move || open_evo_path_modal(&evo_path_state)),
                            "进化路径"
                                                } }, [("data-test-my-pm-evo-path-open", ("1").to_string())]) }
                        { crate::button::hooked(tairitsu_macros::rsx! { Button {
                            class: ("ev-calculator-btn").to_string(),
                            onclick: crate::button::click(move || open_evolution_modal(&evo_state)),
                            "进化"
                                                } }, [("data-test-my-pm-evo-open", ("1").to_string())]) }
                    }
                }
                ..vec![radar]
            }
        }
    }
}

/// 五维雷达图（SVG，旧版 `RadarChart`）：五边形网格、最小半径 20%、上限 300 点，
/// 数据点、中心的等级、每个顶点上的数值与带色点的标签。
fn radar_chart(
    level: u64,
    attack: i64,
    defense: i64,
    sp_attack: i64,
    sp_defense: i64,
    speed: i64,
) -> VNode {
    let cx = 140.0f64;
    let cy = 140.0f64;
    let max_r = 70.0f64;
    let min_r = max_r * 0.2;
    let max_stat = 300.0f64;

    let angles: Vec<f64> = (0..5)
        .map(|i| (-90.0 + i as f64 * 72.0).to_f64_radians())
        .collect();
    let labels = ["攻击", "防御", "特攻", "特防", "速度"];
    let values = [attack, defense, sp_attack, sp_defense, speed];
    let colors = ["#E74C3C", "#F5A623", "#9B59B6", "#1ABC9C", "#3498DB"];

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

    // 旧版的数值是无符号的；这边的六维可能为负，低于 0 的按 0 画。
    let data_points: Vec<(f64, f64)> = values
        .iter()
        .zip(angles.iter())
        .map(|(&v, &a)| {
            let ratio = (v.max(0) as f64 / max_stat).min(1.0);
            let r = min_r + ratio * (max_r - min_r);
            (cx + r * a.cos(), cy + r * a.sin())
        })
        .collect();
    let data_path = format!(
        "M {} Z",
        data_points
            .iter()
            .map(|(x, y)| format!("{:.1},{:.1}", x, y))
            .collect::<Vec<_>>()
            .join(" L ")
    );
    let label_positions: Vec<(f64, f64)> = angles
        .iter()
        .map(|&a| {
            let r = max_r + 15.0;
            (cx + r * a.cos(), cy + r * a.sin())
        })
        .collect();
    let level_text = format!("Lv {level}");

    // 轴线与数据点的坐标照旧版原样 Display（不四舍五入）。
    let axis_nodes: Vec<VNode> = angles
        .iter()
        .map(|a| {
            let x1 = format!("{cx}");
            let y1 = format!("{cy}");
            let x2 = format!("{}", cx + max_r * a.cos());
            let y2 = format!("{}", cy + max_r * a.sin());
            rsx! {
                line {
                    x1: x1,
                    y1: y1,
                    x2: x2,
                    y2: y2,
                    stroke: "#e0e0e0",
                    "stroke-width": "0.5"
                }
            }
        })
        .collect();
    let dot_nodes: Vec<VNode> = data_points
        .iter()
        .map(|(x, y)| {
            let cx_s = format!("{x}");
            let cy_s = format!("{y}");
            rsx! {
                circle {
                    cx: cx_s,
                    cy: cy_s,
                    r: "4",
                    fill: "#3498db",
                    stroke: "white",
                    "stroke-width": "2"
                }
            }
        })
        .collect();
    // 每个顶点两行：上面是数值（粗体），下面是带色点的属性名。
    let label_nodes: Vec<VNode> = label_positions
        .iter()
        .enumerate()
        .flat_map(|(i, (lx, ly))| {
            let x = format!("{lx}");
            let y_value = format!("{}", ly - 8.0);
            let y_label = format!("{}", ly + 4.0);
            let value = format!("{}", values[i]);
            let color = colors[i];
            let label = format!("● {}", labels[i]);
            vec![
                rsx! {
                    text {
                        x: x.clone(),
                        y: y_value,
                        "text-anchor": "middle",
                        "font-size": "12",
                        fill: color,
                        "font-weight": "bold",
                        "{value}"
                    }
                },
                rsx! {
                    text {
                        x: x,
                        y: y_label,
                        "text-anchor": "middle",
                        "font-size": "12",
                        fill: color,
                        "{label}"
                    }
                },
            ]
        })
        .collect();

    rsx! {
        div { class: "stats-radar-chart",
            svg {
                class: "radar-svg responsive",
                "viewBox": "0 0 280 280",
                width: "200px",
                height: "200px",
                for i in 0..grid_paths.len() {
                    path {
                        d: grid_paths[i].clone(),
                        fill: "none",
                        stroke: "#e0e0e0",
                        "stroke-width": "1"
                    }
                }
                for i in 0..axis_nodes.len() {
                    ..vec![axis_nodes[i].clone()]
                }
                path {
                    d: data_path,
                    fill: "rgba(52, 152, 219, 0.2)",
                    stroke: "#3498db",
                    "stroke-width": "2"
                }
                for i in 0..dot_nodes.len() {
                    ..vec![dot_nodes[i].clone()]
                }
                text {
                    x: "140",
                    y: "144",
                    "text-anchor": "middle",
                    "font-size": "16",
                    "font-weight": "bold",
                    fill: "#333",
                    "{level_text}"
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
        let skill_type_id = skill.type_id;
        let card_state = st.clone();
        let forget_state = st.clone();
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
        // 右上角的 PP：`<em>PP </em><strong>15</strong> /15`（旧版如此，不是一整段文字）。
        let pp_str = skill.pp.to_string();
        let pp_tail = if skill.max_pp > 0 {
            format!(" /{}", skill.max_pp)
        } else {
            format!(" /{}", skill.pp)
        };
        // 徽章走旧版 TypeBadge（crate::type_badge）：属性、分类各一张。
        let power_label = match skill.category.as_str() {
            "特攻" | "特殊" => "特攻：",
            "物攻" | "物理" => "物攻：",
            _ => "威力：",
        };
        let power_str = skill.power.to_string();
        // 底部那行 PP：`PP：<strong>15/15</strong>`，上限在 strong 里面（旧版如此）；
        // 没有上限时只有当前值。
        let pp_stat = if skill.max_pp > 0 {
            format!("{}/{}", skill.pp, skill.max_pp)
        } else {
            skill.pp.to_string()
        };
        let level_text = if skill.level > 0 {
            Some(format!("Lv {}", skill.level))
        } else {
            None
        };
        let stype = skill.skill_type.clone();
        let scat = skill.category.clone();
        let badges = crate::type_badge::type_badge(&stype, &scat, "");

        let forget_block = if is_selected {
            let btn_cls = if skill.can_forget {
                "skill-forget-btn"
            } else {
                "skill-forget-btn disabled"
            };
            rsx! {
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: (btn_cls).to_string(),
                    disabled: loading,
                    onclick: Some(tairitsu_vdom::EventHandler::new(move |e: tairitsu_vdom::MouseEvent| {
                        // 遗忘按钮在技能卡内部：不挡住冒泡会连卡片一起 toggle。
                        e.stop_propagation();
                        forget_skill(&forget_state, skill_type_id);
                    })),
                    "遗忘此技能"
                                } }, [("data-test-skill-forget", (tid.clone()).to_string())]) }
            }
        } else {
            VNode::empty()
        };

        cards.push(rsx! {
            div { class: card_cls,
                "data-test-skill-card": tid.clone(),
                onclick: move |_| toggle_skill_card(&card_state, skill_type_id),
                div { class: "skill-card-top",
                    span { class: "skill-name", "{name}" }
                    span { class: "skill-pp",
                        em { "PP " }
                        strong { "{pp_str}" }
                        "{pp_tail}"
                    }
                }
                div { class: "skill-card-bottom",
                    ..vec![badges]
                    div { class: "skill-stats",
                        span { class: "skill-power",
                            "{power_label}"
                            strong { "{power_str}" }
                        }
                        span { class: "skill-pp",
                            "PP："
                            strong { "{pp_stat}" }
                        }
                    }
                    if let Some(_lv) = level_text {
                        span { class: "skill-level", "{_lv}" }
                    }
                }
                ..vec![forget_block]
            }
        });
    }
    for _ in 0..empty_count {
        let learn_slot_state = st.clone();
        cards.push(rsx! {
            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                class: ("skill-slot-empty").to_string(),
                disabled: loading,
                onclick: crate::button::click(move || open_learn_modal(&learn_slot_state)),
                div { class: "skill-slot-plus", "+" }
                div { class: "skill-slot-hint", "点击学习新技能" }
                        } }, [("data-test-skill-learn-slot", ("1").to_string())]) }
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
    let overlay_state = st.clone();
    let close_state = st.clone();

    let body = match st.my_pm.learnable.get() {
        None => rsx! { div { class: "skill-learn-loading", "加载技能列表中..." } },
        Some(Err(error)) => {
            let msg = format!("技能加载失败：{error}");
            let reload_state = st.clone();
            rsx! {
                div { role: "alert", class: "error-message", "{msg}" }
                { crate::button::hooked(tairitsu_macros::rsx! { Button {
                    class: ("btn btn-primary").to_string(),
                    onclick: crate::button::click(move || retry_learnable_load(&reload_state)),
                    "重新加载技能"
                                } }, [("data-test-learn-reload", ("1").to_string())]) }
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
                    rows.push(learn_item(skill, st.my_pm.skill_loading.get(), Some(st)));
                }
            }
            if !locked.is_empty() {
                rows.push(rsx! { div { class: "skill-section-title", "等级不足" } });
                for skill in &locked {
                    rows.push(learn_item(skill, true, None));
                }
            }
            rsx! {
                for i in 0..rows.len() {
                    ..vec![rows[i].clone()]
                }
            }
        }
    };

    // 外壳是共用的 hikari Modal（见 crate::modal）；正文保持旧类名，样式不动。
    let body: Vec<VNode> = vec![rsx! {
        div { class: "skill-learn-content", ..vec![body] }
    }];
    crate::modal::modal_hooked(
        crate::modal::ModalSpec::new(&title, false),
        crate::modal::hook_close_button("data-test-learn-close", move || {
            close_learn_modal(&close_state)
        }),
        move || close_learn_modal(&overlay_state),
        body,
    )
}

fn learn_item(skill: &crate::state::LearnableSkill, loading: bool, st: Option<&AppState>) -> VNode {
    let sid = skill.id.to_string();
    let skill_id = skill.id;
    let click_state = st.cloned();
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
    // 可用/锁定是两个不同的 span 类（Dioxus：learn-btn 蓝色药丸、level-hint 灰字）。
    let learn_hint = skill.is_available;
    let hint_text = if skill.is_available {
        "学习".to_string()
    } else {
        format!("需要 Lv.{}", skill.required_level)
    };
    let sname = skill.name.clone();
    let stype = skill.skill_type.clone();
    let scat = skill.category.clone();
    let sdesc = skill.description.clone();

    rsx! {
        { crate::button::hooked(tairitsu_macros::rsx! { Button {
            class: (cls).to_string(),
            disabled: !skill.is_available || loading,
            onclick: crate::button::click(move || {
                if let Some(st) = &click_state {
                    learn_skill(st, skill_id);
                }
            }),
            div { class: "skill-learn-item-header",
                span { class: "skill-learn-name", "{sname}" }
                if learn_hint {
                    span { class: "skill-learn-learn-btn", "{hint_text}" }
                } else {
                    span { class: "skill-learn-level-hint", "{hint_text}" }
                }
            }
            div { class: "skill-learn-item-body",
                span { class: type_cls, "{stype}" }
                if !scat.is_empty() {
                    span { class: cat_cls, "{scat}" }
                }
                if let Some(_p) = power_text {
                    span { class: "skill-power", "{_p}" }
                }
                span { class: "skill-pp", "{pp_text}" }
            }
            if !skill.description.is_empty() {
                div { class: "skill-description", "{sdesc}" }
            }
                } }, [
                ("style", ("width: 100%; text-align: left; font: inherit;").to_string()),
                ("data-test-learn-item", (sid).to_string()),
            ]) }
    }
}

// ---------------------------------------------------------------------------
// 属性页弹窗：努力值计算器 / 进化路径 / 进化
// ---------------------------------------------------------------------------

const EV_STAT_NAMES: [&str; 6] = ["HP", "攻击", "防御", "特攻", "特防", "速度"];

/// 努力值分配计算器（纯客户端）：总分配 = 已分配 + 待分配，单项 ≤255、总计 ≤510。
/// 输入只放在「待分配」一行（对齐 Dioxus 版：已分配行是纯文本）。
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
    let (status_cls, status_text, status_icon) = if valid {
        ("ev-status-valid", "配置有效", IconName::Check)
    } else {
        ("ev-status-invalid", "配置无效（超过限制）", IconName::X)
    };

    let header_cells: Vec<VNode> = EV_STAT_NAMES
        .iter()
        .map(|n| {
            let label = *n;
            rsx! { div { class: "ev-cell", "{label}" } }
        })
        .collect();

    let total_cells: Vec<VNode> = (0..6)
        .map(|i| {
            let t = totals[i].to_string();
            rsx! { div { class: "ev-cell", "{t}" } }
        })
        .collect();
    let cur_cells: Vec<VNode> = (0..6)
        .map(|i| {
            let t = current[i].to_string();
            rsx! { div { class: "ev-cell", "{t}" } }
        })
        .collect();
    let add_cells: Vec<VNode> = (0..6)
        .map(|i| {
            let attr = format!("add-{i}");
            let value = added[i].to_string();
            let cell_state = st.clone();
            rsx! {
                div { class: "ev-cell",
                    input {
                        class: "ev-input",
                        r#type: "number",
                        min: "0",
                        max: "255",
                        value: value,
                        "data-test-ev-input": attr,
                        oninput: move |e: tairitsu_vdom::InputEvent| {
                            set_ev_value(&cell_state, i, true, &e.value);
                        }
                    }
                }
            }
        })
        .collect();
    let cur_sum = current.iter().sum::<u32>().to_string();
    let added_sum = added.iter().sum::<u32>().to_string();
    let grand_text = grand.to_string();
    // 合计超出 510 时整格转红（.grand-total.invalid）。
    let grand_cls = if valid {
        "ev-cell ev-total grand-total"
    } else {
        "ev-cell ev-total grand-total invalid"
    };

    let ev_overlay_state = st.clone();
    let ev_close_state = st.clone();
    // 外壳是共用的 hikari Modal（见 crate::modal）；正文保持旧类名，样式不动。
    let body = rsx! {
        div { class: "ev-calculator",
            div { class: "ev-notice",
                p { "注意事项：" }
                ul {
                    li { "每项努力值最高 255 点" }
                    li { "6 项努力值总计最高 510 点" }
                    li { "本工具仅用于计算，不会保存数据" }
                }
            }
            div { class: "ev-table",
                div { class: "ev-table-header",
                    div { class: "ev-cell ev-label", "努力值" }
                    for i in 0..header_cells.len() {
                        ..vec![header_cells[i].clone()]
                    }
                    div { class: "ev-cell ev-total", "总计" }
                }
                div { class: "ev-table-row",
                    div { class: "ev-cell ev-label", "总分配" }
                    for i in 0..total_cells.len() {
                        ..vec![total_cells[i].clone()]
                    }
                    div { class: grand_cls, "{grand_text}" }
                }
                div { class: "ev-table-row",
                    div { class: "ev-cell ev-label", "已分配" }
                    for i in 0..cur_cells.len() {
                        ..vec![cur_cells[i].clone()]
                    }
                    div { class: "ev-cell ev-total", "{cur_sum}" }
                }
                div { class: "ev-table-row input-row",
                    div { class: "ev-cell ev-label", "待分配" }
                    for i in 0..add_cells.len() {
                        ..vec![add_cells[i].clone()]
                    }
                    div { class: "ev-cell ev-total", "{added_sum}" }
                }
            }
            div { class: "ev-status",
                span { class: status_cls,
                    {Icon(IconProps {
                        name: Some(status_icon),
                        size: Some(12),
                        class: Some("lucide-inline".to_string()),
                    })}
                    "{status_text}"
                }
            }
        }
    };
    crate::modal::modal_hooked(
        crate::modal::ModalSpec::new("努力值分配计算器", true),
        crate::modal::hook_close_button("data-test-ev-close", move || {
            close_ev_modal(&ev_close_state)
        }),
        move || close_ev_modal(&ev_overlay_state),
        vec![body],
    )
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

    let path_overlay_state = st.clone();
    let path_close_state = st.clone();
    let fwd_state = st.clone();
    let bwd_state = st.clone();
    // 外壳是共用的 hikari Modal（见 crate::modal）；正文保持旧类名，样式不动。
    let mut body_nodes: Vec<VNode> = vec![rsx! {
        div { class: "evo-path-tab-bar",
            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                class: (fwd_cls).to_string(),
                onclick: crate::button::click(move || switch_evo_path_tab(&fwd_state, true)),
                "进化路线"
                        } }, [("data-test-evo-path-tab", ("forward").to_string())]) }
            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                class: (bwd_cls).to_string(),
                onclick: crate::button::click(move || switch_evo_path_tab(&bwd_state, false)),
                "退化路线"
                        } }, [("data-test-evo-path-tab", ("backward").to_string())]) }
        }
    }];
    body_nodes.push(body);
    crate::modal::modal_hooked(
        crate::modal::ModalSpec::new("进化路径", false),
        crate::modal::hook_close_button("data-test-evo-path-close", move || {
            close_evo_path_modal(&path_close_state)
        }),
        move || close_evo_path_modal(&path_overlay_state),
        body_nodes,
    )
}

/// 进化弹窗：可进化路线（条件 + 状态 + 执行按钮）。
fn evolution_modal(st: &AppState) -> VNode {
    let evolving = st.my_pm.evolving.get();
    let evolve_state = st.clone();
    let evo_overlay_state = st.clone();
    let evo_close_state = st.clone();

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
                        let evo_id = evo.to_id;
                        let evo_state = evolve_state.clone();
                        {
                            // 钩子里要用的名字先留一份：下面的闭包会把 to_name 挪走。
                            let hook_name = to_name.clone();
                            crate::button::hooked(tairitsu_macros::rsx! { Button {
                                class: ("ev-calculator-btn").to_string(),
                                disabled: evolving,
                                onclick: crate::button::click(move || evolve_to(&evo_state, evo_id, &to_name)),
                                "进化"
                            } }, [("data-test-evo-do", to_id.to_string()), ("data-test-evo-do-name", hook_name)])
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

    // 外壳是共用的 hikari Modal（见 crate::modal）；正文保持旧类名，样式不动。
    crate::modal::modal_hooked(
        crate::modal::ModalSpec::new("进化", false),
        crate::modal::hook_close_button("data-test-evo-modal-close", move || {
            close_evolution_modal(&evo_close_state)
        }),
        move || close_evolution_modal(&evo_overlay_state),
        vec![body],
    )
}
