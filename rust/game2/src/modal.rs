//! 通用弹窗外壳：hikari `Modal` 标准件，观感压回旧版 `.modal-*`。
//!
//! 结构交给 hikari（遮罩 / 面框 / 标题栏 / 关闭键 / 可滚动正文），样式由
//! `styles/main.scss` 的 `.game-modal` 段按旧版 `.modal-overlay` /
//! `.modal-container` / `.modal-header` / `.modal-body` 调回去——皮肤排在
//! hikari 组件表之后，同权重覆盖。点遮罩或关闭键回调 `on_close`；点面框本体
//! 不关闭（hikari 只把遮罩上的点击当成关闭请求）。
//!
//! 没有 Teleport：面框 `position: fixed` 原地渲染，别把它挂在带 transform 的
//! 祖先里。

use std::rc::Rc;

use hikari_components::{Modal, ModalProps};
use tairitsu_macros::rsx;
use tairitsu_vdom::{EventHandler, VNode};

/// 旧版两种宽度的字面值：`modal-container` 600px，`modal-lg` 800px。hikari 把它
/// 落成面框的 `max-width`，外面再套一层「视口减去两侧边距」的上限；边距的
/// 宽度是变量 `--viewport-gutter`，皮肤把它设成旧版的 20px。
const WIDTH: &str = "600px";
const WIDTH_LG: &str = "800px";

/// 一处弹窗在外壳上的差异。默认值就是旧版各处手绘弹窗的形状：
/// 宽度按 `large` 取，自带关闭键（没有测试钩子），没有额外的标题栏动作。
pub struct ModalSpec<'a> {
    pub title: &'a str,
    pub large: bool,
    /// hikari 自带的关闭键。关掉它就得自己出一枚（`header_actions`）——
    /// 回归脚本要找带 `data-test-*` 的关闭键，而自带的那个没有属性槽。
    pub closable: bool,
    /// 标题右侧的动作槽。自带关闭键时排在它前面。
    pub header_actions: Option<VNode>,
    /// 旧正文是 `modal-body custom-scrollbar` 的那一种：共用外壳 `modal()`
    /// 的弹窗（如「关于本项目」）。那条规则设了 `scrollbar-width: thin`，
    /// 实测 10px；其余弹窗的正文是 `.modal-body` 自己的 6px。
    pub custom_scrollbar: bool,
    /// 旧正文在 `modal-body` 之外还带的类（`.switch-pokemon-modal`、
    /// `.modal-map-details` 这些）。它必须落在带 `modal-body` 的同一个元素上，
    /// 见 [`modal_with`]。
    pub body_class: &'static str,
}

impl<'a> ModalSpec<'a> {
    pub fn new(title: &'a str, large: bool) -> Self {
        Self {
            title,
            large,
            closable: true,
            header_actions: None,
            custom_scrollbar: false,
            body_class: "",
        }
    }
}

/// 旧版共用外壳 `Modal` / `ModalLg` 的入口：它的正文是
/// `modal-body custom-scrollbar`。
pub fn modal(title: &str, large: bool, on_close: Rc<dyn Fn()>, body: Vec<VNode>) -> VNode {
    let on_close = Rc::clone(&on_close);
    let spec = ModalSpec {
        custom_scrollbar: true,
        ..ModalSpec::new(title, large)
    };
    modal_with(spec, move || on_close(), body)
}

/// 同上，差异由 `spec` 给。
///
/// 包着正文的那个 div 就是旧版的 `.modal-body`：它带着 `modal-body`、按需加
/// `custom-scrollbar`，再加上 `body_class`。旧正文自己的类各有一条规则，和
/// `.modal-body` 的同权重、靠源码顺序分胜负，所以必须写在同一个元素上才算得
/// 出旧版的值——实测 `.switch-pokemon-modal { padding: 12px 0 }` 在旧版败给
/// `.modal-body` 的 `20px`；落进内层另一个 div 就会生效，弹窗高出 24px。调用
/// 方因此不要再包一层根 div，`body` 里直接放旧 `.modal-body` 的子节点。hikari
/// 正文层的内边距归零（皮肤里 `--hk-pad: 0px`），内边距由这个 div 给。
pub fn modal_with(spec: ModalSpec<'_>, on_close: impl Fn() + 'static, body: Vec<VNode>) -> VNode {
    let close = Rc::new(on_close);
    let mut body_class = String::from("modal-body game-modal-body");
    let mut content_class = String::from("game-modal");
    if spec.custom_scrollbar {
        body_class.push_str(" custom-scrollbar");
        content_class.push_str(" game-modal--thin-scroll");
    }
    if !spec.body_class.is_empty() {
        body_class.push(' ');
        body_class.push_str(spec.body_class);
    }
    Modal(ModalProps {
        open: true,
        title: Some(spec.title.to_string()),
        closable: spec.closable,
        header_actions: spec.header_actions,
        width: if spec.large { WIDTH_LG } else { WIDTH }.to_string(),
        content_class,
        children: rsx! {
            div { class: body_class, ..body }
        },
        on_close: Some(EventHandler::new(
            move |_event: tairitsu_vdom::MouseEvent| close(),
        )),
        ..Default::default()
    })
}

/// 关闭键要带回归钩子的弹窗。`close_button` 是调用方用
/// [`hook_close_button`] 出的那一枚；点遮罩与点它走同一个 `on_close`。
/// `spec` 照常给（标题、宽度、`body_class`），这里只把自带关闭键换掉。
pub fn modal_hooked(
    spec: ModalSpec<'_>,
    close_button: VNode,
    on_close: impl Fn() + 'static,
    body: Vec<VNode>,
) -> VNode {
    let spec = ModalSpec {
        closable: false,
        header_actions: Some(close_button),
        ..spec
    };
    modal_with(spec, on_close, body)
}

/// 出一枚带回归钩子的关闭键，长相与 hikari 自带的那枚相同（同一个图标、同一
/// 组类名，皮肤按 `.game-modal .hk-modal-close` 调）。`hook` 是 `data-test-*` 的
/// 属性名；`on_close` 是点击时调用的闭包（要 `move`，它得活得比渲染久）。
///
/// ```ignore
/// hook_close_button("data-test-ev-close", move || close_ev_modal(&state))
/// ```
///
/// 有意不是 hikari `Button`：它是 Modal 的外壳部件，布局来自 hikari 的
/// window-close 表和皮肤，不是 game.css——换成 `Button` 的话皮肤里给旧观感按钮
/// 准备的撤销层会把这些一起退掉。这里就是 hikari 自带关闭键的那份标记，多一个属性。
pub fn hook_close_button(hook: &'static str, on_close: impl Fn() + 'static) -> VNode {
    crate::button::with_attrs(
        tairitsu_macros::rsx! {
            button {
                class: "hk-window-close hk-modal-close",
                r#type: "button",
                "aria-label": "关闭",
                onclick: move |_| on_close(),
                {hikari_components::prelude::Icon(hikari_components::prelude::IconProps {
                    icon: hikari_components::prelude::MdiIcon::Close,
                    size: 16,
                    ..Default::default()
                })}
            }
        },
        |el| {
            el.attributes.insert(hook.to_string(), "1".to_string());
        },
    )
}
