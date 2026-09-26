use crate::dioxus::prelude::*;

use crate::dioxus::state::ADMIN_TOOLTIP;

/// 全局Tooltip容器组件 - 渲染在根级别，不受父元素层叠上下文影响
#[component]
pub fn TooltipContainer() -> Element {
    let tooltip = ADMIN_TOOLTIP();

    if !tooltip.visible || tooltip.text.is_empty() {
        return rsx! {};
    }

    // tooltip.x 是锚点的中心X坐标，tooltip.y 是锚点的底部Y坐标
    let x = tooltip.x;
    let y = tooltip.y + 6.0; // 在锚点下方6px处显示

    rsx! {
        div {
            class: "admin-tooltip-portal",
            style: "left:{x}px;top:{y}px;",
            {tooltip.text.clone()}
        }
    }
}
