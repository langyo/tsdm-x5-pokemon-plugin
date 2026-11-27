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

/// 弹窗外壳。`large` 对应旧版的 `modal-lg`。
pub fn modal(title: &str, large: bool, on_close: Rc<dyn Fn()>, body: Vec<VNode>) -> VNode {
    let close = Rc::clone(&on_close);
    Modal(ModalProps {
        open: true,
        title: Some(title.to_string()),
        width: if large { WIDTH_LG } else { WIDTH }.to_string(),
        content_class: "game-modal".to_string(),
        children: rsx! {
            div { class: "game-modal-body", ..body }
        },
        on_close: Some(EventHandler::new(move |_event: tairitsu_vdom::MouseEvent| {
            close()
        })),
        ..Default::default()
    })
}
