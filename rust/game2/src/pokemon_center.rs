//! 宠物中心（治疗）页（tairitsu 版，1:1 移植自
//! rust/game/src/pages/pokemon_center.rs）。同名同类直用 game.css
//! （.page-pokemon-center / .center-card / .heal-pokemon-card …）。
//!
//! 覆盖：携带宠物的受伤/异常状态列表、单只治疗、全部治疗、战斗中首位的
//! 「脱战并治疗」、PP 未满提示、闪光/神兽标记。
//!
//! 状态判定对齐服务端负面码表（0/2-7/11/15/20-22 与 hp<=0）；
//! 箱子里的宠物（site 非 1/2）不在此页显示（对齐 Dioxus 版）。

use hikari_components::{Button, ButtonProps};
use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use crate::app_state::AppState;
use crate::state::{PokemonBasic, ToastKind};
use crate::toast;

const IMG_PATH: &str = "source/plugin/pokemon/images";

/// 首次进入装载（对齐 Dioxus 版 use_resource 首次求值）。
pub fn ensure_loaded(state: &AppState) {
    if state.center.pokemons.get().is_none() {
        reload(state);
    }
}

fn reload(state: &AppState) {
    let center = state.center.clone();
    crate::api::pokemon_list(Box::new(move |result| {
        center.pokemons.inner().set(Some(result));
        crate::request_remount();
    }));
}

/// 治疗一只（`flee=true` 时走脱战治疗）。
pub fn heal_one(state: &AppState, pokemon_id: u64, flee: bool) {
    let center = state.center.clone();
    if center.healing.get() {
        return;
    }
    center.healing.inner().set(true);
    crate::request_remount();
    crate::api::heal_pokemon(
        pokemon_id,
        flee,
        Box::new(move |result| {
            match result {
                Ok(out) => {
                    let msg = if out.cost > 0 {
                        format!("{}（花费 {} 金币）", out.message, out.cost)
                    } else {
                        out.message
                    };
                    toast::show(ToastKind::Success, msg);
                }
                Err(e) => {
                    toast::show(ToastKind::Error, format!("治疗失败: {e}"));
                }
            }
            center.healing.inner().set(false);
            crate::request_remount();
            // 治疗成功后刷新列表（HP/PP/状态都会变）。
            let center2 = center;
            crate::api::pokemon_list(Box::new(move |result| {
                if let Ok(list) = result {
                    center2.pokemons.inner().set(Some(Ok(list)));
                    crate::request_remount();
                }
            }));
        }),
    );
}

/// 全部治疗：逐只串行治疗（对齐 Dioxus 版，跳过战斗中首位）。
pub fn heal_all(state: &AppState) {
    let center = state.center.clone();
    if center.healing.get() {
        return;
    }
    let Some(Ok(list)) = center.pokemons.get() else {
        return;
    };
    let in_battle = center.in_battle.get();
    let targets: Vec<u64> = list
        .iter()
        .filter(|p| (p.site == 1 || p.site == 2) && p.needs_healing())
        .filter(|p| !(in_battle && p.site == 1))
        .map(|p| p.id)
        .collect();
    if targets.is_empty() {
        toast::show(ToastKind::Error, "没有需要治疗的宠物！");
        crate::request_remount();
        return;
    }
    center.healing.inner().set(true);
    crate::request_remount();
    heal_sequence(state.clone(), targets, 0, 0, 0);
}

/// 递归串行治疗：每只完成后继续下一只（回调链，替代 async 循环）。
/// `cost` 累计总花费，与 Dioxus 版「共花费 N 金币」的提示对齐。
fn heal_sequence(state: AppState, ids: Vec<u64>, idx: usize, healed: u32, cost: i64) {
    if idx >= ids.len() {
        let center = state.center.clone();
        center.healing.inner().set(false);
        if healed > 0 {
            toast::show(
                ToastKind::Success,
                format!("已治疗 {healed} 只宠物，共花费 {cost} 金币"),
            );
        }
        crate::request_remount();
        let center2 = center;
        crate::api::pokemon_list(Box::new(move |result| {
            if let Ok(list) = result {
                center2.pokemons.inner().set(Some(Ok(list)));
                crate::request_remount();
            }
        }));
        return;
    }
    let id = ids[idx];
    let next_state = state.clone();
    crate::api::heal_pokemon(
        id,
        false,
        Box::new(move |result| {
            let (healed_next, cost_next) = match result {
                Ok(outcome) => (healed + 1, cost + outcome.cost),
                Err(_) => (healed, cost),
            };
            heal_sequence(next_state, ids, idx + 1, healed_next, cost_next);
        }),
    );
}

// ---------------------------------------------------------------------------
// 组件
// ---------------------------------------------------------------------------

#[component]
pub fn PokemonCenter(state: Box<AppState>) -> VNode {
    let st = state;
    let center = st.center.clone();
    let data = center.pokemons.get();
    let healing = center.healing.get();
    let in_battle = center.in_battle.get();

    let carried: Vec<PokemonBasic> = match &data {
        Some(Ok(list)) => list
            .iter()
            .filter(|p| p.site == 1 || p.site == 2)
            .cloned()
            .collect(),
        _ => Vec::new(),
    };
    let injured: Vec<PokemonBasic> = carried
        .iter()
        .filter(|p| p.needs_healing())
        .cloned()
        .collect();
    let has_healable = carried
        .iter()
        .any(|p| p.needs_healing() && !(in_battle && p.site == 1));

    let body = match &data {
        None => rsx! { div { class: "center-loading", "正在加载宠物数据..." } },
        Some(Err(message)) => {
            let m = message.clone();
            rsx! { div { class: "center-empty", "{m}" } }
        }
        Some(Ok(_)) if carried.is_empty() => {
            let src = format!("{IMG_PATH}/item/jlq.gif");
            rsx! {
                div { class: "center-empty",
                    img { class: "empty-icon-img", src: src }
                    p { "你还没有宠物" }
                }
            }
        }
        Some(Ok(_)) if injured.is_empty() => {
            let src = format!("{IMG_PATH}/rpg/center_health.gif");
            rsx! {
                div { class: "center-healthy",
                    img { class: "healthy-icon-img", src: src }
                    p { "所有宠物都很健康，不需要治疗！" }
                }
            }
        }
        Some(Ok(_)) => {
            let rows: Vec<VNode> = injured
                .iter()
                .map(|pm| {
                    let id = pm.id;
                    // 治疗卡片上用的是物种名（旧版 `pm.name`），不是昵称。
                    let name = pm.name.clone();
                    let alt_name = name.clone();
                    let level = format!("Lv.{}", pm.level);
                    let sprite = format!("{IMG_PATH}/spm/{}.gif", pm.type_id);
                    let hp_pct = if pm.max_hp > 0 {
                        ((pm.hp.max(0) as f64) / (pm.max_hp as f64) * 100.0) as u32
                    } else {
                        0
                    };
                    let hp_cls = if pm.max_hp <= 0 || pm.hp <= 0 {
                        "hp-fill hp-low"
                    } else {
                        let pct = pm.hp as f64 / pm.max_hp as f64;
                        if pct > 0.5 {
                            "hp-fill hp-high"
                        } else if pct > 0.2 {
                            "hp-fill hp-mid"
                        } else {
                            "hp-fill hp-low"
                        }
                    };
                    let hp_style = format!("width: {hp_pct}%");
                    let hp_text = format!("{}/{}", pm.hp, pm.max_hp);
                    let state_cls = format!("state-label state-{}", pm.state_class);
                    let state_text = pm.state_text.clone();
                    let depleted = pm.depleted_pp();
                    let pp_hint = if depleted.is_empty() {
                        String::new()
                    } else {
                        format!("技能PP未满：{}", depleted.join("、"))
                    };
                    let is_in_battle = in_battle && pm.site == 1;
                    let btn_cls = if is_in_battle {
                        "heal-btn heal-btn-warning"
                    } else {
                        "heal-btn"
                    };
                    let btn_text = if is_in_battle {
                        "脱战并治疗"
                    } else {
                        "治疗"
                    };
                    let shiny = pm.is_shiny;
                    let legendary = pm.is_legendary;
                    // 回调须 'static：每张卡片各持一份状态克隆。
                    let row_state = st.clone();
                    let heal_key = pm.id.to_string();
                    rsx! {
                        div { class: "heal-pokemon-card",
                            div { class: "heal-pm-sprite",
                                img { src: sprite, alt: alt_name }
                            }
                            div { class: "heal-pm-info",
                                div { class: "heal-pm-name-row",
                                    span { class: "heal-pm-name", "{name}" }
                                    if shiny {
                                        span { class: "pet-mark-compact", "★" }
                                    }
                                    if legendary {
                                        span { class: "pet-mark-compact", "◆" }
                                    }
                                    span { class: "heal-pm-level", "{level}" }
                                }
                                div { class: "heal-pm-state",
                                    span { class: state_cls, "{state_text}" }
                                }
                                div { class: "heal-pm-hp",
                                    span { class: "hp-label", "HP" }
                                    div { class: "hp-track",
                                        div { class: hp_cls, style: hp_style }
                                    }
                                    span { class: "hp-text", "{hp_text}" }
                                }
                                if !pp_hint.is_empty() {
                                    div { class: "heal-pm-pp",
                                        span { class: "pp-label", "PP" }
                                        span { class: "pp-text", "{pp_hint}" }
                                    }
                                }
                            }
                            { crate::button::with_attrs(tairitsu_macros::rsx! { Button {
                                class: (btn_cls).to_string(),
                                disabled: healing,
                                onclick: crate::button::click(move || heal_one(&row_state, id, is_in_battle)),
                                "{btn_text}"
                            } }, |el| {
                                // 战斗中的那一个治疗键同时是「逃跑」入口，多一个钩子。
                                el.attributes.insert("data-test-center-heal".to_string(), heal_key.to_string());
                                if is_in_battle {
                                    el.attributes.insert("data-test-center-flee".to_string(), "1".to_string());
                                }
                            }) }
                        }
                    }
                })
                .collect();
            rsx! {
                div { class: "center-pokemon-scroll",
                    div { class: "center-pokemon-list",
                        for i in 0..rows.len() {
                            ..vec![rows[i].clone()]
                        }
                    }
                }
            }
        }
    };

    let heal_all_cls = if has_healable && !healing {
        "heal-all-btn"
    } else {
        "heal-all-btn disabled"
    };
    let header_src = format!("{IMG_PATH}/rpg/center_health.gif");
    let doctor_src = format!("{IMG_PATH}/rpg/doctor.gif");
    let heal_all_state = st.clone();

    rsx! {
        div { class: "page-pokemon-center",
            div { class: "center-card",
                div { class: "center-card-header",
                    img { class: "header-icon-img", src: header_src, alt: "" }
                    span { "宠物中心" }
                }
                div { class: "center-card-body",
                    div { class: "center-welcome-banner",
                        img { class: "welcome-icon-img", src: doctor_src, alt: "" }
                        div { class: "welcome-text",
                            h3 { "欢迎来到宠物中心！" }
                            p { "辛苦了！这里可以恢复宠物的体力。" }
                        }
                        if has_healable {
                            { crate::button::hooked(tairitsu_macros::rsx! { Button {
                                class: (heal_all_cls).to_string(),
                                disabled: healing,
                                onclick: crate::button::click(move || heal_all(&heal_all_state)),
                                "全部治疗"
                                                        } }, [("data-test-center-heal-all", ("1").to_string())]) }
                        }
                    }
                    ..vec![body]
                }
            }
        }
    }
}
