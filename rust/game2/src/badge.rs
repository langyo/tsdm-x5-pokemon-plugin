//! 徽章：hikari `Badge` 带着旧版类名渲染，观感仍由 game.css 定。
//!
//! 和 [`crate::button`] 里的按钮同一回事：旧徽章的类（`.badge.badge-type-*`、
//! `.badge.badge-skill-type.*`、`.badge.badge-category.*`）原样挂在 `class` 上，
//! 旧规则照常生效；hikari 的徽章表对它整表退回（`styles/main.scss` 的
//! `hikari-reset` 分层）。
//!
//! hikari `Badge` 没有 `class` 属性（和 `Button` 一样没有透传），所以旧类名是
//! 补在组件产出的根元素上的：它经 `VElement` 搭出来，类名在 `VElement::class` 里。

use hikari_components::{Badge, BadgeProps};
use tairitsu_vdom::VNode;

/// 一枚旧版徽章：`class` 是旧类名全串，`text` 是徽章文字。
pub fn legacy(class: impl Into<String>, text: impl Into<String>) -> VNode {
    let class = class.into();
    let text = text.into();
    crate::button::with_attrs(
        Badge(BadgeProps {
            // rsx 的顶层只收元素/表达式/控制流，一段文字直接给文本节点。
            children: VNode::Text(tairitsu_vdom::VText::new(&text)),
            ..Default::default()
        }),
        move |el| {
            el.class = el.class.clone().add(&class);
        },
    )
}
