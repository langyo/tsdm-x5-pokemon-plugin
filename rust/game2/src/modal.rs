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
/// 宽度按 `large` 取，自带关闭键（没有测试钩子），没有额外的标题栏动作，
/// 正文是 6px 细滚动条。
pub struct ModalSpec<'a> {
    pub title: &'a str,
    pub large: bool,
    /// hikari 自带的关闭键。关掉它就得自己出一枚（`header_actions`）——
    /// 回归脚本要找带 `data-test-*` 的关闭键，而自带的那个没有属性槽。
    pub closable: bool,
    /// 标题右侧的动作槽。自带关闭键时排在它前面。
    pub header_actions: Option<VNode>,
    /// 旧版正文带 `custom-scrollbar` 的弹窗：那条规则设了
    /// `scrollbar-width: thin`，实测 10px；其余是 `.modal-body` 自己的 6px。
    pub thin_scrollbar: bool,
}

impl<'a> ModalSpec<'a> {
    pub fn new(title: &'a str, large: bool) -> Self {
        Self {
            title,
            large,
            closable: true,
            header_actions: None,
            thin_scrollbar: false,
        }
    }
}

/// 旧版共用外壳 `Modal` / `ModalLg` 的入口：它的正文是
/// `modal-body custom-scrollbar`（细滚动条走 `thin`）。
pub fn modal(title: &str, large: bool, on_close: Rc<dyn Fn()>, body: Vec<VNode>) -> VNode {
    let on_close = Rc::clone(&on_close);
    let spec = ModalSpec {
        thin_scrollbar: true,
        ..ModalSpec::new(title, large)
    };
    modal_with(spec, move || on_close(), body)
}

/// 同上，差异由 `spec` 给。
pub fn modal_with(
    spec: ModalSpec<'_>,
    on_close: impl Fn() + 'static,
    body: Vec<VNode>,
) -> VNode {
    let close = Rc::new(on_close);
    let content_class = if spec.thin_scrollbar {
        "game-modal game-modal--thin-scroll"
    } else {
        "game-modal"
    };
    Modal(ModalProps {
        open: true,
        title: Some(spec.title.to_string()),
        closable: spec.closable,
        header_actions: spec.header_actions,
        width: if spec.large { WIDTH_LG } else { WIDTH }.to_string(),
        content_class: content_class.to_string(),
        children: rsx! {
            div { class: "game-modal-body", ..body }
        },
        on_close: Some(EventHandler::new(move |_event: tairitsu_vdom::MouseEvent| {
            close()
        })),
        ..Default::default()
    })
}

/// 关闭键要带回归钩子的弹窗。`close_button` 是调用方用
/// [`hook_close_button!`] 出的那一枚；点遮罩与点它走同一个 `on_close`。
pub fn modal_hooked(
    title: &str,
    large: bool,
    close_button: VNode,
    on_close: impl Fn() + 'static,
    body: Vec<VNode>,
) -> VNode {
    modal_with(
        ModalSpec {
            closable: false,
            header_actions: Some(close_button),
            ..ModalSpec::new(title, large)
        },
        on_close,
        body,
    )
}

/// 出一枚带回归钩子的关闭键，长相与 hikari 自带的那枚相同（同一个图标、同一
/// 组类名，皮肤按 `.game-modal .hk-modal-close` 调）。
///
/// 写成宏只为让 `data-test-*` 的属性名能写成字面值：rsx 的属性名必须是字面值，
/// 函数传不进去。`$close` 是点击时调用的闭包（要 `move`，它得活得比渲染久）；
/// 宏里的路径按调用处解析，所以全写全限定。
///
/// ```ignore
/// hook_close_button!("data-test-ev-close", move || close_ev_modal(&state))
/// ```
macro_rules! hook_close_button {
    ($hook:literal, $close:expr) => {{
        let close = $close;
        tairitsu_macros::rsx! {
            button {
                class: "hk-window-close hk-modal-close",
                r#type: "button",
                "aria-label": "关闭",
                $hook: "1",
                onclick: move |_| close(),
                {hikari_components::prelude::Icon(hikari_components::prelude::IconProps {
                    icon: hikari_components::prelude::MdiIcon::Close,
                    size: 16,
                    ..Default::default()
                })}
            }
        }
    }};
}
pub(crate) use hook_close_button;
