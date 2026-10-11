use crate::prelude::*;

mod log;
use log::BattleLogPanel;

use crate::components::common::Modal;
use crate::{
    components::{common::TypeBadge, layout::IMG_PATH, layout::IMG_PATH_REMOTE},
    state::{POKEMON_STATE, SWITCH_POKEMON_REQUEST},
    utils::pokemon::{hp_class, hp_percent},
};
use _utils::types::{
    api_battle::{
        BattleItem, BattlePokemon, BattleScene, BattleSkill, BattleStatus, PpRestoreSkill,
    },
    api_pokemon::PokemonBasic,
    api_user::InventoryItem,
};

#[derive(Clone, Copy, PartialEq)]
pub enum BattleTab {
    Skills,
    Items,
    Capture,
    Flee,
}

#[component]
pub fn BattlePage(
    battle: BattleScene,
    loading: bool,
    on_use_skill: EventHandler<u64>,
    on_flee: EventHandler<()>,
    on_end: EventHandler<()>,
    on_use_item: EventHandler<u64>,
    on_attack: EventHandler<()>,
    on_capture: EventHandler<u64>,
    items: Vec<BattleItem>,
    balls: Vec<InventoryItem>,
    #[props(default)] skill_selection_mode: Option<(u64, String, Vec<PpRestoreSkill>)>,
    on_select_skill: EventHandler<u64>,
    on_cancel_skill_selection: EventHandler<()>,
    #[props(default)] on_enter_items_tab: EventHandler<()>,
    #[props(default)] on_enter_capture_tab: EventHandler<()>,
    #[props(default)] on_switch_pokemon: EventHandler<()>,
    #[props(default)] on_replace_pokemon: EventHandler<u64>,
    #[props(default = false)] can_continue: bool,
    on_continue: EventHandler<()>,
) -> Element {
    let mut current_tab = use_signal(|| BattleTab::Skills);
    let mut show_log = use_signal(|| false);

    // 统一的战斗事件 modal 状态
    // BattleEvent: None | SwitchPokemon | ReplacePokemon | BattleEnd
    #[derive(Clone, PartialEq)]
    enum BattleEvent {
        SwitchPokemon,
        ReplacePokemon(String), // 倒下的宠物名字
        BattleEnd,
    }
    let mut battle_event = use_signal(|| Option::<BattleEvent>::None);

    // 检查全局信号（每次渲染时检查）
    let external_request = *SWITCH_POKEMON_REQUEST.read();
    if external_request {
        *SWITCH_POKEMON_REQUEST.write() = false;
        if !loading {
            battle_event.set(Some(if battle.needs_replacement() {
                BattleEvent::ReplacePokemon(battle.my_pokemon.name.clone())
            } else {
                BattleEvent::SwitchPokemon
            }));
        }
    }

    // 检查宠物是否倒下（HP <= 0）且战斗仍在继续。
    // 服务端约定：宠物倒下但还有替补时 status 报 defeat 而 battle_over=false、
    // can_continue_switch=true，此时应弹换宠而不是当作终局
    let pokemon_fainted = battle.needs_replacement();

    // 组件挂载时检查：如果战斗已经结束（battle_over），直接显示结束弹窗
    // 解决页面切换后返回战斗页面时弹窗不显示的问题
    {
        let battle_over = battle.battle_over;
        use_effect(move || {
            if battle_over && battle_event.read().is_none() {
                battle_event.set(Some(BattleEvent::BattleEnd));
            }
        });
    }

    // 记录上次是否倒下，用于检测刚倒下的瞬间
    let mut was_fainted = use_signal(|| false);
    let just_fainted = pokemon_fainted && !*was_fainted.read();
    if just_fainted {
        was_fainted.set(true);
        // 更新 POKEMON_STATE 中倒下宠物的 HP 为 0
        let fainted_pokemon_id = battle.my_pokemon.instance_id;
        {
            let mut pokemon_state = POKEMON_STATE.write();
            if let Some(pokemon) = pokemon_state
                .list
                .iter_mut()
                .find(|p| p.id == fainted_pokemon_id)
            {
                pokemon.hp = 0;
            }
        }

        // 服务端已确认有替补；列表晚于恢复战斗返回时仍应等待选择，不能误判终局。
        battle_event.set(Some(BattleEvent::ReplacePokemon(
            battle.my_pokemon.name.clone(),
        )));
    } else if !pokemon_fainted {
        was_fainted.set(false);
        let replacing = matches!(
            battle_event.read().as_ref(),
            Some(BattleEvent::ReplacePokemon(_))
        );
        if replacing {
            battle_event.set(None);
        }
    }

    // 检查战斗是否真正结束（battle_over）：胜利/逃脱/捕捉成功/无替补战败。
    // defeat 但 battle_over=false 表示宠物倒下、战斗继续（走上面的换宠流程）
    let battle_ended = battle.battle_over;
    let mut was_battle_ended = use_signal(|| false);
    let just_ended = battle_ended && !*was_battle_ended.read();
    if just_ended {
        was_battle_ended.set(true);
        battle_event.set(Some(BattleEvent::BattleEnd));
    } else if !battle_ended {
        was_battle_ended.set(false);
    }

    // 记录上次的标签页，用于检测
    let mut last_tab = use_signal(|| BattleTab::Skills);

    // 使用 use_effect 监听标签页变化
    use_effect(move || {
        let new_tab = *current_tab.read();
        let old_tab = *last_tab.read();

        if new_tab != old_tab {
            // 切换到了 Items 标签页
            if new_tab == BattleTab::Items {
                on_enter_items_tab.call(());
            }
            // 切换到了 Capture 标签页
            else if new_tab == BattleTab::Capture {
                on_enter_capture_tab.call(());
            }

            last_tab.set(new_tab);
        }
    });

    // 始终显示战斗界面，通过 modal 处理所有战斗事件
    let is_battle_end = battle_event
        .read()
        .as_ref()
        .is_some_and(|e| matches!(e, BattleEvent::BattleEnd));
    rsx! {
        div { class: "page-battle",
            div { class: "battle-tools",
                button { class: "btn btn-secondary", "data-testid": "battle-log-open",
                    disabled: battle.engine_battle_id == 0,
                    onclick: move |_| show_log.set(true), "查看战报 / 分享" }
            }
            div { class: "battle-card",
                div { class: "battle-card-body",
                    div { class: "battle-content-area",
                        // 左侧 Tab 菜单
                        div { class: "battle-tab-menu",
                            button {
                                class: if *current_tab.read() == BattleTab::Skills { "battle-tab active" } else { "battle-tab" },
                                onclick: move |_| current_tab.set(BattleTab::Skills),
                                if *current_tab.read() == BattleTab::Skills {
                                    span { class: "tab-arrow left", ">" }
                                }
                                span { "技能" }
                                if *current_tab.read() == BattleTab::Skills {
                                    span { class: "tab-arrow right", "<" }
                                }
                            }
                            button {
                                class: if *current_tab.read() == BattleTab::Items { "battle-tab active" } else { "battle-tab" },
                                onclick: move |_| {
                                    if *current_tab.read() == BattleTab::Items && !loading {
                                        on_enter_items_tab.call(());
                                    }
                                    current_tab.set(BattleTab::Items);
                                },
                                if *current_tab.read() == BattleTab::Items {
                                    span { class: "tab-arrow left", ">" }
                                }
                                span { "道具" }
                                if *current_tab.read() == BattleTab::Items {
                                    span { class: "tab-arrow right", "<" }
                                }
                            }
                            button {
                                class: if *current_tab.read() == BattleTab::Capture { "battle-tab active" } else { "battle-tab" },
                                onclick: move |_| {
                                    if *current_tab.read() == BattleTab::Capture && !loading {
                                        on_enter_capture_tab.call(());
                                    }
                                    current_tab.set(BattleTab::Capture);
                                },
                                if *current_tab.read() == BattleTab::Capture {
                                    span { class: "tab-arrow left", ">" }
                                }
                                span { "捕捉" }
                                if *current_tab.read() == BattleTab::Capture {
                                    span { class: "tab-arrow right", "<" }
                                }
                            }
                            button {
                                class: if *current_tab.read() == BattleTab::Flee { "battle-tab active" } else { "battle-tab" },
                                onclick: move |_| current_tab.set(BattleTab::Flee),
                                if *current_tab.read() == BattleTab::Flee {
                                    span { class: "tab-arrow left", ">" }
                                }
                                span { "退出战斗" }
                                if *current_tab.read() == BattleTab::Flee {
                                    span { class: "tab-arrow right", "<" }
                                }
                            }
                        }

                        // 分隔线
                        div { class: "battle-divider" }

                        // 右侧主内容区
                        div { class: "battle-main-content",
                                // 上方：战斗场景（带圆角边框）
                                div { class: "battle-scene-container",
                                    div { class: "battle-scene-upper",
                                        div { class: "battle-field",
                                            WildPokemonSection { pokemon: battle.wild_pokemon.clone() }
                                            MyPokemonSection {
                                                pokemon: battle.my_pokemon.clone(),
                                                on_click: move |_| battle_event.set(Some(BattleEvent::SwitchPokemon)),
                                            }
                                        }

                                        div { class: "battle-turn-display",
                                            span { class: "turn-label", "回合" }
                                            span { class: "turn-number", "{battle.turn}" }
                                        }
                                    }
                                }

                                // 下方：Tab 面板内容
                                div { class: "battle-tab-content",
                                    match *current_tab.read() {
                                        BattleTab::Skills => rsx! {
                                            SkillsTabContent {
                                                skills: battle.my_pokemon.skills.clone(),
                                                loading,
                                                on_use_skill: move |skill_id| on_use_skill.call(skill_id),
                                                on_attack: move |_| on_attack.call(()),
                                            }
                                        },
                                        BattleTab::Items => rsx! {
                                            ItemsTabContent {
                                                loading,
                                                items: items.clone(),
                                                my_pokemon: battle.my_pokemon.clone(),
                                                on_use_item: move |item_id| on_use_item.call(item_id),
                                            }
                                        },
                                        BattleTab::Capture => rsx! {
                                            CaptureTabContent {
                                                loading,
                                                balls: balls.clone(),
                                                on_use_ball: move |ball_id| on_capture.call(ball_id),
                                            }
                                        },
                                        BattleTab::Flee => rsx! {
                                            FleeTabContent { loading, on_flee: move |_| on_flee.call(()) }
                                        },
                                    }
                                }
                        }
                    }
                }
            }

            // 技能选择 Modal（用于PP恢复道具）
            if let Some((_item_id, item_name, skills)) = &skill_selection_mode {
                Modal {
                    is_open: true,
                    close_on_overlay: !loading,
                    on_close: move |_| {
                        if !loading {
                            on_cancel_skill_selection.call(());
                        }
                    },
                    title: format!("选择技能 - {}", item_name),
                    div { class: "skill-selection-modal",
                        p { class: "skill-selection-hint", "请选择要恢复PP的技能：" }
                        div { class: "skill-selection-list",
                            for skill in skills {
                                {
                                    let skill_id = skill.id;
                                    let skill_name = skill.name.clone();
                                    let skill_pp = skill.current_pp;
                                    let skill_max_pp = skill.max_pp;
                                    let pp_class = if skill_pp == 0 {
                                        "low"
                                    } else if skill_max_pp > 0 && skill_pp <= skill_max_pp / 3 {
                                        "warn"
                                    } else {
                                        ""
                                    };
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            class: "skill-selection-item",
                                            "data-testid": "pp-skill-{skill_id}",
                                            "aria-disabled": loading,
                                            disabled: loading,
                                            onclick: move |_| {
                                                if !loading {
                                                    on_select_skill.call(skill_id);
                                                }
                                            },
                                            div { class: "skill-selection-info",
                                                span { class: "skill-selection-name", "{skill_name}" }
                                                span { class: "skill-selection-pp {pp_class}", "PP: {skill_pp}/{skill_max_pp}" }
                                            }
                                            span { class: "skill-selection-action", "选择" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 统一的战斗事件 Modal
            if let Some(event) = &*battle_event.read() {
                Modal {
                    is_open: true,
                    close_on_overlay: !loading && !pokemon_fainted,
                    on_close: move |_| {
                        if loading || pokemon_fainted {
                            return;
                        }
                        if is_battle_end {
                            on_end.call(());
                        }
                        battle_event.set(None);
                    },
                    title: match event {
                        BattleEvent::SwitchPokemon => "切换上场宠物".to_string(),
                        BattleEvent::ReplacePokemon(_) => "选择替换宠物".to_string(),
                        BattleEvent::BattleEnd => match battle.status {
                            BattleStatus::Victory => "战斗胜利！".to_string(),
                            BattleStatus::Defeat => "战斗失败...".to_string(),
                            BattleStatus::Fled => "成功逃脱！".to_string(),
                            BattleStatus::Captured => "捕捉成功！".to_string(),
                            BattleStatus::Active => "战斗结束".to_string(),
                        },
                    },
                    match event {
                        BattleEvent::SwitchPokemon => rsx! {
                            div { class: "switch-pokemon-modal",
                                p { class: "switch-pokemon-warning",
                                    "⚠️ 切换宠物可能会让当前上场宠物再受到一次攻击！"
                                }
                                p { class: "switch-pokemon-hint", "确定要切换吗？" }
                                div { class: "switch-pokemon-actions",
                                    button {
                                        class: "btn btn-secondary",
                                        onclick: move |_| battle_event.set(None),
                                        "取消"
                                    }
                                    button {
                                        class: "btn btn-primary",
                                        disabled: loading,
                                        onclick: move |_| {
                                            battle_event.set(None);
                                            on_switch_pokemon.call(());
                                        },
                                        "确定切换"
                                    }
                                }
                            }
                        },
                        BattleEvent::ReplacePokemon(fainted_name) => rsx! {
                            div { class: "faint-replace-modal",
                                p { class: "faint-message",
                                    "{fainted_name} 倒下了！请选择一只宠物继续战斗："
                                }
                                if POKEMON_STATE.read().loading {
                                    p { "正在加载可替换的宠物..." }
                                }
                                div { class: "replacement-pokemon-list",
                                    {
                                        let all_pokemons: Vec<PokemonBasic> = POKEMON_STATE
                                            .read()
                                            .list
                                            .iter()
                                            .filter(|p| p.site == 1 || p.site == 2)
                                            .cloned()
                                            .collect();
                                        let fainted_instance_id = battle.my_pokemon.instance_id;
                                        let on_replace = on_replace_pokemon;

                                        rsx! {
                                            for pm in &all_pokemons {
                                                {
                                                    let pm_clone = pm.clone();
                                                    let pm_id = pm_clone.id;
                                                    let on_replace_clone = on_replace;
                                                    let is_the_fainted_one = pm_clone.id == fainted_instance_id;
                                                    let is_fainted = is_the_fainted_one || pm_clone.hp <= 0;
                                                    let is_first = pm_clone.site == 1;
                                                    let display_hp = if is_the_fainted_one { 0 } else { pm_clone.hp };
                                                    rsx! {
                                                        button {
                                                            key: "replace-{pm_id}",
                                                            "data-testid": "replacement-pokemon-{pm_id}",
                                                            class: if is_fainted {
                                                                "replacement-pokemon-card disabled"
                                                            } else {
                                                                "replacement-pokemon-card"
                                                            },
                                                            disabled: loading || is_fainted,
                                                            onclick: move |_| {
                                                                if !loading && !is_fainted {
                                                                    on_replace_clone.call(pm_id);
                                                                }
                                                            },
                                                            img {
                                                                class: "replacement-pokemon-sprite",
                                                                src: "{IMG_PATH}/spm/{pm_clone.type_id}.gif",
                                                                alt: "{pm_clone.name}",
                                                            }
                                                            div { class: "replacement-pokemon-info",
                                                                div { class: "replacement-pokemon-name",
                                                                    "{pm_clone.nickname.as_ref().unwrap_or(&pm_clone.name)}"
                                                                    if is_first {
                                                                        span { class: "first-badge", "首位" }
                                                                    }
                                                                    if is_the_fainted_one {
                                                                        span { class: "fainted-badge", "已倒下" }
                                                                    }
                                                                }
                                                                div { class: "replacement-pokemon-level",
                                                                    "Lv.{pm_clone.level}"
                                                                }
                                                            }
                                                            div { class: "replacement-pokemon-hp",
                                                                span {
                                                                    class: if is_fainted { "hp-text fainted" } else { "hp-text" },
                                                                    "HP: {display_hp}/{pm_clone.max_hp}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                // 添加逃跑按钮（被动切换时可以无惩罚逃跑）
                                div { class: "faint-replace-actions",
                                    button {
                                        class: "btn btn-warning",
                                        disabled: loading,
                                        onclick: move |_| {
                                            on_flee.call(());
                                        },
                                        "🏃 逃跑（无惩罚）"
                                    }
                                }
                            }
                        },
                        BattleEvent::BattleEnd => rsx! {
                            BattleResultSection {
                                status: battle.status.clone(),
                                rewards: battle.rewards.clone(),
                                level_up: battle.level_up.clone(),
                                my_pokemon_name: battle.my_pokemon.name.clone(),
                                can_continue,
                                loading,
                                on_view_log: move |_| show_log.set(true),
                                on_end: move |_| {
                                    battle_event.set(None);
                                    on_end.call(())
                                },
                                on_continue: move |_| {
                                    battle_event.set(None);
                                    on_continue.call(())
                                },
                            }
                        },
                    }
                }
            }
            if *show_log.read() {
                BattleLogPanel { battle_id: battle.engine_battle_id, on_close: move |_| show_log.set(false) }
            }
        }
    }
}

#[component]
fn WildPokemonSection(pokemon: _utils::types::api_battle::WildPokemon) -> Element {
    let hp_pct = hp_percent(pokemon.hp, pokemon.max_hp);
    let hp_class = hp_class(pokemon.hp, pokemon.max_hp);

    rsx! {
        div { class: "enemy-pokemon-block",
            div { class: "enemy-sprite-container",
                img {
                    class: "enemy-sprite",
                    src: "{IMG_PATH_REMOTE}/pm/{pokemon.id}.gif",
                    alt: "{pokemon.name}",
                }
            }

            div { class: "enemy-info-box",
                div { class: "enemy-header",
                    div { class: "enemy-name-row",
                        span { class: "enemy-name", "{pokemon.name}" }
                        if pokemon.is_shiny {
                            span { class: "shiny-icon", "★" }
                        }
                        span { class: "enemy-gender {gender_class(pokemon.gender)}",
                            "{gender_symbol(pokemon.gender)}"
                        }
                    }
                    span { class: "enemy-level-text", "Lv.{pokemon.level}" }
                }

                div { class: "enemy-hp-row",
                    div { class: "hp-bar-bg",
                        div {
                            class: "hp-bar-fg {hp_class}",
                            style: "width: {hp_pct}%;",
                        }
                    }
                    span { class: "hp-text", "{pokemon.hp}/{pokemon.max_hp}" }
                }
            }
        }
    }
}

#[component]
fn MyPokemonSection(
    pokemon: _utils::types::api_battle::BattlePokemon,
    #[props(default)] on_click: EventHandler<()>,
) -> Element {
    let hp_pct = hp_percent(pokemon.hp, pokemon.max_hp);
    let hp_class = hp_class(pokemon.hp, pokemon.max_hp);
    let mut show_tooltip = use_signal(|| false);

    rsx! {
        div { class: "player-pokemon-block",
            onclick: move |_| on_click.call(()),
            onmouseenter: move |_| show_tooltip.set(true),
            onmouseleave: move |_| show_tooltip.set(false),
            div { class: "player-info-box",
                div { class: "player-header",
                    div { class: "player-name-row",
                        span { class: "player-name", "{pokemon.name}" }
                        span { class: "player-level", "Lv.{pokemon.level}" }
                    }
                }

                div { class: "player-hp-row",
                    div { class: "hp-bar-bg",
                        div {
                            class: "hp-bar-fg {hp_class}",
                            style: "width: {hp_pct}%;",
                        }
                    }
                    span { class: "hp-text", "{pokemon.hp}/{pokemon.max_hp}" }
                }
            }

            div { class: "player-sprite-container",
                img {
                    class: "player-sprite",
                    src: "{IMG_PATH_REMOTE}/pmb/{pokemon.id}.gif",
                    alt: "{pokemon.name}",
                }
            }

            // 宠物详情 Tooltip
            if *show_tooltip.read() {
                div { class: "pokemon-tooltip",
                    div { class: "pokemon-tooltip-header",
                        div { class: "tooltip-name-row",
                            span { class: "tooltip-name", "{pokemon.name}" }
                            span { class: "tooltip-level", "Lv.{pokemon.level}" }
                        }
                    }
                    div { class: "pokemon-tooltip-stats",
                        div { class: "tooltip-stat",
                            span { class: "stat-label", "HP" }
                            span { class: "stat-value", "{pokemon.hp}/{pokemon.max_hp}" }
                        }
                    }
                    div { class: "pokemon-tooltip-skills",
                        for skill in pokemon.skills.iter() {
                            div { class: "tooltip-skill-row",
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
}

#[component]
fn SkillsTabContent(
    skills: Vec<BattleSkill>,
    loading: bool,
    on_use_skill: EventHandler<u64>,
    on_attack: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "skills-panel",
            // 左侧：普通攻击大按钮
            div { class: "attack-button-section",
                button {
                    class: if loading { "normal-attack-btn disabled" } else { "normal-attack-btn" },
                    onclick: move |_| on_attack.call(()),
                    disabled: loading,
                    span { class: "attack-btn-icon", "⚔" }
                    span { class: "attack-btn-label", "普通攻击" }
                }
            }

            // 右侧：技能网格（2×2）
            div { class: "skills-grid-section",
                div { class: "skills-grid",
                    // 显示已有技能
                    for (_idx , skill) in skills.iter().enumerate() {
                        {
                            let skill_id = skill.id;
                            let is_disabled = loading || skill.pp == 0;
                            let pp_class = get_pp_class(skill.pp, skill.max_pp);
                            rsx! {
                                button {
                                    key: "skill-{skill_id}",
                                    class: if is_disabled { "skill-btn disabled" } else { "skill-btn" },
                                    onclick: move |_| on_use_skill.call(skill_id),
                                    disabled: is_disabled,
                                    div { class: "skill-btn-content",
                                        div { class: "skill-name", "{skill.name}" }
                                        TypeBadge {
                                            skill_type: skill.skill_type.clone(),
                                            category: skill.category.clone(),
                                        }
                                        div { class: "skill-stats-row",
                                            if skill.category == "特攻" || skill.category == "特殊" {
                                                span { class: "skill-power",
                                                    "特攻："
                                                    strong { "{skill.power}" }
                                                }
                                            } else if skill.category == "物攻" || skill.category == "物理" {
                                                span { class: "skill-power",
                                                    "物攻："
                                                    strong { "{skill.power}" }
                                                }
                                            } else {
                                                span { class: "skill-power",
                                                    "威力："
                                                    strong { "{skill.power}" }
                                                }
                                            }
                                            span { class: "skill-pp {pp_class}",
                                                "PP："
                                                strong {
                                                    "{skill.pp}"
                                                    if skill.max_pp > 0 {
                                                        "/{skill.max_pp}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 填充空技能槽（虚线框）
                    for i in skills.len()..4 {
                        div { key: "empty-{i}", class: "empty-skill-slot",
                            span { class: "empty-slot-text", "空" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ItemsTabContent(
    loading: bool,
    items: Vec<BattleItem>,
    my_pokemon: BattlePokemon,
    on_use_item: EventHandler<u64>,
) -> Element {
    rsx! {
        div { class: "items-panel",
            if items.is_empty() {
                div { class: "empty-items-hint", "暂无道具" }
            } else {
                div { class: "battle-items-grid",
                    for (_idx , item_data) in items.iter().enumerate() {
                        {
                            let item_id = item_data.id;
                            let is_disabled = loading || !item_data.can_use_on(&my_pokemon);
                            let item_for_card = battle_item_for_card(item_data);
                            let condition = if my_pokemon.hp <= 0 {
                                "宠物已倒下，请先替换宠物。"
                            } else if item_data.is_pp_restore() {
                                if item_data.can_use_on(&my_pokemon) { "选择一个PP未满的技能，恢复后对方会反击。" }
                                else { "所有技能PP已满，暂时不需要恢复。" }
                            } else if item_data.can_use_on(&my_pokemon) {
                                "恢复当前宠物的HP，使用后对方会反击。"
                            } else { "当前HP已满，暂时不需要恢复。" }.to_string();

                            rsx! {
                                BattleItemCard {
                                    item: item_for_card,
                                    on_click: move |_| on_use_item.call(item_id),
                                    show_tooltip: true,
                                    disabled: is_disabled,
                                    busy: loading,
                                    condition,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 复用道具卡的显示字段，保留战斗 API 的种类 ID 与模块语义。
fn battle_item_for_card(item: &BattleItem) -> InventoryItem {
    InventoryItem {
        id: item.id,
        type_id: item.id,
        item_type: item.item_type,
        use_target: None,
        name: item.name.clone(),
        description: if item.module == "pp99" {
            "完全恢复一个技能的PP".to_string()
        } else if item.is_pp_restore() {
            format!(
                "恢复一个技能的 {} 点PP",
                item.module.trim_start_matches("pp")
            )
        } else {
            format!("恢复 {} HP", item.addhp)
        },
        image: item.img.clone(),
        quantity: item.nums,
        nums: item.nums,
        type_name: if item.is_pp_restore() {
            "PP恢复"
        } else {
            "HP恢复"
        }
        .to_string(),
        can_use: item.nums > 0,
        addhp: item.addhp,
    }
}

#[component]
fn CaptureTabContent(
    loading: bool,
    balls: Vec<InventoryItem>,
    on_use_ball: EventHandler<u64>,
) -> Element {
    rsx! {
        div { class: "capture-panel",
            if balls.is_empty() {
                div { class: "empty-items-hint", "暂无精灵球" }
            } else {
                div { class: "battle-items-grid",
                    for (_idx , ball_data) in balls.iter().enumerate() {
                        {
                            let ball_for_click = ball_data.clone();
                            let ball_type_id = ball_for_click.type_id;

                            rsx! {
                                BattleItemCard {
                                    item: ball_for_click,
                                    on_click: move |_| on_use_ball.call(ball_type_id),
                                    show_tooltip: true,
                                    is_ball: true,
                                    disabled: loading,
                                    busy: loading,
                                    condition: "用于捕捉当前野生宠物；捕捉失败时对方可能反击。".to_string(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BattleItemCard(
    item: InventoryItem,
    on_click: EventHandler<u64>,
    show_tooltip: bool,
    #[props(default = false)] is_ball: bool,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] busy: bool,
    #[props(default)] condition: String,
) -> Element {
    let mut tooltip_visible = use_signal(|| false);
    let mut tooltip_position = use_signal(|| (0.0f64, 0.0f64));
    let mut details_open = use_signal(|| false);

    let image_src = if item.image.contains('.') {
        item.image.clone()
    } else {
        format!("{}.gif", item.image)
    };

    let card_class = if disabled {
        "battle-item-card disabled"
    } else if is_ball {
        "battle-item-card capture-ball-card"
    } else {
        "battle-item-card"
    };

    // 优先使用 nums（战斗API返回），否则使用 quantity
    let qty = if item.nums > 0 {
        item.nums
    } else {
        item.quantity
    };

    let item_for_click = item.clone();

    rsx! {
        div { class: "battle-item-card-wrapper",
            button {
                r#type: "button",
                class: card_class,
                "data-testid": if is_ball { format!("battle-ball-{}", item.type_id) } else { format!("battle-item-{}", item.id) },
                disabled: busy,
                "aria-label": format!("查看 {}", item.name),
                "aria-disabled": busy,
                onclick: move |_| {
                    if !busy { tooltip_visible.set(false); details_open.set(true); }
                },
                onmouseenter: move |evt: Event<MouseData>| {
                    if show_tooltip {
                        let coords = evt.data().client_coordinates();
                        tooltip_position.set((coords.x, coords.y));
                        tooltip_visible.set(true);
                    }
                },
                onmouseleave: move |_| {
                    tooltip_visible.set(false);
                },
                div { class: "battle-item-icon",
                    img {
                        src: "{IMG_PATH}/item/{image_src}",
                        alt: "{item.name}",
                    }
                }
                span { class: "battle-item-qty", "{qty}" }
            }
            if *tooltip_visible.read() && show_tooltip {
                BattleItemTooltip {
                    item: item.clone(),
                    left: tooltip_position.read().0,
                    top: tooltip_position.read().1,
                }
            }
            if *details_open.read() {
                Modal {
                    is_open: true, title: item.name.clone(),
                    on_close: move |_| details_open.set(false),
                    div { class: "battle-item-details", "data-testid": "battle-item-details",
                        p { "效果：{item.description}" }
                        p { "持有数量：{qty}" }
                        p { "适用条件：{condition}" }
                        button { class: "btn btn-primary", "data-testid": "battle-item-use",
                            disabled: disabled || busy || qty <= 0,
                            onclick: move |_| {
                                if !disabled && !busy && qty > 0 {
                                    details_open.set(false);
                                    on_click.call(item_for_click.id);
                                }
                            },
                            if is_ball { "使用精灵球" } else { "使用道具" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BattleItemTooltip(item: InventoryItem, left: f64, top: f64) -> Element {
    let img_src = if item.image.contains('.') {
        item.image.clone()
    } else {
        format!("{}.gif", item.image)
    };

    let tooltip_width = 180.0;
    let tooltip_left = left - tooltip_width / 2.0;
    let tooltip_top = top + 30.0;

    rsx! {
        div {
            class: "battle-item-tooltip",
            style: "left: {tooltip_left}px; top: {tooltip_top}px;",
            div { class: "tooltip-header",
                img {
                    class: "tooltip-icon",
                    src: "{IMG_PATH}/item/{img_src}",
                    alt: "{item.name}",
                }
                span { class: "tooltip-name", "{item.name}" }
            }
            if !item.description.is_empty() {
                div { class: "tooltip-desc", "{item.description}" }
            }
            div { class: "tooltip-info",
                div { class: "tooltip-type", "类型: {item.type_name}" }
                div { class: "tooltip-qty", "数量: {item.quantity}" }
            }
        }
    }
}

#[component]
fn FleeTabContent(loading: bool, on_flee: EventHandler<()>) -> Element {
    rsx! {
        div { class: "flee-panel",
            button {
                class: if loading { "flee-button disabled" } else { "flee-button" },
                onclick: move |_| on_flee.call(()),
                disabled: loading,
                span { class: "flee-icon", "❌" }
                span { "退出战斗" }
            }
        }
    }
}

#[component]
fn BattleResultSection(
    status: BattleStatus,
    rewards: Option<_utils::types::api_battle::BattleRewards>,
    level_up: Option<_utils::types::api_battle::LevelUpInfo>,
    my_pokemon_name: String,
    can_continue: bool,
    loading: bool,
    on_end: EventHandler<()>,
    on_continue: EventHandler<()>,
    on_view_log: EventHandler<()>,
) -> Element {
    let (result_text, result_class) = match status {
        BattleStatus::Victory => ("🎉 战斗胜利！", "victory"),
        BattleStatus::Defeat => ("💔 战斗失败...", "defeat"),
        BattleStatus::Fled => ("🏃 成功逃脱！", "fled"),
        BattleStatus::Captured => ("🎉 捕捉成功！", "victory"),
        BattleStatus::Active => ("", ""),
    };

    rsx! {
        div { class: "battle-result-section",
            div { class: "battle-result {result_class}",
                h2 { "{result_text}" }

                if let Some(r) = rewards {
                    div { class: "battle-rewards",
                        h4 { "获得奖励" }
                        div { class: "reward-items",
                            div { class: "reward-item",
                                span { class: "reward-icon", "⭐" }
                                span { class: "reward-value", "{r.exp} 经验" }
                            }
                            div { class: "reward-item",
                                span { class: "reward-icon", "💰" }
                                span { class: "reward-value", "{r.money} 金币" }
                            }
                        }
                    }
                }

                if let Some(lu) = level_up {
                    if lu.level_up {
                        div { class: "level-up-notice",
                            span { class: "level-up-icon", "⬆️" }
                            span { class: "level-up-text", "{my_pokemon_name} 升级了！" }
                            span { class: "level-up-levels", "Lv.{lu.old_level} → Lv.{lu.new_level}" }
                        }
                    }
                }
            }

            div { class: "battle-end-buttons",
                button { class: "battle-end-btn", "data-testid": "battle-end-log", onclick: move |_| on_view_log.call(()), "查看 / 分享战报" }
                button { class: "battle-end-btn", disabled: loading, onclick: move |_| on_end.call(()), "返回冒险地图" }
                if can_continue {
                    button { class: "battle-continue-btn", disabled: loading, onclick: move |_| on_continue.call(()), "继续战斗" }
                }
            }
        }
    }
}

fn gender_symbol(gender: u8) -> &'static str {
    match gender {
        0 => "♂",
        1 => "♀",
        _ => "",
    }
}

fn gender_class(gender: u8) -> &'static str {
    match gender {
        0 => "male",
        1 => "female",
        _ => "",
    }
}

fn get_pp_class(pp: u64, max_pp: u64) -> &'static str {
    if pp == 0 {
        "low"
    } else if max_pp > 0 && pp <= max_pp / 3 {
        "warn"
    } else {
        ""
    }
}
