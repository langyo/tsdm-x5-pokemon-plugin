//! 携带宠物迷你格（对齐 Dioxus 版 `components/pokemon/mini_grid.rs`）。
//!
//! 2 行 × 3 列：有宠物的格子是精灵图 + 等级标 + 战斗/异常状态角标，空位是
//! 灰叉；悬浮显示详情 tooltip（HP / EXP 条 + 技能表）。结构、类名与旧版
//! 完全一致，样式走 game.css（.pm-mini-*）。

use tairitsu_macros::rsx;
use tairitsu_vdom::VNode;

use crate::state::PokemonBasic;

const IMG_PATH: &str = "source/plugin/pokemon/images";

fn hp_pct_class(pm: &PokemonBasic) -> (&'static str, u64) {
    let pct = if pm.max_hp > 0 {
        ((pm.hp as f64 / pm.max_hp as f64) * 100.0).max(0.0) as u64
    } else {
        0
    };
    let cls = if pct == 0 {
        "fainted"
    } else if pct <= 20 {
        "critical"
    } else if pct <= 50 {
        "low"
    } else {
        ""
    };
    (cls, pct)
}

/// 该宠物是否处于负面状态（与旧版 `has_negative_state` 同一张码表）。
fn has_negative_state(state: u8) -> bool {
    matches!(state, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 11 | 15 | 20 | 21 | 22)
}

/// 迷你格。`team` 是携带中的宠物（site 1/2，最多 6 只），`active_id` 是
/// 当前高亮的宠物 id，`on_select(index)` 由侧栏决定点击后去哪。`user_in_battle`
/// 是用户此刻是否在战斗里：战斗中的图标只给「在战斗且是首位」的那只（旧版
/// `is_user_in_battle && p.site == 1`），光看 site 的话平时首只头上也一直有。
///
/// `hover_slot` / `set_hover` 是悬浮态：tairitsu 的信号不跨渲染保留，所以
/// tooltip 的显隐放在侧栏状态里（旧版是组件内 `use_signal`）。
pub fn mini_grid(
    team: &[PokemonBasic],
    active_id: Option<u64>,
    user_in_battle: bool,
    hover_slot: Option<usize>,
    set_hover: std::rc::Rc<dyn Fn(Option<usize>)>,
    on_select: std::rc::Rc<dyn Fn(usize)>,
) -> VNode {
    let slots: Vec<VNode> = (0..6)
        .map(|i| match team.get(i) {
            Some(pm) => slot(
                pm,
                i,
                Some(pm.id) == active_id,
                user_in_battle,
                hover_slot == Some(i),
                set_hover.clone(),
                on_select.clone(),
            ),
            None => rsx! {
                div { class: "pm-mini-slot empty-slot",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "12",
                        height: "12",
                        "viewBox": "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        class: "pm-mini-cross",
                        dangerous_inner_html: r#"<path d="M18 6 6 18" /><path d="m6 6 12 12" />"#,
                    }
                }
            },
        })
        .collect();
    rsx! {
        div { class: "pm-mini-grid", ..slots }
    }
}

fn slot(
    pm: &PokemonBasic,
    index: usize,
    is_active: bool,
    user_in_battle: bool,
    hovered: bool,
    set_hover: std::rc::Rc<dyn Fn(Option<usize>)>,
    on_select: std::rc::Rc<dyn Fn(usize)>,
) -> VNode {
    let (hp_cls, _) = hp_pct_class(pm);
    let sprite = format!("{IMG_PATH}/spm/{}.gif", pm.type_id);
    let alt = pm.display_name();
    let level = format!("Lv.{}", pm.level);
    let slot_cls = if is_active {
        "pm-mini-slot active"
    } else {
        "pm-mini-slot"
    };
    let in_battle = user_in_battle && pm.site == 1;
    let negative = has_negative_state(pm.state);
    let hp_low = !hp_cls.is_empty() && hp_cls != "fainted";
    let hover_enter = set_hover.clone();
    let hover_leave = set_hover;
    let tooltip = if hovered {
        Tooltip(TooltipProps {
            pm: Some(Box::new(pm.clone())),
            hp_class: Some(hp_cls.to_string()),
        })
    } else {
        VNode::empty()
    };
    rsx! {
        div { class: "pm-mini-slot-wrapper",
            button {
                r#type: "button",
                class: slot_cls,
                onclick: move |_| on_select(index),
                onmouseenter: move |_| hover_enter(Some(index)),
                onmouseleave: move |_| hover_leave(None),
                img { class: "pm-mini-sprite", src: sprite, alt: alt }
                if pm.is_shiny || pm.is_legendary {
                    span { class: "pm-mini-marks",
                        if pm.is_shiny {
                            span { class: "pet-mark pet-mark-shiny pet-mark-compact", title: "异色（闪光）宠物", "闪" }
                        }
                        if pm.is_legendary {
                            span { class: "pet-mark pet-mark-god pet-mark-compact", title: "神兽 / 活动定制宠物", "神" }
                        }
                    }
                }
                span { class: "pm-mini-level", "{level}" }
                if in_battle {
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "8",
                        height: "8",
                        "viewBox": "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        class: "pm-mini-battle-icon",
                        dangerous_inner_html: r#"<path d="M14.5 17.5 3 6V3h3l11.5 11.5" /><path d="M13 19l6-6" /><path d="M16 16l4 4" /><path d="M19 21l2-2" />"#,
                    }
                }
                if hp_low {
                    span { class: "pm-mini-hp-indicator {hp_cls}" }
                }
                if negative {
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "10",
                        height: "10",
                        "viewBox": "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        class: "pm-mini-status-indicator",
                        dangerous_inner_html: r#"<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3" /><path d="M12 9v4" /><path d="M12 17h.01" />"#,
                    }
                }
            }
            ..vec![tooltip]
        }
    }
}

/// 迷你格 tooltip（对齐旧版 PokemonTooltip：名字 + 等级 + 状态 + HP/EXP 条 + 技能）。
#[tairitsu_macros::component]
fn Tooltip(pm: Box<PokemonBasic>, hp_class: String) -> VNode {
    let display_name = pm.display_name();
    let (_, hp_pct) = hp_pct_class(&pm);
    let hp_style = format!("width: {hp_pct}%;");
    let hp_text = format!("{}/{}", pm.hp, pm.max_hp);

    let in_level = pm.exp.saturating_sub(pm.exp_for_current_level);
    let needed = pm
        .exp_for_next_level
        .saturating_sub(pm.exp_for_current_level);
    let exp_pct = if needed > 0 {
        ((in_level as f64 / needed as f64) * 100.0).min(100.0) as u64
    } else {
        100
    };
    let exp_style = format!("width: {exp_pct}%;");
    let exp_text = format!("{in_level}/{needed}");

    let skills: Vec<VNode> = pm
        .skills
        .iter()
        .filter(|s| !s.name.is_empty())
        .map(|s| {
            let type_cls = format!("badge badge-type-{}", s.skill_type);
            let pp = format!("PP: {}/{}", s.pp, s.max_pp);
            let name = s.name.clone();
            let skill_type = s.skill_type.clone();
            rsx! {
                div { class: "pm-tooltip-skill-row",
                    span { class: "skill-name", "{name}" }
                    span { class: "skill-pp", "{pp}" }
                    if !skill_type.is_empty() {
                        span { class: "badge {type_cls} type-badge-sm", "{skill_type}" }
                    }
                }
            }
        })
        .collect();

    let negative = has_negative_state(pm.state);
    let state_text = pm.state_text.clone();
    let level_text = format!("Lv.{}", pm.level);
    rsx! {
        div { class: "pm-mini-tooltip",
            div { class: "pm-tooltip-header",
                div { class: "pm-tooltip-name-row",
                    span { class: "pm-tooltip-name", "{display_name}" }
                    if pm.is_shiny {
                        span { class: "pet-mark pet-mark-shiny pet-mark-compact", title: "异色（闪光）宠物", "闪" }
                    }
                    if pm.is_legendary {
                        span { class: "pet-mark pet-mark-god pet-mark-compact", title: "神兽 / 活动定制宠物", "神" }
                    }
                    span { class: "pm-tooltip-level", "{level_text}" }
                }
                if negative && !state_text.is_empty() {
                    span { class: "pm-tooltip-status", "{state_text}" }
                }
            }
            div { class: "pm-tooltip-stats",
                div { class: "pm-tooltip-stat",
                    span { class: "stat-label", "HP" }
                    div { class: "stat-hp-bar",
                        div { class: "hp-bar-bg",
                            div { class: "hp-bar-fg {hp_class}", style: hp_style }
                            span { class: "stat-value-inside", "{hp_text}" }
                        }
                    }
                }
                div { class: "pm-tooltip-stat",
                    span { class: "stat-label", "EXP" }
                    div { class: "stat-exp-bar",
                        div { class: "exp-bar-bg",
                            div { class: "exp-bar-fg", style: exp_style }
                            span { class: "stat-value-inside", "{exp_text}" }
                        }
                    }
                }
            }
            if !skills.is_empty() {
                div { class: "pm-tooltip-skills", ..skills }
            }
        }
    }
}
