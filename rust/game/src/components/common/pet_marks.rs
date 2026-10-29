use crate::prelude::*;

/// 闪光（异色）宠物标记 — 在宠物名字旁统一显示「闪」字徽标。
#[component]
pub fn ShinyMark(
    /// 额外的 class（如迷你网格上的紧凑样式 `pet-mark-compact`）
    #[props(default = None)]
    class: Option<String>,
) -> Element {
    let class_name = match class {
        Some(extra) if !extra.is_empty() => format!("pet-mark pet-mark-shiny {extra}"),
        _ => "pet-mark pet-mark-shiny".to_string(),
    };

    rsx! {
        span {
            class: "{class_name}",
            title: "异色（闪光）宠物",
            "闪"
        }
    }
}
