//! 装备悬浮提示（旧版 `EquipmentTooltipInline`）：hikari `Popover` 的锚定分支承接。
//!
//! 旧版：装备槽（有物品时）和物品栏卡片在 `onmouseenter` 记下指针坐标、挂出
//! `div.equipment-tooltip-inline`（`position: fixed`、内联 `left` / `top`、
//! `pointer-events: none`），`onmouseleave` 摘掉。定位见
//! [`crate::popup_types::equipment_tooltip_position`]。
//!
//! 端口把同一份提示挂在应用根部（[`provider`]）：面板是 `Popover`，零尺寸锚点加
//! `BottomStart`、偏移 0，左上角正好落在旧版算出的点上；`class` 给
//! `equipment-tooltip-inline`，底色、阴影、内边距、宽度上下限、层叠 9999 和
//! `pointer-events: none` 都是 game.css 那条旧规则给的。不要背板、不要 Esc：它
//! 只是跟着指针的一层说明。
//!
//! 为什么不是 hikari `Tooltip`（`feedback/tooltip.rs`）：
//! - 气泡只有一段文字（`text` 属性），装不下旧版的内容（图标 + 名称 + 描述 +
//!   加成标签 + 一行提示）；
//! - 它按触发元素的盒子居中锚定，还要把触发元素包进两层 `span`；旧版跟指针走，
//!   卡片外面什么也不多包；
//! - 在 wasm32-wasip2 上它不工作：hikari 只在 `wasm32-unknown-unknown` 上编译
//!   web 平台，这个目标是 stub（`platform/stub.rs`——`set_timeout` 丢掉回调、
//!   `query_selector` / `get_bounding_client_rect` 恒为 None），延迟显示的计时器
//!   不会触发，触发的盒子也量不到。`Popover` 不碰平台层，位置全由 `anchor` 给。

use hikari_components::{Popover, PopoverAnchorRect, PopoverPlacement, PopoverProps};
use tairitsu_macros::rsx;
use tairitsu_vdom::VNode;

use crate::app_state::AppState;
use crate::icon::{Icon, IconName, IconProps};
use crate::popup_types::equipment_tooltip_position;
use crate::state::{EquipmentItem, EquipmentTooltip};

const IMG_PATH: &str = "source/plugin/pokemon/images";

/// 指针进入卡片：记下物品与坐标。旧版是每个卡片自己的 `show_tooltip` /
/// `tooltip_position`；这里放进 `my_pm.equip_tooltip`。
pub fn show(state: &AppState, item: EquipmentItem, client_x: i32, client_y: i32) {
    let tip = EquipmentTooltip {
        item,
        x: client_x as f64,
        y: client_y as f64,
    };
    // 同一张卡片上指针挪动不会重新触发 mouseenter；值相同就不派发。
    if state.my_pm.equip_tooltip.get().as_ref() != Some(&tip) {
        state.my_pm.equip_tooltip.inner().set(Some(tip));
        crate::request_remount();
    }
}

/// 指针离开卡片：收起。
pub fn hide(state: &AppState) {
    if state.my_pm.equip_tooltip.get().is_some() {
        state.my_pm.equip_tooltip.inner().set(None);
        crate::request_remount();
    }
}

/// 旧版 `EquipmentTooltipInline` 的正文：标题行（图标 + 名称）、描述、非零的加成
/// 标签、被占用时的提示行。
fn content(item: &EquipmentItem) -> VNode {
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

/// 应用根部常驻的一席：有卡片被悬浮才出节点（旧版 `if *show_tooltip.read() { … }`）。
pub fn provider(state: &AppState) -> Option<VNode> {
    let tip = state.my_pm.equip_tooltip.get()?;
    let (x, y) = equipment_tooltip_position(tip.x, tip.y);
    Some(Popover(PopoverProps {
        open: true,
        placement: PopoverPlacement::BottomStart,
        offset: 0.0,
        // 说明层不挡页面：没有背板，也不参与外部点击与 Esc。
        backdrop: false,
        close_on_backdrop: false,
        close_on_escape: false,
        // 外观全由 game.css 的 `.equipment-tooltip-inline` 给。
        glass: false,
        anchor: Some(PopoverAnchorRect {
            x,
            y,
            width: 0.0,
            height: 0.0,
        }),
        class: "equipment-tooltip-inline".to_string(),
        children: content(&tip.item),
        ..Default::default()
    }))
}
