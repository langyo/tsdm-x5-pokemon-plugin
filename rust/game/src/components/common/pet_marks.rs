use crate::prelude::*;

/// 拼接 pet-mark 的 class：基础标记 + 种类修饰符 + 可选额外 class。
fn pet_mark_class(modifier: &str, extra: Option<String>) -> String {
    match extra {
        Some(extra) if !extra.is_empty() => format!("pet-mark {modifier} {extra}"),
        _ => format!("pet-mark {modifier}"),
    }
}

/// 闪光（异色）宠物标记 — 在宠物名字旁统一显示「闪」字徽标。
#[component]
pub fn ShinyMark(
    /// 额外的 class（如迷你网格上的紧凑样式 `pet-mark-compact`）
    #[props(default = None)]
    class: Option<String>,
) -> Element {
    let class_name = pet_mark_class("pet-mark-shiny", class);

    rsx! {
        span {
            class: "{class_name}",
            title: "异色（闪光）宠物",
            "闪"
        }
    }
}

/// 神兽 / 论坛活动定制宠物标记 — 在宠物名字旁统一显示「神」字徽标。
#[component]
pub fn GodMark(
    /// 额外的 class（如迷你网格上的紧凑样式 `pet-mark-compact`）
    #[props(default = None)]
    class: Option<String>,
) -> Element {
    let class_name = pet_mark_class("pet-mark-god", class);

    rsx! {
        span {
            class: "{class_name}",
            title: "神兽 / 活动定制宠物",
            "神"
        }
    }
}
