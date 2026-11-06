//! 内联 lucide SVG 图标（与 Dioxus 版 Icon 组件同一份路径数据，
//! lucide-static v0.544.0, ISC License）。tairitsu 没有自己的图标组件，
//! 这里以 dangerous_inner_html 复用同一份 markup，保证 1:1 视觉。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

/// 组件用到的图标子集（商店页）。
pub fn markup(name: IconName) -> &'static str {
    match name {
        IconName::ChevronLeft => r#"<path d="m15 18-6-6 6-6" />"#,
        IconName::ChevronRight => r#"<path d="m9 18 6-6-6-6" />"#,
        IconName::Coins => {
            r#"<circle cx="8" cy="8" r="6" /><path d="M18.09 10.37A6 6 0 1 1 10.34 18" /><path d="M7 6h1v4" /><path d="m16.71 13.88.7.71-2.82 2.82" />"#
        }
        IconName::Search => r#"<circle cx="11" cy="11" r="8" /><path d="m21 21-4.3-4.3" />"#,
        IconName::Shoppingcart => {
            r#"<circle cx="8" cy="21" r="1" /><circle cx="19" cy="21" r="1" /><path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" />"#
        }
        IconName::X => r#"<path d="M18 6 6 18" /><path d="m6 6 12 12" />"#,
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IconName {
    ChevronLeft,
    ChevronRight,
    Coins,
    Search,
    Shoppingcart,
    X,
}

/// 内联 SVG 图标。`class` 透传以便复用 game.css 的图标样式
/// （.header-icon / .price-icon 等，与 Dioxus 版同名同类）。
#[component]
pub fn Icon(name: IconName, size: u32, class: String) -> VNode {
    let size_str = size.to_string();
    let inner = markup(name);
    rsx! {
        svg {
            class: class,
            xmlns: "http://www.w3.org/2000/svg",
            width: size_str.clone(),
            height: size_str,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            dangerous_inner_html: inner,
        }
    }
}
