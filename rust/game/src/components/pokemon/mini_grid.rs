use crate::prelude::*;

use crate::{components::common::TypeBadge, components::layout::IMG_PATH};

/// 技能简要信息（用于 tooltip 显示）
#[derive(Clone, PartialEq)]
pub struct MiniSkillInfo {
    pub name: String,
    pub pp: u64,
    pub max_pp: u64,
    pub skill_type: String,
}

/// 宠物简要信息（用于迷你网格渲染）
#[derive(Clone, PartialEq)]
pub struct MiniPokemonInfo {
    pub type_id: u64,
    pub name: String,
    pub nickname: Option<String>,
    pub level: u64,
    pub hp: i64,
    pub max_hp: i64,
    pub is_in_battle: bool,
    pub skills: Vec<MiniSkillInfo>,
    pub state: u8,
    pub state_text: String,
    pub exp: u64,
    pub exp_for_current_level: u64,
    pub exp_for_next_level: u64,
}

impl MiniPokemonInfo {
    /// 获取显示名称（优先使用昵称）
    pub fn display_name(&self) -> String {
        self.nickname.as_ref().unwrap_or(&self.name).clone()
    }

    /// 是否处于负面状态
    pub fn has_negative_state(&self) -> bool {
        matches!(self.state, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 11 | 15)
    }
}

/// 宠物迷你网格组件 — 统一的 2行×3列 布局
/// - 有宠物的格子显示精灵图 + 右下角等级标
/// - 空位显示灰色 ✕ 占位
/// - 支持可选的选中高亮（active_index）和点击回调（on_select）
/// - 支持悬浮显示宠物详细信息 tooltip
#[component]
pub fn PokemonMiniGrid(
    /// 宠物列表
    pokemons: Vec<MiniPokemonInfo>,
    /// 当前选中索引（None 则不高亮任何一个）
    #[props(default)]
    active_index: Option<usize>,
    /// 点击某个宠物格子时的回调（参数为索引）
    #[props(default)]
    on_select: Option<EventHandler<usize>>,
) -> Element {
    rsx! {
        div { class: "pm-mini-grid",
            for i in 0..6 {
                if let Some(pm) = pokemons.get(i) {
                    PokemonMiniSlot {
                        pokemon: pm.clone(),
                        index: i,
                        is_active: active_index == Some(i),
                        on_select: on_select.as_ref().copied(),
                    }
                } else {
                    div { class: "pm-mini-slot empty-slot",
                        span { class: "pm-mini-cross", "✕" }
                    }
                }
            }
        }
    }
}

/// 单个宠物小卡片槽位
#[component]
fn PokemonMiniSlot(
    pokemon: MiniPokemonInfo,
    index: usize,
    is_active: bool,
    on_select: Option<EventHandler<usize>>,
) -> Element {
    let mut show_tooltip = use_signal(|| false);
    let hp_pct = if pokemon.max_hp > 0 {
        (pokemon.hp as f64 / pokemon.max_hp as f64 * 100.0).max(0.0) as u64
    } else {
        0
    };
    let hp_class = if hp_pct == 0 {
        "fainted"
    } else if hp_pct <= 20 {
        "critical"
    } else if hp_pct <= 50 {
        "low"
    } else {
        ""
    };

    rsx! {
        div { class: "pm-mini-slot-wrapper",
            button {
                class: if is_active { "pm-mini-slot active" } else { "pm-mini-slot" },
                onclick: move |_| {
                    if let Some(handler) = &on_select {
                        handler.call(index);
                    }
                },
                onmouseenter: move |_| show_tooltip.set(true),
                onmouseleave: move |_| show_tooltip.set(false),
                img {
                    class: "pm-mini-sprite",
                    src: "{IMG_PATH}/spm/{pokemon.type_id}.gif",
                    alt: "{pokemon.display_name()}",
                }
                span { class: "pm-mini-level", "Lv.{pokemon.level}" }
                if pokemon.is_in_battle {
                    span { class: "pm-mini-battle-icon", "⚔️" }
                }
                // HP 状态指示器
                if hp_pct > 0 && hp_pct <= 50 {
                    span { class: "pm-mini-hp-indicator {hp_class}" }
                }
                if pokemon.has_negative_state() {
                    span { class: "pm-mini-status-indicator", "⚠️" }
                }
            }

            // 宠物详情 Tooltip
            if *show_tooltip.read() {
                PokemonTooltip { pokemon: pokemon.clone(), hp_class: hp_class.to_string() }
            }
        }
    }
}

/// 宠物详情 Tooltip
#[component]
fn PokemonTooltip(pokemon: MiniPokemonInfo, hp_class: String) -> Element {
    let display_name = pokemon.display_name();
    let hp_pct = if pokemon.max_hp > 0 {
        (pokemon.hp as f64 / pokemon.max_hp as f64 * 100.0).max(0.0) as u64
    } else {
        0
    };

    // EXP 计算：当前等级内的经验 / 当前等级总经验需求
    let exp_in_current_level = pokemon.exp.saturating_sub(pokemon.exp_for_current_level);
    let exp_needed = pokemon
        .exp_for_next_level
        .saturating_sub(pokemon.exp_for_current_level);
    let exp_pct = if exp_needed > 0 {
        (exp_in_current_level as f64 / exp_needed as f64 * 100.0).min(100.0) as u64
    } else {
        100
    };

    rsx! {
        div { class: "pm-mini-tooltip",
            div { class: "pm-tooltip-header",
                div { class: "pm-tooltip-name-row",
                    span { class: "pm-tooltip-name", "{display_name}" }
                    span { class: "pm-tooltip-level", "Lv.{pokemon.level}" }
                }
                if pokemon.has_negative_state() && !pokemon.state_text.is_empty() {
                    span { class: "pm-tooltip-status", "{pokemon.state_text}" }
                }
            }
            div { class: "pm-tooltip-stats",
                div { class: "pm-tooltip-stat",
                    span { class: "stat-label", "HP" }
                    div { class: "stat-hp-bar",
                        div { class: "hp-bar-bg",
                            div {
                                class: "hp-bar-fg {hp_class}",
                                style: "width: {hp_pct}%;",
                            }
                            span { class: "stat-value-inside", "{pokemon.hp}/{pokemon.max_hp}" }
                        }
                    }
                }
                div { class: "pm-tooltip-stat",
                    span { class: "stat-label", "EXP" }
                    div { class: "stat-exp-bar",
                        div { class: "exp-bar-bg",
                            div {
                                class: "exp-bar-fg",
                                style: "width: {exp_pct}%;",
                            }
                            span { class: "stat-value-inside", "{exp_in_current_level}/{exp_needed}" }
                        }
                    }
                }
            }
            if !pokemon.skills.is_empty() {
                div { class: "pm-tooltip-skills",
                    for skill in pokemon.skills.iter() {
                        div { class: "pm-tooltip-skill-row",
                            span { class: "skill-name", "{skill.name}" }
                            span { class: "skill-pp", "PP: {skill.pp}/{skill.max_pp}" }
                            if !skill.skill_type.is_empty() {
                                TypeBadge {
                                    skill_type: skill.skill_type.clone(),
                                    class: "type-badge-sm".to_string(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
