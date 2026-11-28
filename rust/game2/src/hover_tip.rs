//! 悬浮提示（旧版各页的悬浮提示）：hikari `Popover` 的锚定分支承接。
//!
//! 旧版里跟着指针走的那几层说明——装备槽 / 物品栏卡片
//! （`EquipmentTooltipInline`）、商店的商品 / 宠物瓦片（`ShopItemTooltip` /
//! `PetTooltip`）、战斗页的道具卡（`BattleItemTooltip`）——都是同一个形状：
//! 指针进入时记下坐标，挂出一个 `position: fixed`、内联 `left` / `top`、
//! `pointer-events: none` 的 div，指针离开时摘掉。
//!
//! 端口把它们收成应用级的一席（`AppState::hover_tip`）。面板是 `Popover`，零尺寸
//! 锚点加 `BottomStart`、偏移 0，左上角正好落在各页自己算出的点上；`class` 给旧版
//! 的类名，底色、阴影、内边距、宽度上下限、层叠和 `pointer-events: none` 都是
//! game.css 里那条旧规则给的。不要背板、不要 Esc：它只是跟着指针的一层说明。各页
//! 只管三件事：定位函数、正文、进入 / 离开时调用 [`show`] / [`hide`]。
//!
//! 提示挂在哪儿有两种，按旧版把它画在哪儿来定：
//! - 挂到应用根部（[`provider`]）：旧版卡片外面没有变换，固定定位就是视口坐标。
//!   装备槽 / 物品栏卡片、战斗道具卡是这样。
//! - 就地渲染（[`surface_for`]）：旧版把提示画在带 `transform` 的卡片里，固定定位
//!   于是相对那张卡片的内边距盒落位。商店瓦片是这样——悬停时瓦片上移 2px，提示的
//!   内联 `left` / `top` 是按视口算的，实际却落在「瓦片左上角 + 那两个数」的位置
//!   （Dioxus 版的实际效果，端口照搬；见 `HoverTip::in_place_owner`）。
//!
//! 两处旧版提示没有挪过来：侧栏迷你格的 `.pm-mini-tooltip` 和战斗页玩家块的
//! `.pokemon-tooltip` 是 CSS 就地定位的（前者 `position: fixed` 不带偏移、留在流内
//! 位置，后者 `position: absolute` 挂在块右侧），不跟指针走，也不需要一层浮出来
//! 的面板——它们照旧是卡片里的 Dioxus 标记。
//!
//! 为什么不是 hikari `Tooltip`（`feedback/tooltip.rs`）：
//! - 气泡只有一段文字（`text` 属性），装不下旧版的内容（图标 + 名称 + 描述 +
//!   加成标签 + 一行提示）；
//! - 它按触发元素的盒子居中锚定，还要把触发元素包进 `span.hk-tooltip-wrapper` /
//!   `span.hk-tooltip-trigger`；旧版跟指针走，卡片外面什么也不多包；
//! - 在 wasm32-wasip2 上它不出现：hikari 只在 `wasm32-unknown-unknown` 上编译
//!   web 平台，这个目标是 stub（`platform/stub.rs`——`set_timeout` 丢掉回调、
//!   `query_selector` / `get_bounding_client_rect` 恒为 None），延迟显示的计时器
//!   不会触发，触发的盒子也量不到（2026-11-28 实测：悬停两秒后气泡仍是
//!   `opacity: 0`）。`Popover` 不碰平台层，位置全由 `anchor` 给。

use std::rc::Rc;

use hikari_components::{Popover, PopoverAnchorRect, PopoverPlacement, PopoverProps};
use tairitsu_macros::rsx;
use tairitsu_vdom::VNode;

use crate::app_state::AppState;
use crate::icon::{Icon, IconName, IconProps};
use crate::popup_types::equipment_tooltip_position;
use crate::state::{EquipmentItem, HoverTip};

const IMG_PATH: &str = "source/plugin/pokemon/images";

/// 指针进入卡片：挂出提示。`content` 在每次渲染时调用，页面数据变了内容跟着变。
pub fn show(state: &AppState, tip: HoverTip) {
    state.hover_tip.inner().set(Some(tip));
    crate::request_remount();
}

/// 指针离开卡片：收起。卡片被换掉（切标签页、换页、开详情弹窗）时也调它——
/// 那时鼠标未必「离开」过卡片，而旧版的提示是随卡片组件一起销毁的。
pub fn hide(state: &AppState) {
    if state.hover_tip.get().is_some() {
        state.hover_tip.inner().set(None);
        crate::request_remount();
    }
}

/// 一层提示的面板：hikari `Popover` 的锚定分支，带着旧类名，左上角落在 `(x, y)`。
fn surface(tip: &HoverTip) -> VNode {
    Popover(PopoverProps {
        open: true,
        placement: PopoverPlacement::BottomStart,
        offset: 0.0,
        // 说明层不挡页面：没有背板，也不参与外部点击与 Esc。
        backdrop: false,
        close_on_backdrop: false,
        close_on_escape: false,
        // 外观全由 game.css 里带旧类名的那条规则给。
        glass: false,
        anchor: Some(PopoverAnchorRect {
            x: tip.x,
            y: tip.y,
            width: 0.0,
            height: 0.0,
        }),
        class: tip.class.to_string(),
        children: (tip.content)(),
        ..Default::default()
    })
}

/// 应用根部常驻的一席：有卡片被悬浮、且那张卡片不自己渲染提示时才出节点
/// （旧版 `if *show_tooltip.read() { … }`）。
pub fn provider(state: &AppState) -> Option<VNode> {
    let tip = state.hover_tip.get()?;
    if tip.in_place_owner.is_some() {
        return None;
    }
    Some(surface(&tip))
}

/// 就地渲染：当前的提示属于下标为 `owner` 的卡片时，给出它的面板，由那张卡片放在
/// 自己里面（`HoverTip::in_place_owner`）。卡片外面 hikari 还包一层
/// `div[data-hk-popout-layer]`，皮肤把它设成 `display: contents`，面板于是和旧版
/// 一样直接落在卡片里（`styles/main.scss`）。
pub fn surface_for(state: &AppState, owner: usize) -> Option<VNode> {
    let tip = state.hover_tip.get()?;
    (tip.in_place_owner == Some(owner)).then(|| surface(&tip))
}

// ---------------------------------------------------------------------------
// 装备槽 / 物品栏卡片（旧版 `EquipmentTooltipInline`）
// ---------------------------------------------------------------------------

/// 指针进入装备槽或物品栏卡片：按旧版的算法定位（见
/// [`equipment_tooltip_position`]），正文是进入那一刻的物品。
pub fn show_equipment(state: &AppState, item: EquipmentItem, client_x: i32, client_y: i32) {
    let (x, y) = equipment_tooltip_position(client_x as f64, client_y as f64);
    show(
        state,
        HoverTip {
            class: "equipment-tooltip-inline",
            x,
            y,
            in_place_owner: None,
            content: Rc::new(move || equipment_content(&item)),
        },
    );
}

/// 旧版 `EquipmentTooltipInline` 的正文：标题行（图标 + 名称）、描述、非零的加成
/// 标签、被占用时的提示行。
fn equipment_content(item: &EquipmentItem) -> VNode {
    let img_src = if item.image.contains('.') {
        item.image.clone()
    } else {
        format!("{}.gif", item.image)
    };
    let src = format!("{IMG_PATH}/item/{img_src}");
    let alt = item.name.clone();
    let name = item.name.clone();
    let description = item.description.clone();

    let stats: Vec<VNode> = [
        ("hp", "HP", item.equipment_hp),
        ("atk", "攻击", item.equipment_atk),
        ("def", "防御", item.equipment_def),
        ("spat", "特攻", item.equipment_spatk),
        ("spdef", "特防", item.equipment_spdef),
        ("spd", "速度", item.equipment_sd),
    ]
    .into_iter()
    .filter(|(_, _, value)| *value > 0)
    .map(|(class, label, value)| {
        let class = format!("tooltip-stat {class}");
        let text = format!("{label} +{value}");
        rsx! {
            div { class: class, "{text}" }
        }
    })
    .collect();

    let hint = if item.is_equipped {
        Some("此装备已被装备")
    } else if item.equipped_count > 0 {
        Some("其他宠物使用中，请先卸下装备")
    } else {
        None
    };
    let hint_node = match hint {
        Some(text) => {
            let text = text.to_string();
            rsx! {
                div { class: "tooltip-equipped-hint",
                    {Icon(IconProps {
                        name: Some(IconName::TriangleAlert),
                        size: Some(12),
                        class: Some("lucide-inline".to_string()),
                    })}
                    "{text}"
                }
            }
        }
        None => VNode::empty(),
    };

    let mut nodes: Vec<VNode> = Vec::with_capacity(4);
    nodes.push(rsx! {
        div { class: "tooltip-header",
            img { class: "tooltip-icon", src: src, alt: alt }
            span { class: "tooltip-name", "{name}" }
        }
    });
    if !description.is_empty() {
        nodes.push(rsx! {
            div { class: "tooltip-desc", "{description}" }
        });
    }
    nodes.push(rsx! {
        div { class: "tooltip-stats",
            for i in 0..stats.len() {
                ..vec![stats[i].clone()]
            }
        }
    });
    nodes.push(hint_node);
    VNode::Fragment(nodes)
}
