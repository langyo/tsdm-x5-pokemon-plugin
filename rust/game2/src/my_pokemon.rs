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

use crate::app_state::AppState;
use crate::quality::{quality_grade, quality_rank, UNKNOWN_RANK};

const IMG_PATH: &str = "source/plugin/pokemon/images";
/// 昵称上限（对齐 Dioxus 版输入框）。
const NICKNAME_MAX: usize = 14;

/// 首次进入装载：拉宠物列表，并给当前选中宠物拉详情。
pub fn ensure_loaded(state: &AppState) {
    if state.center.pokemons.get().is_none() {
        load_list(state);
    } else {
        ensure_detail(state);
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
    let Some((id, name, is_shiny)) = pick_target(&list, state.my_pm.selected_id.get()) else {
        state.my_pm.detail.inner().set(Some(Ok(Default::default())));
        crate::request_remount();
        return;
    };
    let detail = state.my_pm.detail.get();
    let already = detail
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .map(|d| d.id == id && !d.name.is_empty())
        .unwrap_or(false);
    if already {
        return;
    }
    let _ = (name, is_shiny);
    let my = state.my_pm.clone();
    crate::api::pokemon_detail(
        id,
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
) -> Option<(u64, String, bool)> {
    let pm = selected
        .and_then(|id| list.iter().find(|p| p.id == id))
        .or_else(|| list.first())?;
    Some((pm.id, pm.display_name(), pm.is_shiny))
}

/// 切换选中的宠物。
pub fn select_pokemon(state: &AppState, id: u64) {
    if state.my_pm.selected_id.get() == Some(id) {
        return;
    }
    let my = state.my_pm.clone();
    my.selected_id.inner().set(Some(id));
    my.detail.inner().set(None);
    // 切换宠物时退出编辑态，避免把上一只的输入带过去。
    my.editing.inner().set(false);
    my.save_error.inner().set(None);
    my.save_success.inner().set(false);
    crate::request_remount();
    ensure_detail(state);
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
    } else if let Some((pm_id, pm_name, _shiny)) = current.clone() {
        let selector = pet_selector(&list, pm_id, &st);
        let panel = match &detail {
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
                    let hint = format!("正在加载 {pm_name} 的属性...");
                    rsx! { div { class: "loading", "{hint}" } }
                }
            },
        };
        rsx! {
            div { class: "mypm-content-area",
                ..vec![selector]
                div { class: "mypm-divider" }
                div { class: "mypm-main-content",
                    ..vec![panel]
                }
            }
        }
    } else {
        rsx! { div { class: "empty-hint", "你还没有宠物，快去冒险捕捉吧！" } }
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

    let sprite = format!("{IMG_PATH}/spm/{}.gif", d.type_id);

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
            let original = d.name.clone();
            let nick = display_name.clone();
            rsx! {
                span { class: "pokemon-name-with-original",
                    span { class: "pokemon-name", "{nick}" }
                    span { class: "original-name", " ({original})" }
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
                }
                ..vec![radar]
            }
            div { class: "pokemon-sprite-area",
                img { class: "pokemon-sprite-img", src: sprite, alt: display_name }
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
