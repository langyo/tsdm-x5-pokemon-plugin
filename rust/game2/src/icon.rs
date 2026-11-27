//! 内联 lucide SVG 图标（与 Dioxus 版 Icon 组件同一份路径数据，
//! lucide-static v0.544.0, ISC License）。
//!
//! tairitsu 没有自己的图标组件，hikari-icons 只有 MDI：游戏图标继续用
//! lucide 这一份。外层 `<svg>` 用 rsx 写（需要 tairitsu 在 SVG 命名空间里
//! 创建元素，见 celestia-island/tairitsu 的 SVG namespace 修复），路径数据
//! 以 dangerous_inner_html 复用同一份 markup，保证 1:1 视觉。
//!
//! 注意：tairitsu 的属性名原样透传，不做 Dioxus 那样的 `view_box → viewBox`
//! 映射。SVG 专有的驼峰 / 连字符属性必须写成最终名字（带引号的键）。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

/// 图标内部元素（lucide 官方 path/shape 数据，属性省略公共 stroke 设置）。
/// 与 Dioxus 版 `IconName` 同一套；`Search` 是 tairitsu 版商店筛选追加的。
pub fn markup(name: IconName) -> &'static str {
    match name {
        IconName::ArrowUp => r#"<path d="m5 12 7-7 7 7" /><path d="M12 19V5" />"#,
        IconName::Check => r#"<path d="M20 6 9 17l-5-5" />"#,
        IconName::ChevronLeft => r#"<path d="m15 18-6-6 6-6" />"#,
        IconName::ChevronRight => r#"<path d="m9 18 6-6-6-6" />"#,
        IconName::Circle => r#"<circle cx="12" cy="12" r="10" />"#,
        IconName::CircleX => {
            r#"<circle cx="12" cy="12" r="10" /><path d="m15 9-6 6" /><path d="m9 9 6 6" />"#
        }
        IconName::Coins => {
            r#"<circle cx="8" cy="8" r="6" /><path d="M18.09 10.37A6 6 0 1 1 10.34 18" /><path d="M7 6h1v4" /><path d="m16.71 13.88.7.71-2.82 2.82" />"#
        }
        IconName::Crown => {
            r#"<path d="M11.562 3.266a.5.5 0 0 1 .876 0L15.39 8.87a1 1 0 0 0 1.516.294L21.183 5.5a.5.5 0 0 1 .798.519l-2.834 10.246a1 1 0 0 1-.956.734H5.81a1 1 0 0 1-.957-.734L2.02 6.02a.5.5 0 0 1 .798-.519l4.276 3.664a1 1 0 0 0 1.516-.294z" /><path d="M5 21h14" />"#
        }
        IconName::Egg => r#"<path d="M12 2C8 2 4 8 4 14a8 8 0 0 0 16 0c0-6-4-12-8-12" />"#,
        IconName::Footprints => {
            r#"<path d="M4 16v-2.38C4 11.5 2.97 10.5 3 8c.03-2.72 1.49-6 4.5-6C9.37 2 10 3.8 10 5.5c0 3.11-2 5.66-2 8.68V16a2 2 0 1 1-4 0Z" /><path d="M20 20v-2.38c0-2.12 1.03-3.12 1-5.62-.03-2.72-1.49-6-4.5-6C14.63 6 14 7.8 14 9.5c0 3.11 2 5.66 2 8.68V20a2 2 0 1 0 4 0Z" /><path d="M16 17h4" /><path d="M4 13h4" />"#
        }
        IconName::Gamepad2 => {
            r#"<line x1="6" x2="10" y1="11" y2="11" /><line x1="8" x2="8" y1="9" y2="13" /><line x1="15" x2="15.01" y1="12" y2="12" /><line x1="18" x2="18.01" y1="10" y2="10" /><path d="M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z" />"#
        }
        IconName::Gem => {
            r#"<path d="M10.5 3 8 9l4 13 4-13-2.5-6" /><path d="M17 3a2 2 0 0 1 1.6.8l3 4a2 2 0 0 1 .013 2.382l-7.99 10.986a2 2 0 0 1-3.247 0l-7.99-10.986A2 2 0 0 1 2.4 7.8l2.998-3.997A2 2 0 0 1 7 3z" /><path d="M2 9h20" />"#
        }
        IconName::HeartCrack => {
            r#"<path d="M12.409 5.824c-.702.792-1.15 1.496-1.415 2.166l2.153 2.156a.5.5 0 0 1 0 .707l-2.293 2.293a.5.5 0 0 0 0 .707L12 15" /><path d="M13.508 20.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5a5.5 5.5 0 0 1 9.591-3.677.6.6 0 0 0 .818.001A5.5 5.5 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5z" />"#
        }
        IconName::Package => {
            r#"<path d="M11 21.73a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73z" /><path d="M12 22V12" /><polyline points="3.29 7 12 12 20.71 7" /><path d="m7.5 4.27 9 5.15" />"#
        }
        IconName::PartyPopper => {
            r#"<path d="M5.8 11.3 2 22l10.7-3.79" /><path d="M4 3h.01" /><path d="M22 8h.01" /><path d="M15 2h.01" /><path d="M22 20h.01" /><path d="m22 2-2.24.75a2.9 2.9 0 0 0-1.96 3.12c.1.86-.57 1.63-1.45 1.63h-.38c-.86 0-1.6.6-1.76 1.44L14 10" /><path d="m22 13-.82-.33c-.86-.34-1.82.2-1.98 1.11c-.11.7-.72 1.22-1.43 1.22H17" /><path d="m11 2 .33.82c.34.86-.2 1.82-1.11 1.98C9.52 4.9 9 5.52 9 6.23V7" /><path d="M11 13c1.93 1.93 2.83 4.17 2 5-.83.83-3.07-.07-5-2-1.93-1.93-2.83-4.17-2-5 .83-.83 3.07.07 5 2Z" />"#
        }
        IconName::Pencil => {
            r#"<path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z" /><path d="m15 5 4 4" />"#
        }
        IconName::Pill => {
            r#"<path d="m10.5 20.5 10-10a4.95 4.95 0 1 0-7-7l-10 10a4.95 4.95 0 1 0 7 7Z" /><path d="m8.5 8.5 7 7" />"#
        }
        IconName::Search => r#"<circle cx="11" cy="11" r="8" /><path d="m21 21-4.3-4.3" />"#,
        IconName::Shoppingcart => {
            r#"<circle cx="8" cy="21" r="1" /><circle cx="19" cy="21" r="1" /><path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" />"#
        }
        IconName::Sparkles => {
            r#"<path d="M11.017 2.814a1 1 0 0 1 1.966 0l1.051 5.558a2 2 0 0 0 1.594 1.594l5.558 1.051a1 1 0 0 1 0 1.966l-5.558 1.051a2 2 0 0 0-1.594 1.594l-1.051 5.558a1 1 0 0 1-1.966 0l-1.051-5.558a2 2 0 0 0-1.594-1.594l-5.558-1.051a1 1 0 0 1 0-1.966l5.558-1.051a2 2 0 0 0 1.594-1.594z" /><path d="M20 2v4" /><path d="M22 4h-4" /><circle cx="4" cy="20" r="2" />"#
        }
        IconName::Star => {
            r#"<path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z" />"#
        }
        IconName::Swords => {
            r#"<polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5" /><line x1="13" x2="19" y1="19" y2="13" /><line x1="16" x2="20" y1="16" y2="20" /><line x1="19" x2="21" y1="21" y2="19" /><polyline points="14.5 6.5 18 3 21 3 21 6 17.5 9.5" /><line x1="5" x2="9" y1="14" y2="18" /><line x1="7" x2="4" y1="17" y2="20" /><line x1="3" x2="5" y1="19" y2="21" />"#
        }
        IconName::TriangleAlert => {
            r#"<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3" /><path d="M12 9v4" /><path d="M12 17h.01" />"#
        }
        IconName::X => r#"<path d="M18 6 6 18" /><path d="m6 6 12 12" />"#,
        IconName::Zap => {
            r#"<path d="M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z" />"#
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IconName {
    ArrowUp,
    Check,
    ChevronLeft,
    ChevronRight,
    Circle,
    CircleX,
    Coins,
    Crown,
    Egg,
    Footprints,
    Gamepad2,
    Gem,
    HeartCrack,
    Package,
    PartyPopper,
    Pencil,
    Pill,
    Search,
    Shoppingcart,
    Sparkles,
    Star,
    Swords,
    TriangleAlert,
    X,
    Zap,
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
            "viewBox": "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            "aria-hidden": "true",
            dangerous_inner_html: inner,
        }
    }
}
