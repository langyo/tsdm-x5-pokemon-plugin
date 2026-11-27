//! 按钮：hikari `Button` 带着旧版类名渲染，观感仍由 game.css 定。
//!
//! 旧按钮的类（`.btn`、各处的专用类）原样放在 `class` 上，旧规则照常生效；
//! hikari 的按钮表对它整表退回（`styles/main.scss` 的 `hikari-reset` 分层），
//! 所以换前换后计算样式一致。文字和图标作为子节点直接传进去——不用 `label`
//! 属性，那样 hikari 会多包一层 `span.hk-btn-label`。
//!
//! hikari `Button` 没有属性透传（其头注释列的 delta），而回归脚本靠按钮上的
//! `data-test-*` 找它们，旧标记里的 `title`、`aria-label` 也要留着。tairitsu 的
//! `VElement::attributes` 是公开的——rsx 自己也是往里写——所以属性补在组件产出的
//! 根元素上，[`hooked`] 是这件事的写法：
//!
//! ```text
//! { crate::button::hooked(tairitsu_macros::rsx! { Button {
//!     class: "btn btn-warning".to_string(), onclick: ..., "逃跑"
//! } }, [("data-test-bt-flee", "1".to_string())]) }
//! ```
//!
//! 这里得是函数、内层的 `rsx!` 得写在调用处，不能收成宏：属性列表经
//! `macro_rules` 的 `tt` 片段转交之后，内层 rsx 从 `"{name}"` 这样的插值里合成的
//! 标识符拿不到调用处的上下文，看不到外层的局部变量（实测）。

use tairitsu_vdom::{EventHandler, MouseEvent, VElement, VNode};

/// 「点了就做」→ hikari `Button` 要的处理器。旧按钮的处理器多半不看事件。
pub fn click(run: impl Fn() + 'static) -> Option<EventHandler<MouseEvent>> {
    Some(EventHandler::new(move |_event: MouseEvent| run()))
}

/// hikari `Button` 的根元素上再带几个属性。
pub fn hooked<const N: usize>(node: VNode, attrs: [(&'static str, String); N]) -> VNode {
    with_attrs(node, |el| {
        for (name, value) in attrs {
            el.attributes.insert(name.to_string(), value);
        }
    })
}

/// 在一个节点的根元素上补东西（属性、`inner_html`）。根不是元素时原样返回。
pub fn with_attrs(mut node: VNode, edit: impl FnOnce(&mut VElement)) -> VNode {
    if let VNode::Element(el) = &mut node {
        edit(el);
    }
    node
}
